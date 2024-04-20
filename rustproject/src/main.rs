mod question;

use axum::{routing::get, Router};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use question::*;

#[tokio::main]
async fn main() {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string()))
    );
    println!("{:?}", question);

    // Create an app
    let hello = Router::new().route("/", get("Hello, World!"));

    // Host the app on localhost:3000
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    // Run the app
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, hello).await.unwrap();
}
