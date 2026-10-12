# Spec Delta: API Automation Environment

## ADDED Requirements

### Requirement: Login Endpoint Dynamic Credential Parameterization across Collections
The collection request for authenticating user credentials (`POST /auth/login` - `Authenticate user credentials`) SHALL reference the environment variables `{{email}}` and `{{password}}` in its JSON request body content rather than static or hardcoded credentials.

#### Scenario: Inspecting login request body
- **WHEN** the `POST /auth/login` request in `postman/postman/collections/Forge Platform API/auth/login/Authenticate user credentials.request.yaml` is inspected
- **THEN** the JSON body content defines `"email": "{{email}}"` and `"password": "{{password}}"`.

#### Scenario: Dynamic credential resolution during collection execution
- **WHEN** the authentication request is executed within Postman or Newman with an active environment selected
- **THEN** the request payload dynamically resolves `email` and `password` from the active environment variables.

## MODIFIED Requirements

### Requirement: Core Variables for Local Development
The `environment.json` file MUST contain all variable definitions necessary to execute the Forge Platform API collections without unresolved variable errors. This MUST include `baseUrl` set to `http://localhost:9000/api/v1`, placeholder variables for `bearerToken`, `refreshToken`, and `apiKey`, default admin test credentials `adminEmail` (`admin1@forge.local`) and `adminPassword` (`Password123!`), and dynamic active authentication credentials `email` (`admin1@forge.local`) and `password` (`Password123!`).

#### Scenario: Running requests against local API
- **WHEN** requests referencing `{{baseUrl}}`, `{{bearerToken}}`, or auth credentials (`{{email}}`, `{{password}}`) are run with this environment
- **THEN** all variable references resolve successfully to local development values.

### Requirement: Multi-Role and Tenancy Matrix Environments
The system SHALL provide 6 distinct Postman environment configurations combining user roles (`admin`, `developer`, `viewer`) and organization tenancy states (with organization and without organization):
1. `Admin - With Organization`: contains admin credentials (`admin1@forge.local`), test password (`Password123!`), active auth credentials `email` (`admin1@forge.local`) and `password` (`Password123!`), and `organization_id` set to the standard test organization UUID (`00000000-0000-0000-0000-000000000001`).
2. `Admin - Without Organization`: contains admin credentials (`admin1@forge.local`), test password (`Password123!`), active auth credentials `email` (`admin1@forge.local`) and `password` (`Password123!`), and `organization_id` set to an empty string (`""`).
3. `Developer - With Organization`: contains developer credentials (`dev1@forge.local`), test password (`Password123!`), active auth credentials `email` (`dev1@forge.local`) and `password` (`Password123!`), and `organization_id` set to the standard test organization UUID (`00000000-0000-0000-0000-000000000001`).
4. `Developer - Without Organization`: contains developer credentials (`dev1@forge.local`), test password (`Password123!`), active auth credentials `email` (`dev1@forge.local`) and `password` (`Password123!`), and `organization_id` set to an empty string (`""`).
5. `Viewer - With Organization`: contains viewer credentials (`viewer1@forge.local`), test password (`Password123!`), active auth credentials `email` (`viewer1@forge.local`) and `password` (`Password123!`), and `organization_id` set to the standard test organization UUID (`00000000-0000-0000-0000-000000000001`).
6. `Viewer - Without Organization`: contains viewer credentials (`viewer1@forge.local`), test password (`Password123!`), active auth credentials `email` (`viewer1@forge.local`) and `password` (`Password123!`), and `organization_id` set to an empty string (`""`).

Each of these environments MUST define `baseUrl`, `bearerToken`, `refreshToken`, `apiKey`, `email`, `password`, `userEmail`, `userPassword`, `testOrgId`, `testProjectId`, and `organization_id`.

#### Scenario: Running collection requests as an Admin with organization
- **WHEN** requests are executed using the `Admin - With Organization` environment
- **THEN** authentication requests use `admin1@forge.local` credentials via `email` and `password`, and `Organization-ID` header resolves to `00000000-0000-0000-0000-000000000001`.

#### Scenario: Running collection requests as a Developer without organization
- **WHEN** requests are executed using the `Developer - Without Organization` environment
- **THEN** authentication requests use `dev1@forge.local` credentials via `email` and `password`, and `Organization-ID` header resolves to an empty value (`""`).

#### Scenario: Running collection requests as a Viewer with organization
- **WHEN** requests are executed using the `Viewer - With Organization` environment
- **THEN** authentication requests use `viewer1@forge.local` credentials via `email` and `password`, and `Organization-ID` header resolves to `00000000-0000-0000-0000-000000000001`.

### Requirement: Development and Production Tier Environments
The system SHALL provide 2 dedicated deployment lifecycle environment configurations:
1. `Development`: configured for local API server testing with `baseUrl` set to `http://localhost:9000/api/v1`, placeholder authentication tokens, default test credentials (including `email` set to `admin1@forge.local` and `password` set to `Password123!`), and test IDs.
2. `Production`: configured for production API environments with `baseUrl` set to a production host endpoint (`https://api.forge.local/api/v1`), empty placeholder tokens marked as secret, and unpopulated organization/user credentials (including `email` set to `""` and `password` marked as secret with empty value `""`).

#### Scenario: Running requests against Development tier
- **WHEN** collections are executed using the `Development` environment
- **THEN** `baseUrl` resolves to `http://localhost:9000/api/v1` and standard local test values including `email` and `password` are available.

#### Scenario: Inspecting Production tier configuration
- **WHEN** the `Production` environment is inspected
- **THEN** `baseUrl` targets the production host URL, and credential/secret fields (`bearerToken`, `refreshToken`, `apiKey`, `password`, `userPassword`, `adminPassword`, `devPassword`, `viewerPassword`) are configured as blank secrets while `email` is an empty string.
