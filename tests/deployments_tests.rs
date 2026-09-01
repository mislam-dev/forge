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

#[allow(dead_code)]
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
async fn test_trigger_deployment_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/v1/projects/{}/deployments", Uuid::new_v4()))
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"branch": "main"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_deployments_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/v1/projects/{}/deployments", Uuid::new_v4()))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_deployment_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!(
            "/api/v1/projects/{}/deployments/{}",
            Uuid::new_v4(),
            Uuid::new_v4()
        ))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_redeploy_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!(
            "/api/v1/projects/{}/deployments/{}/redeploy",
            Uuid::new_v4(),
            Uuid::new_v4()
        ))
        .method("POST")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_rollback_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!(
            "/api/v1/projects/{}/deployments/rollback",
            Uuid::new_v4()
        ))
        .method("POST")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_status_internal_unauthorized_service_token() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!(
            "/api/v1/projects/internal/deployments/{}/status",
            Uuid::new_v4()
        ))
        .method("PUT")
        .header("Content-Type", "application/json")
        .header("x-service-token", "invalid_token")
        .body(Body::from(r#"{"status": "Building"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_status_internal_invalid_status_name() {
    let config = setup_test_config();
    let master_key = config.secrets.master_encryption_key.clone();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!(
            "/api/v1/projects/internal/deployments/{}/status",
            Uuid::new_v4()
        ))
        .method("PUT")
        .header("Content-Type", "application/json")
        .header("x-service-token", master_key)
        .body(Body::from(r#"{"status": "UnknownInvalidStatus"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_update_status_internal_validation_failure_empty_status() {
    let config = setup_test_config();
    let master_key = config.secrets.master_encryption_key.clone();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!(
            "/api/v1/projects/internal/deployments/{}/status",
            Uuid::new_v4()
        ))
        .method("PUT")
        .header("Content-Type", "application/json")
        .header("x-service-token", master_key)
        .body(Body::from(r#"{"status": ""}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
