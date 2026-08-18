use super::handlers::{add, list, remove, show, update};
use crate::{app::state::AppState, modules::auth::token::JwtClaims};
use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};

pub fn user_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/{id}", get(show))
        .route("/", post(add))
        .route("/{id}", patch(update))
        .route("/{id}", delete(remove))
        .route_layer(middleware::from_extractor::<JwtClaims>())
}
