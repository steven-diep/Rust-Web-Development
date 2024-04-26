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

    pub fn init(&mut self) {
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
        self.add_question(question);
        self.add_question(question2);
        self.add_question(question3);
    }

    pub fn add_question(&mut self, question: Question) {
        self.questions.insert(question.id.clone(), question);
    }

    pub fn update_question(&mut self, id: &str, question: Question) -> Result<StatusCode, Error> {
        if !self.questions.contains_key(id) {
            return Err(Error::QuestionNotFound);
        }

        self.questions
            .entry(id.to_string())
            .and_modify(|q| *q = question);

        Ok(StatusCode::OK)
    }

    pub fn delete_question(&mut self, id: &str) -> Result<StatusCode, Error> {
        if !self.questions.contains_key(id) {
            return Err(Error::QuestionNotFound);
        }

        self.questions.remove(id);

        Ok(StatusCode::OK)
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

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    QuestionNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            },
            Error::MissingParameters => write!(f, "Missing parameter"), 
            Error::QuestionNotFound => write!(f, "Question not found"),
        }
    }
}