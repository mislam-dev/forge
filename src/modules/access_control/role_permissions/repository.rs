use super::entities::role_permissions::{
    ActiveModel as RolePermissionActiveModel, Column as RolePermissionColumn,
    Entity as RolePermissionEntity, Model as RolePermissionModel,
};
use crate::modules::access_control::permissions::dto::response::PermissionResponseDto;
use crate::modules::access_control::permissions::service::PermissionsService;
use crate::shared::error::AppError;
use crate::shared::pagination::{PaginatedResponse, PaginationParams};
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
    ) -> Result<Vec<RolePermissionModel>, AppError> {
        let mut data: Vec<RolePermissionModel> = vec![];
        for permission_id in permission_ids {
            if let Ok(_) = Self::find_permissions_entry(db, role_id, permission_id).await {
                continue;
            }
            let active_model = RolePermissionActiveModel {
                role_id: Set(role_id),
                permission_id: Set(permission_id),
            };
            let da = active_model.insert(db).await?;
            data.push(da);
        }
        Ok(data)
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
        params: PaginationParams,
    ) -> Result<PaginatedResponse<PermissionResponseDto>, AppError> {
        let role_perms = RolePermissionEntity::find()
            .filter(RolePermissionColumn::RoleId.eq(role_id))
            .all(db)
            .await
            .map_err(AppError::Database)?;

        let perm_ids: Vec<Uuid> = role_perms.into_iter().map(|rp| rp.permission_id).collect();

        let data = PermissionsService::find_by_permission_ids(db, perm_ids, params).await?;
        Ok(data)
    }
    pub async fn find_permissions_entry(
        db: &DatabaseConnection,
        role_id: Uuid,
        permission_id: Uuid,
    ) -> Result<RolePermissionModel, AppError> {
        let role_permission = RolePermissionEntity::find()
            .filter(RolePermissionColumn::RoleId.eq(role_id))
            .filter(RolePermissionColumn::PermissionId.eq(permission_id))
            .one(db)
            .await
            .map_err(AppError::Database)?;

        if role_permission.is_none() {
            return Err(AppError::NotFound("Resource not found".to_string()));
        }

        Ok(role_permission.unwrap())
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
        let role_id = Uuid::new_v4();
        let result = RolePermissionsRepository::assign(&db, role_id, vec![]).await;
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
        let role_id = Uuid::new_v4();
        let perm_id = Uuid::new_v4();
        let result = RolePermissionsRepository::remove(&db, role_id, vec![perm_id]).await;
        assert!(result.is_ok());
    }
}
