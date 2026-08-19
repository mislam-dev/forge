use sea_orm::DatabaseConnection;
use std::collections::HashSet;
use uuid::Uuid;

use super::role_permissions::repository::RolePermissionsRepository;
use super::user_permissions::repository::UserPermissionsRepository;
use super::user_roles::repository::UserRolesRepository;
use crate::shared::error::AppError;

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
            let role_perms =
                RolePermissionsRepository::find_permissions_by_role_id(db, role.id).await?;
            for perm in role_perms {
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
