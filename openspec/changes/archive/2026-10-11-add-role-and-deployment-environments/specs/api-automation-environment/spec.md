# Spec Delta

## ADDED Requirements

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
