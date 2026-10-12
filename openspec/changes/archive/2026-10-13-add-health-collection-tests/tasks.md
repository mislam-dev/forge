# Tasks

## 1. Liveness Probe Test Suite

- [x] 1.1 Attach automated post-response test script to `postman/postman/collections/Forge Platform API/health/live/Liveness probe — check process health.request.yaml` validating HTTP 200 OK, Content-Type, and liveness payload schema (`status`, `service`, `version`, `environment`, `timestamp`), and verify valid YAML formatting.

## 2. Readiness Probe Test Suite

- [x] 2.1 Attach automated post-response test script to `postman/postman/collections/Forge Platform API/health/ready/Readiness probe — check dependency readiness.request.yaml` validating status codes (200 OK or 503 Service Unavailable), Content-Type, and dependency check breakdown (`database`, `job_queue`, `container_runtime`), and verify valid YAML formatting.

## 3. Deep Health Check Test Suite

- [x] 3.1 Attach automated post-response test script to `postman/postman/collections/Forge Platform API/health/deep/Deep health check — all internal services and dependencies.request.yaml` validating status codes (200 OK, 401 Unauthorized, 403 Forbidden), diagnostic metrics (`status`, `uptime_seconds`, `service`, `version`, `timestamp`), and standardized error response contract, and verify valid YAML formatting.

## 4. Verification and Validation

- [x] 4.1 Validate YAML syntax across all modified Postman health request files to ensure valid structure without indentation errors.
- [x] 4.2 Run `openspec validate` on the change to verify artifact integrity and schema compliance.
