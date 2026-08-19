use axum::{
    middleware,
    routing::{post, put},
    Router,
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn team_members_router() -> Router<AppState> {
    Router::new()
        .route("/{id}/members", post(handlers::add_member).get(handlers::list_members))
        .route(
            "/{id}/members/{user_id}",
            put(handlers::update_member).delete(handlers::remove_member),
        )
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_members_router_creation() {
        let _router = team_members_router();
    }
}
