use super::entities::roles::{
    ActiveModel as RoleActiveModel, Column as RoleColumn, Entity as RoleEntity, Model as RoleModel,
};
use crate::{
    modules::access_control::roles::dto::request::{RoleCreateDto, RoleUpdateDto},
    shared::error::AppError,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

pub struct RoleRepository;

impl RoleRepository {
    pub async fn find(db: &DatabaseConnection) -> Result<Vec<RoleModel>, AppError> {
        RoleEntity::find().all(db).await.map_err(AppError::Database)
    }
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<RoleModel>, AppError> {
        RoleEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn find_by_value(
        db: &DatabaseConnection,
        value: &String,
    ) -> Result<Option<RoleModel>, AppError> {
        let condition = Condition::all().add(RoleColumn::Value.eq(value));
        RoleEntity::find()
            .filter(condition)
            .one(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn create(
        db: &DatabaseConnection,
        dto: RoleCreateDto,
    ) -> Result<RoleModel, AppError> {
        let active_model = RoleActiveModel {
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
        role_data: RoleUpdateDto,
    ) -> Result<RoleModel, AppError> {
        let existing_role = Self::find_by_id(db, id)
            .await?
            .ok_or(AppError::NotFound("Role not found!".to_string()))?;

        let mut active_role: RoleActiveModel = existing_role.into();

        if let Some(key) = role_data.key {
            active_role.key = Set(key);
        }

        if let Some(value) = role_data.value {
            active_role.value = Set(value);
        }

        if let Some(descriptions) = role_data.descriptions {
            active_role.description = Set(Some(descriptions));
        }

        active_role.update(db).await.map_err(AppError::Database)
    }

    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
        let result = RoleEntity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(AppError::Database)?;

        if result.rows_affected == 0 {
            return Err(AppError::NotFound("Role not found!".to_string()));
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
        let result = RoleRepository::find_by_id(&db, id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_by_value_empty_db() {
        let db = setup_mock_db();
        let result = RoleRepository::find_by_value(&db, &"admin".to_string()).await;
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
        let result = RoleRepository::remove(&db, id).await;
        assert!(result.is_err());
    }
}

