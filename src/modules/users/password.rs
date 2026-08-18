use crate::shared::error::AppError;
use bcrypt::{DEFAULT_COST, hash, verify};

pub struct PasswordService;

impl PasswordService {
    pub async fn hash(password: &str) -> Result<String, AppError> {
        let password_owned = password.to_owned();
        tokio::task::spawn_blocking(move || {
            hash(password_owned, DEFAULT_COST).map_err(|e| {
                AppError::InternalServerError(format!("Password hashing failed: {}", e))
            })
        })
        .await
        .map_err(|_| {
            AppError::InternalServerError("Failed to execute background task".to_string())
        })?
    }
    pub async fn verify(hash: &str, password: &str) -> Result<bool, AppError> {
        let has_owned = hash.to_owned();
        let password_owned = password.to_owned();
        tokio::task::spawn_blocking(move || {
            verify(password_owned, &has_owned)
                .map_err(|_| AppError::Unauthorized(format!("Unauthorized")))
        })
        .await
        .map_err(|_| {
            AppError::InternalServerError("Failed to execute background task".to_string())
        })?
    }
}
