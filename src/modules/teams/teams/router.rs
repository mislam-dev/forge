use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn teams_router() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_team).get(handlers::list_teams))
        .route(
            "/{id}",
            get(handlers::get_team)
                .put(handlers::update_team)
                .delete(handlers::delete_team),
        )
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_teams_router_creation() {
        let _router = teams_router();
    }
}
