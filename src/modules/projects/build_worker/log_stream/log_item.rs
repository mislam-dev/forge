use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Info,
    Error,
    Warn,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogItem {
    pub step: String,
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    pub deployment_id: String,
    pub is_end: bool,
}

impl LogItem {
    pub fn new(
        step: &str,
        level: LogLevel,
        message: String,
        deployment_id: String,
        is_end: bool,
    ) -> Self {
        Self {
            step: step.to_string(),
            level,
            message,
            deployment_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
            is_end,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "error"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
        }
    }
}

impl Display for LogItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.timestamp, self.level, self.step, self.message
        )
    }
}
