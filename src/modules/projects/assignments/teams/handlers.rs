use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use super::dto::{AssignProjectTeamDTO, ProjectTeamResponse};
use super::service::ProjectAssignmentsService;
use crate::app::state::AppState;
use crate::modules::projects::extractors::{
    OrgValidationRequired, RequiredOrgAdmin, RequiredOrgViewer,
};
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

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
    use validator::Validate;

    #[test]
    fn test_assign_team_dto_validation() {
        let req = AssignProjectTeamDTO {
            team_id: Uuid::new_v4(),
        };
        assert!(req.validate().is_ok());
    }
}
