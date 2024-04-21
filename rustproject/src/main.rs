mod question;

use axum::{
    routing::get, 
    Json,
    Router,
    http::StatusCode,
};

use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, str::FromStr};
use question::*;

// Handler method for getting a question
// Used ChatGPT to fix the return type
async fn get_questions() -> Result<Json<Question>, (StatusCode, String)> {
    // Create a new question
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()])
    );

    // If the question id can be parsed to an integer, return the question as a json response
    // Otherwise, return an error message
    match question.id.0.parse::<i32>() {
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid Question ID".to_string())),
        Ok(_) => {
            Ok(Json(question))
        },
    }
}

#[tokio::main]
async fn main() {
    // Create an app with a handler for questions
    let app = Router::new().route("/questions", get(get_questions));

    // Host the app on localhost:3000
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    // Run the app
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
