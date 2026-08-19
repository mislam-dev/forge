use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn projects_core_router() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_project).get(handlers::list_projects))
        .route(
            "/{id}",
            get(handlers::get_project)
                .put(handlers::update_project)
                .delete(handlers::delete_project),
        )
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projects_core_router_creation() {
        let _router = projects_core_router();
    }
}
