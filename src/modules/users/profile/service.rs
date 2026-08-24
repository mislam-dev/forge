use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::dto::request::{CreateUserProfileDto, UpdateUserProfileDto};
use super::dto::response::UserProfileResponse;
use super::entities::user_profile::Model;
use super::repository::UserProfileRepository;
use crate::shared::error::AppError;

pub struct UserProfileService;

impl UserProfileService {
    fn map_model_to_response(model: Model) -> UserProfileResponse {
        UserProfileResponse {
            id: model.id,
            user_id: model.user_id,
            first_name: model.first_name,
            last_name: model.last_name,
            phone: model.phone,
            date_of_birth: model.date_of_birth,
            gender: model.gender,
            image: model.image,
            created_at: model.created_at.to_string(),
            updated_at: model.updated_at.to_string(),
        }
    }

    pub async fn get_profile(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<UserProfileResponse, AppError> {
        let profile = UserProfileRepository::find_by_user_id(db, user_id).await?;
        let profile = profile.ok_or(AppError::NotFound("Profile not found".to_string()))?;
        Ok(Self::map_model_to_response(profile))
    }

    pub async fn create_profile(
        db: &DatabaseConnection,
        user_id: Uuid,
        dto: CreateUserProfileDto,
    ) -> Result<UserProfileResponse, AppError> {
        let profile = UserProfileRepository::create_profile(db, user_id, dto).await?;
        Ok(Self::map_model_to_response(profile))
    }

    pub async fn create_default_profile(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<UserProfileResponse, AppError> {
        let profile = UserProfileRepository::create_default_profile(db, user_id).await?;
        Ok(Self::map_model_to_response(profile))
    }

    pub async fn update_profile(
        db: &DatabaseConnection,
        user_id: Uuid,
        dto: UpdateUserProfileDto,
    ) -> Result<UserProfileResponse, AppError> {
        let profile = UserProfileRepository::update_profile(db, user_id, dto).await?;
        Ok(Self::map_model_to_response(profile))
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
    async fn test_get_profile_not_found() {
        let db = setup_mock_db();
        let user_id = Uuid::new_v4();
        let result = UserProfileService::get_profile(&db, user_id).await;
        assert!(result.is_err());
    }
}
