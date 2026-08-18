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
                descriptions: c.description,
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
            descriptions: role.description,
        })
    }

    pub async fn create(
        db: &DatabaseConnection,
        dto: RoleCreateDto,
    ) -> Result<RoleResponseDto, AppError> {
        let role = RoleRepository::find_by_value(db, &dto.value).await?;
        if role.is_some() {
            return Err(AppError::BadRequest("Role already exists!".to_string()));
        }

        let role = RoleRepository::create(db, dto).await?;

        Ok(RoleResponseDto {
            id: role.id.to_string(),
            key: role.key,
            value: role.value,
            descriptions: role.description,
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
            descriptions: role.description,
        })
    }

    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
        RoleRepository::remove(db, id).await
    }
}
