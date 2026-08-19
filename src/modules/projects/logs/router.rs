use axum::{Router, middleware, routing::get};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn logs_router() -> Router<AppState> {
    Router::new()
        .route(
            "/{id}/deployments/{deployment_id}/logs",
            get(handlers::get_logs),
        )
        .route(
            "/{id}/deployments/{deployment_id}/logs/stream",
            get(handlers::stream_logs),
        )
        .route(
            "/{id}/deployments/{deployment_id}/logs/download",
            get(handlers::download_logs),
        )
        .route(
            "/{id}/deployments/{deployment_id}/logs/search",
            get(handlers::search_logs),
        )
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logs_router_creation() {
        let _router = logs_router();
    }
}
