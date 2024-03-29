use axum::error_handling::HandleErrorExt;
use axum::http::StatusCode;
use axum::routing::service_method_routing as service;
use axum::Router;
use tower_http::services::ServeDir;

#[allow(unused)]
async fn axum_fx_txt() -> String {
    std::fs::read_to_string("static/axum-rs.txt").unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new().nest(
        "/static",
        service::get(ServeDir::new("static")).handle_error(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("process error: {:?}", err),
            )
        }),
    );

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
