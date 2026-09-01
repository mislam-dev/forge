use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::project_members::Model as ProjectMemberModel;
use super::super::entities::project_teams::Model as ProjectTeamModel;
use crate::modules::projects::assignments::entities::sea_orm_active_enums::ProjectMembersRole;
use crate::modules::teams::teams::entities::team::Model as TeamModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMemberResponse {
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub role: Option<ProjectMembersRole>,
    pub assigned_at: String,
}

impl ProjectMemberResponse {
    pub fn from_model(model: ProjectMemberModel) -> Self {
        Self {
            project_id: model.project_id,
            user_id: model.user_id,
            role: model.role,
            assigned_at: model.assigned_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectTeamResponse {
    pub project_id: Uuid,
    pub team_id: Uuid,
    pub assigned_at: String,
    pub team: Option<TeamModel>,
}

impl ProjectTeamResponse {
    pub fn from_model(model: ProjectTeamModel, team: Option<TeamModel>) -> Self {
        Self {
            project_id: model.project_id,
            team_id: model.team_id,
            assigned_at: model.assigned_at.to_rfc3339(),
            team,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_project_member_response_from_model() {
        let project_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = Utc::now().into();

        let model = ProjectMemberModel {
            project_id,
            user_id,
            role: Some(ProjectMembersRole::Developer),
            assigned_at: now,
        };

        let res = ProjectMemberResponse::from_model(model);
        assert_eq!(res.project_id, project_id);
        assert_eq!(res.user_id, user_id);
        assert_eq!(res.role, Some(ProjectMembersRole::Developer));
    }

    #[test]
    fn test_project_team_response_from_model() {
        let project_id = Uuid::new_v4();
        let team_id = Uuid::new_v4();
        let now = Utc::now().into();

        let model = ProjectTeamModel {
            project_id,
            team_id,
            assigned_at: now,
        };

        let res = ProjectTeamResponse::from_model(model, None);
        assert_eq!(res.project_id, project_id);
        assert_eq!(res.team_id, team_id);
    }
}
