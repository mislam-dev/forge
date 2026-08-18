use crate::app::state::AppState;
use crate::modules::access_control::roles::dto::{
    request::{RoleCreateDto, RoleUpdateDto},
    response::RoleResponseDto,
};
use crate::modules::access_control::roles::service::RolesService;
use crate::shared::{error::AppError, utils::IdParams, validation::JsonValidate};
use axum::{
    Json,
    extract::{Path, State},
};

pub struct RolesHandlers;

impl RolesHandlers {
    pub async fn list(
        State(state): State<AppState>,
    ) -> Result<Json<Vec<RoleResponseDto>>, AppError> {
        let users = RolesService::find(&state.db).await?;
        Ok(Json(users))
    }

    pub async fn show(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<Json<RoleResponseDto>, AppError> {
        let user = RolesService::find_by_id(&state.db, id.0).await?;
        Ok(Json(user))
    }

    pub async fn add(
        State(state): State<AppState>,
        JsonValidate(payload): JsonValidate<RoleCreateDto>,
    ) -> Result<Json<RoleResponseDto>, AppError> {
        let user = RolesService::create(&state.db, payload).await?;
        Ok(Json(user))
    }

    pub async fn update(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
        JsonValidate(payload): JsonValidate<RoleUpdateDto>,
    ) -> Result<Json<RoleResponseDto>, AppError> {
        let user = RolesService::update(&state.db, id.0, payload).await?;
        Ok(Json(user))
    }

    pub async fn remove(
        State(state): State<AppState>,
        Path(id): Path<IdParams>,
    ) -> Result<(), AppError> {
        let _ = RolesService::remove(&state.db, id.0).await?;

        Ok(())
    }
}
