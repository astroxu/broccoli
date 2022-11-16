use axum::routing::get;
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
    cfg.user = Some("ritllXXX".to_string());
    cfg.password = Some("rNJWKAbFAsra01_mgpGUOPBMcxY7XXX".to_string());
    cfg.dbname = Some("ritllXXX".to_string());
    cfg.host = Some("lucky.db.elephantsql.com".to_string());
    cfg.port = Some(5432);
    cfg
}

///  get db link
async fn get_client() -> Result<deadpool_postgres::Client, String> {
    let pool = get_cfg()
        .create_pool(tokio_postgres::NoTls)
        .map_err(|err| err.to_string())?;
    pool.get().await.map_err(|err| err.to_string())
}

/// insert
async fn insert(Path(username): Path<String>) -> Result<&'static str, String> {
    let create_user = CreateAccount {
        username,
        balance: 0,
    };
    let client = get_client().await?;
    let stmt = client
        .prepare("INSERT INTO account (username,balance) VALUES ($1,$2)")
        .await
        .map_err(|err| err.to_string())?;
    let rows = client
        .execute(&stmt, &[&create_user.username, &create_user.balance])
        .await
        .map_err(|err| err.to_string())?;
    if rows < 1 {
        return Err("Insert account failed".to_string());
    }
    Ok("Successfully insert account")
}

/// update
async fn update(Path((id, balance)): Path<(i32, i32)>) -> Result<&'static str, String> {
    let client = get_client().await?;
    let stmt = client
        .prepare("UPDATE account SET balance = $1 WHERE id=$2")
        .await
        .map_err(|err| err.to_string())?;
    let rows = client
        .execute(&stmt, &[&balance, &id])
        .await
        .map_err(|err| err.to_string())?;
    if rows < 1 {
        return Err("UPDATE account failed".to_string());
    }
    Ok("Successfully update account")
}

/// delete
async fn delete(Path(id): Path<i32>) -> Result<&'static str, String> {
    let client = get_client().await?;
    let stmt = client
        .prepare("DELETE FROM account WHERE id=$1")
        .await
        .map_err(|err| err.to_string())?;
    let rows = client
        .execute(&stmt, &[&id])
        .await
        .map_err(|err| err.to_string())?;
    if rows < 1 {
        return Err("Delete account failed".to_string());
    }
    Ok("Successfully delete account")
}

/// list
async fn list() -> Result<String, String> {
    let client = get_client().await?;
    let stmt = client
        .prepare("SELECT id,username,balance FROM account ORDER BY id DESC")
        .await
        .map_err(|err| err.to_string())?;
    let account_list = client
        .query(&stmt, &[])
        .await
        .map_err(|err| err.to_string())?
        .iter()
        .map(|row| Account::from_row_ref(&row).unwrap())
        .collect::<Vec<Account>>();

    let mut output = Vec::with_capacity(account_list.len());
    for account in account_list.iter() {
        output.push(format!("{:?}", account));
    }

    Ok(output.join("\n"))
}

/// find
async fn find(Path(id): Path<i32>) -> Result<String, String> {
    let client = get_client().await?;
    let stmt = client
        .prepare("SELECT id,username,balance FROM account WHERE id=$1 ORDER BY id DESC LIMIT 1")
        .await
        .map_err(|err| err.to_string())?;
    let account = client
        .query(&stmt, &[&id])
        .await
        .map_err(|err| err.to_string())?
        .iter()
        .map(|row| Account::from_row_ref(&row).unwrap())
        .collect::<Vec<Account>>()
        .pop()
        .ok_or(format!("Couldn't find account #{}", id))?;

    Ok(format!("{:?}", account))
}

/// transfer
async fn transfer(
    Path((from_id, to_id, balance)): Path<(i32, i32, i32)>,
) -> Result<&'static str, String> {
    let mut client = get_client().await?;
    let tx = client.transaction().await.map_err(|err| err.to_string())?;

    // out
    let stmt = tx
        .prepare("UPDATE account SET balance=balance-$1 WHERE id=$2 AND balance>=$1")
        .await
        .map_err(|err| err.to_string())?;
    match tx.execute(&stmt, &[&balance, &from_id]).await {
        Ok(_rows) if _rows > 0 => {}
        _ => {
            tx.rollback().await.map_err(|err| err.to_string())?;
            return Err("Step 1 failed".to_string());
        }
    };

    // out
    let stmt = tx
        .prepare("UPDATE account SET balance=balance+$1 WHERE id=$2")
        .await
        .map_err(|err| err.to_string())?;
    match tx.execute(&stmt, &[&balance, &to_id]).await {
        Ok(_rows) if _rows > 0 => {}
        _ => {
            tx.rollback().await.map_err(|err| err.to_string())?;
            return Err("Step 2 failed".to_string());
        }
    };

    // commit
    tx.commit().await.map_err(|err| err.to_string())?;
    Ok("Successfully transfer")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(list))
        .route("/find/:id", routing::get(find))
        .route("/insert/:username", routing::get(insert))
        .route("/update/:id/:balance", routing::get(update))
        .route("/delete/:id", routing::get(delete))
        .route("/transfer/:from_id/:to_id/:balance", get(transfer));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    println!("Hello, world!");
}
