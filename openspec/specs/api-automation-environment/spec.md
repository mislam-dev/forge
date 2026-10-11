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

### Requirement: Organization ID Header Parameterization across Collections
All collection requests within the Forge Platform API that require or accept tenant scoping via the `Organization-ID` header SHALL reference the environment variable `{{organization_id}}` as their header value rather than hardcoded or static UUID values.

#### Scenario: Inspecting tenant-scoped request headers
- **WHEN** any collection request under `postman/postman/collections/Forge Platform API/` that includes the `Organization-ID` header is inspected
- **THEN** the `Organization-ID` header value is configured as `"{{organization_id}}"`
- **AND** the header retains `disabled: false`, description, and UUID schema metadata.

#### Scenario: Dynamic variable resolution during execution
- **WHEN** requests are executed within Postman or Newman against a configured environment containing `organization_id`
- **THEN** the `Organization-ID` header dynamically resolves to the active environment's `organization_id` value.

### Requirement: Multi-Role and Tenancy Matrix Environments
The system SHALL provide 6 distinct Postman environment configurations combining user roles (`admin`, `developer`, `viewer`) and organization tenancy states (with organization and without organization):
1. `Admin - With Organization`: contains admin credentials (`admin1@forge.local`), test password (`Password123!`), and `organization_id` set to the standard test organization UUID (`00000000-0000-0000-0000-000000000001`).
2. `Admin - Without Organization`: contains admin credentials (`admin1@forge.local`), test password (`Password123!`), and `organization_id` set to an empty string (`""`).
3. `Developer - With Organization`: contains developer credentials (`dev1@forge.local`), test password (`Password123!`), and `organization_id` set to the standard test organization UUID (`00000000-0000-0000-0000-000000000001`).
4. `Developer - Without Organization`: contains developer credentials (`dev1@forge.local`), test password (`Password123!`), and `organization_id` set to an empty string (`""`).
5. `Viewer - With Organization`: contains viewer credentials (`viewer1@forge.local`), test password (`Password123!`), and `organization_id` set to the standard test organization UUID (`00000000-0000-0000-0000-000000000001`).
6. `Viewer - Without Organization`: contains viewer credentials (`viewer1@forge.local`), test password (`Password123!`), and `organization_id` set to an empty string (`""`).

Each of these environments MUST define `baseUrl`, `bearerToken`, `refreshToken`, `apiKey`, `userEmail`, `userPassword`, `testOrgId`, `testProjectId`, and `organization_id`.

#### Scenario: Running collection requests as an Admin with organization
- **WHEN** requests are executed using the `Admin - With Organization` environment
- **THEN** authentication requests use `admin1@forge.local` credentials and `Organization-ID` header resolves to `00000000-0000-0000-0000-000000000001`.

#### Scenario: Running collection requests as a Developer without organization
- **WHEN** requests are executed using the `Developer - Without Organization` environment
- **THEN** authentication requests use `dev1@forge.local` credentials and `Organization-ID` header resolves to an empty value (`""`).

#### Scenario: Running collection requests as a Viewer with organization
- **WHEN** requests are executed using the `Viewer - With Organization` environment
- **THEN** authentication requests use `viewer1@forge.local` credentials and `Organization-ID` header resolves to `00000000-0000-0000-0000-000000000001`.

### Requirement: Development and Production Tier Environments
The system SHALL provide 2 dedicated deployment lifecycle environment configurations:
1. `Development`: configured for local API server testing with `baseUrl` set to `http://localhost:9000/api/v1`, placeholder authentication tokens, default test credentials, and test IDs.
2. `Production`: configured for production API environments with `baseUrl` set to a production host endpoint (`https://api.forge.local/api/v1`), empty placeholder tokens marked as secret, and unpopulated organization/user credentials.

#### Scenario: Running requests against Development tier
- **WHEN** collections are executed using the `Development` environment
- **THEN** `baseUrl` resolves to `http://localhost:9000/api/v1` and standard local test values are available.

#### Scenario: Inspecting Production tier configuration
- **WHEN** the `Production` environment is inspected
- **THEN** `baseUrl` targets the production host URL, and credential/secret fields (`bearerToken`, `refreshToken`, `apiKey`) are configured as blank secrets.

### Requirement: Workspace Resource Registration for Environments
All 8 environments MUST be defined in Postman v12 workspace format under `postman/postman/environments/*.environment.yaml` and registered in `postman/.postman/resources.yaml` under `localResources.environments` and `resourceNameMappings.environments`.

#### Scenario: Inspecting Postman workspace environment resources
- **WHEN** `postman/.postman/resources.yaml` is evaluated
- **THEN** all 8 environments (Admin With Org, Admin Without Org, Developer With Org, Developer Without Org, Viewer With Org, Viewer Without Org, Development, Production) are declared under `localResources.environments` and mapped in `resourceNameMappings.environments`.
