use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{AddTeamMemberRequest, TeamMemberResponse, UpdateTeamMemberRoleRequest};
use super::service::TeamMembersService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn add_member(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<AddTeamMemberRequest>,
) -> Result<ApiResponse<TeamMemberResponse>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let member = TeamMembersService::add_member(&state.db, claims.sub, is_admin, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Member added to team successfully.".to_string())
        .body(Some(member)))
}

pub async fn list_members(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<Vec<TeamMemberResponse>>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let members = TeamMembersService::list_members(&state.db, claims.sub, is_admin, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team members retrieved successfully.".to_string())
        .body(Some(members)))
}

pub async fn update_member(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
    JsonValidate(payload): JsonValidate<UpdateTeamMemberRoleRequest>,
) -> Result<ApiResponse<TeamMemberResponse>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let member = TeamMembersService::update_member_role(&state.db, claims.sub, is_admin, id, user_id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team member role updated successfully.".to_string())
        .body(Some(member)))
}

pub async fn remove_member(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<()>, AppError> {
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    TeamMembersService::remove_member(&state.db, claims.sub, is_admin, id, user_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team member removed successfully.".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn test_add_member_handler_validation() {
        let req = AddTeamMemberRequest {
            user_id: Uuid::new_v4(),
            role: "".to_string(),
        };
        assert!(req.validate().is_err());
    }
}
