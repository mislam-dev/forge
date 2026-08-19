use axum::Router;

use super::members::router::team_members_router;
use super::teams::router::teams_router as core_teams_router;
use crate::app::state::AppState;

pub fn teams_router() -> Router<AppState> {
    Router::new()
        .merge(core_teams_router())
        .merge(team_members_router())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_teams_router_creation() {
        let _router = teams_router();
    }
}
