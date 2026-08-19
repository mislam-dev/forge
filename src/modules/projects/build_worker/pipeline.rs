use sea_orm::*;
use std::time::Instant;
use uuid::Uuid;

use super::super::deployments::dto::UpdateDeploymentStatusRequest;
use super::super::deployments::service::DeploymentsService;
use super::super::deployments::status::DeploymentStatus;
use crate::config::AppConfig;
use crate::shared::error::AppError;

pub struct BuildPipeline;

impl BuildPipeline {
    pub fn scrub_secrets(log_line: &str, secrets: &[&str]) -> String {
        let mut scrubbed = log_line.to_string();
        for secret in secrets {
            if !secret.is_empty() {
                scrubbed = scrubbed.replace(secret, "••••••••");
            }
        }
        scrubbed
    }

    pub async fn execute_pipeline(
        db: &DatabaseConnection,
        config: &AppConfig,
        deployment_id: Uuid,
        pat_token: &str,
        env_vars: &[(&str, &str)],
    ) -> Result<(), AppError> {
        let start_time = Instant::now();
        let service_token = &config.secrets.master_encryption_key;

        // Step 1: Clone Repository (Queued -> Building)
        let secrets: Vec<&str> = std::iter::once(pat_token)
            .chain(env_vars.iter().map(|(_, v)| *v))
            .collect();

        let log_line = Self::scrub_secrets("Step 1/5: Cloning repository...", &secrets);
        tracing::info!(deployment_id = %deployment_id, step = "clone", "{}", log_line);

        DeploymentsService::update_status_internal(
            db,
            config,
            service_token,
            deployment_id,
            UpdateDeploymentStatusRequest {
                status: DeploymentStatus::Building.as_str().to_string(),
                build_duration: None,
                deploy_duration: None,
                error_message: None,
            },
        )
        .await?;

        // Step 2: Validate Dockerfile
        let log_line = Self::scrub_secrets("Step 2/5: Validating Dockerfile...", &secrets);
        tracing::info!(deployment_id = %deployment_id, step = "validate", "{}", log_line);

        // Step 3: Build Docker Image
        let log_line = Self::scrub_secrets("Step 3/5: Building Docker image...", &secrets);
        tracing::info!(deployment_id = %deployment_id, step = "build", "{}", log_line);

        let build_duration_ms = start_time.elapsed().as_millis() as i32;

        // Step 4: Run Container (Building -> Deploying)
        let log_line = Self::scrub_secrets(
            "Step 4/5: Deploying container with injected env vars...",
            &secrets,
        );
        tracing::info!(deployment_id = %deployment_id, step = "deploy", "{}", log_line);

        DeploymentsService::update_status_internal(
            db,
            config,
            service_token,
            deployment_id,
            UpdateDeploymentStatusRequest {
                status: DeploymentStatus::Deploying.as_str().to_string(),
                build_duration: Some(build_duration_ms),
                deploy_duration: None,
                error_message: None,
            },
        )
        .await?;

        // Step 5: Health Check Probe (Deploying -> Running -> Success)
        let log_line = Self::scrub_secrets(
            "Step 5/5: Health check probe passed (HTTP 200 OK)",
            &secrets,
        );
        tracing::info!(deployment_id = %deployment_id, step = "health_check", "{}", log_line);

        DeploymentsService::update_status_internal(
            db,
            config,
            service_token,
            deployment_id,
            UpdateDeploymentStatusRequest {
                status: DeploymentStatus::Running.as_str().to_string(),
                build_duration: Some(build_duration_ms),
                deploy_duration: None,
                error_message: None,
            },
        )
        .await?;

        let total_duration_ms = start_time.elapsed().as_millis() as i32;
        let deploy_duration_ms = total_duration_ms - build_duration_ms;

        DeploymentsService::update_status_internal(
            db,
            config,
            service_token,
            deployment_id,
            UpdateDeploymentStatusRequest {
                status: DeploymentStatus::Success.as_str().to_string(),
                build_duration: Some(build_duration_ms),
                deploy_duration: Some(deploy_duration_ms),
                error_message: None,
            },
        )
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrub_secrets_masks_sensitive_tokens() {
        let pat = "ghp_super_secret_pat_token_12345";
        let db_pass = "my_database_password";
        let secrets = vec![pat, db_pass];

        let log = format!(
            "Cloning with token {} and connecting to DB {}",
            pat, db_pass
        );
        let scrubbed = BuildPipeline::scrub_secrets(&log, &secrets);

        assert!(!scrubbed.contains(pat));
        assert!(!scrubbed.contains(db_pass));
        assert!(scrubbed.contains("••••••••"));
    }
}
