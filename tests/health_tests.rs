use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use forge::{
    app::{app::create_app, state::AppState},
    config::AppConfig,
};
use tower::util::ServiceExt;

fn setup_test_config() -> AppConfig {
    unsafe {
        std::env::set_var("DATABASE_URL", "postgres://user:pass@localhost:5432/testdb");
        std::env::set_var("JWT_SECRET", "test_secret_key_12345_67890_super_secret");
        std::env::set_var("MASTER_ENCRYPTION_KEY", "0123456789abcdef0123456789abcdef");
    }
    AppConfig::load().expect("Test AppConfig must load successfully")
}

#[tokio::test]
async fn test_public_health_probe_accessible_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/health/")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // Since AppState::mock uses a disconnected MockDatabase, DB probe fails returning status critical -> 503 SERVICE_UNAVAILABLE
    // The key test assertion is that it does NOT return 401 Unauthorized because /health is public!
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_api_health_probe_accessible_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/health/")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_ne!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_health_details_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/health/details")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_health_liveness_probe() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/health/live")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["status"], "healthy");
    assert_eq!(body["service"], "forge-platform");
    assert_eq!(body["version"], "1.0.0");
    assert_eq!(body["environment"], "production");
    assert!(body["timestamp"].as_str().is_some());
}

#[tokio::test]
async fn test_health_readiness_probe() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/health/ready")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["status"], "ready");
    assert_eq!(body["service"], "forge-platform");
    assert_eq!(body["checks"]["database"]["status"], "healthy");
    assert_eq!(body["checks"]["database"]["latency_ms"], 4);
    assert_eq!(body["checks"]["job_queue"]["status"], "healthy");
    assert_eq!(body["checks"]["job_queue"]["latency_ms"], 2);
    assert_eq!(body["checks"]["container_runtime"]["status"], "healthy");
    assert_eq!(body["checks"]["container_runtime"]["latency_ms"], 8);
}

#[tokio::test]
async fn test_health_deep_check_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/health/deep")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_health_deep_check_forbidden_for_non_admin() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    use forge::modules::auth::token::{AuthTokenService, JwtPayload};
    use uuid::Uuid;

    let token = AuthTokenService::access(JwtPayload {
        user_id: Uuid::new_v4(),
        email: "dev@example.com".to_string(),
        roles: vec!["developer".to_string()],
        permissions: vec![],
    })
    .unwrap();

    let req = Request::builder()
        .uri("/health/deep")
        .method("GET")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_health_deep_check_authorized_for_admin() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    use forge::modules::auth::token::{AuthTokenService, JwtPayload};
    use uuid::Uuid;

    let token = AuthTokenService::access(JwtPayload {
        user_id: Uuid::new_v4(),
        email: "admin@example.com".to_string(),
        roles: vec!["system_admin".to_string()],
        permissions: vec![],
    })
    .unwrap();

    let req = Request::builder()
        .uri("/health/deep?timeout_ms=5000")
        .method("GET")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["status"], "healthy");
    assert_eq!(body["service"], "forge-platform");
    assert_eq!(body["version"], "1.0.0");
    assert_eq!(body["uptime_seconds"], 864000);
    assert!(body["timestamp"].as_str().is_some());
}

