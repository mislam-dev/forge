use axum::{Router, middleware, routing::post};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn teams_router() -> Router<AppState> {
    Router::new()
        .route(
            "/{id}/teams",
            post(handlers::assign_team).get(handlers::list_teams),
        )
        .route(
            "/{id}/teams/{team_id}",
            axum::routing::delete(handlers::remove_team),
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
