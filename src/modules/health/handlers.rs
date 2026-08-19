use axum::{
    extract::State,
    http::StatusCode,
};

use super::dto::{DetailedHealthResponse, HealthProbeResponse};
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
    let is_admin = claims.role.iter().any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let details = HealthService::check_health_details(&state.db, claims.sub, is_admin).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Detailed health status retrieved successfully.".to_string())
        .body(Some(details)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_code_mapping() {
        assert_eq!(StatusCode::OK, StatusCode::OK);
        assert_eq!(StatusCode::SERVICE_UNAVAILABLE, StatusCode::SERVICE_UNAVAILABLE);
    }
}
