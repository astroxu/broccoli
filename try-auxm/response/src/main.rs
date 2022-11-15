//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-form
//! ```
//!
//!

use axum::handler::Handler;
use axum::http::header;
use axum::http::header::HeaderName;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Form, // State,
    },
    http::{HeaderMap, HeaderValue, StatusCode},
    response::Html,
    routing::get,
    Json, Router,
};
use errors::AppError;
use serde::Serialize;

pub mod errors;

/// 纯文本 &str
async fn str_response() -> &'static str {
    "hello"
}

/// 纯文本 string
async fn string_response() -> String {
    "hello string".to_string()
}

/// 状态码
async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

/// 响应头
async fn with_headers() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-powered"),
        HeaderValue::from_static("axum"),
    );

    (headers, "axum")
}

/// 响应头+状态码
async fn with_headers_and_status() -> (StatusCode, HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-powered"),
        HeaderValue::from_static("axum"),
    );

    (StatusCode::OK, headers, "axum")
}

/// Html
async fn html() -> Html<&'static str> {
    Html("hello <em>axum</em>")
}

/// JSON
async fn json() -> Json<serde_json::Value> {
    Json(serde_json::json!({"axum":"呵呵"}))
}

/// Result
async fn result() -> Result<&'static str, StatusCode> {
    let flag = false;
    if flag {
        Ok("hello")
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[derive(Serialize)]
struct Info {
    web_site: String,
    email: String,
    level: i32,
}

/// 自定义结构体
async fn info_struct() -> Json<Info> {
    let info = Info {
        web_site: "https://www.bing.com".to_string(),
        email: "seanx24@outlook.com".to_string(),
        level: 12,
    };

    Json(info)
}

/// 自定义错误
async fn app_error() -> Result<&'static str, AppError> {
    let flag = false;
    if flag {
        Ok("hello")
    } else {
        Err(AppError {
            message: "Opps!".to_string(),
        })
    }
}

/// 中文响应
async fn cn() -> (HeaderMap, &'static str) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("text/plain;charset=utf-8"),
    );

    (headers, "呵呵")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/str", get(str_response))
        .route("/string", get(string_response))
        .route("/404", get(not_found))
        .route("/with_headers", get(with_headers))
        .route("/with_headers_and_status", get(with_headers_and_status))
        .route("/html", get(html))
        .route("/json", get(json))
        .route("/result", get(result))
        .route("/info_struct", get(info_struct))
        .route("/app_error", get(app_error))
        .route("/cn", get(cn));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
