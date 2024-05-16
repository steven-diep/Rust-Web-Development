use crate::*;

/// Question struct used to store questions in the database
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Question {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

/// New question struct used to create and update questions in the database
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
