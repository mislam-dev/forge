use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::modules::users::profile::entities::sea_orm_active_enums::Gender;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateUserProfileDto {
    #[validate(length(min = 1, message = "First name must not be empty"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "Last name must not be empty"))]
    pub last_name: String,
    pub phone: String,
    pub date_of_birth: NaiveDate,
    pub gender: Gender,
    pub image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateUserProfileDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub image: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_profile_dto_validation_success() {
        let dto = CreateUserProfileDto {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: "+1234567890".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
            gender: Gender::Male,
            image: Some("https://example.com/avatar.png".to_string()),
        };

        assert!(dto.validate().is_ok());
    }

    #[test]
    fn test_create_user_profile_dto_validation_failure() {
        let dto = CreateUserProfileDto {
            first_name: "".to_string(),
            last_name: "".to_string(),
            phone: "+1234567890".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
            gender: Gender::Female,
            image: None,
        };

        assert!(dto.validate().is_err());
    }

    #[test]
    fn test_update_user_profile_dto_serialization() {
        let dto = UpdateUserProfileDto {
            first_name: Some("Jane".to_string()),
            last_name: None,
            phone: None,
            date_of_birth: None,
            gender: Some(Gender::Other),
            image: None,
        };

        let json_str = serde_json::to_string(&dto).unwrap();
        assert!(json_str.contains("Jane"));
        assert!(json_str.contains("Other"));
    }
}
