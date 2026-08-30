use super::super::entities::organization_members::{
    ActiveModel as MemberActiveModel, Column as MemberColumn, Entity as MemberEntity,
    Model as MemberModel,
};
use super::super::entities::sea_orm_active_enums::OrganizationMemberRole;
use crate::shared::error::AppError;
use sea_orm::*;
use uuid::Uuid;

pub struct OrganizationMembersRepository;

impl OrganizationMembersRepository {
    pub async fn find_members_by_org_id(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<Vec<MemberModel>, AppError> {
        let members = MemberEntity::find()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .all(db)
            .await?;

        Ok(members)
    }

    pub async fn find_member(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<MemberModel>, AppError> {
        MemberEntity::find()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .filter(MemberColumn::UserId.eq(user_id))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn count_owners(db: &DatabaseConnection, org_id: Uuid) -> Result<u64, AppError> {
        MemberEntity::find()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .filter(MemberColumn::Role.eq(OrganizationMemberRole::Owner))
            .count(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn count_members(db: &DatabaseConnection, org_id: Uuid) -> Result<u64, AppError> {
        MemberEntity::find()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .count(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<MemberModel>, AppError> {
        MemberEntity::find()
            .filter(MemberColumn::UserId.eq(user_id))
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn add_member(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
        role: OrganizationMemberRole,
    ) -> Result<MemberModel, AppError> {
        Self::add_member_with_txn(db, org_id, user_id, role).await
    }

    pub async fn add_member_with_txn<C>(
        db: &C,
        org_id: Uuid,
        user_id: Uuid,
        role: OrganizationMemberRole,
    ) -> Result<MemberModel, AppError>
    where
        C: ConnectionTrait,
    {
        let active_model = MemberActiveModel {
            organization_id: Set(org_id),
            user_id: Set(user_id),
            role: Set(Some(role)),
            joined_at: Set(chrono::Utc::now().into()),
        };
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_member_role(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
        new_role: OrganizationMemberRole,
    ) -> Result<MemberModel, AppError> {
        let member = Self::find_member(db, org_id, user_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Member not found in organization".to_string()))?;

        let mut active: MemberActiveModel = member.into();
        active.role = Set(Some(new_role));
        active.update(db).await.map_err(AppError::from)
    }

    pub async fn remove_member(
        db: &DatabaseConnection,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<u64, AppError> {
        let res = MemberEntity::delete_many()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .filter(MemberColumn::UserId.eq(user_id))
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
        let org_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let result = OrganizationMembersRepository::find_member(&db, org_id, user_id).await;
        assert!(result.is_err());
    }
}
