# Module 16 — Live Build Logs

> **Module Type:** Sub-Module (Deployments)
> **Priority:** P2 — Post-MVP
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **Source Docs:** [Live Build Logs Module](../../modules/deployments/live-build-logs-module.md)

---

## 1. Module Overview

### Purpose

The Live Build Logs sub-module provides **real-time build log streaming** to clients via Server-Sent Events (SSE), and **stored log retrieval** from Grafana Loki for completed deployments.

### Responsibilities

- SSE stream: real-time log lines from RabbitMQ topic exchange during active deployments
- Stored logs: paginated retrieval from Loki using LogQL for any deployment
- Log download: full log stream as `.log` file download
- Log search: LogQL pattern matching on stored logs

### Scope

**Included:**
- `GET /deployments/:id/logs` — list stored logs (paginated, Loki)
- `GET /deployments/:id/logs/stream` — SSE real-time stream
- `GET /deployments/:id/logs/download` — download as `.log` file
- `GET /deployments/:id/logs/search?q={pattern}` — search logs

**Excluded:**
- Log emission (Build Worker)
- Deployment status (Deployments module)

---

## 2. Dependencies

### Depends On
- **Build Worker** (emits log lines to RabbitMQ + Loki)
- **Loki** (stored log retrieval via LogQL)
- **RabbitMQ** (live stream via topic exchange)
- **Deployments** (validate deployment_id, check ownership)
- **Authentication**

---

## 3. API Implementation

### GET /deployments/:id/logs

- **Auth:** JWT + project member
- **Query params:** `page`, `per_page`, `level` (optional)
- **Service logic:** Query Loki `GET /loki/api/v1/query_range` with LogQL: `{service="build-worker"} | json | deployment_id="{id}"`
- **Response:** `200 { message, data: [{ timestamp, level, step, message }], meta: pagination }`

### GET /deployments/:id/logs/stream (SSE)

- **Auth:** JWT + project member
- **Protocol:** Server-Sent Events (text/event-stream)
- **Service logic:**
  1. Validate deployment_id and check project membership
  2. Check if deployment is in non-terminal state (stream only makes sense for active deployments)
  3. Create ephemeral RabbitMQ queue: `forge.logs.client.{uuid}`, bind to `forge.logs` topic exchange with routing key `deployment.{deployment_id}`
  4. Open SSE connection (infinite stream)
  5. Forward each RabbitMQ message as SSE event: `data: {json_log_line}\n\n`
  6. On deployment terminal state (detect via periodic polling or Deployment event): send SSE `event: done\ndata: {}\n\n`, close stream
  7. On client disconnect: delete ephemeral queue, release channel
- **Response headers:** `Content-Type: text/event-stream`, `Cache-Control: no-cache`, `Connection: keep-alive`

### GET /deployments/:id/logs/download

- **Auth:** JWT + project member
- **Service logic:** Query full log from Loki (no pagination), format as plain text
- **Response:** `Content-Disposition: attachment; filename="deployment-{id}.log"`, plain text body

### GET /deployments/:id/logs/search

- **Auth:** JWT + project member
- **Query params:** `q` (search pattern), `page`, `per_page`
- **Service logic:** LogQL query: `{service="build-worker"} | json | deployment_id="{id}" | message=~"{q}"`
- **Response:** `200 { message, data: [matching log lines], meta: pagination }`

---

## 4. SSE Protocol

```
GET /deployments/{id}/logs/stream HTTP/1.1
Accept: text/event-stream

HTTP/1.1 200 OK
Content-Type: text/event-stream
Cache-Control: no-cache
Connection: keep-alive

data: {"timestamp":"...","level":"INFO","step":"clone","message":"Cloning repository..."}

data: {"timestamp":"...","level":"INFO","step":"build","message":"Step 1/5: FROM node:20"}

event: done
data: {}
```

---

## 5. Log Modes

| Mode | Availability | Source | Mechanism |
|------|-------------|--------|-----------|
| Live streaming | While deployment is active | RabbitMQ topic | SSE push |
| Stored retrieval | Any time | Grafana Loki | LogQL query |
| Download | Any time | Grafana Loki | Full stream fetch |
| Search | Any time | Grafana Loki | LogQL pattern |

---

## 6. Implementation Tasks

### Loki HTTP Client
- [ ] Implement `LokiClient` with `query_range` method
- [ ] Parse Loki response format into `LogLine` structs
- [ ] Handle pagination via `start`/`end` timestamps

### SSE Handler
- [ ] Implement Axum SSE handler using `axum::response::sse::Sse`
- [ ] Create ephemeral RabbitMQ queue on connection
- [ ] Forward RabbitMQ messages as SSE events
- [ ] Detect terminal state and close stream
- [ ] Cleanup ephemeral queue on client disconnect

### Handlers
- [ ] Implement `GET /logs` handler (Loki query)
- [ ] Implement `GET /logs/stream` SSE handler
- [ ] Implement `GET /logs/download` handler
- [ ] Implement `GET /logs/search` handler
- [ ] Register routes in router

### Testing
- [ ] Unit test: LogQL query construction
- [ ] Integration test: SSE connection established, log lines forwarded
- [ ] Integration test: stored log retrieval from Loki
- [ ] Integration test: search returns matching lines

---

## 7. Definition of Done

- [ ] `GET /logs` returns stored logs from Loki
- [ ] `GET /logs/stream` SSE works during active deployment
- [ ] SSE stream closes on terminal state
- [ ] Ephemeral queue cleaned up on client disconnect
- [ ] Log download returns plain text file
- [ ] Log search returns matching lines
- [ ] All tests pass

---

## 8. Estimated Effort

**Large (3–4 days)**

SSE with RabbitMQ integration and Loki query client require careful async implementation.

---

## 9. Recommendations

**Required:**
- Ephemeral SSE queues must be auto-deleted on client disconnect to prevent queue accumulation.
- SSE stream must close gracefully on deployment terminal state.

**Recommended:**
- Max concurrent SSE streams per deployment: configurable (recommended: 10)
- Log lines should be buffered and sent as batches during high-throughput builds

**Future Enhancement:**
- WebSocket alternative to SSE for bidirectional communication.
- Log line filtering by level in the SSE stream.
