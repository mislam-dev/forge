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
            timestamp: chrono::Utc::now().to_string(),
            is_end,
        }
    }
}
