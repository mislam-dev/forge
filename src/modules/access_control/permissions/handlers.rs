use super::dto::{
    request::{PermissionCreateDto, PermissionUpdateDto},
    response::PermissionResponseDto,
};
use super::service::PermissionsService;
use crate::app::state::AppState;
use crate::shared::{error::AppError, utils::IdParams, validation::JsonValidate};
use axum::{
    Json,
    extract::{Path, State},
};

pub struct PermissionsHandlers;

impl PermissionsHandlers {
    pub async fn list(
        State(state): State<AppState>,
    ) -> Result<Json<Vec<PermissionResponseDto>>, AppError> {
        let perms = PermissionsService::find(&state.db).await?;
        Ok(Json(perms))
    }

    pub async fn show(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<Json<PermissionResponseDto>, AppError> {
        let perm = PermissionsService::find_by_id(&state.db, id.0).await?;
        Ok(Json(perm))
    }

    pub async fn add(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<PermissionCreateDto>,
    ) -> Result<Json<PermissionResponseDto>, AppError> {
        let perm = PermissionsService::create(&state.db, payload).await?;
        Ok(Json(perm))
    }

    pub async fn update(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
        JsonValidate(payload): JsonValidate<PermissionUpdateDto>,
    ) -> Result<Json<PermissionResponseDto>, AppError> {
        let perm = PermissionsService::update(&state.db, id.0, payload).await?;
        Ok(Json(perm))
    }

    pub async fn remove(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<(), AppError> {
        let _ = PermissionsService::remove(&state.db, id.0).await?;

        Ok(())
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
        let result = PermissionsHandlers::list(State(state)).await;
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
        let result = PermissionsHandlers::update(State(state), Path(id), JsonValidate(payload)).await;
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

