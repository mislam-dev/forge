use std::fs;
use std::path::Path;

use super::builder::Builder;
use super::deploy::{Deploy, DeployDTO};
use super::file_generator::FileGenerator;
use super::validation::Validation;
use crate::{
    modules::projects::build_worker::{
        DeploymentPath, docker_client::DockerClient, traits::ProjectBuilder,
    },
    shared::error::AppError,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct PythonBuilder {
    project_path: DeploymentPath,
    env_vars: Vec<(String, String)>,
    project_id: String,
    docker_client: DockerClient,
    org_or_user_id: String,
    deployment_id: String,
    app_name: String,
}

impl PythonBuilder {
    pub fn new(
        project_path: DeploymentPath,
        project_id: String,
        env_vars: Vec<(String, String)>,
        org_or_user_id: String,
        deployment_id: String,
        app_name: String,
    ) -> Result<Self, AppError> {
        let client = DockerClient::new().map_err(|e| {
            AppError::InternalServerError(format!("failed initialize docker client: {}", e))
        })?;

        Ok(Self {
            project_path,
            project_id,
            env_vars,
            docker_client: client,
            org_or_user_id,
            deployment_id,
            app_name,
        })
    }
}

#[async_trait]
impl ProjectBuilder for PythonBuilder {
    async fn download_pkgs(&self) -> Result<(), AppError> {
        Ok(())
    }

    async fn validate(&self) -> Result<(), AppError> {
        Validation::validate(&self.project_path.source.to_string_lossy())
    }

    async fn create_files(&self) -> Result<(), AppError> {
        let file_gen = FileGenerator::new(self.project_path.source.clone());
        file_gen.create_dockerfile().await?;
        file_gen.create_dockerignore().await?;
        Ok(())
    }

    async fn build(&self, version: String) -> Result<String, AppError> {
        let builder = Builder::new(self.docker_client.clone(), self.project_path.source.clone());
        let image_name = format!(
            "{}/{}/{}:{}",
            self.org_or_user_id, self.project_id, self.deployment_id, version
        );
        let image_id = builder.build(&image_name).await?;
        Ok(image_id)
    }

    async fn deploy(
        &self,
        image_id: String,
        environment: String,
        instance_no: u16,
    ) -> Result<String, AppError> {
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

        Ok(response.container_id)
    }

    async fn run(&self, container_id: String) -> Result<(), AppError> {
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

        Ok(())
    }

    async fn health_check(&self) -> Result<(), AppError> {
        Ok(())
    }

    async fn cleanup(&self) -> Result<(), AppError> {
        fs::remove_dir_all(Path::new(&self.project_path.root)).map_err(|e| {
            AppError::InternalServerError(format!("failed to remove workspace: {}", e))
        })?;
        Ok(())
    }
}
