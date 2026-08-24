use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{
    AssignProjectMemberRequest, AssignProjectTeamRequest, ProjectMemberResponse,
    ProjectTeamResponse,
};
use super::service::ProjectAssignmentsService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn assign_member(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<AssignProjectMemberRequest>,
) -> Result<ApiResponse<ProjectMemberResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let member =
        ProjectAssignmentsService::assign_member(&state.db, claims.sub, is_admin, id, payload)
            .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Member assigned to project successfully.".to_string())
        .body(Some(member)))
}

pub async fn list_members(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<Vec<ProjectMemberResponse>>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let members =
        ProjectAssignmentsService::list_members(&state.db, claims.sub, is_admin, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Project members retrieved successfully.".to_string())
        .body(Some(members)))
}

pub async fn remove_member(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<()>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    ProjectAssignmentsService::remove_member(&state.db, claims.sub, is_admin, id, user_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Member removed from project successfully.".to_string()))
}

pub async fn assign_team(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<AssignProjectTeamRequest>,
) -> Result<ApiResponse<ProjectTeamResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let team = ProjectAssignmentsService::assign_team(&state.db, claims.sub, is_admin, id, payload)
        .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Team assigned to project successfully.".to_string())
        .body(Some(team)))
}

pub async fn list_teams(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<Vec<ProjectTeamResponse>>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let teams = ProjectAssignmentsService::list_teams(&state.db, claims.sub, is_admin, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Assigned project teams retrieved successfully.".to_string())
        .body(Some(teams)))
}

pub async fn remove_team(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, team_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<()>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    ProjectAssignmentsService::remove_team(&state.db, claims.sub, is_admin, id, team_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team removed from project successfully.".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_assign_member_handler_validation() {
        let req = AssignProjectMemberRequest {
            user_id: Uuid::new_v4(),
            role: "".to_string(),
        };
        assert!(req.validate().is_err());
    }
}
