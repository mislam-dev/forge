use super::dto::request::{AssignUserPermissionsDto, RemoveUserPermissionsDto};
use super::service::UserPermissionsService;
use crate::app::state::AppState;
use crate::modules::access_control::permissions::dto::response::PermissionResponseDto;
use crate::shared::{error::AppError, utils::IdParams, validation::JsonValidate};
use axum::{
    Json,
    extract::{Path, State},
};

pub struct UserPermissionsHandlers;

impl UserPermissionsHandlers {
    pub async fn assign(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<AssignUserPermissionsDto>,
    ) -> Result<(), AppError> {
        UserPermissionsService::assign(&state.db, payload).await
    }

    pub async fn remove(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<RemoveUserPermissionsDto>,
    ) -> Result<(), AppError> {
        UserPermissionsService::remove(&state.db, payload).await
    }

    pub async fn show(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<Json<Vec<PermissionResponseDto>>, AppError> {
        let perms = UserPermissionsService::find_permissions_by_user_id(&state.db, id.0).await?;
        Ok(Json(perms))
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
    async fn test_assign_handler() {
        let state = setup_mock_state();
        let payload = AssignUserPermissionsDto {
            user_id: Uuid::new_v4(),
            permission_ids: vec![Uuid::new_v4()],
        };
        let result = UserPermissionsHandlers::assign(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_handler() {
        let state = setup_mock_state();
        let payload = RemoveUserPermissionsDto {
            user_id: Uuid::new_v4(),
            permission_ids: vec![Uuid::new_v4()],
        };
        let result = UserPermissionsHandlers::remove(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_show_handler() {
        let state = setup_mock_state();
        let id = IdParams(Uuid::new_v4());
        let result = UserPermissionsHandlers::show(State(state), Path(id)).await;
        assert!(result.is_err());
    }
}

