use crate::shared::error::AppError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PackageJson {
    name: Option<String>,
    version: Option<String>,
    scripts: Option<HashMap<String, String>>,
    dependencies: Option<HashMap<String, String>>,

    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<HashMap<String, String>>,
}
impl PackageJson {
    pub fn new(package_str: String) -> Result<Self, AppError> {
        let package_json: PackageJson = serde_json::from_str(&package_str)
            .map_err(|e| AppError::BadRequest(format!("Invalid package.json file: `{}`", e)))?;
        Ok(Self {
            name: package_json.name,
            version: package_json.version,
            scripts: package_json.scripts,
            dependencies: package_json.dependencies,
            dev_dependencies: package_json.dev_dependencies,
        })
    }

    pub fn validate(&self) -> Result<Vec<String>, AppError> {
        let mut errors: Vec<String> = vec![];

        self.check_scripts(&mut errors);
        self.check_dependencies(&mut errors);

        Ok(errors)
    }

    fn check_scripts(&self, errors: &mut Vec<String>) {
        if self.scripts.is_none() {
            errors.push("package.json file does not have scripts section".to_string());
            return;
        }
        let scripts = self.scripts.as_ref().unwrap();
        if !scripts.contains_key("build") {
            errors.push("package.json file does not have 'build' script".to_string());
            return;
        }
        if !scripts.contains_key("start") {
            errors.push("package.json file does not have 'start' script".to_string());
            return;
        }
        let malicious_script: [String; 3] = [
            "wget".to_string(),
            "curl".to_string(),
            "powershell".to_string(),
        ];

        for (script_name, script_content) in scripts.iter() {
            for malicious_cmd in &malicious_script {
                if script_content.contains(malicious_cmd) {
                    errors.push(format!(
                        "package.json file has suspicious script: {}: {}",
                        script_name, script_content
                    ));
                    return;
                }
            }
        }
    }
    fn check_dependencies(&self, errors: &mut Vec<String>) {
        if self.dependencies.is_none() && self.dev_dependencies.is_none() {
            return;
        }

        let malicious_pkgs: Vec<String> = vec![];

        if self.dependencies.is_some() {
            let deps = self.dependencies.as_ref().unwrap();
            for (name, _version) in deps.iter() {
                if malicious_pkgs.contains(name) {
                    errors.push(format!(
                        "package.json file has malicious dependency: {}",
                        name
                    ));
                    return;
                }
            }
        }
        if self.dev_dependencies.is_some() {
            let deps = self.dev_dependencies.as_ref().unwrap();
            for (name, _version) in deps.iter() {
                if malicious_pkgs.contains(name) {
                    errors.push(format!(
                        "package.json file has malicious dev dependency: {}",
                        name
                    ));
                    return;
                }
            }
        }
    }
}
