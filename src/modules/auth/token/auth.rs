use crate::shared::error::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{Header, TokenData};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub email: String,
    pub role: Vec<String>,
    pub permissions: Vec<String>,
    pub iat: usize,
    pub exp: usize,
}

pub struct JwtPayload {
    pub user_id: Uuid,
    pub email: String,
    pub role: Vec<String>,
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
            role: data.role,
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
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| AppError::Config("JWT_SECRET must be set in .env".to_string()))?;

        let now = Utc::now();
        let exp: usize = (now + Duration::hours(24)).timestamp() as usize;
        let a = Self::create(data, exp, secret).map_err(|_| {
            AppError::InternalServerError("Failed to generate access_token".to_string())
        })?;

        Ok(a)
    }
    pub fn refresh(data: RefreshTokenPayload) -> Result<String, AppError> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| AppError::Config("JWT_SECRET must be set in .env".to_string()))?;

        let now = Utc::now();
        let exp: usize = (now + Duration::days(7)).timestamp() as usize;
        let a = Self::create(
            JwtPayload {
                user_id: data.user_id,
                email: data.email,
                role: vec![],
                permissions: vec![],
            },
            exp,
            secret,
        )
        .map_err(|_| {
            AppError::InternalServerError("Failed to generate refresh_token".to_string())
        })?;

        Ok(a)
    }

    pub fn verify(token: &str) -> Result<JwtClaims, AppError> {
        let secret = std::env::var("JWT_SECRET")
            .map_err(|_| AppError::Config("JWT_SECRET must be set in .env".to_string()))?;

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
