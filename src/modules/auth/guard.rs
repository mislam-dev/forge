use super::token::{AccessTokenClaims, AccessTokenService};
use crate::shared::error::{AppError, codes};
use axum::http::header::AUTHORIZATION;
use axum::{extract::FromRequestParts, http::request::Parts};

impl<S> FromRequestParts<S> for AccessTokenClaims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                AppError::unauthorized(
                    codes::auth::AUTH_000,
                    "Missing Authorization header",
                )
            })?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
                AppError::unauthorized(
                    codes::auth::AUTH_000,
                    "Invalid Authorization header format",
                )
            })?;

        let jwt_claims = AccessTokenService::verify(token)
            .map_err(|_| AppError::unauthorized(codes::auth::AUTH_000, "Unauthorized"))?;
        Ok(jwt_claims)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use axum::extract::FromRequestParts;
    use axum::http::Request;

    #[tokio::test]
    async fn test_guard_missing_authorization_header() {
        let req = Request::builder().body(()).unwrap();
        let (mut parts, _) = req.into_parts();

        let result = AccessTokenClaims::from_request_parts(&mut parts, &()).await;
        assert!(result.is_err());
        match result {
            Err(AppError::Unauthorized(code, msg)) => {
                assert_eq!(code, codes::auth::AUTH_000);
                assert_eq!(msg, "Missing Authorization header");
            }
            _ => panic!("Expected Unauthorized error"),
        }
    }

    #[tokio::test]
    async fn test_guard_invalid_format() {
        let req = Request::builder()
            .header("Authorization", "Basic 12345")
            .body(())
            .unwrap();
        let (mut parts, _) = req.into_parts();

        let result = AccessTokenClaims::from_request_parts(&mut parts, &()).await;
        assert!(result.is_err());
        match result {
            Err(AppError::Unauthorized(code, msg)) => {
                assert_eq!(code, codes::auth::AUTH_000);
                assert_eq!(msg, "Invalid Authorization header format");
            }
            _ => panic!("Expected Unauthorized error"),
        }
    }
}

