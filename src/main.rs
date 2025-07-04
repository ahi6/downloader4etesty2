mod extractor;
mod types;

use crate::extractor::Extractor;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    let extractor = Extractor::new();

    let topics = extractor.fetch_bulletin_topics().unwrap();

    let mut topic_map = HashMap::new();

    for topic in &topics {
        topic_map.insert(&topic.title, &topic.url);
    }

    let topics_to_download = inquire::MultiSelect::new(
        "Select topics to download:",
        topic_map.keys().cloned().collect(),
    )
    .with_page_size(10)
    .prompt()
    .unwrap();

    let output_dir = inquire::Text::new("Enter output directory:")
        .with_initial_value("./output")
        .prompt()
        .unwrap();
    let output_path = Path::new(&output_dir);

    std::fs::create_dir_all(output_path).expect("Failed to create output directory");

    let should_download_media = inquire::Confirm::new("Download media?")
        .with_default(true)
        .with_help_message(
            "This will save images and videos alongside the topics in the output directory",
        )
        .prompt()
        .unwrap();

    for topic in topics_to_download {
        // File path is truncated to avoid errors from file length limit
        let path = output_path
            .join(String::from(topic.chars().take(24).collect::<String>()).to_string() + ".json");
        let topic_file = std::fs::File::create(&path).expect("Failed to create file");

        let topic_url = topic_map.get(&topic).unwrap();
        println!("Downloading {}", topic);

        let questions = extractor.fetch_questions(topic_url).unwrap();

        if should_download_media {
            download_media_to_file(&questions, &extractor, &output_path);
        }

        let _ = serde_json::to_writer_pretty(topic_file, &questions).expect("Failed to write JSON");

        println!("Downloaded to {}", path.display());
    }
}

fn download_media_to_file(
    questions: &Vec<types::Question>,
    extractor: &Extractor,
    output_path: &Path,
) {
    // generic closure to download media
    let download_media = |url: &str| {
        println!("Downloading media: {}", url);
        let media_path = output_path.join(Path::new(url).strip_prefix("/").unwrap());

        if media_path.exists() {
            println!("Media already exists, skipping");
            return;
        }

        let media = extractor
            .fetch_media_file(url)
            .expect("Failed to download media");

        // Create parent folder if it doesn't exist
        if let Some(folder_path) = media_path.parent() {
            let _ = std::fs::create_dir_all(folder_path).expect("Failed to create folder");
        }

        let _ = std::fs::write(&media_path, media).expect("Failed to write media");
    };

    // download all media types using closure
    for question in questions {
        // Question media
        if let Some(image_url) = &question.question_image {
            download_media(image_url);
        }
        if let Some(video_url) = &question.question_video {
            download_media(video_url);
        }

        // Answer media
        if let types::QuestionOptionType::Image(image_url) = &question.option_a.content {
            download_media(image_url);
        }
        if let types::QuestionOptionType::Image(image_url) = &question.option_b.content {
            download_media(image_url);
        }
        if let Some(option_c) = &question.option_c {
            if let types::QuestionOptionType::Image(image_url) = &option_c.content {
                download_media(image_url);
            }
        }
    }
}
