use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct InviteMemberRequest {
    pub user_id: Option<Uuid>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateMemberRoleRequest {
    pub role: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invite_member_request_validation_success() {
        let req = InviteMemberRequest {
            user_id: None,
            email: Some("user@example.com".to_string()),
            role: "admin".to_string(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_invite_member_request_validation_invalid_email() {
        let req = InviteMemberRequest {
            user_id: None,
            email: Some("invalid-email".to_string()),
            role: "admin".to_string(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_update_member_role_request_validation() {
        let req = UpdateMemberRoleRequest {
            role: "editor".to_string(),
        };
        assert!(req.validate().is_ok());
    }
}
