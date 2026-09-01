use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateProjectEnvVarDTO {
    #[validate(length(min = 1, message = "Environment is required"))]
    pub environment: String,
    #[validate(length(
        min = 1,
        max = 255,
        message = "Key must be between 1 and 255 characters"
    ))]
    pub key: String,
    pub value: String,
    pub is_secret: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateProjectEnvVarDTO {
    pub value: Option<String>,
    pub is_secret: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct ProjectEnvVarItemDTO {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Key must be between 1 and 255 characters"
    ))]
    pub key: String,
    pub value: String,
    pub is_secret: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BulkCreateProjectEnvVarDTO {
    #[validate(length(min = 1, message = "Environment is required"))]
    pub environment: String,
    #[validate(nested)]
    pub vars: Vec<ProjectEnvVarItemDTO>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectEnvVarQueryDTO {
    pub environment: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_env_var_dto_validation() {
        let req = CreateProjectEnvVarDTO {
            environment: "Production".to_string(),
            key: "DATABASE_URL".to_string(),
            value: "postgres://db.example.com".to_string(),
            is_secret: Some(true),
        };
        assert!(req.validate().is_ok());

        let invalid_req = CreateProjectEnvVarDTO {
            environment: "".to_string(),
            key: "".to_string(),
            value: "".to_string(),
            is_secret: None,
        };
        assert!(invalid_req.validate().is_err());
    }

    #[test]
    fn test_bulk_create_dto_validation() {
        let req = BulkCreateProjectEnvVarDTO {
            environment: "Development".to_string(),
            vars: vec![ProjectEnvVarItemDTO {
                key: "PORT".to_string(),
                value: "8080".to_string(),
                is_secret: Some(false),
            }],
        };
        assert!(req.validate().is_ok());

        let invalid_req = BulkCreateProjectEnvVarDTO {
            environment: "".to_string(),
            vars: vec![],
        };
        assert!(invalid_req.validate().is_err());
    }
}
