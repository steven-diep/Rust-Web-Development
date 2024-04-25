use crate::*;

#[derive(Clone)]
pub struct Store {
    pub questions: HashMap<String, Question>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }

    pub fn init(self) -> Self {
        let question = Question::new(
            "1".to_string(),
            "First Question".to_string(),
            "Content of question".to_string(),
            Some(vec!["faq".to_string()]),
        );
        let question2 = Question::new(
            "2".to_string(),
            "Second Question".to_string(),
            "Content of question".to_string(),
            Some(vec!["faq".to_string()]),
        );
        let question3 = Question::new(
            "3".to_string(),
            "Third Question".to_string(),
            "Content of question".to_string(),
            Some(vec!["faq".to_string()]),
        );
        let mut store = self.add_question(question);
        store = store.add_question(question2);
        store.add_question(question3)
    }

    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoResponse for &Store {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self.questions)).into_response()
    }
}