# Notifications API Specification

## Purpose

Provides real-time and stored user notification management, including Server-Sent Events (SSE) notification streaming and lifecycle management without deprecated hard deletion endpoints.

## Requirements

### Requirement: Real-Time SSE Notification Streaming
The system SHALL provide a `GET /notifications/stream` endpoint that establishes an HTTP Server-Sent Events connection returning `Content-Type: text/event-stream`.

#### Scenario: Open SSE stream
- **WHEN** an authenticated client sends an HTTP GET request to `/notifications/stream` with header `Accept: text/event-stream`
- **THEN** the system returns HTTP status code 200 with `Content-Type: text/event-stream` and begins streaming notification event frames.

#### Scenario: Unauthorized SSE stream attempt
- **WHEN** an unauthenticated client sends an HTTP GET request to `/notifications/stream`
- **THEN** the system returns HTTP status code 401 Unauthorized conforming to the `ErrorResponse` schema.

### Requirement: Absence of Deprecated Notification Dismissal Route
The system SHALL NOT expose or support `DELETE /notifications/{id}` in published OpenAPI specifications or automated Postman collections.

#### Scenario: Requesting removed deletion endpoint
- **WHEN** a client sends an HTTP DELETE request to `/notifications/{id}`
- **THEN** the system returns HTTP status code 404 Not Found conforming to the `ErrorResponse` schema.
