use std::fmt;

use crate::modules::projects::build_worker::docker_client::errors::DockerClientError;

use super::container::DockerContainer;
use super::image::DockerImage;
use super::network::DockerNetwork;
use super::volume::DockerVolume;
use bollard::Docker;

#[derive(Clone)]
pub struct DockerClient {
    client: Docker,
    pub image: DockerImage,
    pub container: DockerContainer,
    pub networks: DockerNetwork,
    pub volume: DockerVolume,
}

impl DockerClient {
    pub fn new() -> Result<Self, DockerClientError> {
        let client = Docker::connect_with_local_defaults().map_err(|e| {
            DockerClientError::ConnectionError(format!(
                "Failed to connect to docker: {}",
                e.to_string()
            ))
        })?;
        Ok(Self {
            client: client.clone(),
            image: DockerImage::new(client.clone()),
            container: DockerContainer::new(client.clone()),
            networks: DockerNetwork::new(client.clone()),
            volume: DockerVolume::new(client.clone()),
        })
    }
}

impl fmt::Debug for DockerClient {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}
