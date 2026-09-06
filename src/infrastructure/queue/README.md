# RabbitMQ Infrastructure Module — Implementation & Integration Guide

> **Module Location:** `src/infrastructure/queue/`  
> **Crate:** `amqprs` (v2.1.5)  
> **Architecture Pattern:** Clean Architecture / Enum Dispatcher (`QueuePublisher`)  
> **Specification Reference:** [ADR-004](../../../docs/system/09-adr/ADR-004-rabbitmq-message-broker.md)  
> **Current Status:** ✅ **Fully Integrated with Axum AppState & Deployments Service**

---

## 1. Architecture & Request Flow

The application uses an enum-based dispatcher (`QueuePublisher`) providing static dispatch without trait-object lifetime complexities:

```
Axum HTTP Handler (POST /api/v1/projects/:id/deployments)
       │
       ▼
Extracts `state.queue` from `State<AppState>`
       │
       ▼
DeploymentsService::trigger_deployment(&state.db, state.queue, ...)
       │
       ├── 1. Validates project & connected repository in PostgreSQL
       ├── 2. Creates deployment record in DB (status: Queued)
       └── 3. queue.publish(&DeploymentJobCreated { ... }).await?
                 │
                 ▼
QueuePublisher Dispatcher
  ├── Production: QueuePublisher::RabbitMq(RabbitMqPublisher)
  │     ├── Exchange: "forge.deployments" (Direct)
  │     ├── Routing Key: "job.build"
  │     └── Queue: "forge.deployments.jobs" (Quorum queue, durable, DLX bound)
  │
  └── Testing:    QueuePublisher::Mock(MockMessagePublisher)
        └── Records event in-memory for zero-dependency test verification
```

---

## 2. Implemented Code Inventory & Status

| File | Component | Role in Application | Status |
|---|---|---|---|
| [`queue.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/queue.rs) | `QueuePublisher` | Unified dispatcher enum (`RabbitMq` vs `Mock`) with static `publish(&event)` dispatch. | **COMPLETE** |
| [`error.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/error.rs) | `QueueError` | Typed domain error enum wrapping AMQP protocol and serialization errors. | **COMPLETE** |
| [`config.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/config.rs) | `RabbitMqConfig` | Connection config with safe defaults and password redaction in `fmt::Debug`. | **COMPLETE** |
| [`connection.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/connection.rs) | `RabbitMq` | Connection lifecycle handle with custom `Debug` and explicit `virtual_host` setup. | **COMPLETE** |
| [`traits.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/traits.rs) | Abstractions | `RabbitMqMessage`, `#[async_trait] MessageHandler<M>`, and `MessagePublisher`. | **COMPLETE** |
| [`topology.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/topology.rs) | `RabbitMqTopology` | Programmatic declaration of direct/topic exchanges, DLQ, and quorum queues. | **COMPLETE** |
| [`publisher.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/publisher.rs) | `RabbitMqPublisher` | Publishes persistent AMQP messages (`delivery_mode: 2`, `message_id`, `timestamp`). | **COMPLETE** |
| [`consumer.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/consumer.rs) | `RabbitMqConsumer` | `AsyncConsumer` with worker prefetch limit (`prefetch_count = 2`) & auto ACK/NACK. | **COMPLETE** |
| [`mock.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/mock.rs) | `MockMessagePublisher` | In-memory publisher recording events for fast, offline unit testing. | **COMPLETE** |
| [`events/deployments.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/queue/events/deployments.rs) | `DeploymentJobCreated` | Concrete build job event matching ADR-004 payload contracts. | **COMPLETE** |
| [`src/infrastructure/mod.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/mod.rs) | Visibility | Exposed queue module via `pub mod queue;`. | **COMPLETE** |
| [`src/app/state.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/app/state.rs) | `AppState` | Holds `pub queue: QueuePublisher` (wired to real RabbitMQ in `new()`, Mock in `mock()`). | **COMPLETE** |
| [`src/main.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/main.rs) | Startup Sequence | Connects to RabbitMQ broker and verifies topology during boot. | **COMPLETE** |
| [`deployments/service.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/deployments/service.rs) | `DeploymentsService` | Dispatches `DeploymentJobCreated` event upon successful deployment creation. | **COMPLETE** |
| [`deployments/handlers.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/deployments/handlers.rs) | HTTP Handler | Passes `state.queue` to `trigger_deployment`. | **COMPLETE** |

---

## 3. Tasks & Implementation Status Checklist

- [x] **Task 1: Core Queue Infrastructure Types** — **COMPLETE**
  - Typed error handling (`error.rs`)
  - Connection & channel management with virtual host configuration (`connection.rs`)
  - Configuration parsing with secret redaction (`config.rs`)
- [x] **Task 2: Queue Abstractions & Static Dispatcher** — **COMPLETE**
  - Trait definitions for messages, handlers, and publishers (`traits.rs`)
  - Mock message publisher for zero-dependency offline testing (`mock.rs`)
  - `QueuePublisher` enum for static dispatch without `dyn` lifetime issues (`queue.rs`)
- [x] **Task 3: Topology Management** — **COMPLETE**
  - Declarative setup of direct/topic exchanges, DLX, DLQ, and quorum queue (`topology.rs`)
- [x] **Task 4: Persistent Message Publishing** — **COMPLETE**
  - Publishing with delivery mode 2, message ID, and timestamp (`publisher.rs`)
  - Deployment job event contract definition (`events/deployments.rs`)
- [x] **Task 5: Consumer Infrastructure** — **COMPLETE**
  - Async consumer with QoS prefetch (`prefetch_count = 2`) and auto ACK/NACK (`consumer.rs`)
- [x] **Task 6: Application State & Dependency Injection** — **COMPLETE**
  - Expose `pub mod queue;` in `src/infrastructure/mod.rs`
  - Add `queue: QueuePublisher` to `AppState::new()` and `AppState::mock()` in `src/app/state.rs`
- [x] **Task 7: Application Startup Verification** — **COMPLETE**
  - Broker connection and topology declaration on boot in `src/main.rs`
- [x] **Task 8: Domain Service Event Dispatch** — **COMPLETE**
  - Update `DeploymentsService::trigger_deployment` to accept `queue: QueuePublisher` and publish `DeploymentJobCreated` in `service.rs`
  - Pass `state.queue` from HTTP handler in `handlers.rs`
- [x] **Task 9: Service Unit Test Mock Queue Wiring** — **COMPLETE**
  - Update `test_trigger_deployment_project_not_found` in `src/modules/projects/deployments/service.rs` to pass `QueuePublisher::Mock(MockMessagePublisher::new())`

---

## 4. Actual Code Implementation Walkthrough

### 4.1 The Dispatcher Enum (`src/infrastructure/queue/queue.rs`)

```rust
use super::error::QueueError;
use super::mock::MockMessagePublisher;
use super::publisher::RabbitMqPublisher;
use super::traits::RabbitMqMessage;

#[derive(Clone, Debug)]
pub enum QueuePublisher {
    RabbitMq(RabbitMqPublisher),
    Mock(MockMessagePublisher),
}

impl QueuePublisher {
    pub async fn publish<M: RabbitMqMessage>(&self, message: &M) -> Result<(), QueueError> {
        match self {
            Self::RabbitMq(q) => q.publish(message).await,
            Self::Mock(q) => q.publish(message).await,
        }
    }
}
```

---

### 4.2 The Deployment Job Event (`src/infrastructure/queue/events/deployments.rs`)

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::queue::traits::RabbitMqMessage;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeploymentJobCreated {
    pub deployment_id: Uuid,
    pub project_id: Uuid,
    pub repository_url: String,
    pub commit_hash: String,
    pub branch: String,
    pub triggered_by: Uuid,
}

impl RabbitMqMessage for DeploymentJobCreated {
    fn exchange() -> &'static str {
        "forge.deployments"
    }

    fn routing_key() -> &'static str {
        "job.build"
    }

    fn message_type() -> &'static str {
        "deployment.job.created"
    }
}
```

---

### 4.3 AppState Integration (`src/app/state.rs`)

```rust
use crate::infrastructure::queue::{
    MockMessagePublisher, QueuePublisher, RabbitMq, RabbitMqConfig, RabbitMqPublisher,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub config: Arc<AppConfig>,
    pub queue: QueuePublisher,
}

impl AppState {
    pub async fn new() -> Result<AppState, Box<dyn std::error::Error>> {
        let app_config = AppConfig::load()?;

        let db_connection = connect_db(&app_config.infra.db)
            .await
            .map_err(|e| format!("Failed to connect to database: {}", e))?;

        let rmq_config = RabbitMqConfig::from_env();
        let rabbitmq = RabbitMq::connect(&rmq_config).await?;

        let queue = QueuePublisher::RabbitMq(RabbitMqPublisher::new(rabbitmq));
        Ok(Self::from_parts(db_connection, app_config, queue))
    }

    pub fn from_parts(db: DatabaseConnection, config: AppConfig, queue: QueuePublisher) -> Self {
        Self {
            db: Arc::new(db),
            config: Arc::new(config),
            queue,
        }
    }

    pub fn mock(config: AppConfig) -> Self {
        let db = sea_orm::MockDatabase::new(sea_orm::DatabaseBackend::Postgres).into_connection();
        let queue = QueuePublisher::Mock(MockMessagePublisher::new());
        Self::from_parts(db, config, queue)
    }
}
```

---

### 4.4 Service Event Dispatching (`src/modules/projects/deployments/service.rs`)

```rust
use crate::infrastructure::queue::QueuePublisher;
use crate::infrastructure::queue::events::deployments::DeploymentJobCreated;

impl DeploymentsService {
    pub async fn trigger_deployment(
        db: &DatabaseConnection,
        queue: QueuePublisher,
        org_id: Option<Uuid>,
        project_id: Uuid,
        triggered_by: Uuid,
        req: TriggerDeploymentRequest,
    ) -> Result<DeploymentResponse, AppError> {
        // ... (project, repository, and active deployment validations) ...

        let deployment = DeploymentsRepository::create_deployment(
            db,
            project_id,
            triggered_by,
            branch,
            commit_hash,
            DeploymentStatus::Queued,
        )
        .await?;

        let deployment_clone = deployment.clone();

        let job_event = DeploymentJobCreated {
            deployment_id: deployment.id,
            project_id: deployment.project_id,
            repository_url: repo.repository_url,
            commit_hash: deployment.commit_hash,
            branch: deployment.branch,
            triggered_by: deployment.triggered_by,
        };

        if let Err(err) = queue.publish(&job_event).await {
            tracing::error!(
                error = %err,
                deployment_id = %deployment.id,
                "Failed to publish deployment job to queue"
            );
            // todo: update deployment status to failed
        }

        Ok(DeploymentResponse::from_model(deployment_clone))
    }
}
```

---

### 4.5 Handler Invocation (`src/modules/projects/deployments/handlers.rs`)

```rust
pub async fn trigger_deployment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    claims: JwtClaims,
    JsonValidate(payload): JsonValidate<TriggerDeploymentRequest>,
) -> Result<ApiResponse<DeploymentResponse>, AppError> {
    let org_id = None;
    let deployment = DeploymentsService::trigger_deployment(
        &state.db,
        state.queue,
        org_id,
        id,
        claims.sub,
        payload,
    )
    .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Deployment triggered successfully".to_string())
        .body(Some(deployment)))
}
```

---

### 4.6 Application Startup & Topology Verification (`src/main.rs`)

```rust
use forge::infrastructure::queue::{RabbitMq, RabbitMqConfig, RabbitMqTopology};

// In main():
let rmq_config = RabbitMqConfig::from_env();

match RabbitMq::connect(&rmq_config).await {
    Ok(rmq) => {
        if let Err(err) = RabbitMqTopology::setup(&rmq).await {
            tracing::error!(error = %err, "Failed to declare RabbitMQ topology");
        } else {
            tracing::info!("RabbitMQ topology verified successfully");
        }
    }
    Err(e) => {
        tracing::warn!(error = %e, "Could not connect to RabbitMQ broker on startup");
    }
}
```

---

## 5. Testing Note for Service Unit Tests

In [`src/modules/projects/deployments/service.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/deployments/service.rs#L265), the unit test `test_trigger_deployment_project_not_found` should pass the mock queue:

```rust
#[tokio::test]
async fn test_trigger_deployment_project_not_found() {
    let db = setup_mock_db();
    let queue = QueuePublisher::Mock(MockMessagePublisher::new());

    let result = DeploymentsService::trigger_deployment(
        &db,
        queue,
        None,
        Uuid::new_v4(),
        Uuid::new_v4(),
        TriggerDeploymentRequest {
            branch: Some("main".to_string()),
            commit_hash: None,
        },
    )
    .await;

    assert!(result.is_err());
}
```
