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
use crate::shared::error::{AppError, codes};
use crate::shared::logger::logging_middleware;
use axum::{Router, http::StatusCode, middleware};
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

pub const API_V1_PATH: &str = "/api/v1";

pub async fn create_app(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let api_v1_router: Router<AppState> = Router::new()
        .nest("/auth", auth_router())
        .nest("/users", user_router())
        .nest("/access-control", access_control_router())
        .nest("/organizations", organization_router())
        .nest("/teams", teams_router())
        .nest("/projects", projects_router())
        .nest("/notifications", notifications_router())
        .nest("/dashboard", dashboard_router())
        .nest("/health", health_router())
        .merge(docs_router());

    let final_router: Router = Router::new()
        .nest(API_V1_PATH, api_v1_router)
        .nest("/health", health_router())
        .fallback(not_found_handler)
        .layer(cors_middleware())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(middleware::from_fn(logging_middleware))
        .with_state(app_state);

    Ok(final_router)
}

async fn not_found_handler() -> AppError {
    AppError::not_found(
        codes::system::SYS_004,
        "The requested resource was not found",
    )
}
