use axum::{routing, Json, Router};
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

const REDIS_DSN: &str = "redis://127.0.0.1:6379/";

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
}

async fn set() -> Result<&'static str, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;
    conn.set("author", "sean")
        .await
        .map_err(|err| err.to_string())?;
    Ok("Successfully set")
}

async fn get() -> Result<String, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;
    let val = conn.get("author").await.map_err(|err| err.to_string())?;
    Ok(val)
}

async fn set_user() -> Result<&'static str, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;

    let user = UserInfo {
        id: 1,
        username: "sean".to_string(),
        email: "sean@email.com".to_string(),
    };
    let user = json!(user);
    conn.set("user", user.to_string())
        .await
        .map_err(|err| err.to_string())?;

    Ok("Successfully set user")
}

async fn get_user() -> Result<Json<UserInfo>, String> {
    let client = Client::open(REDIS_DSN).map_err(|err| err.to_string())?;
    let mut conn = client
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;

    let val: String = conn.get("user").await.map_err(|err| err.to_string())?;
    let user: UserInfo = from_str(&val).map_err(|err| err.to_string())?;
    Ok(Json(user))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/set", routing::get(set))
        .route("/get", routing::get(get))
        .route("/set_user", routing::get(set_user))
        .route("/get_user", routing::get(get_user));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
