# Tasks

## 1. Collection Bundling & Tooling Setup

- [x] 1.1 Create `scripts/bundle-collection.js` to traverse the Postman v12 Local Workspace directory (`postman/postman/collections/Forge Platform API`), parse request YAML files and `afterResponse` scripts, and generate a valid Postman Collection v2.1 JSON file at `postman/forge-platform-api.collection.json`. Verify by running `node scripts/bundle-collection.js` and checking JSON schema validity and item counts.
- [x] 1.2 Create `package.json` defining devDependencies for `newman`, `newman-reporter-htmlextra`, and `yaml`, and verify `npm install` completes successfully.


## 2. Justfile Recipes & Headless Execution

- [x] 2.1 Add `bundle-collection` recipe to `justfile` and verify that running `just bundle-collection` regenerates `postman/forge-platform-api.collection.json`.
- [x] 2.2 Add `test-api` and `test-api-env` recipes to `justfile` using Newman to execute the bundled collection against `environment.json` or specified environment JSON files in `postman/environments/`, and verify via `just --list`.

## 3. Multi-Role Matrix & RBAC Policy Automation

- [x] 3.1 Create `scripts/run-role-matrix.js` to sequentially run Newman against all 6 role and tenancy environments (`admin-with-org`, `admin-without-org`, `developer-with-org`, `developer-without-org`, `viewer-with-org`, `viewer-without-org`), verify expected RBAC status boundaries, and output an aggregated summary table to the console. Verify script execution and error handling.
- [x] 3.2 Add `test-api-matrix` recipe to `justfile` invoking `scripts/run-role-matrix.js` and verify execution via `just --list`.

## 4. Visual Reporting with htmlextra

- [x] 4.1 Add `test-api-report` recipe to `justfile` to run Newman with `-r cli,htmlextra` and export an interactive HTML dashboard to `reports/postman/index.html`. Verify that report flags and export directories are created properly.


## 5. Postman Mock Server & Documentation Generation

- [x] 5.1 Create Postman v12 mock server definition at `postman/postman/mocks/Forge Platform API.mock.yaml` and update `postman/.postman/resources.yaml` with mock resource mappings. Verify YAML format and workspace resource integrity.
- [x] 5.2 Create `docs/API_TESTING_AND_MOCKING.md` documenting headless Newman testing, multi-role matrix verification, running Postman v12 mock servers, and publishing interactive documentation from `openapi.yaml`. Verify markdown links and completeness.

