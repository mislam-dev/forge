# Proposal: Parameterize Login Credentials Across Environments and Login Request

## Why

Currently, the Postman collection request for user login (`POST /auth/login` - `Authenticate user credentials`) contains hardcoded credentials in its JSON body (`viewer1@forge.local` / `Password123!`), requiring manual editing whenever switching between test roles (Admin, Developer, Viewer) or target deployment environments (Development, Production). While role-specific credential variables (`adminEmail`, `devEmail`, `viewerEmail`, `userEmail`) exist across some environments, the login request body does not dynamically reference environment variables, and the standard variable names `email` and `password` are missing across the Postman environment configurations.

Adding dedicated `email` and `password` variables to every environment configuration and dynamically parameterizing the login request body allows automated tests, Newman test runs, and Postman workspace users to seamlessly authenticate as the active environment's configured role without modifying collection request bodies.

## What Changes

- Add `email` and `password` variables across all Postman environment configurations:
  - Root Postman environment (`environment.json`).
  - Standalone Postman JSON environments (`postman/environments/*.environment.json`): `admin-with-org`, `admin-without-org`, `developer-with-org`, `developer-without-org`, `viewer-with-org`, `viewer-without-org`, `development`, and `production`.
  - Workspace Postman YAML environments (`postman/postman/environments/*.environment.yaml`): `Admin - With Organization`, `Admin - Without Organization`, `Developer - With Organization`, `Developer - Without Organization`, `Viewer - With Organization`, `Viewer - Without Organization`, `Development`, `Production`, and `Local Development`.
- Populate role-appropriate credentials for each environment:
  - Admin environments: `email` = `"admin1@forge.local"`, `password` = `"Password123!"`
  - Developer environments: `email` = `"dev1@forge.local"`, `password` = `"Password123!"`
  - Viewer environments: `email` = `"viewer1@forge.local"`, `password` = `"Password123!"`
  - Development / Local Development environments: `email` = `"admin1@forge.local"`, `password` = `"Password123!"`
  - Production environment: `email` = `""`, `password` = `""` (configured as secret).
- Parameterize the `POST /auth/login` request body in `postman/postman/collections/Forge Platform API/auth/login/Authenticate user credentials.request.yaml` to dynamically reference `{{email}}` and `{{password}}`.

## Capabilities

### New Capabilities

*(None)*

### Modified Capabilities

- `api-automation-environment`: Update the specification to require `email` and `password` variables across all environment configuration files and to require dynamic parameterization of the `POST /auth/login` collection request body using `{{email}}` and `{{password}}`.

## Impact

- **Postman Collections**: `postman/postman/collections/Forge Platform API/auth/login/Authenticate user credentials.request.yaml` request payload is updated from hardcoded email/password to dynamic Postman template variables.
- **Environment Configurations**:
  - `environment.json`
  - `postman/environments/*.environment.json` (8 files)
  - `postman/postman/environments/*.environment.yaml` (9 files)
- **CI / Automated Testing**: Collection runs in Postman CLI or Newman will now automatically authenticate with the active environment's specified user role without needing script pre-steps to overwrite request payloads.
