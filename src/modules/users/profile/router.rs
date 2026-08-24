use axum::{
    Router, middleware,
    routing::{get, put},
};

use super::handlers::{get_profile, update_profile};
use crate::{app::state::AppState, modules::auth::token::JwtClaims};

pub fn profile_router() -> Router<AppState> {
    Router::new()
        .route("/{id}/profile", get(get_profile))
        .route("/{id}/profile", put(update_profile))
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_router_creation() {
        let _router = profile_router();
    }
}
