use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{
    ConnectProjectRepositoryDTO, ProjectRepositoryResponse, UpdateProjectRepositoryDTO,
};
use super::service::ProjectRepositoriesService;
use crate::app::state::AppState;
use crate::modules::projects::extractors::{
    OptionalOrgAdmin, OptionalOrgViewer, OrgValidationOptional,
};
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn connect_repository(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgAdmin,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<ConnectProjectRepositoryDTO>,
) -> Result<ApiResponse<ProjectRepositoryResponse>, AppError> {
    let repository =
        ProjectRepositoriesService::connect_repository(&state.db, org_id, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Repository connected successfully.".to_string())
        .body(Some(repository)))
}

pub async fn get_repository(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgViewer,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<ProjectRepositoryResponse>, AppError> {
    let repository = ProjectRepositoriesService::get_repository(&state.db, org_id, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Repository details retrieved successfully.".to_string())
        .body(Some(repository)))
}

pub async fn update_repository(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgAdmin,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<UpdateProjectRepositoryDTO>,
) -> Result<ApiResponse<ProjectRepositoryResponse>, AppError> {
    let repository =
        ProjectRepositoriesService::update_repository(&state.db, org_id, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Repository updated successfully.".to_string())
        .body(Some(repository)))
}

pub async fn disconnect_repository(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgAdmin,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>, AppError> {
    ProjectRepositoriesService::disconnect_repository(&state.db, org_id, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Repository disconnected successfully.".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_connect_repository_handler_validation() {
        let req = ConnectProjectRepositoryDTO {
            repository_url: "git".to_string(),
            auth_type: None,
            access_token: None,
            default_branch: None,
        };
        assert!(req.validate().is_err());
    }
}
