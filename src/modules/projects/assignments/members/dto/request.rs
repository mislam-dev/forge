use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use super::super::entities::sea_orm_active_enums::ProjectMembersRole;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AssignProjectMemberDTO {
    pub user_id: Uuid,
    pub role: ProjectMembersRole,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_member_request_validation() {
        let req = AssignProjectMemberDTO {
            user_id: Uuid::new_v4(),
            role: ProjectMembersRole::Developer,
        };
        assert!(req.validate().is_ok());
    }
}
