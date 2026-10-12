# Proposal

## Why

The Forge Platform API organization and membership management endpoints (`/organizations/*`) in the Postman collection currently lack test assertion scripts. Without automated test assertions verifying both success (2xx) and failure (4xx) responses against the schemas and examples defined in OpenAPI specification, automated API test suites (Newman, Postman CLI, CI regression runs) cannot reliably validate whether organization endpoints adhere to contract specifications or handle role/membership operations properly.

## What Changes

- Add comprehensive Postman test assertion scripts (`scripts` with `type: afterResponse`) to all 11 organization requests in `postman/postman/collections/Forge Platform API/organizations/`:
  - `POST /organizations`: Validate 201 Created (`OrganizationResponse` schema) and 400 Bad Request error responses (`ErrorResponse` schema). Optionally persist `organization_id` into Postman environment variables if returned.
  - `GET /organizations`: Validate 200 OK list response (`data` array of `OrganizationResponse` items) and 401 Unauthorized errors.
  - `GET /organizations/{id}`: Validate 200 OK single organization response (`OrganizationResponse` schema), 401 Unauthorized, and 404 Not Found error responses.
  - `PATCH /organizations/{id}`: Validate 200 OK update response, 400 Bad Request, 401 Unauthorized, and 404 Not Found errors.
  - `DELETE /organizations/{id}`: Validate 200 OK deletion confirmation message, 401 Unauthorized, and 404 Not Found errors.
  - `POST /organizations/{id}/invitations`: Validate 201 Created (`OrgInvitationResponse` schema), 400 Bad Request, 401 Unauthorized, and 404 Not Found errors.
  - `GET /organizations/{id}/invitations`: Validate 200 OK list response (`data` array of `OrgInvitationResponse` items), 401 Unauthorized, and 404 Not Found errors.
  - `POST /organizations/invitations/{token}/accept`: Validate 200 OK acceptance response (`OrgMemberResponse` schema), 400 Bad Request, 401 Unauthorized, and 404 Not Found errors.
  - `GET /organizations/{id}/members`: Validate 200 OK list response (`data` array of `OrgMemberResponse` items), 401 Unauthorized, and 404 Not Found errors.
  - `PATCH /organizations/{id}/members/{user_id}`: Validate 200 OK update response (`OrgMemberResponse` schema), 400 Bad Request, 401 Unauthorized, and 404 Not Found errors.
  - `DELETE /organizations/{id}/members/{user_id}`: Validate 200 OK removal confirmation message, 401 Unauthorized, and 404 Not Found errors.
- Ensure test scripts validate HTTP status codes, Content-Type headers (`application/json`), and JSON response body structures.
- Support dual-response assertion logic handling both expected success structures and standardized error structures (`is_error`, `code`, `message`).

## Capabilities

### New Capabilities
- `organization-collection-tests`: Defines contract and automated response assertions across organization and organization membership collection endpoints in Postman/Newman, validating both success (2xx) and client/server error responses according to OpenAPI schemas.

### Modified Capabilities
<!-- None -->

## Impact

- Postman collection request YAML files under `postman/postman/collections/Forge Platform API/organizations/` will be updated with `afterResponse` test scripts.
- No backend Rust code or database schema changes are required.
- Automated API test runners (Newman, Postman CLI) will now validate responses and status codes across all organization endpoints.
