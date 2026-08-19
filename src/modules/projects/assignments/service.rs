use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::dto::{AssignProjectMemberRequest, AssignProjectTeamRequest, ProjectMemberResponse, ProjectTeamResponse};
use super::entities::project_member::ActiveModel as ProjectMemberActiveModel;
use super::entities::project_team::ActiveModel as ProjectTeamActiveModel;
use super::repository::ProjectAssignmentsRepository;
use super::super::permissions::role::ProjectRole;
use super::super::permissions::service::ProjectPermissionsService;
use super::super::projects::repository::ProjectsRepository;
use crate::modules::organization::permissions::service::OrgPermissionsService;
use crate::modules::teams::teams::repository::TeamsRepository;
use crate::modules::users::repository::UserRepository;
use crate::shared::error::AppError;

pub struct ProjectAssignmentsService;

impl ProjectAssignmentsService {
    pub async fn assign_member(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        req: AssignProjectMemberRequest,
    ) -> Result<ProjectMemberResponse, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Admin,
            )
            .await?;
        }

        // Verify target user is a member of the parent organization!
        let org_role = OrgPermissionsService::resolve_org_role(db, project.organization_id, req.user_id).await?;
        if org_role.is_none() {
            return Err(AppError::BadRequest(
                "Target user is not a member of the parent organization".to_string(),
            ));
        }

        if (ProjectAssignmentsRepository::find_member(db, project_id, req.user_id).await?).is_some() {
            return Err(AppError::Conflict(
                "User is already assigned to this project".to_string(),
            ));
        }

        let role: ProjectRole = req.role.parse().map_err(AppError::BadRequest)?;
        if role == ProjectRole::Owner {
            return Err(AppError::BadRequest(
                "Project Owner is determined by project ownership, not member assignment".to_string(),
            ));
        }

        let now = Utc::now().into();
        let active_model = ProjectMemberActiveModel {
            project_id: Set(project_id),
            user_id: Set(req.user_id),
            role: Set(role.as_str().to_string()),
            assigned_at: Set(now),
        };

        let member = ProjectAssignmentsRepository::add_member(db, active_model).await?;
        let user = UserRepository::find_by_id(db, req.user_id).await?;

        Ok(ProjectMemberResponse::from_model(member, user))
    }

    pub async fn list_members(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
    ) -> Result<Vec<ProjectMemberResponse>, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Viewer,
            )
            .await?;
        }

        let members = ProjectAssignmentsRepository::find_members_by_project_id(db, project_id).await?;
        Ok(members
            .into_iter()
            .map(|(m, u)| ProjectMemberResponse::from_model(m, u))
            .collect())
    }

    pub async fn remove_member(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        target_user_id: Uuid,
    ) -> Result<(), AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Admin,
            )
            .await?;
        }

        if project.owner_id == target_user_id {
            return Err(AppError::BadRequest(
                "Cannot remove the Project Owner from the project".to_string(),
            ));
        }

        let member = ProjectAssignmentsRepository::find_member(db, project_id, target_user_id).await?;
        if member.is_none() {
            return Err(AppError::NotFound("User is not assigned to this project".to_string()));
        }

        ProjectAssignmentsRepository::remove_member(db, project_id, target_user_id).await?;
        Ok(())
    }

    pub async fn assign_team(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        req: AssignProjectTeamRequest,
    ) -> Result<ProjectTeamResponse, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Admin,
            )
            .await?;
        }

        let team = TeamsRepository::find_by_id(db, req.team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        if team.organization_id != Some(project.organization_id) {
            return Err(AppError::BadRequest(
                "Team does not belong to the project's organization".to_string(),
            ));
        }

        if (ProjectAssignmentsRepository::find_team(db, project_id, req.team_id).await?).is_some() {
            return Err(AppError::Conflict(
                "Team is already assigned to this project".to_string(),
            ));
        }

        let now = Utc::now().into();
        let active_model = ProjectTeamActiveModel {
            project_id: Set(project_id),
            team_id: Set(req.team_id),
            assigned_at: Set(now),
        };

        let project_team = ProjectAssignmentsRepository::add_team(db, active_model).await?;
        Ok(ProjectTeamResponse::from_model(project_team, Some(team)))
    }

    pub async fn list_teams(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
    ) -> Result<Vec<ProjectTeamResponse>, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Viewer,
            )
            .await?;
        }

        let teams = ProjectAssignmentsRepository::find_teams_by_project_id(db, project_id).await?;
        Ok(teams
            .into_iter()
            .map(|(pt, t)| ProjectTeamResponse::from_model(pt, t))
            .collect())
    }

    pub async fn remove_team(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Admin,
            )
            .await?;
        }

        let project_team = ProjectAssignmentsRepository::find_team(db, project_id, team_id).await?;
        if project_team.is_none() {
            return Err(AppError::NotFound("Team is not assigned to this project".to_string()));
        }

        ProjectAssignmentsRepository::remove_team(db, project_id, team_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_list_members_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectAssignmentsService::list_members(&db, Uuid::new_v4(), false, Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_team_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectAssignmentsService::remove_team(&db, Uuid::new_v4(), false, Uuid::new_v4(), Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
