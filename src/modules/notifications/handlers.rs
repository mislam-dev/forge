use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::sse::{Event, KeepAlive, Sse},
};
use std::convert::Infallible;
use std::time::Duration;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;

use super::dto::{
    CreateNotificationRequest, NotificationQuery, NotificationResponse, UnreadCountResponse,
};
use super::service::NotificationsService;
use crate::app::state::AppState;
use crate::modules::auth::token::AccessTokenClaims;
use crate::shared::error::AppError;
use crate::shared::pagination::PaginatedResponse;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn list_notifications(
    State(state): State<AppState>,
    claims: AccessTokenClaims,
    Query(query): Query<NotificationQuery>,
) -> Result<ApiResponse<PaginatedResponse<NotificationResponse>>, AppError> {
    let paginated =
        NotificationsService::list_user_notifications(&state.db, claims.sub, query).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Notifications retrieved successfully.".to_string())
        .body(Some(paginated)))
}

pub async fn get_unread_count(
    State(state): State<AppState>,
    claims: AccessTokenClaims,
) -> Result<ApiResponse<UnreadCountResponse>, AppError> {
    let count = NotificationsService::get_unread_count(&state.db, claims.sub).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Unread notification count retrieved successfully.".to_string())
        .body(Some(count)))
}

pub async fn mark_as_read(
    State(state): State<AppState>,
    claims: AccessTokenClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<NotificationResponse>, AppError> {
    let notification = NotificationsService::mark_as_read(&state.db, claims.sub, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Notification marked as read.".to_string())
        .body(Some(notification)))
}

pub async fn mark_all_as_read(
    State(state): State<AppState>,
    claims: AccessTokenClaims,
) -> Result<ApiResponse<()>, AppError> {
    let count = NotificationsService::mark_all_as_read(&state.db, claims.sub).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message(format!(
            "All notifications marked as read ({} updated).",
            count
        ))
        .body(None))
}

pub async fn create_notification_internal(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<CreateNotificationRequest>,
) -> Result<ApiResponse<NotificationResponse>, AppError> {
    let notification = NotificationsService::create_notification(&state.db, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Notification created successfully.".to_string())
        .body(Some(notification)))
}

// TODO: Implement real-time SSE notification stream using event bus / pub-sub
pub async fn notification_stream(
    _claims: AccessTokenClaims,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let (tx, rx) = tokio::sync::mpsc::channel::<Result<Event, Infallible>>(10);

    let initial_event = Event::default()
        .event("connected")
        .data(r#"{"status":"connected","message":"Notification stream established"}"#);
    let _ = tx.send(Ok(initial_event)).await;

    let stream = ReceiverStream::new(rx);
    Ok(Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_query_defaults() {
        let query = NotificationQuery {
            page: None,
            per_page: None,
            is_read: Some(false),
        };
        assert_eq!(query.is_read, Some(false));
    }
}
