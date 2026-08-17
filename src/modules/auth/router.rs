use axum::{
    Router,
    routing::{get, post},
};

use crate::app::state::AppState;

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(|| async { "Hello, World!" }))
        .route("/login", post(|| async { "Hello, World!" }))
        .route("/logout", post(|| async { "Hello, World!" }))
        .route("/me", get(|| async { "Hello, World!" }))
        .route("/forgot-password", post(|| async { "Hello, World!" }))
        .route("/verify-email", get(|| async { "Hello, World!" }))
}
