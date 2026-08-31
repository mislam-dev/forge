use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use uuid::Uuid;

use super::dto::{CreateProjectDTO, ProjectResponse, UpdateProjectDTO};
use super::service::ProjectsService;
use crate::modules::projects::extractors::{
    OrgValidationOptional,
    organization_validation::{RequiredOrgAdmin, RequiredOrgOwner, RequiredOrgViewer},
};
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;
use crate::{
    app::state::AppState, modules::projects::extractors::organization_validation::RequiredOrgEditor,
};

pub async fn create_project(
    State(state): State<AppState>,
    OrgValidationOptional(claims, org_id, _): RequiredOrgEditor,
    JsonValidate(payload): JsonValidate<CreateProjectDTO>,
) -> Result<ApiResponse<ProjectResponse>, AppError> {
    let project = ProjectsService::create_project(&state.db, claims.sub, org_id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Project created successfully.".to_string())
        .body(Some(project)))
}

pub async fn list_projects(
    State(state): State<AppState>,
    OrgValidationOptional(claims, org_id, _): RequiredOrgViewer,
) -> Result<ApiResponse<Vec<ProjectResponse>>, AppError> {
    let projects = ProjectsService::list_projects(&state.db, claims.sub, org_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Projects retrieved successfully.".to_string())
        .body(Some(projects)))
}

pub async fn get_project(
    State(state): State<AppState>,
    OrgValidationOptional(claims, org_id, _): RequiredOrgViewer,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<ProjectResponse>, AppError> {
    let project = ProjectsService::get_project(&state.db, org_id, claims.sub, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Project retrieved successfully.".to_string())
        .body(Some(project)))
}

pub async fn update_project(
    State(state): State<AppState>,
    OrgValidationOptional(claims, org_id, _): RequiredOrgAdmin,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<UpdateProjectDTO>,
) -> Result<ApiResponse<ProjectResponse>, AppError> {
    let project =
        ProjectsService::update_project(&state.db, org_id, claims.sub, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Project updated successfully.".to_string())
        .body(Some(project)))
}

pub async fn delete_project(
    State(state): State<AppState>,
    OrgValidationOptional(claims, org_id, _): RequiredOrgOwner,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>, AppError> {
    let _r = ProjectsService::delete_project(&state.db, claims.sub, org_id, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Project deleted successfully.".to_string()))
}
