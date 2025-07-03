mod extractor;
mod types;

use inquire;
use serde_json;
use std::collections::HashMap;

fn main() {
    let topics = extractor::fetch_bulletin_topics().unwrap();

    let mut topic_map = HashMap::new();

    for topic in &topics {
        topic_map.insert(&topic.title, &topic.url);
    }

    let topics_to_download = inquire::MultiSelect::new(
        "Select topics to download:",
        topic_map.keys().cloned().collect(),
    )
    .prompt()
    .unwrap();

    let output_dir = inquire::Text::new("Enter output directory:")
        .with_initial_value("./output")
        .prompt()
        .unwrap();
    let output_path = std::path::Path::new(&output_dir);

    std::fs::create_dir_all(output_path).expect("Failed to create output directory");

    for topic in topics_to_download {
        // File path is truncated to avoid errors from file length limit
        let path = output_path
            .join(String::from(topic.chars().take(24).collect::<String>()).to_string() + ".json");
        let topic_file = std::fs::File::create(&path).expect("Failed to create file");

        let topic_url = topic_map.get(&topic).unwrap();
        println!("Downloading {}", topic);

        let questions = extractor::fetch_questions(topic_url).unwrap();

        let _ = serde_json::to_writer_pretty(topic_file, &questions).expect("Failed to write JSON");

        println!("Downloaded to {}", path.display());
    }
}
