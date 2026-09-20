use super::errors::DockerClientError;
use bollard::Docker;
use bollard::plugin::{NetworkCreateRequest, NetworkCreateResponse};

#[derive(Clone)]
pub struct DockerNetwork {
    client: Docker,
}

impl DockerNetwork {
    pub fn new(client: Docker) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        config: NetworkCreateRequest,
    ) -> Result<NetworkCreateResponse, DockerClientError> {
        let res = self.client.create_network(config).await.map_err(|e| {
            DockerClientError::NetworkError(format!(
                "failed to create new network: {}",
                e.to_string()
            ))
        })?;
        Ok(res)
    }

    pub async fn remove(&self, network_name: &str) -> Result<(), DockerClientError> {
        self.client
            .remove_network(network_name)
            .await
            .map_err(|e| {
                DockerClientError::NetworkError(format!(
                    "failed to create new network: {}",
                    e.to_string()
                ))
            })?;

        Ok(())
    }
}
