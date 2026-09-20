use super::super::deployments::dto::UpdateDeploymentStatusRequest;
use super::super::deployments::status::DeploymentStatus;
use super::builders::NodeJsBuilder;
use super::cloning::RepoCloning;
use super::deployment_durations::DeploymentDurations;
use super::deployment_path::{DeploymentPath, DeploymentPathDTO, Owner, OwnerType};
use crate::config::AppConfig;
use crate::modules::projects::build_worker::traits::{self, ProjectBuilder};
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
    project_type: Option<ProjectType>,
    builder: Option<Box<dyn traits::ProjectBuilder + Send + Sync>>,
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
        _env_vars: &[(&str, &str)], // todo: update this later
        repo_url: String,
    ) -> Self {
        Self {
            db: db.clone(),
            config: config.clone(),
            deployment_id,
            project_id,
            org_or_user_id,
            owner_type,
            pat_token: pat_token.map(|s| s.to_string()),
            env_vars: vec![],
            repo_url,                   // todo: must be dynamic
            branch: "main".to_string(), // todo: must be dynamic
            durations: DeploymentDurations::new(),
            project_type: None,
            builder: None,
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

    fn infer_project_type(&mut self) -> Result<(), AppError> {
        let path = self.construct_path();
        if !path.source.exists() {
            return Err(AppError::InternalServerError(
                "Project directory does not exist.".to_string(),
            ));
        }

        let dockerfile_path = path.source.join("Dockerfile");
        let dockerfile_exists = dockerfile_path.exists();

        if dockerfile_exists {
            self.project_type = Some(ProjectType::DockerContainer);
            return Ok(());
        }

        let package_json_path = path.source.join("package.json");
        if package_json_path.exists() {
            self.project_type = Some(ProjectType::NodeJs);
            return Ok(());
        }

        let cargo_toml_path = path.source.join("Cargo.toml");
        if cargo_toml_path.exists() {
            self.project_type = Some(ProjectType::Rust);
            return Ok(());
        }

        let requirements_txt_path = path.source.join("requirements.txt");
        if requirements_txt_path.exists() {
            self.project_type = Some(ProjectType::Python);
            return Ok(());
        }

        let go_mod_path = path.source.join("go.mod");
        if go_mod_path.exists() {
            self.project_type = Some(ProjectType::Go);
            return Ok(());
        }

        self.project_type = Some(ProjectType::StaticFiles);

        Ok(())
    }

    fn get_builder(&mut self) -> Result<Option<NodeJsBuilder>, AppError> {
        self.infer_project_type()?;
        let project_path = self.construct_path();
        if let Some(project_type) = &self.project_type {
            let builder = match project_type {
                ProjectType::NodeJs => {
                    NodeJsBuilder::new(project_path, self.project_id.to_string(), vec![])?
                }
                _ => NodeJsBuilder::new(project_path, self.project_id.to_string(), vec![])?,
            };
            return Ok(Some(builder));
        }

        Ok(None)
    }

    pub async fn execute_pipeline(&mut self) -> Result<(), AppError> {
        let _ = &self.durations.set_start_time();
        self.log_info("clone", "cloning into...");
        // Step 1: Clone Repository (Queued -> Building)
        RepoCloning::new(
            self.pat_token.clone(),
            self.repo_url.clone(),
            self.branch.clone(),
            self.construct_path().source.clone(),
        )
        .clone()
        .await?;

        let builder = self.get_builder()?;

        if builder.is_none() {
            return Err(AppError::BadRequest(format!("No builder found!")));
        }
        let builder = builder.unwrap();

        // Step 2: Validation
        builder.validate().await?;

        // Step 3: generate necessary files
        builder.create_files().await?;

        // Step 4: Build Docker Image
        let image_id = builder.build().await?;

        let _ = &self.durations.set_build_duration();

        // Step 5: Deploy Container (Building -> Deploying)
        let container_id = builder.deploy(image_id).await?;

        // Step 6: Run container Deploying -> Running)
        builder.run(container_id).await?;

        // Step 7: Health Check Probe (Running -> Success)
        builder.health_check().await?;

        // Step 8: Cleanup
        builder.cleanup().await?;

        Ok(())
    }

    fn log_info(&self, step: &str, log_line: &str) {
        tracing::info!(deployment_id = %self.deployment_id, step = %step, "{}", log_line);
    }

    async fn update_status_internal(&self, status: DeploymentStatus) -> Result<(), AppError> {
        let _dto: UpdateDeploymentStatusRequest = match status {
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

        // let _c = DeploymentsService::update_status_internal(
        //     &self.db,
        //     &self.config,
        //     &self.config.secrets.master_encryption_key,
        //     self.deployment_id,
        //     dto,
        // )
        // .await?;
        // todo: uncomment later
        Ok(())
    }
}
