use super::builder::NodejsDockerBuilder;
use super::client::NodejsDockerClient;
use super::image_builder::DockerImageDetails;
use super::image_runner::ImageRunner;
use crate::shared::error::AppError;
use bollard::Docker;
use std::path::PathBuf;

pub struct BuildOutput {
    success: bool,
    docker_image: DockerImageDetails,
}

pub struct NodejsDockerWorker {
    client: Docker,
    source_path: PathBuf,
    project_id: String,
}

impl NodejsDockerWorker {
    pub fn new(source_path: PathBuf, project_id: String) -> Self {
        let docker = NodejsDockerClient::new();

        Self {
            client: docker,
            source_path,
            project_id,
        }
    }

    pub async fn execute(&self) -> Result<BuildOutput, AppError> {
        let builder = NodejsDockerBuilder::new(
            self.client.clone(),
            self.source_path.clone(),
            self.project_id.clone(),
        );
        let docker_image = builder.build().await?;

        let runner = ImageRunner::new(self.client.clone(), docker_image.clone());
        let b = runner.start().await?;
        // todo: deploy within a container
        // todo: heath check of the container

        Ok(BuildOutput {
            success: true,
            docker_image,
        })
    }
}
