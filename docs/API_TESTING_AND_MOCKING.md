# API Automation, Multi-Role Matrix Testing & Mock Servers

This guide details the headless execution, multi-role RBAC security matrix testing, visual reporting, and mock server workflows available for the Forge Platform API.

---

## 1. Architecture Overview

Forge Platform API tests and contracts are organized into three layers:

1. **Postman v12 Local Workspace** (`postman/postman/`):
   - **Collections** (`postman/postman/collections/Forge Platform API/`): Modular `.request.yaml` files containing endpoints, schemas, and contract assertions (`pm.test`).
   - **Specs** (`postman/postman/specs/Forge Platform API/openapi.yaml`): Authoritative OpenAPI 3.0 specification.
   - **Environments** (`postman/postman/environments/`): Workspace environment files for local development, roles, and deployment stages.
   - **Mocks** (`postman/postman/mocks/`): Postman mock server configurations.
2. **Bundled Collection** (`postman/forge-platform-api.collection.json`):
   - Assembled from the Postman v12 workspace files by `scripts/bundle-collection.js` for standalone Newman execution.
3. **Execution & Automation**:
   - Executable via `justfile` recipes and Node.js runners with zero cloud credentials required.

---

## 2. Headless Collection Execution

Run the collection headlessly via `justfile` recipes:

### Bundle Collection

Compiles the Postman v12 Local Workspace directory into a single Postman Collection v2.1 JSON:

```bash
just bundle-collection
```

_Note: Test recipes automatically bundle the collection prior to execution._

### Run Tests against Local Development

Executes all collection requests against `http://localhost:9000/api/v1` using `environment.json`:

```bash
just test-api
```

### Run Tests against a Specific Environment

Executes the collection against any configured environment file in `postman/environments/`:

```bash
# Admin with organization scoping
just test-api-env admin-with-org

# Viewer without organization
just test-api-env viewer-without-org

# Developer with organization
just test-api-env developer-with-org

# Development environment
just test-api-env development
```

---

## 3. Multi-Role Matrix & RBAC Policy Automation

The system provides 6 distinct tenant and role environment matrices:

| Environment                          | Role        | Organization Header | User Email            |
| :----------------------------------- | :---------- | :------------------ | :-------------------- |
| **Admin - With Organization**        | `admin`     | Populated UUID      | `admin1@forge.local`  |
| **Admin - Without Organization**     | `admin`     | Empty (`""`)        | `admin1@forge.local`  |
| **Developer - With Organization**    | `developer` | Populated UUID      | `dev1@forge.local`    |
| **Developer - Without Organization** | `developer` | Empty (`""`)        | `dev1@forge.local`    |
| **Viewer - With Organization**       | `viewer`    | Populated UUID      | `viewer1@forge.local` |
| **Viewer - Without Organization**    | `viewer`    | Empty (`""`)        | `viewer1@forge.local` |

### Running the Matrix

To execute the suite across all 6 environments in sequence:

```bash
just test-api-matrix
```

The matrix runner:

- Sequentially executes Newman for each environment.
- Validates dual-branch response contracts (e.g. verifying that mutations receive `403 Forbidden` for Viewers while succeeding with `200`/`201` for Admins).
- Outputs a terminal comparison summary table indicating pass/fail status and assertion metrics across all environments.

---

## 4. Visual Reporting with `newman-reporter-htmlextra`

Generate interactive visual HTML reports containing request payloads, response bodies, headers, and assertion charts:

```bash
just test-api-report
```

The report is written to:

```
reports/postman/index.html
```

### Features of the HTML Report:

- **Interactive Dashboard**: Bar charts and progress meters showing total requests, pass/fail counts, and response latency.
- **Payload Inspection**: Full collapsible view of sent request payloads, headers, and response bodies.
- **Assertion Failure Highlighting**: Pinpoints failed assertion lines with expected vs actual values.

---

## 5. Postman Mock Servers & API Documentation Generation

Postman v12 Local Workspace allows frontend developers and clients to develop against mock endpoints and inspect interactive documentation before backend services are running.

### Workspace Registration

Mock servers are declared in:

- Definition: `postman/postman/mocks/Forge Platform API.mock.yaml`
- Workspace Registry: `postman/.postman/resources.yaml`

### Using Postman Mock Servers in Postman v12

1. Open the project in Postman v12 (`File -> Open Folder -> select postman/`).
2. Postman detects the workspace resources, environments, and mock server.
3. Select **Mocks** in the sidebar and click **Start Mock Server** or choose **Mock Server** from the active environment dropdown.
4. Requests made to the mock server URL will automatically return response examples defined in the collection and OpenAPI specification.

### Running Local Mock Servers with Prism

You can also run a local mock server directly from the command line using Prism backed by `openapi.yaml`:

```bash
npx --yes @stoplight/prism-cli mock "postman/postman/specs/Forge Platform API/openapi.yaml" -p 9000
```

This serves simulated API endpoints on `http://localhost:9000/api/v1` matching all OpenAPI schemas and response examples.

### Interactive API Documentation

- **Postman v12 Desktop**: Open the `Forge Platform API` collection and click **View Documentation** in the right-hand panel to view interactive Markdown descriptions, parameters, and example responses.
- **OpenAPI Preview**: The complete OpenAPI 3.0 specification is maintained at `postman/postman/specs/Forge Platform API/openapi.yaml` and can be rendered using Redoc or Swagger UI.

