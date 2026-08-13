# Module 17 — Deployment History

> **Module Type:** Sub-Module (Deployments)
> **Priority:** P2 — Post-MVP
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Deployment History Module](../../modules/deployments/deployment-history-module.md)

---

## 1. Module Overview

### Purpose

The Deployment History sub-module provides **historical deployment management** — viewing past deployments, re-triggering a previous deployment (redeploy), and rolling back to a previous successful deployment.

### Responsibilities

- List historical deployments with filtering (status, branch, date range)
- Get a specific historical deployment record
- Redeploy: re-trigger a deployment using the same config (branch + commit) as a previous deployment
- Rollback: re-trigger a deployment using the commit hash of the last `Success` deployment

### Scope

**Included:**
- `GET /projects/:project_id/deployments/history` — paginated deployment history
- `POST /projects/:project_id/deployments/:deployment_id/redeploy` — redeploy
- `POST /projects/:project_id/deployments/rollback` — rollback to last success

**Excluded:**
- Current deployment status (Deployments module)
- Build log access (Live Build Logs)

> **Note:** These endpoints build on top of the `deployments` table and the existing Deployments service. This sub-module adds history query filters and the redeploy/rollback operations.

---

## 2. Dependencies

### Depends On
- **Deployments** (reads deployments table, reuses trigger logic)
- **Repository** (validates branch/commit for redeploy)
- **RabbitMQ** (redeploy/rollback publishes a new build job)
- **Project Permissions**

---

## 3. API Implementation

### GET /projects/:project_id/deployments/history

- **Auth:** JWT + project member
- **Query params:** `page`, `per_page`, `status` (filter), `branch` (filter), `from_date`, `to_date`
- **Service logic:** Query `deployments` table with filters, ordered by `created_at DESC`
- **Response:** `200 { message, data: [deployments], meta: pagination }`

### POST /projects/:project_id/deployments/:deployment_id/redeploy

- **Auth:** JWT + project role: Developer, Admin, or Owner
- **Service logic:**
  1. Load the referenced deployment record (must be terminal state — Success or Failed)
  2. Use the same `branch` and `commit_hash`
  3. Reuse Deployments trigger logic: create new deployment record, publish to RabbitMQ
- **Response:** `201 { message, data: new_deployment }`
- **Errors:** `400` if referenced deployment is not in terminal state

### POST /projects/:project_id/deployments/rollback

- **Auth:** JWT + project role: Admin or Owner
- **Service logic:**
  1. Find the most recent deployment with `status = Success` (not the currently running one)
  2. Use its `branch` and `commit_hash`
  3. Reuse Deployments trigger logic: create new deployment, publish to RabbitMQ
- **Response:** `201 { message, data: new_deployment }`
- **Errors:** `404` if no successful deployment exists to roll back to

---

## 4. Authorization Matrix

| Action | Viewer | Developer | Admin | Owner |
|--------|--------|-----------|-------|-------|
| View history | Yes | Yes | Yes | Yes |
| Redeploy | No | Yes | Yes | Yes |
| Rollback | No | No | Yes | Yes |

---

## 5. Testing

### Integration Tests
- [ ] `GET /history` — list with filters
- [ ] `GET /history` — date range filter works
- [ ] `POST /redeploy` — triggers new deployment with same commit
- [ ] `POST /redeploy` — non-terminal deployment: 400 returned
- [ ] `POST /rollback` — success deployment exists: new deployment triggered
- [ ] `POST /rollback` — no success deployment: 404 returned
- [ ] `POST /rollback` — Developer role: 403 returned

---

## 6. Implementation Tasks

- [ ] Implement `DeploymentHistoryService` in `src/modules/deployments/history_service.rs`
- [ ] Implement filtered history query with date range support
- [ ] Implement `redeploy()` — loads referenced deployment, calls deployment trigger
- [ ] Implement `rollback()` — finds last success, calls deployment trigger
- [ ] Implement handlers for all 3 endpoints
- [ ] Register routes in router
- [ ] Write all integration tests

---

## 7. Definition of Done

- [ ] History endpoint returns paginated, filterable deployment list
- [ ] Redeploy triggers new deployment with same branch/commit
- [ ] Rollback finds last success and re-deploys it
- [ ] Authorization enforced correctly
- [ ] All tests pass

---

## 8. Estimated Effort

**Medium (1–2 days)**

Mostly builds on existing Deployments service. New logic is the history query filtering and rollback logic.

---

## 9. Recommendations

**Required:**
- Rollback should only use `Success` state deployments — never `Failed`.
- Redeploy target must be in a terminal state (cannot redeploy an in-progress deployment).

**Recommended:**
- Include `triggered_by_user` name in history responses (join with users table).

**Future Enhancement:**
- Scheduled deployments (deploy at a specified time).
- Deployment approval workflows (require Admin approval before deployment executes).
