use super::handlers::RolePermissionsHandlers;
use crate::{app::state::AppState, modules::auth::token::JwtClaims};
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub fn role_permissions_router() -> Router<AppState> {
    Router::new()
        .route("/assign", post(RolePermissionsHandlers::assign))
        .route("/remove", post(RolePermissionsHandlers::remove))
        .route("/{id}", get(RolePermissionsHandlers::show))
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_permissions_router_creation() {
        let _router = role_permissions_router();
    }
}
