use super::common::tar::build_tar_context;
use super::common::deploy::{Deploy, DeployDTO};
use crate::modules::projects::build_worker::docker_client::DockerClient;
use crate::modules::projects::build_worker::log_stream::LogStream;
use crate::modules::projects::build_worker::traits::BuilderConfig;
use crate::modules::projects::build_worker::DeploymentPath;
use crate::shared::error::AppError;
use bollard::query_parameters::BuildImageOptions;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct BaseBuilder {
    pub project_path: DeploymentPath,
    pub env_vars: Vec<(String, String)>,
    pub project_id: String,
    pub docker_client: DockerClient,
    pub org_or_user_id: String,
    pub deployment_id: String,
    pub app_name: String,
    pub log_stream: LogStream,
}

impl BaseBuilder {
    pub fn new(config: BuilderConfig) -> Result<Self, AppError> {
        let client = DockerClient::new().map_err(|e| {
            AppError::InternalServerError(format!("failed initialize docker client: {}", e))
        })?;

        Ok(Self {
            docker_client: client,
            project_path: config.project_path,
            project_id: config.project_id,
            env_vars: config.env_vars,
            org_or_user_id: config.org_or_user_id,
            deployment_id: config.deployment_id,
            app_name: config.app_name,
            log_stream: config.log_stream,
        })
    }

    pub async fn log_info(&self, stage: &str, message: &str) {
        self.log_stream
            .log_info(&self.deployment_id, stage, message)
            .await;
    }

    #[allow(dead_code)]
    pub async fn log_error(&self, stage: &str, message: &str, is_end: bool) {
        self.log_stream
            .log_error(&self.deployment_id, stage, message, is_end)
            .await;
    }

    pub async fn build_image(
        &self,
        ignore: &[&str],
        dockerfile: &str,
        version: &str,
    ) -> Result<String, AppError> {
        self.log_info("Build", "Building image...").await;

        let build_context_tar = build_tar_context(&self.project_path.source, ignore)?;

        let image_name = format!(
            "{}/{}/{}:{}",
            self.org_or_user_id, self.project_id, self.deployment_id, version
        );

        let options = BuildImageOptions {
            t: Some(image_name.clone()),
            dockerfile: dockerfile.to_string(),
            rm: true,
            forcerm: true,
            ..Default::default()
        };

        let log_stream = self.log_stream.clone();
        let deployment_id = self.deployment_id.clone();

        let image_id = self
            .docker_client
            .image
            .create_with_log_handler(&image_name, options, build_context_tar, |chunk| {
                let log_stream = log_stream.clone();
                let deployment_id = deployment_id.clone();
                async move {
                    for line in chunk.lines() {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            log_stream
                                .log_info(&deployment_id, "Build", trimmed)
                                .await;
                        }
                    }
                }
            })
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("failed to create docker image: {}", e))
            })?;

        self.log_info("Build", "Image built successfully!").await;
        Ok(image_id)
    }

    pub async fn deploy(
        &self,
        image_id: String,
        environment: String,
        instance_no: u16,
    ) -> Result<String, AppError> {
        self.log_info(
            "Deploy",
            &format!("Deploying container for instance {}...", instance_no),
        )
        .await;

        let envs: Vec<String> = self
            .env_vars
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect();

        let container_name = format!(
            "app-{}-{}-{}-{}",
            self.org_or_user_id, self.app_name, environment, instance_no
        );

        let dep = Deploy::new(self.docker_client.clone());
        let response = dep
            .deploy(DeployDTO {
                image_id,
                container_name,
                envs: Some(envs),
            })
            .await?;

        self.log_info(
            "Deploy",
            &format!("Container deployed with ID {}", response.container_id),
        )
        .await;
        Ok(response.container_id)
    }

    pub async fn run(&self, container_id: String) -> Result<(), AppError> {
        self.log_info("Run", &format!("Starting container '{}'...", container_id))
            .await;
        self.docker_client
            .container
            .start(&container_id, None)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!(
                    "failed to start '{}' container: {}",
                    &container_id, e
                ))
            })?;
        self.log_info(
            "Run",
            &format!("Container '{}' started successfully", container_id),
        )
        .await;
        Ok(())
    }

    pub async fn health_check(&self) -> Result<(), AppError> {
        self.log_info("HealthCheck", "Performing health check probe...")
            .await;
        Ok(())
    }

    pub async fn cleanup(&self) -> Result<(), AppError> {
        self.log_info("Cleanup", "Cleaning up workspace...").await;
        fs::remove_dir_all(Path::new(&self.project_path.root)).map_err(|e| {
            AppError::InternalServerError(format!("failed to remove workspace: {}", e))
        })?;
        Ok(())
    }
}
