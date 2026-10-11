# Proposal

## Why

Currently, numerous requests across the Forge Platform API Postman collection contain hardcoded organization UUIDs (`07c0060e-8e8c-44c1-942c-3004f5a6c5b6`) in their `Organization-ID` header. This prevents developers from easily switching tenant contexts during testing and development. By parameterizing the `Organization-ID` header to reference the `{{organization_id}}` environment variable (defined in `Local Development.environment.yaml`), tenant scoping can be managed from a single central configuration.

## What Changes

- Update all request files (`*.request.yaml`) in `postman/postman/collections/Forge Platform API/` that specify hardcoded UUID values for the `Organization-ID` header so they reference `{{organization_id}}`.
- Standardize the header definition across all requests to match the structure in `postman/postman/collections/Forge Platform API/projects/List projects for organization.request.yaml`, preserving header properties (`disabled: false`, description, and `$type` UUID formatting).
- Ensure consistent tenant-context switching without manual per-request edits.

## Capabilities

### New Capabilities
None.

### Modified Capabilities
- `api-automation-environment`: Extend the automation environment specification to require that all tenant-scoped collection requests utilize the `{{organization_id}}` variable in their `Organization-ID` headers rather than hardcoded identifiers.

## Impact

- Affects Postman request specification files in `postman/postman/collections/Forge Platform API/` (specifically under organizations, projects, teams, environment variables, deployments, and memberships).
- No modifications to the backend Rust codebase or OpenAPI specification files are required.
- Collection runs in Postman CLI or Newman will now resolve `Organization-ID` dynamically from the active environment.
