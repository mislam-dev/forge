use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthItem {
    pub status: String,
    pub latency_ms: u128,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthProbeResponse {
    pub status: String,
    pub timestamp: String,
    pub services: HashMap<String, ServiceHealthItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetailedHealthResponse {
    pub status: String,
    pub timestamp: String,
    pub version: String,
    pub services: HashMap<String, ServiceHealthItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_probe_response_serialization() {
        let mut services = HashMap::new();
        services.insert(
            "database".to_string(),
            ServiceHealthItem {
                status: "ok".to_string(),
                latency_ms: 2,
                error: None,
            },
        );

        let res = HealthProbeResponse {
            status: "ok".to_string(),
            timestamp: "2026-08-19T10:00:00Z".to_string(),
            services,
        };

        let json = serde_json::to_string(&res).unwrap();
        assert!(json.contains("database"));
        assert!(json.contains("ok"));
    }
}
