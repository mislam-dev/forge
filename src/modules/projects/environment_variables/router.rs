use axum::{
    middleware,
    routing::{post, put},
    Router,
};

use super::handlers;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;

pub fn environment_variables_router() -> Router<AppState> {
    Router::new()
        .route(
            "/{id}/env-vars",
            post(handlers::create_env_var).get(handlers::list_env_vars),
        )
        .route("/{id}/env-vars/bulk", post(handlers::bulk_create_env_vars))
        .route(
            "/{id}/env-vars/{env_id}",
            put(handlers::update_env_var).delete(handlers::delete_env_var),
        )
        .route_layer(middleware::from_extractor::<JwtClaims>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_variables_router_creation() {
        let _router = environment_variables_router();
    }
}
