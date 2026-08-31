use sea_orm::*;
use uuid::Uuid;

use super::dto::{AddTeamMemberRequest, TeamMemberResponse, UpdateTeamMemberRoleRequest};
use super::entities::team_member::ActiveModel as TeamMemberActiveModel;
use super::repository::TeamMembersRepository;
use super::role::TeamRole;
use crate::modules::teams::teams::repository::TeamsRepository;
use crate::modules::users::repository::UserRepository;
use crate::shared::error::AppError;

pub struct TeamMembersService;

impl TeamMembersService {
    pub async fn add_member(
        db: &DatabaseConnection,
        team_id: Uuid,
        dto: AddTeamMemberRequest,
    ) -> Result<TeamMemberResponse, AppError> {
        let _team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        // Check if user is already a member of the team
        if (TeamMembersRepository::find_member(db, team_id, dto.user_id).await?).is_some() {
            return Err(AppError::Conflict(
                "User is already a member of this team".to_string(),
            ));
        }

        let member = TeamMembersRepository::add_member(db, team_id, dto).await?;
        let user = UserRepository::find_by_id(db, member.user_id).await?;

        Ok(TeamMemberResponse::from_model(member, user))
    }

    pub async fn list_members(
        db: &DatabaseConnection,
        team_id: Uuid,
    ) -> Result<Vec<TeamMemberResponse>, AppError> {
        let _team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        let members_with_users =
            TeamMembersRepository::find_members_by_team_id(db, team_id).await?;
        let mut responses = Vec::with_capacity(members_with_users.len());

        for (member, user) in members_with_users {
            responses.push(TeamMemberResponse::from_model(member, user));
        }

        Ok(responses)
    }

    pub async fn update_member_role(
        db: &DatabaseConnection,

        team_id: Uuid,
        target_user_id: Uuid,
        req: UpdateTeamMemberRoleRequest,
    ) -> Result<TeamMemberResponse, AppError> {
        let _team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        let member = TeamMembersRepository::find_member(db, team_id, target_user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Member not found in team".to_string()))?;

        let role: TeamRole = req.role.parse().map_err(AppError::BadRequest)?;

        let mut active_model: TeamMemberActiveModel = member.into();
        active_model.role = Set(role.as_str().to_string());

        let updated_member = TeamMembersRepository::update_member(db, active_model).await?;
        let user = UserRepository::find_by_id(db, target_user_id).await?;

        Ok(TeamMemberResponse::from_model(updated_member, user))
    }

    pub async fn remove_member(
        db: &DatabaseConnection,
        team_id: Uuid,
        target_user_id: Uuid,
    ) -> Result<(), AppError> {
        let _team = TeamsRepository::find_by_id(db, team_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Team not found".to_string()))?;

        let member = TeamMembersRepository::find_member(db, team_id, target_user_id).await?;
        if member.is_none() {
            return Err(AppError::NotFound("Member not found in team".to_string()));
        }

        TeamMembersRepository::remove_member(db, team_id, target_user_id).await?;
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
    async fn test_list_members_team_not_found() {
        let db = setup_mock_db();
        let team_id = Uuid::new_v4();
        let result = TeamMembersService::list_members(&db, team_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_member_team_not_found() {
        let db = setup_mock_db();
        let team_id = Uuid::new_v4();
        let target_user_id = Uuid::new_v4();
        let result = TeamMembersService::remove_member(&db, team_id, target_user_id).await;
        assert!(result.is_err());
    }
}
