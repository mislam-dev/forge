use axum::Router;

use crate::{app::state::AppState, modules::access_control::roles::router::roles_router};

pub fn access_control_router() -> Router<AppState> {
    Router::new().nest("/roles", roles_router())
}
