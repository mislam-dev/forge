use crate::app::state::AppState;
use crate::modules::auth::dto::request::{
    ForgotPasswordDto, LoginUserDto, RefreshTokenDto, RegisterUserDto, ResetPasswordDto,
    VerifyEmailDto,
};
use crate::modules::auth::dto::response::{
    LoginResponseDto, MeResponseDto, RefreshTokenResponseDto, RegisterResponseDto,
};
use crate::modules::auth::service::AuthService;
use crate::modules::auth::token::JwtClaims;
use crate::shared::{error::AppError, response::ApiResponse, validation::JsonValidate};
use axum::extract::State;
use axum::http::StatusCode;

pub async fn register(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<RegisterUserDto>,
) -> Result<ApiResponse<RegisterResponseDto>, AppError> {
    let user = AuthService::register(&state.db, payload).await?;
    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("User registered successfully".to_string())
        .body(Some(user)))
}

pub async fn login(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<LoginUserDto>,
) -> Result<ApiResponse<LoginResponseDto>, AppError> {
    let res = AuthService::login(&state.db, payload).await?;
    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Logged in successfully".to_string())
        .body(Some(res)))
}

pub async fn logout(
    State(state): State<AppState>,
    jwt_claims: JwtClaims,
) -> Result<ApiResponse<()>, AppError> {
    AuthService::logout(&state.db, jwt_claims.sub).await?;
    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Logged out successfully".to_string())
        .body(None))
}

pub async fn refresh(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<RefreshTokenDto>,
) -> Result<ApiResponse<RefreshTokenResponseDto>, AppError> {
    let res = AuthService::refresh(&state.db, payload).await?;
    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Token refreshed successfully".to_string())
        .body(Some(res)))
}

pub async fn me(
    State(state): State<AppState>,
    jwt_claims: JwtClaims,
) -> Result<ApiResponse<MeResponseDto>, AppError> {
    let res = AuthService::me(&state.db, jwt_claims).await?;
    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("User profile retrieved successfully".to_string())
        .body(Some(res)))
}

pub async fn forgot_password(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<ForgotPasswordDto>,
) -> Result<ApiResponse<()>, AppError> {
    AuthService::forgot_password(&state.db, payload).await?;
    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("If the email exists, a reset link has been sent".to_string()))
}

pub async fn reset_password(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<ResetPasswordDto>,
) -> Result<ApiResponse<()>, AppError> {
    AuthService::reset_password(&state.db, payload).await?;
    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Password reset successfully".to_string()))
}

pub async fn verify_email(
    State(state): State<AppState>,
    JsonValidate(payload): JsonValidate<VerifyEmailDto>,
) -> Result<ApiResponse<()>, AppError> {
    AuthService::verify_email(&state.db, payload).await?;
    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Email verified successfully".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use uuid::Uuid;

    fn setup_mock_state() -> AppState {
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://user:pass@localhost:5432/testdb");
            std::env::set_var("JWT_SECRET", "test_secret_key_12345_67890_super_secret");
            std::env::set_var("MASTER_ENCRYPTION_KEY", "0123456789abcdef0123456789abcdef");
        }
        let config = AppConfig::load().expect("Test AppConfig must load successfully");
        AppState::mock(config)
    }

    #[tokio::test]
    async fn test_register_handler() {
        let state = setup_mock_state();
        let payload = RegisterUserDto {
            username: "testuser".to_string(),
            email: "testuser@example.com".to_string(),
            password: "Password123!".to_string(),
        };
        let result = register(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_handler() {
        let state = setup_mock_state();
        let payload = LoginUserDto {
            email: "user@example.com".to_string(),
            password: "Password123!".to_string(),
        };
        let result = login(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_logout_handler() {
        let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_exec_results([sea_orm::MockExecResult {
                last_insert_id: 0,
                rows_affected: 0,
            }])
            .into_connection();
        let config = setup_mock_state().config.as_ref().clone();
        let state = AppState::from_parts(db, config);
        let jwt_claims = JwtClaims {
            sub: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            role: vec![],
            permissions: vec![],
            iat: 100000,
            exp: 200000,
        };
        let result = logout(State(state), jwt_claims).await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.msg, "Logged out successfully");
    }

    #[tokio::test]
    async fn test_refresh_handler() {
        let state = setup_mock_state();
        let payload = RefreshTokenDto {
            refresh_token: "invalid.refresh.token".to_string(),
        };
        let result = refresh(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_me_handler() {
        let state = setup_mock_state();
        let jwt_claims = JwtClaims {
            sub: Uuid::new_v4(),
            email: "user@example.com".to_string(),
            role: vec![],
            permissions: vec![],
            iat: 100000,
            exp: 200000,
        };
        let result = me(State(state), jwt_claims).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_forgot_password_handler() {
        let state = setup_mock_state();
        let payload = ForgotPasswordDto {
            email: "user@example.com".to_string(),
        };
        let result = forgot_password(State(state), JsonValidate(payload)).await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert_eq!(res.msg, "If the email exists, a reset link has been sent");
    }

    #[tokio::test]
    async fn test_reset_password_handler() {
        let state = setup_mock_state();
        let payload = ResetPasswordDto {
            token: "invalid_reset_token".to_string(),
            new_password: "NewPassword123!".to_string(),
            confirm_password: "NewPassword123!".to_string(),
        };
        let result = reset_password(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_email_handler() {
        let state = setup_mock_state();
        let payload = VerifyEmailDto {
            token: "invalid_verify_token".to_string(),
        };
        let result = verify_email(State(state), JsonValidate(payload)).await;
        assert!(result.is_err());
    }
}
