mod question;
mod store;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use question::*;
use store::*;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

async fn return_error() -> Response {
    (StatusCode::NOT_FOUND, "Route not found").into_response()
}

// Handler method for getting a question
// Used ChatGPT to fix the return type
async fn get_questions() -> Result<Json<Question>, (StatusCode, String)> {
    // Create a new question
    let question = Question::new(
        "1".to_string(),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    // If the question id can be parsed to an integer, return the question as a json response
    // Otherwise, return an error message
    match question.id.parse::<i32>() {
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid Question ID".to_string())),
        Ok(_) => Ok(Json(question)),
    }
}

#[tokio::main]
async fn main() {
    // Create an in-memory database
    let store = Store::new();

    // Create an app with a handler for questions
    // Fallback calls the error handler if the route cannot be found
    let app = Router::new()
        .route("/questions", get(get_questions))
        .fallback(return_error)
        .with_state(store);

    // Host the app on localhost:3000
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    // Run the app
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
