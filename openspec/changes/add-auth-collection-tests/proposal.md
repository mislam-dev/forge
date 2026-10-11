# Proposal

## Why

The Forge Platform API authentication endpoints (`/auth/*`) in the Postman collection currently lack test assertion scripts, with only `login` storing token environment variables. Without automated test assertions verifying both success (2xx) and failure (4xx) responses against the schemas and examples defined in `openapi.yaml`, API automation workflows (such as Newman and Postman CLI) cannot validate whether authentication endpoints behave correctly or adhere to contract specifications.

## What Changes

- Add comprehensive Postman test assertion scripts (`scripts` with `type: afterResponse`) to all 8 requests in `postman/postman/collections/Forge Platform API/auth/`:
  - `POST /auth/register`: Validate 201 Created (`UserResponse` schema) and 400/409 error responses (`ErrorResponse` schema).
  - `POST /auth/login`: Validate 200 OK (`AuthTokenResponse` schema) and 401 Unauthorized (`ErrorResponse`), while preserving environment token persistence (`bearerToken`, `refreshToken`).
  - `POST /auth/logout`: Validate 200 OK session invalidation and 401 Unauthorized errors.
  - `POST /auth/refresh`: Validate 200 OK token refreshment (`AuthTokenResponse`), environment token updates, and 401 Unauthorized errors.
  - `POST /auth/forgot-password`: Validate 200 OK notification response and 400 Bad Request error response.
  - `POST /auth/reset-password`: Validate 200 OK password reset confirmation, 400 Bad Request, and 401 Unauthorized (`AUTH_006`) errors.
  - `GET /auth/verify-email`: Validate 200 OK account activation and 400 Bad Request (`AUTH_006`/`AUTH_007`) errors.
  - `GET /auth/me`: Validate 200 OK profile retrieval (`UserResponse` schema) and 401 Unauthorized errors.
- Ensure test scripts validate HTTP status codes, Content-Type headers, and response body schemas derived from `openapi.yaml`.
- Support dual-response assertion logic (handling both expected success structures and standardized error responses `{ is_error: true, code, message, errors }`).

## Capabilities

### New Capabilities
- `auth-collection-tests`: Defines the contract and assertions for automated response validation across authentication collection endpoints in Postman/Newman, covering both success (2xx) and error (4xx) responses according to OpenAPI schemas.

### Modified Capabilities
<!-- None -->

## Impact

- Collection request files under `postman/postman/collections/Forge Platform API/auth/` will be updated with `afterResponse` test scripts.
- No backend Rust code or database schema changes are required.
- Automated API test runners (Newman, Postman CLI) will now execute assertions and report test results across all authentication endpoints.
