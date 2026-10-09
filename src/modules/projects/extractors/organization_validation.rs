use std::marker::PhantomData;

use crate::{
    app::state::AppState,
    modules::{
        auth::token::AccessTokenClaims,
        organization::{
            permissions::{extractors::OrgIdHeaderOptional, role::OrgRole},
            service::OrgPermissionsService,
        },
    },
    shared::error::AppError,
};
use axum::extract::{FromRef, FromRequestParts};
use uuid::Uuid;

trait OrgValidationRequiredRoles: Send + Sync + 'static {
    fn required_roles() -> Vec<OrgRole>;
}

#[derive(Debug)]
pub struct OrgValidationRequired<R>(pub AccessTokenClaims, pub Uuid, pub PhantomData<R>);

impl<R, S> FromRequestParts<S> for OrgValidationRequired<R>
where
    S: Send + Sync,
    AppState: FromRef<S>,
    R: OrgValidationRequiredRoles,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let claims = AccessTokenClaims::from_request_parts(parts, state).await?;
        let org_id = OrgIdHeaderOptional::from_request_parts(parts, state).await?;

        if org_id.0.is_none() {
            return Err(AppError::NotFound("Organization not found".to_string()));
        }
        let org_id = org_id.0.unwrap();
        let app_state = AppState::from_ref(state);
        let db = &app_state.db;
        let required_roles = R::required_roles();
        let _ = OrgPermissionsService::validate(db, org_id, claims.sub, required_roles).await?;

        Ok(Self(claims, org_id, PhantomData))
    }
}

pub struct OrgOnlyViewer;
impl OrgValidationRequiredRoles for OrgOnlyViewer {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Viewer]
    }
}

pub struct OrgOnlyAdmin;
impl OrgValidationRequiredRoles for OrgOnlyAdmin {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Admin]
    }
}

pub struct OrgOnlyOwner;
impl OrgValidationRequiredRoles for OrgOnlyOwner {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Owner]
    }
}
pub struct OrgOnlyEditor;
impl OrgValidationRequiredRoles for OrgOnlyEditor {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Editor]
    }
}

pub type RequiredOrgViewer = OrgValidationRequired<OrgOnlyViewer>;
pub type RequiredOrgAdmin = OrgValidationRequired<OrgOnlyAdmin>;
pub type RequiredOrgOwner = OrgValidationRequired<OrgOnlyOwner>;
pub type RequiredOrgEditor = OrgValidationRequired<OrgOnlyEditor>;
