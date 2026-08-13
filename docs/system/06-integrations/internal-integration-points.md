# Internal Integration Points

> **Document:** Internal Integration Points  
> **Section:** 06 — Integrations  
> **Version:** 1.0  
> **Status:** Draft

This document describes the internal infrastructure integrations within Forge — async workers, message queues, pub/sub systems, and the service contracts between internal components.

---

## 1. Integration Overview

Forge relies on the following internal integration points beyond the primary database:

| Integration | Technology | Purpose |
|-------------|-----------|---------|
| **Job Queue** | Redis / RabbitMQ | Async dispatch of deployment build jobs |
| **Pub/Sub Channel** | Redis Pub/Sub / WebSocket broker | Real-time log line delivery from Build Worker to Live Logs API |
| **Docker Runtime** | Docker / OCI-compatible runtime | Container image build and execution during deployments |
| **Git Service** | Native git / libgit2 | Repository clone and commit metadata fetching |
| **Log Store** | Grafana Loki | Persistent aggregation and storage of raw build/deployment log output streams ([ADR-005](../09-adr/ADR-005-use-loki-for-centralized-logging.md)) |
| **Encryption Key Service** | Application-level (master key + project salt) | AES-256-GCM key derivation for secrets |

---

## 2. Job Queue — Deployment Dispatch

### Purpose

Decouples the Deployment API (which must respond immediately) from the Build Worker (which executes long-running operations). Ensures builds are processed reliably even if workers temporarily fail.

### Dispatch Contract

**Producer:** Deployment Module  
**Consumer:** Build Worker  
**Queue:** Named deployment job queue (e.g., `forge:deployments`)

**Job Payload:**

```json
{
  "deployment_id": "UUID",
  "project_id": "UUID",
  "repo_url": "https://github.com/org/repo.git",
  "auth_type": "pat | public",
  "commit_hash": "40-char SHA",
  "branch": "main",
  "environment": "Development | Preview | Production",
  "image_tag": "project-{id}:{commit_short_sha}"
}
```

### Guarantees

| Property | Value |
|----------|-------|
| Delivery semantics | At-least-once (idempotent consumer required) |
| Worker pickup latency | < 5 seconds |
| Retry behavior | Build Worker retries status updates to Deployment API with exponential backoff (max 3 retries) |
| Queue availability | Required for deployments to function; classified as **critical dependency** in Health module |

---

## 3. Build Worker — Internal Service Contract

The Build Worker is an internal async service. It does not expose a public API. Its only outbound communication paths are:

### 3.1 Status Updates → Deployment API

| Direction | Mechanism |
|-----------|-----------|
| Build Worker → Deployment API | `PATCH /deployments/:id/status` via internal service token |

**Payload:**

```json
{
  "status": "Building | Deploying | Success | Failed",
  "build_duration": 45200,
  "deploy_duration": 12800,
  "error_message": "optional error detail"
}
```

**State transition sequence:**

```
Queued → [Worker picks up] → Building → Deploying → Running → Success
                                                  ↘ Failed (any step)
```

### 3.2 Log Writes → Log Store & Pub/Sub

Each build step emits structured log entries in real time:

```json
{
  "deployment_id": "UUID",
  "timestamp": "ISO 8601",
  "level": "INFO | WARN | ERROR | DEBUG",
  "step": "clone | build | deploy | health_check",
  "message": "Log line content"
}
```

- Log lines are **persisted** to Grafana Loki (per [ADR-005](../09-adr/ADR-005-use-loki-for-centralized-logging.md)).
- Log lines are **published** to RabbitMQ topic exchange for `deployment_id` for real-time SSE delivery.

### 3.3 Env Var Decryption → Environment Variables API

Before running the container, the Build Worker requests decrypted env vars:

| Direction | Mechanism |
|-----------|-----------|
| Build Worker → Env Vars API | `GET /projects/:id/env-vars/decrypt` with internal service token |
| Response | Array of `{key, value}` pairs in plaintext |

> **Security constraint:** Decrypted values are passed to `docker run` via in-memory environment injection. They must **never** be written to logs or stored anywhere in plaintext.

---

## 4. Pub/Sub Channel — Real-Time Log Streaming

### Purpose

Delivers log lines from the Build Worker to the Live Build Logs API in real time, which then pushes them to connected clients via SSE or WebSocket.

### Channel Naming

Each deployment gets a dedicated channel:

```
forge:logs:{deployment_id}
```

### Message Format

```json
{
  "deployment_id": "UUID",
  "timestamp": "ISO 8601",
  "level": "INFO",
  "step": "build",
  "message": "Step 3/8 : RUN cargo build --release"
}
```

### Lifecycle

| Event | Action |
|-------|--------|
| Build Worker starts | Begins publishing to `forge:logs:{deployment_id}` |
| Live Logs API client connects | Subscribes to `forge:logs:{deployment_id}` |
| Each log line emitted | Forwarded to all SSE subscribers |
| Deployment reaches terminal state | Final message emitted; channel closed |

### Fallback

If a client connects after the deployment has already reached a terminal state, the Live Logs API queries stored logs directly from Grafana Loki (per [ADR-005](../09-adr/ADR-005-use-loki-for-centralized-logging.md)), bypassing real-time Pub/Sub.

---

## 5. Docker Runtime Integration

The Build Worker interacts with Docker (or an OCI-compatible runtime) for two operations:

### 5.1 Image Build

```bash
docker build -t project-{id}:{commit_short_sha} {workspace_path}
```

- Build context is the cloned repository workspace.
- Output is streamed to the log store line-by-line.
- `build_duration` is recorded from build start to image push.

### 5.2 Container Run

```bash
docker run --env-file {injected_env_file} {image_tag}
```

- Environment variables are injected via in-memory env file (never via command-line args to prevent token exposure in process listings).
- `deploy_duration` is recorded from container start to health check success.

### 5.3 Health Check Poll

After container start, the worker polls the container's health endpoint:

- **URL:** `http://localhost:{port}/health`
- **Method:** `GET`
- **Success condition:** HTTP `200` response
- **Timeout:** 30 seconds
- **Retry interval:** 1000ms
- **Failure action:** Transition deployment to `Failed` (WORKER_005)

---

## 6. Git Service Integration

The Repository module and Build Worker both interact with Git:

### 6.1 Repository Module Operations

| Operation | Git Command | Auth |
|-----------|------------|------|
| Validate connection | `git ls-remote {url}` | Public or PAT |
| List remote branches | `git ls-remote --heads {url}` | Public or PAT |
| Fetch latest commit | `git ls-remote {url} refs/heads/{branch}` | Public or PAT |

### 6.2 Build Worker Clone

| Operation | Git Command | Auth |
|-----------|------------|------|
| Clone at commit | `git clone --depth 1 {url}` then `git checkout {commit_hash}` | Decrypted PAT (in-memory credential helper) |

**PAT injection security:**  
PAT tokens are injected via Git's credential helper or via a memory-buffered `GIT_ASKPASS` script — **never** appended directly to the URL in command-line arguments.

---

## 7. Encryption Key Management

### Key Derivation

- A platform-level **master secret key** is stored in secure environment configuration (not in the database).
- Per-project keys are derived using `HKDF(master_key, salt=project_id)`.
- This ensures that compromise of one project's encrypted data does not compromise others.

### Encryption Process (AES-256-GCM)

```
1. Derive key: K = HKDF(master_key, project_id)
2. Generate random IV: IV = random_bytes(12)
3. Encrypt: (ciphertext, auth_tag) = AES-256-GCM(K, IV, plaintext)
4. Store: Base64(IV || ciphertext || auth_tag)
```

### Where Encryption Is Applied

| Module | What | When |
|--------|------|------|
| Repository | PAT tokens | On `POST /projects/:id/repository` (connect) |
| Environment Variables | Secret values | On `POST` and `PUT` when `is_secret = true` |

---

## 8. Health Module — Service Registry

The Health module maintains a registry of critical and non-critical dependencies to probe:

| Dependency | Classification | Probe Method |
|------------|---------------|--------------|
| Primary Database (PostgreSQL) | **Critical** | Test connection |
| Job Queue (Redis/RabbitMQ) | **Critical** | Ping broker |
| Auth Module | **Critical** | Internal health endpoint |
| Build Worker availability | **Critical** | Queue depth or worker heartbeat |
| Log Store | **Non-Critical** | Read test |
| Pub/Sub broker | **Non-Critical** | Ping |

### Aggregate Status Rules

| Condition | Platform Status |
|-----------|----------------|
| All critical dependencies healthy | `ok` |
| One or more critical dependencies down | `critical` |
| Only non-critical dependencies down | `degraded` |

---

**Document Version:** 1.0  
**Last Updated:** 2026-08-12  
**Author:** Backend Architecture Team
