use super::service::UserService;
use crate::app::state::AppState;
use crate::modules::users::dto::{
    request::{CreateUserDto, UpdateUserDto},
    response::UserItemResponse,
};
use crate::shared::{error::AppError, utils::IdParams, validation::JsonValidate};
use axum::{
    Json,
    extract::{Path, State},
};

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<UserItemResponse>>, AppError> {
    let users = UserService::find(&state.db).await?;
    Ok(Json(users))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<IdParams>,
) -> Result<Json<UserItemResponse>, AppError> {
    let user = UserService::find_one(&state.db, id.0).await?;
    Ok(Json(user))
}

pub async fn add(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<CreateUserDto>,
) -> Result<Json<UserItemResponse>, AppError> {
    let user = UserService::create(&state.db, payload).await?;
    Ok(Json(user))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<IdParams>,
    JsonValidate(payload): JsonValidate<UpdateUserDto>,
) -> Result<Json<UserItemResponse>, AppError> {
    let user = UserService::update(&state.db, id.0, payload).await?;
    Ok(Json(user))
}

pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<IdParams>,
) -> Result<(), AppError> {
    let _ = UserService::remove(&state.db, id.0).await?;

    Ok(())
}
