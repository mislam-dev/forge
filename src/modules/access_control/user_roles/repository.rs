use super::entities::user_roles::{
    ActiveModel as UserRoleActiveModel, Column as UserRoleColumn, Entity as UserRoleEntity,
};
use crate::modules::access_control::roles::entities::roles::{
    Column as RoleColumn, Entity as RoleEntity, Model as RoleModel,
};
use crate::shared::error::AppError;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

pub struct UserRolesRepository;

impl UserRolesRepository {
    pub async fn assign(
        db: &DatabaseConnection,
        user_id: Uuid,
        role_ids: Vec<Uuid>,
    ) -> Result<(), AppError> {
        for role_id in role_ids {
            let active_model = UserRoleActiveModel {
                user_id: Set(user_id),
                role_id: Set(role_id),
            };
            let _ = active_model.insert(db).await;
        }
        Ok(())
    }

    pub async fn remove(
        db: &DatabaseConnection,
        user_id: Uuid,
        role_ids: Vec<Uuid>,
    ) -> Result<(), AppError> {
        let filter = Condition::all()
            .add(UserRoleColumn::UserId.eq(user_id))
            .add(UserRoleColumn::RoleId.is_in(role_ids));

        UserRoleEntity::delete_many()
            .filter(filter)
            .exec(db)
            .await
            .map_err(AppError::Database)?;

        Ok(())
    }

    pub async fn find_roles_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<RoleModel>, AppError> {
        let user_roles = UserRoleEntity::find()
            .filter(UserRoleColumn::UserId.eq(user_id))
            .all(db)
            .await
            .map_err(AppError::Database)?;

        let role_ids: Vec<Uuid> = user_roles.into_iter().map(|ur| ur.role_id).collect();

        if role_ids.is_empty() {
            return Ok(vec![]);
        }

        RoleEntity::find()
            .filter(RoleColumn::Id.is_in(role_ids))
            .all(db)
            .await
            .map_err(AppError::Database)
    }
}
