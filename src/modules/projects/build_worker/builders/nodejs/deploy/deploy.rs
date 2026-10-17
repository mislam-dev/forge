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
    pub exposed_ports: Option<Vec<String>>,
    pub container_name: String,
    pub envs: Option<Vec<String>>,
    pub port_bindings: Option<HashMap<String, Option<Vec<PortBinding>>>>,
    pub primary_port: Option<u16>,
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
        if let Some(container_summary) = self
            .docker_client
            .container
            .exist(&dto.container_name)
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!("failed to get container: {}", e))
            })?
            && let Some(container_id) = &container_summary.id {
                self.docker_client
                    .container
                    .stop(container_id)
                    .await
                    .map_err(|e| {
                        AppError::InternalServerError(format!(
                            "failed to stop existing '{}' container: {}",
                            container_id,
                            e
                        ))
                    })?;
                self.docker_client
                    .container
                    .remove(container_id)
                    .await
                    .map_err(|e| {
                        AppError::InternalServerError(format!(
                            "failed to remove existing '{}' container: {}",
                            container_id,
                            e
                        ))
                    })?;
            }

        let options = CreateContainerOptions {
            name: Some(dto.container_name),
            ..Default::default()
        };

        let bind_port = self.port_manager.acquire().await?;
        let primary_port = dto.primary_port.unwrap_or(3000);
        let primary_port_key = format!("{}/tcp", primary_port);

        let mut default_port_bindings = HashMap::from([(
            primary_port_key.clone(),
            Some(vec![PortBinding {
                host_ip: Some("0.0.0.0".to_string()),
                host_port: Some(bind_port.to_string()),
            }]),
        )]);

        if let Some(bindings) = dto.port_bindings {
            default_port_bindings.extend(bindings);
        }

        let mut default_exposed_port = vec![primary_port_key];

        if let Some(expoosed_ports) = dto.exposed_ports {
            default_exposed_port.extend(expoosed_ports);
        }

        let mut default_envs = vec![format!("PORT={}", primary_port)];

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
                    e
                ))
            })?;

        Ok(DeployResponse {
            bind_ports: default_port_bindings,
            container_id,
        })
    }
}
