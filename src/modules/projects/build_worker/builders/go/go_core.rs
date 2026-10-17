use super::super::base_builder::BaseBuilder;
use super::super::super::traits::{BuilderConfig, ProjectBuilder};
use super::file_generator::FileGenerator;
use super::validation::Validation;
use crate::shared::error::AppError;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct GoBuilder {
    base: BaseBuilder,
}

impl GoBuilder {
    pub fn new(config: BuilderConfig) -> Result<Self, AppError> {
        let base = BaseBuilder::new(config)?;
        Ok(Self { base })
    }
}

#[async_trait]
impl ProjectBuilder for GoBuilder {
    async fn download_pkgs(&self) -> Result<(), AppError> {
        self.base.log_info("Package", "Resolving Go modules...").await;
        Ok(())
    }

    async fn validate(&self) -> Result<(), AppError> {
        self.base.log_info("Validation", "Validating Go project...").await;
        Validation::validate(&self.base.project_path.source.to_string_lossy())?;
        self.base.log_info("Validation", "Go project validated successfully").await;
        Ok(())
    }

    async fn create_files(&self) -> Result<(), AppError> {
        self.base
            .log_info("Generator", "Generating Dockerfile and .dockerignore...")
            .await;
        let file_gen = FileGenerator::new(self.base.project_path.source.clone());
        file_gen.create_dockerfile().await?;
        file_gen.create_dockerignore().await?;
        self.base
            .log_info("Generator", "Configuration files created successfully")
            .await;
        Ok(())
    }

    async fn build(&self, version: String) -> Result<String, AppError> {
        self.base
            .build_image(&[".git", "logs", "bin"], "Dockerfile", &version)
            .await
    }

    async fn deploy(
        &self,
        image_id: String,
        environment: String,
        instance_no: u16,
    ) -> Result<String, AppError> {
        self.base.deploy(image_id, environment, instance_no).await
    }

    async fn run(&self, container_id: String) -> Result<(), AppError> {
        self.base.run(container_id).await
    }

    async fn health_check(&self) -> Result<(), AppError> {
        self.base.health_check().await
    }

    async fn cleanup(&self) -> Result<(), AppError> {
        self.base.cleanup().await
    }
}
