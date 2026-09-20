use thiserror::Error;

#[derive(Debug, Error)]
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
