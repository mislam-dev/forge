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
        role: vec!["User".to_string()],
        permissions: vec![],
    };
    AuthTokenService::access(payload).unwrap()
}

#[tokio::test]
async fn test_create_project_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/projects")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(format!(
            r#"{{"organization_id": "{}", "name": "Forge API", "project_type": "repo", "runtime": "Rust"}}"#,
            Uuid::new_v4()
        )))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_projects_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/projects")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_project_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/projects/{}", Uuid::new_v4()))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_connect_repository_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/projects/{}/repository", Uuid::new_v4()))
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(
            r#"{"repository_url": "https://github.com/forge/core"}"#,
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
        .uri(format!("/api/projects/{}/repository", Uuid::new_v4()))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_env_var_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/projects/{}/env-vars", Uuid::new_v4()))
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(
            r#"{"environment": "Production", "key": "DATABASE_URL", "value": "postgres://localhost/db"}"#,
        ))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_env_vars_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/projects/{}/env-vars", Uuid::new_v4()))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_bulk_create_env_vars_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/projects/{}/env-vars/bulk", Uuid::new_v4()))
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(
            r#"{"environment": "Development", "vars": [{"key": "PORT", "value": "3000"}]}"#,
        ))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_assign_project_member_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/projects/{}/members", Uuid::new_v4()))
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(format!(
            r#"{{"user_id": "{}", "role": "developer"}}"#,
            Uuid::new_v4()
        )))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_assign_project_team_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri(format!("/api/projects/{}/teams", Uuid::new_v4()))
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(format!(
            r#"{{"team_id": "{}"}}"#,
            Uuid::new_v4()
        )))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
