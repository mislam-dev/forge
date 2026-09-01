use axum::{
    Router, middleware,
    routing::{get, post, put},
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn deployments_router() -> Router<AppState> {
    let public_deployments_router = Router::new()
        .route(
            "/{id}/deployments",
            post(handlers::trigger_deployment).get(handlers::list_deployments),
        )
        .route(
            "/{id}/deployments/{deployment_id}",
            get(handlers::get_deployment),
        )
        .route(
            "/{id}/deployments/{deployment_id}/redeploy",
            post(handlers::redeploy),
        )
        .route(
            "/{id}/deployments/rollback",
            post(handlers::rollback),
        )
        .route_layer(middleware::from_extractor::<JwtClaims>());

    let internal_deployments_router = Router::new().route(
        "/internal/deployments/{deployment_id}/status",
        put(handlers::update_status_internal),
    );

    Router::new()
        .merge(public_deployments_router)
        .merge(internal_deployments_router)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployments_router_creation() {
        let _router = deployments_router();
    }
}
