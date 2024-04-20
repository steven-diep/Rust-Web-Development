use axum::{routing::get, Router};
use std::net::{SocketAddr, IpAddr, Ipv4Addr};

#[tokio::main]
async fn main() {
    // Create an app
    let hello = Router::new().route("/", get("Hello, World!"));

    // Host the app on localhost:3000
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    // Run the app
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, hello).await.unwrap();
}