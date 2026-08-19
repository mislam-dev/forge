use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AddTeamMemberRequest {
    pub user_id: Uuid,
    #[validate(length(min = 1, message = "Role cannot be empty"))]
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTeamMemberRoleRequest {
    #[validate(length(min = 1, message = "Role cannot be empty"))]
    pub role: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_team_member_request_validation() {
        let req = AddTeamMemberRequest {
            user_id: Uuid::new_v4(),
            role: "developer".to_string(),
        };
        assert!(req.validate().is_ok());

        let invalid = AddTeamMemberRequest {
            user_id: Uuid::new_v4(),
            role: "".to_string(),
        };
        assert!(invalid.validate().is_err());
    }

    #[test]
    fn test_update_team_member_role_request_validation() {
        let req = UpdateTeamMemberRoleRequest {
            role: "admin".to_string(),
        };
        assert!(req.validate().is_ok());
    }
}
