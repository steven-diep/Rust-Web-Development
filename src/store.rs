use crate::*;

/// Store struct that is being used as an in-memory storage for questions
#[derive(Clone)]
pub struct Store {
    pub questions: HashMap<String, Question>,
}

impl Store {
    /// Constructor to create a new in-memory storage
    pub fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }

    /// Method used to create a set of sample questions and add them to storage
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

    /// Add a given question to the hash map
    pub fn add_question(&mut self, question: Question) {
        self.questions.insert(question.id.clone(), question);
    }

    /// Return a reference to the entire hash map
    pub fn get_questions(&self) -> &HashMap<String, Question> {
        &self.questions
    }

    /// Return a reference to a question given a specified id
    pub fn get_question(&self, id: &str) -> Result<&Question, Error> {
        match self.questions.get(id) {
            Some(q) => Ok(q),
            None => Err(Error::QuestionNotFound),
        }
    }

    /// Update a question given a specified id and new data
    pub fn update_question(&mut self, id: &str, question: Question) -> Result<(), Error> {
        match self.questions.get_mut(id) {
            // When a mutable reference is returned, update its content
            Some(q) => {
                *q = question;
                Ok(())
            }
            None => Err(Error::QuestionNotFound),
        }
    }

    /// Delete a question given a specified id
    pub fn delete_question(&mut self, id: &str) -> Result<(), Error> {
        match self.questions.remove(id) {
            Some(_) => Ok(()),
            None => Err(Error::QuestionNotFound),
        }
    }
}

/// Required by Clippy for some reason
impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
