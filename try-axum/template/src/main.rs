use askama::Template;
use axum::{response::Html, routing, Router};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub name: String,
}

async fn index() -> Result<Html<String>, String> {
    let name = String::from("axum");
    let tpl = IndexTemplate { name };
    let html = tpl.render().map_err(|err| err.to_string())?;
    Ok(Html(html))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(index));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
