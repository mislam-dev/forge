pub mod dto;
pub mod entities;
pub mod handlers;
pub mod repository;
pub mod router;
pub mod service;

use entities::sea_orm_active_enums::ProjectEnvironmentVariablesEnvironment as EnvironmentEnum;
use std::{fmt, str::FromStr};

impl EnvironmentEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Staging => "staging",
            Self::Production => "production",
        }
    }
}

impl fmt::Display for EnvironmentEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for EnvironmentEnum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(Self::Development),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            other => Err(format!("unknown environment: {other}")),
        }
    }
}
