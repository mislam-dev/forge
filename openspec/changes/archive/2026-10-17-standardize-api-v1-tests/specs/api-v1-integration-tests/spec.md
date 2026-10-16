# Spec Delta

## Purpose

Standardizes the Rust Axum integration test suites and application router on a common canonical `/api/v1` base path prefix, eliminating hardcoded unversioned routes and ensuring all automated tests and routers compose paths uniformly from a single source of truth without backward-compatibility shims.

## ADDED Requirements

### Requirement: Centralized Common API Base Path
The system SHALL define a centralized, exportable constant for the API base path (defaulting to `"/api/v1"`) used by the application router to nest all domain service routes and available to integration test suites for consistent URI composition.

#### Scenario: Routing traffic through common base path
- **WHEN** the Axum application router is constructed via `create_app`
- **THEN** all versioned modular routes are mounted under the common base path constant (`API_V1_PATH`), serving requests accurately under that prefix

### Requirement: Standardized API v1 Authentication Integration Tests
The system SHALL execute integration test suites for authentication against the canonical `/api/v1/auth` endpoint namespace composed with the common base path, verifying registration, login, logout, token refresh, current user profile, password reset, and email verification without unversioned route aliases.

#### Scenario: Authentication endpoints target /api/v1/auth
- **WHEN** authentication integration tests execute against the application router
- **THEN** requests target `/api/v1/auth/me`, `/api/v1/auth/register`, `/api/v1/auth/login`, `/api/v1/auth/logout`, `/api/v1/auth/refresh`, `/api/v1/auth/forgot-password`, `/api/v1/auth/reset-password`, and `/api/v1/auth/verify-email`, receiving expected status codes and conforming responses instead of 404 Not Found

### Requirement: Standardized API v1 Teams Integration Tests
The system SHALL execute integration test suites for teams against the canonical `/api/v1/teams` endpoint namespace composed with the common base path, verifying team creation, listing, retrieval, update, deletion, and membership management without unversioned route aliases.

#### Scenario: Teams endpoints target /api/v1/teams
- **WHEN** teams integration tests execute against the application router
- **THEN** requests target `/api/v1/teams`, `/api/v1/teams/{id}`, `/api/v1/teams/{id}/members`, and `/api/v1/teams/{id}/members/{user_id}`, receiving expected status codes and conforming responses instead of 404 Not Found

### Requirement: Standardized API v1 Notifications Integration Tests
The system SHALL execute integration test suites for notifications against the canonical `/api/v1/notifications` endpoint namespace composed with the common base path, verifying notification listing, unread counts, marking single notification as read, and marking all notifications as read without unversioned route aliases.

#### Scenario: Notifications endpoints target /api/v1/notifications
- **WHEN** notifications integration tests execute against the application router
- **THEN** requests target `/api/v1/notifications`, `/api/v1/notifications/unread-count`, `/api/v1/notifications/{id}/read`, and `/api/v1/notifications/read-all`, receiving expected status codes and conforming responses instead of 404 Not Found

### Requirement: Standardized API v1 Dashboard and Health Integration Tests
The system SHALL execute integration test suites for dashboard and health against the canonical `/api/v1/dashboard` and `/api/v1/health` endpoint namespaces composed with the common base path, verifying summary metrics, user metrics, organization metrics, and health probe endpoints without unversioned route aliases.

#### Scenario: Dashboard and health endpoints target /api/v1 namespaces
- **WHEN** dashboard and health integration tests execute against the application router
- **THEN** requests target `/api/v1/dashboard`, `/api/v1/dashboard/user`, `/api/v1/dashboard/org/{id}`, `/api/v1/health`, and `/api/v1/health/details`, receiving expected status codes and conforming responses instead of 404 Not Found

### Requirement: Standardized API v1 User Profile Integration Tests
The system SHALL execute integration test suites for user profiles against the canonical `/api/v1/users/{id}/profile` endpoint namespace across all profile management operations without legacy unversioned routes.

#### Scenario: User profile operations target /api/v1/users/{id}/profile
- **WHEN** user profile integration tests execute against the application router
- **THEN** all requests, including profile deletion and retrieval, target `/api/v1/users/{id}/profile`, receiving expected status codes instead of 404 Not Found
