use super::handlers::UserRolesHandlers;
use crate::{app::state::AppState, modules::auth::token::JwtClaims};
use axum::{
    Router, middleware,
    routing::{get, post},
};

pub fn user_roles_router() -> Router<AppState> {
    Router::new()
        .route("/assign", post(UserRolesHandlers::assign))
        .route("/remove", post(UserRolesHandlers::remove))
        .route("/user/{id}", get(UserRolesHandlers::show))
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_roles_router_creation() {
        let _router = user_roles_router();
    }
}

