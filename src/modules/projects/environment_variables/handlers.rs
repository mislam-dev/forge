use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{
    BulkCreateEnvVarRequest, CreateEnvVarRequest, EnvVarQuery, EnvVarResponse, UpdateEnvVarRequest,
};
use super::service::ProjectEnvironmentVariablesService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn create_env_var(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<CreateEnvVarRequest>,
) -> Result<ApiResponse<EnvVarResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let env_var = ProjectEnvironmentVariablesService::create_env_var(
        &state.db, claims.sub, is_admin, id, payload,
    )
    .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Environment variable created successfully.".to_string())
        .body(Some(env_var)))
}

pub async fn list_env_vars(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    Query(query): Query<EnvVarQuery>,
) -> Result<ApiResponse<Vec<EnvVarResponse>>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let env_vars = ProjectEnvironmentVariablesService::list_env_vars(
        &state.db, claims.sub, is_admin, id, query,
    )
    .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Environment variables retrieved successfully.".to_string())
        .body(Some(env_vars)))
}

pub async fn update_env_var(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, env_id)): Path<(Uuid, Uuid)>,
    JsonValidate(payload): JsonValidate<UpdateEnvVarRequest>,
) -> Result<ApiResponse<EnvVarResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let env_var = ProjectEnvironmentVariablesService::update_env_var(
        &state.db, claims.sub, is_admin, id, env_id, payload,
    )
    .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Environment variable updated successfully.".to_string())
        .body(Some(env_var)))
}

pub async fn delete_env_var(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, env_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<()>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    ProjectEnvironmentVariablesService::delete_env_var(&state.db, claims.sub, is_admin, id, env_id)
        .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Environment variable deleted successfully.".to_string()))
}

pub async fn bulk_create_env_vars(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<BulkCreateEnvVarRequest>,
) -> Result<ApiResponse<Vec<EnvVarResponse>>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let env_vars = ProjectEnvironmentVariablesService::bulk_create_env_vars(
        &state.db, claims.sub, is_admin, id, payload,
    )
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
        let req = CreateEnvVarRequest {
            environment: "".to_string(),
            key: "".to_string(),
            value: "".to_string(),
            is_secret: None,
        };
        assert!(req.validate().is_err());
    }
}
