# Proposal: Fix API Collection Test Failures and Align API Contracts

## Why

Automated Newman collection tests (`just test-api`) currently fail with 8 assertion errors across authentication, notifications, and deployment status endpoints due to HTTP method mismatches, unmounted routes, non-standardized 404 fallback responses, unhandled path parameter rejections, and orphaned deprecated endpoints. Resolving these issues restores a clean CI testing matrix and ensures end-to-end contract alignment between Axum routers, OpenAPI specifications, and Postman collections.

## What Changes

- **Auth Email Verification Route Support**: Add `GET` support for `/auth/verify-email` with query parameter token extraction alongside existing `POST` handling in `src/modules/auth/router.rs` and `handlers.rs`.
- **Remove Deprecated Notification Dismissal**: Remove `DELETE /notifications/{id}` ("Dismiss (soft-delete) a notification") from OpenAPI specs (`openapi.yaml`), Postman local workspace requests (`.request.yaml`), and bundle artifact (`forge-platform-api.collection.json`).
- **Dummy Notification SSE Stream Route**: Mount `GET /notifications/stream` in `src/modules/notifications/router.rs` returning an SSE stream with a placeholder event and a clear `TODO` comment for future full pub/sub event integration.
- **Graceful Error Handling in 404 Fallback**: Update Axum's global `not_found_handler` in `src/app/app.rs` to return a structured JSON response adhering to `ErrorResponse` (`is_error: true`, `code: "SYS_004"`, `message: "..."`) instead of raw `ApiResponse`.
- **Graceful Path Parameter Rejection & Bundler Path Variables**: Support JSON error responses for path extraction failures (e.g., non-UUID parameters) and update `scripts/bundle-collection.js` to serialize `pathVariables` into Postman collection URL objects so Newman accurately binds `:deployment_id`.

## Capabilities

### New Capabilities
- `notifications-api`: Defines active notification endpoints, introducing the SSE notification stream (`GET /notifications/stream`) and establishing the removal of the deprecated `DELETE /notifications/{id}` endpoint.

### Modified Capabilities
- `error-handling`: Mandate that all 4xx error responses, specifically including the global 404 fallback handler and route extractor deserialization rejections, return structured JSON conforming to `ErrorResponse`.
- `auth-collection-tests`: Ensure `GET /auth/verify-email` is actively routed and tested with query token validation, returning 200 OK or 400 Bad Request error responses.

## Impact

- **Affected Backend Code**: `src/app/app.rs`, `src/modules/auth/router.rs`, `src/modules/auth/handlers.rs`, `src/modules/notifications/router.rs`, `src/modules/notifications/handlers.rs`, `src/shared/error/`.
- **Affected API Specs & Collections**: `docs/system/05-api/openapi.yaml`, `postman/postman/specs/Forge Platform API/openapi.yaml`, `postman/postman/collections/Forge Platform API/notifications/{id}/`, `scripts/bundle-collection.js`, `postman/forge-platform-api.collection.json`.
- **Breaking Changes**: None for active clients; `DELETE /notifications/{id}` was previously deprecated and marked for removal.
