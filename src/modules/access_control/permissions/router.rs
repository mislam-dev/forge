use super::handlers::PermissionsHandlers;
use crate::{app::state::AppState, modules::auth::token::JwtClaims};
use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};

pub fn permissions_router() -> Router<AppState> {
    Router::new()
        .route("/", get(PermissionsHandlers::list))
        .route("/{id}", get(PermissionsHandlers::show))
        .route("/", post(PermissionsHandlers::add))
        .route("/{id}", patch(PermissionsHandlers::update))
        .route("/{id}", delete(PermissionsHandlers::remove))
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permissions_router_creation() {
        let _router = permissions_router();
    }
}
