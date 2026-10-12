use super::package_json::PackageJson;
use crate::shared::error::{codes, AppError};
use std::fs::read_to_string;
use std::path::Path;

const FILES: [&str; 1] = ["package.json"];

pub struct Validation;

impl Validation {
    pub fn validate(project_source: &str) -> Result<(), AppError> {
        let path = Path::new(project_source);
        if !path.exists() {
            return Err(AppError::not_found(
                codes::projects::PRJ_FILE_001,
                "Project source not found",
            ));
        }

        for file in FILES {
            if !path.join(file).exists() {
                return Err(AppError::not_found(
                    codes::projects::PRJ_FILE_001,
                    format!("{} not found", file),
                ));
            }
        }

        let package_file = read_to_string(path.join("package.json")).map_err(|e| {
            AppError::not_found(
                codes::projects::PRJ_FILE_001,
                format!("Failed to read package.json: {}", e),
            )
        })?;

        let errors = PackageJson::new(package_file)?.validate()?;

        if !errors.is_empty() {
            return Err(AppError::bad_request(
                codes::deployments::WORKER_003,
                format!("Invalid package.json file:\n{}", errors.join("\n")),
            ));
        }

        Ok(())
    }
}
