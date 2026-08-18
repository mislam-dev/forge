use super::dto::request::{AssignRolePermissionsDto, RemoveRolePermissionsDto};
use super::repository::RolePermissionsRepository;
use crate::modules::access_control::permissions::dto::response::PermissionResponseDto;
use crate::modules::access_control::roles::repository::RoleRepository;
use crate::shared::error::AppError;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct RolePermissionsService;

impl RolePermissionsService {
    pub async fn assign(
        db: &DatabaseConnection,
        dto: AssignRolePermissionsDto,
    ) -> Result<(), AppError> {
        let role = RoleRepository::find_by_id(db, dto.role_id).await?;
        if role.is_none() {
            return Err(AppError::NotFound("Role not found!".to_string()));
        }

        RolePermissionsRepository::assign(db, dto.role_id, dto.permission_ids).await
    }

    pub async fn remove(
        db: &DatabaseConnection,
        dto: RemoveRolePermissionsDto,
    ) -> Result<(), AppError> {
        let role = RoleRepository::find_by_id(db, dto.role_id).await?;
        if role.is_none() {
            return Err(AppError::NotFound("Role not found!".to_string()));
        }

        RolePermissionsRepository::remove(db, dto.role_id, dto.permission_ids).await
    }

    pub async fn find_permissions_by_role_id(
        db: &DatabaseConnection,
        role_id: Uuid,
    ) -> Result<Vec<PermissionResponseDto>, AppError> {
        let role = RoleRepository::find_by_id(db, role_id).await?;
        if role.is_none() {
            return Err(AppError::NotFound("Role not found!".to_string()));
        }

        let perms = RolePermissionsRepository::find_permissions_by_role_id(db, role_id).await?;

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
    async fn test_assign_role_not_found() {
        let db = setup_mock_db();
        let dto = AssignRolePermissionsDto {
            role_id: Uuid::new_v4(),
            permission_ids: vec![Uuid::new_v4()],
        };
        let result = RolePermissionsService::assign(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_role_not_found() {
        let db = setup_mock_db();
        let dto = RemoveRolePermissionsDto {
            role_id: Uuid::new_v4(),
            permission_ids: vec![Uuid::new_v4()],
        };
        let result = RolePermissionsService::remove(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_find_permissions_by_role_id_role_not_found() {
        let db = setup_mock_db();
        let role_id = Uuid::new_v4();
        let result = RolePermissionsService::find_permissions_by_role_id(&db, role_id).await;
        assert!(result.is_err());
    }
}

