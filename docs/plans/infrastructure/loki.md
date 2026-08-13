# Infrastructure Plan: Grafana Loki — Centralized Logging

> **Plan Type:** Infrastructure
> **Priority:** P1 — Core
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **ADR:** [ADR-005](../../system/09-adr/ADR-005-use-loki-for-centralized-logging.md)

---

## 1. Overview

Grafana Loki is the **sole centralized logging platform** for the Forge Platform (ADR-005). It stores **both**:

1. **Application-level logs** — Axum HTTP access logs, middleware events, service errors, DB/Redis/RabbitMQ client events
2. **Build and deployment logs** — Git clone output, Docker build output, container runtime logs, health check results

PostgreSQL does **not** store raw log strings. The `build_logs` database table is legacy and should not be used for new log writes.

The logging infrastructure lives in `src/infrastructure/observability/`.

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/infrastructure/observability/mod.rs` | Exists — empty stub |
| `tracing` subscriber setup | Not implemented |
| Loki push integration | Not implemented |
| Request ID middleware | Not implemented |
| Structured log format | Not implemented |

---

## 3. Dependencies

### Depends On
- Foundation (Cargo.toml, AppState)
- Grafana Loki server (Docker Compose service or external)

### Used By
- Every module (all application logs go through `tracing`)
- Build Worker (build pipeline logs pushed to Loki)
- Live Build Logs (Loki LogQL queries for stored log retrieval)

---

## 4. Required Cargo Dependencies

```toml
[dependencies]
# Structured logging / tracing
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Loki push (choose one approach):
# Option A: Direct HTTP push
tracing-loki = "0.2"
# Option B: Stdout + Promtail/Alloy agent (simpler, recommended for MVP)
# (No additional dep needed — just structured stdout)

# Request ID generation
uuid = { version = "1", features = ["v4"] }
```

> **Note per ADR-005:** The log collection mechanism is TBD — either direct push via `tracing-loki` or stdout capture via Promtail/Grafana Alloy agent. **Recommendation:** Use stdout JSON for MVP (simpler), and migrate to direct push later.

---

## 5. Loki Label Strategy

Per ADR-005 section 8, only **low-cardinality** fields are Loki labels. High-cardinality values go in JSON log line content.

### Stream Labels (Indexed)

| Label | Sample Values |
|-------|--------------|
| `environment` | `Development`, `Preview`, `Production` |
| `service` | `forge-api`, `build-worker`, `notification-worker` |
| `log_type` | `application`, `build`, `deployment` |
| `level` | `INFO`, `WARN`, `ERROR`, `DEBUG`, `TRACE` |

### Structured Fields (in JSON log line — not indexed)

- `request_id` (UUID)
- `trace_id` (W3C trace string)
- `user_id` (UUID)
- `organization_id` (UUID)
- `project_id` (UUID)
- `deployment_id` (UUID)
- `http_method`
- `http_status`
- `duration_ms`
- `message`

---

## 6. What Must NOT Appear in Logs

Per ADR-005 section 9:

- Plaintext passwords or password hashes
- JWT access tokens or refresh tokens
- Git Personal Access Tokens (PAT)
- Decrypted environment variable values
- Database connection strings
- `Authorization` HTTP header values
- `Cookie` header values
- Sensitive request bodies (e.g., `POST /auth/login` body)

---

## 7. Implementation Tasks

### Cargo Setup
- [ ] Add `tracing`, `tracing-subscriber` to Cargo.toml
- [ ] Decide on collection mechanism: stdout JSON (MVP) vs `tracing-loki` (direct push)

### Tracing Setup
- [ ] Initialize tracing subscriber in `main.rs` before Axum starts
- [ ] Configure JSON format output (`tracing-subscriber` with `fmt::json()`)
- [ ] Configure `env_filter` from `RUST_LOG` environment variable
- [ ] Set service name label from environment variable

### Request ID Middleware
- [ ] Implement Axum middleware that generates a UUID request ID per request
- [ ] Inject request ID into `tracing` span context
- [ ] Return `X-Request-ID` response header

### HTTP Access Logging
- [ ] Log every HTTP request: method, path, status code, duration_ms, user_id (if authenticated)
- [ ] Use `tower-http`'s `TraceLayer` or custom middleware
- [ ] Redact `Authorization` header value before logging

### Application Service Logging Standards
- [ ] Document `tracing::info!`, `tracing::warn!`, `tracing::error!` usage guidelines
- [ ] All service functions should log entry/exit at DEBUG level
- [ ] All errors must be logged with structured fields (no bare strings)
- [ ] All security events must be logged at INFO or WARN level

### Build Worker Log Push
- [ ] Build Worker emits structured log lines to Loki for each pipeline step
- [ ] Each log line includes `deployment_id`, `step`, `level`, `timestamp`, `message`
- [ ] Secrets redacted before log emission (PAT tokens, env var values)

### Loki Query Integration (for Live Build Logs)
- [ ] Implement HTTP client to query Loki `/loki/api/v1/query_range` API
- [ ] LogQL query template: `{service="build-worker"} | json | deployment_id="<uuid>"`
- [ ] Return paginated log lines for `GET /deployments/:id/logs`

### Testing
- [ ] Unit test: tracing subscriber initializes without panic
- [ ] Unit test: request ID middleware adds header
- [ ] Integration test: log lines contain required fields
- [ ] Security test: Authorization header not logged in plaintext
- [ ] Integration test: Loki push succeeds (or stdout check)

---

## 8. Definition of Done

- [ ] Tracing initialized in `main.rs`
- [ ] All log lines are structured JSON
- [ ] Request ID injected per request
- [ ] HTTP access logs emitted per request
- [ ] No secrets appear in log output
- [ ] Build Worker logs include `deployment_id` and `step`
- [ ] Loki query client implemented for stored log retrieval
- [ ] Tests pass

---

## 9. Estimated Effort

**Medium (1–2 days)**

Setting up `tracing` is quick. The Loki push integration or Promtail setup adds complexity. The Build Worker log redaction requires careful implementation.

---

## 10. Recommendations

**Required:**
- Structured JSON logging (not plain text) is required per ADR-005.
- All log lines from the Build Worker must be redacted for secrets.
- The `request_id` field must appear on every HTTP log entry.

**Recommended:**
- For MVP, use stdout JSON + Promtail/Alloy agent rather than `tracing-loki` direct push. This keeps the application simpler and separates concerns.
- Set `RUST_LOG=info` as the default, with `debug` available for development.

**Future Enhancement:**
- OpenTelemetry distributed tracing (`opentelemetry` crate) for cross-service trace correlation.
- Grafana dashboard for application metrics derived from log streams.
