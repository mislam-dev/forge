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

fn generate_test_token(user_id: Uuid) -> String {
    let payload = JwtPayload {
        user_id,
        email: "test@forge.dev".to_string(),
        roles: vec!["User".to_string()],
        permissions: vec![],
    };
    AuthTokenService::access(payload).unwrap()
}

#[tokio::test]
async fn test_connect_repository_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/v1/projects/{}/repository", Uuid::new_v4()))
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(
            r#"{"repository_url": "https://github.com/test/repo.git", "auth_type": "pat", "access_token": "secret"}"#,
        ))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_repository_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/v1/projects/{}/repository", Uuid::new_v4()))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_repository_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/v1/projects/{}/repository", Uuid::new_v4()))
        .method("PUT")
        .header("Content-Type", "application/json")
        .body(Body::from(
            r#"{"repository_url": "https://github.com/test/repo-updated.git"}"#,
        ))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_disconnect_repository_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/v1/projects/{}/repository", Uuid::new_v4()))
        .method("DELETE")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_connect_repository_validation_failure() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");
    let user_id = Uuid::new_v4();
    let token = generate_test_token(user_id);

    let req = Request::builder()
        .uri(format!("/api/v1/projects/{}/repository", Uuid::new_v4()))
        .method("POST")
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"repository_url": "git"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
