use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionId(pub String);

impl Question {
    // Method for creating a new Question given some input
    // Returns a Question (Self)
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    // Method to create a Question Id from a string
    // If the string is empty: return an error; otherwise, return the question id
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}
