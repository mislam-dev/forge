# Spec: API Automation Testing

## Purpose

Enables automated headless Postman API test execution, RBAC multi-role matrix verification across tenancy boundaries, visual test reporting with interactive HTML dashboards, and mock server documentation generation for the Forge platform.

## Requirements

### Requirement: Headless Postman Collection Execution via Justfile
The system SHALL provide task runner recipes in `justfile` (`just test-api` and `just test-api-env <env>`) that execute the Forge Platform API collections headlessly using Newman or Postman CLI against the local API server or remote hosts. The execution runner SHALL automatically resolve the collection from the project Postman workspace and execute with the specified environment configuration.

#### Scenario: Running default collection test against local development
- **WHEN** a developer executes `just test-api`
- **THEN** the collection runner executes the Forge Platform API test collection against the active local development environment (`environment.json`) and outputs test results to standard output.

#### Scenario: Running collection test with explicit environment selection
- **WHEN** a developer executes `just test-api-env admin-with-org`
- **THEN** the collection runner resolves `postman/environments/admin-with-org.environment.json` and runs the collection using the Admin credentials and organization scoping.

### Requirement: Multi-Role and Tenancy RBAC Matrix Execution
The system SHALL provide an automated matrix test runner (`just test-api-matrix`) that executes the API collection across all 6 role and tenancy environments (`Admin - With Organization`, `Admin - Without Organization`, `Developer - With Organization`, `Developer - Without Organization`, `Viewer - With Organization`, `Viewer - Without Organization`). The matrix runner SHALL evaluate RBAC policies, verifying that role restrictions (such as Viewer receiving 403 Forbidden on create, update, and delete endpoints) and organization isolation boundaries are enforced.

#### Scenario: Running complete RBAC matrix across all 6 environments
- **WHEN** a developer executes `just test-api-matrix`
- **THEN** the test runner sequentially iterates through each of the 6 environment configurations, evaluates assertion outcomes for each role tier, and outputs an aggregated matrix summary table indicating status and pass/fail counts for each environment.

#### Scenario: Detecting unauthorized access during matrix run
- **WHEN** an endpoint permits an operation that should be forbidden for a Viewer or Developer role in the matrix run
- **THEN** the matrix runner marks that environment iteration as failed and exits with a non-zero exit code.

### Requirement: Interactive Visual HTML Test Reporting
The test automation runner SHALL support generating rich, interactive HTML reports using `newman-reporter-htmlextra` via `just test-api-report`. The generated report SHALL document overall test counts, pass/fail status, detailed request headers and payloads, response bodies, and visual assertion breakdowns saved into `reports/postman/`.

#### Scenario: Generating visual HTML report for test execution
- **WHEN** a user executes `just test-api-report`
- **THEN** an interactive HTML report is created in `reports/postman/` containing interactive tabs for total requests, failures, and response payloads.

#### Scenario: Inspecting failed assertion diagnostics in HTML report
- **WHEN** a request fails an assertion during a reported run
- **THEN** the generated HTML report highlights the failure, displays the expected vs actual values, and includes the full HTTP response body for debugging.

### Requirement: Postman Mock Server and Documentation Generation
The system SHALL provide Postman mock server configurations and interactive documentation resources within the Postman v12 Local Workspace format, derived directly from `postman/postman/specs/Forge Platform API/openapi.yaml` and collection examples. The mock resources SHALL be registered under `postman/postman/mocks/` and referenced in `postman/.postman/resources.yaml`, enabling frontend developers and clients to develop against mock endpoints before backend services are running.

#### Scenario: Inspecting mock server resource registration
- **WHEN** `postman/.postman/resources.yaml` is inspected
- **THEN** mock server definitions are declared under `localResources.mocks` and mapped in `resourceNameMappings.mocks`.

#### Scenario: Developing against mock responses
- **WHEN** a client or test runner targets the configured Postman mock server
- **THEN** endpoints return the example response payloads declared in the OpenAPI specification without requiring a running Rust backend service.
