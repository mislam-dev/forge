use super::image_builder::DockerImageDetails;
use crate::shared::error::AppError;
use bollard::Docker;
use bollard::plugin::{ContainerCreateBody, HostConfig, PortBinding};
use bollard::query_parameters::{
    CreateContainerOptions, RemoveContainerOptions, StartContainerOptions, StopContainerOptions,
};

#[derive(Debug)]
pub struct ImageRunner {
    client: Docker,
    image_details: DockerImageDetails,
}

impl ImageRunner {
    pub fn new(client: Docker, image_details: DockerImageDetails) -> Self {
        Self {
            client,
            image_details,
        }
    }

    pub async fn start(&self) -> Result<(String, String), AppError> {
        // let container_name = format!("{}_container", self.image_details.tag);
        let container_name = format!("test_a_container");
        let container_options = CreateContainerOptions {
            name: Some(container_name.clone()),
            ..Default::default()
        };

        // todo: need a available free port finder on the system.
        let config = ContainerCreateBody {
            image: Some(self.image_details.id.clone()),
            exposed_ports: Some(["3000/tcp".to_string()].into_iter().collect()),
            host_config: Some(HostConfig {
                port_bindings: Some(
                    [(
                        "3000/tcp".to_string(),
                        Some(vec![
                            PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("3900".to_string()),
                            },
                            PortBinding {
                                host_ip: Some("0.0.0.0".to_string()),
                                host_port: Some("4900".to_string()),
                            },
                        ]),
                    )]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            }),
            ..Default::default()
        };
        let container = self
            .client
            .create_container(Some(container_options), config)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!(
                    "Failed to create container: {}",
                    e.to_string()
                ))
            })?;

        let options = StartContainerOptions {
            ..Default::default()
        };

        self.client
            .start_container(&container.id, Some(options))
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!(
                    "Failed to start container: {}",
                    e.to_string()
                ))
            })?;

        Ok((container.id.clone(), container_name))
    }

    pub async fn stop(&self, id: String) -> Result<(), AppError> {
        let options = StopContainerOptions {
            ..Default::default()
        };

        self.client
            .stop_container(&id, Some(options))
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!(
                    "Failed to stop container: {}",
                    e.to_string()
                ))
            })?;
        Ok(())
    }

    pub async fn remove(&self, id: String) -> Result<(), AppError> {
        let options = RemoveContainerOptions {
            ..Default::default()
        };

        self.client
            .remove_container(&id, Some(options))
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!(
                    "Failed to start container: {}",
                    e.to_string()
                ))
            })?;
        Ok(())
    }
}
