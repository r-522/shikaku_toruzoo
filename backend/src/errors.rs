use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    ValidationError(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    Internal(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status, code, message) = match self {
            AppError::ValidationError(msg) => {
                (actix_web::http::StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg.clone())
            }
            AppError::Unauthorized(msg) => {
                (actix_web::http::StatusCode::UNAUTHORIZED, "UNAUTHORIZED", msg.clone())
            }
            AppError::Forbidden(msg) => {
                (actix_web::http::StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone())
            }
            AppError::NotFound(msg) => {
                (actix_web::http::StatusCode::NOT_FOUND, "NOT_FOUND", msg.clone())
            }
            AppError::Conflict(msg) => {
                (actix_web::http::StatusCode::CONFLICT, "CONFLICT", msg.clone())
            }
            AppError::Internal(msg) => {
                log::error!("Internal error: {}", msg);
                (
                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "サーバー内部エラーが発生しました".to_string(),
                )
            }
        };

        HttpResponse::build(status).json(serde_json::json!({
            "error": message,
            "code": code,
        }))
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Internal(format!("HTTP client error: {}", err))
    }
}
