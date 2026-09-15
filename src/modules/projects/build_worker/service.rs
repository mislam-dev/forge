use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use super::pipeline::BuildPipeline;
use crate::config::AppConfig;
use crate::infrastructure::queue::events::deployments::DeploymentJobCreated;
use crate::infrastructure::queue::{MessageHandler, QueueError};
use crate::modules::projects::build_worker::OwnerType;
use crate::modules::projects::deployments::DeploymentStatus;
use crate::modules::projects::deployments::dto::UpdateDeploymentStatusRequest;
use crate::modules::projects::projects::entities::sea_orm_active_enums::ProjectTypes;
use crate::modules::projects::repositories::repository::ProjectRepositoriesRepository;
use crate::modules::projects::repositories::utils::ATService;
use crate::modules::projects::{
    DeploymentsService, ProjectEnvironmentVariablesService, ProjectsService,
};
use crate::shared::error::AppError;

pub struct BuildWorkerService {
    db: Arc<DatabaseConnection>,
    config: Arc<AppConfig>,
}

impl BuildWorkerService {
    pub fn new(db: Arc<DatabaseConnection>, config: Arc<AppConfig>) -> Self {
        Self { db, config }
    }

    pub async fn process_job(
        db: &DatabaseConnection,
        config: &AppConfig,
        deployment_id: Uuid,
    ) -> Result<(), AppError> {
        let deployment =
            DeploymentsService::get_deployment_by_id_internal(db, deployment_id).await?;

        let project_id = deployment.project_id;

        let project = ProjectsService::get_project_by_internal(db, project_id).await?;
        let owner_type: OwnerType;
        let org_or_user_id = if let Some(org_id) = project.organization_id {
            owner_type = OwnerType::Org;
            org_id
        } else {
            owner_type = OwnerType::User;
            project.owner_id
        };

        let pat_token = if project.project_type == ProjectTypes::Repo {
            let repo = ProjectRepositoriesRepository::find_by_project_id(db, project_id).await?;
            match repo {
                Some(repo) => Some(ATService::decrypt(&repo.access_token_encrypted)),
                None => None,
            }
        } else {
            None
        };

        let env_map = ProjectEnvironmentVariablesService::get_decrypted_env_vars(
            db,
            None,
            project_id,
            "production",
        )
        .await
        .unwrap_or_default();

        let env_vars: Vec<(&str, &str)> = env_map
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let mut build_pipeline = BuildPipeline::new(
            db,
            config,
            deployment_id,
            project_id,
            org_or_user_id,
            owner_type,
            pat_token.as_deref(),
            &env_vars,
            "".to_string(),
        );
        build_pipeline.execute_pipeline().await
    }
    pub async fn process_job_dummy(
        db: &DatabaseConnection,
        config: &AppConfig,
    ) -> Result<(), AppError> {
        tracing::info!("starting dummy build process");
        let env_map = HashMap::<String, String>::new();

        let env_vars: Vec<(&str, &str)> = env_map
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let repo_url = "https://github.com/mislam-dev/do-no-repeat-yourself.git".to_string();
        let mut build_pipeline = BuildPipeline::new(
            db,
            config,
            Uuid::from_str("f573cd3d-d784-47ef-9764-b707349aeb32").unwrap(),
            Uuid::from_str("1fc0075a-d398-44bc-8f3a-b0d062b2c545").unwrap(),
            Uuid::from_str("308274c2-29c0-4e25-8d91-73c2388a1366").unwrap(),
            OwnerType::User,
            None,
            &env_vars,
            repo_url,
        );
        build_pipeline.execute_pipeline().await
    }
}

#[async_trait]
impl MessageHandler<DeploymentJobCreated> for BuildWorkerService {
    async fn handle(&self, job: DeploymentJobCreated) -> Result<(), QueueError> {
        tracing::info!(
            deployment_id = %job.deployment_id,
            project_id = %job.project_id,
            "Received deployment job from queue"
        );

        let a = Self::process_job(&self.db, &self.config, job.deployment_id).await;

        match a {
            Ok(_) => {
                tracing::info!(
                    deployment_id = %job.deployment_id,
                    "Build pipeline completed successfully"
                );
                Ok(())
            }
            Err(e) => {
                tracing::error!(
                    error = %e,
                    deployment_id = %job.deployment_id,
                    "Build pipeline failed. Marking deployment as Failed."
                );

                let service_token = self.config.secrets.master_encryption_key.clone();

                let _ = DeploymentsService::update_status_internal(
                    &self.db,
                    &self.config,
                    &service_token,
                    job.deployment_id,
                    UpdateDeploymentStatusRequest {
                        status: DeploymentStatus::Failed.as_str().to_string(),
                        build_duration: None,
                        deploy_duration: None,
                        error_message: Some(e.to_string()),
                    },
                )
                .await;

                Err(QueueError::PublishedNackedError)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    fn setup_mock_config() -> Arc<AppConfig> {
        Arc::new(AppConfig {
            infra: crate::config::InfraConnectionUrls {
                db: crate::database::DbConfig {
                    max_connections: 5,
                    min_connections: 1,
                    connect_timeout_secs: 10,
                    idle_timeout_secs: 60,
                    db_url: "postgres://localhost/test".to_string(),
                },
                redis_url: "redis://localhost:6379".to_string(),
                rabbitmq_url: "amqp://localhost:5672".to_string(),
                loki_url: "http://localhost:3100".to_string(),
            },
            secrets: crate::config::Secrets {
                jwt_secret: "test_secret".to_string(),
                master_encryption_key: "0123456789abcdef0123456789abcdef".to_string(),
                jwt_expiry_seconds: 3600,
                refresh_token_expiry_days: 7,
            },
            server_config: crate::config::ServerConfig {
                rust_log: false,
                server_port: 3000,
                server_host: std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
            },
        })
    }

    #[tokio::test]
    async fn test_build_worker_service_new() {
        let db = Arc::new(MockDatabase::new(DatabaseBackend::Postgres).into_connection());
        let config = setup_mock_config();
        let service = BuildWorkerService::new(db, config);
        assert_eq!(
            service.config.secrets.master_encryption_key,
            "0123456789abcdef0123456789abcdef"
        );
    }

    #[tokio::test]
    async fn test_build_worker_process_job_deployment_not_found() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([Vec::<
                crate::modules::projects::deployments::entities::deployments::Model,
            >::new()])
            .into_connection();
        let config = setup_mock_config();
        let deployment_id = Uuid::new_v4();

        let result = BuildWorkerService::process_job(&db, &config, deployment_id).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(AppError::NotFound(_))));
    }

    #[tokio::test]
    async fn test_build_worker_handle_job_failure_triggers_nack() {
        let db = Arc::new(MockDatabase::new(DatabaseBackend::Postgres).into_connection());
        let config = setup_mock_config();
        let service = BuildWorkerService::new(db, config);

        let job = DeploymentJobCreated {
            deployment_id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            repository_url: "https://github.com/org/repo".to_string(),
            commit_hash: "abc1234".to_string(),
            branch: "main".to_string(),
            triggered_by: Uuid::new_v4(),
        };

        let result = service.handle(job).await;
        assert!(result.is_err());
        assert!(matches!(result, Err(QueueError::PublishedNackedError)));
    }
}
