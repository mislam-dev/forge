use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{CreateProjectRequest, ProjectQuery, ProjectResponse, UpdateProjectRequest};
use super::service::ProjectsService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn create_project(
    State(state): State<AppState>,
    claims: JwtClaims,
    JsonValidate(payload): JsonValidate<CreateProjectRequest>,
) -> Result<ApiResponse<ProjectResponse>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let project = ProjectsService::create_project(&state.db, claims.sub, is_admin, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Project created successfully.".to_string())
        .body(Some(project)))
}

pub async fn list_projects(
    State(state): State<AppState>,
    claims: JwtClaims,
    Query(query): Query<ProjectQuery>,
) -> Result<ApiResponse<Vec<ProjectResponse>>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let projects = ProjectsService::list_projects(&state.db, claims.sub, is_admin, query).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Projects retrieved successfully.".to_string())
        .body(Some(projects)))
}

pub async fn get_project(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<ProjectResponse>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let project = ProjectsService::get_project(&state.db, claims.sub, is_admin, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Project retrieved successfully.".to_string())
        .body(Some(project)))
}

pub async fn update_project(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<UpdateProjectRequest>,
) -> Result<ApiResponse<ProjectResponse>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let project = ProjectsService::update_project(&state.db, claims.sub, is_admin, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Project updated successfully.".to_string())
        .body(Some(project)))
}

pub async fn delete_project(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    ProjectsService::delete_project(&state.db, claims.sub, is_admin, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Project deleted successfully.".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_create_project_handler_validation() {
        let req = CreateProjectRequest {
            organization_id: Uuid::new_v4(),
            name: "P".to_string(),
            description: None,
            project_type: "".to_string(),
            runtime: "".to_string(),
            port: None,
            health_check_url: None,
        };
        assert!(req.validate().is_err());
    }
}
