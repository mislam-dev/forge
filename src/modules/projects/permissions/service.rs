use sea_orm::*;
use uuid::Uuid;

use super::super::assignments::entities::project_member::{
    Column as MemberColumn, Entity as MemberEntity,
};
use super::super::assignments::entities::project_team::{
    Column as ProjectTeamColumn, Entity as ProjectTeamEntity,
};
use super::super::projects::repository::ProjectsRepository;
use super::role::ProjectRole;
use crate::modules::organization::permissions::role::OrgRole;
use crate::modules::organization::permissions::service::OrgPermissionsService;
use crate::modules::teams::members::entities::team_member::{
    Column as TeamMemberColumn, Entity as TeamMemberEntity,
};
use crate::shared::error::AppError;

pub struct ProjectPermissionsService;

impl ProjectPermissionsService {
    pub async fn resolve_project_role(
        db: &DatabaseConnection,
        project_id: Uuid,
        user_id: Uuid,
        org_id: Option<Uuid>,
        is_system_admin: bool,
    ) -> Result<Option<ProjectRole>, AppError> {
        if is_system_admin {
            return Ok(Some(ProjectRole::Owner));
        }

        // Check Org role inheritance (Org Owner -> Project Owner, Org Admin -> Project Admin)
        if let Some(org_id) = org_id {
            if let Some(org_role) =
                OrgPermissionsService::resolve_org_role(db, org_id, user_id).await?
            {
                match org_role {
                    OrgRole::Owner => return Ok(Some(ProjectRole::Owner)),
                    OrgRole::Admin => return Ok(Some(ProjectRole::Admin)),
                    _ => {}
                }
            }
        }

        // Fetch project to check owner_id
        if let Some(project) = ProjectsRepository::find_any_by_id(db, project_id).await? {
            if project.owner_id == user_id {
                return Ok(Some(ProjectRole::Owner));
            }
        }

        // Check project_members junction table
        if let Some(member) = MemberEntity::find()
            .filter(MemberColumn::ProjectId.eq(project_id))
            .filter(MemberColumn::UserId.eq(user_id))
            .one(db)
            .await?
        {
            if let Ok(role) = member.role.parse::<ProjectRole>() {
                return Ok(Some(role));
            }
        }

        // Check project_teams & team_members junction tables
        let project_teams = ProjectTeamEntity::find()
            .filter(ProjectTeamColumn::ProjectId.eq(project_id))
            .all(db)
            .await?;

        if !project_teams.is_empty() {
            let team_ids: Vec<Uuid> = project_teams.into_iter().map(|pt| pt.team_id).collect();
            let team_member = TeamMemberEntity::find()
                .filter(TeamMemberColumn::TeamId.is_in(team_ids))
                .filter(TeamMemberColumn::UserId.eq(user_id))
                .one(db)
                .await?;

            if team_member.is_some() {
                return Ok(Some(ProjectRole::Developer));
            }
        }

        Ok(None)
    }

    pub async fn verify_project_role(
        db: &DatabaseConnection,
        project_id: Uuid,
        user_id: Uuid,
        org_id: Option<Uuid>,
        is_system_admin: bool,
        required_role: ProjectRole,
    ) -> Result<ProjectRole, AppError> {
        let role =
            Self::resolve_project_role(db, project_id, user_id, org_id, is_system_admin).await?;

        match role {
            Some(r) if r >= required_role => Ok(r),
            Some(_) => Err(AppError::Forbidden(format!(
                "Requires minimum project role: {}",
                required_role
            ))),
            None => Err(AppError::Forbidden(
                "You do not have permission to access this project".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_system_admin_bypasses_project_role_checks() {
        let db = setup_mock_db();
        let role = ProjectPermissionsService::resolve_project_role(
            &db,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            true,
        )
        .await
        .unwrap();

        assert_eq!(role, Some(ProjectRole::Owner));
    }
}
