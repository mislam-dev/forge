use super::dto::request::{AssignUserRolesDto, RemoveUserRolesDto};
use super::service::UserRolesService;
use crate::app::state::AppState;
use crate::modules::access_control::roles::dto::response::RoleResponseDto;
use crate::shared::{error::AppError, utils::IdParams, validation::JsonValidate};
use axum::{
    Json,
    extract::{Path, State},
};

pub struct UserRolesHandlers;

impl UserRolesHandlers {
    pub async fn assign(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<AssignUserRolesDto>,
    ) -> Result<(), AppError> {
        UserRolesService::assign(&state.db, payload).await
    }

    pub async fn remove(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<RemoveUserRolesDto>,
    ) -> Result<(), AppError> {
        UserRolesService::remove(&state.db, payload).await
    }

    pub async fn show(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<Json<Vec<RoleResponseDto>>, AppError> {
        let roles = UserRolesService::find_roles_by_user_id(&state.db, id.0).await?;
        Ok(Json(roles))
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
        let payload = AssignUserRolesDto {
            user_id: Uuid::new_v4(),
            role_ids: vec![Uuid::new_v4()],
        };
        let result = UserRolesHandlers::assign(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_handler() {
        let state = setup_mock_state();
        let payload = RemoveUserRolesDto {
            user_id: Uuid::new_v4(),
            role_ids: vec![Uuid::new_v4()],
        };
        let result = UserRolesHandlers::remove(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_show_handler() {
        let state = setup_mock_state();
        let id = IdParams(Uuid::new_v4());
        let result = UserRolesHandlers::show(State(state), Path(id)).await;
        assert!(result.is_err());
    }
}
