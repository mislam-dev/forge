use crate::shared::error::AppError;
use bollard::Docker;
use bollard::container::LogOutput;
use bollard::query_parameters::{LogsOptionsBuilder, RemoveContainerOptions};
use std::time::Duration;
use tokio::time::timeout;
use tokio_stream::StreamExt;

pub struct NodeDockerUtils;

impl NodeDockerUtils {
    pub async fn cleanup_container(client: &Docker, container_id: &str) -> Result<(), AppError> {
        client
            .remove_container(
                container_id,
                Some(RemoveContainerOptions {
                    force: true,
                    v: true,
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| {
                AppError::InternalServerError(format!(
                    "Failed to remove container: {}",
                    e.to_string()
                ))
            })?;

        Ok(())
    }

    pub async fn wait_with_timeout(
        client: &Docker,
        container_id: &str,
        duration: Duration,
    ) -> Result<i64, AppError> {
        let wait_future = async {
            let mut stream = client.wait_container(container_id, None);
            match stream.next().await {
                Some(Ok(response)) => Ok(response.status_code),
                Some(Err(e)) => Err(AppError::InternalServerError(format!(
                    "Wait Error: {}",
                    e.to_string()
                ))),
                None => Err(AppError::InternalServerError(
                    "Wait Error: No response".to_string(),
                )),
            }
        };

        let exit_code = timeout(duration, wait_future)
            .await
            .map_err(|e| AppError::InternalServerError(format!("Build timeout: {}", e)))?
            .map_err(|e| AppError::InternalServerError(format!("Build failed: {}", e)))?;

        Ok(exit_code)
    }

    pub async fn collect_logs(
        client: &Docker,
        container_id: &str,
    ) -> Result<Vec<String>, AppError> {
        let options = LogsOptionsBuilder::new()
            .stdout(true)
            .stderr(true)
            .follow(false)
            .timestamps(true)
            .build();
        let mut stream = client.logs(container_id, Some(options));
        let mut output: Vec<String> = vec![];

        while let Some(chunk) = stream.next().await {
            match chunk.map_err(|e| {
                AppError::InternalServerError(format!(
                    "Failed to extract logs stream: {}",
                    e.to_string()
                ))
            })? {
                LogOutput::StdOut { message } => {
                    output.push(String::from_utf8_lossy(&message).to_string())
                }
                LogOutput::StdErr { message } => {
                    output.push(String::from_utf8_lossy(&message).to_string())
                }
                _ => {}
            }
        }

        Ok(output)
    }
}
