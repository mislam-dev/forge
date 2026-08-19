use sea_orm::*;
use uuid::Uuid;

use super::super::members::entities::team_member::{
    Column as TeamMemberColumn, Entity as TeamMemberEntity,
};
use super::entities::team::{
    ActiveModel as TeamActiveModel, Column as TeamColumn, Entity as TeamEntity, Model as TeamModel,
};
use crate::shared::error::AppError;

pub struct TeamsRepository;

impl TeamsRepository {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<TeamModel>, AppError> {
        TeamEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_org_id(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<Vec<TeamModel>, AppError> {
        TeamEntity::find()
            .filter(TeamColumn::OrganizationId.eq(org_id))
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_org_and_name(
        db: &DatabaseConnection,
        org_id: Uuid,
        name: &str,
    ) -> Result<Option<TeamModel>, AppError> {
        TeamEntity::find()
            .filter(TeamColumn::OrganizationId.eq(org_id))
            .filter(TeamColumn::Name.eq(name))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_teams_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<TeamModel>, AppError> {
        let memberships = TeamMemberEntity::find()
            .filter(TeamMemberColumn::UserId.eq(user_id))
            .all(db)
            .await?;

        let team_ids: Vec<Uuid> = memberships.iter().map(|m| m.team_id).collect();
        if team_ids.is_empty() {
            return Ok(vec![]);
        }

        TeamEntity::find()
            .filter(TeamColumn::Id.is_in(team_ids))
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn count_members(db: &DatabaseConnection, team_id: Uuid) -> Result<u64, AppError> {
        TeamMemberEntity::find()
            .filter(TeamMemberColumn::TeamId.eq(team_id))
            .count(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn create_team(
        db: &DatabaseConnection,
        active_model: TeamActiveModel,
    ) -> Result<TeamModel, AppError> {
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_team(
        db: &DatabaseConnection,
        active_model: TeamActiveModel,
    ) -> Result<TeamModel, AppError> {
        active_model.update(db).await.map_err(AppError::from)
    }

    pub async fn delete_team(db: &DatabaseConnection, id: Uuid) -> Result<u64, AppError> {
        let res = TeamEntity::delete_by_id(id).exec(db).await?;
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
    async fn test_find_by_id_empty_db() {
        let db = setup_mock_db();
        let result = TeamsRepository::find_by_id(&db, Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_by_org_id_empty_db() {
        let db = setup_mock_db();
        let result = TeamsRepository::find_by_org_id(&db, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
