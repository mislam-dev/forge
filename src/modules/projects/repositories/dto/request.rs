use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ConnectRepositoryRequest {
    #[validate(length(min = 5, message = "Repository URL must be valid"))]
    pub repository_url: String,
    pub auth_type: Option<String>,
    pub access_token: Option<String>,
    pub default_branch: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateRepositoryRequest {
    pub repository_url: Option<String>,
    pub auth_type: Option<String>,
    pub access_token: Option<String>,
    pub default_branch: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_repository_request_validation() {
        let req = ConnectRepositoryRequest {
            repository_url: "https://github.com/org/repo.git".to_string(),
            auth_type: Some("pat".to_string()),
            access_token: Some("ghp_secret123".to_string()),
            default_branch: Some("main".to_string()),
        };
        assert!(req.validate().is_ok());
    }
}
