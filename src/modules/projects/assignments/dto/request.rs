use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AssignProjectMemberRequest {
    pub user_id: Uuid,
    #[validate(length(min = 1, message = "Role is required"))]
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AssignProjectTeamRequest {
    pub team_id: Uuid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_member_request_validation() {
        let req = AssignProjectMemberRequest {
            user_id: Uuid::new_v4(),
            role: "developer".to_string(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_assign_team_request_validation() {
        let req = AssignProjectTeamRequest {
            team_id: Uuid::new_v4(),
        };
        assert!(req.validate().is_ok());
    }
}
