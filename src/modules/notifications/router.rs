use axum::{
    middleware,
    routing::{get, patch, post},
    Router,
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn notifications_router() -> Router<AppState> {
    let internal_routes = Router::new().route(
        "/internal",
        post(handlers::create_notification_internal),
    );

    let protected_routes = Router::new()
        .route("/", get(handlers::list_notifications))
        .route("/unread-count", get(handlers::get_unread_count))
        .route("/{id}/read", patch(handlers::mark_as_read))
        .route("/read-all", patch(handlers::mark_all_as_read))
        .route_layer(middleware::from_extractor::<JwtClaims>());

    Router::new().merge(internal_routes).merge(protected_routes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifications_router_creation() {
        let _router = notifications_router();
    }
}
