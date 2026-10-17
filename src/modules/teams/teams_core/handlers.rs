use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{CreateTeamDTO, TeamResponse, UpdateTeamDTO};
use super::service::TeamsService;
use crate::modules::organization::permissions::extractors::{OrgIdHeader, RequireAdmin};
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;
use crate::{
    app::state::AppState,
    modules::organization::permissions::extractors::{RequireOrgRole, RequireViewer},
};

pub async fn create_team(
    State(state): State<AppState>,
    RequireOrgRole(claims, _): RequireViewer,
    JsonValidate(payload): JsonValidate<CreateTeamDTO>,
) -> Result<ApiResponse<TeamResponse>, AppError> {
    let team = TeamsService::create_team(&state.db, claims.sub, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Team created successfully.".to_string())
        .body(Some(team)))
}

pub async fn list_teams(
    State(state): State<AppState>,
    RequireOrgRole(_, _): RequireViewer,
    OrgIdHeader(org_id): OrgIdHeader,
) -> Result<ApiResponse<Vec<TeamResponse>>, AppError> {
    let teams = TeamsService::list_teams(&state.db, org_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Teams retrieved successfully.".to_string())
        .body(Some(teams)))
}

pub async fn get_team(
    State(state): State<AppState>,
    RequireOrgRole(_, _): RequireViewer,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<TeamResponse>, AppError> {
    let team = TeamsService::get_team_by_id(&state.db, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team retrieved successfully.".to_string())
        .body(Some(team)))
}

pub async fn update_team(
    State(state): State<AppState>,
    RequireOrgRole(_, _): RequireAdmin,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<UpdateTeamDTO>,
) -> Result<ApiResponse<TeamResponse>, AppError> {
    let team = TeamsService::update_team(&state.db, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team updated successfully.".to_string())
        .body(Some(team)))
}

pub async fn delete_team(
    State(state): State<AppState>,
    RequireOrgRole(_, _): RequireAdmin,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<()>, AppError> {
    TeamsService::delete_team(&state.db, id).await?;

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
        let req = CreateTeamDTO {
            organization_id: Uuid::new_v4(),
            name: "A".to_string(),
            descriptions: None,
        };
        assert!(req.validate().is_err());
    }
}
