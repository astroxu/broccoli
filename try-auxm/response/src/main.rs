//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-form
//! ```
//!
//!

use axum::http::header::HeaderName;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Form, // State,
    },
    http::{HeaderMap,HeaderValue,StatusCode},

    response::{Html},
    routing::get,
    Router,
    Json,
};
use axum::handler::Handler;
use errors::AppError;
use serde::Serialize;

pub mod errors;

/// 纯文本 &str
async fn string_response() -> &'static str {
    "hello"
}

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/str",get(str_response));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}