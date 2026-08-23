use super::dto::request::{PermissionCreateDto, PermissionUpdateDto};
use super::entities::permissions::{
    ActiveModel as PermissionActiveModel, Column as PermissionColumn, Entity as PermissionEntity,
    Model as PermissionModel,
};
use crate::shared::error::AppError;
use crate::shared::pagination::{PaginatedResponse, PaginationParams};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use sea_orm::{PaginatorTrait, QueryOrder};
use uuid::Uuid;

pub struct PermissionsRepository;

impl PermissionsRepository {
    pub async fn find(
        db: &DatabaseConnection,
        params: &PaginationParams,
    ) -> Result<PaginatedResponse<PermissionModel>, AppError> {
        let current_page = params.page.saturating_sub(1);
        let limit = params.per_page;

        let paginator = PermissionEntity::find()
            .order_by_asc(PermissionColumn::CreatedAt)
            .paginate(db, limit);

        let total_pages = paginator.num_pages().await?;
        let total_items = paginator.num_items().await?;
        let items = paginator.fetch_page(current_page).await?;

        Ok(PaginatedResponse {
            page: params.page,
            per_page: params.per_page,
            total: total_items,
            total_pages,
            data: items,
        })
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<PermissionModel>, AppError> {
        PermissionEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn find_by_value(
        db: &DatabaseConnection,
        value: &String,
    ) -> Result<Option<PermissionModel>, AppError> {
        let condition = Condition::all().add(PermissionColumn::Value.eq(value));
        PermissionEntity::find()
            .filter(condition)
            .one(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn create(
        db: &DatabaseConnection,
        dto: PermissionCreateDto,
    ) -> Result<PermissionModel, AppError> {
        let active_model = PermissionActiveModel {
            key: Set(dto.key),
            value: Set(dto.value),
            description: Set(dto.descriptions),
            ..Default::default()
        };
        active_model.insert(db).await.map_err(AppError::Database)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        perm_data: PermissionUpdateDto,
    ) -> Result<PermissionModel, AppError> {
        let existing_perm = Self::find_by_id(db, id)
            .await?
            .ok_or(AppError::NotFound("Permission not found!".to_string()))?;

        let mut active_perm: PermissionActiveModel = existing_perm.into();

        if let Some(key) = perm_data.key {
            active_perm.key = Set(key);
        }

        if let Some(value) = perm_data.value {
            active_perm.value = Set(value);
        }

        if let Some(descriptions) = perm_data.descriptions {
            active_perm.description = Set(Some(descriptions));
        }

        active_perm.update(db).await.map_err(AppError::Database)
    }

    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
        let result = PermissionEntity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(AppError::Database)?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Permission not found!".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_find_by_id_empty_db() {
        let db = setup_mock_db();
        let id = Uuid::new_v4();
        let result = PermissionsRepository::find_by_id(&db, id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_by_value_empty_db() {
        let db = setup_mock_db();
        let result = PermissionsRepository::find_by_value(&db, &"create-user".to_string()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_not_found() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([sea_orm::MockExecResult {
                last_insert_id: 0,
                rows_affected: 0,
            }])
            .into_connection();
        let id = Uuid::new_v4();
        let result = PermissionsRepository::remove(&db, id).await;
        assert!(result.is_err());
    }
}
