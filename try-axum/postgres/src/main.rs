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



fn main() {
    println!("Hello, world!");
}
