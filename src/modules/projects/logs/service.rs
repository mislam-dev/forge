use sea_orm::*;
use uuid::Uuid;

use super::super::deployments::repository::DeploymentsRepository;
use super::super::permissions::role::ProjectRole;
use super::super::permissions::service::ProjectPermissionsService;
use super::super::projects::repository::ProjectsRepository;
use super::dto::{BuildLogResponse, LogItem, LogSearchQuery};
use crate::shared::error::AppError;

pub struct BuildLogsService;

impl BuildLogsService {
    fn mock_logs_for_deployment(_deployment_id: Uuid) -> Vec<LogItem> {
        vec![
            LogItem {
                timestamp: "2026-08-19T10:00:00Z".to_string(),
                level: "INFO".to_string(),
                step: "clone".to_string(),
                message: "Cloning repository...".to_string(),
            },
            LogItem {
                timestamp: "2026-08-19T10:00:02Z".to_string(),
                level: "INFO".to_string(),
                step: "validate".to_string(),
                message: "Dockerfile validated successfully.".to_string(),
            },
            LogItem {
                timestamp: "2026-08-19T10:00:05Z".to_string(),
                level: "INFO".to_string(),
                step: "build".to_string(),
                message: "Docker build completed successfully.".to_string(),
            },
            LogItem {
                timestamp: "2026-08-19T10:00:10Z".to_string(),
                level: "INFO".to_string(),
                step: "deploy".to_string(),
                message: "Container started and listening on configured port.".to_string(),
            },
            LogItem {
                timestamp: "2026-08-19T10:00:12Z".to_string(),
                level: "INFO".to_string(),
                step: "health_check".to_string(),
                message: "Health check probe returned HTTP 200 OK.".to_string(),
            },
        ]
    }

    pub async fn get_logs(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        deployment_id: Uuid,
    ) -> Result<BuildLogResponse, AppError> {
        let project = ProjectsRepository::find_by_id(db, project_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Project not found".to_string()))?;

        if !is_system_admin {
            ProjectPermissionsService::verify_project_role(
                db,
                project_id,
                requester_id,
                project.organization_id,
                is_system_admin,
                ProjectRole::Viewer,
            )
            .await?;
        }

        let deployment = DeploymentsRepository::find_by_id(db, deployment_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Deployment not found".to_string()))?;

        if deployment.project_id != project_id {
            return Err(AppError::NotFound(
                "Deployment not found in this project".to_string(),
            ));
        }

        Ok(BuildLogResponse {
            deployment_id: deployment_id.to_string(),
            logs: Self::mock_logs_for_deployment(deployment_id),
        })
    }

    pub async fn download_logs(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        deployment_id: Uuid,
    ) -> Result<String, AppError> {
        let logs_response =
            Self::get_logs(db, requester_id, is_system_admin, project_id, deployment_id).await?;
        let mut buffer = String::new();
        for item in logs_response.logs {
            buffer.push_str(&format!(
                "{} [{}] [{}] {}\n",
                item.timestamp, item.level, item.step, item.message
            ));
        }
        Ok(buffer)
    }

    pub async fn search_logs(
        db: &DatabaseConnection,
        requester_id: Uuid,
        is_system_admin: bool,
        project_id: Uuid,
        deployment_id: Uuid,
        query: LogSearchQuery,
    ) -> Result<BuildLogResponse, AppError> {
        let logs_response =
            Self::get_logs(db, requester_id, is_system_admin, project_id, deployment_id).await?;
        let pattern = query.q.to_lowercase();

        let filtered = logs_response
            .logs
            .into_iter()
            .filter(|item| {
                item.message.to_lowercase().contains(&pattern)
                    || item.step.to_lowercase().contains(&pattern)
            })
            .collect();

        Ok(BuildLogResponse {
            deployment_id: deployment_id.to_string(),
            logs: filtered,
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
    async fn test_get_logs_deployment_not_found() {
        let db = setup_mock_db();
        let result =
            BuildLogsService::get_logs(&db, Uuid::new_v4(), false, Uuid::new_v4(), Uuid::new_v4())
                .await;
        assert!(result.is_err());
    }
}
