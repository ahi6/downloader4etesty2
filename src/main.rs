mod extractor;

fn main() {
    println!("Topics:");
    let topics = extractor::fetch_bulletin_topics();
    if let Ok(topics) = &topics {
        for topic in topics {
            println!("{}", &topic);
        }
    }

    println!("Questions:");
    let questions = extractor::fetch_questions(topics.unwrap()[1].clone());
    if let Ok(questions) = &questions {
        for question in questions {
            println!("{:?}", &question);
        }
    }
}
