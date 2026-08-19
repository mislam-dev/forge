use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::users::profile::entities::sea_orm_active_enums::Gender;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub image: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_profile_response_serialization() {
        let id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let response = UserProfileResponse {
            id,
            user_id,
            first_name: Some("Alice".to_string()),
            last_name: Some("Smith".to_string()),
            phone: Some("+1987654321".to_string()),
            date_of_birth: Some(NaiveDate::from_ymd_opt(1995, 5, 20).unwrap()),
            gender: Some(Gender::Female),
            image: None,
            created_at: "2026-08-19T10:00:00Z".to_string(),
            updated_at: "2026-08-19T10:00:00Z".to_string(),
        };

        let json_value = serde_json::to_value(&response).unwrap();
        assert_eq!(json_value["first_name"], "Alice");
        assert_eq!(json_value["gender"], "Female");
    }
}
