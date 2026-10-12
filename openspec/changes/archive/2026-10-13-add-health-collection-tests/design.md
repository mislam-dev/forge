# Design

## Context

The Forge Platform API Postman collection organizes requests into modular directories containing YAML definitions (`$kind: http-request`). Currently, the health probe endpoints under `postman/postman/collections/Forge Platform API/health/` lack post-response assertion scripts (`scripts` with `type: afterResponse`).

Other modules (such as `auth` and `organizations`) implement standardized test assertion suites validating status codes, Content-Type headers, JSON validity, and schema conformance for both successful responses and error states. This design aligns the health probe requests with those established patterns.

## Goals / Non-Goals

**Goals:**
- Provide robust, automated Postman test assertion scripts for:
  - `GET /health/live` (Liveness probe)
  - `GET /health/ready` (Readiness probe)
  - `GET /health/deep` (Deep health check)
- Verify HTTP status codes, Content-Type (`application/json`), valid JSON response parsing, and field contracts matching the OpenAPI specification.
- Handle stateful/conditional responses gracefully (e.g., 200 vs 503 for readiness probes, and 200 vs 401/403 for deep health diagnostics).

**Non-Goals:**
- Modifying backend Rust route handlers, middleware, or domain logic.
- Altering existing collection request URLs, query parameters, headers, or authentication settings.
- Adding new health routes or custom Newman reporter plugins.

## Decisions

### Decision: Dual-Status and Conditional Schema Assertions
- **Choice**: Structure test scripts to assert acceptable HTTP status codes (e.g., 200 for liveness; 200 or 503 for readiness; 200, 401, or 403 for deep health check) and branch payload validation accordingly.
- **Rationale**: In automated test environments or pipeline stages, dependencies may not yet be initialized (triggering 503 on readiness), or tokens may be absent/expired (triggering 401/403 on deep check). By asserting both success contracts and standard error schemas, test runs provide informative assertion reports rather than brittle script crashes.
- **Alternatives Considered**: Expecting only HTTP 200 OK. Rejected because it causes misleading failures when tests are run against uninitialized or unauthenticated environments.

### Decision: Standardized Property and Type Assertions
- **Choice**: Validate specific expected fields and data types rather than exact string equality for dynamic properties:
  - `status`: String (`"healthy"`, `"ready"`, or `"not_ready"`)
  - `service`: Non-empty string (`"forge-platform"`)
  - `timestamp`: Valid ISO 8601 UTC date string parseable by `Date.parse()`
  - `checks`: Object containing dependency keys (`database`, `job_queue`, `container_runtime`) with `status` and `latency_ms` properties
  - `uptime_seconds`: Non-negative number
- **Rationale**: Ensures contract compliance without failing on non-deterministic data like timestamps or latencies.

## Risks / Trade-offs

- **[Risk]** Test runner fails when response body is not JSON (e.g., raw gateway 502/504 error) → **[Mitigation]** Wrap `pm.response.json()` in a `try...catch` block and fail with a clear assertion message (`"Response body is not valid JSON"`).
- **[Risk]** Collection YAML formatting corruption when adding multiline script blocks → **[Mitigation]** Format scripts using YAML block scalars (`code: |-`) matching existing collection conventions in `auth` and `organizations`.
