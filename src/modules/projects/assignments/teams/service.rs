use sea_orm::*;
use uuid::Uuid;

use super::super::super::projects::repository::ProjectsRepository;
use super::dto::{AssignProjectTeamDTO, ProjectTeamResponse};
use super::repository::TeamRepository;
use crate::modules::teams::teams::repository::TeamsRepository;

use crate::shared::error::AppError;

pub struct ProjectAssignmentsService;

impl ProjectAssignmentsService {
    pub async fn assign_team(
        db: &DatabaseConnection,

        org_id: Uuid,
        project_id: Uuid,
        req: AssignProjectTeamDTO,
    ) -> Result<ProjectTeamResponse, AppError> {
        let _project = ProjectsRepository::find_by_id_with_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let team = TeamsRepository::find_by_id(db, req.team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        if team.organization_id != Some(org_id) {
            return Err(AppError::Forbidden(
                "Team does not belong to the project's organization".to_string(),
            ));
        }

        if (TeamRepository::find_team(db, project_id, req.team_id).await?).is_some() {
            return Err(AppError::Conflict(
                "Team is already assigned to this project".to_string(),
            ));
        }

        let project_team = TeamRepository::add_team(db, project_id, req.team_id).await?;
        Ok(ProjectTeamResponse::from_model(project_team, Some(team)))
    }

    pub async fn list_teams(
        db: &DatabaseConnection,
        org_id: Uuid,
        project_id: Uuid,
    ) -> Result<Vec<ProjectTeamResponse>, AppError> {
        let _project = ProjectsRepository::find_by_id_with_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let teams = TeamRepository::find_teams_by_project_id(db, project_id).await?;
        Ok(teams
            .into_iter()
            .map(|(pt, t)| ProjectTeamResponse::from_model(pt, t))
            .collect())
    }

    pub async fn remove_team(
        db: &DatabaseConnection,
        org_id: Uuid,
        project_id: Uuid,
        team_id: Uuid,
    ) -> Result<(), AppError> {
        let _project = ProjectsRepository::find_by_id_with_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        let project_team = TeamRepository::find_team(db, project_id, team_id).await?;
        if project_team.is_none() {
            return Err(AppError::NotFound(
                "Team is not assigned to this project".to_string(),
            ));
        }

        TeamRepository::remove_team(db, project_id, team_id).await?;
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
    async fn test_assign_team_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectAssignmentsService::assign_team(
            &db,
            Uuid::new_v4(),
            Uuid::new_v4(),
            AssignProjectTeamDTO {
                team_id: Uuid::new_v4(),
            },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_teams_project_not_found() {
        let db = setup_mock_db();
        let result =
            ProjectAssignmentsService::list_teams(&db, Uuid::new_v4(), Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_team_project_not_found() {
        let db = setup_mock_db();
        let result = ProjectAssignmentsService::remove_team(
            &db,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        )
        .await;
        assert!(result.is_err());
    }
}
