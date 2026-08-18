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
