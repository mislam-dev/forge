use super::super::deployments::dto::UpdateDeploymentStatusRequest;
use super::super::deployments::status::DeploymentStatus;
use super::builders::NodeJsBuilder;
use super::cloning::RepoCloning;
use super::deployment_durations::DeploymentDurations;
use super::deployment_path::{DeploymentPath, DeploymentPathDTO, Owner, OwnerType};
use crate::config::AppConfig;
use crate::modules::projects::DeploymentsService;
use crate::modules::projects::build_worker::traits::ProjectBuilder;
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
        let project_type = self.infer_project_type()?;
        let project_path = self.construct_path();

        let builder: Box<dyn ProjectBuilder> = match project_type {
            ProjectType::NodeJs => Box::new(NodeJsBuilder::new(
                project_path,
                self.project_id.to_string(),
                self.env_vars.clone(),
                self.org_or_user_id.to_string(),
                self.deployment_id.to_string(),
                "testing_app".to_string(),
            )?),
            _ => Box::new(NodeJsBuilder::new(
                project_path,
                self.project_id.to_string(),
                self.env_vars.clone(),
                self.org_or_user_id.to_string(),
                self.deployment_id.to_string(),
                "testing_app".to_string(),
            )?),
        };
        return Ok(Some(builder));
    }

    pub async fn execute_pipeline(&mut self) -> Result<(), AppError> {
        self.log_info("Start", "Executing pipelines...");
        let _ = &self.durations.set_start_time();
        // Step 1: Clone Repository (Queued -> Building)
        self.log_info("Download", "Cloning repo data...");
        RepoCloning::new(
            self.pat_token.clone(),
            self.repo_url.clone(),
            self.branch.clone(),
            self.construct_path().source.clone(),
        )
        .clone()
        .await?;
        self.durations.set_clone_duration();

        self.log_info("Build", "0/3 | Starting Build");

        self.update_status_internal(DeploymentStatus::Building)
            .await?;

        let builder = self.get_builder()?;

        if builder.is_none() {
            return Err(AppError::BadRequest(format!("No builder found!")));
        }
        let builder = builder.unwrap();

        // Step 2: Validation
        self.log_info("Build", "1/7 | Validating...");
        builder.validate().await?;
        self.durations.set_validate_duration();

        // Step 3: generate necessary files
        builder.create_files().await?;
        self.log_info("Build", "2/7 | Creating necessary files...");

        let version_tag = "1.0.0".to_string();
        // Step 4: Build Docker Image
        self.log_info("Build", "3/7 | Building...");
        let image_id = builder.build(version_tag).await?;
        let _ = &self.durations.set_build_duration();

        // Step 5: Deploy Container (Building -> Deploying)
        self.update_status_internal(DeploymentStatus::Deploying)
            .await?;
        self.log_info("Build", "4/7 | Deploying...");
        let container_id = builder
            .deploy(image_id, "production".to_string(), 1)
            .await?;
        self.durations.set_deploy_duration();

        self.log_info("Build", "5/7 | Starting...");
        // Step 6: Run container Deploying -> Running)
        builder.run(container_id).await?;
        self.update_status_internal(DeploymentStatus::Running)
            .await?;
        self.log_info("Build", "6/7 | Checking...");
        // Step 7: Health Check Probe (Running -> Success)
        builder.health_check().await?;
        self.durations.set_health_check_duration();

        self.log_info("Build", "7/7 | Finishing...");
        // Step 8: Cleanup
        builder.cleanup().await?;

        Ok(())
    }

    fn log_info(&self, step: &str, log_line: &str) {
        let _log_v = format!("[{}]: {}", step, log_line);
        // todo: stream to log info to fe with SSE.
        tracing::info!("[{}]: {}", step, log_line);
    }

    async fn update_status_internal(&self, status: DeploymentStatus) -> Result<(), AppError> {
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
}
