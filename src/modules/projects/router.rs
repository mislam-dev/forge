use axum::Router;

// use super::assignments::router::assignments_router;
// use super::deployments::router::deployments_router;
// use super::environment_variables::router::environment_variables_router;
// use super::logs::router::logs_router;
// use super::repositories::router::repositories_router;
use super::projects::router::projects_core_router;
use crate::app::state::AppState;

pub fn projects_router() -> Router<AppState> {
    Router::new().merge(projects_core_router())
    // .merge(repositories_router())
    // .merge(environment_variables_router())
    // .merge(assignments_router())
    // .merge(deployments_router())
    // .merge(logs_router())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_projects_router_creation() {
        let _router = projects_router();
    }
}
