use askama::Template;
use axum::{
    extract::{Extension, Form},
    response::Html,
    routing, AddExtensionLayer, Router,
};
use serde::Deserialize;

/// hcaptcha verify mod
mod hcaptcha_verify {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize)]
    pub struct VerifyRequest {
        pub secret: String,
        pub response: String,
    }

    #[derive(Deserialize)]
    pub struct VerifyResponse {
        pub success: bool,
    }

    pub async fn verify(response: String, secret: String) -> Result<bool, String> {
        let req = VerifyRequest { secret, response };
        let client = reqwest::Client::new();
        let res = client
            .post("https://hcaptcha.com/siteverify")
            .form(&req)
            .send()
            .await
            .map_err(|err| err.to_string())?;
        let res = res.text().await.map_err(|err| err.to_string())?;
        let res: VerifyResponse = serde_json::from_str(&res).map_err(|err| err.to_string())?;
        Ok(res.success)
    }
}

/// HCAPTCHA  Site Key
/// 10000000-ffff-ffff-ffff-000000000001 test
const HCAPTCHA_SITE_KEY: &str = "10000000-ffff-ffff-ffff-000000000001";
///  HCAPTCHA  Secret
/// 0x0000000000000000000000000000000000000000 test
const HCAPTCHA_SECRET_KEY: &str = "0x0000000000000000000000000000000000000000";

#[derive(Template)]
#[template(path = "feed.html")]
pub struct FeedTemplate {
    pub site_key: String,
}

#[derive(Template)]
#[template(path = "feed_action.html")]
pub struct FeedActionTemplate {
    pub feed: Feed,
}

pub struct Feed {
    pub nickname: String,
    pub email: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct SubmitFeed {
    pub nickname: String,
    pub email: String,
    pub message: String,
    pub hcaptcha_response: String,
}

#[derive(Clone)]
pub struct HCaptchaConfig {
    pub site_key: String,
    pub secret: String,
}

/// feed
async fn feed(Extension(cfg): Extension<HCaptchaConfig>) -> Result<Html<String>, String> {
    let tpl = FeedTemplate {
        site_key: cfg.site_key,
    };
    let html = tpl.render().map_err(|err| err.to_string())?;
    Ok(Html(html))
}

/// feed_action
async fn feed_action(
    Extension(cfg): Extension<HCaptchaConfig>,
    Form(frm): Form<SubmitFeed>,
) -> Result<Html<String>, String> {
    let result = hcaptcha_verify::verify(frm.hcaptcha_response, cfg.secret).await?;
    if !result {
        return Err("Please verify you hcaptcha".to_string());
    }

    let tpl = FeedActionTemplate {
        feed: Feed {
            nickname: frm.nickname,
            email: frm.email,
            message: frm.message,
        },
    };

    let html = tpl.render().map_err(|err| err.to_string())?;
    Ok(Html(html))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/feed", routing::get(feed).post(feed_action))
        .layer(AddExtensionLayer::new(HCaptchaConfig {
            site_key: String::from(HCAPTCHA_SITE_KEY),
            secret: String::from(HCAPTCHA_SECRET_KEY),
        }));

    axum::Server::bind(&"127.0.0.1:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
