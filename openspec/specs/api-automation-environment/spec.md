# Spec: API Automation Environment

## Purpose

Provides a standardized Postman-compatible environment configuration file for local development and automated API execution across the Forge platform endpoints.

## Requirements

### Requirement: Root Environment Configuration File
The system SHALL provide a standard Postman environment file named `environment.json` at the root of the project directory. The file MUST comply with the Postman Environment format schema (including `id`, `name`, `values`, and `_postman_variable_scope`).

#### Scenario: Inspecting root environment file
- **WHEN** an automated test runner or developer inspects `environment.json` in the project root
- **THEN** the file is valid JSON adhering to the Postman environment specification with variable scope set to `environment`.

### Requirement: Core Variables for Local Development
The `environment.json` file MUST contain all variable definitions necessary to execute the Forge Platform API collections without unresolved variable errors. This MUST include `baseUrl` set to `http://localhost:9000/api/v1`, placeholder variables for `bearerToken`, `refreshToken`, and `apiKey`, and default admin test credentials `adminEmail` (`admin1@forge.local`) and `adminPassword` (`Password123!`).

#### Scenario: Running requests against local API
- **WHEN** requests referencing `{{baseUrl}}`, `{{bearerToken}}`, or auth credentials are run with this environment
- **THEN** all variable references resolve successfully to local development values.

### Requirement: Postman Local Workspace Environment Alignment
The environment configuration MUST be aligned with the Postman v12 local workspace format under `postman/postman/environments/` so that users opening the project in Postman v12 or executing tests with Postman CLI/Newman have synchronized variable scopes.

#### Scenario: Running automated collection runs in Postman or Newman
- **WHEN** Newman or Postman CLI executes collections using either `environment.json` or workspace environment files
- **THEN** identical variable resolutions occur across both execution tools.
