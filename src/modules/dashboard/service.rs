use sea_orm::*;
use uuid::Uuid;

use super::dto::{
    OrgDashboardResponse, SystemDashboardResponse, UserDashboardResponse,
};
use crate::modules::organization::members::repository::OrganizationMembersRepository;
use crate::modules::organization::orgs::repository::OrganizationRepository;
use crate::modules::organization::permissions::role::OrgRole;
use crate::modules::organization::permissions::service::OrgPermissionsService;
use crate::shared::error::AppError;

pub struct DashboardService;

impl DashboardService {
    pub async fn get_org_dashboard(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        org_id: Uuid,
    ) -> Result<OrgDashboardResponse, AppError> {
        let org = OrganizationRepository::find_by_id(db, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        if !is_system_admin {
            OrgPermissionsService::verify_org_role(
                db,
                org.id,
                requester_id,
                OrgRole::Viewer,
                is_system_admin,
            )
            .await?;
        }

        let members_count = OrganizationMembersRepository::count_members(db, org_id).await?;

        let projects_count = 0u64;
        let teams_count = 0u64;
        let deployments_count = 0u64;
        let active_deployments_count = 0u64;
        let success_rate = 1.0;

        Ok(OrgDashboardResponse {
            org_id,
            members_count,
            projects_count,
            teams_count,
            deployments_count,
            success_rate,
            active_deployments_count,
            recent_deployments: vec![],
        })
    }

    pub async fn get_user_dashboard(
        db: &DatabaseConnection,
        requester_id: Uuid,
    ) -> Result<UserDashboardResponse, AppError> {
        let org_memberships_count = OrganizationMembersRepository::find_by_user_id(db, requester_id)
            .await?
            .len() as u64;

        Ok(UserDashboardResponse {
            assigned_projects_count: 0,
            deployments_triggered_count: 0,
            org_memberships_count,
            recent_activity: vec![],
        })
    }

    pub async fn get_system_dashboard(
        db: &DatabaseConnection,
        _requester_id: Uuid,
        is_system_admin: bool,
    ) -> Result<SystemDashboardResponse, AppError> {
        if !is_system_admin {
            return Err(AppError::Forbidden(
                "Only System Admins can access system dashboard".to_string(),
            ));
        }

        let total_organizations = OrganizationRepository::count_all(db).await?;
        let total_users = 0u64;
        let total_projects = 0u64;
        let total_deployments = 0u64;

        Ok(SystemDashboardResponse {
            total_organizations,
            total_users,
            total_projects,
            total_deployments,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_get_org_dashboard_not_found() {
        let db = setup_mock_db();
        let result = DashboardService::get_org_dashboard(&db, Uuid::new_v4(), false, Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_system_dashboard_forbidden_for_non_admin() {
        let db = setup_mock_db();
        let result = DashboardService::get_system_dashboard(&db, Uuid::new_v4(), false).await;
        assert!(result.is_err());
    }
}
