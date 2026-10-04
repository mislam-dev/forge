use super::super::deployments::dto::UpdateDeploymentStatusRequest;
use super::super::deployments::status::DeploymentStatus;
use super::builders::{
    DockerBuilder, GoBuilder, NodeJsBuilder, PythonBuilder, RustBuilder, StaticFilesBuilder,
};
use super::cloning::RepoCloning;
use super::deployment_durations::DeploymentDurations;
use super::deployment_path::{DeploymentPath, DeploymentPathDTO, Owner, OwnerType};
use super::log_stream::LogStream;
use super::log_stream::{LogItem, LogLevel};
use crate::config::AppConfig;
use crate::modules::projects::DeploymentsService;
use crate::modules::projects::build_worker::traits::{BuilderConfig, ProjectBuilder};
use crate::shared::error::AppError;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

#[derive(Debug, Clone)]
enum ProjectType {
    StaticFiles,
    NodeJs,
    Rust,
    Go,
    Python,
    DockerContainer,
}

// #[derive(Clone, Debug)]
pub struct BuildPipeline {
    db: DatabaseConnection,
    config: AppConfig,
    deployment_id: Uuid,
    project_id: Uuid,
    owner_type: OwnerType,
    org_or_user_id: Uuid,
    pat_token: Option<String>,
    env_vars: Vec<(String, String)>,
    repo_url: String,
    branch: String,
    durations: DeploymentDurations,
    log_stream: LogStream,
}

impl BuildPipeline {
    pub fn new(
        db: &DatabaseConnection,
        config: &AppConfig,
        deployment_id: Uuid,
        project_id: Uuid,
        org_or_user_id: Uuid,
        owner_type: OwnerType,
        pat_token: Option<&str>,
        env_vars: &[(&str, &str)], // todo: update this later
        repo_url: String,
        log_stream: LogStream,
    ) -> Self {
        let transformed_envs: Vec<(String, String)> = env_vars
            .iter()
            .map(|(key, value)| return (key.to_string(), value.to_string()))
            .collect();
        Self {
            db: db.clone(),
            config: config.clone(),
            deployment_id,
            project_id,
            org_or_user_id,
            owner_type,
            pat_token: pat_token.map(|s| s.to_string()),
            env_vars: transformed_envs,
            repo_url,                   // todo: must be dynamic
            branch: "main".to_string(), // todo: must be dynamic
            durations: DeploymentDurations::new(),
            log_stream,
        }
    }

    fn construct_path(&self) -> DeploymentPath {
        let owner = match self.owner_type {
            OwnerType::User => Owner::user(self.org_or_user_id),
            OwnerType::Org => Owner::org(self.org_or_user_id),
        };
        let deployment_path = DeploymentPath::new(DeploymentPathDTO {
            project_id: self.project_id.to_string(),
            deployment_id: self.deployment_id.to_string(),
            owner,
            base: None,
        });
        deployment_path
    }

    fn infer_project_type(&mut self) -> Result<ProjectType, AppError> {
        let path = self.construct_path();
        if !path.source.exists() {
            return Err(AppError::InternalServerError(
                "Project directory does not exist.".to_string(),
            ));
        }
        let mut project_type = ProjectType::StaticFiles;

        if path.source.join("Dockerfile").exists() {
            project_type = ProjectType::DockerContainer;
        } else if path.source.join("package.json").exists() {
            project_type = ProjectType::NodeJs;
        } else if path.source.join("Cargo.toml").exists() {
            project_type = ProjectType::Rust;
        } else if path.source.join("requirements.txt").exists() {
            project_type = ProjectType::Python;
        } else if path.source.join("go.mod").exists() {
            project_type = ProjectType::Go;
        }

        Ok(project_type)
    }

    fn get_builder(&mut self) -> Result<Option<Box<dyn ProjectBuilder>>, AppError> {
        let config = BuilderConfig {
            project_path: self.construct_path(),
            project_id: self.project_id.to_string(),
            env_vars: self.env_vars.clone(),
            org_or_user_id: self.org_or_user_id.to_string(),
            deployment_id: self.deployment_id.to_string(),
            app_name: "app".to_string(),
            log_stream: self.log_stream.clone(),
        };
        let builder: Box<dyn ProjectBuilder> = match self.infer_project_type()? {
            ProjectType::DockerContainer => Box::new(DockerBuilder::new(config)?),
            ProjectType::NodeJs => Box::new(NodeJsBuilder::new(config)?),
            ProjectType::Go => Box::new(GoBuilder::new(config)?),
            ProjectType::Rust => Box::new(RustBuilder::new(config)?),
            ProjectType::Python => Box::new(PythonBuilder::new(config)?),
            ProjectType::StaticFiles => Box::new(StaticFilesBuilder::new(config)?),
        };
        return Ok(Some(builder));
    }

    pub async fn execute_pipeline(&mut self) -> Result<(), AppError> {
        match self.execute_pipeline_internal().await {
            Ok(()) => Ok(()),
            Err(err) => {
                let err_msg = err.to_string();
                self.log_stream
                    .log_error(
                        &self.deployment_id.to_string(),
                        "Failure",
                        &format!("Pipeline execution failed: {}", err_msg),
                        true,
                    )
                    .await;
                let _ = self.update_status_failed(&err_msg).await;
                Err(err)
            }
        }
    }

    async fn execute_pipeline_internal(&mut self) -> Result<(), AppError> {
        self.log_info("Start", "Executing pipelines...").await;
        let _ = &self.durations.set_start_time();
        // Step 1: Clone Repository (Queued -> Building)
        self.log_info("Download", "Cloning repo data...").await;
        RepoCloning::new(
            self.pat_token.clone(),
            self.repo_url.clone(),
            self.branch.clone(),
            self.construct_path().source.clone(),
        )
        .clone()
        .await?;
        self.durations.set_clone_duration();

        self.log_info("Build", "0/3 | Starting Build").await;

        self.update_status_internal(DeploymentStatus::Building)
            .await?;

        let builder = self.get_builder()?;

        if builder.is_none() {
            return Err(AppError::BadRequest(format!("No builder found!")));
        }
        let builder = builder.unwrap();

        // Step 2: Validation
        self.log_info("Build", "1/7 | Validating...").await;
        builder.validate().await?;
        self.durations.set_validate_duration();

        // Step 3: generate necessary files
        builder.create_files().await?;
        self.log_info("Build", "2/7 | Creating necessary files...")
            .await;

        let version_tag = "1.0.0".to_string();
        // Step 4: Build Docker Image
        self.log_info("Build", "3/7 | Building...").await;
        let image_id = builder.build(version_tag).await?;
        let _ = &self.durations.set_build_duration();

        // Step 5: Deploy Container (Building -> Deploying)
        self.update_status_internal(DeploymentStatus::Deploying)
            .await?;
        self.log_info("Build", "4/7 | Deploying...").await;
        let container_id = builder
            .deploy(image_id, "production".to_string(), 1)
            .await?;
        self.durations.set_deploy_duration();

        self.log_info("Build", "5/7 | Starting...").await;
        // Step 6: Run container Deploying -> Running)
        builder.run(container_id).await?;
        self.update_status_internal(DeploymentStatus::Running)
            .await?;
        self.log_info("Build", "6/7 | Checking...").await;
        // Step 7: Health Check Probe (Running -> Success)
        builder.health_check().await?;
        self.durations.set_health_check_duration();

        self.log_info("Build", "7/7 | Finishing...").await;
        // Step 8: Cleanup
        builder.cleanup().await?;

        self.update_status_internal(DeploymentStatus::Success)
            .await?;

        self.log_stream
            .log_end(
                &self.deployment_id.to_string(),
                "Success",
                "Build completed successfully",
            )
            .await;
        Ok(())
    }

    async fn log_info(&self, step: &str, log_line: &str) {
        let log_v = format!("[{}]: {}", step, log_line);
        let _a = self
            .log_stream
            .stream(LogItem {
                level: LogLevel::Info,
                step: step.to_string(),
                message: log_v,
                deployment_id: self.deployment_id.to_string(),
                timestamp: chrono::Utc::now().to_string(),
                is_end: false,
            })
            .await;
        // todo: stream to log info to fe with SSE.
        tracing::info!("[{}]: {}", step, log_line);
    }

    async fn update_status_internal(&self, status: DeploymentStatus) -> Result<(), AppError> {
        tracing::info!("[deployment_status]: {}", status.as_str());

        let dto: UpdateDeploymentStatusRequest = match status {
            DeploymentStatus::Deploying => UpdateDeploymentStatusRequest {
                status: status.as_str().to_string(),
                build_duration: Some(self.durations.build),
                deploy_duration: None,
                error_message: None,
            },
            DeploymentStatus::Success => UpdateDeploymentStatusRequest {
                status: status.as_str().to_string(),
                build_duration: Some(self.durations.build),
                deploy_duration: Some(self.durations.deploy_duration()),
                error_message: None,
            },
            _ => UpdateDeploymentStatusRequest {
                status: status.as_str().to_string(),
                build_duration: None,
                deploy_duration: None,
                error_message: None,
            },
        };

        let _c = DeploymentsService::update_status_internal(
            &self.db,
            &self.config,
            &self.config.secrets.master_encryption_key,
            self.deployment_id,
            dto,
        )
        .await?;

        Ok(())
    }

    async fn update_status_failed(&self, error_message: &str) -> Result<(), AppError> {
        tracing::error!("[deployment_status]: Failed - {}", error_message);
        let dto = UpdateDeploymentStatusRequest {
            status: DeploymentStatus::Failed.as_str().to_string(),
            build_duration: if self.durations.build > 0 {
                Some(self.durations.build)
            } else {
                None
            },
            deploy_duration: None,
            error_message: Some(error_message.to_string()),
        };
        let _ = DeploymentsService::update_status_internal(
            &self.db,
            &self.config,
            &self.config.secrets.master_encryption_key,
            self.deployment_id,
            dto,
        )
        .await;
        Ok(())
    }
}
