use crate::shared::error::AppError;

use super::token::{AuthTokenService, JwtClaims};
use axum::http::header::AUTHORIZATION;
use axum::{extract::FromRequestParts, http::request::Parts};

impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::Unauthorized(
                "Missing Authorization header".to_string(),
            ))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized(
                "Invalid Authorization header format".to_string(),
            ))?;

        AuthTokenService::verify(token)
            .map_err(|_| AppError::Unauthorized("Unauthorized".to_string()))
    }
}
