use std::collections::HashMap;

use super::app_error_data::AppErrorData;
use axum::{Json, http::StatusCode, response::IntoResponse};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Invalid UUID: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("Authentication Error: {0}")]
    Unauthorized(String, String),

    #[error("Forbidden: {0}")]
    Forbidden(String, String),

    #[error("Validation Error: {0}")]
    Validation(String, ValidationErrors),

    #[error("Not Found: {0}")]
    NotFound(String, String),

    #[error("Bad Request: {0}")]
    BadRequest(String, String),

    #[error("Bad Request with errors: {0}")]
    BadRequestWithErrors(String, String, Option<HashMap<String, Vec<String>>>),

    #[error("Conflict: {0}")]
    Conflict(String, String),

    #[error("database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Configuration Error: {0}")]
    Config(String),

    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
}

impl AppError {
    pub fn not_found(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::NotFound(code.into(), msg.into())
    }

    pub fn bad_request(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::BadRequest(code.into(), msg.into())
    }

    pub fn bad_request_with_errors(
        code: impl Into<String>,
        msg: impl Into<String>,
        errors: Option<HashMap<String, Vec<String>>>,
    ) -> Self {
        Self::BadRequestWithErrors(code.into(), msg.into(), errors)
    }

    pub fn unauthorized(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::Unauthorized(code.into(), msg.into())
    }

    pub fn forbidden(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::Forbidden(code.into(), msg.into())
    }

    pub fn conflict(code: impl Into<String>, msg: impl Into<String>) -> Self {
        Self::Conflict(code.into(), msg.into())
    }

    pub fn validation(code: impl Into<String>, errors: ValidationErrors) -> Self {
        Self::Validation(code.into(), errors)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let mut error_data = AppErrorData::new();
        let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;

        match self {
            AppError::Uuid(err) => {
                tracing::warn!(error = %err, "Invalid UUID in request");

                status_code = StatusCode::BAD_REQUEST;
                error_data = error_data
                    .message("Invalid ID".to_string())
                    .code(super::codes::system::BAD_REQUEST.to_string());
            }

            AppError::Validation(code, errors) => {
                error_data = error_data
                    .message("Validation failed for one or more request fields.".to_string())
                    .validation_error(errors)
                    .code(code);
                status_code = StatusCode::BAD_REQUEST;
                tracing::error!(errors = ?error_data.errors, "Request validation failed");
            }

            AppError::Unauthorized(code, msg) => {
                let message = if msg.is_empty() {
                    "You are unauthorized to perform this action!".to_string()
                } else {
                    msg.clone()
                };
                error_data = error_data.message(message).code(code);
                status_code = StatusCode::UNAUTHORIZED;
                tracing::warn!(message = %msg, "Unauthorized request");
            }
            AppError::Forbidden(code, msg) => {
                let message = if msg.is_empty() {
                    "You don't have permission to perform this action!".to_string()
                } else {
                    msg.clone()
                };
                error_data = error_data.message(message).code(code);
                status_code = StatusCode::FORBIDDEN;
                tracing::warn!(message = %msg, "Forbidden request");
            }
            AppError::NotFound(code, msg) => {
                let message = if msg.is_empty() {
                    "The requested resource could not be found.".to_string()
                } else {
                    msg.clone()
                };
                error_data = error_data.message(message).code(code);
                status_code = StatusCode::NOT_FOUND;
                tracing::warn!(message = %msg, "Resource not found");
            }
            AppError::BadRequest(code, msg) => {
                error_data = error_data.message(msg.clone()).code(code);
                status_code = StatusCode::BAD_REQUEST;
                tracing::warn!(message = %msg, "Bad request");
            }
            AppError::BadRequestWithErrors(code, msg, errors) => {
                error_data = error_data.message(msg.clone()).code(code);
                if let Some(errors) = errors {
                    error_data = error_data.errors(errors);
                }
                status_code = StatusCode::BAD_REQUEST;
                tracing::warn!(message = %msg, "Bad request");
            }
            AppError::Conflict(code, msg) => {
                let message = if msg.is_empty() {
                    "The request conflicts with the current state of the resource.".to_string()
                } else {
                    msg.clone()
                };
                error_data = error_data.message(message).code(code);
                status_code = StatusCode::CONFLICT;
                tracing::warn!(message = %msg, "Conflict resource state");
            }
            AppError::Database(err) => {
                tracing::error!(error = %err, "Database error");
            }
            AppError::Config(msg) => {
                tracing::error!(message = %msg, "Configuration error");
            }
            AppError::InternalServerError(msg) => {
                tracing::error!(message = %msg, "Internal server error");
            }
        };

        (status_code, Json(error_data.to_json())).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use validator::ValidationError;

    #[tokio::test]
    async fn test_not_found_error_into_response() {
        let err = AppError::not_found("USER_001", "User with ID 123 not found");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json["code"], "USER_001");
        assert_eq!(json["message"], "User with ID 123 not found");
        assert_eq!(json["is_error"], true);
    }

    #[tokio::test]
    async fn test_unauthorized_error_into_response() {
        let err = AppError::unauthorized("AUTH_000", "Invalid or expired JWT token");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json["code"], "AUTH_000");
        assert_eq!(json["message"], "Invalid or expired JWT token");
        assert_eq!(json["is_error"], true);
    }

    #[tokio::test]
    async fn test_bad_request_error_into_response() {
        let err = AppError::bad_request("PRJ_002", "Invalid query parameters");
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json["code"], "PRJ_002");
        assert_eq!(json["message"], "Invalid query parameters");
        assert_eq!(json["is_error"], true);
    }

    #[tokio::test]
    async fn test_config_error_into_response() {
        let err = AppError::Config("JWT_SECRET missing".to_string());
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json["code"], "INTERNAL_SERVER_ERROR");
        assert_eq!(json["message"], "An internal server error occurred");
        assert_eq!(json["is_error"], true);
    }

    #[tokio::test]
    async fn test_validation_error_into_response() {
        let mut errors = ValidationErrors::new();
        let mut custom_err = ValidationError::new("email_invalid");
        custom_err.message = Some("Must be a valid email address".into());
        errors.add("email", custom_err);

        let err = AppError::validation("VALIDATION_ERROR", errors);
        let response = err.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(json["code"], "VALIDATION_ERROR");
        assert_eq!(
            json["message"],
            "Validation failed for one or more request fields."
        );
        assert_eq!(json["errors"]["email"][0], "Must be a valid email address");
        assert_eq!(json["is_error"], true);
    }
}
