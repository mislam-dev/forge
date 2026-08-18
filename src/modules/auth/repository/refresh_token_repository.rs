use crate::modules::auth::entities::refresh_tokens::{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refresh_token_struct_creation() {
        let user_id = Uuid::new_v4();
        let dto = RefreshToken {
            token: "refresh_token_sample".to_string(),
            user_id,
            expires_at: 604800,
        };
        assert_eq!(dto.token, "refresh_token_sample");
        assert_eq!(dto.user_id, user_id);
        assert_eq!(dto.expires_at, 604800);
    }
}

