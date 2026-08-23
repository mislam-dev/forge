use super::repository::UserRepository;
use crate::modules::users::profile::repository::UserProfileRepository;
use crate::{
    modules::users::dto::{
        request::{CreateUserDto, UpdateUserDto},
        response::{UserItemResponse, UserItemWithPassword},
    },
    shared::error::AppError,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct UserService;

impl UserService {
    pub async fn find(db: &DatabaseConnection) -> Result<Vec<UserItemResponse>, AppError> {
        let users = UserRepository::find(db).await?;
        let users_data = users
            .into_iter()
            .map(|c| UserItemResponse {
                id: c.id,
                name: c.name,
                email: c.email,
            })
            .collect::<Vec<UserItemResponse>>();
        Ok(users_data)
    }

    pub async fn find_one(db: &DatabaseConnection, id: Uuid) -> Result<UserItemResponse, AppError> {
        let user = UserRepository::find_by_id(db, id).await?;

        let user = user.ok_or(AppError::NotFound("User not found".to_string()))?;

        Ok(UserItemResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }

    pub async fn create(
        db: &DatabaseConnection,
        dto: CreateUserDto,
    ) -> Result<UserItemResponse, AppError> {
        let find = UserRepository::find_by_email_with_password(db, &dto.email).await?;

        if find.is_some() {
            return Err(AppError::Conflict("User already exists".to_string()));
        }

        let user = UserRepository::create(db, dto).await?;

        // Auto-create default profile for new user
        let _ = UserProfileRepository::create_default_profile(db, user.id).await;

        // todo: generate email verification token
        // todo: send verification token

        Ok(UserItemResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        dto: UpdateUserDto,
    ) -> Result<UserItemResponse, AppError> {
        let user = UserRepository::update(db, id, dto).await?;

        Ok(UserItemResponse {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }

    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
        let _ = UserRepository::remove(db, id).await?;

        Ok(())
    }

    pub async fn find_by_email_with_password(
        db: &DatabaseConnection,
        email: &String,
    ) -> Result<UserItemWithPassword, AppError> {
        let user = UserRepository::find_by_email_with_password(db, email).await?;
        let user = user.ok_or(AppError::NotFound("User not found".to_string()))?;
        Ok(UserItemWithPassword {
            id: user.id,
            name: user.name,
            email: user.email,
            password: user.password_hash,
        })
    }

    pub async fn update_password(
        db: &DatabaseConnection,
        id: Uuid,
        new_password: &str,
    ) -> Result<(), AppError> {
        UserRepository::update_password(db, id, new_password).await?;
        Ok(())
    }

    pub async fn update_status(
        db: &DatabaseConnection,
        id: Uuid,
        status: super::entities::sea_orm_active_enums::UserStatus,
    ) -> Result<(), AppError> {
        UserRepository::update_status(db, id, status).await?;
        Ok(())
    }
}
