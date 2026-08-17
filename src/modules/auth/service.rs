use crate::modules::auth::dto::request::{
    ForgotPasswordDto, LoginUserDto, RefreshTokenDto, RegisterUserDto, ResetPasswordDto,
    VerifyEmailDto,
};
use crate::modules::auth::dto::response::{
    LoginResponseDto, MeResponseDto, RefreshTokenResponseDto, RegisterResponseDto,
};
use crate::modules::auth::repository::{
    PasswordResetToken, PasswordResetTokenRepository, RefreshToken, RefreshTokenRepository,
};
use crate::modules::auth::token::{
    AuthTokenService, JwtClaims, JwtPayload, PasswordResetToken as PasswordResetTokenService,
    ResetTokenData,
};
use crate::modules::users::dto::request::CreateUserDto;
use crate::modules::users::entities::sea_orm_active_enums::UserStatus;
use crate::modules::users::password::verify_password;
use crate::modules::users::repository::UserRepository;
use crate::modules::users::service::UserService;
use crate::shared::{error::AppError, validation::JsonValidate};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub async fn login(
        db: &DatabaseConnection,
        JsonValidate(dto): JsonValidate<LoginUserDto>,
    ) -> Result<LoginResponseDto, AppError> {
        let user = UserService::find_by_email_with_password(db, &dto.email)
            .await
            .map_err(|_| AppError::BadRequest("Invalid Credentials".to_string()))?;
        let is_valid = verify_password(&user.password, &dto.password)
            .await
            .map_err(|_| AppError::BadRequest("Invalid credentials".to_string()))?;

        if !is_valid {
            return Err(AppError::BadRequest("Invalid credentials".to_string()));
        }

        let access_token = AuthTokenService::access(JwtPayload {
            user_id: user.id,
            email: user.email.clone(),
            permissions: vec![],
            role: vec![],
        })
        .map_err(|_| {
            AppError::InternalServerError("Failed to generate access token".to_string())
        })?;

        let refresh_token = AuthTokenService::refresh(JwtPayload {
            user_id: user.id,
            email: user.email.clone(),
            permissions: vec![],
            role: vec![],
        })
        .map_err(|_| {
            AppError::InternalServerError("Failed to generate refresh token".to_string())
        })?;

        RefreshTokenRepository::save_refresh_token(
            db,
            RefreshToken {
                token: refresh_token.clone(),
                user_id: user.id,
                expires_at: 4,
            },
        )
        .await
        .map_err(|_| {
            AppError::InternalServerError("Failed to store refresh token on the db".to_string())
        })?;

        Ok(LoginResponseDto {
            access_token,
            refresh_token,
            expires_in: 3600,
        })
    }

    pub async fn register(
        db: &DatabaseConnection,
        JsonValidate(dto): JsonValidate<RegisterUserDto>,
    ) -> Result<RegisterResponseDto, AppError> {
        let user = UserService::create(
            db,
            CreateUserDto {
                username: dto.username,
                email: dto.email.clone(),
                password: dto.password,
            },
        )
        .await
        .map_err(|_| AppError::BadRequest("Failed to create user".to_string()))?;

        Ok(RegisterResponseDto {
            id: user.id,
            name: user.name,
            email: dto.email,
        })
    }

    pub async fn logout(db: &DatabaseConnection, user_id: Uuid) -> Result<(), AppError> {
        RefreshTokenRepository::remove_tokens_by_user_id(db, user_id)
            .await
            .map_err(|_| {
                AppError::InternalServerError(
                    "Failed to remove refresh tokens from the db".to_string(),
                )
            })?;
        Ok(())
    }

    pub async fn refresh(
        db: &DatabaseConnection,
        JsonValidate(dto): JsonValidate<RefreshTokenDto>,
    ) -> Result<RefreshTokenResponseDto, AppError> {
        let token_decode = AuthTokenService::verify(&dto.refresh_token)?;
        let user = UserService::find_one(db, token_decode.sub)
            .await
            .map_err(|_| AppError::NotFound("User not found".to_string()))?;

        let access_token = AuthTokenService::access(JwtPayload {
            user_id: user.id,
            email: user.email.clone(),
            permissions: vec![],
            role: vec![],
        })
        .map_err(|_| {
            AppError::InternalServerError("Failed to generate access token".to_string())
        })?;

        let refresh_token = AuthTokenService::refresh(JwtPayload {
            user_id: user.id,
            email: user.email.clone(),
            permissions: vec![],
            role: vec![],
        })
        .map_err(|_| {
            AppError::InternalServerError("Failed to generate refresh token".to_string())
        })?;

        RefreshTokenRepository::remove_tokens_by_user_id(db, user.id)
            .await
            .map_err(|_| {
                AppError::InternalServerError(
                    "Failed to remove refresh tokens from the db".to_string(),
                )
            })?;

        RefreshTokenRepository::save_refresh_token(
            db,
            RefreshToken {
                token: refresh_token.clone(),
                user_id: user.id,
                expires_at: 4,
            },
        )
        .await
        .map_err(|_| {
            AppError::InternalServerError("Failed to store refresh token on the db".to_string())
        })?;

        Ok(RefreshTokenResponseDto {
            access_token,
            refresh_token,
            expires_in: 3600,
        })
    }

    pub async fn me(
        db: &DatabaseConnection,
        jwt_claims: JwtClaims,
    ) -> Result<MeResponseDto, AppError> {
        let user = UserService::find_one(db, jwt_claims.sub)
            .await
            .map_err(|_| AppError::NotFound("User not found".to_string()))?;
        Ok(MeResponseDto {
            id: user.id,
            name: user.name,
            email: user.email,
        })
    }

    pub async fn forgot_password(
        db: &DatabaseConnection,
        JsonValidate(dto): JsonValidate<ForgotPasswordDto>,
    ) -> Result<(), AppError> {
        // find user with email
        let user = UserService::find_by_email_with_password(db, &dto.email)
            .await
            .ok();
        if let Some(user) = user {
            let password_token =
                PasswordResetTokenService::token(ResetTokenData { user_id: user.id }).map_err(
                    |_| AppError::InternalServerError("Failed to create reset token".to_string()),
                )?;
            PasswordResetTokenRepository::create(
                db,
                PasswordResetToken {
                    user_id: user.id,
                    token: password_token,
                    expires_at: 3600,
                },
            )
            .await
            .map_err(|_| {
                AppError::InternalServerError("Failed to create reset token".to_string())
            })?;
            // todo send email to the user
        }
        // create password reset token
        // send mail to user email with token
        Ok(())
    }

    pub async fn reset_password(
        db: &DatabaseConnection,
        JsonValidate(dto): JsonValidate<ResetPasswordDto>,
    ) -> Result<(), AppError> {
        // Validate passwords match before doing any DB work
        if dto.new_password != dto.confirm_password {
            return Err(AppError::BadRequest("Passwords do not match".to_string()));
        }

        // Verify token and check expiration (PasswordResetToken::verify handles exp check)
        let claims = PasswordResetTokenService::verify(&dto.token)
            .map_err(|_| AppError::BadRequest("Invalid or expired reset token".to_string()))?;

        // Confirm the token exists in the DB (single-use guard)
        let _db_token = PasswordResetTokenRepository::find_one(db, &dto.token)
            .await?
            .ok_or_else(|| {
                AppError::BadRequest("Reset token not found or already used".to_string())
            })?;

        // Hash the new password and persist it
        UserRepository::update_password(db, claims.sub, &dto.new_password)
            .await
            .map_err(|_| AppError::InternalServerError("Failed to update password".to_string()))?;

        // Invalidate the token so it cannot be reused
        PasswordResetTokenRepository::remove_by_user_id(db, claims.sub)
            .await
            .map_err(|_| {
                AppError::InternalServerError("Failed to remove reset token".to_string())
            })?;

        Ok(())
    }

    pub async fn verify_email(
        db: &DatabaseConnection,
        JsonValidate(dto): JsonValidate<VerifyEmailDto>,
    ) -> Result<(), AppError> {
        // Verify the JWT and expiration
        let claims = PasswordResetTokenService::verify(&dto.token).map_err(|_| {
            AppError::BadRequest("Invalid or expired verification token".to_string())
        })?;

        // Confirm token exists in DB
        let _db_token = PasswordResetTokenRepository::find_one(db, &dto.token)
            .await?
            .ok_or_else(|| {
                AppError::BadRequest("Verification token not found or already used".to_string())
            })?;

        // Mark user as active and email as verified
        UserRepository::update_status(db, claims.sub, UserStatus::Active)
            .await
            .map_err(|_| {
                AppError::InternalServerError("Failed to update user status".to_string())
            })?;

        // Consume the token
        PasswordResetTokenRepository::remove_by_user_id(db, claims.sub)
            .await
            .map_err(|_| {
                AppError::InternalServerError("Failed to remove verification token".to_string())
            })?;

        Ok(())
    }
}
