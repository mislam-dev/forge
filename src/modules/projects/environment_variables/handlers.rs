use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{
    BulkCreateProjectEnvVarDTO, CreateProjectEnvVarDTO, ProjectEnvVarQueryDTO,
    ProjectEnvVarResponse, UpdateProjectEnvVarDTO,
};
use super::service::ProjectEnvironmentVariablesService;
use crate::app::state::AppState;
use crate::modules::projects::extractors::{
    OptionalOrgAdmin, OptionalOrgViewer, OrgValidationOptional,
};
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn create_env_var(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgAdmin,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<CreateProjectEnvVarDTO>,
) -> Result<ApiResponse<ProjectEnvVarResponse>, AppError> {
    let env_var =
        ProjectEnvironmentVariablesService::create_env_var(&state.db, org_id, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Environment variable created successfully.".to_string())
        .body(Some(env_var)))
}

pub async fn list_env_vars(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgViewer,
    Path(id): Path<Uuid>,
    Query(query): Query<ProjectEnvVarQueryDTO>,
) -> Result<ApiResponse<Vec<ProjectEnvVarResponse>>, AppError> {
    let env_vars =
        ProjectEnvironmentVariablesService::list_env_vars(&state.db, org_id, id, query).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Environment variables retrieved successfully.".to_string())
        .body(Some(env_vars)))
}

pub async fn update_env_var(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgAdmin,
    Path((id, env_id)): Path<(Uuid, Uuid)>,
    JsonValidate(payload): JsonValidate<UpdateProjectEnvVarDTO>,
) -> Result<ApiResponse<ProjectEnvVarResponse>, AppError> {
    let env_var = ProjectEnvironmentVariablesService::update_env_var(
        &state.db, org_id, id, env_id, payload,
    )
    .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Environment variable updated successfully.".to_string())
        .body(Some(env_var)))
}

pub async fn delete_env_var(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgAdmin,
    Path((id, env_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<()>, AppError> {
    ProjectEnvironmentVariablesService::delete_env_var(&state.db, org_id, id, env_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Environment variable deleted successfully.".to_string()))
}

pub async fn bulk_create_env_vars(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgAdmin,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<BulkCreateProjectEnvVarDTO>,
) -> Result<ApiResponse<Vec<ProjectEnvVarResponse>>, AppError> {
    let env_vars =
        ProjectEnvironmentVariablesService::bulk_create_env_vars(&state.db, org_id, id, payload)
            .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Environment variables created in bulk successfully.".to_string())
        .body(Some(env_vars)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_create_env_var_handler_validation() {
        let req = CreateProjectEnvVarDTO {
            environment: "".to_string(),
            key: "".to_string(),
            value: "".to_string(),
            is_secret: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_bulk_create_handler_validation() {
        let req = BulkCreateProjectEnvVarDTO {
            environment: "".to_string(),
            vars: vec![],
        };
        assert!(req.validate().is_err());
    }
}
