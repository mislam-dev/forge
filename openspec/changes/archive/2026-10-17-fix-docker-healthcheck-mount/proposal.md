# Proposal

## Why

In containerized deployments and orchestration platforms, health probes must be accessible at root-level endpoints (`/health`, `/health/live`, `/health/ready`) without authentication or API version prefixing. Currently, `src/app/app.rs` nests all routes—including `health_router()`—under `/api/v1`, causing requests to `/health`, `/health/live`, and `/health/ready` to return HTTP 404 Not Found.

Consequently:
1. The container HEALTHCHECK instructions in `Dockerfile` and `docker/Dockerfile.prod` fail, preventing containers from reaching the `healthy` state in production and Docker Compose.
2. The healthcheck probe in `Dockerfile` targets `http://localhost:9000/health` rather than the dedicated process liveness probe `http://localhost:9000/health/live`, causing container initialization to fail if backing services are briefly initializing.
3. Integration tests in `tests/health_tests.rs` targeting `/health/live`, `/health/ready`, and `/health/deep` fail with 404.

Resolving this (task T-002) is a P0 production blocker required for stable container orchestration and passing test suites.

## What Changes

- **Mount Root `/health` Router in `src/app/app.rs`**: Mount `health_router()` directly at the root `/health` on the top-level application router alongside the nested `/api/v1/health` router.
- **Ensure Public Access to Health Probes**: Verify that root-level `/health`, `/health/live`, and `/health/ready` are accessible without JWT authentication tokens.
- **Update Dockerfile HEALTHCHECK Instructions**: Update line 59 in both `Dockerfile` and `docker/Dockerfile.prod` to query `http://localhost:9000/health/live || exit 1`.
- **Verify Health Integration Tests**: Ensure `tests/health_tests.rs` passes its root-level probe assertions (`/health/live`, `/health/ready`, `/health/deep`).

## Capabilities

### New Capabilities
<!-- None: No new domain capabilities are introduced. -->

### Modified Capabilities
- `health-probes`: Ensure health probe endpoints (`/health`, `/health/live`, `/health/ready`, `/health/deep`, `/health/details`) are mounted and accessible at root `/health` as well as `/api/v1/health`.
- `docker-environments`: Update the production Docker container specification to configure the container healthcheck command to target `http://localhost:9000/health/live`.

## Impact

- **Production Code**: `src/app/app.rs` (router hierarchy and root `/health` mount).
- **Container Configurations**: `Dockerfile` and `docker/Dockerfile.prod` (HEALTHCHECK instruction).
- **Tests**: `tests/health_tests.rs` tests for root health probe accessibility will turn green.
- **Documentation**: Task `T-002` in `docs/03-TASKS.md` will be satisfied.
