use crate::*;

pub struct Store {
    pub questions: HashMap<QuestionId, Question>,
}