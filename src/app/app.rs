use crate::app::{middleware::cors_middleware, state::AppState};
use crate::modules::access_control::router::access_control_router;
use crate::modules::auth::router::auth_router;
use crate::modules::dashboard::router::dashboard_router;
use crate::modules::docs::router::docs_router;
use crate::modules::health::router::health_router;
use crate::modules::notifications::router::notifications_router;
use crate::modules::organization::router::organization_router;
use crate::modules::projects::router::projects_router;
use crate::modules::teams::router::teams_router;
use crate::modules::users::router::user_router;
use crate::shared::logger::logging_middleware;
use crate::shared::response::ApiResponse;
use axum::{Router, http::StatusCode, middleware, routing::get};
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

pub async fn create_app(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api/auth", auth_router())
        .nest("/api/users", user_router())
        .nest("/api/access-control", access_control_router())
        .nest("/api/organizations", organization_router())
        .nest("/api/teams", teams_router())
        .nest("/api/projects", projects_router())
        .nest("/api/notifications", notifications_router())
        .nest("/api/dashboard", dashboard_router())
        .nest("/api/health", health_router())
        .nest("/health", health_router())
        .merge(docs_router())
        .fallback(not_found_handler)
        .layer(cors_middleware())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(middleware::from_fn(logging_middleware))
        .with_state(app_state);

    Ok(router)
}

async fn not_found_handler() -> ApiResponse<()> {
    ApiResponse::new()
        .status(StatusCode::NOT_FOUND)
        .message("you requested resource not found".to_string())
}
