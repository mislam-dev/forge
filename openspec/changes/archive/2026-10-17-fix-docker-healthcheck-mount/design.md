# Design

## Context

The Forge backend is built with Axum. In `src/app/app.rs`, the router hierarchy is constructed by creating `root_app_router` and nesting it under `API_V1_PATH` (`/api/v1`). Because `root_app_router` contains `.nest("/health", health_router())`, the health endpoints are only accessible under `/api/v1/health`.

Orchestration platforms, Docker daemon healthchecks, and Kubernetes liveness probes require root-level endpoints (`/health`, `/health/live`, `/health/ready`). Furthermore, `tests/health_tests.rs` tests expect root-level `/health/live`, `/health/ready`, and `/health/deep`.

The production Docker configuration (`Dockerfile` and `docker/Dockerfile.prod`) currently specifies:
```dockerfile
HEALTHCHECK --interval=15s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:9000/health || exit 1
```
Because `/health` returns 404, the container never transitions to the `healthy` status. Additionally, using `/health/live` ensures the healthcheck tests process responsiveness without failing during database cold starts.

## Goals / Non-Goals

**Goals:**
- Mount `health_router()` at root `/health` alongside `/api/v1/health` in `src/app/app.rs`.
- Ensure all public probes (`/health`, `/health/live`, `/health/ready`) respond with HTTP 200 without requiring JWT authentication.
- Update `Dockerfile` and `docker/Dockerfile.prod` HEALTHCHECK instructions to target `http://localhost:9000/health/live || exit 1`.
- Verify `tests/health_tests.rs` passes its root-level probe assertions.

**Non-Goals:**
- Altering the internal implementation of `health_liveness_probe` or `health_readiness_probe` in `src/modules/health/handlers.rs`.
- Modifying authentication requirements for protected health endpoints (`/health/details`, `/health/deep`).
- Introducing backward-compatibility shims for other `/api/` domain routes (addressed separately in T-003).

## Decisions

### Decision 1: Mount health router at both `/health` and `/api/v1/health` with shared middleware
- **Approach**: In `src/app/app.rs`, structure the router so that `api_v1_router` contains the versioned endpoints (including `/health`), while the outer application router also nests `health_router()` at `/health`. Common middleware (CORS, Request Timeout, Logging, Fallback) and `.with_state(app_state)` are applied at the outermost level.
- **Alternatives Considered**:
  - *Mounting only at `/health`*: Would break clients expecting the versioned `/api/v1/health` endpoints.
  - *HTTP 301/308 Redirect*: Unnecessary latency and complex probe handling for automated orchestrators that do not follow redirects.

### Decision 2: Target `/health/live` for Docker HEALTHCHECK
- **Approach**: Configure Docker container healthcheck instruction as:
  `CMD curl -f http://localhost:9000/health/live || exit 1`
- **Alternatives Considered**:
  - *`http://localhost:9000/health`*: Checks database status; if the database is temporarily busy or reconnecting, the Docker container could be marked unhealthy and restarted unnecessarily.
  - *`http://localhost:9000/health/ready`*: Readiness probe suited for Kubernetes ingress traffic routing rather than container process liveness.

### Decision 3: Update both `Dockerfile` and `docker/Dockerfile.prod`
- **Approach**: Synchronize line 59 in both Dockerfiles to keep production container specifications consistent across standalone builds (`Dockerfile`) and Docker Compose (`docker/Dockerfile.prod`).

## Risks / Trade-offs

- **[Risk] Route collision or missing middleware on root `/health`** → **Mitigation**: Nest `/health` at the top-level router before `.layer(...)` and `.with_state(app_state)` so that logging, timeout, and state injection are applied uniformly.
- **[Risk] Docker container healthcheck flapping during high load** → **Mitigation**: Using `/health/live` avoids database queries and dependency checks, evaluating purely process responsiveness in <1ms.
