use axum::Router;

use crate::app::state::AppState;
use crate::modules::access_control::router::access_control_router;

pub fn app_router() -> Router<AppState> {
    Router::new().nest("access-control", access_control_router())
}
