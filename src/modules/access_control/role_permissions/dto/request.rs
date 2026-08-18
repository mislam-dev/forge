use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AssignRolePermissionsDto {
    pub role_id: Uuid,
    pub permission_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RemoveRolePermissionsDto {
    pub role_id: Uuid,
    pub permission_ids: Vec<Uuid>,
}
