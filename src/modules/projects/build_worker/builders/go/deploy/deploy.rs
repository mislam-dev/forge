use crate::modules::projects::build_worker::docker_client::DockerClient;
use crate::modules::projects::build_worker::port_manager::PortManager;
use crate::shared::error::AppError;
use bollard::{
    plugin::{ContainerCreateBody, HostConfig, PortBinding, RestartPolicy, RestartPolicyNameEnum},
    query_parameters::CreateContainerOptions,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct DeployDTO {
    pub image_id: String,
    pub container_name: String,
    pub envs: Option<Vec<String>>,
}

pub struct DeployResponse {
    pub container_id: String,
}

pub struct Deploy {
    docker_client: DockerClient,
    port_manager: PortManager,
}

impl Deploy {
    pub fn new(docker_client: DockerClient) -> Self {
        let port_manager = PortManager::new(4000, 4999);
        Self {
            docker_client,
            port_manager,
        }
    }

    pub async fn deploy(&self, dto: DeployDTO) -> Result<DeployResponse, AppError> {
        if let Some(container_summary) = self
            .docker_client
            .container
            .exist(&dto.container_name)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("failed to get container: {}", e))
            })?
        {
            if let Some(container_id) = &container_summary.id {
                let _ = self.docker_client.container.stop(container_id).await;
                let _ = self.docker_client.container.remove(container_id).await;
            }
        }

        let options = CreateContainerOptions {
            name: Some(dto.container_name),
            ..Default::default()
        };

        let bind_port = self.port_manager.acquire().await?;
        let port_key = "8080/tcp".to_string();

        let default_port_bindings = HashMap::from([(
            port_key.clone(),
            Some(vec![PortBinding {
                host_ip: Some("0.0.0.0".to_string()),
                host_port: Some(bind_port.to_string()),
            }]),
        )]);

        let default_exposed_port = vec![port_key];
        let mut default_envs = vec!["PORT=8080".to_string()];
        if let Some(envs) = dto.envs {
            default_envs.extend(envs);
        }

        let config = ContainerCreateBody {
            image: Some(dto.image_id),
            exposed_ports: Some(
                default_exposed_port
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect(),
            ),
            env: Some(
                default_envs
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect(),
            ),
            host_config: Some(HostConfig {
                port_bindings: Some(default_port_bindings),
                memory: Some(1024 * 1024 * 1024),
                restart_policy: Some(RestartPolicy {
                    name: Some(RestartPolicyNameEnum::UNLESS_STOPPED),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        let container_id = self
            .docker_client
            .container
            .create(Some(options), config)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("Failed to create Go container: {}", e))
            })?;

        Ok(DeployResponse { container_id })
    }
}
