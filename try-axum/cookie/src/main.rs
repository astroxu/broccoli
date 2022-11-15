use axum::{
    extract::Form,
    http::{HeaderMap, StatusCode},
    response::Html,
    routing, Router,
};

use serde::Deserialize;

const COOKIE_NAME: &'static str = "username";

#[derive(Deserialize)]
pub struct UserLoginForm {
    pub username: String,
    pub password: String,
}

/// homepage
async fn user_center(headers: HeaderMap) -> Result<Html<String>, &'static str> {
    let cookies = headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .unwrap_or("".to_string());

    if cookies.is_empty() {
        return Err("NO COOKIE SETED!");
    }

    let mut logined_username: Option<String> = None;
    let cookies: Vec<&str> = cookies.split(';').collect();
    for cookie in cookies {
        let cookie_pair: Vec<&str> = cookie.split('=').collect();
        let cookie_name = cookie_pair[0].trim();
        let cookie_value = cookie_pair[1].trim();
        if cookie_name == COOKIE_NAME && !cookie_value.is_empty() {
            logined_username = Some(String::from(cookie_value));
            break;
        }
    }

    if logined_username.is_none() {
        return Err("COOKIE IS NONE!");
    }

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              用户中心-AXUM中文网
            </title>
          </head>
          <body>
          <p>你好，<strong>{}</strong>！你已成功登录。[<a href="/logout">退出登录</a>]
          </body>
          </html>
        "#,
        logined_username.unwrap()
    );

    Ok(Html(html))
}

/// login form
async fn user_login() -> Html<String> {
    let html = r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              用户登录-AXUM中文网
            </title>
          </head>
          <body>
          <form method="post" action="/login">
          <div>
            <label>用户名</label>
            <input type="text" name="username">
          </div>
          <div>
            <label>密码</label>
            <input type="password" name="password">
          </div>
          <div>
            <button type="submit">提交</button>
          </div>
          </form>
          </body>
          </html>
        "#
    .to_string();

    Html(html)
}

/// user login
async fn user_login_action(Form(frm): Form<UserLoginForm>) -> (StatusCode, HeaderMap, ()) {
    let mut headers = HeaderMap::new();
    if !(&frm.username == "zhangsan" && &frm.password == "zhangsan") {
        headers.insert(
            axum::http::header::LOCATION,
            "/login?msg=usernameorpassworderror".parse().unwrap(),
        );
    } else {
        let cookie = format!("{}={}", COOKIE_NAME, frm.username);
        headers.insert(
            axum::http::header::SET_COOKIE,
            cookie.as_str().parse().unwrap(),
        );
        headers.insert(axum::http::header::LOCATION, "/".parse().unwrap());
    }

    (StatusCode::FOUND, headers, ())
}

/// logout
async fn user_logout() -> (StatusCode, HeaderMap, ()) {
    let cookie = format!("{}=", COOKIE_NAME);
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        cookie.as_str().parse().unwrap(),
    );
    headers.insert(axum::http::header::LOCATION, "/login".parse().unwrap());
    (StatusCode::FOUND, headers, ())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(user_center))
        .route("/login", routing::get(user_login).post(user_login_action))
        .route("/logout", routing::get(user_logout));
    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
