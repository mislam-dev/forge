use axum::{Json, extract::FromRequest};
use serde::de::DeserializeOwned;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::shared::error::AppError;

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
            .map_err(|err| AppError::BadRequest(err.body_text()))?;
        value
            .validate()
            .map_err(|err| AppError::Validation(err.into()))?;

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
