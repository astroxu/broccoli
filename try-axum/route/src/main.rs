use axum::{
    extract::{Form,Path},
    http::{HeaderMap,StatusCode},
    response::Html,
    routing::get,
    Router,
};
use axum::routing::Route;

use serde::Deserialize;

/// form
#[derive(Deserialize)]
pub struct EditUser{
    pub id : i32,
    pub username:String,
    pub email:String,
}

/// db model
pub struct UserModel{
    pub id:i32,
    pub username:String,
    pub email:String,
}

/// edit user
async fn edit_user(Path(id):Path<i32>)->Html<String>{
    let model = UserModel{
        id,
        username:"zhangsan".to_string(),
        email:"zhangsan@email.com".to_string(),
    };

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              修改用户-AXUM中文网
            </title>
          </head>
          <body>
          <form method="post" action="/edit_user/{}">
          <input type="hidden" name="id" value="{}">
          <div>
            <label>用户名</label>
            <input type="text" name="username" value="{}">
          </div>
          <div>
            <label>Email</label>
            <input type="email" name="email" value="{}">
          </div>
          <div>
            <button type="submit">提交</button>
          </div>
          </form>
          </body>
          </html>
        "#,model.id,model.id,model.username,model.email
    );

    Html(html)
}

/// edit user action
async fn edit_user_action(Form(frm):Form<EditUser>) ->Html<String>{
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              修改用户-AXUM中文网
            </title>
          </head>
          <body>
            <h1>修改成功！</h1>
            <p>修改后的用户资料：</p>
            <div>ID: {} </div>
            <div>用户名: {} </div>
            <div>Email: {} </div>
          </body>
          </html>"#,frm.id,frm.username,frm.email
    );

    Html(html)
}

/// news index
async fn news_index()->&'static str{
    "new index"
}

/// news detail
async fn news_detail(Path(id):Path<i32>)->String{
    format!("new detail {}",id)
}

/// news comments
async fn news_comments(Path(id):Path<i32>)->String{
    format!("new comments {}",id)
}

/// redirect
async fn redirect()->(StatusCode,HeaderMap,()){
    let mut headers= HeaderMap::new();
    headers.insert(
        axum::http::header::LOCATION,
        "https://www.bing.com".parse().unwrap(),
    );
    (StatusCode::FOUND,headers,())
}

#[tokio::main]
async fn main() {
    let news_route = Router::new()
        .route("/",get(news_index))
        .route("/detail/:id",get(news_detail))
        .route("/comments/:id",get(news_comments));

    let app = Router::new()
        .route("/edit_user/:id",get(edit_user).post(edit_user_action))
        .nest("/news",news_route)
        .route("/go",get(redirect));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await.unwrap();
}
