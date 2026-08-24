use super::dto::request::{AssignRolePermissionsDto, RemoveRolePermissionsDto};
use super::service::RolePermissionsService;
use crate::app::state::AppState;
use crate::modules::access_control::permissions::dto::response::PermissionResponseDto;
use crate::modules::access_control::role_permissions::dto::response::RolePermissionsResponse;
use crate::shared::pagination::{PaginatedResponse, PaginationParams};
use crate::shared::response::ApiResponse;
use crate::shared::{error::AppError, utils::IdParams, validation::JsonValidate};
use axum::extract::Query;
use axum::extract::{Path, State};
use axum::http::StatusCode;

pub struct RolePermissionsHandlers;

impl RolePermissionsHandlers {
    pub async fn assign(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<AssignRolePermissionsDto>,
    ) -> Result<ApiResponse<Vec<RolePermissionsResponse>>, AppError> {
        let data = RolePermissionsService::assign(&state.db, payload).await?;

        Ok(ApiResponse::new()
            .message("Permissions assigned to the role".to_string())
            .status(StatusCode::CREATED)
            .body(Some(data)))
    }

    pub async fn remove(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<RemoveRolePermissionsDto>,
    ) -> Result<ApiResponse<()>, AppError> {
        RolePermissionsService::remove(&state.db, payload).await?;

        Ok(ApiResponse::new().status(StatusCode::NO_CONTENT))
    }

    pub async fn show(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
        Query(params): Query<PaginationParams>,
    ) -> Result<ApiResponse<PaginatedResponse<PermissionResponseDto>>, AppError> {
        let perms =
            RolePermissionsService::find_permissions_by_role_id(&state.db, id.0, params).await?;
        Ok(ApiResponse::new()
            .message("Permissions fetched successfully!".to_string())
            .body(Some(perms)))
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
        let payload = AssignRolePermissionsDto {
            role_id: Uuid::new_v4(),
            permission_ids: vec![Uuid::new_v4()],
        };
        let result = RolePermissionsHandlers::assign(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_handler() {
        let state = setup_mock_state();
        let payload = RemoveRolePermissionsDto {
            role_id: Uuid::new_v4(),
            permission_ids: vec![Uuid::new_v4()],
        };
        let result = RolePermissionsHandlers::remove(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_show_handler() {
        let state = setup_mock_state();
        let id = IdParams(Uuid::new_v4());
        let result = RolePermissionsHandlers::show(
            State(state),
            Path(id),
            Query(PaginationParams {
                page: 1,
                per_page: 10,
            }),
        )
        .await;
        assert!(result.is_err());
    }
}
