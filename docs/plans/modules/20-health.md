# Module 20 — Health & Observability

> **Module Type:** Core Module
> **Priority:** P0 — Blocker (basic) / P2 (full probes)
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Health Module](../../modules/health/health-observability-module.md) | [Observability & Health](../../system/07-operations/observability-and-health.md)

---

## 1. Module Overview

### Purpose

The Health module is the **operational nerve center** of the Forge Platform. It provides standardized health probes for infrastructure monitoring, load balancer health checks, and operational runbook support. It aggregates the health status of all registered service dependencies.

### Responsibilities

- `GET /health` — public aggregated health probe (load balancer ready)
- `GET /health/details` — detailed per-service health with latency (System Admin only)
- Probe all registered service dependencies in parallel
- Classify each dependency as Critical or Non-Critical
- Compute platform-wide status: `ok | degraded | critical`
- Return structured JSON response with per-service status and latency

### Scope

**Included:**
- Two health endpoints: `/health` and `/health/details`
- Parallel probe of all service dependencies
- Critical/non-critical classification and aggregation
- Latency measurement per service

**Excluded:**
- Deployment metrics (Deployments module)
- Build log monitoring (Live Build Logs)

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/modules/health/mod.rs` | Exists — empty stub |
| Health probes | Not implemented |
| Service registry | Not implemented |

---

## 3. Dependencies

### Depends On (for basic /health — P0)
- **Foundation** (AppState, AppConfig)
- **Database** (ping connection)

### Depends On (for full probes — P2)
- All infrastructure services: Database, Redis, RabbitMQ, Loki
- Build Worker availability

### Used By
- Load balancers (polling `GET /health`)
- Infrastructure monitoring tools
- Operations runbooks

---

## 4. Service Registry & Classification

Per observability documentation:

| Service | Classification | Impact if Down |
|---------|---------------|----------------|
| PostgreSQL Database | **Critical** | All platform functionality unavailable |
| Job Queue (RabbitMQ) | **Critical** | Deployments cannot be queued |
| Auth Module | **Critical** | No users can authenticate |
| Build Worker availability | **Critical** | Queued deployments not processed |
| Log Store (Loki) | **Non-Critical** | Build logs unavailable; deployments still work |
| Pub/Sub broker (RabbitMQ topic) | **Non-Critical** | Live streaming unavailable; stored logs accessible |

---

## 5. Status Computation

```
if any Critical dependency is down:
    platform_status = "critical"
elif any Non-Critical dependency is down:
    platform_status = "degraded"
else:
    platform_status = "ok"
```

---

## 6. API Implementation

### GET /health

- **Auth:** Public (no JWT required)
- **Response:** `200` for `ok` and `degraded`; `503` for `critical`
- **Service logic:**
  1. Probe all services in parallel (tokio::join! or futures::join_all)
  2. Compute platform status
  3. Return response in < 1000ms (timeout per service probe: 500ms)
- **Response format:**
  ```json
  {
    "status": "ok | degraded | critical",
    "timestamp": "ISO 8601",
    "services": {
      "database": { "status": "ok", "latency_ms": 3 },
      "job_queue": { "status": "ok", "latency_ms": 1 },
      "log_store": { "status": "degraded", "error": "connection timeout" }
    }
  }
  ```

### GET /health/details

- **Auth:** JWT + System Admin
- **Response:** Same format as `/health` but includes additional diagnostic details
- **Additional fields:** Host information, version, uptime, connection pool stats

---

## 7. Probe Implementations

### Database Probe

```rust
// Execute a lightweight query to validate connection
SELECT 1
```

Measure latency from start to response.

### Redis Probe

```rust
// PING command
redis_client.ping().await
```

### RabbitMQ Probe

```rust
// Use management API or heartbeat check
channel.connection().status() == Connected
```

### Loki Probe

```rust
// GET /ready on Loki HTTP API
reqwest::get("{loki_url}/ready").await
```

### Build Worker Probe

Simplest approach: check if the RabbitMQ consumer channel is active and `forge.deployments.jobs` queue is reachable.

---

## 8. Implementation Phases

### Phase 1 (P0 — Basic Health — Implement First)

Implement immediately in the Foundation phase, before other modules:
- `GET /health` with only Database probe
- Response: `{ "status": "ok|critical", "timestamp": "..." }`
- This unblocks CI health checks

### Phase 2 (P2 — Full Health — After All Infrastructure)

After all infrastructure services are implemented:
- Add Redis, RabbitMQ, Loki probes
- Add `/health/details` endpoint
- Integrate with monitoring tools

---

## 9. Logging

| Event | Level | Fields |
|-------|-------|--------|
| Health probe executed | DEBUG | status, latency_ms per service |
| Service probe failure | WARN | service_name, error, latency_ms |
| Platform status changed | INFO | old_status, new_status |
| Critical dependency down | ERROR | service_name, error |

---

## 10. Testing

### Unit Tests
- [ ] Status computation: all ok -> "ok"
- [ ] Status computation: non-critical down -> "degraded"
- [ ] Status computation: critical down -> "critical"

### Integration Tests
- [ ] `GET /health` — all services up: `status: ok`, `200` returned
- [ ] `GET /health` — database down: `status: critical`, `503` returned
- [ ] `GET /health` — loki down: `status: degraded`, `200` returned
- [ ] `GET /health/details` — System Admin: detailed response
- [ ] `GET /health/details` — non-admin: 403 returned
- [ ] Response time < 1000ms with all probes (timeout protection)

---

## 11. Implementation Tasks

### Phase 1 (P0)
- [ ] Implement `HealthService` with database-only probe
- [ ] Implement `GET /health` handler
- [ ] Register health route (public — no JWT middleware)
- [ ] Return 503 on critical failure, 200 on ok/degraded

### Phase 2 (P2)
- [ ] Add Redis probe to `HealthService`
- [ ] Add RabbitMQ probe
- [ ] Add Loki probe
- [ ] Add Build Worker availability probe
- [ ] Implement `GET /health/details` handler (System Admin only)
- [ ] Implement probe timeout (500ms per service)
- [ ] Write all tests listed above

---

## 12. Definition of Done (Phase 1)

- [ ] `GET /health` returns structured JSON with database probe result
- [ ] 200 on ok/degraded, 503 on critical
- [ ] No authentication required
- [ ] Response time < 500ms

## Definition of Done (Phase 2)

- [ ] All 6 service probes implemented
- [ ] Parallel probe execution with 500ms timeout per service
- [ ] `/health/details` protected by System Admin check
- [ ] Status computation correct for all combinations
- [ ] All tests pass

---

## 13. Estimated Effort

**Phase 1:** Small (< 0.5 day)
**Phase 2:** Small-Medium (0.5–1 day additional)

---

## 14. Recommendations

**Required:**
- `GET /health` must be public — no JWT required. Load balancers need to access it without authentication.
- Each probe must have a timeout (500ms recommended) to prevent health endpoint from hanging.
- `GET /health` must return 503 when platform status is `critical`.

**Recommended:**
- Run all probes in parallel (not sequential) to minimize total response time.
- Cache health probe results for 5 seconds in Redis to prevent thundering herd from load balancers.

**Future Enhancement:**
- Readiness and liveness probe separation (`/health/live` and `/health/ready`) for Kubernetes deployments.
- Prometheus metrics endpoint (`/metrics`) for Grafana dashboards.
