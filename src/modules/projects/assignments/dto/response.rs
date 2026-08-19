use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::project_member::Model as ProjectMemberModel;
use super::super::entities::project_team::Model as ProjectTeamModel;
use crate::modules::teams::teams::entities::team::Model as TeamModel;
use crate::modules::users::entities::users::Model as UserModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMemberResponse {
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub assigned_at: String,
    pub user: Option<UserModel>,
}

impl ProjectMemberResponse {
    pub fn from_model(model: ProjectMemberModel, user: Option<UserModel>) -> Self {
        Self {
            project_id: model.project_id,
            user_id: model.user_id,
            role: model.role,
            assigned_at: model.assigned_at.to_rfc3339(),
            user,
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
            role: "developer".to_string(),
            assigned_at: now,
        };

        let res = ProjectMemberResponse::from_model(model, None);
        assert_eq!(res.project_id, project_id);
        assert_eq!(res.user_id, user_id);
        assert_eq!(res.role, "developer");
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
