mod extractor;
use inquire;
use std::collections::HashMap;

fn main() {
    println!("Topics:");
    let topics = extractor::fetch_bulletin_topics();

    let mut topic_map = HashMap::new();
    if let Ok(topics) = &topics {
        for topic in topics {
            topic_map.insert(&topic.title, &topic.url);
            // println!("{}", &topic);
        }
    }

    let topics_to_download = inquire::MultiSelect::new(
        "Select topics to download:",
        topic_map.keys().cloned().collect(),
    )
    .prompt()
    .unwrap();

    for topic in topics_to_download {
        let topic_url = topic_map.get(&topic).unwrap();
        println!("Downloading {}", topic);

        println!("Questions:");
        let questions = extractor::fetch_questions(topic_url);
        if let Ok(questions) = &questions {
            for question in questions {
                println!("{:?}", &question);
            }
        }
    }
}
