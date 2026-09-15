use crate::shared::error::AppError;
use bollard::Docker;
use bollard::query_parameters::{BuildImageOptions, CreateImageOptions};
use bytes::Bytes;
use http_body_util::{Either, Full};
use std::path::PathBuf;
use tar::Builder;
use tokio_stream::StreamExt;

pub struct DockerImageDetails {
    pub id: String,
    pub tag: String,
    pub ports: Vec<String>,
}

pub struct NodejsDockerImage {
    client: Docker,
    project_id: String,
}

impl NodejsDockerImage {
    pub fn new(client: Docker, project_id: String) -> Self {
        Self { client, project_id }
    }

    pub async fn build_image(
        &self,
        project_path: PathBuf,
        org_or_user_id: String,
        version_id: String,
    ) -> Result<DockerImageDetails, AppError> {
        let tag = format!("{}/{}:{}", org_or_user_id, &self.project_id, version_id);

        let build_context_tar = self.build_tar_context(project_path)?;

        let options = BuildImageOptions {
            t: Some(tag.clone()),
            dockerfile: "Dockerfile".to_string(),
            rm: true,
            forcerm: true,
            ..Default::default()
        };

        let body: Either<Full<Bytes>, _> = Either::Left(Full::new(build_context_tar));

        let mut stream = self.client.build_image(options, None, Some(body));

        while let Some(event) = stream.next().await {
            let event = event.map_err(|e| {
                AppError::InternalServerError(format!("Failed to build image 1: {}", e))
            })?;
            if let Some(v) = event.stream {
                print!("{}", v);
            }
        }

        let image = self.client.inspect_image(&tag).await.map_err(|e| {
            AppError::InternalServerError(format!("Failed to inspect image: {}", e))
        })?;
        let image_id = image.id.unwrap_or_else(|| String::from(""));

        Ok(DockerImageDetails {
            tag,
            id: image_id,
            ports: vec!["3000".to_string()],
        })
    }

    fn build_tar_context(&self, context_dir: PathBuf) -> Result<Bytes, AppError> {
        let mut archive = Builder::new(Vec::new());
        let ignore = vec!["target", ".git", "logs", "node_modules", "dist"];

        let tsconfig_path = &context_dir.join("tsconfig.json");
        if !tsconfig_path.exists() {
            return Err(AppError::InternalServerError(
                "tsconfig.json not found in build context".to_string(),
            ));
        }

        for entry in std::fs::read_dir(context_dir).map_err(|e| {
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
