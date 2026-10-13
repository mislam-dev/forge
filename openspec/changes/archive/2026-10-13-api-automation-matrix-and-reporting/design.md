# Design

## Context

The Forge Platform API utilizes the Postman v12 Local Workspace layout located under `postman/postman/`. This format stores collections as directories of modular YAML files (`.request.yaml`, `definition.yaml`) and environments as `.environment.yaml` and `.environment.json`. Comprehensive test assertions exist in `.request.yaml` files, and 6 role & tenancy environments (`admin-with-org`, `admin-without-org`, `developer-with-org`, `developer-without-org`, `viewer-with-org`, `viewer-without-org`) exist under `postman/environments/`.

However, Newman and standard CLI test runners expect a bundled Postman Collection v2.1 JSON file. There are currently no headless CLI execution recipes in `justfile`, no visual HTML report generator, and no registered mock server or documentation publication guide in the workspace.

See `proposal.md` for background motivation and `specs/api-automation-testing/spec.md` for behavior requirements.

## Goals / Non-Goals

**Goals:**
- Provide a Node-based collection bundler (`scripts/bundle-collection.js`) to assemble Postman v12 Local Workspace request YAML files and definitions into a standard Postman Collection v2.1 JSON (`postman/forge-platform-api.collection.json`).
- Add Justfile recipes (`bundle-collection`, `test-api`, `test-api-env`, `test-api-matrix`, `test-api-report`) enabling one-command headless execution.
- Create a matrix runner script (`scripts/run-role-matrix.js`) iterating through all 6 role & tenancy environments, verifying RBAC enforcement and displaying a status matrix summary table.
- Configure `newman-reporter-htmlextra` to generate interactive HTML reports with request/response payloads, headers, and pass/fail assertion visual graphs in `reports/postman/`.
- Configure Postman v12 mock server definitions under `postman/postman/mocks/` registered in `postman/.postman/resources.yaml`, and provide API documentation workflows backed by `openapi.yaml`.

**Non-Goals:**
- Adding GitHub Actions or remote CI workflows (per explicit project requirement).
- Modifying backend Rust route handlers, database schemas, or business logic.
- Altering existing Postman test assertions or rewriting `.request.yaml` files.
- Requiring Postman Cloud credentials or paid Postman enterprise licenses to run headless CLI tests.

## Decisions

### 1. Collection Bundling Strategy (Postman v12 YAML to Collection v2.1 JSON)
- **Decision**: Implement a lightweight Node.js script `scripts/bundle-collection.js` that recursively walks `postman/postman/collections/Forge Platform API`, parses YAML headers, URLs, methods, bodies, and `scripts` blocks (`type: afterResponse`), and serializes them into a canonical `postman/forge-platform-api.collection.json` (schema `https://schema.getpostman.com/json/collection/v2.1.0/collection.json`).
- **Rationale**: Postman v12 Local Workspace breaks collections into granular YAML files for Git friendliness, but Newman and third-party tools require a single v2.1 JSON file. Bundling locally requires zero cloud sync and preserves offline determinism.
- **Alternatives Considered**: 
  - *Using Postman CLI exclusively*: Rejected because Postman CLI requires a cloud API key or login for workspace sync, failing in restricted offline or headless environments without Postman credentials.
  - *Converting from openapi.yaml*: Rejected because it would omit the custom `pm.test` assertions, auth chaining, and token extraction code embedded in `.request.yaml` files.

### 2. Multi-Role RBAC Matrix Automation
- **Decision**: Create `scripts/run-role-matrix.js` invoked via `just test-api-matrix`. The script runs the collection across all 6 environments:
  1. `admin-with-org.environment.json`
  2. `admin-without-org.environment.json`
  3. `developer-with-org.environment.json`
  4. `developer-without-org.environment.json`
  5. `viewer-with-org.environment.json`
  6. `viewer-without-org.environment.json`
- **Execution & Validation**: The script executes Newman in sequence for each environment, captures assertion tallies and failures, formats a terminal comparison table, and exits non-zero if unexpected failures or privilege boundary breaches occur.
- **Alternatives Considered**: Shell script with loops. A Node script was chosen for cross-platform portability (Mac, Linux) and structured JSON result processing.

### 3. Visual Reporting with `newman-reporter-htmlextra`
- **Decision**: Integrate `newman-reporter-htmlextra` via `npx` or `package.json` devDependencies. Output reports to `reports/postman/`.
- **Options Configured**:
  - `--reporter-htmlextra-title "Forge Platform API Test Report"`
  - `--reporter-htmlextra-export reports/postman/index.html`
  - Visual tabs displaying request payloads, response bodies, and visual assertion graphs.

### 4. Postman Mock Server and Workspace Documentation
- **Decision**: Define a Postman mock server configuration in `postman/postman/mocks/Forge Platform API.mock.yaml` and map it into `postman/.postman/resources.yaml`.
- **Documentation**: Document the workflow in `docs/API_TESTING_AND_MOCKING.md` detailing:
  - How to run mock servers in Postman v12 or via OpenAPI mock tools (such as Prism / `@stoplight/prism-cli`).
  - How to view interactive documentation from `openapi.yaml`.

## Risks / Trade-offs

- **[Risk]** Collection drift between Postman v12 YAML workspace files and bundled JSON.
  → **Mitigation**: Add automatic bundle step as a dependency in `justfile` recipes (e.g. `test-api` automatically bundles before running).
- **[Risk]** Dependency on Node.js / npx for test runner execution in a Rust repository.
  → **Mitigation**: Node.js is already standard for Newman and Postman CLI; recipes use `npx --yes` to avoid mandatory global installations.
