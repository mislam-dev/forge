use super::entities::sea_orm_active_enums::UserStatus;
use super::entities::users::{
    ActiveModel as UsersActiveModel, Column as UserColumn, Entity as UsersEntity,
    Model as UserModel,
};
use crate::{
    modules::users::{
        dto::request::{CreateUserDto, UpdateUserDto},
        password::PasswordService,
    },
    shared::error::AppError,
};
use sea_orm::DatabaseConnection;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DeleteResult, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn find(db: &DatabaseConnection) -> Result<Vec<UserModel>, AppError> {
        UsersEntity::find()
            .all(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<UserModel>, AppError> {
        UsersEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn create(
        db: &DatabaseConnection,
        dto: CreateUserDto,
    ) -> Result<UserModel, AppError> {
        let hash_p = PasswordService::hash(&dto.password).await?;
        let new_user = UsersActiveModel {
            email: Set(dto.email),
            password_hash: Set(hash_p),
            name: Set(dto.username),
            ..Default::default()
        };
        new_user.insert(db).await.map_err(AppError::Database)
    }

    pub async fn update(
        db: &DatabaseConnection,
        id: Uuid,
        user_data: UpdateUserDto,
    ) -> Result<UserModel, AppError> {
        let existing_user = Self::find_by_id(db, id)
            .await?
            .ok_or(AppError::NotFound("User not found!".to_string()))?;

        let mut active_user: UsersActiveModel = existing_user.into();

        if let Some(name) = user_data.name {
            active_user.name = Set(name);
        }

        active_user.update(db).await.map_err(AppError::Database)
    }

    pub async fn remove(db: &DatabaseConnection, id: Uuid) -> Result<DeleteResult, AppError> {
        UsersEntity::delete_by_id(id)
            .exec(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn find_by_email_with_password(
        db: &DatabaseConnection,
        email: &String,
    ) -> Result<Option<UserModel>, AppError> {
        let filter = Condition::all().add(UserColumn::Email.eq(email));
        UsersEntity::find()
            .filter(filter)
            .one(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn update_password(
        db: &DatabaseConnection,
        id: Uuid,
        new_password: &str,
    ) -> Result<UserModel, AppError> {
        let existing_user = Self::find_by_id(db, id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        let new_hash = PasswordService::hash(new_password).await?;
        let mut active_user: UsersActiveModel = existing_user.into();
        active_user.password_hash = Set(new_hash);
        active_user.update(db).await.map_err(AppError::Database)
    }

    pub async fn update_status(
        db: &DatabaseConnection,
        id: Uuid,
        status: UserStatus,
    ) -> Result<UserModel, AppError> {
        let existing_user = Self::find_by_id(db, id)
            .await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;

        let mut active_user: UsersActiveModel = existing_user.into();
        active_user.status = Set(Some(status));
        active_user.update(db).await.map_err(AppError::Database)
    }
}
