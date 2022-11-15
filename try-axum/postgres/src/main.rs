use axum::{extract::Path, routing, Router};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper, Debug)]
#[pg_mapper(table = "account")]
pub struct Account {
    pub id: i32,
    pub username: String,
    pub balance: i32,
}

pub struct CreateAccount {
    pub username: String,
    pub balance: i32,
}

/// db config
fn get_cfg() -> deadpool_postgres::Config {
    let mut cfg = deadpool_postgres::Config::new();
    cfg.user = Some("ritlluxg".to_string());
    cfg.password = Some("rNJWKAbFAsra01_mgpGUOPBMcxY7rOhW".to_string());
    cfg.db = Some("ritlluxg".to_string());
    cfg.host = Some("lucky.db.elephantsql.com/ritlluxg".to_string());
    cfg.port = Some(5432);
    cfg
}

//  get db link
//async fn get_client

fn main() {
    println!("Hello, world!");
}
