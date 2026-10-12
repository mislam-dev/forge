# Tasks: Parameterize Login Credentials Across Environments and Login Request

## 1. Update Postman Workspace Environment YAML Files

- [x] 1.1 Add `email` (`admin1@forge.local`) and `password` (`Password123!`) to `Admin - With Organization.environment.yaml` and `Admin - Without Organization.environment.yaml`, and verify YAML formatting.
- [x] 1.2 Add `email` (`dev1@forge.local`) and `password` (`Password123!`) to `Developer - With Organization.environment.yaml` and `Developer - Without Organization.environment.yaml`, and verify YAML formatting.
- [x] 1.3 Add `email` (`viewer1@forge.local`) and `password` (`Password123!`) to `Viewer - With Organization.environment.yaml` and `Viewer - Without Organization.environment.yaml`, and verify YAML formatting.
- [x] 1.4 Add `email` (`admin1@forge.local`) and `password` (`Password123!`) to `Development.environment.yaml` and `Local Development.environment.yaml`, and verify YAML formatting.
- [x] 1.5 Add `email` (`""`) and `password` (`""` with `secret: true`) to `Production.environment.yaml`, and verify YAML formatting.

## 2. Update Postman Export JSON Files and Root Environment

- [x] 2.1 Add `email` (`admin1@forge.local`) and `password` (`Password123!`) to root `environment.json`, and verify JSON validity.
- [x] 2.2 Add `email` and `password` variables to `admin-with-org.environment.json` and `admin-without-org.environment.json`, and verify JSON validity.
- [x] 2.3 Add `email` and `password` variables to `developer-with-org.environment.json` and `developer-without-org.environment.json`, and verify JSON validity.
- [x] 2.4 Add `email` and `password` variables to `viewer-with-org.environment.json` and `viewer-without-org.environment.json`, and verify JSON validity.
- [x] 2.5 Add `email` and `password` variables to `development.environment.json` and `production.environment.json`, ensuring `production.environment.json` marks `password` as secret.

## 3. Parameterize Login Collection Request Body & Verification

- [x] 3.1 Update `postman/postman/collections/Forge Platform API/auth/login/Authenticate user credentials.request.yaml` to dynamically reference `"{{email}}"` and `"{{password}}"` in the request body JSON string, and verify YAML structure.
- [x] 3.2 Verify all updated JSON and YAML environment files parse without syntax errors and that variable keys are consistent across all environments.
