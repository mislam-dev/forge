use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::modules::projects::assignments::entities::sea_orm_active_enums::ProjectMembersRole;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AssignProjectMemberDTO {
    pub user_id: Uuid,
    pub role: ProjectMembersRole,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AssignProjectTeamDTO {
    pub team_id: Uuid,
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

    #[test]
    fn test_assign_team_request_validation() {
        let req = AssignProjectTeamDTO {
            team_id: Uuid::new_v4(),
        };
        assert!(req.validate().is_ok());
    }
}
