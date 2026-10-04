use super::super::super::common::tar::build_tar_context;
use crate::modules::projects::build_worker::docker_client::DockerClient;
use crate::modules::projects::build_worker::log_stream::LogStream;
use crate::shared::error::AppError;
use bollard::query_parameters::BuildImageOptions;
use std::path::PathBuf;

pub struct Builder {
    source_path: PathBuf,
    docker_client: DockerClient,
    log_stream: LogStream,
    deployment_id: String,
}

impl Builder {
    pub fn new(
        docker_client: DockerClient,
        source_path: PathBuf,
        log_stream: LogStream,
        deployment_id: String,
    ) -> Self {
        Self {
            source_path,
            docker_client,
            log_stream,
            deployment_id,
        }
    }

    pub async fn build(&self, image_name: &str) -> Result<String, AppError> {
        let ignore = vec!["target", ".git", "logs", "node_modules", "dist"];
        let build_context_tar = build_tar_context(&self.source_path, &ignore)?;

        let options = BuildImageOptions {
            t: Some(image_name.to_string()),
            dockerfile: "Dockerfile".to_string(),
            rm: true,
            forcerm: true,
            ..Default::default()
        };

        let log_stream = self.log_stream.clone();
        let deployment_id = self.deployment_id.clone();

        let image_id = self
            .docker_client
            .image
            .create_with_log_handler(image_name, options, build_context_tar, |chunk| {
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

        Ok(image_id)
    }
}
