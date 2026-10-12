# Proposal

## Why

The Forge Platform API team and team membership management endpoints (`/teams/*`) in the Postman collection currently lack test assertion scripts. Without automated test assertions verifying both success (2xx) and failure (4xx) responses against the schemas and examples defined in OpenAPI specification, automated API test suites (Newman, Postman CLI, CI regression runs) cannot reliably validate whether team management endpoints adhere to contract specifications or handle role/membership operations properly.

## What Changes

- Add comprehensive Postman test assertion scripts (`scripts` with `type: afterResponse`) to all 9 team requests in `postman/postman/collections/Forge Platform API/teams/`:
  - `POST /teams`: Validate 201 Created (`TeamResponse` schema) and client error responses (400 Bad Request, 401 Unauthorized, 403 Forbidden conforming to `ErrorResponse` schema). Persist `team_id` into Postman environment variables upon creation.
  - `GET /teams`: Validate 200 OK list response (`data` array of `TeamResponse` items) and error responses (400 Bad Request, 401 Unauthorized, 403 Forbidden conforming to `ErrorResponse` schema).
  - `GET /teams/{id}`: Validate 200 OK single team response (`TeamResponse` schema) and error responses (401 Unauthorized, 403 Forbidden, 404 Not Found conforming to `ErrorResponse` schema).
  - `PATCH /teams/{id}`: Validate 200 OK update response (`TeamResponse` schema) and error responses (400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found conforming to `ErrorResponse` schema).
  - `DELETE /teams/{id}`: Validate 200 OK deletion confirmation message and error responses (401 Unauthorized, 403 Forbidden, 404 Not Found conforming to `ErrorResponse` schema).
  - `GET /teams/{id}/members`: Validate 200 OK list response (`data` array of `TeamMemberResponse` items) and error responses (401 Unauthorized, 403 Forbidden, 404 Not Found conforming to `ErrorResponse` schema).
  - `POST /teams/{id}/members`: Validate 201 Created (`TeamMemberResponse` schema) and error responses (400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found, 409 Conflict conforming to `ErrorResponse` schema).
  - `PATCH /teams/{id}/members/{user_id}`: Validate 200 OK update response (`TeamMemberResponse` schema) and error responses (400 Bad Request, 401 Unauthorized, 403 Forbidden, 404 Not Found conforming to `ErrorResponse` schema).
  - `DELETE /teams/{id}/members/{user_id}`: Validate 200 OK removal confirmation message and error responses (401 Unauthorized, 403 Forbidden, 404 Not Found conforming to `ErrorResponse` schema).
- Ensure test scripts validate HTTP status codes, Content-Type headers (`application/json`), and JSON response body structures.
- Support dual-branch assertion logic handling both expected success structures and standardized error structures (`is_error`, `code`, `message`).

## Capabilities

### New Capabilities
- `teams-collection-tests`: Defines contract and automated response assertions across team and team membership collection endpoints in Postman/Newman, validating both success (2xx) and client/server error responses according to OpenAPI schemas.

### Modified Capabilities
<!-- None -->

## Impact

- Postman collection request YAML files under `postman/postman/collections/Forge Platform API/teams/` will be updated with `afterResponse` test scripts.
- No backend Rust code or database schema changes are required.
- Automated API test runners (Newman, Postman CLI) will now validate responses and status codes across all team endpoints.
