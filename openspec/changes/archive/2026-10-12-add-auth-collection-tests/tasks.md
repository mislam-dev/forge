# Tasks

## 1. Primary Authentication Endpoints

- [x] 1.1 Add test assertion script to `Register a new user account.request.yaml` validating 201 Created (`UserResponse`), 400 Bad Request, and 409 Conflict (`ErrorResponse`), and verify YAML formatting
- [x] 1.2 Update test assertion script in `Authenticate user credentials.request.yaml` validating 200 OK (`AuthTokenResponse`), 401 Unauthorized (`ErrorResponse`), and persisting environment tokens (`bearerToken`, `refreshToken`), and verify YAML formatting

## 2. Session and Token Lifecycle Endpoints

- [x] 2.1 Add test assertion script to `Invalidate user session.request.yaml` validating 200 OK session invalidation and 401 Unauthorized (`ErrorResponse`), and verify YAML formatting
- [x] 2.2 Add test assertion script to `Refresh JWT access token.request.yaml` validating 200 OK token refresh (`AuthTokenResponse`), environment token updates, and 401 Unauthorized (`ErrorResponse`), and verify YAML formatting

## 3. Account Recovery and Verification Endpoints

- [x] 3.1 Add test assertion script to `Request password reset email.request.yaml` validating 200 OK notification response and 400 Bad Request (`ErrorResponse`), and verify YAML formatting
- [x] 3.2 Add test assertion script to `Reset password using a valid reset token.request.yaml` validating 200 OK password reset confirmation, 400 Bad Request, and 401 Unauthorized (`ErrorResponse`), and verify YAML formatting
- [x] 3.3 Add test assertion script to `Verify user email address.request.yaml` validating 200 OK account activation and 400 Bad Request (`ErrorResponse`), and verify YAML formatting

## 4. User Profile and Workspace Validation

- [x] 4.1 Add test assertion script to `Get authenticated user profile.request.yaml` validating 200 OK (`UserResponse`) and 401 Unauthorized (`ErrorResponse`), and verify YAML formatting
- [x] 4.2 Validate all 8 updated request YAML files against the OpenAPI specification schemas and verify Postman collection parsing integrity
