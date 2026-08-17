use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserItemResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}
#[derive(Serialize)]
pub struct UserItemWithPassword {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}
