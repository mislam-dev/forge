use super::dto::request::{PermissionCreateDto, PermissionUpdateDto};
use super::entities::permissions::{
    ActiveModel as PermissionActiveModel, Column as PermissionColumn,
    Entity as PermissionEntity, Model as PermissionModel,
};
use crate::shared::error::AppError;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

pub struct PermissionsRepository;

impl PermissionsRepository {
    pub async fn find(db: &DatabaseConnection) -> Result<Vec<PermissionModel>, AppError> {
        PermissionEntity::find()
            .all(db)
            .await
            .map_err(AppError::Database)
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
