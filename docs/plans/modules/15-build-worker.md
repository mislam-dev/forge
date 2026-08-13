# Module 15 — Build Worker

> **Module Type:** Sub-Module (Deployments)
> **Priority:** P1 — Core
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Build Worker Module](../../modules/deployments/build-worker-module.md)

---

## 1. Module Overview

### Purpose

The Build Worker sub-module is the **asynchronous background process** responsible for executing the complete build and deployment pipeline. It consumes deployment jobs from RabbitMQ, executes a 5-step pipeline (clone → Dockerfile validate → build → run → health check), emits structured logs to Loki, and updates the deployment status via the internal Deployment API.

### Responsibilities

1. **Clone Repository** — authenticate with GitHub, clone at specified commit
2. **Read & Validate Dockerfile** — locate and validate `Dockerfile` in repo root
3. **Build Docker Image** — `docker build` the image
4. **Run Container** — `docker run` with environment variables injected
5. **Health Check** — probe the container's health endpoint; mark Success or Failed
6. **Log all steps** — emit structured log lines to Loki (via `tracing`) and RabbitMQ topic exchange (for live streaming)
7. **Update deployment status** — call internal Deployment API with each state transition

### Scope

**Included:**
- RabbitMQ consumer for `forge.deployments.jobs`
- 5-step build pipeline
- Environment variable injection (decrypted in-memory)
- Loki log emission (via `tracing` subscriber)
- RabbitMQ log line publication (for live streaming)
- Internal Deployment API calls for status updates
- Prefetch count: max 2 concurrent builds per worker process

**Excluded:**
- Deployment triggering (Deployments module)
- Live log streaming to clients (Live Build Logs)
- User-facing API (Build Worker is internal only)

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/modules/build_worker/mod.rs` | Exists — empty stub |
| Worker process | Not implemented |
| Pipeline steps | Not implemented |
| Log emission | Not implemented |

---

## 3. Dependencies

### Depends On
- **Deployments** (reads deployment config, calls status update API)
- **Environment Variables** (decrypts env vars for runtime injection)
- **Repository** (reads repository URL + PAT)
- **RabbitMQ** (consumes jobs, publishes log lines)
- **Loki** (structured log emission)
- **Encryption** (decrypt PAT and env var values)

### Used By
- No modules consume from Build Worker directly
- **Live Build Logs** reads log lines from RabbitMQ topic exchange

---

## 4. Worker Execution Model

The Build Worker runs as a Tokio async task that starts when the application starts:

```
main() -> tokio::spawn(build_worker::start())
```

It consumes from `forge.deployments.jobs` queue indefinitely, spawning a Tokio task per job (limited by `prefetch_count=2`).

**Concurrency limit:** `basic.qos(prefetch_count=2)` — at most 2 builds run simultaneously per worker process instance.

---

## 5. Build Pipeline Steps

### Step 1: Clone Repository

```
Status transition: Queued -> Building
```

1. Load deployment record from DB (to get project_id, commit_hash, branch)
2. Load project_repositories for the project (get repository_url + decrypted PAT)
3. Clone: `git clone --depth 1 --branch {branch} {repo_url} /tmp/forge/builds/{deployment_id}/`
4. Checkout: `git checkout {commit_hash}`
5. Log each git output line to Loki and RabbitMQ log exchange
6. On failure: call status API `status=Failed`, `error_message=clone error`

### Step 2: Validate Dockerfile

1. Check if `/tmp/forge/builds/{deployment_id}/Dockerfile` exists
2. Basic validation: non-empty, contains `FROM`
3. Log validation result
4. On failure: call status API `status=Failed`, `error_message=Dockerfile missing or invalid`

### Step 3: Build Docker Image

```
Status remains: Building
```

1. Load env vars for project + environment (decrypted in-memory, NEVER logged)
2. Construct build args (env vars passed as `--build-arg`)
3. Execute: `docker build -t forge/{project_id}:{deployment_id} /tmp/forge/builds/{deployment_id}/`
4. Stream docker build output line by line → emit each line to Loki + RabbitMQ
5. Record `build_duration` (milliseconds from build start)
6. On failure: status API `status=Failed`, `error_message=docker build error`

### Step 4: Run Container

```
Status transition: Building -> Deploying
```

1. Stop and remove any existing container for this project (`docker rm -f forge-{project_id}`)
2. Execute: `docker run -d --name forge-{project_id} -p {port}:{port} --env-file /dev/stdin forge/{project_id}:{deployment_id}` (env vars piped via stdin, not args)
3. Log container ID and start output
4. On failure: status API `status=Failed`, `error_message=container start error`

### Step 5: Health Check

```
Status transition: Deploying -> Running -> Success | Failed
```

1. Poll `http://localhost:{port}{health_check_url}` every 2 seconds
2. Wait up to 30 seconds for 200 OK response
3. On first 200: record `deploy_duration`, call status API `status=Success`
4. On timeout: call status API `status=Failed`, `error_message=health check timeout after 30s`

---

## 6. Log Emission

Every log line emitted by the pipeline must include:

```json
{
  "deployment_id": "UUID",
  "timestamp": "ISO 8601",
  "level": "INFO | WARN | ERROR | DEBUG",
  "step": "clone | build | deploy | health_check",
  "message": "string"
}
```

Log lines go to **two** destinations simultaneously:
1. **Loki** — via `tracing` subscriber (persistent storage)
2. **RabbitMQ** `forge.logs` topic exchange — routing key `deployment.{deployment_id}` (for live streaming)

**Security:** PAT tokens and env var values must **never** appear in any log line. Scrub all secrets before emission.

---

## 7. Secret Handling

- PAT: decrypt from DB → use for `git clone` only → discard in-memory immediately after
- Env vars: decrypt all at once → inject into Docker container → discard immediately after injection
- Never pass secrets as command-line arguments (visible in process list) — use stdin or temp files deleted immediately
- Never log decrypted secrets, even at DEBUG level

---

## 8. Cleanup

After each build (success or failure):
- Delete workspace: `rm -rf /tmp/forge/builds/{deployment_id}/`
- Remove dangling Docker images from failed builds
- Cleanup should happen even if the build fails at any step

---

## 9. Idempotency

The Build Worker must be idempotent:
- Before starting step 1, check `deployment.status` in DB
- If status is already `Failed` or `Success` (terminal), skip processing and `ack` the message
- This handles RabbitMQ message redelivery on worker restart

---

## 10. Implementation Tasks

### RabbitMQ Consumer
- [ ] Implement consumer for `forge.deployments.jobs` queue
- [ ] Configure `basic.qos(prefetch_count=2)`
- [ ] Implement manual ack/nack (ack on completion, nack(requeue=false) on unrecoverable error)

### Pipeline Steps
- [ ] Implement step 1: repository clone (`git2` crate or `std::process::Command` for git)
- [ ] Implement step 2: Dockerfile validation
- [ ] Implement step 3: Docker image build (`bollard` crate for Docker API, or `std::process::Command`)
- [ ] Implement step 4: Container run with env var injection
- [ ] Implement step 5: Health check poll with timeout

### Log Emission
- [ ] Emit structured log lines to `tracing` (Loki via subscriber)
- [ ] Emit log lines to RabbitMQ `forge.logs` topic exchange per step
- [ ] Secret scrubbing before log emission

### Status Updates
- [ ] Implement internal HTTP client call to `PUT /internal/deployments/:id/status`
- [ ] Include SERVICE_TOKEN in Authorization header

### Cleanup
- [ ] Implement workspace cleanup in `Drop` or after each build
- [ ] Implement Docker image cleanup

### Testing
- [ ] Unit test: idempotency (skip if terminal state)
- [ ] Unit test: state machine transitions emitted correctly
- [ ] Integration test: full pipeline with mock Docker (or containerized test)

---

## 11. Required Cargo Dependencies

```toml
[dependencies]
# Docker API client
bollard = "0.17"

# Git operations
git2 = { version = "0.19", features = ["vendored-openssl"] }
# OR: std::process::Command with system git

# HTTP client for status callbacks
reqwest = { version = "0.12", features = ["json"] }

# RabbitMQ (already from infrastructure plan)
lapin = "2"
```

---

## 12. Definition of Done

- [ ] Build Worker consumes from RabbitMQ successfully
- [ ] All 5 pipeline steps implemented
- [ ] Deployment status updated at each step transition
- [ ] Log lines emitted to Loki and RabbitMQ for each step
- [ ] Env vars injected without logging
- [ ] PAT used for clone without logging
- [ ] Workspace cleaned up after build
- [ ] Idempotency implemented
- [ ] All tests pass

---

## 13. Estimated Effort

**Very Large (5–7 days)**

This is the most complex module in the platform. Docker API integration, git operations, secret handling, log streaming, and concurrency control all intersect here.

---

## 14. Risks

| Risk | Impact | Mitigation |
|------|--------|-----------|
| Docker API changes | Medium | Use `bollard` — actively maintained crate |
| Secrets leaked in log output | Critical | Implement scrubbing layer before all log emission |
| Build takes > 10 min | High | Implement timeout; kill build and mark Failed |
| Worker crash mid-build | Medium | Idempotency check on restart; RabbitMQ redelivery |
| Disk space exhaustion from builds | Medium | Aggressive cleanup; monitor disk in health probes |

---

## 15. Recommendations

**Required:**
- Secret scrubbing must be a separate layer, not ad-hoc per log line.
- Build timeout: hard limit 10 minutes (600 seconds) — kill process and mark Failed.
- Workspace cleanup must happen even if build fails.

**Recommended:**
- Use `bollard` (async Docker API client) rather than `std::process::Command docker` for better error handling and streaming.
- Set `prefetch_count=2` to limit concurrent Docker builds per worker process.

**Future Enhancement:**
- Docker registry push (deploy to cloud container runtime instead of local Docker host).
- Kubernetes deployment instead of direct Docker.
