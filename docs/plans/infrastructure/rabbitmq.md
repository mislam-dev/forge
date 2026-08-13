# Infrastructure Plan: RabbitMQ

> **Plan Type:** Infrastructure
> **Priority:** P1 — Core
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **ADR:** [ADR-004](../../system/09-adr/ADR-004-rabbitmq-message-broker.md)

---

## 1. Overview

RabbitMQ (AMQP 0-9-1) is the **official message broker for all background build workflows, task queues, and real-time event streaming** in the Forge Platform (ADR-004).

**Three primary responsibilities:**
1. **Deployment job dispatch** — `forge.deployments.jobs` queue delivers build jobs from Deployment API to Build Workers
2. **Real-time log streaming** — `forge.logs` topic exchange routes build log lines from Build Workers to Live Logs SSE API
3. **Notification delivery** — `forge.notifications.jobs` queue delivers async notification events

RabbitMQ client lives in `src/infrastructure/queue/`.

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/infrastructure/queue/mod.rs` | Exists — empty stub |
| RabbitMQ client | Not implemented |
| Exchange declarations | Not implemented |
| Queue declarations | Not implemented |
| AppState integration | Not implemented |

---

## 3. Dependencies

### Depends On
- Foundation (Cargo.toml, AppState)
- RabbitMQ server (Docker Compose service)

### Used By
- Deployments module (publish build jobs)
- Build Worker (consume build jobs, publish log events)
- Live Build Logs module (consume log events)
- Notifications module (publish and consume notification events)

---

## 4. Required Cargo Dependencies

```toml
[dependencies]
# RabbitMQ AMQP client (async, Tokio-compatible)
lapin = "2"

# JSON for message payloads
serde_json = "1"
serde = { version = "1", features = ["derive"] }
```

---

## 5. Exchange and Queue Topology

Per ADR-004:

### Exchanges

| Exchange | Type | Purpose |
|----------|------|---------|
| `forge.deployments` | Direct | Routes build jobs to workers |
| `forge.deployments.dlx` | Direct | Dead-Letter Exchange for failed jobs |
| `forge.logs` | Topic | Routes log lines to SSE clients |

### Queues

| Queue | Type | Durable | DLX | Routing Key |
|-------|------|---------|-----|-------------|
| `forge.deployments.jobs` | Quorum | Yes | `forge.deployments.dlx` | `job.build` |
| `forge.deployments.dead-letter` | Classic | Yes | None | `job.dead-letter` |
| `forge.notifications.jobs` | Quorum | Yes | None | `job.notification` |

### Dynamic SSE Log Queues

Per deployment, when a client connects to `GET /deployments/:id/logs/stream`:
- Create ephemeral auto-delete queue: `forge.logs.client.<uuid>`
- Bind to `forge.logs` exchange with routing key: `deployment.<deployment_id>`
- Delete when client disconnects

---

## 6. Message Payload Contracts

### Build Job Payload (`forge.deployments.jobs`)

```json
{
  "deployment_id": "UUID",
  "project_id": "UUID",
  "repository_url": "string",
  "commit_hash": "string (40-char SHA)",
  "branch": "string",
  "triggered_by": "UUID"
}
```

### Log Line Payload (`forge.logs` topic exchange)

```json
{
  "deployment_id": "UUID",
  "timestamp": "ISO 8601",
  "level": "INFO | WARN | ERROR | DEBUG",
  "step": "clone | build | deploy | health_check",
  "message": "string"
}
```

### Notification Payload (`forge.notifications.jobs`)

```json
{
  "user_id": "UUID",
  "type": "string (event type)",
  "message": "string"
}
```

---

## 7. Reliability Requirements

Per ADR-004:

- **Publisher Confirms:** Deployment API waits for RabbitMQ `basic.ack` before returning `201 Created`
- **Manual Consumer Acknowledgment:** Workers send `basic.ack` only after terminal state or `basic.nack(requeue=false)` on unrecoverable error
- **Prefetch Limit:** `basic.qos(prefetch_count=2)` per worker process — max 2 concurrent Docker builds
- **Idempotency:** Workers check `deployment.status` in PostgreSQL before re-executing steps (handles redelivery)

---

## 8. Implementation Tasks

### Cargo Setup
- [ ] Add `lapin`, `serde_json`, `serde` to Cargo.toml

### Connection Setup
- [ ] Implement `create_rabbitmq_connection()` in `src/infrastructure/queue/mod.rs`
- [ ] Connection string from environment variable `RABBITMQ_URL`
- [ ] Implement connection recovery on drop (lapin supports this)
- [ ] Expose `lapin::Connection` via `AppState`

### Topology Declaration (at startup)
- [ ] Declare `forge.deployments` direct exchange (durable)
- [ ] Declare `forge.deployments.dlx` direct exchange (durable)
- [ ] Declare `forge.logs` topic exchange (durable)
- [ ] Declare `forge.deployments.jobs` quorum queue with DLX binding
- [ ] Declare `forge.deployments.dead-letter` classic queue
- [ ] Declare `forge.notifications.jobs` quorum queue
- [ ] Bind `forge.deployments.jobs` to `forge.deployments` exchange with routing key `job.build`
- [ ] Bind `forge.notifications.jobs` to notification exchange

### Publisher Helpers
- [ ] `publish_deployment_job(channel, payload)` — publishes to `forge.deployments` with confirms
- [ ] `publish_log_line(channel, deployment_id, payload)` — publishes to `forge.logs` topic
- [ ] `publish_notification(channel, payload)` — publishes to `forge.notifications.jobs`

### Consumer Helpers
- [ ] `consume_deployment_jobs(channel) -> Stream<DeliveryResult>` — for Build Worker
- [ ] `consume_log_lines(channel, deployment_id) -> Stream<LogLine>` — for Live Logs SSE
- [ ] `consume_notification_jobs(channel) -> Stream<NotificationJob>` — for Notification worker

### Testing
- [ ] Unit test: connection established
- [ ] Unit test: topology declared without error
- [ ] Unit test: message published and received end-to-end (integration)
- [ ] Unit test: dead-letter routing (nack -> DLX)
- [ ] Unit test: prefetch limit (max 2 concurrent)

---

## 9. Definition of Done

- [ ] RabbitMQ connection established from environment variable
- [ ] All exchanges and queues declared at startup
- [ ] Publisher helpers implemented with Publisher Confirms
- [ ] Consumer helpers with manual ack/nack
- [ ] `AppState` provides access to channel pool
- [ ] Integration tests pass with real RabbitMQ instance
- [ ] Dead-letter routing verified

---

## 10. Estimated Effort

**Medium (1–2 days)**

AMQP topology and lapin API require some learning, but the patterns are well-established. The main complexity is implementing idempotent consumer acknowledgment.

---

## 11. Recommendations

**Required:**
- All queues declared as durable (survive RabbitMQ restarts)
- Publisher Confirms enabled before returning 201 from deployment trigger
- prefetch_count=2 per worker (build job concurrency control)

**Recommended:**
- Use a channel pool (not a single shared channel) for concurrent publishers
- Implement heartbeat handling to detect stale connections

**Future Enhancement:**
- RabbitMQ Quorum Queue clustering for production HA
- Management plugin health probes for the Health module
