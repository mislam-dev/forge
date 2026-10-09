use crate::{modules::auth::token::AccessTokenClaims, shared::error::AppError};
use axum::extract::FromRequestParts;
use std::marker::PhantomData;

pub trait AuthPolicyTrait: Send + Sync + 'static {
    fn check(claims: &AccessTokenClaims) -> bool;
}

pub struct Guard<P>(pub AccessTokenClaims, pub PhantomData<P>);

impl<P> Guard<P> {
    pub fn claims(self) -> AccessTokenClaims {
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
        let claims = AccessTokenClaims::from_request_parts(parts, state).await?;

        if !P::check(&claims) {
            return Err(AppError::Forbidden("Forbidden".to_string()));
        }
        Ok(Guard(claims, PhantomData))
    }
}

// policies
