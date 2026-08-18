use crate::shared::error::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{Header, TokenData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// password reset token

pub struct ResetTokenData {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetTokenClaims {
    pub sub: Uuid,
    pub iat: usize,
    pub exp: usize,
}

pub struct PasswordResetToken;

impl PasswordResetToken {
    fn create(data: ResetTokenData, exp: usize, secret: String) -> Result<String, AppError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let claims = ResetTokenClaims {
            sub: data.user_id,
            exp,
            iat,
        };
        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|err| AppError::Config(format!("JWT encoding failed: {}", err)))
    }

    pub fn token(data: ResetTokenData) -> Result<String, AppError> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| AppError::Config("JWT_SECRET must be set in .env".to_string()))?;

        let now = Utc::now();
        let exp: usize = (now + Duration::hours(24)).timestamp() as usize;
        let a = Self::create(data, exp, secret).map_err(|_| {
            AppError::InternalServerError("Failed to generate access_token".to_string())
        })?;

        Ok(a)
    }

    pub fn verify(token: &str) -> Result<ResetTokenClaims, AppError> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| AppError::Config("JWT_SECRET must be set in .env".to_string()))?;

        let token_data: TokenData<ResetTokenClaims> = jsonwebtoken::decode(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|err| AppError::Config(format!("JWT decoding failed: {}", err)))?;

        let now = Utc::now();
        let exp: usize = token_data.claims.exp;
        let iat = now.timestamp() as usize;

        if exp < iat {
            return Err(AppError::Unauthorized("Token has expired".to_string()));
        }

        Ok(token_data.claims)
    }
}
