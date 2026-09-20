use super::super::super::traits::ProjectBuilder;
use super::file_generator::FileGenerator;
use super::validation::Validation;
use crate::{
    modules::projects::build_worker::{DeploymentPath, docker_client::DockerClient},
    shared::error::AppError,
};
use async_trait::async_trait;

use super::builder::Builder;

#[derive(Debug, Clone)]
pub struct NodeJsBuilder {
    project_path: DeploymentPath,
    env_vars: Vec<(String, String)>,
    project_id: String,
    docker_client: DockerClient,
}

impl NodeJsBuilder {
    pub fn new(
        project_path: DeploymentPath,
        project_id: String,
        env_vars: Vec<(String, String)>,
    ) -> Result<Self, AppError>
    where
        Self: Sized,
    {
        let client = DockerClient::new().map_err(|e| {
            AppError::InternalServerError(format!(
                "failed initialize docker client: {}",
                e.to_string()
            ))
        })?;

        Ok(Self {
            project_path,
            project_id,
            env_vars,
            docker_client: client,
        })
    }
}

#[async_trait]
impl ProjectBuilder for NodeJsBuilder {
    async fn download_pkgs(&self) -> Result<(), AppError> {
        Ok(())
    }
    async fn create_files(&self) -> Result<(), AppError> {
        let file_gen = FileGenerator::new(self.project_path.source.clone());
        file_gen.create_dockerfile().await?;
        file_gen.create_dockerignore().await?;

        Ok(())
    }

    async fn validate(&self) -> Result<(), AppError> {
        Ok(Validation::validate(
            &self.project_path.source.to_string_lossy().to_string(),
        )?)
    }

    async fn build(&self) -> Result<String, AppError> {
        let builder = Builder::new(self.docker_client.clone(), self.project_path.source.clone());
        let image_id = builder.build("image_name").await?;

        Ok(image_id)
    }

    async fn deploy(&self, image_id: String) -> Result<String, AppError> {
        // todo: deploy the builded image with proper port mapping
        // todo: update deployment record

        Ok("".to_string())
    }
    async fn run(&self, container_id: String) -> Result<(), AppError> {
        // todo: deploy the builded image with proper port mapping
        // todo: update deployment record

        Ok(())
    }

    async fn cleanup(&self) -> Result<(), AppError> {
        // todo: remove builded image
        // todo: remove docker container
        Ok(())
    }

    async fn health_check(&self) -> Result<(), AppError> {
        Ok(())
    }
}
