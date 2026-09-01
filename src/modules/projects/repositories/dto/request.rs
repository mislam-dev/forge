use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ConnectProjectRepositoryDTO {
    #[validate(length(min = 5, message = "Repository URL must be valid"))]
    pub repository_url: String,
    pub auth_type: Option<String>,
    pub access_token: Option<String>,
    pub default_branch: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateProjectRepositoryDTO {
    #[validate(length(min = 5, message = "Repository URL must be valid"))]
    pub repository_url: Option<String>,
    pub auth_type: Option<String>,
    pub access_token: Option<String>,
    pub default_branch: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_repository_dto_validation() {
        let req = ConnectProjectRepositoryDTO {
            repository_url: "https://github.com/org/repo.git".to_string(),
            auth_type: Some("pat".to_string()),
            access_token: Some("ghp_secret123".to_string()),
            default_branch: Some("main".to_string()),
        };
        assert!(req.validate().is_ok());

        let invalid = ConnectProjectRepositoryDTO {
            repository_url: "git".to_string(),
            auth_type: None,
            access_token: None,
            default_branch: None,
        };
        assert!(invalid.validate().is_err());
    }
}
