use sea_orm::*;
use uuid::Uuid;

use super::pipeline::BuildPipeline;
use crate::config::AppConfig;
use crate::shared::error::AppError;

pub struct BuildWorkerService;

impl BuildWorkerService {
    pub async fn process_job(
        db: &DatabaseConnection,
        config: &AppConfig,
        deployment_id: Uuid,
    ) -> Result<(), AppError> {
        BuildPipeline::execute_pipeline(db, config, deployment_id, "", &[]).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_worker_service_instantiation() {
        let _ = BuildWorkerService;
    }
}
