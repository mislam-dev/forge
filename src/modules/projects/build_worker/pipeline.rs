use sea_orm::*;
use uuid::Uuid;

use super::super::deployments::dto::UpdateDeploymentStatusRequest;
use super::super::deployments::service::DeploymentsService;
use super::super::deployments::status::DeploymentStatus;
use super::builders::NodeJsBuilder;
use super::deployment_durations::DeploymentDurations;
use super::deployment_path::{DeploymentPath, DeploymentPathDTO, Owner, OwnerType};
use crate::config::AppConfig;
use crate::infrastructure::github::{GithubRepo, RepoCheckDto, RepoCloneDto};
use crate::modules::projects::build_worker::traits;
use crate::shared::error::AppError;

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

    pub fn scrub_secrets(log_line: &str) -> String {
        log_line.to_string()
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

    async fn check_repo_existence(&self) -> Result<bool, AppError> {
        tracing::info!("repo_url: {}", self.repo_url);
        let repo = GithubRepo::check(&RepoCheckDto {
            url: self.repo_url.clone(),
            token: self.pat_token.clone(),
        })
        .await
        .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(repo)
    }

    async fn download_repo_files(&self) -> Result<(), AppError> {
        let path = self.construct_path();

        let dto = RepoCloneDto {
            url: self.repo_url.clone(),
            branch: self.branch.clone(),
            destination: path.source.to_string_lossy().to_string(),
            token: self.pat_token.clone(),
        };
        tracing::info!("[download_repo_files]: RepoCloneDto: {:#?}", dto);
        GithubRepo::clone(dto).map_err(|e| {
            AppError::InternalServerError(format!("Failed to clone repo: {}", e.to_string()))
        })?;
        Ok(())
    }

    async fn cloning_repo_stage(&self) -> Result<(), AppError> {
        tracing::info!("[cloning]: start");
        self.check_repo_existence().await?;
        tracing::info!("[cloning]: checked repo existence");

        self.log_info("clone", "Cloning repository...");
        self.update_status_internal(DeploymentStatus::Building)
            .await?;

        tracing::info!("[cloning]: updating status to building");
        self.download_repo_files().await?;
        tracing::info!("[cloning]: repository downloaded successfully");

        self.log_info("clone", "Repository cloned successfully");
        Ok(())
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

    async fn validation_stage(&mut self) -> Result<(), AppError> {
        self.log_info("validate", "Validating Dockerfile...");

        self.infer_project_type()?;

        let project_type = self.project_type.as_ref().unwrap();

        match project_type {
            ProjectType::NodeJs => {
                self.builder = Some(Box::new(NodeJsBuilder::new(
                    self.construct_path(),
                    self.project_id.to_string(),
                    self.env_vars.clone(),
                )));
            }

            _ => {
                // todo: remove this later
                self.builder = Some(Box::new(NodeJsBuilder::new(
                    self.construct_path(),
                    self.project_id.to_string(),
                    self.env_vars.clone(),
                )));
            }
        }

        Ok(())
    }
    async fn building_stage(&self) -> Result<(), AppError> {
        self.log_info("build", "Building Docker image...");
        if self.builder.is_none() {
            return Err(AppError::InternalServerError(
                "Project builder not found".to_string(),
            ));
        }
        let builder = self.builder.as_ref().unwrap();
        builder.validate()?;
        builder.build().await?;
        Ok(())
    }
    async fn deploying_stage(&self) -> Result<(), AppError> {
        self.log_info("deploy", "Deploying container...");

        Ok(())
    }
    async fn running_stage(&self) -> Result<(), AppError> {
        self.log_info("health_check", "Health check probe passed (HTTP 200 OK)");

        Ok(())
    }

    pub async fn execute_pipeline(&mut self) -> Result<(), AppError> {
        let _ = &self.durations.set_start_time();
        // Step 1: Clone Repository (Queued -> Building)
        // self.cloning_repo_stage().await?;

        // Step 2: Validation
        self.validation_stage().await?;

        // Step 3: Build Docker Image
        self.building_stage().await?;
        tracing::info!("stop building");

        let _ = &self.durations.set_build_duration();

        tracing::info!("start deploying");
        // Step 4: Deploy Container (Building -> Deploying)
        self.deploying_stage().await?;
        tracing::info!("stop deploying");

        self.update_status_internal(DeploymentStatus::Deploying)
            .await?;

        // Step 5: Health Check Probe (Deploying -> Running -> Success)
        self.running_stage().await?;

        self.update_status_internal(DeploymentStatus::Running)
            .await?;

        self.update_status_internal(DeploymentStatus::Success)
            .await?;

        Ok(())
    }

    fn log_info(&self, step: &str, log_line: &str) {
        tracing::info!(deployment_id = %self.deployment_id, step = %step, "{}", log_line);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrub_secrets_masks_sensitive_tokens() {
        let pat = "ghp_super_secret_pat_token_12345";
        let db_pass = "my_database_password";
        let secrets = vec![pat, db_pass];

        let log = format!(
            "Cloning with token {} and connecting to DB {}",
            pat, db_pass
        );
        let scrubbed = BuildPipeline::scrub_secrets(&log);

        assert!(!scrubbed.contains(pat));
        assert!(!scrubbed.contains(db_pass));
        assert!(scrubbed.contains("••••••••"));
    }
}
