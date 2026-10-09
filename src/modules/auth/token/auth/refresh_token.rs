use crate::{config::AppConfig, shared::error::AppError};
use chrono::{Duration, Utc};
use jsonwebtoken::{Header, TokenData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct RefreshTokenDto {
    pub user_id: Uuid,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: Uuid,
    pub email: String,
    pub iat: usize,
    pub exp: usize,
}

pub struct RefreshTokenService;

impl RefreshTokenService {
    pub fn generate(data: RefreshTokenDto) -> Result<String, AppError> {
        let config = AppConfig::load().map_err(|_| {
            AppError::InternalServerError("[AuthToken]:Failed to load config".to_string())
        })?;
        let secret = &config.secrets.jwt_secret;

        let now = Utc::now();
        let exp: usize =
            (now + Duration::days(config.secrets.refresh_token_expiry_days)).timestamp() as usize;

        let claims = RefreshTokenClaims {
            sub: data.user_id,
            email: data.email.to_owned(),
            exp,
            iat: now.timestamp() as usize,
        };
        let token = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(|err| {
            AppError::InternalServerError(format!(
                "Failed to generate refresh_token | JWT encoding failed: {}",
                err
            ))
        })?;

        Ok(token)
    }

    pub fn verify(token: &str) -> Result<RefreshTokenClaims, AppError> {
        let config = AppConfig::load().map_err(|_| {
            AppError::InternalServerError("[AuthToken]:Failed to load config".to_string())
        })?;
        let secret = &config.secrets.jwt_secret;

        let token_data: TokenData<RefreshTokenClaims> = jsonwebtoken::decode(
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_secret() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test_secret_key_12345_67890_super_secret");
        }
    }

    #[test]
    fn test_refresh_token_creation_and_verification() {
        setup_secret();
        let user_id = Uuid::new_v4();
        let email = "refresh@example.com".to_string();
        let payload = RefreshTokenDto {
            user_id,
            email: email.clone(),
        };

        let token =
            RefreshTokenService::generate(payload).expect("Refresh token creation should succeed");
        assert!(!token.is_empty());

        let claims =
            RefreshTokenService::verify(&token).expect("Token verification should succeed");
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_verify_invalid_token_format() {
        setup_secret();
        let result = RefreshTokenService::verify("malformed.jwt.token");
        assert!(result.is_err());
    }
}
