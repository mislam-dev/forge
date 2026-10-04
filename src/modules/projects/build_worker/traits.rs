use crate::{
    modules::projects::build_worker::{DeploymentPath, log_stream::LogStream},
    shared::error::AppError,
};
use async_trait::async_trait;

#[async_trait]
pub trait ProjectBuilder: Send + Sync {
    async fn download_pkgs(&self) -> Result<(), AppError>;
    async fn validate(&self) -> Result<(), AppError>;
    async fn create_files(&self) -> Result<(), AppError>;
    async fn build(&self, version: String) -> Result<String, AppError>;
    async fn deploy(
        &self,
        image_id: String,
        environment: String,
        instance_no: u16,
    ) -> Result<String, AppError>;
    async fn run(&self, container_id: String) -> Result<(), AppError>;
    async fn health_check(&self) -> Result<(), AppError>;
    async fn cleanup(&self) -> Result<(), AppError>;
}

pub struct BuilderConfig {
    pub project_path: DeploymentPath,
    pub project_id: String,
    pub env_vars: Vec<(String, String)>,
    pub org_or_user_id: String,
    pub deployment_id: String,
    pub app_name: String, // the "app" literal
    pub log_stream: LogStream,
}
