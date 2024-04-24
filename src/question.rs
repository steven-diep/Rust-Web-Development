use crate::*;

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

impl Question {
    // Method for creating a new Question given some input
    // Returns a Question (Self)
    pub fn new(id: String, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}
