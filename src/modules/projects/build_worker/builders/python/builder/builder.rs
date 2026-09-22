use super::super::super::common::tar::build_tar_context;
use crate::modules::projects::build_worker::docker_client::DockerClient;
use crate::shared::error::AppError;
use bollard::query_parameters::BuildImageOptions;
use std::path::PathBuf;

pub struct Builder {
    source_path: PathBuf,
    docker_client: DockerClient,
}

impl Builder {
    pub fn new(docker_client: DockerClient, source_path: PathBuf) -> Self {
        Self {
            source_path,
            docker_client,
        }
    }

    pub async fn build(&self, image_name: &str) -> Result<String, AppError> {
        let ignore = vec!["__pycache__", ".venv", "venv", ".git", "logs"];
        let build_context_tar = build_tar_context(&self.source_path, &ignore)?;

        let options = BuildImageOptions {
            t: Some(image_name.to_string()),
            dockerfile: "Dockerfile".to_string(),
            rm: true,
            forcerm: true,
            ..Default::default()
        };

        let image_id = self
            .docker_client
            .image
            .create(image_name, options, build_context_tar)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("failed to create docker image: {}", e))
            })?;

        Ok(image_id)
    }
}
