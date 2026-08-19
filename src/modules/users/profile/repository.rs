use super::dto::request::{CreateUserProfileDto, UpdateUserProfileDto};
use super::entities::user_profile::{
    ActiveModel as ProfileActiveModel, Column as ProfileColumn, Entity as ProfileEntity,
    Model as ProfileModel,
};
use crate::shared::error::AppError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub struct UserProfileRepository;

impl UserProfileRepository {
    pub async fn find_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Option<ProfileModel>, AppError> {
        let profile = ProfileEntity::find()
            .filter(ProfileColumn::UserId.eq(user_id))
            .one(db)
            .await?;
        Ok(profile)
    }

    pub async fn create_default_profile(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<ProfileModel, AppError> {
        let active_model = ProfileActiveModel {
            user_id: Set(user_id),
            ..Default::default()
        };

        let profile = active_model.insert(db).await?;
        Ok(profile)
    }

    pub async fn create_profile(
        db: &DatabaseConnection,
        user_id: Uuid,
        dto: CreateUserProfileDto,
    ) -> Result<ProfileModel, AppError> {
        let active_model = ProfileActiveModel {
            user_id: Set(user_id),
            first_name: Set(Some(dto.first_name)),
            last_name: Set(Some(dto.last_name)),
            phone: Set(Some(dto.phone)),
            date_of_birth: Set(Some(dto.date_of_birth)),
            gender: Set(Some(dto.gender)),
            image: Set(dto.image),
            ..Default::default()
        };

        let profile = active_model.insert(db).await?;
        Ok(profile)
    }

    pub async fn update_profile(
        db: &DatabaseConnection,
        user_id: Uuid,
        dto: UpdateUserProfileDto,
    ) -> Result<ProfileModel, AppError> {
        let existing = Self::find_by_user_id(db, user_id).await?;

        match existing {
            Some(model) => {
                let mut active: ProfileActiveModel = model.into();

                if let Some(first_name) = dto.first_name {
                    active.first_name = Set(Some(first_name));
                }
                if let Some(last_name) = dto.last_name {
                    active.last_name = Set(Some(last_name));
                }
                if let Some(phone) = dto.phone {
                    active.phone = Set(Some(phone));
                }
                if let Some(dob) = dto.date_of_birth {
                    active.date_of_birth = Set(Some(dob));
                }
                if let Some(gender) = dto.gender {
                    active.gender = Set(Some(gender));
                }
                if let Some(image) = dto.image {
                    active.image = Set(Some(image));
                }

                let updated = active.update(db).await?;
                Ok(updated)
            }
            None => {
                let active_model = ProfileActiveModel {
                    id: Set(Uuid::new_v4()),
                    user_id: Set(user_id),
                    first_name: Set(dto.first_name),
                    last_name: Set(dto.last_name),
                    phone: Set(dto.phone),
                    date_of_birth: Set(dto.date_of_birth),
                    gender: Set(dto.gender),
                    image: Set(dto.image),
                    ..Default::default()
                };
                let created = active_model.insert(db).await?;
                Ok(created)
            }
        }
    }

    pub async fn delete_by_user_id(db: &DatabaseConnection, user_id: Uuid) -> Result<(), AppError> {
        ProfileEntity::delete_many()
            .filter(ProfileColumn::UserId.eq(user_id))
            .exec(db)
            .await?;
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

    #[tokio::test]
    async fn test_find_by_user_id_empty_db() {
        let db = setup_mock_db();
        let user_id = Uuid::new_v4();
        let result = UserProfileRepository::find_by_user_id(&db, user_id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_by_user_id_empty_db() {
        let db = setup_mock_db();
        let user_id = Uuid::new_v4();
        let result = UserProfileRepository::delete_by_user_id(&db, user_id).await;
        assert!(result.is_err());
    }
}
