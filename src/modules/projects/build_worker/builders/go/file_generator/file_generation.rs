use crate::shared::error::AppError;
use std::fs;
use std::path::PathBuf;

pub struct FileGenerator {
    project_path: PathBuf,
}

impl FileGenerator {
    pub fn new(project_path: PathBuf) -> Self {
        Self { project_path }
    }

    pub async fn create_dockerfile(&self) -> Result<(), AppError> {
        let dockerfile_path = self.project_path.join("Dockerfile");
        if !dockerfile_path.exists() {
            const DOCKERFILE_TEMPLATE: &str = include_str!("./assets/Dockerfile");
            fs::write(&dockerfile_path, DOCKERFILE_TEMPLATE).map_err(|e| {
                AppError::InternalServerError(format!("Failed to write Go Dockerfile: {}", e))
            })?;
        }
        Ok(())
    }

    pub async fn create_dockerignore(&self) -> Result<(), AppError> {
        let ignore_path = self.project_path.join(".dockerignore");
        if !ignore_path.exists() {
            const DOCKERIGNORE_TEMPLATE: &str = include_str!("./assets/.dockerignore");
            fs::write(&ignore_path, DOCKERIGNORE_TEMPLATE).map_err(|e| {
                AppError::InternalServerError(format!("Failed to write Go .dockerignore: {}", e))
            })?;
        }
        Ok(())
    }
}
