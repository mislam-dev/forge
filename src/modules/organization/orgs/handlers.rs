use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::{ValidationError, ValidationErrors};

use super::dto::request::{CreateOrganizationRequest, UpdateOrganizationRequest};
use super::dto::response::OrganizationResponse;
use super::service::OrganizationService;
use crate::modules::{
    auth::token::JwtClaims,
    organization::permissions::extractors::{RequireOwner, RequireViewer},
};
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;
use crate::{
    app::state::AppState,
    modules::organization::permissions::extractors::{RequireAdmin, RequireOrgRole},
};

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
    RequireOrgRole(_, _): RequireViewer,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<OrganizationResponse>, AppError> {
    let res = OrganizationService::get_organization_by_id(&state.db, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Organization retrieved.".to_string())
        .body(Some(res)))
}

pub async fn update(
    State(state): State<AppState>,
    RequireOrgRole(claims, _): RequireAdmin,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<UpdateOrganizationRequest>,
) -> Result<ApiResponse<OrganizationResponse>, AppError> {
    let res = OrganizationService::update_organization(&state.db, id, claims.sub, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Organization updated.".to_string())
        .body(Some(res)))
}

pub async fn remove(
    State(state): State<AppState>,
    RequireOrgRole(_, _): RequireOwner,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>, AppError> {
    OrganizationService::delete_organization(&state.db, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Organization deleted.".to_string()))
}
