use serde::Serialize;
use std::fmt::Display;

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

#[derive(Debug, Serialize)]
pub enum QuestionOptionType {
    Text(String),
    Image(String),
}

#[derive(Debug, Serialize)]
pub struct QuestionOption {
    pub content: QuestionOptionType,
    pub is_correct: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct Question {
    pub code: String,
    pub date_added: String,
    pub question_text: String,
    pub question_image: Option<String>,
    pub option_a: QuestionOption,
    pub option_b: QuestionOption,
    pub option_c: QuestionOption,
}
