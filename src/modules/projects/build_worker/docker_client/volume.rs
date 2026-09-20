use super::errors::DockerClientError;
use bollard::Docker;
use bollard::plugin::{Volume, VolumeCreateRequest};
use bollard::query_parameters::RemoveVolumeOptions;

#[derive(Clone)]
pub struct DockerVolume {
    client: Docker,
}

impl DockerVolume {
    pub fn new(client: Docker) -> Self {
        Self { client }
    }

    pub async fn create(&self, config: VolumeCreateRequest) -> Result<Volume, DockerClientError> {
        let res = self.client.create_volume(config).await.map_err(|e| {
            DockerClientError::NetworkError(format!(
                "failed to create new volume: {}",
                e.to_string()
            ))
        })?;
        Ok(res)
    }

    pub async fn remove(&self, volume_name: &str) -> Result<(), DockerClientError> {
        self.client
            .remove_volume(volume_name, None::<RemoveVolumeOptions>)
            .await
            .map_err(|e| {
                DockerClientError::NetworkError(format!(
                    "failed to remove volume: {}",
                    e.to_string()
                ))
            })?;

        Ok(())
    }
}
