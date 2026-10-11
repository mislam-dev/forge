# Proposal

## Why

The OpenAPI specification has been imported into the project's Postman v12 workspace to test and automate Forge Platform API flows. However, automated collection runs and API execution require an environment configuration defining target URLs, auth tokens, and session credentials that match the local Forge backend.

## What Changes

- Add a standardized Postman `environment.json` file at the root of the project configured for Forge local development.
- Populate key environment variables used across collection endpoints: `baseUrl` (`http://localhost:9000/api/v1`), `bearerToken`, `refreshToken`, `apiKey`, and default test credentials (`adminEmail`, `adminPassword`).
- Synchronize or mirror the environment configuration with `postman/postman/environments/Local Development.environment.yaml` so Postman desktop, Postman v12 Local Workspace, and automated test runners (such as Newman or Postman CLI) can seamlessly execute requests.

## Capabilities

### New Capabilities
- `api-automation-environment`: Defines the environment schema, variable hierarchy, and execution contexts required for automated API testing and Postman collection runs against the Forge backend.

### Modified Capabilities
<!-- None -->

## Impact

- Root workspace: Adds `environment.json` at repository root.
- Postman workspace: Adds environment definition under `postman/postman/environments/`.
- Automation: Enables CLI tools (e.g., `newman run ... -e environment.json`) and Postman v12 to run requests without missing unresolved variables.
