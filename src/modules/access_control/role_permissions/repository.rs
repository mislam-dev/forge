use super::entities::role_permissions::{
    ActiveModel as RolePermissionActiveModel, Column as RolePermissionColumn,
    Entity as RolePermissionEntity,
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

pub struct RolePermissionsRepository;

impl RolePermissionsRepository {
    pub async fn assign(
        db: &DatabaseConnection,
        role_id: Uuid,
        permission_ids: Vec<Uuid>,
    ) -> Result<(), AppError> {
        for perm_id in permission_ids {
            let active_model = RolePermissionActiveModel {
                role_id: Set(role_id),
                permission_id: Set(perm_id),
            };
            let _ = active_model.insert(db).await;
        }
        Ok(())
    }

    pub async fn remove(
        db: &DatabaseConnection,
        role_id: Uuid,
        permission_ids: Vec<Uuid>,
    ) -> Result<(), AppError> {
        let filter = Condition::all()
            .add(RolePermissionColumn::RoleId.eq(role_id))
            .add(RolePermissionColumn::PermissionId.is_in(permission_ids));

        RolePermissionEntity::delete_many()
            .filter(filter)
            .exec(db)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn find_permissions_by_role_id(
        db: &DatabaseConnection,
        role_id: Uuid,
    ) -> Result<Vec<PermissionModel>, AppError> {
        let role_perms = RolePermissionEntity::find()
            .filter(RolePermissionColumn::RoleId.eq(role_id))
            .all(db)
            .await
            .map_err(AppError::Database)?;

        let perm_ids: Vec<Uuid> = role_perms.into_iter().map(|rp| rp.permission_id).collect();

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
