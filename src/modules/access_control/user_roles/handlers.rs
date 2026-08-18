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
