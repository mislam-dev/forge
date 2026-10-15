# Spec Delta: Error Handling

## MODIFIED Requirements

### Requirement: Structured Error Response Envelope
The system SHALL format all HTTP 4xx client error and HTTP 500 server error responses—including unrouted fallback endpoints and route parameter extraction failures—as a structured JSON object containing `is_error: true`, `code: string`, `message: string`, and `errors: object`.

#### Scenario: Client error response envelope
- **WHEN** any API endpoint encounters a client error (HTTP 400, 401, 403, 404, 409)
- **THEN** the response body is JSON with `is_error` set to true, `code` set to the specific documented error code string, `message` containing a human-readable explanation, and `errors` containing an object (or null/empty object when no field-level errors exist).

#### Scenario: Internal server error response envelope
- **WHEN** an unexpected internal error or database failure occurs
- **THEN** the system returns HTTP 500 Internal Server Error with `code: "INTERNAL_SERVER_ERROR"`, a sanitized generic message, and `is_error: true`.

#### Scenario: Unrouted endpoint fallback response envelope
- **WHEN** a client sends an HTTP request to an unregistered endpoint or route
- **THEN** the system returns HTTP 404 Not Found with JSON containing `is_error: true`, `code: "SYS_004"`, and a clear not-found `message`.

#### Scenario: Path parameter deserialization failure
- **WHEN** an incoming request path parameter cannot be parsed into the expected type
- **THEN** the system returns HTTP 400 Bad Request with JSON containing `is_error: true`, `code: "VALIDATION_ERROR"`, and a human-readable explanation in `message`.
