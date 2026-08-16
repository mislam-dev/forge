use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Configuration Error: {0}")]
    Config(String),

    #[error("Invalid UUID: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Authentication Error: {0}")]
    Unauthorized(String),

    #[error("Validation Error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Database(err) => {
                tracing::error!(error = %err, "Database error");
                let body = Json(json!({
                  "message": "An internal server erorr occurred".to_string()
                }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::Uuid(err) => {
                tracing::warn!(error = %err, "Invalid UUID in request");

                let body = Json(json!({
                  "message":format!("Invalid ID format: {}", err),
                }));

                (StatusCode::BAD_REQUEST, body).into_response()
            }
            AppError::Config(msg) => {
                tracing::error!(message = %msg, "Configuration error");

                let body = Json(json!({
                  "message":"An internal server error occurred",
                }));

                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::Validation(errors) => {
                let mut err_map = serde_json::Map::new();

                for (field, field_errors) in errors.field_errors() {
                    let messages: Vec<String> = field_errors
                        .iter()
                        .map(|e| {
                            // Use custom message if provided, otherwise default to the rule code (e.g., "email")
                            e.message
                                .as_ref()
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| e.code.to_string())
                        })
                        .collect();

                    err_map.insert(field.to_string(), json!(messages));
                }

                let body = Json(json!({
                    "message": "Validation failed",
                    "errors": err_map // e.g., { "email": ["Invalid email format"], "password": ["Too short"] }
                }));
                tracing::warn!(details = ?err_map, "Request validation failed");
                (StatusCode::BAD_REQUEST, body).into_response()
            }

            AppError::Unauthorized(msg) => {
                tracing::warn!(message = %msg, "Unauthorized request");
                let body = Json(json!({
                  "message": msg,
                }));

                (StatusCode::UNAUTHORIZED, body).into_response()
            }
            AppError::NotFound(msg) => {
                tracing::warn!(message = %msg, "Resource not found");
                let body = Json(json!({
                  "message": msg,
                }));

                (StatusCode::NOT_FOUND, body).into_response()
            }
            AppError::BadRequest(msg) => {
                tracing::warn!(message = %msg, "Bad request");
                let body = Json(json!({
                  "message": msg,
                }));

                (StatusCode::BAD_REQUEST, body).into_response()
            }
            AppError::InternalServerError(msg) => {
                tracing::error!(message = %msg, "Internal server error");
                let body = Json(json!({
                  "message": "An internal server error occurred",
                }));

                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}
