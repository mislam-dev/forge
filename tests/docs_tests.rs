use axum::{
    body::{to_bytes, Body},
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
        std::env::set_var("JWT_SECRET", "test_secret_key_12345");
        std::env::set_var("MASTER_ENCRYPTION_KEY", "0123456789abcdef0123456789abcdef");
    }
    AppConfig::load().expect("Test AppConfig must load successfully")
}

#[tokio::test]
async fn test_openapi_yaml_served_successfully() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/docs/openapi.yaml")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response.headers().get("content-type").unwrap().to_str().unwrap();
    assert!(content_type.contains("application/x-yaml") || content_type.contains("yaml"));

    let body_bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_str.contains("Forge Platform API"));
}

#[tokio::test]
async fn test_swagger_ui_served_at_docs() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/docs/")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // utoipa-swagger-ui returns OK (200) or SEE_OTHER (303) when loading the UI index
    assert!(response.status().is_success() || response.status().is_redirection());
}
