use std::marker::PhantomData;

use crate::{
    app::state::AppState,
    modules::{
        auth::token::JwtClaims,
        organization::{
            permissions::{extractors::OrgIdHeaderOptional, role::OrgRole},
            service::OrgPermissionsService,
        },
    },
    shared::error::AppError,
};
use axum::extract::{FromRef, FromRequestParts};
use uuid::Uuid;

pub trait OrgValidationOptionalRequiredRoles: Send + Sync + 'static {
    fn required_roles() -> Vec<OrgRole>;
}

#[derive(Debug)]
pub struct OrgValidationOptional<R>(pub JwtClaims, pub Option<Uuid>, pub PhantomData<R>);

impl<R, S> FromRequestParts<S> for OrgValidationOptional<R>
where
    S: Send + Sync,
    AppState: FromRef<S>,
    R: OrgValidationOptionalRequiredRoles,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let claims = JwtClaims::from_request_parts(parts, state).await?;
        let org_id = OrgIdHeaderOptional::from_request_parts(parts, state).await?;

        if let Some(org_id) = org_id.0 {
            let app_state = AppState::from_ref(state);
            let db = &app_state.db;
            let required_roles = R::required_roles();
            let _ = OrgPermissionsService::validate(db, org_id, claims.sub, required_roles).await?;
        }

        Ok(Self(claims, org_id.0, PhantomData))
    }
}

pub struct OrgOnlyViewer;
impl OrgValidationOptionalRequiredRoles for OrgOnlyViewer {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Viewer]
    }
}

pub struct OrgOnlyAdmin;
impl OrgValidationOptionalRequiredRoles for OrgOnlyAdmin {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Admin]
    }
}

pub struct OrgOnlyOwner;
impl OrgValidationOptionalRequiredRoles for OrgOnlyOwner {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Owner]
    }
}
pub struct OrgOnlyEditor;
impl OrgValidationOptionalRequiredRoles for OrgOnlyEditor {
    fn required_roles() -> Vec<OrgRole> {
        vec![OrgRole::Editor]
    }
}

pub type OptionalOrgViewer = OrgValidationOptional<OrgOnlyViewer>;
pub type OptionalOrgAdmin = OrgValidationOptional<OrgOnlyAdmin>;
pub type OptionalOrgOwner = OrgValidationOptional<OrgOnlyOwner>;
pub type OptionalOrgEditor = OrgValidationOptional<OrgOnlyEditor>;
