use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn orgs_router() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create))
        .route("/", get(handlers::list))
        .route("/{id}", get(handlers::show))
        .route("/{id}", patch(handlers::update))
        .route("/{id}", delete(handlers::remove))
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orgs_router_creation() {
        let _router = orgs_router();
    }
}
