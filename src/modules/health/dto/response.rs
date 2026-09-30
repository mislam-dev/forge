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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LivenessProbeResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub environment: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DependencyCheck {
    pub status: String,
    pub latency_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReadinessChecks {
    pub database: DependencyCheck,
    pub job_queue: DependencyCheck,
    pub container_runtime: DependencyCheck,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReadinessProbeResponse {
    pub status: String,
    pub service: String,
    pub timestamp: String,
    pub checks: ReadinessChecks,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeepHealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub environment: String,
    pub uptime_seconds: u64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct DeepHealthQuery {
    pub timeout_ms: Option<u64>,
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

    #[test]
    fn test_liveness_probe_response_serialization() {
        let res = LivenessProbeResponse {
            status: "healthy".to_string(),
            service: "forge-platform".to_string(),
            version: "1.0.0".to_string(),
            environment: "production".to_string(),
            timestamp: "2026-08-12T21:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&res).unwrap();
        assert!(json.contains("\"status\":\"healthy\""));
        assert!(json.contains("\"service\":\"forge-platform\""));
    }

    #[test]
    fn test_readiness_probe_response_serialization() {
        let res = ReadinessProbeResponse {
            status: "ready".to_string(),
            service: "forge-platform".to_string(),
            timestamp: "2026-08-12T21:00:00Z".to_string(),
            checks: ReadinessChecks {
                database: DependencyCheck {
                    status: "healthy".to_string(),
                    latency_ms: Some(4),
                    message: None,
                },
                job_queue: DependencyCheck {
                    status: "healthy".to_string(),
                    latency_ms: Some(2),
                    message: None,
                },
                container_runtime: DependencyCheck {
                    status: "healthy".to_string(),
                    latency_ms: Some(8),
                    message: None,
                },
            },
        };

        let json = serde_json::to_string(&res).unwrap();
        assert!(json.contains("\"status\":\"ready\""));
        assert!(json.contains("\"database\":{\"status\":\"healthy\",\"latency_ms\":4}"));
        assert!(!json.contains("\"message\""));
    }

    #[test]
    fn test_deep_health_response_serialization() {
        let res = DeepHealthResponse {
            status: "healthy".to_string(),
            service: "forge-platform".to_string(),
            version: "1.0.0".to_string(),
            environment: "production".to_string(),
            uptime_seconds: 864000,
            timestamp: "2026-08-12T21:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&res).unwrap();
        assert!(json.contains("\"uptime_seconds\":864000"));
    }
}
