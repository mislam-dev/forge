use crate::app::state::AppState;
use crate::modules::auth::handlers::{
    forgot_password, login, logout, me, refresh, register, reset_password, verify_email,
    verify_email_get,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/logout", post(logout))
        .route("/refresh", post(refresh))
        .route("/me", get(me))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
        .route("/verify-email", post(verify_email).get(verify_email_get))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_router_creation() {
        let _router = auth_router();
    }
}
