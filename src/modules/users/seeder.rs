use std::collections::HashMap;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    modules::{
        access_control::{
            roles::repository::RoleRepository, user_roles::repository::UserRolesRepository,
        },
        users::{
            dto::request::CreateUserDto,
            entities::sea_orm_active_enums::UserStatus,
            profile::{
                dto::request::UpdateUserProfileDto, repository::UserProfileRepository,
            },
            repository::UserRepository,
        },
    },
    shared::error::AppError,
};

#[derive(Debug, Clone)]
pub struct UserSeed {
    pub name: &'static str,
    pub email: &'static str,
    pub first_name: &'static str,
    pub last_name: &'static str,
    pub role_key: &'static str,
}

pub struct UserSeeder;

impl UserSeeder {
    pub fn get_default_users() -> Vec<UserSeed> {
        vec![
            // 2 Admin Users
            UserSeed {
                name: "Admin One",
                email: "admin1@forge.local",
                first_name: "Admin",
                last_name: "One",
                role_key: "admin",
            },
            UserSeed {
                name: "Admin Two",
                email: "admin2@forge.local",
                first_name: "Admin",
                last_name: "Two",
                role_key: "admin",
            },

            // 5 Developer Users
            UserSeed {
                name: "Developer One",
                email: "dev1@forge.local",
                first_name: "Dev",
                last_name: "One",
                role_key: "developer",
            },
            UserSeed {
                name: "Developer Two",
                email: "dev2@forge.local",
                first_name: "Dev",
                last_name: "Two",
                role_key: "developer",
            },
            UserSeed {
                name: "Developer Three",
                email: "dev3@forge.local",
                first_name: "Dev",
                last_name: "Three",
                role_key: "developer",
            },
            UserSeed {
                name: "Developer Four",
                email: "dev4@forge.local",
                first_name: "Dev",
                last_name: "Four",
                role_key: "developer",
            },
            UserSeed {
                name: "Developer Five",
                email: "dev5@forge.local",
                first_name: "Dev",
                last_name: "Five",
                role_key: "developer",
            },

            // 10 Viewer Users
            UserSeed {
                name: "Viewer One",
                email: "viewer1@forge.local",
                first_name: "Viewer",
                last_name: "One",
                role_key: "viewer",
            },
            UserSeed {
                name: "Viewer Two",
                email: "viewer2@forge.local",
                first_name: "Viewer",
                last_name: "Two",
                role_key: "viewer",
            },
            UserSeed {
                name: "Viewer Three",
                email: "viewer3@forge.local",
                first_name: "Viewer",
                last_name: "Three",
                role_key: "viewer",
            },
            UserSeed {
                name: "Viewer Four",
                email: "viewer4@forge.local",
                first_name: "Viewer",
                last_name: "Four",
                role_key: "viewer",
            },
            UserSeed {
                name: "Viewer Five",
                email: "viewer5@forge.local",
                first_name: "Viewer",
                last_name: "Five",
                role_key: "viewer",
            },
            UserSeed {
                name: "Viewer Six",
                email: "viewer6@forge.local",
                first_name: "Viewer",
                last_name: "Six",
                role_key: "viewer",
            },
            UserSeed {
                name: "Viewer Seven",
                email: "viewer7@forge.local",
                first_name: "Viewer",
                last_name: "Seven",
                role_key: "viewer",
            },
            UserSeed {
                name: "Viewer Eight",
                email: "viewer8@forge.local",
                first_name: "Viewer",
                last_name: "Eight",
                role_key: "viewer",
            },
            UserSeed {
                name: "Viewer Nine",
                email: "viewer9@forge.local",
                first_name: "Viewer",
                last_name: "Nine",
                role_key: "viewer",
            },
            UserSeed {
                name: "Viewer Ten",
                email: "viewer10@forge.local",
                first_name: "Viewer",
                last_name: "Ten",
                role_key: "viewer",
            },
        ]
    }

    pub async fn seed_users(
        db: &DatabaseConnection,
    ) -> Result<(), AppError> {
        let default_users = Self::get_default_users();
        let default_password = "Password123!";

        // Fetch existing system roles for assignment
        let all_roles = RoleRepository::find(db).await?;
        let mut roles_map: HashMap<String, Uuid> = HashMap::new();
        for r in all_roles {
            roles_map.insert(r.value, r.id);
        }

        for user_seed in default_users {
            let existing_user = UserRepository::find_by_email_with_password(db, &user_seed.email.to_string()).await?;
            let user_id = match existing_user {
                Some(u) => u.id,
                None => {
                    let created = UserRepository::create(
                        db,
                        CreateUserDto {
                            name: user_seed.name.to_string(),
                            email: user_seed.email.to_string(),
                            password: default_password.to_string(),
                        },
                    )
                    .await?;
                    let _ = UserRepository::update_status(db, created.id, UserStatus::Active).await?;
                    created.id
                }
            };

            // Seed user profile details
            let _ = UserProfileRepository::update_profile(
                db,
                user_id,
                UpdateUserProfileDto {
                    first_name: Some(user_seed.first_name.to_string()),
                    last_name: Some(user_seed.last_name.to_string()),
                    phone: None,
                    date_of_birth: None,
                    gender: None,
                    image: None,
                },
            )
            .await?;

            // Assign role to user
            if let Some(&role_id) = roles_map.get(user_seed.role_key) {
                UserRolesRepository::assign(db, user_id, vec![role_id]).await?;
            }
        }

        Ok(())
    }

    pub async fn seed_all(db: &DatabaseConnection) -> Result<(), AppError> {
        Self::seed_users(db).await
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
    fn test_default_users_structure() {
        let users = UserSeeder::get_default_users();
        assert_eq!(users.len(), 17);

        let admin_count = users.iter().filter(|u| u.role_key == "admin").count();
        let dev_count = users.iter().filter(|u| u.role_key == "developer").count();
        let viewer_count = users.iter().filter(|u| u.role_key == "viewer").count();

        assert_eq!(admin_count, 2);
        assert_eq!(dev_count, 5);
        assert_eq!(viewer_count, 10);
    }

    #[tokio::test]
    async fn test_seed_users_empty_mock_db() {
        let db = setup_mock_db();
        let result = UserSeeder::seed_users(&db).await;
        assert!(result.is_err());
    }
}
