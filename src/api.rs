use crate::*;

// Create
// Handler method for creating a question, given input from a request
pub async fn add_question(
    State(store): State<Arc<RwLock<Store>>>,
    Json(question): Json<Question>,
) -> Response {
    store.write().await.add_question(question);
    (StatusCode::CREATED, "Question added".to_string()).into_response()
}

// Read
// Handler method for getting every question
pub async fn get_questions(State(store): State<Arc<RwLock<Store>>>) -> Response {
    // Ask for the lock to read the store, wait for the lock to be granted, return the store as a response
    store.read().await.into_response()

    // Add pagination
}

// Handler method for getting a specific question, given the id from a request
pub async fn get_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(id): Path<String>,
) -> Response {
    match store.write().await.get_question(&id) {
        Ok(q) => (StatusCode::OK, Json(q)).into_response(),
        Err(Error::QuestionNotFound) => (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, "Bad Request".to_string()).into_response(),
    }
}

// Update
pub async fn update_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(id): Path<String>,
    Json(question): Json<Question>,
) -> Response {
    match store.write().await.update_question(&id, question) {
        Ok(_) => (StatusCode::OK, "Question updated".to_string()).into_response(),
        Err(Error::QuestionNotFound) => (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, "Bad Request".to_string()).into_response(),
    }
}

// Delete
pub async fn delete_question(
    State(store): State<Arc<RwLock<Store>>>,
    Path(id): Path<String>,
) -> Response {
    match store.write().await.delete_question(&id) {
        Ok(_) => (StatusCode::OK, "Question deleted".to_string()).into_response(),
        Err(Error::QuestionNotFound) => (StatusCode::NOT_FOUND, "Question not found".to_string()).into_response(),
        Err(_) => (StatusCode::BAD_REQUEST, "Bad Request".to_string()).into_response(),
    }
}