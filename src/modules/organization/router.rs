use axum::Router;

use super::members::router::members_router;
use super::orgs::router::orgs_router;
use crate::app::state::AppState;

pub fn organization_router() -> Router<AppState> {
    Router::new()
        .merge(orgs_router())
        .merge(members_router())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_router_creation() {
        let _router = organization_router();
    }
}
