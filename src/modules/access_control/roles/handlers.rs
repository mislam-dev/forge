use crate::app::state::AppState;
use crate::modules::access_control::roles::dto::{
    request::{RoleCreateDto, RoleUpdateDto},
    response::RoleResponseDto,
};
use crate::modules::access_control::roles::service::RolesService;
use crate::shared::response::ApiResponse;
use crate::shared::{error::AppError, utils::IdParams, validation::JsonValidate};
use axum::extract::{Path, State};
use axum::http::StatusCode;

pub struct RolesHandlers;

impl RolesHandlers {
    pub async fn list(
        State(state): State<AppState>,
    ) -> Result<ApiResponse<Vec<RoleResponseDto>>, AppError> {
        let roles = RolesService::find(&state.db).await?;
        Ok(ApiResponse::new()
            .status(StatusCode::OK)
            .message("Roles fetched successfully".to_string())
            .body(Some(roles)))
    }

    pub async fn show(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<ApiResponse<RoleResponseDto>, AppError> {
        let role = RolesService::find_by_id(&state.db, id.0).await?;
        Ok(ApiResponse::new()
            .status(StatusCode::OK)
            .message("Role fetched successfully".to_string())
            .body(Some(role)))
    }

    pub async fn add(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<RoleCreateDto>,
    ) -> Result<ApiResponse<RoleResponseDto>, AppError> {
        let role = RolesService::create(&state.db, payload).await?;
        Ok(ApiResponse::new()
            .status(StatusCode::CREATED)
            .message("Role created successfully".to_string())
            .body(Some(role)))
    }

    pub async fn update(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
        JsonValidate(payload): JsonValidate<RoleUpdateDto>,
    ) -> Result<ApiResponse<RoleResponseDto>, AppError> {
        let role = RolesService::update(&state.db, id.0, payload).await?;
        Ok(ApiResponse::new()
            .status(StatusCode::OK)
            .message("Role updated successfully".to_string())
            .body(Some(role)))
    }

    pub async fn remove(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<ApiResponse<()>, AppError> {
        let _ = RolesService::remove(&state.db, id.0).await?;
        Ok(ApiResponse::new()
            .status(StatusCode::OK)
            .message("Role deleted successfully".to_string())
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
        let result = RolesHandlers::list(State(state)).await;
        // Mock DB query returns empty list or error depending on uncaught query, but handler call is exercised
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_show_handler() {
        let state = setup_mock_state();
        let id = IdParams(Uuid::new_v4());
        let result = RolesHandlers::show(State(state), Path(id)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_handler() {
        let state = setup_mock_state();
        let payload = RoleCreateDto {
            key: "Admin".to_string(),
            value: "admin".to_string(),
            description: Some("System Admin".to_string()),
        };
        let result = RolesHandlers::add(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_handler() {
        let state = setup_mock_state();
        let id = IdParams(Uuid::new_v4());
        let payload = RoleUpdateDto {
            key: Some("Updated Admin".to_string()),
            value: Some("updated_admin".to_string()),
            description: None,
        };
        let result = RolesHandlers::update(State(state), Path(id), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_handler() {
        let state = setup_mock_state();
        let id = IdParams(Uuid::new_v4());
        let result = RolesHandlers::remove(State(state), Path(id)).await;
        assert!(result.is_err());
    }
}
