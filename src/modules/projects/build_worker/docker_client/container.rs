use crate::modules::projects::build_worker::docker_client::errors::DockerClientError;
use bollard::Docker;
use bollard::plugin::ContainerCreateBody;
use bollard::query_parameters::{
    CreateContainerOptions, RemoveContainerOptions, StartContainerOptions, StopContainerOptions,
};
#[derive(Clone)]
pub struct DockerContainer {
    client: Docker,
}

impl DockerContainer {
    pub fn new(client: Docker) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        options: Option<CreateContainerOptions>,
        config: ContainerCreateBody,
    ) -> Result<String, DockerClientError> {
        let container = self
            .client
            .create_container(options, config)
            .await
            .map_err(|e| {
                DockerClientError::ContainerError(format!(
                    "Failed to create container: {}",
                    e.to_string()
                ))
            })?;

        Ok(container.id)
    }

    pub async fn start(
        &self,
        container_id: &str,
        options: StartContainerOptions,
    ) -> Result<(), DockerClientError> {
        self.client
            .start_container(container_id, Some(options))
            .await
            .map_err(|e| {
                DockerClientError::ContainerError(format!(
                    "Failed to start container: {}",
                    e.to_string()
                ))
            })?;

        Ok(())
    }

    pub async fn stop(&self, container_id: &str) -> Result<(), DockerClientError> {
        let options = StopContainerOptions {
            ..Default::default()
        };

        self.client
            .stop_container(container_id, Some(options))
            .await
            .map_err(|e| {
                DockerClientError::ContainerError(format!(
                    "Failed to stop container: {}",
                    e.to_string()
                ))
            })?;
        Ok(())
    }

    pub async fn remove(&self, container_id: &str) -> Result<(), DockerClientError> {
        let options = RemoveContainerOptions {
            ..Default::default()
        };

        self.client
            .remove_container(container_id, Some(options))
            .await
            .map_err(|e| {
                DockerClientError::ContainerError(format!(
                    "Failed to remove container: {}",
                    e.to_string()
                ))
            })?;
        Ok(())
    }
}
