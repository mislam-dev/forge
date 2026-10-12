use crate::{config::AppConfig, shared::error::{codes, AppError}};
use chrono::{Duration, Utc};
use jsonwebtoken::{Header, TokenData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AccessTokenDto {
    pub user_id: Uuid,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub iat: usize,
    pub exp: usize,
}

pub struct AccessTokenService;

impl AccessTokenService {
    pub fn generate(data: AccessTokenDto) -> Result<String, AppError> {
        let config = AppConfig::load().map_err(|_| {
            AppError::InternalServerError("[AuthToken]:Failed to load config".to_string())
        })?;
        let secret = &config.secrets.jwt_secret;

        let now = Utc::now();
        let exp: usize =
            (now + Duration::seconds(config.secrets.jwt_expiry_seconds)).timestamp() as usize;

        let claims = AccessTokenClaims {
            sub: data.user_id,
            email: data.email.to_owned(),
            roles: data.roles,
            permissions: data.permissions,
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
                "Failed to generate access_token | JWT encoding failed: {}",
                err
            ))
        })?;

        Ok(token)
    }

    pub fn verify(token: &str) -> Result<AccessTokenClaims, AppError> {
        let config = AppConfig::load().map_err(|_| {
            AppError::InternalServerError("[AuthToken]:Failed to load config".to_string())
        })?;
        let secret = &config.secrets.jwt_secret;

        let token_data: TokenData<AccessTokenClaims> = jsonwebtoken::decode(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::default(),
        )
        .map_err(|err| AppError::Config(format!("JWT decoding failed: {}", err)))?;

        let now = Utc::now();
        let exp: usize = token_data.claims.exp;
        let iat = now.timestamp() as usize;

        if exp < iat {
            return Err(AppError::unauthorized(
                codes::auth::AUTH_004,
                "Token has expired",
            ));
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
    fn test_access_token_creation_and_verification() {
        setup_secret();
        let user_id = Uuid::new_v4();
        let email = "user@example.com".to_string();
        let payload = AccessTokenDto {
            user_id,
            email: email.clone(),
            roles: vec!["Admin".to_string()],
            permissions: vec!["write:users".to_string()],
        };

        let token =
            AccessTokenService::generate(payload).expect("Access token creation should succeed");
        assert!(!token.is_empty());

        let claims = AccessTokenService::verify(&token).expect("Token verification should succeed");
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.roles, vec!["Admin".to_string()]);
        assert_eq!(claims.permissions, vec!["write:users".to_string()]);
    }
}
