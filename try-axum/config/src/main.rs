use axum::{extract::Extension, routing, AddExtensionLayer, Router};
use dotenv::dotenv;
use serde::Deserialize;
use std::os::macos::raw::stat;

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct RedisConfig {
    pub dsn: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
    pub redis: RedisConfig,
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}

#[derive(Clone)]
pub struct AppState {
    pub pool: deadpool_postgres::Pool,
    pub rdc: redis::Client,
}

/// get pg client
async fn try_pg(Extension(state): Extension<AppState>) -> Result<&'static str, String> {
    let _client: deadpool_postgres::Client =
        state.pool.get().await.map_err(|err| err.to_string())?;
    Ok("Successfully got database client from postgresql pool in AppState")
}

/// get redis link
async fn try_redis(Extension(state): Extension<AppState>) -> Result<&'static str, String> {
    let _conn = state
        .rdc
        .get_async_connection()
        .await
        .map_err(|err| err.to_string())?;
    Ok("Successfully got async connection via redis client in AppState")
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cfg = Config::from_env().expect("init config failed");

    let pool = cfg
        .pg
        .create_pool(tokio_postgres::NoTls)
        .expect("create postgres connection pool failed");
    let rdc = redis::Client::open(cfg.redis.dsn).expect("create redis connection failed");

    let app = Router::new()
        .route("/pg", routing::get(try_pg))
        .route("/rds", routing::get(try_redis))
        .layer(AddExtensionLayer::new(AppState { pool, rdc }));

    axum::Server::bind(&cfg.web.addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
