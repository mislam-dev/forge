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
