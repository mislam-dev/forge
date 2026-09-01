use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct AssignProjectTeamDTO {
    pub team_id: Uuid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_team_request_validation() {
        let req = AssignProjectTeamDTO {
            team_id: Uuid::new_v4(),
        };
        assert!(req.validate().is_ok());
    }
}
