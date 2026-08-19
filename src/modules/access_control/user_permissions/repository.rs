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

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_assign_empty_vec() {
        let db = setup_mock_db();
        let user_id = Uuid::new_v4();
        let result = UserPermissionsRepository::assign(&db, user_id, vec![]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remove_empty_db() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([sea_orm::MockExecResult {
                last_insert_id: 0,
                rows_affected: 0,
            }])
            .into_connection();
        let user_id = Uuid::new_v4();
        let perm_id = Uuid::new_v4();
        let result = UserPermissionsRepository::remove(&db, user_id, vec![perm_id]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_permissions_by_user_id_empty_db() {
        let db = setup_mock_db();
        let user_id = Uuid::new_v4();
        let result = UserPermissionsRepository::find_permissions_by_user_id(&db, user_id).await;
        assert!(result.is_err());
    }
}
