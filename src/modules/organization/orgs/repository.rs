use sea_orm::*;
use uuid::Uuid;

use super::entities::organization::{
    ActiveModel as OrganizationActiveModel, Column as OrganizationColumn,
    Entity as OrganizationEntity, Model as OrganizationModel,
};
use crate::modules::organization::members::entities::organization_member::{
    Column as MemberColumn, Entity as MemberEntity,
};
use crate::shared::error::AppError;

pub struct OrganizationRepository;

impl OrganizationRepository {
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<OrganizationModel>, AppError> {
        OrganizationEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_slug(
        db: &DatabaseConnection,
        slug: &str,
    ) -> Result<Option<OrganizationModel>, AppError> {
        OrganizationEntity::find()
            .filter(OrganizationColumn::Slug.eq(slug))
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<OrganizationModel>, AppError> {
        OrganizationEntity::find()
            .order_by_asc(OrganizationColumn::Name)
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_user_organizations(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<OrganizationModel>, AppError> {
        let members = MemberEntity::find()
            .filter(MemberColumn::UserId.eq(user_id))
            .all(db)
            .await?;

        let org_ids: Vec<Uuid> = members.into_iter().map(|m| m.organization_id).collect();

        if org_ids.is_empty() {
            return Ok(vec![]);
        }

        OrganizationEntity::find()
            .filter(OrganizationColumn::Id.is_in(org_ids))
            .order_by_asc(OrganizationColumn::Name)
            .all(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn create(
        db: &DatabaseConnection,
        active_model: OrganizationActiveModel,
    ) -> Result<OrganizationModel, AppError> {
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn create_with_txn<C>(
        db: &C,
        active_model: OrganizationActiveModel,
    ) -> Result<OrganizationModel, AppError>
    where
        C: ConnectionTrait,
    {
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update(
        db: &DatabaseConnection,
        active_model: OrganizationActiveModel,
    ) -> Result<OrganizationModel, AppError> {
        active_model.update(db).await.map_err(AppError::from)
    }

    pub async fn delete(db: &DatabaseConnection, id: Uuid) -> Result<u64, AppError> {
        let res = OrganizationEntity::delete_by_id(id).exec(db).await?;
        Ok(res.rows_affected)
    }

    pub async fn find_owner_id(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<Option<Uuid>, AppError> {
        let owner_member = MemberEntity::find()
            .filter(MemberColumn::OrganizationId.eq(org_id))
            .filter(MemberColumn::Role.eq("owner"))
            .one(db)
            .await?;

        Ok(owner_member.map(|m| m.user_id))
    }

    pub async fn count_all(db: &DatabaseConnection) -> Result<u64, AppError> {
        OrganizationEntity::find()
            .count(db)
            .await
            .map_err(AppError::from)
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
        let id = Uuid::new_v4();
        let result = OrganizationRepository::find_by_id(&db, id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_by_slug_empty_db() {
        let db = setup_mock_db();
        let result = OrganizationRepository::find_by_slug(&db, "nonexistent").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_all_empty_db() {
        let db = setup_mock_db();
        let result = OrganizationRepository::find_all(&db).await;
        assert!(result.is_err());
    }
}
