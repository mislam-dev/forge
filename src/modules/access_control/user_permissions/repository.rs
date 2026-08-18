use super::entities::user_permissions::{
    ActiveModel as UserPermissionActiveModel, Column as UserPermissionColumn,
    Entity as UserPermissionEntity,
};
use crate::modules::access_control::permissions::entities::permissions::{
    Column as PermissionColumn, Entity as PermissionEntity, Model as PermissionModel,
};
use crate::shared::error::AppError;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

pub struct UserPermissionsRepository;

impl UserPermissionsRepository {
    pub async fn assign(
        db: &DatabaseConnection,
        user_id: Uuid,
        permission_ids: Vec<Uuid>,
    ) -> Result<(), AppError> {
        for perm_id in permission_ids {
            let active_model = UserPermissionActiveModel {
                user_id: Set(user_id),
                permission_id: Set(perm_id),
            };
            let _ = active_model.insert(db).await;
        }
        Ok(())
    }

    pub async fn remove(
        db: &DatabaseConnection,
        user_id: Uuid,
        permission_ids: Vec<Uuid>,
    ) -> Result<(), AppError> {
        let filter = Condition::all()
            .add(UserPermissionColumn::UserId.eq(user_id))
            .add(UserPermissionColumn::PermissionId.is_in(permission_ids));

        UserPermissionEntity::delete_many()
            .filter(filter)
            .exec(db)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn find_permissions_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<PermissionModel>, AppError> {
        let user_perms = UserPermissionEntity::find()
            .filter(UserPermissionColumn::UserId.eq(user_id))
            .all(db)
            .await
            .map_err(AppError::Database)?;

        let perm_ids: Vec<Uuid> = user_perms.into_iter().map(|up| up.permission_id).collect();

        if perm_ids.is_empty() {
            return Ok(vec![]);
        }

        PermissionEntity::find()
            .filter(PermissionColumn::Id.is_in(perm_ids))
            .all(db)
            .await
            .map_err(AppError::Database)
    }
}
