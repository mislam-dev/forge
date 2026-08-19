use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{CreateTeamRequest, TeamQuery, TeamResponse, UpdateTeamRequest};
use super::service::TeamsService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn create_team(
    State(state): State<AppState>,
    claims: JwtClaims,
    JsonValidate(payload): JsonValidate<CreateTeamRequest>,
) -> Result<ApiResponse<TeamResponse>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let team = TeamsService::create_team(&state.db, claims.sub, is_admin, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Team created successfully.".to_string())
        .body(Some(team)))
}

pub async fn list_teams(
    State(state): State<AppState>,
    claims: JwtClaims,
    Query(query): Query<TeamQuery>,
) -> Result<ApiResponse<Vec<TeamResponse>>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let teams = TeamsService::list_teams(&state.db, claims.sub, is_admin, query).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Teams retrieved successfully.".to_string())
        .body(Some(teams)))
}

pub async fn get_team(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<TeamResponse>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let team = TeamsService::get_team_by_id(&state.db, claims.sub, is_admin, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team retrieved successfully.".to_string())
        .body(Some(team)))
}

pub async fn update_team(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<UpdateTeamRequest>,
) -> Result<ApiResponse<TeamResponse>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let team = TeamsService::update_team(&state.db, claims.sub, is_admin, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team updated successfully.".to_string())
        .body(Some(team)))
}

pub async fn delete_team(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    TeamsService::delete_team(&state.db, claims.sub, is_admin, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team deleted successfully.".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_create_team_handler_validation() {
        let req = CreateTeamRequest {
            organization_id: Uuid::new_v4(),
            name: "A".to_string(),
            descriptions: None,
        };
        assert!(req.validate().is_err());
    }
}
