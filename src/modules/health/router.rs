use axum::{
    middleware,
    routing::get,
    Router,
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn health_router() -> Router<AppState> {
    let public_routes = Router::new().route("/", get(handlers::check_health));

    let protected_routes = Router::new()
        .route("/details", get(handlers::check_health_details))
        .route_layer(middleware::from_extractor::<JwtClaims>());

    Router::new().merge(public_routes).merge(protected_routes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_router_creation() {
        let _router = health_router();
    }
}
