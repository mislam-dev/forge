use crate::{modules::projects::build_worker::DeploymentPath, shared::error::AppError};
use std::fs::read_to_string;
use std::path::Path;

use super::super::super::traits::ProjectBuilder;
use super::docker::NodejsDockerWorker;
use super::package_json::PackageJson;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct NodeJsBuilder {
    project_path: DeploymentPath,
    env_vars: Vec<(String, String)>,
    project_id: String,
}

impl NodeJsBuilder {
    pub fn new(
        project_path: DeploymentPath,
        project_id: String,
        env_vars: Vec<(String, String)>,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            project_path,
            project_id,
            env_vars,
        }
    }
}

const FILES: [&str; 1] = ["package.json"];

#[async_trait]
impl ProjectBuilder for NodeJsBuilder {
    fn validate(&self) -> Result<(), AppError> {
        let path = Path::new(&self.project_path.source);
        if !path.exists() {
            return Err(AppError::NotFound("Project source not found".to_string()));
        }

        for file in FILES {
            if !path.join(file).exists() {
                return Err(AppError::NotFound(
                    format!("{} not found", file).to_string(),
                ));
            }
        }

        let package_file = read_to_string(path.join("package.json")).map_err(|e| {
            AppError::NotFound(format!("Failed to read package.json: {}", e.to_string()))
        })?;

        let errors = PackageJson::new(package_file)?.validate()?;

        if !errors.is_empty() {
            return Err(AppError::BadRequest(format!(
                "Invalid package.json file:\n{}",
                errors.join("\n")
            )));
        }

        Ok(())
    }

    async fn build(&self) -> Result<(), AppError> {
        let source_path = Path::new(&self.project_path.source);
        let _build_path = Path::new(&self.project_path.build);

        let result = NodejsDockerWorker::new(source_path.to_path_buf(), self.project_id.clone())
            .execute()
            .await?;

        Ok(())
    }

    fn deploy(&self) -> Result<(), AppError> {
        // todo: deploy the builded image with proper port mapping
        // todo: update deployment record

        Ok(())
    }

    fn cleanup(&self) -> Result<(), AppError> {
        // todo: remove builded image
        // todo: remove docker container
        Ok(())
    }

    fn health_check(&self) -> Result<(), AppError> {
        Ok(())
    }
}
