use axum::{
    body::{to_bytes, Body},
    http::{HeaderValue, Request, StatusCode},
};
use forge::{
    app::{app::create_app, state::AppState},
    config::AppConfig,
};
use tower::util::ServiceExt;

fn setup_test_config() -> AppConfig {
    unsafe {
        std::env::set_var("DATABASE_URL", "postgres://user:pass@localhost:5432/testdb");
        std::env::set_var("JWT_SECRET", "test_secret_key_12345");
        std::env::set_var("MASTER_ENCRYPTION_KEY", "0123456789abcdef0123456789abcdef");
    }
    AppConfig::load().expect("Test AppConfig must load successfully")
}

#[tokio::test]
async fn test_root_route_returns_hello_world() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert_eq!(body_str, "Hello, World!");
}

#[tokio::test]
async fn test_global_404_handler_returns_json() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/unknown/route/path")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json["message"], "you requested resource not found");
}

#[tokio::test]
async fn test_request_id_middleware_generated() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let request_id = response.headers().get("x-request-id");
    assert!(request_id.is_some());
    assert!(!request_id.unwrap().to_str().unwrap().is_empty());
}

#[tokio::test]
async fn test_request_id_middleware_preserved() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let custom_id = "test-request-id-12345";
    let req = Request::builder()
        .uri("/")
        .method("GET")
        .header("x-request-id", custom_id)
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let request_id = response.headers().get("x-request-id").unwrap();
    assert_eq!(request_id.to_str().unwrap(), custom_id);
}

#[tokio::test]
async fn test_cors_preflight_response() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/")
        .method("OPTIONS")
        .header("Origin", "http://localhost:3000")
        .header("Access-Control-Request-Method", "GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.headers().get("access-control-allow-origin"),
        Some(&HeaderValue::from_static("*"))
    );
}
