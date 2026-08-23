use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleResponseDto {
    pub id: String,
    pub key: String,
    pub value: String,
    pub description: Option<String>,
}
