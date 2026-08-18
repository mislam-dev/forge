use axum::Router;

use super::permissions::router::permissions_router;
use super::role_permissions::router::role_permissions_router;
use super::roles::router::roles_router;
use super::user_permissions::router::user_permissions_router;
use super::user_roles::router::user_roles_router;
use crate::app::state::AppState;

pub fn access_control_router() -> Router<AppState> {
    Router::new()
        .nest("/roles", roles_router())
        .nest("/permission", permissions_router())
        .nest("/roles/permissions", role_permissions_router())
        .nest("/role", user_roles_router())
        .nest("/users", user_permissions_router())
}
