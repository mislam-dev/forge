use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use forge::{
    app::{app::create_app, state::AppState},
    config::AppConfig,
    modules::auth::token::{AuthTokenService, JwtPayload},
};
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

#[tokio::test]
async fn test_get_profile_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let user_id = Uuid::new_v4();
    let req = Request::builder()
        .uri(format!("/api/users/{}/profile", user_id))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_profile_authorized_with_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let user_id = Uuid::new_v4();
    let token = AuthTokenService::access(JwtPayload {
        user_id,
        email: "user@example.com".to_string(),
        role: vec!["User".to_string()],
        permissions: vec![],
    })
    .unwrap();

    let req = Request::builder()
        .uri(format!("/api/users/{}/profile", user_id))
        .method("GET")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert!(response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND || response.status() == StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_update_profile_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let user_id = Uuid::new_v4();
    let req = Request::builder()
        .uri(format!("/api/users/{}/profile", user_id))
        .method("PUT")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"first_name": "Jane"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_profile_forbidden_for_other_user() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let user_id = Uuid::new_v4();
    let other_user_id = Uuid::new_v4();
    let token = AuthTokenService::access(JwtPayload {
        user_id,
        email: "user@example.com".to_string(),
        role: vec!["User".to_string()],
        permissions: vec![],
    })
    .unwrap();

    let req = Request::builder()
        .uri(format!("/api/users/{}/profile", other_user_id))
        .method("PUT")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"first_name": "Hack"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_profile_forbidden_for_non_admin() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let user_id = Uuid::new_v4();
    let token = AuthTokenService::access(JwtPayload {
        user_id,
        email: "user@example.com".to_string(),
        role: vec!["User".to_string()],
        permissions: vec![],
    })
    .unwrap();

    let req = Request::builder()
        .uri(format!("/api/users/{}/profile", user_id))
        .method("DELETE")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_delete_profile_authorized_for_admin() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let user_id = Uuid::new_v4();
    let token = AuthTokenService::access(JwtPayload {
        user_id,
        email: "admin@example.com".to_string(),
        role: vec!["Admin".to_string()],
        permissions: vec![],
    })
    .unwrap();

    let req = Request::builder()
        .uri(format!("/api/users/{}/profile", user_id))
        .method("DELETE")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert!(response.status() == StatusCode::OK || response.status() == StatusCode::NOT_FOUND || response.status() == StatusCode::INTERNAL_SERVER_ERROR);
}
