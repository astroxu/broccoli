use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};

pub struct UserAgentInfo;

#[async_trait]
impl<B> FromRequest<B> for UserAgentInfo
where
    B: Send,
{
    type Rejection = (StatusCode, String);
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let user_agent = req
            .headers()
            .and_then(|headers| headers.get(axum::http::header::USER_AGENT))
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        tracing::debug!("this UserAgent is {:?}", user_agent);
        if !user_agent.contains("Firefox") {
            tracing::error!("noFirefox WS");
            return Err((
                StatusCode::BAD_REQUEST,
                "you mast use Firefox to visit this page".to_string(),
            ));
        }

        Ok(UserAgentInfo {})
    }
}
