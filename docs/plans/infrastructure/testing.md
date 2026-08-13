# Infrastructure Plan: Testing Infrastructure

> **Plan Type:** Infrastructure
> **Priority:** P0 — Blocker
> **Status:** Not Started
> **Last Updated:** 2026-08-13

---

## 1. Overview

The Forge Platform SRS (section 13) requires:
- Unit Tests
- Integration Tests
- API Tests
- End-to-End Tests
- Load Testing

This plan covers the foundational testing infrastructure that all module tests will depend on: test database setup, fixtures, helpers, and test utilities.

---

## 2. Current State

| Item | Status |
|------|--------|
| Test utilities | None |
| Test database setup | None |
| Fixtures / seed data | None |
| API test helpers | None |
| `#[cfg(test)]` modules | None |

---

## 3. Dependencies

### Depends On
- Foundation (Cargo.toml)
- Database infrastructure (test DB migrations)
- Authentication infrastructure (test JWT tokens)

### Used By
- Every module's test suite

---

## 4. Required Cargo Dependencies

```toml
[dev-dependencies]
# HTTP client for API tests
reqwest = { version = "0.12", features = ["json"] }

# Test async runtime
tokio = { version = "1", features = ["full"] }

# Mocking
mockall = "0.13"

# Test containers (optional — for isolated test DB)
# testcontainers = "0.23"

# HTTP testing for Axum
axum-test = "15"

# Fake data generation
fake = { version = "2", features = ["derive"] }
```

---

## 5. Test Database Strategy

**Approach:** Separate test database (`forge_test`) with migrations run before test suite.

```bash
# Setup test DB (run once before tests)
DATABASE_URL=postgres://postgres:password@localhost/forge_test just db-up
```

**Per-test isolation:** Each integration test runs inside a database transaction that is rolled back at the end. This avoids expensive database teardown between tests.

---

## 6. Test Utilities

### Required Helpers

```rust
// Test app builder (Axum test client)
pub async fn create_test_app() -> TestClient;

// Authentication helpers
pub async fn create_test_user(db: &DatabaseConnection) -> (User, String); // returns (user, jwt)
pub async fn create_test_admin(db: &DatabaseConnection) -> (User, String);
pub async fn make_auth_header(token: &str) -> HeaderMap;

// Fixture builders
pub fn fake_register_request() -> RegisterRequest;
pub fn fake_org_create_request() -> CreateOrgRequest;
pub fn fake_project_create_request(org_id: Uuid) -> CreateProjectRequest;

// Database helpers
pub async fn setup_test_db() -> DatabaseConnection;
pub async fn teardown_test_db(db: &DatabaseConnection);
```

---

## 7. Test Organization

Each module should have tests organized as:

```
src/modules/<module>/
├── mod.rs
├── handlers.rs
├── service.rs
├── entities/
└── tests/
    ├── unit/
    │   ├── service_tests.rs
    │   └── validation_tests.rs
    └── integration/
        ├── api_tests.rs
        └── auth_tests.rs
```

---

## 8. Testing Standards

- **Unit tests:** Pure function tests, no database access. Use `mockall` for dependencies.
- **Integration tests:** Use real test database, rolled back after each test.
- **API tests:** Use `axum-test` with the full application stack.
- **Authorization tests:** Every protected endpoint must have at least one unauthorized test and one forbidden test.
- **Error case tests:** Every documented error case must have a corresponding test.

---

## 9. Implementation Tasks

### Cargo Setup
- [ ] Add test dependencies to `[dev-dependencies]` in Cargo.toml

### Test Utilities
- [ ] Create `src/test_utils/mod.rs` with shared test helpers
- [ ] Implement `create_test_app()` that returns an `axum-test` client
- [ ] Implement `create_test_user()` and `create_test_admin()` helpers
- [ ] Implement `make_auth_header()` JWT injection helper
- [ ] Implement fixture builders for common entities (user, org, project, deployment)

### Test Database
- [ ] Document how to create and migrate the test database
- [ ] Implement transaction-based test isolation
- [ ] Add `just test` command that runs against test database

### CI Integration
- [ ] Ensure `just ci` runs all tests
- [ ] Ensure tests fail on 0-coverage for key modules

---

## 10. Definition of Done

- [ ] Test utilities compile and can be imported by module tests
- [ ] `create_test_app()` returns a working test client
- [ ] `create_test_user()` creates a valid user with JWT
- [ ] Test database can be migrated and reset
- [ ] Example integration test passes (e.g., `POST /auth/register`)

---

## 11. Estimated Effort

**Small (< 1 day)**

Setting up the test infrastructure is quick, but it must be done before any module tests can be written.

---

## 12. Recommendations

**Required:**
- Every protected API endpoint must have an unauthorized (401) test.
- Every admin-only endpoint must have a forbidden (403) test for non-admin users.

**Recommended:**
- Use `fake` crate for generating realistic test data (realistic names, emails, UUIDs).
- Use transaction rollback for integration tests to keep the test database clean.

**Future Enhancement:**
- `testcontainers` for fully isolated Docker-based test databases.
- Load testing with `k6` or `criterion` for performance benchmarks.
