use crate::modules::access_control::roles::dto::{
    request::{RoleCreateDto, RoleUpdateDto},
    response::RoleResponseDto,
};
use crate::modules::access_control::roles::repository::RoleRepository;
use crate::shared::error::AppError;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct RolesService;

impl RolesService {
    pub async fn find(db: &DatabaseConnection) -> Result<Vec<RoleResponseDto>, AppError> {
        let roles = RoleRepository::find(db).await?;

        let roles_data = roles
            .into_iter()
            .map(|c| RoleResponseDto {
                id: c.id.to_string(),
                key: c.key,
                value: c.value,
                description: c.description,
            })
            .collect::<Vec<RoleResponseDto>>();

        Ok(roles_data)
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<RoleResponseDto, AppError> {
        let role = RoleRepository::find_by_id(db, id).await?;
        let role = role.ok_or(AppError::NotFound("Role not found!".to_string()))?;

        Ok(RoleResponseDto {
            id: role.id.to_string(),
            key: role.key,
            value: role.value,
            description: role.description,
        })
    }

    pub async fn create(
        db: &DatabaseConnection,
        dto: RoleCreateDto,
    ) -> Result<RoleResponseDto, AppError> {
        let role = RoleRepository::find_by_value(db, &dto.value).await?;
        if role.is_some() {
            return Err(AppError::Conflict("Role already exists!".to_string()));
        }

        let role = RoleRepository::create(db, dto).await?;

        Ok(RoleResponseDto {
            id: role.id.to_string(),
            key: role.key,
            value: role.value,
            description: role.description,
        })
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        role_data: RoleUpdateDto,
    ) -> Result<RoleResponseDto, AppError> {
        let role = RoleRepository::update(db, id, role_data).await?;

        Ok(RoleResponseDto {
            id: role.id.to_string(),
            key: role.key,
            value: role.value,
            description: role.description,
        })
    }

    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
        RoleRepository::remove(db, id).await
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
    async fn test_find_by_id_not_found() {
        let db = setup_mock_db();
        let id = Uuid::new_v4();
        let result = RolesService::find_by_id(&db, id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_role_duplicate_check() {
        let db = setup_mock_db();
        let dto = RoleCreateDto {
            key: "Admin".to_string(),
            value: "admin".to_string(),
            description: None,
        };
        let result = RolesService::create(&db, dto).await;
        assert!(result.is_err());
    }
}
