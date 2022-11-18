use crate::Response;
use axum::body::{Bytes, Full};
use axum::response::IntoResponse;
use axum::Json;
use std::convert::Infallible;
use std::fmt::Display;
use std::fmt::Formatter;

/// error type
#[derive(Debug)]
pub enum AppErrorType {
    DbError,
    NotFound,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    fn code(&self) -> i32 {
        match self.error_type {
            AppErrorType::DbError => 1,
            AppErrorType::NotFound => 2,
        }
    }

    fn from_err(err: impl ToString, error_type: AppErrorType) -> Self {
        Self {
            message: None,
            cause: Some(err.to_string()),
            error_type,
        }
    }

    fn from_str(msg: &str, error_type: AppErrorType) -> Self {
        Self {
            message: Some(msg.to_string()),
            cause: None,
            error_type,
        }
    }

    pub fn db_err(err: impl ToString) -> Self {
        Self::from_err(err, AppErrorType::NotFound)
    }

    pub fn not_found() -> Self {
        Self::from_str("nou found record", AppErrorType::NotFound)
    }
}

impl std::error::Error for AppError {}
impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(err: deadpool_postgres::PoolError) -> Self {
        Self::db_err(err)
    }
}

impl From<tokio_postgres::Error> for AppError {
    fn from(err: tokio_postgres::Error) -> Self {
        Self::db_err(err)
    }
}

impl IntoResponse for AppError {
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> axum::http::Response<Self::Body> {
        let code = (&self).code();
        let msg = match self.message {
            Some(msg) => msg,
            None => "have error".to_string(),
        };
        let res: Response<()> = Response::err(code, msg);
        Json(res).into_response()
    }
}
