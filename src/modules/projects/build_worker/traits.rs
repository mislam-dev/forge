use crate::{modules::projects::build_worker::DeploymentPath, shared::error::AppError};
use async_trait::async_trait;

#[async_trait]
pub trait ProjectBuilder: Send + Sync {
    fn validate(&self) -> Result<(), AppError>;
    async fn build(&self) -> Result<(), AppError>;
    fn deploy(&self) -> Result<(), AppError>;
    fn health_check(&self) -> Result<(), AppError>;
    fn cleanup(&self) -> Result<(), AppError>;
}
