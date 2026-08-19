use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
};
use uuid::Uuid;

use super::dto::{
    DeploymentHistoryQuery, DeploymentResponse, TriggerDeploymentRequest, UpdateDeploymentStatusRequest,
};
use super::service::DeploymentsService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::pagination::PaginatedResponse;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn trigger_deployment(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<TriggerDeploymentRequest>,
) -> Result<ApiResponse<DeploymentResponse>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let deployment = DeploymentsService::trigger_deployment(&state.db, claims.sub, is_admin, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Deployment triggered successfully.".to_string())
        .body(Some(deployment)))
}

pub async fn list_deployments(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    Query(query): Query<DeploymentHistoryQuery>,
) -> Result<ApiResponse<PaginatedResponse<DeploymentResponse>>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let paginated = DeploymentsService::list_deployments(&state.db, claims.sub, is_admin, id, query).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Deployments retrieved successfully.".to_string())
        .body(Some(paginated)))
}

pub async fn get_deployment(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, deployment_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<DeploymentResponse>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let deployment = DeploymentsService::get_deployment(&state.db, claims.sub, is_admin, id, deployment_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Deployment details retrieved successfully.".to_string())
        .body(Some(deployment)))
}

pub async fn update_status_internal(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(deployment_id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<UpdateDeploymentStatusRequest>,
) -> Result<ApiResponse<DeploymentResponse>, AppError> {
    let service_token = headers
        .get("x-service-token")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();

    let deployment = DeploymentsService::update_status_internal(
        &state.db,
        &state.config,
        service_token,
        deployment_id,
        payload,
    )
    .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Deployment status updated successfully.".to_string())
        .body(Some(deployment)))
}

pub async fn redeploy(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, deployment_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<DeploymentResponse>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let deployment = DeploymentsService::redeploy(&state.db, claims.sub, is_admin, id, deployment_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Redeploy triggered successfully.".to_string())
        .body(Some(deployment)))
}

pub async fn rollback(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<DeploymentResponse>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let deployment = DeploymentsService::rollback(&state.db, claims.sub, is_admin, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Rollback deployment triggered successfully.".to_string())
        .body(Some(deployment)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_status_internal_handler_extracts_token() {
        let mut headers = HeaderMap::new();
        headers.insert("x-service-token", "secret_token".parse().unwrap());

        let token = headers
            .get("x-service-token")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();

        assert_eq!(token, "secret_token");
    }
}
