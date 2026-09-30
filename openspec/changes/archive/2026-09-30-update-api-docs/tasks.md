# Tasks

## 1. OpenAPI Specification Updates (`docs/system/05-api/openapi.yaml`)

- [x] 1.1 Update Project Repositories API definitions in `docs/system/05-api/openapi.yaml`: remove unimplemented paths (`/validate`, `/clone`, `/commit`, `/branch`, `/branches`), add `PATCH` and `DELETE` under `/projects/{id}/repository`, remove `auth_type` from connect and update request schemas, update status enum to `connected` and `disconnected`, and verify syntax.
- [x] 1.2 Update Project Environment Variables API definitions in `docs/system/05-api/openapi.yaml`: add `POST /projects/{id}/env-vars/bulk`, switch update method from `PUT` to `PATCH` under `/projects/{id}/env-vars/{env_id}`, remove unimplemented `/decrypt` endpoint, update `environment` enum to lowercase (`development`, `staging`, `production`), adjust request/response schemas to match Axum DTOs, and verify syntax.
- [x] 1.3 Update Global Server Configuration in `docs/system/05-api/openapi.yaml`: update default port to `9000`, document allowed and exposed CORS headers (`x-request-id`, `organization-id`), and verify syntax.

## 2. API Companion Documentation Updates (`docs/API_REQUESTS_SUMMARY.md`)

- [x] 2.1 Update Repository Management section in `docs/API_REQUESTS_SUMMARY.md`: remove `auth_type`, document idempotent reconnect logic for `POST /projects/{id}/repository`, add `PATCH` and `DELETE` documentation, and verify markdown consistency.
- [x] 2.2 Update Environment Variables section in `docs/API_REQUESTS_SUMMARY.md`: document `POST /projects/{id}/env-vars/bulk` with per-item `environment` scoping, note lowercase environment enums, update `PATCH` method, and verify response schema details.
- [x] 2.3 Update Architecture and Conventions section in `docs/API_REQUESTS_SUMMARY.md`: update default port (9000) and CORS header table, and verify links.

## 3. Validation and Consistency Verification

- [x] 3.1 Validate `docs/system/05-api/openapi.yaml` with a YAML validator script to ensure valid syntax and references.
- [x] 3.2 Verify that all endpoints and DTO definitions match between `openapi.yaml`, `docs/API_REQUESTS_SUMMARY.md`, and the OpenSpec delta specifications.
