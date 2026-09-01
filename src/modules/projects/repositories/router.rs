use axum::{Router, middleware, routing::post};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn repositories_router() -> Router<AppState> {
    Router::new()
        .route(
            "/{id}/repository",
            post(handlers::connect_repository)
                .get(handlers::get_repository)
                .patch(handlers::update_repository)
                .delete(handlers::disconnect_repository),
        )
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repositories_router_creation() {
        let _router = repositories_router();
    }
}
