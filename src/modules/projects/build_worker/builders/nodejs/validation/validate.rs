use super::package_json::PackageJson;
use crate::shared::error::AppError;
use std::fs::read_to_string;
use std::path::Path;

const FILES: [&str; 1] = ["package.json"];

pub struct Validation;

impl Validation {
    pub fn validate(project_source: &str) -> Result<(), AppError> {
        let path = Path::new(project_source);
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
}
