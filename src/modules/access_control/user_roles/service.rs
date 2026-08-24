use super::dto::request::{AssignUserRolesDto, RemoveUserRolesDto};
use super::repository::UserRolesRepository;
use crate::modules::access_control::roles::dto::response::RoleResponseDto;
use crate::modules::access_control::user_roles::dto::response::UserRoleResponse;
use crate::modules::users::repository::UserRepository;
use crate::shared::error::AppError;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct UserRolesService;

impl UserRolesService {
    pub async fn assign(
        db: &DatabaseConnection,
        dto: AssignUserRolesDto,
    ) -> Result<Vec<UserRoleResponse>, AppError> {
        let user = UserRepository::find_by_id(db, dto.user_id).await?;
        if user.is_none() {
            return Err(AppError::NotFound("User not found!".to_string()));
        }

        let roles = UserRolesRepository::assign(db, dto.user_id, dto.role_ids).await?;
        Ok(roles
            .into_iter()
            .map(|r| UserRoleResponse {
                role_id: r.role_id,
                user_id: r.user_id,
            })
            .collect())
    }

    pub async fn remove(db: &DatabaseConnection, dto: RemoveUserRolesDto) -> Result<(), AppError> {
        let user = UserRepository::find_by_id(db, dto.user_id).await?;
        if user.is_none() {
            return Err(AppError::NotFound("User not found!".to_string()));
        }

        UserRolesRepository::remove(db, dto.user_id, dto.role_ids).await
    }

    pub async fn find_roles_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<RoleResponseDto>, AppError> {
        let user = UserRepository::find_by_id(db, user_id).await?;
        if user.is_none() {
            return Err(AppError::NotFound("User not found!".to_string()));
        }

        let roles = UserRolesRepository::find_roles_by_user_id(db, user_id).await?;

        Ok(roles
            .into_iter()
            .map(|r| RoleResponseDto {
                id: r.id.to_string(),
                key: r.key,
                value: r.value,
                description: r.description,
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
        let dto = AssignUserRolesDto {
            user_id: Uuid::new_v4(),
            role_ids: vec![Uuid::new_v4()],
        };
        let result = UserRolesService::assign(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_user_not_found() {
        let db = setup_mock_db();
        let dto = RemoveUserRolesDto {
            user_id: Uuid::new_v4(),
            role_ids: vec![Uuid::new_v4()],
        };
        let result = UserRolesService::remove(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_roles_by_user_id_user_not_found() {
        let db = setup_mock_db();
        let user_id = Uuid::new_v4();
        let result = UserRolesService::find_roles_by_user_id(&db, user_id).await;
        assert!(result.is_err());
    }
}
