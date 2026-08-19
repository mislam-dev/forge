use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateTeamRequest {
    pub organization_id: Uuid,
    #[validate(length(min = 2, max = 255, message = "Team name must be between 2 and 255 characters"))]
    pub name: String,
    pub descriptions: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub descriptions: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TeamQuery {
    pub organization_id: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_team_request_validation_success() {
        let req = CreateTeamRequest {
            organization_id: Uuid::new_v4(),
            name: "Backend Engineering".to_string(),
            descriptions: Some("Core API team".to_string()),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_create_team_request_validation_failure() {
        let req = CreateTeamRequest {
            organization_id: Uuid::new_v4(),
            name: "A".to_string(),
            descriptions: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_update_team_request_validation() {
        let req = UpdateTeamRequest {
            name: Some("Frontend Team".to_string()),
            descriptions: None,
        };
        assert!(req.validate().is_ok());
    }
}
