use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::super::projects::repository::ProjectsRepository;
use super::super::repositories::repository::ProjectRepositoriesRepository;
use super::dto::{
    DeploymentHistoryQuery, DeploymentResponse, TriggerDeploymentRequest,
    UpdateDeploymentStatusRequest,
};
use super::repository::DeploymentsRepository;
use super::status::DeploymentStatus;
use crate::config::AppConfig;
use crate::infrastructure::queue::QueuePublisher;
use crate::infrastructure::queue::events::deployments::DeploymentJobCreated;
use crate::modules::projects::ProjectRepositoriesService;
use crate::shared::error::{AppError, codes};
use crate::shared::pagination::PaginatedResponse;

pub struct DeploymentsService;

impl DeploymentsService {
    pub async fn trigger_deployment(
        db: &DatabaseConnection,
        queue: QueuePublisher,
        org_id: Option<Uuid>,
        project_id: Uuid,
        triggered_by: Uuid,
        req: TriggerDeploymentRequest,
    ) -> Result<DeploymentResponse, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(codes::deployments::DEPLOY_002, "Project not found")
            })?;

        let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id)
            .await?
            .ok_or_else(|| {
                AppError::bad_request(
                    codes::projects::REPO_001,
                    "No repository connected to this project",
                )
            })?;

        if (DeploymentsRepository::find_running_by_project_id(db, project_id).await?).is_some() {
            return Err(AppError::conflict(
                codes::deployments::DEPLOY_003,
                "A deployment is currently in progress for this project",
            ));
        }

        let branch = req
            .branch
            .or(repo.default_branch)
            .unwrap_or_else(|| "main".to_string());
        let commit_hash = req.commit_hash.unwrap_or_else(|| "HEAD".to_string());

        let deployment = DeploymentsRepository::create_deployment(
            db,
            project_id,
            triggered_by,
            branch,
            commit_hash,
            DeploymentStatus::Queued,
        )
        .await?;

        let deployment_clone = deployment.clone();

        let job_event = DeploymentJobCreated {
            deployment_id: deployment.id,
            project_id: deployment.project_id,
            repository_url: repo.repository_url,
            commit_hash: deployment.commit_hash,
            branch: deployment.branch.clone(),
            triggered_by: deployment.triggered_by,
        };

        if let Err(err) = queue.publish(&job_event).await {
            tracing::error!(
                error = %err,
                deployment_id = %deployment.id,
                "Failed to publish deployment job to queue"
            );

            let _ = DeploymentsRepository::update_status(
                db,
                deployment.id,
                DeploymentStatus::Failed,
                None,
                None,
                Some(err.to_string()),
            )
            .await;
        }

        Ok(DeploymentResponse::from_model(deployment_clone))
    }

    pub async fn list_deployments(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        query: DeploymentHistoryQuery,
    ) -> Result<PaginatedResponse<DeploymentResponse>, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(codes::deployments::DEPLOY_002, "Project not found")
            })?;

        let paginated = DeploymentsRepository::find_by_project_id(db, project_id, query).await?;
        let responses = paginated
            .data
            .into_iter()
            .map(DeploymentResponse::from_model)
            .collect();

        Ok(PaginatedResponse::new(
            responses,
            paginated.page,
            paginated.per_page,
            paginated.total,
        ))
    }

    pub async fn get_deployment(
        db: &DatabaseConnection,
        org_id: Option<Uuid>,
        project_id: Uuid,
        deployment_id: Uuid,
    ) -> Result<DeploymentResponse, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(codes::deployments::DEPLOY_002, "Project not found")
            })?;

        let deployment = DeploymentsRepository::find_by_id(db, deployment_id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(codes::deployments::DEPLOY_001, "Deployment not found")
            })?;

        if deployment.project_id != project_id {
            return Err(AppError::not_found(
                codes::deployments::DEPLOY_001,
                "Deployment not found in this project",
            ));
        }

        Ok(DeploymentResponse::from_model(deployment))
    }
    pub async fn get_deployment_by_id_internal(
        db: &DatabaseConnection,
        deployment_id: Uuid,
    ) -> Result<DeploymentResponse, AppError> {
        let deployment = DeploymentsRepository::find_by_id(db, deployment_id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(codes::deployments::DEPLOY_001, "Deployment not found")
            })?;

        Ok(DeploymentResponse::from_model(deployment))
    }

    pub async fn redeploy(
        db: &DatabaseConnection,
        queue: QueuePublisher,
        org_id: Option<Uuid>,
        project_id: Uuid,
        triggered_by: Uuid,
        deployment_id: Uuid,
    ) -> Result<DeploymentResponse, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(codes::deployments::DEPLOY_002, "Project not found")
            })?;

        let target = DeploymentsRepository::find_by_id(db, deployment_id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(codes::deployments::DEPLOY_001, "Deployment not found")
            })?;

        if target.project_id != project_id {
            return Err(AppError::not_found(
                codes::deployments::DEPLOY_001,
                "Deployment not found in this project",
            ));
        }

        if (DeploymentsRepository::find_running_by_project_id(db, project_id).await?).is_some() {
            return Err(AppError::conflict(
                codes::deployments::DEPLOY_003,
                "A deployment is currently in progress for this project",
            ));
        }

        let deployment = DeploymentsRepository::create_deployment(
            db,
            project_id,
            triggered_by,
            target.branch,
            target.commit_hash,
            DeploymentStatus::Queued,
        )
        .await?;
        let deployment_clone = deployment.clone();
        // todo: trigger an event send this event to rabbitmq with status redeploy
        let repo = ProjectRepositoriesService::get_repository(db, org_id, project_id).await?;

        let job_event = DeploymentJobCreated {
            deployment_id: deployment.id,
            project_id: deployment.project_id,
            repository_url: repo.repository_url,
            commit_hash: deployment.commit_hash,
            branch: deployment.branch.clone(),
            triggered_by: deployment.triggered_by,
        };
        if let Err(err) = queue.publish(&job_event).await {
            tracing::error!(
                error = %err,
                deployment_id = %deployment.id,
                "Failed to publish deployment job to queue"
            );

            let _ = DeploymentsRepository::update_status(
                db,
                deployment.id,
                DeploymentStatus::Failed,
                None,
                None,
                Some(err.to_string()),
            )
            .await;
        }

        Ok(DeploymentResponse::from_model(deployment_clone))
    }

    pub async fn rollback(
        db: &DatabaseConnection,
        queue: QueuePublisher,
        org_id: Option<Uuid>,
        project_id: Uuid,
        triggered_by: Uuid,
    ) -> Result<DeploymentResponse, AppError> {
        let _project = ProjectsRepository::find_by_id_and_optional_org(db, project_id, org_id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(codes::deployments::DEPLOY_002, "Project not found")
            })?;

        if (DeploymentsRepository::find_running_by_project_id(db, project_id).await?).is_some() {
            return Err(AppError::conflict(
                codes::deployments::DEPLOY_003,
                "A deployment is currently in progress for this project",
            ));
        }

        let last_success = DeploymentsRepository::find_last_success_by_project_id(db, project_id)
            .await?
            .ok_or_else(|| {
                AppError::bad_request(
                    codes::deployments::HIST_004,
                    "No previous successful deployment found to rollback to",
                )
            })?;

        let deployment = DeploymentsRepository::create_deployment(
            db,
            project_id,
            triggered_by,
            last_success.branch,
            last_success.commit_hash,
            DeploymentStatus::Queued,
        )
        .await?;

        let deployment_clone = deployment.clone();

        let repo = ProjectRepositoriesService::get_repository(db, org_id, project_id).await?;

        let job_event = DeploymentJobCreated {
            deployment_id: deployment.id,
            project_id: deployment.project_id,
            repository_url: repo.repository_url,
            commit_hash: deployment.commit_hash,
            branch: deployment.branch.clone(),
            triggered_by: deployment.triggered_by,
        };
        if let Err(err) = queue.publish(&job_event).await {
            tracing::error!(
                error = %err,
                deployment_id = %deployment.id,
                "Failed to publish deployment job to queue"
            );

            let _ = DeploymentsRepository::update_status(
                db,
                deployment.id,
                DeploymentStatus::Failed,
                None,
                None,
                Some(err.to_string()),
            )
            .await;
        }
        Ok(DeploymentResponse::from_model(deployment_clone))
    }

    pub async fn update_status_internal(
        db: &DatabaseConnection,
        config: &AppConfig,
        service_token: &str,
        deployment_id: Uuid,
        req: UpdateDeploymentStatusRequest,
    ) -> Result<DeploymentResponse, AppError> {
        if service_token != config.secrets.worker_service_token {
            return Err(AppError::unauthorized(
                codes::auth::AUTH_001,
                "Invalid service authentication token",
            ));
        }

        let target_status = req
            .status
            .parse::<DeploymentStatus>()
            .map_err(|e| AppError::bad_request(codes::deployments::DEPLOY_003, e))?;

        let deployment = DeploymentsRepository::find_by_id(db, deployment_id)
            .await?
            .ok_or_else(|| {
                AppError::not_found(codes::deployments::DEPLOY_001, "Deployment not found")
            })?;

        let current_status = &deployment.status;

        if !current_status.can_transition_to(&target_status) {
            return Err(AppError::bad_request(
                codes::deployments::DEPLOY_003,
                format!(
                    "Invalid deployment status transition from {} to {}",
                    current_status, target_status
                ),
            ));
        }

        let updated = DeploymentsRepository::update_deployment(
            db,
            deployment,
            target_status,
            req.build_duration,
            req.deploy_duration,
            req.error_message,
        )
        .await?;
        Ok(DeploymentResponse::from_model(updated))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::queue::MockMessagePublisher;
    use sea_orm::{DatabaseBackend, MockDatabase};

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[tokio::test]
    async fn test_trigger_deployment_project_not_found() {
        let db = setup_mock_db();
        let queue = QueuePublisher::Mock(MockMessagePublisher::new());
        let result = DeploymentsService::trigger_deployment(
            &db,
            queue,
            None,
            Uuid::new_v4(),
            Uuid::new_v4(),
            TriggerDeploymentRequest {
                branch: Some("main".to_string()),
                commit_hash: None,
            },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_deployments_project_not_found() {
        let db = setup_mock_db();
        let result = DeploymentsService::list_deployments(
            &db,
            None,
            Uuid::new_v4(),
            DeploymentHistoryQuery {
                page: None,
                per_page: None,
                status: None,
                branch: None,
            },
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_deployment_project_not_found() {
        let db = setup_mock_db();
        let result =
            DeploymentsService::get_deployment(&db, None, Uuid::new_v4(), Uuid::new_v4()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_deployment_by_id_internal_not_found() {
        let db = setup_mock_db();
        let result = DeploymentsService::get_deployment_by_id_internal(&db, Uuid::new_v4()).await;
        assert!(result.is_err());
    }
}
