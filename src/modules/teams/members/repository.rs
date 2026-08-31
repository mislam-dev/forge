use sea_orm::*;
use uuid::Uuid;

use super::entities::team_member::{
    ActiveModel as TeamMemberActiveModel, Column as TeamMemberColumn, Entity as TeamMemberEntity,
    Model as TeamMemberModel,
};
use crate::modules::teams::members::TeamRole;
use crate::modules::teams::members::dto::AddTeamMemberDTO;
use crate::shared::error::AppError;

pub struct TeamMembersRepository;

impl TeamMembersRepository {
    pub async fn find_member(
        db: &DatabaseConnection,
        team_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<TeamMemberModel>, AppError> {
        TeamMemberEntity::find()
            .filter(TeamMemberColumn::TeamId.eq(team_id))
            .filter(TeamMemberColumn::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_members_by_team_id(
        db: &DatabaseConnection,
        team_id: Uuid,
    ) -> Result<Vec<TeamMemberModel>, AppError> {
        TeamMemberEntity::find()
            .filter(TeamMemberColumn::TeamId.eq(team_id))
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn add_member(
        db: &DatabaseConnection,
        team_id: Uuid,
        dto: AddTeamMemberDTO,
    ) -> Result<TeamMemberModel, AppError> {
        let role: TeamRole = dto.role.parse().map_err(AppError::BadRequest)?;

        let active_model = TeamMemberActiveModel {
            team_id: Set(team_id),
            user_id: Set(dto.user_id),
            role: Set(role.as_str().to_string()),
            ..Default::default()
        };
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_member(
        db: &DatabaseConnection,
        active_model: TeamMemberActiveModel,
    ) -> Result<TeamMemberModel, AppError> {
        active_model.update(db).await.map_err(AppError::from)
    }

    pub async fn remove_member(
        db: &DatabaseConnection,
        team_id: Uuid,
        user_id: Uuid,
    ) -> Result<u64, AppError> {
        let res = TeamMemberEntity::delete_many()
            .filter(TeamMemberColumn::TeamId.eq(team_id))
            .filter(TeamMemberColumn::UserId.eq(user_id))
            .exec(db)
            .await?;
        Ok(res.rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_find_member_empty_db() {
        let db = setup_mock_db();
        let team_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let result = TeamMembersRepository::find_member(&db, team_id, user_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_members_by_team_id_empty_db() {
        let db = setup_mock_db();
        let team_id = Uuid::new_v4();
        let result = TeamMembersRepository::find_members_by_team_id(&db, team_id).await;
        assert!(result.is_err());
    }
}
