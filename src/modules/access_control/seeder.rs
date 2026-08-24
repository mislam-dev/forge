use crate::{
    modules::access_control::{
        permissions::{dto::request::PermissionCreateDto, repository::PermissionsRepository},
        role_permissions::repository::RolePermissionsRepository,
        roles::{dto::request::RoleCreateDto, repository::RoleRepository},
    },
    shared::error::AppError,
};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RoleSeed {
    pub key: &'static str,
    pub value: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct PermissionSeed {
    pub key: &'static str,
    pub value: &'static str,
    pub description: &'static str,
}

pub struct AccessControlSeeder;

impl AccessControlSeeder {
    pub fn get_default_roles() -> Vec<RoleSeed> {
        vec![
            RoleSeed {
                key: "System Administrator",
                value: "admin",
                description: "Full access to all system resources and access control configuration",
            },
            RoleSeed {
                key: "Developer",
                value: "developer",
                description: "Developer access to create/manage projects, repos, environment variables, and trigger deployments",
            },
            RoleSeed {
                key: "Viewer",
                value: "viewer",
                description: "Read-only access to view organizations, teams, projects, deployments, build logs, and notifications",
            },
        ]
    }

    pub fn get_default_permissions() -> Vec<PermissionSeed> {
        vec![
            // Access Control Module
            PermissionSeed {
                key: "access_control",
                value: "access_control:roles:read",
                description: "List and view system roles",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:roles:create",
                description: "Create new system roles",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:roles:update",
                description: "Update system roles",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:roles:delete",
                description: "Delete system roles",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:permissions:read",
                description: "List and view system permissions",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:permissions:create",
                description: "Create new system permissions",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:permissions:update",
                description: "Update system permissions",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:permissions:delete",
                description: "Delete system permissions",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:role_permissions:assign",
                description: "Assign permissions to system roles",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:role_permissions:remove",
                description: "Remove permissions from system roles",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:user_roles:assign",
                description: "Assign system roles to users",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:user_roles:remove",
                description: "Remove system roles from users",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:user_permissions:assign",
                description: "Assign direct permissions to users",
            },
            PermissionSeed {
                key: "access_control",
                value: "access_control:user_permissions:remove",
                description: "Remove direct permissions from users",
            },
            // Users Module
            PermissionSeed {
                key: "users",
                value: "users:read",
                description: "List and view user profiles",
            },
            PermissionSeed {
                key: "users",
                value: "users:create",
                description: "Create new user accounts",
            },
            PermissionSeed {
                key: "users",
                value: "users:update",
                description: "Update user profile details",
            },
            PermissionSeed {
                key: "users",
                value: "users:delete",
                description: "Delete user accounts",
            },
            // Organizations Module
            PermissionSeed {
                key: "organizations",
                value: "organizations:read",
                description: "View organization details",
            },
            PermissionSeed {
                key: "organizations",
                value: "organizations:create",
                description: "Create new organizations",
            },
            PermissionSeed {
                key: "organizations",
                value: "organizations:update",
                description: "Update organization settings",
            },
            PermissionSeed {
                key: "organizations",
                value: "organizations:delete",
                description: "Delete organizations",
            },
            PermissionSeed {
                key: "organizations",
                value: "organizations:members:read",
                description: "View organization members",
            },
            PermissionSeed {
                key: "organizations",
                value: "organizations:members:create",
                description: "Add members to organization",
            },
            PermissionSeed {
                key: "organizations",
                value: "organizations:members:update",
                description: "Update member roles in organization",
            },
            PermissionSeed {
                key: "organizations",
                value: "organizations:members:delete",
                description: "Remove members from organization",
            },
            // Teams Module
            PermissionSeed {
                key: "teams",
                value: "teams:read",
                description: "View team details",
            },
            PermissionSeed {
                key: "teams",
                value: "teams:create",
                description: "Create new teams",
            },
            PermissionSeed {
                key: "teams",
                value: "teams:update",
                description: "Update team details",
            },
            PermissionSeed {
                key: "teams",
                value: "teams:delete",
                description: "Delete teams",
            },
            PermissionSeed {
                key: "teams",
                value: "teams:members:read",
                description: "View team members",
            },
            PermissionSeed {
                key: "teams",
                value: "teams:members:create",
                description: "Add members to team",
            },
            PermissionSeed {
                key: "teams",
                value: "teams:members:delete",
                description: "Remove members from team",
            },
            // Projects Module
            PermissionSeed {
                key: "projects",
                value: "projects:read",
                description: "View project details",
            },
            PermissionSeed {
                key: "projects",
                value: "projects:create",
                description: "Create new projects",
            },
            PermissionSeed {
                key: "projects",
                value: "projects:update",
                description: "Update project configuration",
            },
            PermissionSeed {
                key: "projects",
                value: "projects:delete",
                description: "Delete projects",
            },
            // Repositories Module
            PermissionSeed {
                key: "repositories",
                value: "repositories:read",
                description: "View repository configuration and branches",
            },
            PermissionSeed {
                key: "repositories",
                value: "repositories:create",
                description: "Connect repository to project",
            },
            PermissionSeed {
                key: "repositories",
                value: "repositories:update",
                description: "Update repository settings or active branch",
            },
            PermissionSeed {
                key: "repositories",
                value: "repositories:validate",
                description: "Validate repository access credentials",
            },
            PermissionSeed {
                key: "repositories",
                value: "repositories:clone",
                description: "Trigger repository clone operation",
            },
            // Environment Variables Module
            PermissionSeed {
                key: "environment_variables",
                value: "environment_variables:read",
                description: "View masked environment variables",
            },
            PermissionSeed {
                key: "environment_variables",
                value: "environment_variables:create",
                description: "Create environment variables",
            },
            PermissionSeed {
                key: "environment_variables",
                value: "environment_variables:update",
                description: "Update environment variables",
            },
            PermissionSeed {
                key: "environment_variables",
                value: "environment_variables:delete",
                description: "Delete environment variables",
            },
            PermissionSeed {
                key: "environment_variables",
                value: "environment_variables:decrypt",
                description: "Decrypt secret environment variable values",
            },
            // Project Assignments Module
            PermissionSeed {
                key: "project_assignments",
                value: "project_assignments:read",
                description: "View project member and team assignments",
            },
            PermissionSeed {
                key: "project_assignments",
                value: "project_assignments:create",
                description: "Assign members or teams to project",
            },
            PermissionSeed {
                key: "project_assignments",
                value: "project_assignments:delete",
                description: "Remove members or teams from project",
            },
            // Deployments Module
            PermissionSeed {
                key: "deployments",
                value: "deployments:read",
                description: "View deployment details and history",
            },
            PermissionSeed {
                key: "deployments",
                value: "deployments:create",
                description: "Trigger a new deployment",
            },
            PermissionSeed {
                key: "deployments",
                value: "deployments:redeploy",
                description: "Redeploy at specific commit",
            },
            PermissionSeed {
                key: "deployments",
                value: "deployments:rollback",
                description: "Rollback to last successful deployment",
            },
            PermissionSeed {
                key: "deployments",
                value: "deployments:status:update",
                description: "Update deployment status (Internal build worker)",
            },
            // Build Logs Module
            PermissionSeed {
                key: "build_logs",
                value: "build_logs:read",
                description: "View build logs",
            },
            PermissionSeed {
                key: "build_logs",
                value: "build_logs:stream",
                description: "Stream live build logs SSE",
            },
            PermissionSeed {
                key: "build_logs",
                value: "build_logs:download",
                description: "Download build log files",
            },
            // Notifications Module
            PermissionSeed {
                key: "notifications",
                value: "notifications:read",
                description: "View notifications",
            },
            PermissionSeed {
                key: "notifications",
                value: "notifications:update",
                description: "Mark notifications as read",
            },
            PermissionSeed {
                key: "notifications",
                value: "notifications:delete",
                description: "Dismiss notifications",
            },
            PermissionSeed {
                key: "notifications",
                value: "notifications:stream",
                description: "Stream real-time notifications SSE",
            },
            // Dashboard Module
            PermissionSeed {
                key: "dashboard",
                value: "dashboard:read",
                description: "View platform overview dashboard",
            },
            // Health Module
            PermissionSeed {
                key: "health",
                value: "health:read",
                description: "View system health probes",
            },
        ]
    }

    pub fn get_developer_permission_values() -> Vec<&'static str> {
        vec![
            "users:read",
            "organizations:read",
            "organizations:members:read",
            "teams:read",
            "teams:members:read",
            "projects:read",
            "projects:create",
            "projects:update",
            "projects:delete",
            "repositories:read",
            "repositories:create",
            "repositories:update",
            "repositories:validate",
            "repositories:clone",
            "environment_variables:read",
            "environment_variables:create",
            "environment_variables:update",
            "environment_variables:delete",
            "project_assignments:read",
            "project_assignments:create",
            "project_assignments:delete",
            "deployments:read",
            "deployments:create",
            "deployments:redeploy",
            "build_logs:read",
            "build_logs:stream",
            "build_logs:download",
            "notifications:read",
            "notifications:update",
            "notifications:delete",
            "notifications:stream",
            "dashboard:read",
            "health:read",
        ]
    }

    pub fn get_viewer_permission_values() -> Vec<&'static str> {
        vec![
            "users:read",
            "organizations:read",
            "organizations:members:read",
            "teams:read",
            "teams:members:read",
            "projects:read",
            "repositories:read",
            "environment_variables:read",
            "project_assignments:read",
            "deployments:read",
            "build_logs:read",
            "build_logs:stream",
            "build_logs:download",
            "notifications:read",
            "notifications:update",
            "notifications:stream",
            "dashboard:read",
            "health:read",
        ]
    }

    pub async fn seed_roles(db: &DatabaseConnection) -> Result<HashMap<String, Uuid>, AppError> {
        let mut roles_map = HashMap::new();
        let default_roles = Self::get_default_roles();

        for role_seed in default_roles {
            let existing_role =
                RoleRepository::find_by_value(db, &role_seed.value.to_string()).await?;
            let role_id = match existing_role {
                Some(r) => r.id,
                None => {
                    let created = RoleRepository::create(
                        db,
                        RoleCreateDto {
                            key: role_seed.key.to_string(),
                            value: role_seed.value.to_string(),
                            description: Some(role_seed.description.to_string()),
                        },
                    )
                    .await?;
                    created.id
                }
            };
            roles_map.insert(role_seed.value.to_string(), role_id);
        }

        Ok(roles_map)
    }

    pub async fn seed_permissions(
        db: &DatabaseConnection,
    ) -> Result<HashMap<String, Uuid>, AppError> {
        let mut perms_map = HashMap::new();
        let default_perms = Self::get_default_permissions();

        for perm_seed in default_perms {
            let existing_perm =
                PermissionsRepository::find_by_value(db, &perm_seed.value.to_string()).await?;
            let perm_id = match existing_perm {
                Some(p) => p.id,
                None => {
                    let created = PermissionsRepository::create(
                        db,
                        PermissionCreateDto {
                            key: perm_seed.key.to_string(),
                            value: perm_seed.value.to_string(),
                            descriptions: Some(perm_seed.description.to_string()),
                        },
                    )
                    .await?;
                    created.id
                }
            };
            perms_map.insert(perm_seed.value.to_string(), perm_id);
        }

        Ok(perms_map)
    }

    pub async fn seed_role_permissions(
        db: &DatabaseConnection,
        roles_map: &HashMap<String, Uuid>,
        perms_map: &HashMap<String, Uuid>,
    ) -> Result<(), AppError> {
        // 1. Admin Role -> ALL permissions
        if let Some(&admin_role_id) = roles_map.get("admin") {
            let all_perm_ids: Vec<Uuid> = perms_map.values().cloned().collect();
            RolePermissionsRepository::assign(db, admin_role_id, all_perm_ids).await?;
        }

        // 2. Developer Role -> Developer permissions subset
        if let Some(&dev_role_id) = roles_map.get("developer") {
            let dev_values = Self::get_developer_permission_values();
            let dev_perm_ids: Vec<Uuid> = dev_values
                .into_iter()
                .filter_map(|val| perms_map.get(val).cloned())
                .collect();
            RolePermissionsRepository::assign(db, dev_role_id, dev_perm_ids).await?;
        }

        // 3. Viewer Role -> Viewer permissions subset
        if let Some(&viewer_role_id) = roles_map.get("viewer") {
            let viewer_values = Self::get_viewer_permission_values();
            let viewer_perm_ids: Vec<Uuid> = viewer_values
                .into_iter()
                .filter_map(|val| perms_map.get(val).cloned())
                .collect();
            RolePermissionsRepository::assign(db, viewer_role_id, viewer_perm_ids).await?;
        }

        Ok(())
    }

    pub async fn seed_all(db: &DatabaseConnection) -> Result<(), AppError> {
        let roles_map = Self::seed_roles(db).await?;
        let perms_map = Self::seed_permissions(db).await?;
        Self::seed_role_permissions(db, &roles_map, &perms_map).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[test]
    fn test_default_roles_structure() {
        let roles = AccessControlSeeder::get_default_roles();
        assert_eq!(roles.len(), 3);
        let values: Vec<&str> = roles.iter().map(|r| r.value).collect();
        assert!(values.contains(&"admin"));
        assert!(values.contains(&"developer"));
        assert!(values.contains(&"viewer"));
    }

    #[test]
    fn test_default_permissions_count() {
        let perms = AccessControlSeeder::get_default_permissions();
        assert_eq!(perms.len(), 64);
    }

    #[test]
    fn test_role_mappings_validity() {
        let perms = AccessControlSeeder::get_default_permissions();
        let all_perm_values: std::collections::HashSet<&str> =
            perms.iter().map(|p| p.value).collect();

        for dev_perm in AccessControlSeeder::get_developer_permission_values() {
            assert!(
                all_perm_values.contains(dev_perm),
                "Developer permission {} does not exist in master permission list",
                dev_perm
            );
        }

        for viewer_perm in AccessControlSeeder::get_viewer_permission_values() {
            assert!(
                all_perm_values.contains(viewer_perm),
                "Viewer permission {} does not exist in master permission list",
                viewer_perm
            );
        }
    }

    #[tokio::test]
    async fn test_seed_roles_empty_mock_db() {
        let db = setup_mock_db();
        let result = AccessControlSeeder::seed_roles(&db).await;
        assert!(result.is_err());
    }
}
