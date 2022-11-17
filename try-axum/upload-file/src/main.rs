use axum::extract::Multipart;
use axum::handler::Handler;
use axum::{
    extract::{multipart, ContentLengthLimit},
    http::HeaderMap,
    response::Html,
    routing, Router,
};

const MAX_UPLOAD_SIZE: u64 = 1024 * 1024 * 10;

/// upload file
async fn upload_file() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
            <meta charset="utf-8">
                <title>上传文件</title>
            </head>
            <body>
                <form action="/upload" method="post" enctype="multipart/form-data">
                    <label>
                        上传文件：
                        <input type="file" name="axum_rs_file">
                    </label>
                    <button type="submit">上传文件</button>
                </form>
            </body>
        </html>
        "#,
    )
}

/// upload file action
async fn upload_file_action(
    ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, { MAX_UPLOAD_SIZE }>,
) -> Result<(HeaderMap, String), String> {
    if let Some(file) = multipart.next_field().await.unwrap() {
        let filename = file.file_name().unwrap().to_string();
        let data = file.bytes().await.unwrap();

        tokio::fs::write(&filename, &data)
            .await
            .map_err(|err| err.to_string())?;

        return cn(format!(
            "[Upload File] file name:{:?}, file size:{:?}",
            filename,
            data.len(),
        ))
        .await;
    }
    cn(String::from("no file upload")).await
}
/// cn
async fn cn(msg: String) -> Result<(HeaderMap, String), String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/plain;charset=utf-8".parse().unwrap(),
    );
    Ok((headers, msg))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/upload",
        routing::get(upload_file).post(upload_file_action),
    );

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    println!("Hello, world!");
}
