# Design

## Context

Forge API collections include endpoints spanning global routes, organization-scoped routes, and multi-tenant resource management (projects, teams, deployments, environment variables). Access control enforces role boundaries across `admin`, `developer`, and `viewer` roles, while tenancy requires or forbids the `Organization-ID` header depending on whether the request is tenant-scoped or global.

Previously, only `Local Development.environment.yaml` existed with a single admin credential set and a static or blank `organization_id`. Automated testing and test run matrixing requires distinct environments for:
- Role & organization combinations (2 tenancy states × 3 roles = 6 environments).
- Deployment stage tiers (`development` and `production` = 2 environments).

## Goals / Non-Goals

**Goals:**
- Provide 6 environment files combining user roles (`admin`, `developer`, `viewer`) with tenancy scoping (`with organization` vs `without organization`).
- Provide 2 deployment lifecycle environments (`development` targeting local host and `production` templated for remote production environments).
- Ensure consistent variable schema across all environments (`baseUrl`, `bearerToken`, `refreshToken`, `apiKey`, `userEmail`, `userPassword`, `adminEmail`, `adminPassword`, `devEmail`, `devPassword`, `viewerEmail`, `viewerPassword`, `testOrgId`, `testProjectId`, `organization_id`).
- Register all environments in `postman/.postman/resources.yaml` so Postman desktop and Postman v12 Local Workspace immediately discover them.
- Provide Postman JSON environment files (including root `environment.json` and modular environment JSON exports) for Newman and CI/CD CLI test runners.

**Non-Goals:**
- Modifying backend Rust route handlers, database schemas, or RBAC policy code.
- Modifying collection request logic or endpoints.
- Storing real production API keys or production secrets in source control.

## Decisions

### 1. Environment Names and File Conventions
We establish standard human-readable environment names and consistent file paths in `postman/postman/environments/`:
- `Admin - With Organization` (`Admin - With Organization.environment.yaml`)
- `Admin - Without Organization` (`Admin - Without Organization.environment.yaml`)
- `Developer - With Organization` (`Developer - With Organization.environment.yaml`)
- `Developer - Without Organization` (`Developer - Without Organization.environment.yaml`)
- `Viewer - With Organization` (`Viewer - With Organization.environment.yaml`)
- `Viewer - Without Organization` (`Viewer - Without Organization.environment.yaml`)
- `Development` (`Development.environment.yaml`)
- `Production` (`Production.environment.yaml`)

*Alternative considered:* Using underscores or kebab-case file names (e.g., `admin_with_org.yaml`). Rejected in favor of spaced names matching the existing convention established by `Local Development.environment.yaml` and Postman v12 workspace resource mapping.

### 2. Variable Schema Alignment
Each environment defines the complete set of variables used across all Forge collections:
- `baseUrl`: `http://localhost:9000/api/v1` for development and role matrix; `https://api.forge.local/api/v1` (configurable) for production.
- `bearerToken` & `refreshToken`: marked as `secret: true`.
- `apiKey`: `forge_dev_api_key_placeholder` (or empty secret in production).
- `userEmail` & `userPassword`: active user identity for authentication requests in the given environment.
- Role-specific fallback credentials: `adminEmail`, `adminPassword`, `devEmail`, `devPassword`, `viewerEmail`, `viewerPassword`.
- `testOrgId` & `testProjectId`: `00000000-0000-0000-0000-000000000001` (or empty in production).
- `organization_id`: populated with `00000000-0000-0000-0000-000000000001` in "With Organization" environments, and empty string `""` in "Without Organization" environments.

### 3. Postman Workspace Registration
Update `postman/.postman/resources.yaml` to declare all 8 environments under `localResources.environments` and configure human-friendly display titles under `resourceNameMappings.environments`.

### 4. Postman CLI / Newman JSON Exports
In addition to the Postman v12 YAML workspace definitions, maintain standard Postman v2.1/v12 environment JSON representations:
- `environment.json` in repository root (mapped to Development environment).
- Individual JSON environment files in `postman/environments/` for CLI automation.

## Risks / Trade-offs

- [Risk] Secrets committed to source control for production environment.
  → Mitigation: The Production environment file only contains placeholder/empty fields marked as secrets; no actual secrets are stored in Git.
- [Risk] Breaking existing scripts relying on `Local Development` environment name.
  → Mitigation: Maintain `Local Development.environment.yaml` or alias it alongside `Development.environment.yaml`.
