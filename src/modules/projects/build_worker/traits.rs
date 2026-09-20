use crate::shared::error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait ProjectBuilder: Send + Sync {
    async fn download_pkgs(&self) -> Result<(), AppError>;
    async fn validate(&self) -> Result<(), AppError>;
    async fn create_files(&self) -> Result<(), AppError>;
    async fn build(&self) -> Result<String, AppError>;
    async fn deploy(&self, image_id: String) -> Result<String, AppError>;
    async fn run(&self, container_id: String) -> Result<(), AppError>;
    async fn health_check(&self) -> Result<(), AppError>;
    async fn cleanup(&self) -> Result<(), AppError>;
}
