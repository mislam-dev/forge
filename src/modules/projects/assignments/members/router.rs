use axum::{Router, middleware, routing::post};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn members_router() -> Router<AppState> {
    Router::new()
        .route(
            "/{id}/members",
            post(handlers::assign_member).get(handlers::list_members),
        )
        .route(
            "/{id}/members/{user_id}",
            axum::routing::delete(handlers::remove_member),
        )
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignments_router_creation() {
        let _router = members_router();
    }
}
