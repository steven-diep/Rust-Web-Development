mod api;
mod question;
mod store;

use api::*;
use axum::{
    extract::{MatchedPath, Path, Query, State},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Json, Router,
};
use question::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use std::error::Error;
use std::sync::Arc;
use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use store::*;
use tokio::{self, sync::RwLock};
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::fmt::format::FmtSpan;

/// Handler to return an error message if a route cannot be found
async fn return_error() -> Response {
    (StatusCode::NOT_FOUND, "Route not found").into_response()
}

#[tokio::main]
async fn main() {
    // Set up tracing in order to get tracing information printed to the console
    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "practical_rust_book=info".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    // Create an data store, if the data store fails to connect, end the program
    // Add error to logging when logging is set up
    let store = Store::new().await.unwrap_or_else(|e| {
        tracing::error!("Error: {:?}", e);
        std::process::exit(1);
    });
    tracing::info!("Connected to database");

    // Make sure the data store can be accessed by multiple threads safely
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
        // Source for trace layer code: https://github.com/tokio-rs/axum/blob/main/examples/tracing-aka-logging/src/main.rs
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .with_state(store);

    // Host the app on 0.0.0.0 so that it can be accessed outside the docker container
    let ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);

    // Run the app
    let listener = tokio::net::TcpListener::bind(ip).await.unwrap();
    tracing::info!("Listening {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
