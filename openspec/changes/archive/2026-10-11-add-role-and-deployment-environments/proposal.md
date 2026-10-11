# Proposal

## Why

Automated API testing, role-based access control (RBAC) verification, and multi-stage lifecycle testing require executing the Forge Platform API collections across different permission personas, tenancy contexts, and deployment stages. 

Currently, the workspace only provides a single local development environment. Adding dedicated environments covering combinations of roles (`admin`, `developer`, `viewer`) and tenancy contexts (with and without organization), along with target environments for `development` and `production`, enables seamless Newman and Postman testing of multi-tenant isolation, authorization boundaries, and deployment-tier configurations.

## What Changes

- Introduce 6 role-and-tenancy matrix environments:
  - **Admin - With Organization**: Configured with admin credentials (`admin1@forge.local`), test passwords, and active `organization_id` set to standard test organization UUID.
  - **Admin - Without Organization**: Configured with admin credentials (`admin1@forge.local`) and an empty `organization_id` to test non-organization or global operations.
  - **Developer - With Organization**: Configured with developer credentials (`dev1@forge.local`) and active `organization_id`.
  - **Developer - Without Organization**: Configured with developer credentials (`dev1@forge.local`) and empty `organization_id`.
  - **Viewer - With Organization**: Configured with viewer credentials (`viewer1@forge.local`) and active `organization_id`.
  - **Viewer - Without Organization**: Configured with viewer credentials (`viewer1@forge.local`) and empty `organization_id`.
- Introduce 2 deployment lifecycle environments:
  - **Development**: Local backend environment targeting `http://localhost:9000/api/v1` with test admin credentials and mock API tokens.
  - **Production**: Production tier template environment targeting a production API URL (e.g. `https://api.forge.local/api/v1` or placeholder), with sensitive variables configured as empty/secret placeholders.
- Synchronize Postman workspace configuration:
  - Create YAML environment definition files under `postman/postman/environments/` for each environment.
  - Register all environments in `postman/.postman/resources.yaml` under `localResources.environments` and `resourceNameMappings.environments`.
- Ensure Postman CLI and Newman compatibility by providing corresponding standard Postman JSON format environment files (including updating root `environment.json` and supporting standalone environment files).

## Capabilities

### New Capabilities
<!-- None -->

### Modified Capabilities
- `api-automation-environment`: Extend environment requirements to mandate multi-role RBAC environments (admin, developer, viewer), tenancy contexts (with and without organization), and deployment tier environments (development and production).

## Impact

- `postman/postman/environments/`: Addition of environment YAML files for all 8 environments.
- `postman/.postman/resources.yaml`: Mapping and resource registration for all environments.
- Root and CLI automation: Postman JSON environment definitions available for automated test execution with Newman and Postman CLI.
