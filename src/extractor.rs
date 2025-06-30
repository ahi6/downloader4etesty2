use reqwest;
use scraper;
use std::fmt::Display;

const BASE_URL: &str = "https://etesty2.mdcr.cz";
const USER_AGENT: &str = " "; // TODO: change to a valid user agent string

#[derive(Debug, Clone)]
pub(crate) struct Topic {
    pub title: String,
    pub url: String,
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
pub(crate) struct Question {
    code: String,
    date_added: String,
    question: String,
    option_a: QuestionOption,
    option_b: QuestionOption,
    option_c: QuestionOption,
}

#[derive(Debug)]
enum QuestionOptionType {
    Text(String),
    Image(String),
}

#[derive(Debug)]
struct QuestionOption {
    content: QuestionOptionType,
    is_correct: bool,
}

pub(crate) fn fetch_questions(topic_url: &str) -> Result<Vec<Question>, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .build()?;
    let url = BASE_URL.to_string() + topic_url + "&pagex=1&pageSize=5";
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

            let mut options = extract_question_options(&question_panel);

            let option_a = options.pop().unwrap();
            let option_b = options.pop().unwrap();
            let option_c = options.pop().unwrap();

            Question {
                code,
                date_added,
                question,
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
