use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Topic {
    pub title: String,
    pub url: String,
}

impl Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.url)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum QuestionOptionType {
    Text(String),
    Image(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct QuestionOption {
    pub content: QuestionOptionType,
    pub is_correct: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Question {
    pub code: String,
    pub date_added: String,
    pub question_text: String,
    pub question_image: Option<String>,
    pub question_video: Option<String>,
    pub option_a: QuestionOption,
    pub option_b: QuestionOption,
    pub option_c: Option<QuestionOption>, // option C is not always present
}

/// Convenient placeholder implementation for Question
/// Useful for testing
impl Default for Question {
    fn default() -> Self {
        Self {
            code: "[12345678]".to_string(),
            date_added: "(1. 1. 2000)".to_string(),
            question_text: "Placeholder question".to_string(),
            question_image: None,
            question_video: None,
            option_a: QuestionOption {
                content: QuestionOptionType::Text("A".to_string()),
                is_correct: false,
            },
            option_b: QuestionOption {
                content: QuestionOptionType::Text("B".to_string()),
                is_correct: false,
            },
            option_c: Some(QuestionOption {
                content: QuestionOptionType::Text("C".to_string()),
                is_correct: true,
            }),
        }
    }
}
