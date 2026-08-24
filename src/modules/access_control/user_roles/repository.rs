use super::entities::user_roles::{
    ActiveModel as UserRoleActiveModel, Column as UserRoleColumn, Entity as UserRoleEntity,
    Model as UserRoleModel,
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
    ) -> Result<Vec<UserRoleModel>, AppError> {
        let mut data: Vec<UserRoleModel> = vec![];
        for role_id in role_ids {
            if let Ok(_) = Self::find_entry(db, user_id, role_id).await {
                continue;
            }
            let active_model = UserRoleActiveModel {
                user_id: Set(user_id),
                role_id: Set(role_id),
            };
            let d = active_model.insert(db).await?;
            data.push(d);
        }
        Ok(data)
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
    pub async fn find_entry(
        db: &DatabaseConnection,
        user_id: Uuid,
        role_id: Uuid,
    ) -> Result<UserRoleModel, AppError> {
        let user_role = UserRoleEntity::find()
            .filter(UserRoleColumn::UserId.eq(user_id))
            .filter(UserRoleColumn::RoleId.eq(role_id))
            .one(db)
            .await
            .map_err(AppError::Database)?;
        if user_role.is_none() {
            return Err(AppError::NotFound("Resource not found!".to_string()));
        }
        Ok(user_role.unwrap())
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
        let result = UserRolesRepository::assign(&db, user_id, vec![]).await;
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
        let role_id = Uuid::new_v4();
        let result = UserRolesRepository::remove(&db, user_id, vec![role_id]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_find_roles_by_user_id_empty_db() {
        let db = setup_mock_db();
        let user_id = Uuid::new_v4();
        let result = UserRolesRepository::find_roles_by_user_id(&db, user_id).await;
        assert!(result.is_err());
    }
}
