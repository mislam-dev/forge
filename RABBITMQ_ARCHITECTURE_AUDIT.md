# RabbitMQ + amqprs Architecture Audit — Forge (Rust Axum SaaS)

> **Document Version:** 1.0.0  
> **Target System:** Forge Platform (Axum + SeaORM + Tokio + amqprs)  
> **Scope:** AMQP 0-9-1 Messaging Infrastructure, Workers, Real-time Streaming, and Application Boundaries  
> **Date:** 2026-09-06  

---

## Table of Contents
1. [Existing Application Architecture](#1-existing-application-architecture)
2. [RabbitMQ Inventory and Code Analysis](#2-rabbitmq-inventory-and-code-analysis)
3. [File Structure and Module Boundary Evaluation](#3-file-structure-and-module-boundary-evaluation)
4. [Abstraction Boundary and Decoupling Analysis](#4-abstraction-boundary-and-decoupling-analysis)
5. [Connection and Channel Lifecycle Management](#5-connection-and-channel-lifecycle-management)
6. [Reconnection and Failure Recovery Mechanisms](#6-reconnection-and-failure-recovery-mechanisms)
7. [Publisher Architecture and Message Delivery Guarantees](#7-publisher-architecture-and-message-delivery-guarantees)
8. [Consumer Architecture, QoS, and Lifecycle](#8-consumer-architecture-qos-and-lifecycle)
9. [RabbitMQ Topology Management](#9-rabbitmq-topology-management)
10. [Error Classification and Propagation](#10-error-classification-and-propagation)
11. [Axum AppState Integration and Dependency Injection](#11-axum-appstate-integration-and-dependency-injection)
12. [Configuration Management and Secrets Handling](#12-configuration-management-and-secrets-handling)
13. [Observability, Structured Logging, and Tracing](#13-observability-structured-logging-and-tracing)
14. [Production Reliability and AMQP Semantics](#14-production-reliability-and-amqp-semantics)
15. [Multi-Instance SaaS Scaling Behavior](#15-multi-instance-saas-scaling-behavior)
16. [Testing Strategy and Mock Abstractions](#16-testing-strategy-and-mock-abstractions)
17. [Dependency Audit (amqprs 2.1.5)](#17-dependency-audit-amqprs-215)
18. [Security and Access Control Review](#18-security-and-access-control-review)
19. [Final Architecture Assessment](#19-final-architecture-assessment)
20. [Phased Implementation Roadmap](#20-phased-implementation-roadmap)
21. [Inventory of Proposed Code Changes](#21-inventory-of-proposed-code-changes)

---

## 1. Existing Application Architecture

### 1.1 Architectural Mental Model & Startup Sequence

```
main.rs
  │
  ├── 1. AppConfig::load()?
  │       └── Parses environment variables into Secrets, InfraConnectionUrls, ServerConfig
  ├── 2. logger::init_tracing()
  │       └── Initializes tracing-subscriber with EnvFilter, JSON formatting, and Loki appender
  ├── 3. AppState::new().await?
  │       ├── Re-executes AppConfig::load()? (Redundant parse)
  │       └── connect_db(&app_config.infra.db).await?
  │             └── SeaORM pool (connect_timeout, idle_timeout, 5 retries with exponential backoff)
  │       └── Wraps db: Arc<DatabaseConnection>, config: Arc<AppConfig>
  ├── 4. create_app(app_state).await?
  │       ├── Mounts modular sub-routers:
  │       │     ├── /api/v1/auth
  │       │     ├── /api/v1/users
  │       │     ├── /api/v1/access-control
  │       │     ├── /api/v1/organizations
  │       │     ├── /api/v1/teams
  │       │     ├── /api/v1/projects (includes /deployments and /environment-variables)
  │       │     ├── /api/v1/notifications
  │       │     ├── /api/v1/dashboard
  │       │     └── /api/v1/health & /health
  │       ├── Attaches global middleware:
  │       │     ├── cors_middleware()
  │       │     ├── TimeoutLayer (30s)
  │       │     └── logging_middleware
  │       └── Attaches state via .with_state(app_state)
  ├── 5. TcpListener::bind(host, port)
  └── 6. axum::serve().with_graceful_shutdown(shutdown_signal())
```

### 1.2 Request Flow & Layer Interactions
1. **HTTP Layer (`Axum Handlers`)**: Receives requests, extracts state via `State(state): State<AppState>`, and validates payload via `JsonValidate<T>`.
2. **Service Layer (`Stateless Associated Functions`)**: Handlers delegate directly to domain services (e.g., `DeploymentsService::trigger_deployment(&state.db, ...)`).
3. **Repository Layer (`SeaORM Entities`)**: Services call static repository methods (e.g., `DeploymentsRepository::create_deployment(&db, ...)`).
4. **Infrastructure Layer**: Currently, **only PostgreSQL** is active in `AppState`. Redis and RabbitMQ connection URLs exist in `AppConfig`, but neither has an instantiated client attached to `AppState`.
5. **Background Workers**: No background Tokio tasks or AMQP consumers are currently running in the process. The deployment triggering process in `src/modules/projects/deployments/service.rs:58` terminates at:
   ```rust
   // todo: trigger an event send this event to rabbitmq
   ```

---

## 2. RabbitMQ Inventory and Code Analysis

### 2.1 Code Inventory

| File Path | Lines | Declared Types / Functions | Actual Functionality |
|---|---|---|---|
| `Cargo.toml` | Line 12 | `amqprs = "2.1.5"` | Dependency specification |
| `src/infrastructure/mod.rs` | 2 lines | `mod queue;` | **Private module declaration** (inaccessible to callers) |
| `src/infrastructure/queue/mod.rs` | 15 lines | Re-exports | Exports `RabbitMqConfig`, `RabbitMq`, `RabbitMqError`, `RabbitMqMessage`, `RabbitMqPublisher` |
| `src/infrastructure/queue/config.rs` | 36 lines | `RabbitMqConfig` | Ad-hoc `std::env::var` parsing; ignores centralized config |
| `src/infrastructure/queue/connection.rs` | 45 lines | `RabbitMq` | Wraps `Connection` and single `Channel`; ignores virtual host; no reconnect |
| `src/infrastructure/queue/publisher.rs` | 27 lines | `RabbitMqPublisher` | **No-op stub**: serializes payload, drops channel, returns `Ok` without publishing |
| `src/infrastructure/queue/consumer.rs` | 16 lines | `RabbitMqConsumer`, `MessageHandler` | Incomplete stub; no consumption loop, no ACK/NACK |
| `src/infrastructure/queue/message.rs` | 8 lines | `RabbitMqMessage` | Trait with static `&'static str` methods; prevents dynamic routing keys |
| `src/infrastructure/queue/topology.rs` | 12 lines | `RabbitMqTopology` | Empty stub: `// create exchage` |
| `src/infrastructure/queue/error.rs` | 2 lines | `RabbitMqError` | Alias to raw `amqprs::error::Error` |
| `src/infrastructure/queue/events/mod.rs` | 1 line | None | Empty file |

### 2.2 Compilation Diagnostics
A check run (`cargo check`) outputs **22 warnings** directly caused by `src/infrastructure/queue/*`:
- Unused variables: `payload` in `publisher.rs`, `channel` in `topology.rs`.
- Dead code / unused structs: `RabbitMqConfig`, `RabbitMq`, `RabbitMqConsumer`, `RabbitMqPublisher`, `RabbitMqTopology`, `MessageHandler`, `RabbitMqMessage`.

---

## 3. File Structure and Module Boundary Evaluation

### 3.1 Evaluation of Existing Files
1. **`src/infrastructure/mod.rs`**: Declares `mod queue;` privately. It must be `pub mod queue;` so domain modules and `AppState` can reference queue types.
2. **`src/infrastructure/queue/config.rs`**: Should **not** independently parse environment variables with raw `std::env::var`. All environment parsing belongs in `src/config/env.rs`.
3. **`src/infrastructure/queue/events/`**: Empty directory. Domain events (such as `DeploymentJobEvent`) should reside within their domain module (e.g., `modules/projects/deployments/events.rs`) or a shared contracts module, rather than embedded within infrastructure transport code.
4. **`src/infrastructure/queue/publisher.rs`**: Takes `RabbitMq` by value (`pub fn new(rabbitmq: RabbitMq)`), which prevents connection sharing across the application.
5. **`src/infrastructure/queue/consumer.rs`**: Trait `MessageHandler<M>` lacks delivery metadata (delivery tag, redelivery flag, channel handle), making reliable acknowledgments impossible.

---

## 4. Abstraction Boundary and Decoupling Analysis

### 4.1 Current vs. Desired Coupling

```
[Current State: No Bridge]
Axum Handler
    ↓
DeploymentsService
    ↓
// todo: trigger an event send this event to rabbitmq (Unconnected)

[Anti-Pattern to Avoid: Direct Driver Coupling]
DeploymentsService
    ↓
amqprs::channel::Channel (Tight coupling to external broker crate)
    ↓
amqprs::error::Error leaks into Domain Error

[Desired Architecture: Decoupled Port & Adapter]
Axum Handler
    ↓
DeploymentsService
    ↓
Arc<dyn MessagePublisher> (Dependency Injection via AppState)
    ├── Production: RabbitMqPublisher (amqprs adapter)
    └── Testing:    MockMessagePublisher / NoopMessagePublisher (In-memory)
```

### 4.2 Why an Abstraction is Justified
Forge already possesses 251 passing unit and integration tests configured via `AppState::mock(config)`. If domain services directly invoke concrete `amqprs` structs or channels:
- Every test invoking a service would require a running RabbitMQ broker container or fail with connection refused.
- Low-level driver details (`DeliveryMode`, `BasicProperties`, channel confirmation polling) would bleed into business domain logic.

---

## 5. Connection and Channel Lifecycle Management

### 5.1 Connection Lifecycle
- **Connection Sharing**: Creating connections per HTTP request is avoided.
- **VHost Ignored**: `connection.rs` creates `OpenConnectionArguments::new(&config.host, config.port, &config.username, &config.password)`, but **never invokes `.virtual_host(&config.virtual_host)`**. All connections default to `"/"`, breaking multi-tenant configurations (such as `/forge` specified in ADR-004).
- **Graceful Shutdown**: `main.rs` catches `SIGINT`/`SIGTERM` via `shutdown_signal()`, but no AMQP connection drain or channel close is registered.

### 5.2 Channel Concurrency & Error Traps
- `RabbitMq` holds a single `publisher_channel: Channel`.
- **The AMQP Channel Invalidation Trap**: In AMQP 0-9-1, any protocol exception (such as publishing to an unroutable exchange or declaring incompatible arguments) triggers a **channel-level exception**, causing the broker to forcibly close that channel.
- If a single `publisher_channel` is stored without validation or renewal, a single error permanently breaks all message publishing for the entire application until the server process is restarted.

---

## 6. Reconnection and Failure Recovery Mechanisms

### 6.1 Broker Outage Behavior
1. **Startup Failure**: Unlike `src/database/connection.rs` (which implements 5 retries with exponential backoff up to 30 seconds), `RabbitMq::connect` executes a single connection attempt. If RabbitMQ is still initializing in Docker, Forge immediately crashes.
2. **Runtime Drop**: `amqprs` does **not** provide transparent automatic reconnection. When TCP connection is broken, `Connection::is_open()` returns `false`, and all subsequent calls return errors.
3. **Channel Re-establishment**: If a channel closes due to an AMQP error, the current codebase has no mechanism to reopen it.

---

## 7. Publisher Architecture and Message Delivery Guarantees

### 7.1 Detailed Finding in `publisher.rs`
```rust
pub async fn publish<T>(&self, message: T) -> Result<T, RabbitMqError>
where
    T: RabbitMqMessage,
{
    let payload = serde_json::to_vec(&message)
        .map_err(|e| RabbitMqError::InternalChannelError(e.to_string()))?;

    let _channel = self.rabbitmq.get_publisher_channel();

    Ok(message) // <-- CRITICAL DEFECT: Channel is never used. Message is dropped.
}
```

### 7.2 Missing Guarantees
1. **Publisher Confirms**: ADR-004 Section 7.2 states:
   > *"The Deployment API enables Publisher Confirms on RabbitMQ channels. When a deployment is triggered, the API handler waits for RabbitMQ to acknowledge receipt of the published message before returning 201 Created."*
   Currently, `confirm_select()` is never called on the channel.
2. **Persistent Delivery**: `amqprs::BasicProperties` must set `delivery_mode: 2` to ensure messages persist to disk on durable queues.
3. **Static Routing Limitations**: `RabbitMqMessage::routing_key() -> &'static str` cannot handle dynamic routing keys, such as `deployment.<deployment_id>` required for the live logs topic exchange.

---

## 8. Consumer Architecture, QoS, and Lifecycle

### 8.1 Analysis of `consumer.rs`
```rust
pub struct RabbitMqConsumer {
    channel: Channel,
}

pub trait MessageHandler<M>: Send + Sync
where
    M: Send + 'static,
{
    async fn handle(&self, message: M) -> Result<(), AppError>;
}
```
- Completely inert. It does not implement `amqprs::consumer::AsyncConsumer` or spawn background listener tasks.
- Lacks message acknowledgement context (`delivery_tag`, `channel`). The handler cannot acknowledge (`basic.ack`) or reject (`basic.nack`) messages.
- Does not enforce `basic.qos(prefetch_count = 2)` specified in ADR-004, creating risk of worker OOM during concurrent Docker builds.

---

## 9. RabbitMQ Topology Management

### 9.1 Topology Requirements (ADR-004)

```
Exchanges:
  ├── forge.deployments     (Direct, durable)
  ├── forge.deployments.dlx (Direct, durable dead-letter exchange)
  └── forge.logs            (Topic, durable)

Queues:
  ├── forge.deployments.jobs
  │     ├── Type: Quorum
  │     ├── Durable: Yes
  │     ├── DLX: forge.deployments.dlx
  │     └── Bound: forge.deployments -> routing_key: "job.build"
  ├── forge.deployments.dead-letter
  │     ├── Type: Classic
  │     ├── Durable: Yes
  │     └── Bound: forge.deployments.dlx -> routing_key: "job.dead-letter"
  └── forge.notifications.jobs
        ├── Type: Quorum
        ├── Durable: Yes
        └── Bound: direct -> routing_key: "job.notification"
```

### 9.2 Recommendation
Declare all exchanges, queues, and bindings programmatically during application boot using an ephemeral setup channel, ensuring all topology exists before consumers or publishers begin operations.

---

## 10. Error Classification and Propagation

### 10.1 Deficiencies
`src/infrastructure/queue/error.rs` simply aliases:
```rust
pub use amqprs::error::Error as RabbitMqError;
```
This causes driver-specific protocol errors to leak across layer boundaries. Furthermore, `AppError` in `src/shared/error.rs` has no variant for queue errors, causing messaging errors to be handled as untyped internal errors.

### 10.2 Recommended Error Hierarchy
```rust
#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    #[error("AMQP broker connection error: {0}")]
    ConnectionError(String),
    #[error("AMQP channel error: {0}")]
    ChannelError(String),
    #[error("Message serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Message publish rejected by broker nack")]
    PublishNacked,
    #[error("Topology declaration failed: {0}")]
    TopologyError(String),
    #[error("AMQP operation timed out")]
    Timeout,
}
```

---

## 11. Axum AppState Integration and Dependency Injection

### 11.1 Current AppState
```rust
#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub config: Arc<AppConfig>,
}
```

### 11.2 Proposed AppState
```rust
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub config: Arc<AppConfig>,
    pub queue: Arc<dyn MessagePublisher>,
}
```
- In `AppState::new()`, `queue` is initialized with `Arc::new(RabbitMqPublisher::new(...))`.
- In `AppState::mock(config)`, `queue` is initialized with `Arc::new(NoopMessagePublisher)`.
- This ensures that all 251 existing tests in `tests/*.rs` continue to compile and pass without requiring an external RabbitMQ broker.

---

## 12. Configuration Management and Secrets Handling

### 12.1 Configuration Discrepancies

| Setting | `.env` | `src/config/env.rs` | `queue/config.rs` | Issue |
|---|---|---|---|---|
| AMQP URL | `RABBITMQ_URL=amqp://...` | Parsed in `InfraConnectionUrls` | Ignored | Duplicated intent |
| Virtual Host | `RABBITMQ_VHOST=/` | Not parsed | `RABBITMQ_VIRTUAL_HOST` | **Name mismatch**: defaults to `"/"` |
| Port | `RABBITMQ_PORT=5672` | Not parsed | `.parse().unwrap()` | **Panic risk** on malformed port |
| Host/User/Pass | Set in `.env` | Not parsed | Parsed independently | Configuration fragmentation |

### 12.2 Resolution
Unify AMQP configuration inside `src/config/env.rs`. Derive connection parameters either by parsing `RABBITMQ_URL` or fallback individual variables, with validation and safe defaults.

---

## 13. Observability, Structured Logging, and Tracing

### 13.1 Current Status
The queue infrastructure currently has zero `tracing` instrumentation.

### 13.2 Required Structured Events
- Connection: `info!(host = %config.host, port = config.port, vhost = %config.virtual_host, "RabbitMQ connected")`
- Retry: `warn!(attempt, backoff_secs, error = %err, "RabbitMQ connection failed, retrying")`
- Publish: `debug!(exchange = %exchange, routing_key = %routing_key, "Published AMQP message")`
- Ack/Nack: `debug!(delivery_tag = tag, queue = %queue, "Acknowledged message")`
- Consumer: `info!(queue = %queue, consumer_tag = %tag, "Started consumer worker")`

---

## 14. Production Reliability and AMQP Semantics

| Guarantee | Requirement | Current Status | Impact |
|---|---|---|---|
| At-Least-Once Delivery | Durable Quorum Queues + Persistent Messages | Not declared / No-op publish | Data loss on broker restart |
| Publisher Confirms | Await broker ACK before HTTP 201 | Missing | HTTP success returned even if message drops |
| Worker Rate Limiting | `prefetch_count = 2` | Missing | Multiple heavy Docker builds trigger worker OOM |
| Dead-Letter Exchange | Route failed builds to DLQ | Not declared | Poison messages loop endlessly |

---

## 15. Multi-Instance SaaS Scaling Behavior

```
           Load Balancer
                 │
       ┌─────────┼─────────┐
       ▼         ▼         ▼
     API-1     API-2     API-3
       │         │         │
       └─────────┼─────────┘
                 │
                 ▼
         RabbitMQ Cluster
        /        │       \
       /         │        \
      ▼          ▼         ▼
Worker-1     Worker-2   Worker-3
```

1. **Task Queue (`forge.deployments.jobs`)**: Uses the **Competing Consumers** pattern. Multiple worker nodes subscribe to the same queue. RabbitMQ distributes jobs round-robin. Workers ensure idempotency by verifying `deployment.status` in PostgreSQL.
2. **Live Log Stream (`forge.logs`)**: Uses **Topic Fanout**. Each client connecting to `GET /deployments/:id/logs/stream` gets an ephemeral, auto-delete queue (`forge.logs.client.<uuid>`). All API instances receive log broadcasts without stealing messages from each other.

---

## 16. Testing Strategy and Mock Abstractions

### 16.1 Testing Pyramid
1. **Unit Tests (In-Memory)**: Use `NoopMessagePublisher` or `MockMessagePublisher` (which records published events in a thread-safe `tokio::sync::Mutex<Vec<Event>>`). Fast, offline, deterministic.
2. **Integration Tests (With RabbitMQ)**: Guarded under `#[ignore]` or triggered via `cargo test -- --ignored`. Verifies topology declaration, publisher confirms, and consume-ack cycle against real Docker container.

---

## 17. Dependency Audit (amqprs 2.1.5)

- `amqprs 2.1.5` is actively maintained and built for Tokio.
- Uses internal actor tasks for connections and channels.
- `Connection` and `Channel` instances are lightweight handles implementing `Clone`.
- Publisher confirms are supported via `channel.confirm_select()`.
- No new external crates are strictly required.

---

## 18. Security and Access Control Review

1. **Credential Exposure**: `RabbitMqConfig` should redact passwords in `fmt::Debug` implementations.
2. **Virtual Host Isolation**: The `/forge` vhost prevents accidental collision with system queues or other apps on the same RabbitMQ server.
3. **TLS Encryption**: Staging and production configurations should support `amqps://` using `amqprs::net::TlsAdaptor`.

---

## 19. Final Architecture Assessment

### 19.1 Overall Scorecard

```
Architecture:       4/10
Abstraction:        3/10
Connection Mgmt:    3/10
Publisher:          1/10
Consumer:           1/10
Error Handling:     2/10
Reliability:        2/10
Scalability:        5/10
Testing:            3/10
Security:           4/10
Maintainability:    3/10
Overall:            2.9/10
```

---

### 19.2 What Is Already Good
- Separation of queue infrastructure into `src/infrastructure/queue/`.
- Selection of `amqprs` as an async Tokio AMQP driver.
- Thorough architectural specification already documented in `ADR-004`.
- Robust foundation in the core Axum app with 251 passing unit tests.

---

### 19.3 Problems Found

#### Problem 1: Message Publishing is a No-Op Stub
- **Severity**: Critical
- **Location**: `src/infrastructure/queue/publisher.rs:15-25`
- **Why it matters**: `publish()` serializes the message and returns `Ok(message)` without calling `channel.basic_publish()`. Zero messages are delivered.
- **Recommended solution**: Implement real `channel.basic_publish()` with persistent delivery mode and publisher confirms.

#### Problem 2: Virtual Host is Ignored
- **Severity**: High
- **Location**: `src/infrastructure/queue/connection.rs:16-24`
- **Why it matters**: `args.virtual_host` is never set on `OpenConnectionArguments`. Connections always fall back to `"/"`.
- **Recommended solution**: Call `args.virtual_host(&config.virtual_host)`.

#### Problem 3: Private Module & Missing AppState Injection
- **Severity**: High
- **Location**: `src/infrastructure/mod.rs:1`, `src/app/state.rs:8-11`
- **Why it matters**: `mod queue;` is private. Services have no access to RabbitMQ because it is missing from `AppState`.
- **Recommended solution**: Change to `pub mod queue;`, declare `MessagePublisher` trait, and add `pub queue: Arc<dyn MessagePublisher>` to `AppState`.

#### Problem 4: Leaking Driver Errors
- **Severity**: Medium
- **Location**: `src/infrastructure/queue/error.rs:1`
- **Why it matters**: Raw `amqprs::error::Error` leaks throughout domain boundaries.
- **Recommended solution**: Define a structured `QueueError` enum using `thiserror` and map into `AppError`.

#### Problem 5: Fragmented Configuration & Panic on Port Parse
- **Severity**: Medium
- **Location**: `src/infrastructure/queue/config.rs:10-27`, `src/config/env.rs:79-83`
- **Why it matters**: Discrepancies in env variable names (`RABBITMQ_VHOST` vs `RABBITMQ_VIRTUAL_HOST`) and calling `.unwrap()` on port parsing risks startup panic.
- **Recommended solution**: Centralize queue configuration in `src/config/env.rs`.

#### Problem 6: Single Channel Invalidation Trap
- **Severity**: High
- **Location**: `src/infrastructure/queue/connection.rs:11`
- **Why it matters**: A single protocol error closes an AMQP channel permanently. Reusing a single channel without renewal breaks publishing until app restart.
- **Recommended solution**: Add channel verification and renewal on error.

---

### 19.4 Missing Pieces
1. **Publisher Confirms**: Awaiting broker acknowledgement before HTTP 201.
2. **Declarative Topology Setup**: Automated creation of exchanges, quorum queues, and bindings.
3. **Consumer Worker Implementation**: Full consumer loop with QoS prefetch and manual ACK/NACK.
4. **Mock / Noop Publisher**: In-memory publisher enabling tests to pass without external RabbitMQ.
5. **Dynamic Routing Keys**: Message trait support for dynamic topics (`deployment.<uuid>`).
6. **Graceful Shutdown Integration**: Channel and connection close hooks on server termination.

---

### 19.5 Recommended Target Architecture

```
                                  Axum HTTP Request
                                          │
                                          ▼
                                   Deployments Handler
                                          │
                                          ▼
                                  DeploymentsService
                                          │
                       ┌──────────────────┴──────────────────┐
                       ▼                                     ▼
             DeploymentsRepository               Arc<dyn MessagePublisher>
                       │                                     │
                       ▼                                     ▼
                  PostgreSQL                       RabbitMqPublisher (or Mock)
                                                             │
                                                             ▼
                                                    amqprs Channel Pool
                                                             │
                                                             ▼
                                                      RabbitMQ Broker
                                                  (Exchanges & Quorum Queues)
                                                             │
                                           ┌─────────────────┴─────────────────┐
                                           ▼                                   ▼
                                 Deployment Build Worker              Live Logs SSE Stream
                                (Prefetch=2, Ack/Nack)               (Ephemeral Auto-delete)
```

---

### 19.6 Recommended File Structure

```
src/
├── config/
│   └── env.rs                      # Unified RabbitMQ configuration
├── infrastructure/
│   ├── mod.rs                      # pub mod queue;
│   └── queue/
│       ├── mod.rs                  # Re-exports: RabbitMq, MessagePublisher, QueueError
│       ├── config.rs               # Queue config derived from AppConfig
│       ├── connection.rs           # Connection lifecycle, retry, and channel renewal
│       ├── error.rs                # Structured QueueError enum using thiserror
│       ├── topology.rs             # Declarative exchange/queue/binding declarations
│       ├── publisher.rs            # Real basic_publish with publisher confirms
│       ├── consumer.rs             # Async consumer worker with QoS prefetch and Ack/Nack
│       ├── traits.rs               # Message and MessagePublisher trait definitions
│       └── mock.rs                 # NoopMessagePublisher / MockMessagePublisher
```

---

## 20. Phased Implementation Roadmap

### Phase 1 — Configuration & Error Handling Unification
- Unify RabbitMQ environment parsing inside `src/config/env.rs`.
- Replace raw `amqprs::error::Error` with a structured `QueueError` enum.
- Map `QueueError` into `AppError::InternalServerError`.

### Phase 2 — Abstraction Layer & Test Mocking
- Define `Message` and `MessagePublisher` traits in `src/infrastructure/queue/traits.rs`.
- Implement `NoopMessagePublisher` and `MockMessagePublisher` in `src/infrastructure/queue/mock.rs`.
- Inject `pub queue: Arc<dyn MessagePublisher>` into `AppState` and wire `NoopMessagePublisher` into `AppState::mock()`.
- Run test suite to verify all 251 existing tests remain 100% passing.

### Phase 3 — Connection Lifecycle, Topology & Resilient Publisher
- Implement retry loop with exponential backoff on RabbitMQ connection startup.
- Fix virtual host configuration on `OpenConnectionArguments`.
- Implement declarative topology setup in `src/infrastructure/queue/topology.rs`.
- Implement real `basic_publish` with `BasicProperties` (persistent delivery) and publisher confirms.

### Phase 4 — Consumer Infrastructure & Graceful Shutdown
- Implement `AsyncConsumer` in `src/infrastructure/queue/consumer.rs` with `basic.qos(prefetch_count = 2)`.
- Implement message routing to handlers with manual ACK/NACK semantics.
- Wire shutdown drain signals into `main.rs`.

### Phase 5 — Domain Event Wiring & Verification
- Wire `DeploymentsService::trigger_deployment` to publish `DeploymentJobCreated` event via `state.queue`.
- Add broker integration tests to verify full roundtrip publish-consume flow.

---

## 21. Inventory of Proposed Code Changes

### Files that would need modification:
- `src/infrastructure/mod.rs` — Change `mod queue;` to `pub mod queue;`.
- `src/config/env.rs` — Unify AMQP configuration.
- `src/shared/error.rs` — Add queue error variant to `AppError`.
- `src/app/state.rs` — Add `queue: Arc<dyn MessagePublisher>` to `AppState` and `AppState::mock`.
- `src/main.rs` — Initialize RabbitMQ and link graceful shutdown.
- `src/infrastructure/queue/mod.rs` — Expose updated queue types.
- `src/infrastructure/queue/config.rs` — Map from centralized `AppConfig`.
- `src/infrastructure/queue/connection.rs` — Fix vhost handling, add retry/backoff.
- `src/infrastructure/queue/publisher.rs` — Implement real `basic_publish` and confirms.
- `src/infrastructure/queue/consumer.rs` — Implement consumer worker with QoS and Ack/Nack.
- `src/infrastructure/queue/topology.rs` — Implement declarative queue and exchange setup.
- `src/infrastructure/queue/error.rs` — Replace type alias with structured `QueueError`.
- `src/modules/projects/deployments/service.rs` — Replace TODO with message dispatch via `MessagePublisher`.

### New files to create:
- `src/infrastructure/queue/traits.rs` — `Message` and `MessagePublisher` traits.
- `src/infrastructure/queue/mock.rs` — `NoopMessagePublisher` and `MockMessagePublisher` for tests.
- `src/modules/projects/deployments/events.rs` — Deployment domain event models.
