use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
};
use uuid::Uuid;

use super::dto::{
    DeploymentHistoryQuery, DeploymentResponse, TriggerDeploymentRequest,
    UpdateDeploymentStatusRequest,
};
use super::service::DeploymentsService;
use crate::app::state::AppState;
use crate::modules::projects::extractors::{
    OptionalOrgAdmin, OptionalOrgEditor, OptionalOrgViewer, OrgValidationOptional,
};
use crate::shared::error::AppError;
use crate::shared::pagination::PaginatedResponse;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn trigger_deployment(
    State(state): State<AppState>,
    OrgValidationOptional(claims, org_id, _): OptionalOrgEditor,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<TriggerDeploymentRequest>,
) -> Result<ApiResponse<DeploymentResponse>, AppError> {
    let deployment =
        DeploymentsService::trigger_deployment(&state.db, org_id, id, claims.sub, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Deployment triggered successfully.".to_string())
        .body(Some(deployment)))
}

pub async fn list_deployments(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgViewer,
    Path(id): Path<Uuid>,
    Query(query): Query<DeploymentHistoryQuery>,
) -> Result<ApiResponse<PaginatedResponse<DeploymentResponse>>, AppError> {
    let paginated = DeploymentsService::list_deployments(&state.db, org_id, id, query).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Deployments retrieved successfully.".to_string())
        .body(Some(paginated)))
}

pub async fn get_deployment(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgViewer,
    Path((id, deployment_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<DeploymentResponse>, AppError> {
    let deployment =
        DeploymentsService::get_deployment(&state.db, org_id, id, deployment_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Deployment details retrieved successfully.".to_string())
        .body(Some(deployment)))
}

pub async fn redeploy(
    State(state): State<AppState>,
    OrgValidationOptional(claims, org_id, _): OptionalOrgEditor,
    Path((id, deployment_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<DeploymentResponse>, AppError> {
    let deployment =
        DeploymentsService::redeploy(&state.db, org_id, id, claims.sub, deployment_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Redeploy triggered successfully.".to_string())
        .body(Some(deployment)))
}

pub async fn rollback(
    State(state): State<AppState>,
    OrgValidationOptional(claims, org_id, _): OptionalOrgAdmin,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<DeploymentResponse>, AppError> {
    let deployment = DeploymentsService::rollback(&state.db, org_id, id, claims.sub).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Rollback deployment triggered successfully.".to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_trigger_deployment_handler_validation() {
        let req = TriggerDeploymentRequest {
            branch: Some("main".to_string()),
            commit_hash: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_update_deployment_status_handler_validation() {
        let req = UpdateDeploymentStatusRequest {
            status: "".to_string(),
            build_duration: None,
            deploy_duration: None,
            error_message: None,
        };
        assert!(req.validate().is_err());
    }
}
