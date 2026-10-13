# Proposal

## Why

The Forge platform has comprehensive Postman collections, contract assertions, and 6 role & tenancy environments defined, but lacks automated headless execution and developer mocking workflows. Developers currently have no automated mechanism to execute these collections headlessly via the terminal/justfile, verify RBAC security isolation across the 6 role matrices, generate visual test run diagnostics, or consume mock endpoints before the backend service is deployed.

Providing standardized CLI recipes via `justfile` and `newman`, matrix automation scripts across all tenant/role combinations, interactive HTML reporting with `newman-reporter-htmlextra`, and Postman mock server and documentation publishing instructions completes the end-to-end API testing lifecycle.

## What Changes

- Add collection bundling and headless execution support via `newman` and Postman CLI, converting/assembling Postman v12 Local Workspace collection definitions into executable runs.
- Add justfile automation recipes (`test-api`, `test-api-env`, `test-api-matrix`, `test-api-report`) for running API collections headlessly against local and remote environments.
- Introduce a Multi-Role Matrix runner that loops through all 6 tenant & role environments (`Admin With Org`, `Admin Without Org`, `Developer With Org`, `Developer Without Org`, `Viewer With Org`, `Viewer Without Org`) to automate RBAC boundary verification (e.g. ensuring Viewer receives 403 Forbidden on mutations while Admin succeeds).
- Configure interactive visual reporting using `newman-reporter-htmlextra` with detailed request/response payloads, headers, and visual pass/fail graphs.
- Establish Postman Mock Server and API Documentation specifications and configurations within the Postman v12 workspace (`postman/postman/mocks/` and workspace documentation) backed by `openapi.yaml`.

## Capabilities

### New Capabilities
- `api-automation-testing`: Covers headless collection execution via CLI and Justfile, multi-role RBAC matrix automation across all 6 tenant/role environments, visual HTML reporting with `htmlextra`, and Postman v12 mock server/documentation generation from collection and OpenAPI specs.

### Modified Capabilities
<!-- No existing spec requirements are modified. Existing environment and collection test specifications remain authoritative contracts. -->

## Impact

- **Tooling & Dependencies**: Node.js ecosystem dependencies (`newman`, `newman-reporter-htmlextra`, and bundling script dependencies).
- **Task Runner**: `justfile` receives new recipes for running API tests, role matrix suites, and HTML report generation.
- **Postman Workspace**: Integrates mock server definition under `postman/postman/mocks/` and updates `postman/.postman/resources.yaml`.
- **Testing**: Enables automated RBAC security policy regression testing across all roles and organization states.
