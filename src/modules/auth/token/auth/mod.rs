mod access_token;
mod refresh_token;

pub use access_token::{AccessTokenClaims, AccessTokenDto, AccessTokenService};
pub use refresh_token::{RefreshTokenClaims, RefreshTokenDto, RefreshTokenService};

// Backward-compatible types and service for integration tests
use crate::shared::error::AppError;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct JwtPayload {
    pub user_id: Uuid,
    pub email: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RefreshTokenPayload {
    pub user_id: Uuid,
    pub email: String,
}

pub struct AuthTokenService;

impl AuthTokenService {
    pub fn access(payload: JwtPayload) -> Result<String, AppError> {
        AccessTokenService::generate(AccessTokenDto {
            user_id: payload.user_id,
            email: payload.email,
            roles: payload.roles,
            permissions: payload.permissions,
        })
    }

    pub fn refresh(payload: RefreshTokenDto) -> Result<String, AppError> {
        RefreshTokenService::generate(payload)
    }

    pub fn verify(token: &str) -> Result<AccessTokenClaims, AppError> {
        AccessTokenService::verify(token).or_else(|_| {
            RefreshTokenService::verify(token).map(|c| AccessTokenClaims {
                sub: c.sub,
                email: c.email,
                roles: vec![],
                permissions: vec![],
                exp: c.exp,
                iat: c.iat,
            })
        })
    }
}


