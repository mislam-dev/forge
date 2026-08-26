use crate::{modules::auth::token::JwtClaims, shared::error::AppError};
use axum::extract::FromRequestParts;
use std::marker::PhantomData;

pub trait AuthPolicyTrait: Send + Sync + 'static {
    fn check(claims: &JwtClaims) -> bool;
}

pub struct Guard<P>(pub JwtClaims, pub PhantomData<P>);

impl<P> Guard<P> {
    pub fn claims(self) -> JwtClaims {
        self.0
    }
}

impl<S, P> FromRequestParts<S> for Guard<P>
where
    S: Send + Sync,
    P: AuthPolicyTrait,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let claims = JwtClaims::from_request_parts(parts, state).await?;

        if !P::check(&claims) {
            return Err(AppError::Forbidden("Forbidden".to_string()));
        }
        Ok(Guard(claims, PhantomData))
    }
}

// policies
