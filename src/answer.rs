use crate::*;

/// Answer struct used to store questions in the database
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Answer {
    pub id: i32,
    pub content: String,
    pub corresponding_question: i32,
}

/// New answer struct used to create and update questions in the database
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct NewAnswer {
    pub content: String,
    pub corresponding_question: i32,
}
