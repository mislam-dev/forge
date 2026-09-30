use super::super::entities::sea_orm_active_enums::ProjectEnvironmentVariablesEnvironment as EnvironmentEnum;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateProjectEnvVarDTO {
    pub environment: EnvironmentEnum,
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
    pub environment: EnvironmentEnum,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct BulkCreateProjectEnvVarDTO {
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
            environment: EnvironmentEnum::Development,
            key: "DATABASE_URL".to_string(),
            value: "postgres://db.example.com".to_string(),
            is_secret: Some(true),
        };
        assert!(req.validate().is_ok());

        let invalid_req = CreateProjectEnvVarDTO {
            environment: EnvironmentEnum::Development,
            key: "".to_string(),
            value: "".to_string(),
            is_secret: None,
        };
        assert!(invalid_req.validate().is_err());
    }

    #[test]
    fn test_bulk_create_dto_validation() {
        let req = BulkCreateProjectEnvVarDTO {
            vars: vec![ProjectEnvVarItemDTO {
                key: "PORT".to_string(),
                environment: EnvironmentEnum::Development,
                value: "8080".to_string(),
                is_secret: Some(false),
            }],
        };
        assert!(req.validate().is_ok());

        let invalid_req = BulkCreateProjectEnvVarDTO { vars: vec![] };
        assert!(invalid_req.validate().is_err());
    }
}
