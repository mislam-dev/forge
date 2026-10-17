use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DockerClientError {
    #[error("docker error: {0}")]
    ConnectionError(String),

    #[error("docker error: {0}")]
    ImageError(String),

    #[error("docker error: {0}")]
    ContainerError(String),

    #[error("docker error: {0}")]
    CrendentailsError(String),

    #[error("docker error: {0}")]
    NetworkError(String),
}
