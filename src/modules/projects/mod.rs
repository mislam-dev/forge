pub mod assignments;
pub mod environment_variables;
pub mod permissions;
pub mod projects;
pub mod repositories;
pub mod router;
pub mod service;

pub use router::projects_router;
pub use service::{
    ProjectAssignmentsService, ProjectEnvironmentVariablesService, ProjectPermissionsService,
    ProjectRepositoriesService, ProjectsService,
};
