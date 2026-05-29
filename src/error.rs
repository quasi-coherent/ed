use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use ed_axum::models::ErrorBody;

/// All non-trivial outcomes that can flow out of a handler.
#[derive(Debug)]
pub enum AppError {
    Db(anyhow::Error),
    Client(anyhow::Error),
    NotFound,
    InvalidInput(String),
    Utf8(std::str::Utf8Error),
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InvalidInput(_) => StatusCode::UNPROCESSABLE_ENTITY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn body(&self) -> ErrorBody {
        let msg = match self {
            AppError::NotFound => "not found".to_string(),
            AppError::InvalidInput(m) => m.clone(),
            AppError::Db(e) | AppError::Client(e) => e.to_string(),
            AppError::Utf8(e) => e.to_string(),
        };
        ErrorBody { error: msg }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body().error)
    }
}

impl std::error::Error for AppError {}

impl From<std::str::Utf8Error> for AppError {
    fn from(e: std::str::Utf8Error) -> Self {
        AppError::Utf8(e)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status();
        let body = self.body();
        if status == StatusCode::NOT_FOUND {
            (status, ()).into_response()
        } else {
            (status, Json(body)).into_response()
        }
    }
}
