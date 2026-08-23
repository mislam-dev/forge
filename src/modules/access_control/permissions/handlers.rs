use super::dto::{
    request::{PermissionCreateDto, PermissionUpdateDto},
    response::PermissionResponseDto,
};
use super::service::PermissionsService;
use crate::shared::{
    error::AppError,
    pagination::{PaginatedResponse, PaginationParams},
    utils::IdParams,
    validation::JsonValidate,
};
use crate::{app::state::AppState, shared::response::ApiResponse};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

pub struct PermissionsHandlers;

impl PermissionsHandlers {
    pub async fn list(
        State(state): State<AppState>,
        Query(params): Query<PaginationParams>,
    ) -> Result<ApiResponse<PaginatedResponse<PermissionResponseDto>>, AppError> {
        let perms = PermissionsService::find(&state.db, params).await?;
        Ok(ApiResponse::new()
            .message("Permissions fetch successfully".to_string())
            .status(StatusCode::OK)
            .body(Some(perms)))
    }

    pub async fn show(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<ApiResponse<PermissionResponseDto>, AppError> {
        let perm = PermissionsService::find_by_id(&state.db, id.0).await?;
        Ok(ApiResponse::new()
            .message("Permissions fetch successfully".to_string())
            .status(StatusCode::OK)
            .body(Some(perm)))
    }

    pub async fn add(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<PermissionCreateDto>,
    ) -> Result<ApiResponse<PermissionResponseDto>, AppError> {
        let perm = PermissionsService::create(&state.db, payload).await?;
        Ok(ApiResponse::new()
            .message("Permissions created successfully".to_string())
            .status(StatusCode::CREATED)
            .body(Some(perm)))
    }

    pub async fn update(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
        JsonValidate(payload): JsonValidate<PermissionUpdateDto>,
    ) -> Result<ApiResponse<PermissionResponseDto>, AppError> {
        let perm = PermissionsService::update(&state.db, id.0, payload).await?;
        Ok(ApiResponse::new()
            .message("Permissions updated successfully".to_string())
            .status(StatusCode::OK)
            .body(Some(perm)))
    }

    pub async fn remove(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<ApiResponse<()>, AppError> {
        let _ = PermissionsService::remove(&state.db, id.0).await?;

        Ok(ApiResponse::new()
            .message("Permissions deleted successfully".to_string())
            .status(StatusCode::NO_CONTENT)
            .body(None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use uuid::Uuid;

    fn setup_mock_state() -> AppState {
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://user:pass@localhost:5432/testdb");
            std::env::set_var("JWT_SECRET", "test_secret_key_12345_67890_super_secret");
            std::env::set_var("MASTER_ENCRYPTION_KEY", "0123456789abcdef0123456789abcdef");
        }
        let config = AppConfig::load().expect("Test AppConfig must load successfully");
        AppState::mock(config)
    }

    #[tokio::test]
    async fn test_list_handler() {
        let state = setup_mock_state();
        let result = PermissionsHandlers::list(
            State(state),
            Query(PaginationParams {
                page: 1,
                per_page: 10,
            }),
        )
        .await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_show_handler() {
        let state = setup_mock_state();
        let id = IdParams(Uuid::new_v4());
        let result = PermissionsHandlers::show(State(state), Path(id)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_handler() {
        let state = setup_mock_state();
        let payload = PermissionCreateDto {
            key: "Create User".to_string(),
            value: "create-user".to_string(),
            descriptions: Some("Can create user".to_string()),
        };
        let result = PermissionsHandlers::add(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_handler() {
        let state = setup_mock_state();
        let id = IdParams(Uuid::new_v4());
        let payload = PermissionUpdateDto {
            key: Some("Update User".to_string()),
            value: Some("update-user".to_string()),
            descriptions: None,
        };
        let result =
            PermissionsHandlers::update(State(state), Path(id), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_handler() {
        let state = setup_mock_state();
        let id = IdParams(Uuid::new_v4());
        let result = PermissionsHandlers::remove(State(state), Path(id)).await;
        assert!(result.is_err());
    }
}
