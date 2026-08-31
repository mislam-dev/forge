use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{AddTeamMemberRequest, TeamMemberResponse, UpdateTeamMemberRoleRequest};
use super::service::TeamMembersService;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;
use crate::{
    app::state::AppState,
    modules::organization::permissions::extractors::{RequireAdmin, RequireOrgRole, RequireViewer},
};

pub async fn add_member(
    State(state): State<AppState>,
    RequireOrgRole(_, _): RequireAdmin,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<AddTeamMemberRequest>,
) -> Result<ApiResponse<TeamMemberResponse>, AppError> {
    let member = TeamMembersService::add_member(&state.db, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Member added to team successfully.".to_string())
        .body(Some(member)))
}

pub async fn list_members(
    State(state): State<AppState>,
    RequireOrgRole(_, _): RequireViewer,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<Vec<TeamMemberResponse>>, AppError> {
    let members = TeamMembersService::list_members(&state.db, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team members retrieved successfully.".to_string())
        .body(Some(members)))
}

pub async fn update_member(
    State(state): State<AppState>,
    RequireOrgRole(_, _): RequireAdmin,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
    JsonValidate(payload): JsonValidate<UpdateTeamMemberRoleRequest>,
) -> Result<ApiResponse<TeamMemberResponse>, AppError> {
    let member = TeamMembersService::update_member_role(&state.db, id, user_id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team member role updated successfully.".to_string())
        .body(Some(member)))
}

pub async fn remove_member(
    State(state): State<AppState>,
    RequireOrgRole(_, _): RequireAdmin,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<()>, AppError> {
    TeamMembersService::remove_member(&state.db, id, user_id).await?;

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
