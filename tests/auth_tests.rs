use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use forge::{
    app::{app::create_app, state::AppState},
    config::AppConfig,
    modules::auth::token::{AuthTokenService, JwtPayload, PasswordResetToken, ResetTokenData},
};
use serde_json::json;
use tower::util::ServiceExt;
use uuid::Uuid;

fn setup_test_config() -> AppConfig {
    unsafe {
        std::env::set_var("DATABASE_URL", "postgres://user:pass@localhost:5432/testdb");
        std::env::set_var("JWT_SECRET", "test_secret_key_12345_67890_super_secret");
        std::env::set_var("MASTER_ENCRYPTION_KEY", "0123456789abcdef0123456789abcdef");
    }
    AppConfig::load().expect("Test AppConfig must load successfully")
}

// --- Unit Tests for Token Services ---

#[test]
fn test_auth_token_service_access_and_verify() {
    let _config = setup_test_config();
    let user_id = Uuid::new_v4();
    let email = "testuser@example.com".to_string();

    let payload = JwtPayload {
        user_id,
        email: email.clone(),
        role: vec!["User".to_string()],
        permissions: vec!["read:profile".to_string()],
    };

    let token = AuthTokenService::access(payload).expect("Access token creation failed");
    assert!(!token.is_empty());

    let claims = AuthTokenService::verify(&token).expect("Token verification failed");
    assert_eq!(claims.sub, user_id);
    assert_eq!(claims.email, email);
    assert_eq!(claims.role, vec!["User".to_string()]);
    assert_eq!(claims.permissions, vec!["read:profile".to_string()]);
}

#[test]
fn test_auth_token_service_refresh_token() {
    let _config = setup_test_config();
    let user_id = Uuid::new_v4();
    let email = "refreshuser@example.com".to_string();

    let payload = JwtPayload {
        user_id,
        email: email.clone(),
        role: vec![],
        permissions: vec![],
    };

    let token = AuthTokenService::refresh(payload).expect("Refresh token creation failed");
    assert!(!token.is_empty());

    let claims = AuthTokenService::verify(&token).expect("Refresh token verification failed");
    assert_eq!(claims.sub, user_id);
    assert_eq!(claims.email, email);
}

#[test]
fn test_auth_token_service_invalid_token() {
    let _config = setup_test_config();
    let result = AuthTokenService::verify("invalid.token.str");
    assert!(result.is_err());
}

#[test]
fn test_password_reset_token_create_and_verify() {
    let _config = setup_test_config();
    let user_id = Uuid::new_v4();

    let token = PasswordResetToken::token(ResetTokenData { user_id })
        .expect("Password reset token creation failed");
    assert!(!token.is_empty());

    let claims = PasswordResetToken::verify(&token).expect("Reset token verification failed");
    assert_eq!(claims.sub, user_id);
}

// --- Integration Tests for Auth Endpoints ---

#[tokio::test]
async fn test_me_endpoint_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/auth/me")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_me_endpoint_authorized_with_valid_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let user_id = Uuid::new_v4();
    let token = AuthTokenService::access(JwtPayload {
        user_id,
        email: "me@example.com".to_string(),
        role: vec![],
        permissions: vec![],
    })
    .unwrap();

    let req = Request::builder()
        .uri("/api/auth/me")
        .method("GET")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // Since MockDatabase has no user seeded, AuthService::me returns AppError::NotFound (404)
    // showing the request successfully passed JWT authorization middleware guard!
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_register_validation_failure() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    // Invalid payload: short password and empty username
    let invalid_payload = json!({
        "username": "",
        "email": "not-an-email",
        "password": "123"
    });

    let req = Request::builder()
        .uri("/api/auth/register")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_vec(&invalid_payload).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert!(json["message"].is_string());
}

#[tokio::test]
async fn test_login_validation_failure() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let invalid_payload = json!({
        "email": "",
        "password": ""
    });

    let req = Request::builder()
        .uri("/api/auth/login")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_vec(&invalid_payload).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_forgot_password_success() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let payload = json!({
        "email": "user@example.com"
    });

    let req = Request::builder()
        .uri("/api/auth/forgot-password")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(
        json["message"],
        "If the email exists, a reset link has been sent"
    );
}

#[tokio::test]
async fn test_reset_password_passwords_mismatch() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let user_id = Uuid::new_v4();
    let token = PasswordResetToken::token(ResetTokenData { user_id }).unwrap();

    let payload = json!({
        "token": token,
        "new_password": "NewPassword123!",
        "confirm_password": "DifferentPassword123!"
    });

    let req = Request::builder()
        .uri("/api/auth/reset-password")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json["message"], "Passwords do not match");
}

#[tokio::test]
async fn test_verify_email_invalid_token() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let payload = json!({
        "token": "invalid_verification_token"
    });

    let req = Request::builder()
        .uri("/api/auth/verify-email")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json["message"], "Invalid or expired verification token");
}
