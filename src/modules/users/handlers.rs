use super::service::UserService;
use crate::app::state::AppState;
use crate::modules::users::dto::{
    request::{CreateUserDto, UpdateUserDto},
    response::UserItemResponse,
};
use crate::shared::pagination::{PaginatedResponse, PaginationParams};
use crate::shared::response::ApiResponse;
use crate::shared::{error::AppError, utils::IdParams, validation::JsonValidate};
use axum::extract::Query;
use axum::extract::{Path, State};
use axum::http::StatusCode;

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<ApiResponse<PaginatedResponse<UserItemResponse>>, AppError> {
    let users = UserService::find(&state.db, &params).await?;
    Ok(ApiResponse::new()
        .message("Users fetched successfully!".to_string())
        .body(Some(users)))
}

pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<IdParams>,
) -> Result<ApiResponse<UserItemResponse>, AppError> {
    let user = UserService::find_one(&state.db, id.0).await?;
    Ok(ApiResponse::new()
        .message("User fetched successfully!".to_string())
        .body(Some(user)))
}

pub async fn add(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<CreateUserDto>,
) -> Result<ApiResponse<UserItemResponse>, AppError> {
    let user = UserService::create(&state.db, payload).await?;
    Ok(ApiResponse::new()
        .message("User created successfully!".to_string())
        .status(StatusCode::CREATED)
        .body(Some(user)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<IdParams>,
    JsonValidate(payload): JsonValidate<UpdateUserDto>,
) -> Result<ApiResponse<UserItemResponse>, AppError> {
    let user = UserService::update(&state.db, id.0, payload).await?;
    Ok(ApiResponse::new()
        .message("User updated successfully!".to_string())
        .body(Some(user)))
}

pub async fn remove(
    State(state): State<AppState>,
    Path(id): Path<IdParams>,
) -> Result<ApiResponse<UserItemResponse>, AppError> {
    let _ = UserService::remove(&state.db, id.0).await?;
    Ok(ApiResponse::new().status(StatusCode::NO_CONTENT))
}
