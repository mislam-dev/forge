use super::handlers::UserPermissionsHandlers;
use crate::{app::state::AppState, modules::auth::token::JwtClaims};
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub fn user_permissions_router() -> Router<AppState> {
    Router::new()
        .route("/permission/assign", post(UserPermissionsHandlers::assign))
        .route("/permission/remove", post(UserPermissionsHandlers::remove))
        .route("/permissions/{id}", get(UserPermissionsHandlers::show))
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_permissions_router_creation() {
        let _router = user_permissions_router();
    }
}
