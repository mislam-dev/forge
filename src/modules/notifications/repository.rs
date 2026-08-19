use sea_orm::{sea_query::Expr, *};
use uuid::Uuid;

use super::dto::NotificationQuery;
use super::entities::notification::{
    ActiveModel as NotificationActiveModel, Column as NotificationColumn, Entity as NotificationEntity,
    Model as NotificationModel,
};
use crate::shared::error::AppError;
use crate::shared::pagination::PaginatedResponse;

pub struct NotificationsRepository;

impl NotificationsRepository {
    pub async fn find_by_id<C: ConnectionTrait>(
        db: &C,
        id: Uuid,
    ) -> Result<Option<NotificationModel>, AppError> {
        NotificationEntity::find_by_id(id)
            .one(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn find_by_user_id<C: ConnectionTrait>(
        db: &C,
        user_id: Uuid,
        query: NotificationQuery,
    ) -> Result<PaginatedResponse<NotificationModel>, AppError> {
        let page = query.page.unwrap_or(1);
        let per_page = query.per_page.unwrap_or(20);

        let mut stmt = NotificationEntity::find()
            .filter(NotificationColumn::UserId.eq(user_id))
            .order_by_desc(NotificationColumn::CreatedAt);

        if let Some(is_read) = query.is_read {
            stmt = stmt.filter(NotificationColumn::IsRead.eq(is_read));
        }

        let paginator = stmt.paginate(db, per_page);
        let total_items = paginator.num_items().await.map_err(AppError::from)?;
        let data = paginator
            .fetch_page(page - 1)
            .await
            .map_err(AppError::from)?;

        Ok(PaginatedResponse::new(data, page, per_page, total_items))
    }

    pub async fn count_unread_by_user_id<C: ConnectionTrait>(
        db: &C,
        user_id: Uuid,
    ) -> Result<u64, AppError> {
        NotificationEntity::find()
            .filter(NotificationColumn::UserId.eq(user_id))
            .filter(NotificationColumn::IsRead.eq(false))
            .count(db)
            .await
            .map_err(AppError::from)
    }

    pub async fn create_notification<C: ConnectionTrait>(
        db: &C,
        active_model: NotificationActiveModel,
    ) -> Result<NotificationModel, AppError> {
        active_model.insert(db).await.map_err(AppError::from)
    }

    pub async fn update_notification<C: ConnectionTrait>(
        db: &C,
        active_model: NotificationActiveModel,
    ) -> Result<NotificationModel, AppError> {
        active_model.update(db).await.map_err(AppError::from)
    }

    pub async fn mark_all_as_read_for_user<C: ConnectionTrait>(
        db: &C,
        user_id: Uuid,
    ) -> Result<u64, AppError> {
        let res = NotificationEntity::update_many()
            .col_expr(NotificationColumn::IsRead, Expr::value(true))
            .filter(NotificationColumn::UserId.eq(user_id))
            .filter(NotificationColumn::IsRead.eq(false))
            .exec(db)
            .await
            .map_err(AppError::from)?;

        Ok(res.rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_find_by_id_empty_db() {
        let db = setup_mock_db();
        let result = NotificationsRepository::find_by_id(&db, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
