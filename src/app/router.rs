use axum::Router;

use crate::app::state::AppState;
use crate::modules::access_control::router::access_control_router;
use crate::modules::organization::router::organization_router;
use crate::modules::projects::router::projects_router;
use crate::modules::teams::router::teams_router;

pub fn app_router() -> Router<AppState> {
    Router::new()
        .nest("access-control", access_control_router())
        .nest("organizations", organization_router())
        .nest("teams", teams_router())
        .nest("projects", projects_router())
}
