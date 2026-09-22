use super::super::super::super::docker_client::DockerClient;
use crate::shared::error::AppError;
use bollard::query_parameters::BuildImageOptions;
use bytes::Bytes;
use std::path::PathBuf;
use tar::Builder as TarBuilder;

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
        let build_context_tar = self.build_tar_context()?;

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
                AppError::InternalServerError(format!(
                    "failed to create docker image: {}",
                    e.to_string()
                ))
            })?;

        Ok(image_id)
    }

    fn build_tar_context(&self) -> Result<Bytes, AppError> {
        let mut archive = TarBuilder::new(Vec::new());
        let ignore = vec!["target", ".git", "logs", "node_modules", "dist"];

        for entry in std::fs::read_dir(&self.source_path).map_err(|e| {
            AppError::InternalServerError(format!("Failed to read artifact tar: {}", e))
        })? {
            let entry = entry.map_err(|e| {
                AppError::InternalServerError(format!("Failed to read artifact tar: {}", e))
            })?;

            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if ignore.iter().any(|i| name_str.starts_with(i)) {
                continue;
            }

            let path = entry.path();
            if path.is_dir() {
                archive.append_dir_all(&name, &path).map_err(|e| {
                    AppError::InternalServerError(format!("Failed to read artifact tar: {}", e))
                })?;
            } else {
                archive.append_path_with_name(&path, &name).map_err(|e| {
                    AppError::InternalServerError(format!("Failed to read artifact tar: {}", e))
                })?;
            }
        }

        let build_context_tar = archive.into_inner().map_err(|e| {
            AppError::InternalServerError(format!("Failed to read artifact tar: {}", e))
        })?;

        let build_context_tar = Bytes::from(build_context_tar);

        Ok(build_context_tar)
    }
}
