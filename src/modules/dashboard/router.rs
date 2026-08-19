use axum::{
    middleware,
    routing::get,
    Router,
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn dashboard_router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::get_system_dashboard))
        .route("/user", get(handlers::get_user_dashboard))
        .route("/org/{org_id}", get(handlers::get_org_dashboard))
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_router_creation() {
        let _router = dashboard_router();
    }
}
