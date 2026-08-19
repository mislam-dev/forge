use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogItem {
    pub timestamp: String,
    pub level: String,
    pub step: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildLogResponse {
    pub deployment_id: String,
    pub logs: Vec<LogItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_log_response_serialization() {
        let log = LogItem {
            timestamp: "2026-08-19T10:00:00Z".to_string(),
            level: "INFO".to_string(),
            step: "clone".to_string(),
            message: "Cloning repository...".to_string(),
        };

        let res = BuildLogResponse {
            deployment_id: "dep-123".to_string(),
            logs: vec![log],
        };

        let json = serde_json::to_string(&res).unwrap();
        assert!(json.contains("Cloning repository"));
    }
}
