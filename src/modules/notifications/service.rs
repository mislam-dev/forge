use chrono::Utc;
use sea_orm::*;
use uuid::Uuid;

use super::dto::{
    CreateNotificationRequest, NotificationQuery, NotificationResponse, UnreadCountResponse,
};
use super::entities::notification::ActiveModel as NotificationActiveModel;
use super::repository::NotificationsRepository;
use crate::shared::error::AppError;
use crate::shared::pagination::PaginatedResponse;

pub struct NotificationsService;

impl NotificationsService {
    pub async fn list_user_notifications(
        db: &DatabaseConnection,
        user_id: Uuid,
        query: NotificationQuery,
    ) -> Result<PaginatedResponse<NotificationResponse>, AppError> {
        let paginated = NotificationsRepository::find_by_user_id(db, user_id, query).await?;
        let responses = paginated
            .data
            .into_iter()
            .map(NotificationResponse::from_model)
            .collect();

        Ok(PaginatedResponse::new(
            responses,
            paginated.page,
            paginated.per_page,
            paginated.total,
        ))
    }

    pub async fn get_unread_count(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<UnreadCountResponse, AppError> {
        let count = NotificationsRepository::count_unread_by_user_id(db, user_id).await?;
        Ok(UnreadCountResponse { count })
    }

    pub async fn mark_as_read(
        db: &DatabaseConnection,
        user_id: Uuid,
        notification_id: Uuid,
    ) -> Result<NotificationResponse, AppError> {
        let notification = NotificationsRepository::find_by_id(db, notification_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Notification not found".to_string()))?;

        if notification.user_id != user_id {
            return Err(AppError::Forbidden(
                "You are not authorized to modify this notification".to_string(),
            ));
        }

        let mut active_model: NotificationActiveModel = notification.into();
        active_model.is_read = Set(true);

        let updated = NotificationsRepository::update_notification(db, active_model).await?;
        Ok(NotificationResponse::from_model(updated))
    }

    pub async fn mark_all_as_read(
        db: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<u64, AppError> {
        NotificationsRepository::mark_all_as_read_for_user(db, user_id).await
    }

    pub async fn create_notification(
        db: &DatabaseConnection,
        req: CreateNotificationRequest,
    ) -> Result<NotificationResponse, AppError> {
        let now = Utc::now().into();
        let active_model = NotificationActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(req.user_id),
            r#type: Set(req.type_name),
            title: Set(req.title),
            message: Set(req.message),
            reference_id: Set(req.reference_id),
            reference_type: Set(req.reference_type),
            is_read: Set(false),
            created_at: Set(now),
        };

        let notification = NotificationsRepository::create_notification(db, active_model).await?;
        Ok(NotificationResponse::from_model(notification))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_get_unread_count_empty_db() {
        let db = setup_mock_db();
        let result = NotificationsService::get_unread_count(&db, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
