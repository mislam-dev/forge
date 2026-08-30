use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct InviteMemberRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub role: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateMemberRoleRequest {
    pub role: String,
}
