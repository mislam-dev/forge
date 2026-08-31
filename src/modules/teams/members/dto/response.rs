use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::entities::team_member::Model as TeamMemberModel;

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamMemberResponse {
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: String,
}

impl TeamMemberResponse {
    pub fn from_model(model: TeamMemberModel) -> Self {
        Self {
            team_id: model.team_id,
            user_id: model.user_id,
            role: model.role,
            joined_at: model.joined_at.to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_team_member_response_from_model() {
        let team_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let now = Utc::now().into();

        let model = TeamMemberModel {
            team_id,
            user_id,
            role: "developer".to_string(),
            joined_at: now,
        };

        let res = TeamMemberResponse::from_model(model);
        assert_eq!(res.team_id, team_id);
        assert_eq!(res.user_id, user_id);
        assert_eq!(res.role, "developer");
    }
}
