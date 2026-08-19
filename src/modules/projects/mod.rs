pub mod assignments;
pub mod build_worker;
pub mod deployments;
pub mod environment_variables;
pub mod logs;
pub mod permissions;
pub mod projects;
pub mod repositories;
pub mod router;
pub mod service;

pub use router::projects_router;
pub use service::{
    BuildLogsService, BuildWorkerService, DeploymentsService, ProjectAssignmentsService,
    ProjectEnvironmentVariablesService, ProjectPermissionsService, ProjectRepositoriesService,
    ProjectsService,
};
