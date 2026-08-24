use super::role_permissions::repository::RolePermissionsRepository;
use super::user_permissions::repository::UserPermissionsRepository;
use super::user_roles::repository::UserRolesRepository;
use crate::modules::access_control::roles::dto::response::RoleResponseDto;
use crate::modules::access_control::user_roles::service::UserRolesService;
use crate::shared::error::AppError;
use crate::shared::pagination::PaginationParams;
use sea_orm::DatabaseConnection;
use std::collections::HashSet;
use uuid::Uuid;

pub struct AccessControlService;

impl AccessControlService {
    pub async fn resolve_user_permissions(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<HashSet<String>, AppError> {
        let mut permissions_set = HashSet::new();

        // 1. Fetch user assigned roles
        let user_roles = UserRolesRepository::find_roles_by_user_id(db, user_id).await?;

        // 2. Fetch permissions for each role
        for role in user_roles {
            let role_perms = RolePermissionsRepository::find_permissions_by_role_id(
                db,
                role.id,
                PaginationParams {
                    page: 1,
                    per_page: 100,
                },
            )
            .await?;
            for perm in role_perms.data {
                permissions_set.insert(perm.value);
            }
        }

        // 3. Fetch direct user permissions
        let direct_perms =
            UserPermissionsRepository::find_permissions_by_user_id(db, user_id).await?;
        for perm in direct_perms {
            permissions_set.insert(perm.value);
        }

        Ok(permissions_set)
    }
    pub async fn get_user_roles_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<RoleResponseDto>, AppError> {
        let user_roles = UserRolesService::find_roles_by_user_id(db, user_id).await?;

        Ok(user_roles)
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
    async fn test_resolve_user_permissions_empty_db() {
        let db = setup_mock_db();
        let user_id = Uuid::new_v4();
        let perms = AccessControlService::resolve_user_permissions(&db, user_id).await;
        assert!(perms.is_err());
    }
}
