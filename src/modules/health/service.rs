use chrono::Utc;
use sea_orm::*;
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

use super::dto::{DetailedHealthResponse, HealthProbeResponse, ServiceHealthItem};
use crate::shared::error::AppError;

pub struct HealthService;

impl HealthService {
    pub async fn check_health(db: &DatabaseConnection) -> HealthProbeResponse {
        let mut services = HashMap::new();
        let start = Instant::now();

        let db_result = db.execute_unprepared("SELECT 1").await;
        let elapsed = start.elapsed().as_millis();

        let (status, db_item) = match db_result {
            Ok(_) => (
                "ok",
                ServiceHealthItem {
                    status: "ok".to_string(),
                    latency_ms: elapsed,
                    error: None,
                },
            ),
            Err(e) => (
                "critical",
                ServiceHealthItem {
                    status: "critical".to_string(),
                    latency_ms: elapsed,
                    error: Some(e.to_string()),
                },
            ),
        };

        services.insert("database".to_string(), db_item);

        HealthProbeResponse {
            status: status.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            services,
        }
    }

    pub async fn check_health_details(
        db: &DatabaseConnection,
        _requester_id: Uuid,
        is_system_admin: bool,
    ) -> Result<DetailedHealthResponse, AppError> {
        if !is_system_admin {
            return Err(AppError::Forbidden(
                "Only System Admins can access detailed health probes".to_string(),
            ));
        }

        let basic = Self::check_health(db).await;

        Ok(DetailedHealthResponse {
            status: basic.status,
            timestamp: basic.timestamp,
            version: env!("CARGO_PKG_VERSION").to_string(),
            services: basic.services,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_check_health_details_forbidden_for_non_admin() {
        let db = setup_mock_db();
        let result = HealthService::check_health_details(&db, Uuid::new_v4(), false).await;
        assert!(result.is_err());
    }
}
