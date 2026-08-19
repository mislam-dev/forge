use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn members_router() -> Router<AppState> {
    Router::new()
        .route("/{id}/invitations", post(handlers::invite))
        .route("/{id}/invitations", get(handlers::list_invitations))
        .route(
            "/invitations/{token}/accept",
            post(handlers::accept_invitation),
        )
        .route("/{id}/members", get(handlers::list_members))
        .route("/{id}/members/{user_id}", put(handlers::update_member))
        .route("/{id}/members/{user_id}", delete(handlers::remove_member))
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_members_router_creation() {
        let _router = members_router();
    }
}
