use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::{ValidationError, ValidationErrors};

use super::dto::request::{CreateOrganizationRequest, UpdateOrganizationRequest};
use super::dto::response::OrganizationResponse;
use super::service::OrganizationService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn create(
    State(state): State<AppState>,
    claims: JwtClaims,
    JsonValidate(mut payload): JsonValidate<CreateOrganizationRequest>,
) -> Result<ApiResponse<OrganizationResponse>, AppError> {
    let is_admin = claims.roles.iter().any(|r| r.eq_ignore_ascii_case("admin"));

    if is_admin && payload.owner_user_id.is_none() {
        let mut validation_errors = ValidationErrors::new();
        validation_errors.add(
            "owner_user_id",
            ValidationError::new("Owner user id is required"),
        );
        return Err(AppError::Validation(validation_errors));
    }

    if !is_admin {
        payload.owner_user_id = Some(claims.sub);
    }

    let res = OrganizationService::create_organization(&state.db, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Organization created.".to_string())
        .body(Some(res)))
}

pub async fn list(
    State(state): State<AppState>,
    claims: JwtClaims,
) -> Result<ApiResponse<Vec<OrganizationResponse>>, AppError> {
    let is_admin = claims.roles.iter().any(|r| r.eq_ignore_ascii_case("admin"));

    let res = OrganizationService::get_user_organizations(&state.db, claims.sub, is_admin).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Organizations retrieved.".to_string())
        .body(Some(res)))
}

pub async fn show(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<OrganizationResponse>, AppError> {
    let is_admin = claims.roles.iter().any(|r| r.eq_ignore_ascii_case("admin"));
    let res =
        OrganizationService::get_organization_by_id(&state.db, id, claims.sub, is_admin).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Organization retrieved.".to_string())
        .body(Some(res)))
}

pub async fn update(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<UpdateOrganizationRequest>,
) -> Result<ApiResponse<OrganizationResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let res =
        OrganizationService::update_organization(&state.db, id, claims.sub, is_admin, payload)
            .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Organization updated.".to_string())
        .body(Some(res)))
}

pub async fn remove(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    OrganizationService::delete_organization(&state.db, id, claims.sub, is_admin).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Organization deleted.".to_string()))
}
