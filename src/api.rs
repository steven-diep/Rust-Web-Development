use crate::*;

// Create
// Handler method for creating a question, given input from a request
pub async fn add_question(
    State(store): State<Arc<RwLock<Store>>>,
    Json(question): Json<Question>,
) -> Response {
    store.write().await.add_question(question);
    (StatusCode::OK, "Question added".to_string()).into_response()
}

// Read
// Handler method for getting every question
pub async fn get_questions(State(store): State<Arc<RwLock<Store>>>) -> Response {
    // Ask for the lock to read the store, wait for the lock to be granted, return the store as a response
    store.read().await.into_response()

    // Add pagination
}

// Handler method for getting a specific question, given the id from a request

// Update

// Delete