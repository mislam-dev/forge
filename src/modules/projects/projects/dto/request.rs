use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateProjectRequest {
    pub organization_id: Uuid,
    #[validate(length(min = 2, max = 255, message = "Project name must be between 2 and 255 characters"))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1, message = "project_type is required"))]
    pub project_type: String,
    #[validate(length(min = 1, message = "runtime is required"))]
    pub runtime: String,
    pub port: Option<i32>,
    pub health_check_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub project_type: Option<String>,
    pub runtime: Option<String>,
    pub port: Option<i32>,
    pub health_check_url: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectQuery {
    pub organization_id: Option<Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project_request_validation_success() {
        let req = CreateProjectRequest {
            organization_id: Uuid::new_v4(),
            name: "Forge API".to_string(),
            description: Some("Core Backend Service".to_string()),
            project_type: "repo".to_string(),
            runtime: "Rust".to_string(),
            port: Some(8080),
            health_check_url: Some("/health".to_string()),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_create_project_request_validation_failure() {
        let req = CreateProjectRequest {
            organization_id: Uuid::new_v4(),
            name: "F".to_string(),
            description: None,
            project_type: "".to_string(),
            runtime: "".to_string(),
            port: None,
            health_check_url: None,
        };
        assert!(req.validate().is_err());
    }
}
