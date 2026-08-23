use super::dto::{
    request::{PermissionCreateDto, PermissionUpdateDto},
    response::PermissionResponseDto,
};
use super::repository::PermissionsRepository;
use crate::shared::{
    error::AppError,
    pagination::{PaginatedResponse, PaginationParams},
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct PermissionsService;

impl PermissionsService {
    pub async fn find(
        db: &DatabaseConnection,
        params: PaginationParams,
    ) -> Result<PaginatedResponse<PermissionResponseDto>, AppError> {
        let data = PermissionsRepository::find(db, &params).await?;

        let perms_data = data
            .data
            .into_iter()
            .map(|c| PermissionResponseDto {
                id: c.id.to_string(),
                key: c.key,
                value: c.value,
                descriptions: c.description,
            })
            .collect::<Vec<PermissionResponseDto>>();

        Ok(PaginatedResponse {
            data: perms_data,
            page: data.page,
            per_page: data.per_page,
            total: data.total,
            total_pages: data.total_pages,
        })
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<PermissionResponseDto, AppError> {
        let perm = PermissionsRepository::find_by_id(db, id).await?;
        let perm = perm.ok_or(AppError::NotFound("Permission not found!".to_string()))?;

        Ok(PermissionResponseDto {
            id: perm.id.to_string(),
            key: perm.key,
            value: perm.value,
            descriptions: perm.description,
        })
    }

    pub async fn create(
        db: &DatabaseConnection,
        dto: PermissionCreateDto,
    ) -> Result<PermissionResponseDto, AppError> {
        let perm = PermissionsRepository::find_by_value(db, &dto.value).await?;
        if perm.is_some() {
            return Err(AppError::Conflict("Permission already exists!".to_string()));
        }

        let perm = PermissionsRepository::create(db, dto).await?;

        Ok(PermissionResponseDto {
            id: perm.id.to_string(),
            key: perm.key,
            value: perm.value,
            descriptions: perm.description,
        })
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        perm_data: PermissionUpdateDto,
    ) -> Result<PermissionResponseDto, AppError> {
        if let Some(value) = &perm_data.value {
            let perm = PermissionsRepository::find_by_value(db, value).await?;
            if perm.is_some() && perm.unwrap().id != id {
                return Err(AppError::Conflict("Permission already exists!".to_string()));
            }
        }
        let perm = PermissionsRepository::update(db, id, perm_data).await?;

        Ok(PermissionResponseDto {
            id: perm.id.to_string(),
            key: perm.key,
            value: perm.value,
            descriptions: perm.description,
        })
    }

    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
        PermissionsRepository::remove(db, id).await
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
        let result = PermissionsService::find_by_id(&db, id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_permission_duplicate_check() {
        let db = setup_mock_db();
        let dto = PermissionCreateDto {
            key: "Create User".to_string(),
            value: "create-user".to_string(),
            descriptions: None,
        };
        let result = PermissionsService::create(&db, dto).await;
        assert!(result.is_err());
    }
}
