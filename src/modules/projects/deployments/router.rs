use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn deployments_router() -> Router<AppState> {
    let internal_routes = Router::new().route(
        "/internal/deployments/{deployment_id}/status",
        put(handlers::update_status_internal),
    );

    let protected_routes = Router::new()
        .route(
            "/{id}/deployments",
            post(handlers::trigger_deployment).get(handlers::list_deployments),
        )
        .route("/{id}/deployments/history", get(handlers::list_deployments))
        .route("/{id}/deployments/rollback", post(handlers::rollback))
        .route(
            "/{id}/deployments/{deployment_id}",
            get(handlers::get_deployment),
        )
        .route(
            "/{id}/deployments/{deployment_id}/redeploy",
            post(handlers::redeploy),
        )
        .route_layer(middleware::from_extractor::<JwtClaims>());

    Router::new().merge(internal_routes).merge(protected_routes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployments_router_creation() {
        let _router = deployments_router();
    }
}
