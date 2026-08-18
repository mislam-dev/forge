use super::dto::request::{AssignUserRolesDto, RemoveUserRolesDto};
use super::repository::UserRolesRepository;
use crate::modules::access_control::roles::dto::response::RoleResponseDto;
use crate::modules::users::repository::UserRepository;
use crate::shared::error::AppError;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct UserRolesService;

impl UserRolesService {
    pub async fn assign(
        db: &DatabaseConnection,
        dto: AssignUserRolesDto,
    ) -> Result<(), AppError> {
        let user = UserRepository::find_by_id(db, dto.user_id).await?;
        if user.is_none() {
            return Err(AppError::NotFound("User not found!".to_string()));
        }

        UserRolesRepository::assign(db, dto.user_id, dto.role_ids).await
    }

    pub async fn remove(
        db: &DatabaseConnection,
        dto: RemoveUserRolesDto,
    ) -> Result<(), AppError> {
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
                descriptions: r.description,
            })
            .collect())
    }
}
