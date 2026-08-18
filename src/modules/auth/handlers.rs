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
