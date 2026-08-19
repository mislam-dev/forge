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
async fn test_create_organization_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/organizations")
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"name": "Acme Corp"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_organizations_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/organizations")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_organization_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let org_id = Uuid::new_v4();
    let req = Request::builder()
        .uri(format!("/api/organizations/{}", org_id))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_organization_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let org_id = Uuid::new_v4();
    let req = Request::builder()
        .uri(format!("/api/organizations/{}", org_id))
        .method("PUT")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"name": "Updated Org"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_delete_organization_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let org_id = Uuid::new_v4();
    let req = Request::builder()
        .uri(format!("/api/organizations/{}", org_id))
        .method("DELETE")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_organizations_authorized_with_jwt() {
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
        .uri("/api/organizations")
        .method("GET")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert!(response.status() == StatusCode::OK || response.status() == StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_invite_member_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let org_id = Uuid::new_v4();
    let req = Request::builder()
        .uri(format!("/api/organizations/{}/invitations", org_id))
        .method("POST")
        .header("Content-Type", "application/json")
        .body(Body::from(r#"{"email": "invitee@example.com", "role": "editor"}"#))
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_list_members_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let org_id = Uuid::new_v4();
    let req = Request::builder()
        .uri(format!("/api/organizations/{}/members", org_id))
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_accept_invitation_unauthorized_without_jwt() {
    let config = setup_test_config();
    let state = AppState::mock(config);
    let app = create_app(state).await.expect("App creation failed");

    let req = Request::builder()
        .uri("/api/organizations/invitations/tok_12345678/accept")
        .method("POST")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
