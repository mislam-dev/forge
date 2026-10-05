use super::super::deployments::repository::DeploymentsRepository;
use super::super::projects::repository::ProjectsRepository;
use super::dto::{BuildLogResponse, LogItem, LogSearchQuery};
use crate::infrastructure::logging::loki_client::{LokiClient, LokiFetchQuery};

use crate::shared::error::AppError;
use sea_orm::*;
use uuid::Uuid;

pub struct BuildLogsService;

impl BuildLogsService {
    fn _mock_logs_for_deployment(_deployment_id: Uuid) -> Vec<LogItem> {
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
        loki: &LokiClient,
        org_id: Option<Uuid>,
        project_id: Uuid,
        deployment_id: Uuid,
    ) -> Result<BuildLogResponse, AppError> {
        if let Some(org_id) = org_id {
            let _project = ProjectsRepository::find_by_id_with_org(db, project_id, org_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Project not found! aa".to_string()))?;
        } else {
            let _project = ProjectsRepository::find_by_id(db, project_id)
                .await?
                .ok_or_else(|| AppError::NotFound("Project not found! aa".to_string()))?;
        }

        let deployment = DeploymentsRepository::find_by_id(db, deployment_id)
            .await?
            .ok_or_else(|| AppError::NotFound("Deployment not found".to_string()))?;

        if deployment.project_id != project_id {
            return Err(AppError::NotFound(
                "Deployment not found in this project".to_string(),
            ));
        }

        let dto = LokiFetchQuery {
            deployment_id: deployment_id.to_string(),
            from: deployment.created_at.into(),
            to: deployment.updated_at.into(),
        };

        let logs = loki.find_by_deployment_id(dto).await.map_err(|e| {
            return AppError::InternalServerError(format!(
                "Failed to fetch logs from Loki: {:?}",
                e
            ));
        })?;

        println!("LokiLogs:  {:#?}", logs);

        let logs = logs
            .into_iter()
            .map(|log| LogItem {
                timestamp: log.timestamp,
                level: log.level.to_string(),
                step: log.step,
                message: log.message,
            })
            .collect();
        println!("Transformed logs:  {:#?}", logs);
        let response_data = BuildLogResponse {
            deployment_id: deployment_id.to_string(),
            logs,
        };

        Ok(response_data)
    }

    pub async fn download_logs(
        db: &DatabaseConnection,
        loki: &LokiClient,
        org_id: Option<Uuid>,
        project_id: Uuid,
        deployment_id: Uuid,
    ) -> Result<String, AppError> {
        let logs_response = Self::get_logs(db, loki, org_id, project_id, deployment_id).await?;
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
        loki: &LokiClient,
        org_id: Option<Uuid>,
        project_id: Uuid,
        deployment_id: Uuid,
        query: LogSearchQuery,
    ) -> Result<BuildLogResponse, AppError> {
        let logs_response = Self::get_logs(db, loki, org_id, project_id, deployment_id).await?;
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
