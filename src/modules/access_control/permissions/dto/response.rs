use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionResponseDto {
    pub id: String,
    pub key: String,
    pub value: String,
    pub descriptions: Option<String>,
}
