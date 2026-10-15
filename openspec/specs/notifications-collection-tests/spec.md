# Spec: Notifications Collection Tests

## Purpose

Provides automated contract test assertion suites for all Notifications endpoints in the Forge Platform API Postman collection, validating status codes, response schemas against OpenAPI contracts, and SSE streaming headers.

## Requirements

### Requirement: Notifications List Response Contract Validation
The system SHALL attach automated test assertion scripts to the `GET /notifications` collection request to validate notifications list responses (HTTP 200 OK conforming to `NotificationListResponse`) and error conditions (HTTP 401 Unauthorized conforming to `ErrorResponse`).

#### Scenario: Successful notifications list retrieval
- **WHEN** `GET /notifications` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data` array of notification items conforming to `NotificationResponse`, and pagination metadata if present.

#### Scenario: Unauthorized notifications list request
- **WHEN** `GET /notifications` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates authentication error.

### Requirement: Unread Notification Count Response Contract Validation
The system SHALL attach automated test assertion scripts to the `GET /notifications/unread-count` collection request to validate unread count responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized conforming to `ErrorResponse`).

#### Scenario: Successful unread notification count retrieval
- **WHEN** `GET /notifications/unread-count` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data` object with non-negative integer `unread_count`, and `message` string.

#### Scenario: Unauthorized unread count request
- **WHEN** `GET /notifications/unread-count` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates unauthorized request.

### Requirement: Mark Notification as Read Contract Validation
The system SHALL attach automated test assertion scripts to the `PATCH /notifications/{id}/read` collection request to validate single notification read status transitions (HTTP 200 OK conforming to `NotificationResponse`) and error responses (HTTP 401 Unauthorized, 403 Forbidden, or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful mark notification read
- **WHEN** `PATCH /notifications/{id}/read` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json`, response contains `data` with `id` matching the target notification, `is_read` is boolean `true`, and confirmation `message`.

#### Scenario: Notification read update failure or not found
- **WHEN** `PATCH /notifications/{id}/read` returns HTTP status code 401, 403, or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates failure reason.

### Requirement: Mark All Notifications Read Contract Validation
The system SHALL attach automated test assertion scripts to the `POST /notifications/read-all` collection request to validate bulk read update responses (HTTP 200 OK) and unauthorized requests (HTTP 401 Unauthorized conforming to `ErrorResponse`).

#### Scenario: Successful bulk read update
- **WHEN** `POST /notifications/read-all` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Unauthorized bulk read update
- **WHEN** `POST /notifications/read-all` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates authentication failure.

### Requirement: Dismiss Notification Response Contract Validation
The system SHALL attach automated test assertion scripts to the `DELETE /notifications/{id}` collection request to validate notification dismissal responses (HTTP 200 OK) and error conditions (HTTP 401 Unauthorized or 404 Not Found conforming to `ErrorResponse`).

#### Scenario: Successful notification dismissal
- **WHEN** `DELETE /notifications/{id}` returns HTTP status code 200
- **THEN** tests assert Content-Type is `application/json` and response contains confirmation `message`.

#### Scenario: Dismissal unauthorized or not found
- **WHEN** `DELETE /notifications/{id}` returns HTTP status code 401 or 404
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates error condition.

### Requirement: SSE Notification Stream Contract Validation
The system SHALL attach automated test assertion scripts to the `GET /notifications/stream` collection request to validate real-time SSE stream initiation (HTTP 200 OK with `text/event-stream` Content-Type) and unauthorized connection rejection (HTTP 401 Unauthorized conforming to `ErrorResponse`).

#### Scenario: Successful SSE notification stream connection
- **WHEN** `GET /notifications/stream` returns HTTP status code 200
- **THEN** tests assert Content-Type includes `text/event-stream` and transfer encoding or connection headers are appropriately configured for streaming.

#### Scenario: Unauthorized SSE stream request
- **WHEN** `GET /notifications/stream` returns HTTP status code 401
- **THEN** tests assert Content-Type is `application/json`, `is_error` is true, and `code` indicates unauthorized connection.
