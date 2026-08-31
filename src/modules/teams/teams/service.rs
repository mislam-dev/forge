use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::super::members::repository::TeamMembersRepository;
use super::dto::{CreateTeamDTO, TeamResponse, UpdateTeamDTO};
use super::entities::team::ActiveModel as TeamActiveModel;
use super::repository::TeamsRepository;
use crate::modules::teams::members::dto::AddTeamMemberRequest;
use crate::shared::error::AppError;

pub struct TeamsService;

impl TeamsService {
    pub async fn create_team(
        db: &DatabaseConnection,
        requester_id: Uuid,
        dto: CreateTeamDTO,
    ) -> Result<TeamResponse, AppError> {
        if (TeamsRepository::find_by_org_and_name(db, dto.organization_id, &dto.name).await?)
            .is_some()
        {
            return Err(AppError::Conflict(format!(
                "Team with name '{}' already exists in this organization",
                dto.name
            )));
        }

        let team = TeamsRepository::create_team(db, dto).await?;

        // todo: call team member service

        let _ = TeamMembersRepository::add_member(
            db,
            team.id,
            AddTeamMemberRequest {
                user_id: requester_id,
                role: "admin".to_string(),
            },
        )
        .await?;

        Ok(TeamResponse::from_model(team))
    }

    pub async fn list_teams(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<Vec<TeamResponse>, AppError> {
        let teams = TeamsRepository::find_by_org_id(db, org_id).await?;

        let responses = teams.into_iter().map(TeamResponse::from_model).collect();

        Ok(responses)
    }

    pub async fn get_team_by_id(
        db: &DatabaseConnection,
        team_id: Uuid,
    ) -> Result<TeamResponse, AppError> {
        let team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        Ok(TeamResponse::from_model(team))
    }

    pub async fn update_team(
        db: &DatabaseConnection,
        team_id: Uuid,
        req: UpdateTeamDTO,
    ) -> Result<TeamResponse, AppError> {
        let team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        let mut active_model: TeamActiveModel = team.into();
        let now = Utc::now().into();
        active_model.updated_at = Set(now);

        if let Some(new_name) = req.name {
            if let Some(org_id) = active_model.organization_id.clone().unwrap() {
                if let Some(existing) =
                    TeamsRepository::find_by_org_and_name(db, org_id, &new_name).await?
                {
                    if existing.id != team_id {
                        return Err(AppError::Conflict(format!(
                            "Team with name '{}' already exists in this organization",
                            new_name
                        )));
                    }
                }
            }
            active_model.name = Set(new_name);
        }

        if let Some(desc) = req.descriptions {
            active_model.descriptions = Set(Some(desc));
        }

        let updated = TeamsRepository::update_team(db, active_model).await?;

        Ok(TeamResponse::from_model(updated))
    }

    pub async fn delete_team(db: &DatabaseConnection, team_id: Uuid) -> Result<(), AppError> {
        let _team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        TeamsRepository::delete_team(db, team_id).await?;
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
    async fn test_get_team_not_found() {
        let db = setup_mock_db();
        let team_id = Uuid::new_v4();

        let result = TeamsService::get_team_by_id(&db, team_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_team_not_found() {
        let db = setup_mock_db();
        let team_id = Uuid::new_v4();
        let result = TeamsService::delete_team(&db, team_id).await;
        assert!(result.is_err());
    }
}
