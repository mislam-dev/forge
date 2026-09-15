use super::file_generation::NodejsDockerFileGeneration;
use super::image_builder::DockerImageDetails;
use super::image_builder::NodejsDockerImage;
use crate::shared::error::AppError;
use bollard::Docker;
use std::path::PathBuf;

pub struct NodejsDockerBuilder {
    client: Docker,
    source_path: PathBuf,
    project_id: String,
    docker_image: NodejsDockerImage,
}

impl NodejsDockerBuilder {
    pub fn new(client: Docker, source_path: PathBuf, project_id: String) -> Self {
        let docker_image = NodejsDockerImage::new(client.clone(), project_id.clone());

        Self {
            client,
            source_path,
            project_id,
            docker_image,
        }
    }

    pub async fn build(&self) -> Result<DockerImageDetails, AppError> {
        let file_gen = NodejsDockerFileGeneration::new(self.source_path.clone());
        file_gen.create_dockerfile().await?;
        file_gen.create_dockerignore().await?;

        let docker_details = self
            .docker_image
            .build_image(
                self.source_path.clone(),
                self.project_id.clone(),
                "version_id".to_string(),
            )
            .await?;

        Ok(docker_details)
    }
}
