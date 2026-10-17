use super::error::{AppError, codes};
use axum::{Json, extract::FromRequest};
use serde::de::DeserializeOwned;
use uuid::Uuid;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, Copy, Default)]
pub struct JsonValidate<T>(pub T);
impl<S, T> FromRequest<S> for JsonValidate<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|err| AppError::bad_request(codes::system::BAD_REQUEST, err.body_text()))?;
        value
            .validate()
            .map_err(|err| AppError::validation(codes::system::VALIDATION_ERROR, err))?;

        Ok(JsonValidate(value))
    }
}

pub fn validate_uuid_format(id: &str) -> Result<(), ValidationError> {
    if Uuid::parse_str(id).is_ok() {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_uuid"))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PathValidate<T>(pub T);

impl<S, T> axum::extract::FromRequestParts<S> for PathValidate<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Send,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(axum::extract::Path(value)) => Ok(PathValidate(value)),
            Err(err) => Err(AppError::bad_request(
                codes::system::VALIDATION_ERROR,
                format!("Failed to parse path parameter: {}", err),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_uuid_format() {
        assert!(validate_uuid_format("a1b2c3d4-e89b-12d3-a456-426614174000").is_ok());
        assert!(validate_uuid_format("invalid-uuid").is_err());
    }
}
