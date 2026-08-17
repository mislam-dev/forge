use super::entities::password_resets::{
    ActiveModel as PasswordResetActiveModel, Column as PasswordResetColumn,
    Entity as PasswordResetEntity, Model as PasswordResethModel,
};
use super::entities::refresh_tokens::{
    ActiveModel as RefreshActiveModel, Column as RefreshColumn, Entity as RefreshEntity,
    Model as RefreshModel,
};
use crate::shared::error::AppError;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;
pub struct RefreshTokenRepository;

pub struct RefreshToken {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}

impl RefreshTokenRepository {
    pub async fn save_refresh_token(
        db: &DatabaseConnection,
        dto: RefreshToken,
    ) -> Result<RefreshModel, AppError> {
        let new_refresh_token = RefreshActiveModel {
            token: Set(dto.token),
            user_id: Set(dto.user_id),
            // expires_at: Set(dto.expires_at),
            ..Default::default()
        };
        new_refresh_token
            .insert(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn fetch_refresh_token(db: &DatabaseConnection, token: &str) -> Result<(), AppError> {
        let filter = Condition::all().add(RefreshColumn::Token.eq(token));
        let _ = RefreshEntity::find()
            .filter(filter)
            .one(db)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }

    pub async fn remove_tokens_by_user_id(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<(), AppError> {
        let filter = Condition::all().add(RefreshColumn::UserId.eq(user_id));
        RefreshEntity::delete_many()
            .filter(filter)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
}

// reset tokens
pub struct PasswordResetToken {
    pub token: String,
    pub user_id: Uuid,
    pub expires_at: i64,
}

pub struct PasswordResetTokenRepository;

impl PasswordResetTokenRepository {
    pub async fn create(
        db: &DatabaseConnection,
        dto: PasswordResetToken,
    ) -> Result<PasswordResethModel, AppError> {
        let token = PasswordResetActiveModel {
            token: Set(dto.token),
            user_id: Set(dto.user_id),
            // expires_at: Set(dto.expires_at),
            ..Default::default()
        };
        token.insert(db).await.map_err(AppError::Database)
    }

    pub async fn find_one(
        db: &DatabaseConnection,
        token: &str,
    ) -> Result<Option<PasswordResethModel>, AppError> {
        let filter = Condition::all().add(PasswordResetColumn::Token.eq(token));
        PasswordResetEntity::find()
            .filter(filter)
            .one(db)
            .await
            .map_err(AppError::Database)
    }

    pub async fn remove_by_user_id(db: &DatabaseConnection, user_id: Uuid) -> Result<(), AppError> {
        let filter = Condition::all().add(PasswordResetColumn::UserId.eq(user_id));
        PasswordResetEntity::delete_many()
            .filter(filter)
            .exec(db)
            .await
            .map_err(AppError::Database)?;
        Ok(())
    }
}
