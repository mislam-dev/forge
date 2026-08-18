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
async fn test_roles_endpoint_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/access-control/roles")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_roles_endpoint_authorized_with_valid_jwt() {
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
        .uri("/api/access-control/roles")
        .method("GET")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    // On mock DB, listing roles returns 200 OK (with empty list or error on DB query depending on mock query)
    assert!(response.status() == StatusCode::OK || response.status() == StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_permissions_endpoint_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/access-control/permission")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_role_permissions_endpoint_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/access-control/roles/permissions/assign")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_user_roles_endpoint_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/access-control/role/assign")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_user_permissions_endpoint_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/access-control/users/permission/assign")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
