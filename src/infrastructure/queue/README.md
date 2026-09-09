# RabbitMQ Infrastructure — Build Worker Consumer Guide

> **Module Location:** [`src/infrastructure/queue/`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue)  
> **Crate:** `amqprs` (v2.1.5)  
> **Architecture Pattern:** Clean Architecture / Static Dispatcher (`QueuePublisher`) & Static Consumer (`MessageHandler<M>`)  
> **Specification Reference:** [ADR-004](../../../docs/system/09-adr/ADR-004-rabbitmq-message-broker.md)  
> **Current Status:** ✅ **Phase 2 Complete — Build Worker Consumer, Execution Engine & Unit Tests Fully Integrated**

---

## 1. Consumer Architecture & Execution Flow

```
RabbitMQ Queue: "forge.deployments.jobs" (Quorum Queue)
       │
       ▼ (QoS prefetch_count = 2)
RabbitMqConsumer (`consumer.rs` / `AsyncConsumer`)
       │
       ▼
BuildWorkerService (impl MessageHandler<DeploymentJobCreated>)
       │
       ▼
BuildWorkerService::process_job(db, config, deployment_id)
       │
       ├── 1. Fetch deployment details:
       │         DeploymentsService::get_deployment_by_id_internal(db, deployment_id)
       │
       ├── 2. Fetch project details:
       │         ProjectsService::get_project_by_internal(db, project_id)
       │
       ├── 3. Secret Resolution:
       │         ├── If project.project_type == ProjectTypes::Repo:
       │         │      ProjectRepositoriesRepository::find_by_project_id(db, project_id)
       │         │      ATService::decrypt(&repo.access_token_encrypted)
       │         └── Decrypted env vars:
       │                ProjectEnvironmentVariablesService::get_decrypted_env_vars(db, None, project_id, "production")
       │
       ├── 4. Execute 5-Step Pipeline:
       │         BuildPipeline::execute_pipeline(db, config, deployment_id, &pat_token, &env_vars)
       │         ├── Step 1: Clone repo via git2 -> update status: Building
       │         ├── Step 2: Validate Dockerfile
       │         ├── Step 3: Build image -> record build_duration
       │         ├── Step 4: Run container -> update status: Deploying
       │         └── Step 5: Health check probe -> update status: Running -> Success
       │
       ├── 5. Success Path:
       │         └── Return Ok(()) -> Consumer sends basic_ack(delivery_tag, false)
       │
       └── 6. Failure Path:
                 ├── Call DeploymentsService::update_status_internal (status: Failed, error_message)
                 └── Return Err(QueueError::PublishedNackedError) -> Consumer sends basic_nack(delivery_tag, false, requeue: false)
                           │
                           ▼
                 Routed to DLX: "forge.deployments.dlx"
                           │
                           ▼
                 DLQ: "forge.deployments.dead-letter" (Routing key: "job.dead-letter")
```

---

## 2. Completed Components & Test Verification

All Phase 2 tasks and verification test suites are implemented and verified:

| File | Component | Role | Status |
|---|---|---|---|
| [`src/infrastructure/queue/publisher.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/publisher.rs) | `RabbitMqPublisher` | Exposes `get_rabbitmq(&self) -> &RabbitMq` | ✅ **Complete** |
| [`src/infrastructure/queue/queue.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/queue.rs) | `QueuePublisher` | Exposes `rabbitmq(&self) -> Option<&RabbitMq>` accessor + unit tests | ✅ **Complete** |
| [`src/main.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/main.rs) | Startup Sequence | Connects broker, declares topology, opens worker channel, and starts `RabbitMqConsumer` | ✅ **Complete** |
| [`src/modules/projects/build_worker/service.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/build_worker/service.rs) | `BuildWorkerService` | Implements `new`, `process_job`, `MessageHandler<DeploymentJobCreated>`, and unit tests | ✅ **Complete** |
| [`src/modules/projects/build_worker/deployment_path.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/build_worker/deployment_path.rs) | `DeploymentPath` | Filesystem layout manager (`source/`, `build/`, `logs/`, `meta.json`, `static/`) + unit tests | ✅ **Complete** |
| [`src/infrastructure/github/repo.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/github/repo.rs) | `GithubRepo` | Clones Git repositories via `git2` with HTTPS token authentication + unit tests | ✅ **Complete** |
| [`src/modules/projects/deployments/repository.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/deployments/repository.rs) | `DeploymentsRepository` | Fixed mock expectations in `test_update_status_success` (SELECT + UPDATE result sets) | ✅ **Complete** |
| [`src/modules/projects/deployments/service.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/deployments/service.rs) | `DeploymentsService` | Added `get_deployment_by_id_internal` + unit test | ✅ **Complete** |
| [`src/modules/projects/projects/service.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/projects/service.rs) | `ProjectsService` | Added `get_project_by_internal` + unit test | ✅ **Complete** |
| [`src/modules/projects/environment_variables/service.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/environment_variables/service.rs) | `ProjectEnvironmentVariablesService` | Added `get_decrypted_env_vars` + unit test | ✅ **Complete** |

---

## 3. Pending Tasks

*(No pending tasks — all Phase 2 components and unit tests are complete).*

---

## 4. Reference Implementation Details

### 4.1 Application Startup & Consumer Registration ([`src/main.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/main.rs))

```rust
if let Some(rmq) = app_state.queue.rabbitmq() {
    match rmq.open_channel().await {
        Ok(worker_channel) => {
            let handler =
                BuildWorkerService::new(app_state.db.clone(), app_state.config.clone());

            match RabbitMqConsumer::start_consumer(
                &worker_channel,
                "forge.deployments.jobs",
                "forge.build_worker",
                2,
                handler,
            )
            .await
            {
                Ok(tag) => {
                    tracing::info!(consumer_tag = %tag, "Build worker consumer registered on forge.deployments.jobs");
                }
                Err(err) => {
                    tracing::error!(error = %err, "Failed to register build worker consumer");
                }
            }
        }
        Err(e) => {
            tracing::error!(error = %e, "Failed to open RabbitMQ channel");
        }
    }
}
```

### 4.2 Build Worker Message Handler ([`src/modules/projects/build_worker/service.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/build_worker/service.rs))

```rust
#[async_trait]
impl MessageHandler<DeploymentJobCreated> for BuildWorkerService {
    async fn handle(&self, job: DeploymentJobCreated) -> Result<(), QueueError> {
        tracing::info!(
            deployment_id = %job.deployment_id,
            project_id = %job.project_id,
            "Received deployment job from queue"
        );

        match Self::process_job(&self.db, &self.config, job.deployment_id).await {
            Ok(_) => {
                tracing::info!(
                    deployment_id = %job.deployment_id,
                    "Build pipeline completed successfully"
                );
                Ok(())
            }
            Err(e) => {
                tracing::error!(
                    error = %e,
                    deployment_id = %job.deployment_id,
                    "Build pipeline failed. Marking deployment as Failed."
                );

                let service_token = self.config.secrets.master_encryption_key.clone();

                let _ = DeploymentsService::update_status_internal(
                    &self.db,
                    &self.config,
                    &service_token,
                    job.deployment_id,
                    UpdateDeploymentStatusRequest {
                        status: DeploymentStatus::Failed.as_str().to_string(),
                        build_duration: None,
                        deploy_duration: None,
                        error_message: Some(e.to_string()),
                    },
                )
                .await;

                Err(QueueError::PublishedNackedError)
            }
        }
    }
}
```

### 4.3 Build Worker Unit Testing Pattern

```rust
#[tokio::test]
async fn test_build_worker_handle_job_failure_triggers_nack() {
    let db = Arc::new(MockDatabase::new(DatabaseBackend::Postgres).into_connection());
    let config = setup_mock_config();
    let service = BuildWorkerService::new(db, config);

    let job = DeploymentJobCreated {
        deployment_id: Uuid::new_v4(),
        project_id: Uuid::new_v4(),
        repository_url: "https://github.com/org/repo".to_string(),
        commit_hash: "abc1234".to_string(),
        branch: "main".to_string(),
        triggered_by: Uuid::new_v4(),
    };

    let result = service.handle(job).await;
    assert!(result.is_err());
    assert!(matches!(result, Err(QueueError::PublishedNackedError)));
}
```
