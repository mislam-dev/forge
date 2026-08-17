use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterUserDto {
    #[validate(length(min = 1, message = "Username is required"))]
    pub username: String,

    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email must be a valid email address")
    )]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUserDto {
    #[validate(length(min = 1, message = "Email is required"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RefreshTokenDto {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ForgotPasswordDto {
    #[validate(length(min = 1, message = "Email is required"))]
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ResetPasswordDto {
    #[validate(length(min = 1, message = "Reset token is required"))]
    pub token: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub new_password: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct VerifyEmailDto {
    #[validate(length(min = 1, message = "Verification token is required"))]
    pub token: String,
}
