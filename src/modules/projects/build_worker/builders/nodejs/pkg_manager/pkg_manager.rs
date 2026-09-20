use crate::shared::error::AppError;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum PkgManger {
    PNPM,
    NPM,
    YARN,
}

impl PkgManger {
    pub fn detect(path: &Path) -> Result<Self, AppError> {
        if path.join("pnpm-lock.yaml").exists() {
            return Ok(PkgManger::PNPM);
        }
        if path.join("yarn.lock").exists() {
            return Ok(PkgManger::YARN);
        }
        if path.join("package-lock.json").exists() {
            return Ok(PkgManger::NPM);
        }

        // ? note: no package manager found , use npm as default;
        return Ok(PkgManger::NPM);
    }

    pub fn lock_file(&self) -> String {
        match self {
            PkgManger::PNPM => "pnpm-lock.yaml".to_string(),
            PkgManger::NPM => "package-lock.json".to_string(),
            PkgManger::YARN => "yarn.lock".to_string(),
        }
    }

    fn base_command_name(&self) -> String {
        match self {
            PkgManger::PNPM => "pnpm".to_string(),
            PkgManger::NPM => "npm".to_string(),
            PkgManger::YARN => "yarn".to_string(),
        }
    }

    pub fn install(&self) -> String {
        format!("{} install", self.base_command_name())
    }

    pub fn build(&self) -> String {
        format!("{} run build", self.base_command_name())
    }
    pub fn start(&self) -> String {
        format!("{} run start", self.base_command_name())
    }
    pub fn postinstall(&self) -> String {
        format!("{} run postinstall", self.base_command_name())
    }
    pub fn prune(&self) -> String {
        format!("{} run prune --if-present", self.base_command_name())
    }

    // todo: remove this
    pub fn has_ts_config(path: &PathBuf) -> bool {
        path.join("tsconfig.json").exists()
    }
}
