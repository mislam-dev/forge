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
    RefreshTokenPayload, ResetTokenData,
};
use crate::modules::users::dto::request::CreateUserDto;
use crate::modules::users::entities::sea_orm_active_enums::UserStatus;
use crate::modules::users::password::PasswordService;
use crate::modules::users::repository::UserRepository;
use crate::modules::users::service::UserService;
use crate::shared::error::AppError;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub async fn login(
        db: &DatabaseConnection,
        dto: LoginUserDto,
    ) -> Result<LoginResponseDto, AppError> {
        let user = UserService::find_by_email_with_password(db, &dto.email)
            .await
            .map_err(|_| AppError::BadRequest("Invalid Credentials".to_string()))?;
        let is_valid = PasswordService::verify(&user.password, &dto.password)
            .await
            .map_err(|_| AppError::BadRequest("Invalid credentials".to_string()))?;

        if !is_valid {
            return Err(AppError::BadRequest("Invalid credentials".to_string()));
        }

        // todo fetch user roles and permissions
        let access_token = AuthTokenService::access(JwtPayload {
            user_id: user.id,
            email: user.email.clone(),
            permissions: vec![],
            role: vec![],
        })
        .map_err(|_| {
            AppError::InternalServerError("Failed to generate access token".to_string())
        })?;

        let refresh_token = AuthTokenService::refresh(RefreshTokenPayload {
            user_id: user.id,
            email: user.email.clone(),
        })
        .map_err(|_| {
            AppError::InternalServerError("Failed to generate refresh token".to_string())
        })?;

        RefreshTokenRepository::save_refresh_token(
            db,
            RefreshToken {
                token: refresh_token.clone(),
                user_id: user.id,
                expires_at: 7, // todo update proper value
            },
        )
        .await
        .map_err(|_| {
            AppError::InternalServerError("Failed to store refresh token on the db".to_string())
        })?;

        // todo: store access_token with expiration in redis

        Ok(LoginResponseDto {
            access_token,
            refresh_token,
            expires_in: 3600,
        })
    }

    pub async fn register(
        db: &DatabaseConnection,
        dto: RegisterUserDto,
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
        // todo remove access_token from redis.
        Ok(())
    }

    pub async fn refresh(
        db: &DatabaseConnection,
        dto: RefreshTokenDto,
    ) -> Result<RefreshTokenResponseDto, AppError> {
        let token_decode = AuthTokenService::verify(&dto.refresh_token)
            .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

        let user = UserService::find_one(db, token_decode.sub)
            .await
            .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

        let access_token = AuthTokenService::access(JwtPayload {
            user_id: user.id,
            email: user.email.clone(),
            permissions: vec![],
            role: vec![],
        })
        .map_err(|_| {
            AppError::InternalServerError("Failed to generate access token".to_string())
        })?;

        let refresh_token = AuthTokenService::refresh(RefreshTokenPayload {
            user_id: user.id,
            email: user.email.clone(),
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
        dto: ForgotPasswordDto,
    ) -> Result<(), AppError> {
        // find user with email
        let user = UserService::find_by_email_with_password(db, &dto.email)
            .await
            .ok();
        if let Some(user) = user {
            let reset_token = PasswordResetTokenService::token(ResetTokenData { user_id: user.id })
                .map_err(|_| {
                    AppError::InternalServerError("Failed to create reset token".to_string())
                })?;

            println!("reset_token: {}", &reset_token);

            PasswordResetTokenRepository::create(
                db,
                PasswordResetToken {
                    user_id: user.id,
                    token: reset_token,
                    expires_at: 3600,
                },
            )
            .await
            .map_err(|_| {
                AppError::InternalServerError("Failed to create reset token".to_string())
            })?;

            // todo send email to the user
        }
        Ok(())
    }

    pub async fn reset_password(
        db: &DatabaseConnection,
        dto: ResetPasswordDto,
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
        dto: VerifyEmailDto,
    ) -> Result<(), AppError> {
        // Verify the JWT and expiration
        // todo: implement this
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

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    fn setup_jwt_secret() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test_secret_key_12345_67890_super_secret");
        }
    }

    #[tokio::test]
    async fn test_login_invalid_credentials() {
        let db = setup_mock_db();
        let dto = LoginUserDto {
            email: "nobody@example.com".to_string(),
            password: "Password123!".to_string(),
        };
        let result = AuthService::login(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_register_failed_creation() {
        let db = setup_mock_db();
        let dto = RegisterUserDto {
            username: "newuser".to_string(),
            email: "newuser@example.com".to_string(),
            password: "Password123!".to_string(),
        };
        let result = AuthService::register(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logout_success() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results([sea_orm::MockExecResult {
                last_insert_id: 0,
                rows_affected: 0,
            }])
            .into_connection();
        let user_id = Uuid::new_v4();
        let result = AuthService::logout(&db, user_id).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_refresh_invalid_token() {
        setup_jwt_secret();
        let db = setup_mock_db();
        let dto = RefreshTokenDto {
            refresh_token: "invalid.refresh.token".to_string(),
        };
        let result = AuthService::refresh(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_me_user_not_found() {
        let db = setup_mock_db();
        let claims = JwtClaims {
            sub: Uuid::new_v4(),
            email: "nobody@example.com".to_string(),
            role: vec![],
            permissions: vec![],
            iat: 100000,
            exp: 200000,
        };
        let result = AuthService::me(&db, claims).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_forgot_password_nonexistent_user() {
        setup_jwt_secret();
        let db = setup_mock_db();
        let dto = ForgotPasswordDto {
            email: "nonexistent@example.com".to_string(),
        };
        let result = AuthService::forgot_password(&db, dto).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_reset_password_mismatched_passwords() {
        let db = setup_mock_db();
        let dto = ResetPasswordDto {
            token: "valid_or_invalid_token".to_string(),
            new_password: "Password123!".to_string(),
            confirm_password: "Password321!".to_string(),
        };
        let result = AuthService::reset_password(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_password_invalid_token() {
        setup_jwt_secret();
        let db = setup_mock_db();
        let dto = ResetPasswordDto {
            token: "invalid.reset.token".to_string(),
            new_password: "Password123!".to_string(),
            confirm_password: "Password123!".to_string(),
        };
        let result = AuthService::reset_password(&db, dto).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_email_invalid_token() {
        setup_jwt_secret();
        let db = setup_mock_db();
        let dto = VerifyEmailDto {
            token: "invalid.verify.token".to_string(),
        };
        let result = AuthService::verify_email(&db, dto).await;
        assert!(result.is_err());
    }
}


