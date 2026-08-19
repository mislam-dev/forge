# Module 14 — Deployments

> **Module Type:** Core Module
> **Priority:** P1 — Core
> **Status:** Completed (100%)
> **Last Updated:** 2026-08-19
> **Source Docs:** [Deployment Module](../../modules/deployments/deployment-module.md)

---

## 1. Module Overview

### Purpose

The Deployments module manages the **full lifecycle of project deployments**. Developers trigger deployments, which are processed asynchronously through a defined state machine. The module owns the `deployments` database table and is the single source of truth for deployment status.

### Responsibilities

- Trigger a new deployment (create record, emit job to RabbitMQ)
- List deployments for a project
- Get a specific deployment by ID
- Receive status update callbacks from Build Worker (internal API)
- Enforce the deployment state machine (immutable terminal states)
- Enforce the single-running-deployment-per-project constraint

### Scope

**Included:**

- `POST /projects/:project_id/deployments` — trigger deployment
- `GET /projects/:project_id/deployments` — list deployments for project
- `GET /projects/:project_id/deployments/:deployment_id` — get deployment
- Internal: `PUT /internal/deployments/:id/status` — Build Worker status callback

**Excluded:**

- Build execution logic (Build Worker)
- Live log streaming (Live Build Logs)
- Deployment history/redeploy/rollback (Deployment History)

---

## 2. Current State

| Item                             | Status              |
| -------------------------------- | ------------------- |
| `src/modules/deployments/mod.rs` | Exists — empty stub |
| Handlers                         | Not implemented     |
| Service                          | Not implemented     |
| Tests                            | None                |

---

## 3. Dependencies

### Depends On

- **Projects** (deployment belongs to a project)
- **Repository** (needs valid git repo + branch to deploy)
- **RabbitMQ** (emit deployment job to queue)
- **Project Permissions** (Developer+ to trigger)
- **Authentication**

### Used By

- **Build Worker** (consumes deployment job, updates status)
- **Live Build Logs** (reads deployment_id for log correlation)
- **Deployment History** (reads deployments table)
- **Dashboard** (aggregates deployment metrics)
- **Health** (monitors deployment states)

---

## 4. Database Table

### `deployments`

| Column          | Type         | Constraints                                                  |
| --------------- | ------------ | ------------------------------------------------------------ |
| id              | UUID         | PK                                                           |
| project_id      | UUID         | FK -> projects.id CASCADE, Not Null                          |
| triggered_by    | UUID         | FK -> users.id, Not Null                                     |
| branch          | VARCHAR(255) | Not Null                                                     |
| commit_hash     | VARCHAR(40)  | Not Null                                                     |
| status          | VARCHAR      | CHECK(Queued, Building, Deploying, Running, Failed, Success) |
| build_duration  | INTEGER      | Nullable (milliseconds)                                      |
| deploy_duration | INTEGER      | Nullable (milliseconds)                                      |
| error_message   | TEXT         | Nullable                                                     |
| created_at      | TIMESTAMP    | Not Null                                                     |
| updated_at      | TIMESTAMP    | Not Null                                                     |

**Critical Indexes:**

- `(project_id, created_at DESC)` — for deployment list queries
- `CREATE UNIQUE INDEX idx_single_running_deployment ON deployments (project_id) WHERE status = 'Running'` — enforces single running deployment per project at the database level

---

## 5. Deployment State Machine

```
Queued -> Building -> Deploying -> Running -> Success
                |           |          |
                +---> Failed (terminal)
```

**Rules:**

- `Queued`, `Building`, `Deploying`, `Running` are transient states
- `Success` and `Failed` are **immutable terminal states** — no transition out
- Only one deployment per project can be in `Running` state (enforced by partial unique index)
- Status transitions can only go forward — no reversal (enforced at service layer)

---

## 6. API Implementation

### POST /projects/:project_id/deployments

- **Auth:** JWT + project role: Developer, Admin, or Owner
- **Request:** `{ branch?, commit_hash? }` — defaults from project defaults if not provided
- **Service logic:**
  1. Validate project exists and has a connected repository
  2. Validate branch/commit (or resolve from GitHub API if not specified)
  3. Check no `Queued` or `Building` deployment already in progress
  4. Create deployment record with `status = Queued`, `triggered_by = jwt_user_id`
  5. Publish build job to RabbitMQ `forge.deployments` exchange with Publisher Confirms
  6. Return deployment record immediately (do not wait for build)
- **Response:** `201 { message, data: deployment }`
- **Errors:** `409 Conflict` if deployment already in progress, `400` if no repository

### GET /projects/:project_id/deployments

- **Auth:** JWT + project member (any role)
- **Query params:** `page`, `per_page`, `status` (optional filter)
- **Response:** `200 { message, data: [deployments], meta: pagination }`

### GET /projects/:project_id/deployments/:deployment_id

- **Auth:** JWT + project member
- **Response:** `200 { message, data: deployment }`

### PUT /internal/deployments/:id/status (Internal — Build Worker only)

- **Auth:** `SERVICE_TOKEN` header
- **Request:** `{ status, build_duration?, deploy_duration?, error_message? }`
- **Service logic:**
  1. Validate SERVICE_TOKEN
  2. Load deployment
  3. Validate status transition (cannot transition from terminal state)
  4. Update deployment record
- **Response:** `200 { message: "Status updated." }`

---

## 7. RabbitMQ Job Payload

Published to `forge.deployments` exchange on deployment trigger:

```json
{
  "deployment_id": "UUID",
  "project_id": "UUID",
  "repository_url": "string",
  "commit_hash": "string",
  "branch": "string",
  "triggered_by": "UUID"
}
```

Publisher Confirms must be awaited before returning `201` to the client.

---

## 8. Authorization Matrix

| Action                   | Viewer             | Developer | Admin | Owner | System Admin |
| ------------------------ | ------------------ | --------- | ----- | ----- | ------------ |
| Trigger deployment       | No                 | Yes       | Yes   | Yes   | Yes          |
| List deployments         | Yes                | Yes       | Yes   | Yes   | Yes          |
| Get deployment           | Yes                | Yes       | Yes   | Yes   | Yes          |
| Update status (internal) | SERVICE_TOKEN only | —         | —     | —     | —            |

---

## 9. Logging

| Event                                    | Level | Fields                                             |
| ---------------------------------------- | ----- | -------------------------------------------------- |
| Deployment triggered                     | INFO  | deployment_id, project_id, branch, commit, user_id |
| Deployment job published to RabbitMQ     | INFO  | deployment_id, request_id                          |
| Deployment status updated                | INFO  | deployment_id, old_status, new_status              |
| Terminal state transition blocked        | WARN  | deployment_id, current_status, attempted_status    |
| Duplicate in-progress deployment blocked | WARN  | project_id, existing_deployment_id                 |
| Build Worker auth failed                 | WARN  | request_id, endpoint                               |

---

## 10. Testing

### Integration Tests

- [ ] `POST /deployments` — no existing in-progress: deployment created
- [ ] `POST /deployments` — existing Queued/Building: 409 returned
- [ ] `POST /deployments` — no repository connected: 400 returned
- [ ] `POST /deployments` — Viewer role: 403 returned
- [ ] `GET /deployments` — list with pagination
- [ ] `GET /deployments/:id` — 200 returned
- [ ] `GET /deployments/:id` — wrong project: 404 returned
- [ ] Internal: `PUT /status` — valid SERVICE_TOKEN: success
- [ ] Internal: `PUT /status` — invalid SERVICE_TOKEN: 401 returned
- [ ] Internal: `PUT /status` — terminal state transition: 400 blocked
- [ ] Partial unique index: two Running deployments for same project blocked by DB

---

## 11. Implementation Tasks

### Database

- [ ] Create `deployments` migration with all columns, CHECK constraints, and partial unique index
- [ ] Generate SeaORM entity for `deployments`

### Service

- [ ] Implement `DeploymentsService` in `src/modules/deployments/service.rs`
- [ ] Implement `trigger_deployment()` — validate, create record, publish to RabbitMQ
- [ ] Implement `list_deployments()` with pagination and optional status filter
- [ ] Implement `get_deployment_by_id()`
- [ ] Implement `update_deployment_status()` — validate state machine transitions
- [ ] Enforce single-in-progress check at service layer (DB constraint as backup)

### RabbitMQ Integration

- [ ] Implement deployment job publisher with Publisher Confirms
- [ ] Handle RabbitMQ publish failure (log error, return 503)

### Handlers

- [ ] Implement handlers for `POST`, `GET` (list), `GET` (single)
- [ ] Implement internal `PUT /status` handler with SERVICE_TOKEN guard
- [ ] Register routes in router

### Testing

- [ ] Write all integration tests listed above
- [ ] Unit test: state machine transition validation

---

## 12. Definition of Done

- [ ] `POST /deployments` triggers async build via RabbitMQ
- [ ] `GET /deployments` returns paginated list
- [ ] `GET /deployments/:id` returns full deployment record
- [ ] Internal status update validates state machine
- [ ] Terminal state transitions blocked
- [ ] Single-running-deployment constraint enforced (DB partial index)
- [ ] All tests pass

---

## 13. Estimated Effort

**Large (3–4 days)**

The deployment module is complex due to:

- RabbitMQ Publisher Confirms integration
- State machine enforcement
- Partial unique index for Running constraint
- Internal SERVICE_TOKEN API for Build Worker

---

## 14. Risks

| Risk                                     | Impact                            | Mitigation                                                                |
| ---------------------------------------- | --------------------------------- | ------------------------------------------------------------------------- |
| RabbitMQ publish fails after DB insert   | High — deployment stuck in Queued | Implement idempotent retry; Build Worker checks DB state before consuming |
| State machine bypass via direct DB write | High                              | Enforce validation in service layer; partial index as DB safeguard        |
| Two workers racing on same deployment    | Medium                            | DB partial index prevents two Running simultaneously                      |
