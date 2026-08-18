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
