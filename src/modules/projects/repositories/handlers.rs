use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{ConnectRepositoryRequest, RepositoryResponse, UpdateRepositoryRequest};
use super::service::ProjectRepositoriesService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn connect_repository(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<ConnectRepositoryRequest>,
) -> Result<ApiResponse<RepositoryResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let repository = ProjectRepositoriesService::connect_repository(
        &state.db, claims.sub, is_admin, id, payload,
    )
    .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Repository connected successfully.".to_string())
        .body(Some(repository)))
}

pub async fn get_repository(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<RepositoryResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let repository =
        ProjectRepositoriesService::get_repository(&state.db, claims.sub, is_admin, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Repository details retrieved successfully.".to_string())
        .body(Some(repository)))
}

pub async fn update_repository(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<UpdateRepositoryRequest>,
) -> Result<ApiResponse<RepositoryResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let repository =
        ProjectRepositoriesService::update_repository(&state.db, claims.sub, is_admin, id, payload)
            .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Repository updated successfully.".to_string())
        .body(Some(repository)))
}

pub async fn disconnect_repository(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    ProjectRepositoriesService::disconnect_repository(&state.db, claims.sub, is_admin, id).await?;

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
        let req = ConnectRepositoryRequest {
            repository_url: "a".to_string(),
            auth_type: None,
            access_token: None,
            default_branch: None,
        };
        assert!(req.validate().is_err());
    }
}
