use super::handlers::RolesHandlers;
use crate::{app::state::AppState, modules::auth::token::JwtClaims};
use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};

pub fn roles_router() -> Router<AppState> {
    Router::new()
        .route("/", get(RolesHandlers::list))
        .route("/{id}", get(RolesHandlers::show))
        .route("/", post(RolesHandlers::add))
        .route("/{id}", patch(RolesHandlers::update))
        .route("/{id}", delete(RolesHandlers::remove))
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roles_router_creation() {
        let _router = roles_router();
    }
}
