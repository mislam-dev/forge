use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::role::OrgRole;
use crate::modules::organization::members::repository::OrganizationMembersRepository;
use crate::shared::error::AppError;

pub struct OrgPermissionsService;

impl OrgPermissionsService {
    pub async fn resolve_org_role(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<OrgRole>, AppError> {
        let member = OrganizationMembersRepository::find_member(db, org_id, user_id).await?;
        Ok(member.and_then(|m| m.role))
    }

    pub async fn verify_org_role(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
        min_role: OrgRole,
        is_system_admin: bool,
    ) -> Result<OrgRole, AppError> {
        if is_system_admin {
            return Ok(OrgRole::Admin);
        }

        let role = Self::resolve_org_role(db, org_id, user_id)
            .await?
            .ok_or_else(|| {
                AppError::Forbidden("You are not a member of this organization".to_string())
            })?;

        if role < min_role {
            return Err(AppError::Forbidden(format!(
                "Insufficient organization permissions. Required: {}, Actual: {}",
                min_role, role
            )));
        }

        Ok(role)
    }

    // latest
    pub async fn find_role(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<OrgRole>, AppError> {
        let member = OrganizationMembersRepository::find_member(db, org_id, user_id).await?;
        Ok(member.and_then(|m| m.role))
    }

    pub async fn match_roles(
        required_roles: Vec<OrgRole>,
        user_roles: Vec<String>,
    ) -> Result<(), AppError> {
        let user_resolved_roles = OrgRole::resolve_roles_hierarchy(user_roles);

        for role in required_roles {
            if !user_resolved_roles.contains(&role) {
                return Err(AppError::Forbidden(
                    "Insufficient organization permissions.".to_string(),
                ));
            }
        }

        Ok(())
    }

    pub async fn validate(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
        required_roles: Vec<OrgRole>,
    ) -> Result<(), AppError> {
        let role = Self::find_role(db, org_id, user_id).await?;

        let role = role.ok_or_else(|| {
            AppError::Forbidden("You are not a member of this organization".to_string())
        })?;

        let _r = Self::match_roles(required_roles, vec![role.to_string()]).await?;

        Ok(())
    }

    pub fn enforce_rename_permission(role: OrgRole) -> Result<(), AppError> {
        if role != OrgRole::Owner {
            return Err(AppError::Forbidden(
                "Access denied. Only the Organization Owner can rename an organization."
                    .to_string(),
            ));
        }
        Ok(())
    }

    pub fn enforce_delete_permission(role: OrgRole, is_system_admin: bool) -> Result<(), AppError> {
        if !is_system_admin && role != OrgRole::Owner {
            return Err(AppError::Forbidden(
                "Access denied. Only the Organization Owner can delete an organization."
                    .to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_resolve_org_role_empty_db() {
        let db = setup_mock_db();
        let org_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let res = OrgPermissionsService::resolve_org_role(&db, org_id, user_id).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_verify_org_role_system_admin_bypasses() {
        let db = setup_mock_db();
        let org_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let role =
            OrgPermissionsService::verify_org_role(&db, org_id, user_id, OrgRole::Owner, true)
                .await
                .unwrap();
        assert_eq!(role, OrgRole::Owner);
    }

    #[test]
    fn test_enforce_rename_permission() {
        assert!(OrgPermissionsService::enforce_rename_permission(OrgRole::Owner).is_ok());
        assert!(OrgPermissionsService::enforce_rename_permission(OrgRole::Admin).is_err());
        assert!(OrgPermissionsService::enforce_rename_permission(OrgRole::Editor).is_err());
        assert!(OrgPermissionsService::enforce_rename_permission(OrgRole::Viewer).is_err());
    }

    #[test]
    fn test_enforce_delete_permission() {
        assert!(OrgPermissionsService::enforce_delete_permission(OrgRole::Owner, false).is_ok());
        assert!(OrgPermissionsService::enforce_delete_permission(OrgRole::Admin, true).is_ok());
        assert!(OrgPermissionsService::enforce_delete_permission(OrgRole::Admin, false).is_err());
        assert!(OrgPermissionsService::enforce_delete_permission(OrgRole::Viewer, false).is_err());
    }
}
