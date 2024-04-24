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
        self.add_question(question)
    }

    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }
}

impl IntoResponse for &Store {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(&self.questions)).into_response()
    }
}