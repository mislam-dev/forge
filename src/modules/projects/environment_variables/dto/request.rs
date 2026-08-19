use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateEnvVarRequest {
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
pub struct UpdateEnvVarRequest {
    pub value: Option<String>,
    pub is_secret: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnvVarItem {
    pub key: String,
    pub value: String,
    pub is_secret: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BulkCreateEnvVarRequest {
    #[validate(length(min = 1, message = "Environment is required"))]
    pub environment: String,
    pub vars: Vec<EnvVarItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EnvVarQuery {
    pub environment: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_env_var_request_validation() {
        let req = CreateEnvVarRequest {
            environment: "Production".to_string(),
            key: "DATABASE_URL".to_string(),
            value: "postgres://db.example.com".to_string(),
            is_secret: Some(true),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn test_bulk_create_request_validation() {
        let req = BulkCreateEnvVarRequest {
            environment: "Development".to_string(),
            vars: vec![EnvVarItem {
                key: "PORT".to_string(),
                value: "8080".to_string(),
                is_secret: Some(false),
            }],
        };
        assert!(req.validate().is_ok());
    }
}
