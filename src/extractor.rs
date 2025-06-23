use reqwest;
use scraper::{self, Element, ElementRef};
use std::fmt::Display;

const BASE_URL: &str = "https://etesty2.mdcr.cz";
const USER_AGENT: &str = "testing"; // TODO: change to a valid user agent string

#[derive(Debug, Clone)]
pub(crate) struct Topic {
    title: String,
    url: String,
}

impl Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.url)
    }
}

pub(crate) fn fetch_bulletin_topics() -> Result<Vec<Topic>, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()?;
    let url = format!("{}/ro/Bulletin", BASE_URL);
    let response = client.get(url).send()?;
    let html = response.text()?;

    let document = scraper::Html::parse_document(&html);
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

#[derive(Debug)]
pub(crate) enum QuestionOption {
    A,
    B,
    C,
}

#[derive(Debug)]
pub(crate) struct Question {
    code: String,
    date_added: String,
    question: String,
    option_a: String,
    option_b: String,
    option_c: String,
    correct_answer: QuestionOption,
}

pub(crate) fn fetch_questions(topic: Topic) -> Result<Vec<Question>, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()?;
    let url = BASE_URL.to_string() + &topic.url + "&pagex=1&pageSize=5";
    let response = client.get(url).send()?;
    let html = response.text()?;

    let document = scraper::Html::parse_document(&html);
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
            let question = extract_text_by_selector(&question_panel, "div.QuestionImagePanel");

            let option_a_selector = "div.AnswersPanel div:nth-of-type(1) div.answer-text";
            let option_a = extract_text_by_selector(&question_panel, option_a_selector);
            let option_a_correct = extract_bool_attribute_by_selector(
                &question_panel,
                option_a_selector,
                "data-iscorrect",
            );

            let option_b_selector = "div.AnswersPanel div:nth-of-type(2) div.answer-text";
            let option_b = extract_text_by_selector(&question_panel, option_b_selector);
            let option_b_correct = extract_bool_attribute_by_selector(
                &question_panel,
                option_b_selector,
                "data-iscorrect",
            );

            let option_c_selector = "div.AnswersPanel div:nth-of-type(3) div.answer-text";
            let option_c = extract_text_by_selector(&question_panel, option_c_selector);
            let option_c_correct = extract_bool_attribute_by_selector(
                &question_panel,
                option_c_selector,
                "data-iscorrect",
            );

            let correct_answer = if option_a_correct {
                QuestionOption::A
            } else if option_b_correct {
                QuestionOption::B
            } else if option_c_correct {
                QuestionOption::C
            } else {
                panic!("No correct answer found")
            };

            Question {
                code,
                date_added,
                question,
                option_a,
                option_b,
                option_c,
                correct_answer,
            }
        })
        .collect();
    Ok(questions)
}

fn extract_text_by_selector(element: &ElementRef, selector: &str) -> String {
    element
        .select(&scraper::Selector::parse(selector).unwrap())
        .next()
        .unwrap()
        .text()
        .collect()
}

fn extract_bool_attribute_by_selector(element: &ElementRef, selector: &str, attr: &str) -> bool {
    element
        .select(&scraper::Selector::parse(selector).unwrap())
        .next()
        .unwrap()
        .value()
        .attr(attr)
        .unwrap()
        .to_lowercase()
        .parse::<bool>()
        .unwrap()
}
