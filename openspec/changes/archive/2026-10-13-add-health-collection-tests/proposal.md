# Proposal

## Why

The Forge Platform API health probe endpoints (`/health/live`, `/health/ready`, `/health/deep`) in the Postman collection currently lack test assertion scripts. Without automated test assertions verifying status codes, response headers, and response payloads against the OpenAPI specification and runtime contracts, automated test runners (such as Newman or Postman CLI) cannot validate whether health probes and dependency diagnostic checks return correct statuses and schemas during continuous integration and deployment health checks.

## What Changes

- Add automated Postman test assertion scripts (`scripts` with `type: afterResponse`) to all 3 health collection requests under `postman/postman/collections/Forge Platform API/health/`:
  - `GET /health/live`: Validate 200 OK status, Content-Type header (`application/json`), valid JSON body, and payload contract conforming to the liveness probe schema (`status: "healthy"`, `service: "forge-platform"`, `version`, `environment`, and ISO 8601 UTC `timestamp`).
  - `GET /health/ready`: Validate acceptable readiness status codes (200 OK when healthy, or 503 Service Unavailable when degraded/unhealthy), Content-Type header (`application/json`), valid JSON body, and dependency check breakdown (`database`, `job_queue`, `container_runtime` status and latency).
  - `GET /health/deep`: Validate authenticated status codes (200 OK when authorized as admin, 401 Unauthorized when unauthenticated, or 403 Forbidden when caller lacks admin permissions), Content-Type header (`application/json`), valid JSON body, diagnostic metrics (`uptime_seconds`, `timestamp`, `service`, `version`), or standardized `ErrorResponse` schema on auth failure.
- Ensure all test assertions gracefully parse JSON responses and provide clear failure messages.

## Capabilities

### New Capabilities
- `health-collection-tests`: Defines automated Postman test assertion suites for health probe endpoints (`/health/live`, `/health/ready`, `/health/deep`), validating response statuses, headers, and schemas against OpenAPI contracts.

### Modified Capabilities
<!-- None -->

## Impact

- Collection request definition files under `postman/postman/collections/Forge Platform API/health/` will be updated with `afterResponse` test scripts.
- No backend Rust code or database schema changes are required.
- Automated API test runners (Newman, Postman CLI, CI pipelines) can now automatically verify health probes and diagnostic contracts.
