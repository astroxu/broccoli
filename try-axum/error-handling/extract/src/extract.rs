use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, RequestParts},
    http::StatusCode,
    BoxError,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::borrow::Cow;

pub struct Json<T>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for Json<T>
where
    B: axum::body::HttpBody + Send,
    T: DeserializeOwned,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req).await {
            Ok(value) => Ok(Self(value.0)),
            Err(err) => {
                let body: Cow<'_, str> = match err {
                    JsonRejection::InvalidJsonBody(err) => format!("lost fields:{}", err).into(),
                    JsonRejection::MissingJsonContentType(err) => {
                        format!("please use JSON request: {}", err).into()
                    }
                    err => format!("err {}", err).into(),
                };
                Err((
                    StatusCode::BAD_REQUEST,
                    axum::Json(json!({ "error": body })),
                ))
            }
        }
    }
}
