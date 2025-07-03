use crate::types::*;
use reqwest;
use scraper;

const BASE_URL: &str = "https://etesty2.mdcr.cz";
// const USER_AGENT: &str = " "; // TODO: change to a valid user agent string

fn get_page(url: &str) -> Result<scraper::Html, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        // .user_agent(USER_AGENT)
        .build()?;
    let response = client.get(url).send()?;
    let html = response.text()?;

    let document = scraper::Html::parse_document(&html);
    Ok(document)
}

pub(crate) fn fetch_bulletin_topics() -> Result<Vec<Topic>, reqwest::Error> {
    let url = format!("{}/ro/Bulletin", BASE_URL);
    let document = get_page(&url)?;
    let selector = scraper::Selector::parse("html body div.content.content-column div.side-panel ul.side-panel-list li.side-panel-item a.side-panel-link").unwrap();

    let elements = document.select(&selector);
    let topics = elements
        .map(|element| {
            let text = element.text().collect::<String>();
            let url = element
                .value()
                .attr("href")
                .unwrap()
                .parse::<String>()
                .unwrap();
            Topic { title: text, url }
        })
        .collect();

    Ok(topics)
}

pub(crate) fn fetch_questions(topic_url: &str) -> Result<Vec<Question>, reqwest::Error> {
    let url = BASE_URL.to_string() + topic_url + "&pagex=1&pageSize=5";
    let document = get_page(&url)?;
    let selector = scraper::Selector::parse(
        "html body div.content.content-column div.container div.QuestionPanel",
    )
    .unwrap();

    let question_panels = document.select(&selector);
    let questions = question_panels
        .map(|question_panel| {
            let code =
                extract_text_by_selector(&question_panel, "div.QuestionText span.QuestionCode");
            let date_added = extract_text_by_selector(
                &question_panel,
                "div.QuestionText span.QuestionChangeDate",
            );
            let question_text = extract_text_by_selector(&question_panel, "div.QuestionImagePanel")
                .trim()
                .to_string();

            let question_image =
                extract_question_image_by_selector(&question_panel, ".question-image > img");

            let mut options = extract_question_options(&question_panel);

            let option_a = options.pop().unwrap();
            let option_b = options.pop().unwrap();
            let option_c = options.pop().unwrap();

            Question {
                code,
                date_added,
                question_text,
                question_image,
                option_a,
                option_b,
                option_c,
            }
        })
        .collect();
    Ok(questions)
}

fn extract_text_by_selector(element: &scraper::ElementRef, selector: &str) -> String {
    element
        .select(&scraper::Selector::parse(selector).expect("Invalid selector"))
        .next()
        .expect("No element found")
        .text()
        .collect()
}

fn extract_question_image_by_selector(
    element: &scraper::ElementRef,
    selector: &str,
) -> Option<String> {
    element
        .select(&scraper::Selector::parse(selector).expect("Invalid selector"))
        .next()
        .map(|img| img.attr("src").unwrap().to_string())
}

fn extract_question_options(question_panel: &scraper::ElementRef) -> Vec<QuestionOption> {
    let mut question_options = Vec::new();

    let option_selector = "div.AnswersPanel > div:nth-of-type(1) div[data-iscorrect]";

    for option in question_panel.select(&scraper::Selector::parse(option_selector).unwrap()) {
        let is_correct = option
            .attr("data-iscorrect")
            .unwrap()
            .to_lowercase()
            .parse::<bool>()
            .unwrap();

        let class = option.attr("class").unwrap();

        let content = match class {
            "answer-text" => {
                let text = option.text().collect();
                QuestionOptionType::Text(text)
            }
            "answer-image" => {
                let img = option
                    .select(&scraper::Selector::parse("img").unwrap())
                    .next()
                    .unwrap();
                let src = img.attr("src").unwrap();
                QuestionOptionType::Image(src.to_string())
            }
            _ => unimplemented!(),
        };

        question_options.push(QuestionOption {
            content,
            is_correct,
        });
    }

    question_options
}
