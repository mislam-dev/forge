use super::dto::request::{AssignUserPermissionsDto, RemoveUserPermissionsDto};
use super::repository::UserPermissionsRepository;
use crate::modules::access_control::permissions::dto::response::PermissionResponseDto;
use crate::modules::users::repository::UserRepository;
use crate::shared::error::AppError;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct UserPermissionsService;

impl UserPermissionsService {
    pub async fn assign(
        db: &DatabaseConnection,
        dto: AssignUserPermissionsDto,
    ) -> Result<(), AppError> {
        let user = UserRepository::find_by_id(db, dto.user_id).await?;
        if user.is_none() {
            return Err(AppError::NotFound("User not found!".to_string()));
        }

        UserPermissionsRepository::assign(db, dto.user_id, dto.permission_ids).await
    }

    pub async fn remove(
        db: &DatabaseConnection,
        dto: RemoveUserPermissionsDto,
    ) -> Result<(), AppError> {
        let user = UserRepository::find_by_id(db, dto.user_id).await?;
        if user.is_none() {
            return Err(AppError::NotFound("User not found!".to_string()));
        }

        UserPermissionsRepository::remove(db, dto.user_id, dto.permission_ids).await
    }

    pub async fn find_permissions_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<PermissionResponseDto>, AppError> {
        let user = UserRepository::find_by_id(db, user_id).await?;
        if user.is_none() {
            return Err(AppError::NotFound("User not found!".to_string()));
        }

        let perms = UserPermissionsRepository::find_permissions_by_user_id(db, user_id).await?;

        Ok(perms
            .into_iter()
            .map(|p| PermissionResponseDto {
                id: p.id.to_string(),
                key: p.key,
                value: p.value,
                descriptions: p.description,
            })
            .collect())
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
    async fn test_assign_user_not_found() {
        let db = setup_mock_db();
        let dto = AssignUserPermissionsDto {
            user_id: Uuid::new_v4(),
            permission_ids: vec![Uuid::new_v4()],
        };
        let result = UserPermissionsService::assign(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_user_not_found() {
        let db = setup_mock_db();
        let dto = RemoveUserPermissionsDto {
            user_id: Uuid::new_v4(),
            permission_ids: vec![Uuid::new_v4()],
        };
        let result = UserPermissionsService::remove(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_permissions_by_user_id_user_not_found() {
        let db = setup_mock_db();
        let user_id = Uuid::new_v4();
        let result = UserPermissionsService::find_permissions_by_user_id(&db, user_id).await;
        assert!(result.is_err());
    }
}
