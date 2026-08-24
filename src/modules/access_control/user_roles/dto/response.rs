use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRoleResponse {
    pub role_id: Uuid,
    pub user_id: Uuid,
}
