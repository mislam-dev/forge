use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AssignUserRolesDto {
    pub user_id: Uuid,
    pub role_ids: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RemoveUserRolesDto {
    pub user_id: Uuid,
    pub role_ids: Vec<Uuid>,
}
