use sea_orm::*;
use uuid::Uuid;

use super::entities::team_member::{
    ActiveModel as TeamMemberActiveModel, Column as TeamMemberColumn, Entity as TeamMemberEntity,
    Model as TeamMemberModel,
};
use crate::modules::teams::members::TeamRole;
use crate::modules::teams::members::dto::AddTeamMemberRequest;
use crate::modules::users::entities::users::{
    Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
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
    ) -> Result<Vec<(TeamMemberModel, Option<UserModel>)>, AppError> {
        let members = TeamMemberEntity::find()
            .filter(TeamMemberColumn::TeamId.eq(team_id))
            .all(db)
            .await?;

        let user_ids: Vec<Uuid> = members.iter().map(|m| m.user_id).collect();

        let users = if !user_ids.is_empty() {
            UserEntity::find()
                .filter(UserColumn::Id.is_in(user_ids))
                .all(db)
                .await?
        } else {
            vec![]
        };

        let mut result = Vec::with_capacity(members.len());
        for m in members {
            let u = users.iter().find(|user| user.id == m.user_id).cloned();
            result.push((m, u));
        }

        Ok(result)
    }

    pub async fn add_member(
        db: &DatabaseConnection,
        team_id: Uuid,
        dto: AddTeamMemberRequest,
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
