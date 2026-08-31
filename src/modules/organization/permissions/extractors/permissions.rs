use std::marker::PhantomData;

use axum::extract::{FromRef, FromRequestParts};
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

pub struct RequireOrgRole<R: OrgRoleRequirements>(pub JwtClaims, pub PhantomData<R>);

impl<R: OrgRoleRequirements> std::fmt::Debug for RequireOrgRole<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RequireOrgRole").field(&self.0).finish()
    }
}

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

        let org_id_header = parts.headers.get("Organization-ID").ok_or_else(|| {
            AppError::BadRequest("Organization-ID header is required".to_string())
        })?;

        let org_id_str = org_id_header.to_str().map_err(|_| {
            AppError::BadRequest("Invalid Organization-ID header encoding".to_string())
        })?;

        let org_id = Uuid::parse_str(org_id_str).map_err(|_| {
            AppError::BadRequest("Invalid Organization-ID header: must be a valid UUID".to_string())
        })?;

        let _r = OrgPermissionsService::validate(db, org_id, user_id, required_roles).await?;

        Ok(Self(claims, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use crate::modules::auth::token::{AuthTokenService, JwtPayload};
    use axum::http::Request;

    struct TestViewerRole;
    impl OrgRoleRequirements for TestViewerRole {
        fn required_roles() -> Vec<OrgRole> {
            vec![OrgRole::Viewer]
        }
    }

    fn setup_test_state() -> AppState {
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://user:pass@localhost:5432/testdb");
            std::env::set_var("JWT_SECRET", "test_secret_key_12345_67890_super_secret");
            std::env::set_var("MASTER_ENCRYPTION_KEY", "0123456789abcdef0123456789abcdef");
        }
        let config = AppConfig::load().expect("Test config should load");
        AppState::mock(config)
    }

    fn generate_valid_jwt() -> String {
        AuthTokenService::access(JwtPayload {
            user_id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            roles: vec!["User".to_string()],
            permissions: vec![],
        })
        .expect("JWT generation should succeed")
    }

    #[tokio::test]
    async fn test_require_org_role_missing_header() {
        let state = setup_test_state();
        let token = generate_valid_jwt();

        let req = Request::builder()
            .uri("/api/v1/organizations/test")
            .header("Authorization", format!("Bearer {}", token))
            .body(())
            .unwrap();

        let (mut parts, _) = req.into_parts();
        let result = RequireOrgRole::<TestViewerRole>::from_request_parts(&mut parts, &state).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert_eq!(msg, "Organization-ID header is required"),
            other => panic!("Expected BadRequest error, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_require_org_role_invalid_uuid_header() {
        let state = setup_test_state();
        let token = generate_valid_jwt();

        let req = Request::builder()
            .uri("/api/v1/organizations/test")
            .header("Authorization", format!("Bearer {}", token))
            .header("Organization-ID", "not-a-valid-uuid")
            .body(())
            .unwrap();

        let (mut parts, _) = req.into_parts();
        let result = RequireOrgRole::<TestViewerRole>::from_request_parts(&mut parts, &state).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => {
                assert_eq!(msg, "Invalid Organization-ID header: must be a valid UUID")
            }
            other => panic!("Expected BadRequest error, got: {:?}", other),
        }
    }
}
