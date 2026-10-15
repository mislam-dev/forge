# Tasks

## 1. Global Error Handling & Graceful Fallback

- [x] 1.1 Update Axum's global `not_found_handler` in `src/app/app.rs` to return an `AppErrorData` JSON envelope conforming to `ErrorResponse` (`is_error: true`, `code: "SYS_004"`, and `message`), and verify with `cargo test`
- [x] 1.2 Implement graceful path parameter deserialization rejection handling so invalid path parameters (e.g. non-UUID values) return HTTP 400 with a structured JSON `ErrorResponse` rather than Axum's plaintext rejection, and verify with a unit test

## 2. Auth Email Verification Route Support

- [x] 2.1 Add `VerifyEmailQueryDto` and GET handler support to `src/modules/auth/handlers.rs` to extract `token` from query parameters and forward to `AuthService::verify_email`
- [x] 2.2 Mount both `GET` and `POST` handlers on `/verify-email` in `src/modules/auth/router.rs`, and verify email verification unit and integration tests pass

## 3. Notifications API Alignment

- [x] 3.1 Remove `DELETE /notifications/{id}` endpoint from `docs/system/05-api/openapi.yaml` and `postman/postman/specs/Forge Platform API/openapi.yaml`
- [x] 3.2 Remove `Dismiss (soft-delete) a notification (Deprecated).request.yaml` and its example resources from `postman/postman/collections/Forge Platform API/notifications/{id}/`
- [x] 3.3 Implement dummy `notification_stream` SSE handler in `src/modules/notifications/handlers.rs` with `text/event-stream` headers, placeholder keep-alive/initial event, and a `// TODO: Implement real-time SSE notification stream using event bus / pub-sub` comment
- [x] 3.4 Mount `GET /stream` in `src/modules/notifications/router.rs` under protected routes, and verify router creation tests pass

## 4. Postman Collection Bundler & Workspace Alignment

- [x] 4.1 Update `scripts/bundle-collection.js` to process `pathVariables` and generate Postman v2.1 URL objects containing variable definitions so `:deployment_id` resolves during Newman execution
- [x] 4.2 Run `node scripts/bundle-collection.js` to regenerate `postman/forge-platform-api.collection.json` and verify the deprecated notification deletion request is absent and deployment status path variable is present

## 5. Verification & Test Suite Execution

- [x] 5.1 Run `cargo test` to verify all backend modules build and tests pass cleanly
- [x] 5.2 Run `npx newman run postman/forge-platform-api.collection.json -e environment.json` (or `just test-api`) to confirm all 8 assertion failures are resolved with zero test failures
