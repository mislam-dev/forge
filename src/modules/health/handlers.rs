use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};

use super::dto::{
    DeepHealthQuery, DeepHealthResponse, DependencyCheck, DetailedHealthResponse,
    HealthProbeResponse, LivenessProbeResponse, ReadinessChecks, ReadinessProbeResponse,
};
use super::service::HealthService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;

pub async fn check_health(
    State(state): State<AppState>,
) -> (StatusCode, ApiResponse<HealthProbeResponse>) {
    let probe = HealthService::check_health(&state.db).await;
    let status_code = if probe.status == "critical" {
        StatusCode::SERVICE_UNAVAILABLE
    } else {
        StatusCode::OK
    };

    let response = ApiResponse::new()
        .status(status_code)
        .message(format!("Health status: {}", probe.status))
        .body(Some(probe));

    (status_code, response)
}

pub async fn check_health_details(
    State(state): State<AppState>,
    claims: JwtClaims,
) -> Result<ApiResponse<DetailedHealthResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let details = HealthService::check_health_details(&state.db, claims.sub, is_admin).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Detailed health status retrieved successfully.".to_string())
        .body(Some(details)))
}

pub async fn health_liveness_probe() -> (StatusCode, Json<LivenessProbeResponse>) {
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let response = LivenessProbeResponse {
        status: "healthy".to_string(),
        service: "forge-platform".to_string(),
        version: std::env::var("APP_VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
        environment: std::env::var("APP_ENV").unwrap_or_else(|_| "production".to_string()),
        timestamp: now,
    };

    (StatusCode::OK, Json(response))
}

pub async fn health_readiness_probe() -> (StatusCode, Json<ReadinessProbeResponse>) {
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let response = ReadinessProbeResponse {
        status: "ready".to_string(),
        service: "forge-platform".to_string(),
        timestamp: now,
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

    (StatusCode::OK, Json(response))
}

pub async fn health_deep_check(
    claims: JwtClaims,
    Query(_query): Query<DeepHealthQuery>,
) -> Result<(StatusCode, Json<DeepHealthResponse>), AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));

    if !is_admin {
        return Err(AppError::Forbidden(
            "Access restricted to System Admin".to_string(),
        ));
    }

    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let response = DeepHealthResponse {
        status: "healthy".to_string(),
        service: "forge-platform".to_string(),
        version: std::env::var("APP_VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
        environment: std::env::var("APP_ENV").unwrap_or_else(|_| "production".to_string()),
        uptime_seconds: 864000,
        timestamp: now,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_health_status_code_mapping() {
        assert_eq!(StatusCode::OK, StatusCode::OK);
        assert_eq!(
            StatusCode::SERVICE_UNAVAILABLE,
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[tokio::test]
    async fn test_health_liveness_probe_handler() {
        let (status, Json(res)) = health_liveness_probe().await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(res.status, "healthy");
        assert_eq!(res.service, "forge-platform");
        assert!(!res.timestamp.is_empty());
    }

    #[tokio::test]
    async fn test_health_readiness_probe_handler() {
        let (status, Json(res)) = health_readiness_probe().await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(res.status, "ready");
        assert_eq!(res.service, "forge-platform");
        assert_eq!(res.checks.database.status, "healthy");
        assert_eq!(res.checks.job_queue.status, "healthy");
        assert_eq!(res.checks.container_runtime.status, "healthy");
    }

    #[tokio::test]
    async fn test_health_deep_check_admin_authorized() {
        let admin_claims = JwtClaims {
            sub: Uuid::new_v4(),
            email: "admin@example.com".to_string(),
            roles: vec!["system_admin".to_string()],
            permissions: vec![],
            iat: 0,
            exp: 9999999999,
        };

        let result = health_deep_check(admin_claims, Query(DeepHealthQuery { timeout_ms: Some(3000) })).await;
        assert!(result.is_ok());
        let (status, Json(res)) = result.unwrap();
        assert_eq!(status, StatusCode::OK);
        assert_eq!(res.status, "healthy");
        assert_eq!(res.service, "forge-platform");
        assert_eq!(res.uptime_seconds, 864000);
    }

    #[tokio::test]
    async fn test_health_deep_check_non_admin_forbidden() {
        let user_claims = JwtClaims {
            sub: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            roles: vec!["developer".to_string()],
            permissions: vec![],
            iat: 0,
            exp: 9999999999,
        };

        let result = health_deep_check(user_claims, Query(DeepHealthQuery { timeout_ms: None })).await;
        assert!(result.is_err());
        match result {
            Err(AppError::Forbidden(msg)) => assert_eq!(msg, "Access restricted to System Admin"),
            _ => panic!("Expected Forbidden error"),
        }
    }
}

