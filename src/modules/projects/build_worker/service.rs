use super::log_stream::transporters::{LokiStreamTransporter, SseStreamTransporter};
use super::pipeline::BuildPipeline;
use crate::config::AppConfig;
use crate::infrastructure::queue::events::deployments::DeploymentJobCreated;
use crate::infrastructure::queue::{MessageHandler, QueueError, QueuePublisher};
use crate::modules::projects::build_worker::OwnerType;
use crate::modules::projects::build_worker::log_stream::LogStream;
use crate::modules::projects::deployments::DeploymentStatus;
use crate::modules::projects::deployments::dto::UpdateDeploymentStatusRequest;
use crate::modules::projects::projects::entities::sea_orm_active_enums::ProjectTypes;
use crate::modules::projects::repositories::repository::ProjectRepositoriesRepository;
use crate::modules::projects::repositories::utils::ATService;
use crate::modules::projects::{
    DeploymentsService, ProjectEnvironmentVariablesService, ProjectRepositoriesService,
    ProjectsService,
};
use crate::shared::error::AppError;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub struct BuildWorkerService {
    db: Arc<DatabaseConnection>,
    config: Arc<AppConfig>,
    queue: Arc<QueuePublisher>,
}

impl BuildWorkerService {
    pub fn new(
        db: Arc<DatabaseConnection>,
        config: Arc<AppConfig>,
        queue: Arc<QueuePublisher>,
    ) -> Self {
        Self { db, config, queue }
    }

    pub async fn process_job(
        db: &DatabaseConnection,
        config: &AppConfig,
        deployment_id: Uuid,
        queue: Arc<QueuePublisher>,
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

        // todo: get repository url from the database.
        let repository =
            ProjectRepositoriesService::get_repository_by_id_internal(db, project_id).await?;

        // log stream;
        let loki_transporter = LokiStreamTransporter::new();
        let sse_transporter: SseStreamTransporter = SseStreamTransporter::new(queue);
        let log_stream =
            LogStream::new(vec![Arc::new(loki_transporter), Arc::new(sse_transporter)]);

        let mut build_pipeline = BuildPipeline::new(
            db,
            config,
            deployment_id,
            project_id,
            org_or_user_id,
            owner_type,
            pat_token.as_deref(),
            &env_vars,
            repository.repository_url,
            log_stream,
        );
        build_pipeline.execute_pipeline().await
    }
    pub async fn process_job_dummy(
        db: &DatabaseConnection,
        config: &AppConfig,
        queue: Arc<QueuePublisher>,
    ) -> Result<(), AppError> {
        tracing::info!("starting dummy build process");
        let env_map = HashMap::<String, String>::new();

        let env_vars: Vec<(&str, &str)> = env_map
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let repo_url = "https://github.com/mislam-dev/do-no-repeat-yourself.git".to_string();
        let loki_transporter = LokiStreamTransporter::new();
        let sse_transporter = SseStreamTransporter::new(queue);
        let log_stream =
            LogStream::new(vec![Arc::new(loki_transporter), Arc::new(sse_transporter)]);

        let mut build_pipeline = BuildPipeline::new(
            db,
            config,
            Uuid::from_str("f573cd3d-d784-47ef-9764-b707349aeb32").unwrap(),
            Uuid::from_str("1fc0075a-d398-44bc-8f3a-b0d062b2c545").unwrap(),
            Uuid::from_str("308274c2-29c0-4e25-8d91-73c2388a1363").unwrap(),
            OwnerType::User,
            None,
            &env_vars,
            repo_url,
            log_stream,
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

        let a = Self::process_job(
            &self.db,
            &self.config,
            job.deployment_id,
            self.queue.clone(),
        )
        .await;

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
