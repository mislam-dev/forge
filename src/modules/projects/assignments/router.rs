use axum::Router;

use crate::app::state::AppState;

use super::members::router::members_router;
use super::teams::router::teams_router;

pub fn assignments_router() -> Router<AppState> {
    Router::new().merge(members_router()).merge(teams_router())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignments_router_creation() {
        let _router = assignments_router();
    }
}
