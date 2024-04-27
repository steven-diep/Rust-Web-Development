mod api;
mod question;
mod store;

use api::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use question::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use store::*;
use tokio::{self, sync::RwLock};

/// Handler to return an error message if a route cannot be found
async fn return_error() -> Response {
    (StatusCode::NOT_FOUND, "Route not found").into_response()
}

#[tokio::main]
async fn main() {
    // Create an in-memory database and populate it
    let mut store = Store::new();
    store.init();

    // Make sure the store can be accessed by multiple threads safely
    let store = Arc::new(RwLock::new(store));

    // Create an app with a handler for questions
    // Fallback calls the error handler if the route cannot be found
    let app = Router::new()
        .route("/questions", get(get_questions))
        .route("/questions/:id", get(get_question))
        .route("/questions", post(add_question))
        .route("/questions/:id", put(update_question))
        .route("/questions/:id", delete(delete_question))
        .fallback(return_error)
        .with_state(store);

    // Host the app on localhost:3000
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3000);

    // Run the app
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
