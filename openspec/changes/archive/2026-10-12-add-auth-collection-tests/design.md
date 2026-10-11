# Design

## Context

The Forge Platform API Postman collections are maintained in the Postman v12 Local Workspace format located at `postman/postman/collections/Forge Platform API/`. Each endpoint is represented by a `*.request.yaml` file. Postman allows attaching lifecycle test scripts under the `scripts:` list with `type: afterResponse` and `language: text/javascript`.

Currently, only `Authenticate user credentials.request.yaml` defines a script, which extracts tokens into environment variables but contains no test assertions (`pm.test`). All other auth requests lack test scripts entirely.

See `proposal.md` for motivation and `specs/auth-collection-tests/spec.md` for requirement contracts.

## Goals / Non-Goals

**Goals:**
- Provide robust, contract-driven test assertion scripts (`pm.test`) for all 8 requests under `postman/postman/collections/Forge Platform API/auth/`.
- Validate status codes, content-type headers, and JSON body structures for both successful (2xx) and failed (4xx) responses based on `openapi.yaml`.
- Ensure token extraction and environment variable persistence (`bearerToken` and `refreshToken`) for `login` and `refresh` endpoints upon successful authentication.
- Maintain compatibility with Newman and Postman CLI runners.

**Non-Goals:**
- Modifying backend Rust code or database entities.
- Modifying OpenAPI schemas or creating mock servers.
- Adding tests to non-auth collection modules.

## Decisions

### Decision 1: Dual-Branch Assertion Logic in `afterResponse` Scripts
- **Choice**: Structure the test scripts to inspect `pm.response.code`.
  - When `pm.response.code >= 200 && pm.response.code < 300`:
    - Assert expected success status (e.g., 200 OK or 201 Created).
    - Assert `Content-Type` header includes `application/json`.
    - Assert endpoint-specific payload fields (`data.access_token`, `data.refresh_token`, `UserResponse.data.id`, etc.) matching OpenAPI models.
  - When `pm.response.code >= 400`:
    - Assert client/server error status code (400, 401, 409).
    - Assert standard `ErrorResponse` schema (`is_error === true`, `code` string, `message` string, `errors` object).
- **Rationale**: Collection requests may be run in varying contexts (e.g., full end-to-end happy path, or intentional negative tests with invalid credentials/expired tokens). Dual-branch assertion ensures the test scripts validate the contract regardless of whether a success or error scenario is being exercised, rather than throwing uncaught script exceptions.
- **Alternatives Considered**: Writing strict success-only assertions. Rejected because executing negative scenarios or encountering invalid tokens would produce uncaught property access errors instead of validating the error response schema.

### Decision 2: Standardized Error Validation Helper Pattern
- **Choice**: Within the error branch, use a standardized validation pattern across all 8 requests:
  ```javascript
  pm.test("Status code is 4xx error", function () {
      pm.expect(pm.response.code).to.be.oneOf([400, 401, 409]);
  });
  pm.test("Error response conforms to OpenAPI ErrorResponse schema", function () {
      let json = pm.response.json();
      pm.expect(json.is_error).to.be.true;
      pm.expect(json.code).to.be.a("string");
      pm.expect(json.message).to.be.a("string");
  });
  ```
- **Rationale**: Directly aligns with `openapi.yaml`'s `#/components/schemas/ErrorResponse` and prevents diverging assertions across endpoints.

### Decision 3: Token Persistence Integration in Login and Refresh
- **Choice**: In `POST /auth/login` and `POST /auth/refresh`, embed the environment token updates (`pm.environment.set("bearerToken", ...)`, `pm.environment.set("refreshToken", ...)`) inside the success verification block.
- **Rationale**: Prevents overwriting valid environment tokens with `undefined` when a login or refresh request fails.

## Risks / Trade-offs

- **[Risk]** Uncaught JSON parse errors on non-JSON response bodies (e.g. 502 Bad Gateway from a proxy).
  → **Mitigation**: Wrap `pm.response.json()` calls or check `pm.response.to.be.json` prior to inspecting JSON properties.
- **[Risk]** YAML indentation errors in multiline `scripts` strings.
  → **Mitigation**: Standardize YAML syntax and validate formatting during change application.
