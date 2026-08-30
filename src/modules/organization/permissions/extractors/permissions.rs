use std::marker::PhantomData;

use axum::extract::{FromRef, FromRequestParts, Path};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app::state::AppState,
    modules::{
        auth::token::JwtClaims,
        organization::{permissions::role::OrgRole, service::OrgPermissionsService},
    },
    shared::error::AppError,
};

pub trait OrgRoleRequirements: Send + Sync + 'static {
    fn required_roles() -> Vec<OrgRole>;
}

#[derive(Deserialize, Debug)]
struct OrgsParams {
    id: Uuid,
}

pub struct RequireOrgRole<R: OrgRoleRequirements>(pub JwtClaims, pub PhantomData<R>);

impl<R: OrgRoleRequirements> RequireOrgRole<R> {
    pub fn into_claims(self) -> JwtClaims {
        self.0
    }
}

impl<S, R> FromRequestParts<S> for RequireOrgRole<R>
where
    S: Send + Sync,
    AppState: FromRef<S>,
    R: OrgRoleRequirements,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let claims = JwtClaims::from_request_parts(parts, state).await?;
        let required_roles = R::required_roles();
        let user_id = claims.sub;

        let app_state = AppState::from_ref(state);
        let db = &app_state.db;

        let Path(path_param) = Path::<OrgsParams>::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                AppError::BadRequest("Organization ID is required in URL path".to_string())
            })?;

        let org_id = path_param.id;

        let _r = OrgPermissionsService::validate(db, org_id, user_id, required_roles).await?;

        Ok(Self(claims, PhantomData))
    }
}
