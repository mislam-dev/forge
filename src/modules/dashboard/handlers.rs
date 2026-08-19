use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{OrgDashboardResponse, SystemDashboardResponse, UserDashboardResponse};
use super::service::DashboardService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;

pub async fn get_org_dashboard(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(org_id): Path<Uuid>,
) -> Result<ApiResponse<OrgDashboardResponse>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let dashboard =
        DashboardService::get_org_dashboard(&state.db, claims.sub, is_admin, org_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Organization dashboard loaded successfully.".to_string())
        .body(Some(dashboard)))
}

pub async fn get_user_dashboard(
    State(state): State<AppState>,
    claims: JwtClaims,
) -> Result<ApiResponse<UserDashboardResponse>, AppError> {
    let dashboard = DashboardService::get_user_dashboard(&state.db, claims.sub).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("User dashboard loaded successfully.".to_string())
        .body(Some(dashboard)))
}

pub async fn get_system_dashboard(
    State(state): State<AppState>,
    claims: JwtClaims,
) -> Result<ApiResponse<SystemDashboardResponse>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let dashboard = DashboardService::get_system_dashboard(&state.db, claims.sub, is_admin).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("System dashboard loaded successfully.".to_string())
        .body(Some(dashboard)))
}
