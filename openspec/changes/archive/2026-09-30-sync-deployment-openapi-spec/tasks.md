# Tasks

## 1. OpenAPI Specification Updates

- [x] 1.1 Update `docs/system/05-api/openapi.yaml` public collection endpoints: change trigger deployment from `POST /deployments` to `POST /projects/{id}/deployments` and list deployments to `GET /projects/{id}/deployments` with existing pagination parameters (`page`, `limit`), `status`, and `branch` query parameters. Verify parameters and operation IDs.
- [x] 1.2 Update `docs/system/05-api/openapi.yaml` item-level endpoints: update single deployment retrieval to `GET /projects/{id}/deployments/{deployment_id}` and redeploy to `POST /projects/{id}/deployments/{deployment_id}/redeploy` with `{id}` and `{deployment_id}` path parameters. Verify path definitions and status codes.
- [x] 1.3 Update `docs/system/05-api/openapi.yaml` rollback endpoint: change to `POST /projects/{id}/deployments/rollback` with `{id}` path parameter, removing root `/projects/{id}/rollback`. Verify response status code 201 Created.
- [x] 1.4 Update `docs/system/05-api/openapi.yaml` internal status endpoint: change from `PATCH /deployments/{id}/status` to `PUT /projects/internal/deployments/{deployment_id}/status`, adding `x-service-token` header and `{deployment_id}` parameter. Verify method and required header.
- [x] 1.5 Update OpenAPI component schemas in `docs/system/05-api/openapi.yaml`: update `TriggerDeploymentRequest`, `UpdateDeploymentStatusRequest`, and `DeploymentResponse` to match Rust DTOs (`src/modules/projects/deployments/dto/`). Verify schema field types and status enum values.

## 2. API Documentation Companion Updates

- [x] 2.1 Update Section 12 (Deployments & Build Worker APIs) in `docs/API_REQUESTS_SUMMARY.md` with the synchronized endpoints, methods, headers, and request/response payloads while retaining `page` and `limit`. Verify documentation consistency against OpenAPI.
- [x] 2.2 Update Section 16 (Complete API Endpoint Summary Table) in `docs/API_REQUESTS_SUMMARY.md` to reflect the updated deployment paths and HTTP methods. Verify summary rows match endpoints.

## 3. Validation

- [x] 3.1 Validate `docs/system/05-api/openapi.yaml` YAML syntax and structure using a YAML parser or validation script to ensure valid syntax.
- [x] 3.2 Run cargo deployment test suite (`cargo test --test deployments_tests`) to verify endpoint behavior and test suite passing.
