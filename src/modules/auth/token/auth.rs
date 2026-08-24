use crate::{config::AppConfig, shared::error::AppError};
use chrono::{Duration, Utc};
use jsonwebtoken::{Header, TokenData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub iat: usize,
    pub exp: usize,
}

pub struct JwtPayload {
    pub user_id: Uuid,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
pub struct RefreshTokenPayload {
    pub user_id: Uuid,
    pub email: String,
}
pub struct AuthTokenService;

impl AuthTokenService {
    fn create(data: JwtPayload, exp: usize, secret: String) -> Result<String, AppError> {
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let claims = JwtClaims {
            sub: data.user_id,
            email: data.email.to_owned(),
            roles: data.roles,
            permissions: data.permissions,
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

    pub fn access(data: JwtPayload) -> Result<String, AppError> {
        let config = AppConfig::load().unwrap();
        let secret = &config.secrets.jwt_secret;

        let now = Utc::now();
        let exp: usize =
            (now + Duration::seconds(config.secrets.jwt_expiry_seconds)).timestamp() as usize;
        let token = Self::create(data, exp, secret.to_string()).map_err(|_| {
            AppError::InternalServerError("Failed to generate access_token".to_string())
        })?;

        Ok(token)
    }
    pub fn refresh(data: RefreshTokenPayload) -> Result<String, AppError> {
        let config = AppConfig::load().unwrap();
        let secret = &config.secrets.jwt_secret;

        let now = Utc::now();
        let exp: usize =
            (now + Duration::days(config.secrets.refresh_token_expiry_days)).timestamp() as usize;
        let a = Self::create(
            JwtPayload {
                user_id: data.user_id,
                email: data.email,
                roles: vec![],
                permissions: vec![],
            },
            exp,
            secret.to_string(),
        )
        .map_err(|_| {
            AppError::InternalServerError("Failed to generate refresh_token".to_string())
        })?;

        Ok(a)
    }

    pub fn verify(token: &str) -> Result<JwtClaims, AppError> {
        let config = AppConfig::load().unwrap();
        let secret = &config.secrets.jwt_secret;

        let token_data: TokenData<JwtClaims> = jsonwebtoken::decode(
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
    fn test_access_token_creation_and_verification() {
        setup_secret();
        let user_id = Uuid::new_v4();
        let email = "user@example.com".to_string();
        let payload = JwtPayload {
            user_id,
            email: email.clone(),
            roles: vec!["Admin".to_string()],
            permissions: vec!["write:users".to_string()],
        };

        let token =
            AuthTokenService::access(payload).expect("Access token creation should succeed");
        assert!(!token.is_empty());

        let claims = AuthTokenService::verify(&token).expect("Token verification should succeed");
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
        assert_eq!(claims.roles, vec!["Admin".to_string()]);
        assert_eq!(claims.permissions, vec!["write:users".to_string()]);
    }

    #[test]
    fn test_refresh_token_creation_and_verification() {
        setup_secret();
        let user_id = Uuid::new_v4();
        let email = "refresh@example.com".to_string();
        let payload = RefreshTokenPayload {
            user_id,
            email: email.clone(),
        };

        let token =
            AuthTokenService::refresh(payload).expect("Refresh token creation should succeed");
        assert!(!token.is_empty());

        let claims = AuthTokenService::verify(&token).expect("Token verification should succeed");
        assert_eq!(claims.sub, user_id);
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_verify_invalid_token_format() {
        setup_secret();
        let result = AuthTokenService::verify("malformed.jwt.token");
        assert!(result.is_err());
    }
}
