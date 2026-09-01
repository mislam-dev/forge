use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{
    AssignProjectMemberDTO, AssignProjectTeamDTO, ProjectMemberResponse, ProjectTeamResponse,
};
use super::service::ProjectAssignmentsService;
use crate::app::state::AppState;
use crate::modules::projects::extractors::{
    OptionalOrgAdmin, OptionalOrgViewer, OrgValidationOptional, OrgValidationRequired,
    RequiredOrgAdmin, RequiredOrgViewer,
};
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

pub async fn assign_member(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgAdmin,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<AssignProjectMemberDTO>,
) -> Result<ApiResponse<ProjectMemberResponse>, AppError> {
    let member = ProjectAssignmentsService::assign_member(&state.db, org_id, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Member assigned to project successfully.".to_string())
        .body(Some(member)))
}

pub async fn list_members(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgViewer,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<Vec<ProjectMemberResponse>>, AppError> {
    let members = ProjectAssignmentsService::list_members(&state.db, org_id, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Project members retrieved successfully.".to_string())
        .body(Some(members)))
}

pub async fn remove_member(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgAdmin,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<()>, AppError> {
    ProjectAssignmentsService::remove_member(&state.db, org_id, id, user_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Member removed from project successfully.".to_string()))
}

pub async fn assign_team(
    State(state): State<AppState>,
    OrgValidationRequired(_, org_id, _): RequiredOrgAdmin,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<AssignProjectTeamDTO>,
) -> Result<ApiResponse<ProjectTeamResponse>, AppError> {
    let team = ProjectAssignmentsService::assign_team(&state.db, org_id, id, payload).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Team assigned to project successfully.".to_string())
        .body(Some(team)))
}

pub async fn list_teams(
    State(state): State<AppState>,
    OrgValidationRequired(_, org_id, _): RequiredOrgViewer,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<Vec<ProjectTeamResponse>>, AppError> {
    let teams = ProjectAssignmentsService::list_teams(&state.db, org_id, id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Assigned project teams retrieved successfully.".to_string())
        .body(Some(teams)))
}

pub async fn remove_team(
    State(state): State<AppState>,
    OrgValidationRequired(_, org_id, _): RequiredOrgAdmin,
    Path((id, team_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<()>, AppError> {
    ProjectAssignmentsService::remove_team(&state.db, org_id, id, team_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Team removed from project successfully.".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::projects::assignments::entities::sea_orm_active_enums::ProjectMembersRole;
    use validator::Validate;

    #[test]
    fn test_assign_member_dto_validation() {
        let req = AssignProjectMemberDTO {
            user_id: Uuid::new_v4(),
            role: ProjectMembersRole::Developer,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_assign_team_dto_validation() {
        let req = AssignProjectTeamDTO {
            team_id: Uuid::new_v4(),
        };
        assert!(req.validate().is_ok());
    }
}
