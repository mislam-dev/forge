use super::super::super::super::docker_client::DockerClient;
use crate::{modules::projects::build_worker::port_manager::PortManager, shared::error::AppError};
use bollard::{
    plugin::{ContainerCreateBody, HostConfig, PortBinding, RestartPolicy, RestartPolicyNameEnum},
    query_parameters::CreateContainerOptions,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
pub struct DeployDTO {
    pub image_id: String,
    pub exposed_ports: Option<Vec<String>>, //
    pub container_name: String,
    pub envs: Option<Vec<String>>,
    pub port_bindings: Option<HashMap<String, Option<Vec<PortBinding>>>>,
}

pub struct DeployResponse {
    pub bind_ports: HashMap<String, Option<Vec<PortBinding>>>,
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
        // todo: check if container name is not used
        // todo: if container name is used, need take decisions, didn't taken any

        let options = CreateContainerOptions {
            name: Some(dto.container_name),
            ..Default::default()
        };

        let bind_port = self.port_manager.acquire().await?;
        let mut default_port_bindings = HashMap::from([(
            "3000/tcp".to_string(),
            Some(vec![PortBinding {
                host_ip: Some("0.0.0.0".to_string()),
                host_port: Some(bind_port.to_string()),
            }]),
        )]);

        if let Some(bindings) = dto.port_bindings {
            default_port_bindings.extend(bindings);
        }

        let mut default_exposed_port = vec!["3000/tcp".to_string()];

        if let Some(expoosed_ports) = dto.exposed_ports {
            default_exposed_port.extend(expoosed_ports);
        }

        let mut default_envs = vec!["PORT=3000".to_string()];

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
                port_bindings: Some(default_port_bindings.clone()),
                memory: Some(1024 * 1024 * 1024), // 1GB
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
                AppError::InternalServerError(format!(
                    "Failed to create container: {}",
                    e.to_string()
                ))
            })?;

        Ok(DeployResponse {
            bind_ports: default_port_bindings,
            container_id,
        })
    }
}
