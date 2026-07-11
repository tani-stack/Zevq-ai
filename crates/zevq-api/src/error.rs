use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::BadRequest(m) => (StatusCode::BAD_REQUEST, m.clone()),
            Self::NotFound(m) => (StatusCode::NOT_FOUND, m.clone()),
            Self::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m.clone()),
        };
        let body = Json(json!({
            "error": msg,
            "code": status.as_u16()
        }));
        (status, body).into_response()
    }
}
