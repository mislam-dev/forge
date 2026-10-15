# Design: API Collection Test Fixes and Contract Alignment

## Context

Running Newman on `postman/forge-platform-api.collection.json` against the local Axum server produced 8 test assertion failures across 4 request groups. The failures stem from mismatched HTTP methods on auth verification, unmounted notification SSE routes, missing standard JSON envelopes in 404 fallback handlers, unhandled path parameter rejections returning plaintext, and lingering deprecated endpoints in the collection.

See [proposal.md](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/openspec/changes/fix-api-collection-test-failures/proposal.md) for background and motivation.

## Goals / Non-Goals

**Goals:**
- Enable `GET /auth/verify-email?token=...` handling in Axum to satisfy the OpenAPI specification and Postman assertions.
- Completely remove `DELETE /notifications/{id}` ("Dismiss (soft-delete) a notification") from OpenAPI specs and Postman collections, updating the bundle script to reflect this removal.
- Provide a stubbed/dummy `GET /notifications/stream` SSE endpoint with a prominent `TODO` comment for future pub/sub integration so that automated contract tests pass.
- Standardize the Axum 404 fallback handler (`not_found_handler`) and path extractor rejection to return JSON matching the `ErrorResponse` schema (`is_error: true`, `code`, `message`).
- Ensure `scripts/bundle-collection.js` serializes `pathVariables` into Postman collection URL objects so that path parameters like `:deployment_id` resolve properly during Newman test execution.

**Non-Goals:**
- Building the complete real-time SSE notification broadcaster or Redis/RabbitMQ pub-sub subscriber (a minimal conforming dummy SSE stream satisfies the route for now).
- Changing database schemas or altering soft-delete behavior in existing entity models.

## Decisions

### Decision 1: Dual-method or GET route for `/auth/verify-email`
- **Choice**: Extend the router in `src/modules/auth/router.rs` to support `GET` (with query token extraction `Query<VerifyEmailQueryDto>`) while preserving `POST` (with JSON body `JsonValidate<VerifyEmailDto>`).
- **Rationale**: The OpenAPI contract documents `GET /auth/verify-email?token=...` as the public link activation endpoint. Supporting `GET` while keeping `POST` ensures zero regressions for any JSON-based callers.
- **Alternatives considered**: Replacing `POST` entirely with `GET`. Rejected to preserve backwards compatibility.

### Decision 2: Complete removal of deprecated `DELETE /notifications/{id}`
- **Choice**: Delete the request file `postman/postman/collections/Forge Platform API/notifications/{id}/Dismiss (soft-delete) a notification (Deprecated).request.yaml` and its associated example YAML files, remove `/notifications/{id}: delete:` from OpenAPI YAML specs, and re-bundle the collection.
- **Rationale**: The endpoint is documented as deprecated and marked for deletion in favor of soft-state transitions (`PATCH /notifications/{id}/read`). Removing it aligns the test suite with current architecture.
- **Alternatives considered**: Implementing a dummy 200 OK handler in Axum. Rejected because the user explicitly instructed: "remove this api from postmane and openapi.yaml file as well."

### Decision 3: Dummy SSE Notification Stream Handler
- **Choice**: Implement `notification_stream` in `src/modules/notifications/handlers.rs` returning Axum's `Sse<impl Stream<Item = Result<Event, Infallible>>>`. The stream sends an initial connected event (or keep-alive comment) and remains open, accompanied by a `// TODO: Implement real-time SSE notification stream using event bus / pub-sub` comment.
- **Rationale**: Satisfies the contract test (`Content-Type: text/event-stream`, status 200) without premature over-engineering of the notifications event pipeline.
- **Alternatives considered**: Returning a static text response. Rejected because Newman / OpenAPI tests validate `text/event-stream`.

### Decision 4: Standardizing Fallback and Rejection Error Envelopes
- **Choice**:
  1. Refactor `not_found_handler` in `src/app/app.rs` to construct an `AppErrorData` envelope with `is_error: true`, `code: "SYS_004"`, and `message: "The requested resource was not found"`.
  2. Implement a custom path extractor or rejection handler (e.g. `PathExtractor` or `FromRequestParts` wrapper) so Axum path deserialization failures (e.g. invalid UUIDs) return HTTP 400 with `AppErrorData` rather than Axum's built-in `text/plain` rejection.
- **Rationale**: Postman contract tests validate that every 4xx and 5xx response body conforms to `ErrorResponse`. Axum's default plain-text rejections violate this global contract.
- **Alternatives considered**: Modifying Postman test scripts to tolerate plaintext. Rejected because the platform spec requires JSON envelopes everywhere.

### Decision 5: Preserving Path Variables in Collection Bundler
- **Choice**: Update `scripts/bundle-collection.js` to inspect `data.pathVariables`. When present, output the URL as a structured Postman v2.1 object containing `raw`, `host`, `path`, and `variable: [{ key, value }]`.
- **Rationale**: Newman relies on `url.variable` definitions to replace `:deployment_id` with a valid mock UUID (`a1b2c3d4-e89b-12d3-a456-426614174000`) specified in the YAML request metadata.
- **Alternatives considered**: Hardcoding variable values in `environment.json`. Updating the bundler is cleaner and durable across all collections.

## Risks / Trade-offs

- **[Risk] Postman collection bundle drifting from YAML sources** → Mitigation: Run `node scripts/bundle-collection.js` via the Justfile recipe `bundle-collection` and verify `forge-platform-api.collection.json`.
- **[Risk] SSE dummy stream holding connections in long test suites** → Mitigation: Ensure stream produces standard SSE events without unbounded memory leaks or hung test runners.
