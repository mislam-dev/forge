# Module 19 — Dashboard

> **Module Type:** Core Module (Read-Only Aggregator)
> **Priority:** P2 — Post-MVP
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Dashboard Module](../../modules/dashboard/dashboard-module.md)

---

## 1. Module Overview

### Purpose

The Dashboard module provides **aggregated cross-domain metrics** for organization administrators and individual users. It is a **read-only** module — it owns **no database tables** and reads from other modules' tables.

### Responsibilities

- Aggregate deployment metrics (total, success rate, active deployments)
- Aggregate organization statistics (member count, project count, team count)
- Aggregate personal user metrics (deployments triggered, assigned projects)
- Cache aggregated results in Redis to avoid expensive queries on every page load

### Scope

**Included:**
- `GET /dashboard` — platform-wide dashboard (System Admin)
- `GET /dashboard/org/:org_id` — org-level dashboard (org member)
- `GET /dashboard/user` — personal user dashboard (self)

**Excluded:**
- Writing any data (read-only module)
- Detailed deployment views (Deployments module)
- Notification history (Notifications module)

> **Note from module documentation:** "The Dashboard module explicitly owns zero database tables. All data is fetched via cross-module queries and cached."

---

## 2. Dependencies

### Depends On
- **Organizations** (member count, project count)
- **Projects** (project metrics)
- **Deployments** (deployment metrics)
- **Teams** (team count)
- **Redis** (caching aggregated results)
- **Authentication**

### Used By
- No other modules depend on Dashboard

---

## 3. Metrics to Aggregate

### Org Dashboard

| Metric | Source Table | Query |
|--------|-------------|-------|
| Total members | `organization_members` | COUNT WHERE org_id |
| Total projects | `projects` | COUNT WHERE org_id |
| Total teams | `teams` | COUNT WHERE org_id |
| Total deployments | `deployments` JOIN projects | COUNT WHERE project.org_id |
| Deployment success rate (last 30d) | `deployments` | COUNT(Success)/COUNT(*) WHERE status IN (Success, Failed) AND created_at > now() - 30d |
| Active deployments (Running) | `deployments` | COUNT WHERE status = Running AND project.org_id |
| Recent deployments (last 10) | `deployments` | SELECT last 10 WHERE project.org_id |

### User Dashboard

| Metric | Source | Query |
|--------|--------|-------|
| Assigned projects | `project_members` | COUNT WHERE user_id |
| Deployments triggered | `deployments` | COUNT WHERE triggered_by = user_id |
| Recent activity | `deployments` | Last 5 deployments triggered by user |
| Org memberships | `organization_members` | COUNT WHERE user_id |

---

## 4. API Implementation

### GET /dashboard/org/:org_id

- **Auth:** JWT + org member
- **Cache:** Redis key `forge:dashboard:{org_id}`, TTL 300s
- **Service logic:**
  1. Check Redis cache — if hit, return cached JSON
  2. If miss: run all org metric queries (see above)
  3. Cache result in Redis
  4. Return aggregated response
- **Response:**
  ```json
  {
    "message": "Dashboard loaded.",
    "data": {
      "org_id": "UUID",
      "members": 12,
      "projects": 5,
      "teams": 3,
      "deployments": {
        "total": 142,
        "success_rate": 0.94,
        "active": 2,
        "recent": [...]
      }
    }
  }
  ```

### GET /dashboard/user

- **Auth:** JWT (own dashboard)
- **Cache:** Redis key `forge:dashboard:user:{user_id}`, TTL 60s
- **Response:** Personal metrics (see above)

### GET /dashboard (System Admin)

- **Auth:** JWT + System Admin
- **Cache:** Redis key `forge:dashboard:system`, TTL 60s
- **Response:** Platform-wide metrics (total orgs, users, projects, deployments)

---

## 5. Caching Strategy

| Cache Key | TTL | Invalidation |
|-----------|-----|--------------|
| `forge:dashboard:{org_id}` | 300s | Time-based only (acceptable staleness) |
| `forge:dashboard:user:{user_id}` | 60s | Time-based only |
| `forge:dashboard:system` | 60s | Time-based only |

> Dashboard metrics can be slightly stale — real-time accuracy is not required. Cache-aside pattern with TTL expiry is sufficient.

---

## 6. Complex Queries

Dashboard requires joins across multiple tables. These should use SeaORM's raw SQL execution (as permitted by ADR-002 for complex aggregation):

```sql
-- Deployment success rate for org (last 30 days)
SELECT
  COUNT(*) FILTER (WHERE status = 'Success') AS success_count,
  COUNT(*) AS total_count
FROM deployments d
JOIN projects p ON d.project_id = p.id
WHERE p.organization_id = $1
  AND d.status IN ('Success', 'Failed')
  AND d.created_at > NOW() - INTERVAL '30 days'
```

---

## 7. Testing

### Integration Tests
- [ ] `GET /dashboard/org/:org_id` — member: metrics returned
- [ ] `GET /dashboard/org/:org_id` — non-member: 403 returned
- [ ] `GET /dashboard/org/:org_id` — cache hit on second request
- [ ] `GET /dashboard/user` — personal metrics returned
- [ ] `GET /dashboard` — System Admin: platform metrics
- [ ] `GET /dashboard` — non-admin: 403 returned

---

## 8. Implementation Tasks

- [ ] Implement `DashboardService` in src/modules (no dedicated module exists in current structure — add to existing structure or create new)
- [ ] Implement org metric queries (join across deployments, projects, organization_members)
- [ ] Implement user metric queries
- [ ] Implement system metric queries (admin only)
- [ ] Implement Redis cache-aside pattern for all three dashboard types
- [ ] Implement handlers for all 3 dashboard endpoints
- [ ] Register routes in router
- [ ] Write integration tests

---

## 9. Definition of Done

- [ ] All 3 dashboard endpoints return aggregated metrics
- [ ] Redis caching working with correct TTLs
- [ ] Org member check enforced
- [ ] System Admin check enforced
- [ ] No data modified (read-only verified)
- [ ] All tests pass

---

## 10. Estimated Effort

**Medium (1–2 days)**

The SQL queries are complex but the module itself has no write operations. Redis caching implementation is the main non-trivial component.

---

## 11. Recommendations

**Required:**
- Dashboard must be strictly read-only — any write operation is a bug.
- Deployment success rate calculation must exclude in-progress deployments (Queued/Building/Deploying/Running).

**Recommended:**
- Use SeaORM's raw query execution for complex aggregations — trying to build these with the query builder adds unnecessary complexity.
- Cache all dashboard responses with TTL; do not attempt real-time accuracy.

**Future Enhancement:**
- Time-series metrics (deployment frequency over time).
- Export metrics to Prometheus/Grafana.
