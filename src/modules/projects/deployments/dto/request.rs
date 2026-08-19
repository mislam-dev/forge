use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct TriggerDeploymentRequest {
    pub branch: Option<String>,
    pub commit_hash: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateDeploymentStatusRequest {
    #[validate(length(min = 1, message = "Status is required"))]
    pub status: String,
    pub build_duration: Option<i32>,
    pub deploy_duration: Option<i32>,
    pub error_message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct DeploymentHistoryQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub status: Option<String>,
    pub branch: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trigger_deployment_request_serialization() {
        let req = TriggerDeploymentRequest {
            branch: Some("main".to_string()),
            commit_hash: Some("a1b2c3d".to_string()),
        };
        assert!(req.validate().is_ok());
    }
}
