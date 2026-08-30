use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateOrganizationRequest {
    #[validate(length(
        min = 2,
        max = 255,
        message = "Name must be between 2 and 255 characters"
    ))]
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub owner_user_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateOrganizationRequest {
    #[validate(length(
        min = 2,
        max = 255,
        message = "Name must be between 2 and 255 characters"
    ))]
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_org_request_validation_success() {
        let req = CreateOrganizationRequest {
            name: "Acme Inc".to_string(),
            slug: Some("acme".to_string()),
            description: Some("Description".to_string()),
            logo_url: None,
            owner_user_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_create_org_request_validation_failure() {
        let req = CreateOrganizationRequest {
            name: "A".to_string(),
            slug: None,
            description: None,
            logo_url: None,
            owner_user_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn test_update_org_request_validation() {
        let req = UpdateOrganizationRequest {
            name: Some("Updated Name".to_string()),
            slug: None,
            description: None,
            logo_url: None,
        };
        assert!(req.validate().is_ok());
    }
}
