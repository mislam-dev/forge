pub use super::assignments::service::ProjectAssignmentsService;
pub use super::environment_variables::service::ProjectEnvironmentVariablesService;
pub use super::permissions::service::ProjectPermissionsService;
pub use super::projects::service::ProjectsService;
pub use super::repositories::service::ProjectRepositoriesService;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_projects_service_reexports() {
        let _ = ProjectsService;
        let _ = ProjectRepositoriesService;
        let _ = ProjectEnvironmentVariablesService;
        let _ = ProjectAssignmentsService;
        let _ = ProjectPermissionsService;
    }
}
