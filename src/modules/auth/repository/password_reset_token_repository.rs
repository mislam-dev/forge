use crate::modules::auth::entities::password_resets::{
    ActiveModel as PasswordResetActiveModel, Column as PasswordResetColumn,
    Entity as PasswordResetEntity, Model as PasswordResethModel,
};

use crate::shared::error::AppError;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use uuid::Uuid;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_reset_token_struct_creation() {
        let user_id = Uuid::new_v4();
        let dto = PasswordResetToken {
            token: "reset_token_sample".to_string(),
            user_id,
            expires_at: 3600,
        };
        assert_eq!(dto.token, "reset_token_sample");
        assert_eq!(dto.user_id, user_id);
        assert_eq!(dto.expires_at, 3600);
    }
}

