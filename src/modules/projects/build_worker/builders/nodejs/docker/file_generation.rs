use super::super::pkg_manager::PkgManger;
use crate::shared::error::AppError;
use std::path::PathBuf;

use std::fs;

pub struct NodejsDockerFileGeneration {
    project_path: PathBuf,
}

impl NodejsDockerFileGeneration {
    pub fn new(project_path: PathBuf) -> Self {
        Self { project_path }
    }

    pub async fn create_dockerfile(&self) -> Result<(), AppError> {
        const DOCKERFILE_TEMPLATE: &str = include_str!("./assets/Dockerfile");

        let mut docker_file_content = DOCKERFILE_TEMPLATE.to_string();
        let pkg_manager = PkgManger::detect(&self.project_path)?;

        let mut replaceable: Vec<(String, String)> = vec![
            (
                String::from("{pkg_installer}"),
                String::from(pkg_manager.install_command()),
            ),
            (
                String::from("{pkg_postinstaller}"),
                String::from(pkg_manager.postinstall_command()),
            ),
            (
                String::from("{pkg_builder}"),
                String::from(pkg_manager.build_command()),
            ),
            (
                String::from("{pkg_prune}"),
                String::from(pkg_manager.prune_command()),
            ),
            (
                String::from("{lock_file}"),
                String::from(pkg_manager.get_lock_file_name()),
            ),
            (
                String::from("{exposed_port}"),
                String::from("3000".to_string()),
            ),
            (
                String::from("{pkg_start}"),
                String::from(format!(
                    "{:?}",
                    pkg_manager
                        .start_command()
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                )),
            ),
        ];

        if PkgManger::has_ts_config(&self.project_path.to_path_buf()) {
            replaceable.push((
                String::from("{ts_config}"),
                String::from("tsconfig.json".to_string()),
            ));
        }

        for (key, value) in replaceable {
            docker_file_content = docker_file_content.replace(&key, &value);
        }

        fs::write(self.project_path.join("Dockerfile"), docker_file_content).map_err(|e| {
            AppError::InternalServerError(format!("Failed to write Dockerfile: {}", e))
        })?;
        Ok(())
    }
    pub async fn create_dockerignore(&self) -> Result<(), AppError> {
        const DOCKERIGNORE_TEMPLATE: &str = include_str!("./assets/.dockerignore");

        fs::write(
            self.project_path.join(".dockerignore"),
            DOCKERIGNORE_TEMPLATE.to_string(),
        )
        .map_err(|e| {
            AppError::InternalServerError(format!("Failed to copy .dockerignore: {}", e))
        })?;

        Ok(())
    }
}
