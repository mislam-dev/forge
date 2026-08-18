use super::dto::request::{AssignRolePermissionsDto, RemoveRolePermissionsDto};
use super::service::RolePermissionsService;
use crate::app::state::AppState;
use crate::modules::access_control::permissions::dto::response::PermissionResponseDto;
use crate::shared::{error::AppError, utils::IdParams, validation::JsonValidate};
use axum::{
    Json,
    extract::{Path, State},
};

pub struct RolePermissionsHandlers;

impl RolePermissionsHandlers {
    pub async fn assign(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<AssignRolePermissionsDto>,
    ) -> Result<(), AppError> {
        RolePermissionsService::assign(&state.db, payload).await
    }

    pub async fn remove(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<RemoveRolePermissionsDto>,
    ) -> Result<(), AppError> {
        RolePermissionsService::remove(&state.db, payload).await
    }

    pub async fn show(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<Json<Vec<PermissionResponseDto>>, AppError> {
        let perms = RolePermissionsService::find_permissions_by_role_id(&state.db, id.0).await?;
        Ok(Json(perms))
    }
}
