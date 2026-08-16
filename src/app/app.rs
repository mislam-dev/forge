use axum::{Router, http::StatusCode, middleware, routing::get};
use std::time::Duration;
use tower_http::timeout::TimeoutLayer;

use crate::app::{middleware::cors_middleware, state::AppState};
use crate::shared::logger::logging_middleware;
use crate::shared::response::ApiResponse;

pub async fn create_app(app_state: AppState) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
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
