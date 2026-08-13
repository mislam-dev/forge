# Module 01 — Foundation & Project Setup

> **Module Type:** Cross-Cutting Infrastructure
> **Priority:** P0 — Blocker
> **Status:** Not Started
> **Last Updated:** 2026-08-13

---

## 1. Module Overview

### Purpose

The Foundation is not a business module — it is the **scaffolding that all other modules depend on**. It establishes the Axum application structure, configuration loading, shared types, error handling, request/response conventions, and the `AppState` that all handlers receive.

Without Foundation, no other module can be built.

### Responsibilities

- Cargo.toml with all production dependencies
- Axum application startup (`main.rs`)
- Environment-based configuration (`AppConfig`)
- Shared error types (`AppError`, `ApiError`)
- Shared response types (`ApiResponse<T>`, `PaginatedResponse<T>`)
- Shared pagination types (`PaginationParams`)
- Shared utility types (UUID aliases, timestamps)
- `AppState` struct holding all infrastructure handles
- Application-level middleware wiring (logging, CORS, request ID)
- Router setup (top-level route registration)

### Scope

**Included:**
- Cargo.toml dependency setup
- `main.rs` Tokio async entry point
- `AppConfig` loaded from environment variables
- `AppState` with `DatabaseConnection`, Redis client, RabbitMQ connection
- Shared response envelope: `{"message": "...", "data": {...}}`
- Shared error response: `{"message": "...", "errors": [...]}`
- `PaginationParams` (page, per_page)
- Global 404 handler
- CORS middleware
- Request ID middleware

**Excluded:**
- Business logic (belongs to individual modules)
- Authentication middleware (belongs to Auth module)
- RBAC middleware (belongs to Access Control module)

---

## 2. Current State

| File | Status |
|------|--------|
| `src/main.rs` | Stub — `println!("Hello, world!")` only |
| `src/lib.rs` | Empty |
| `src/app/app.rs` | Empty stub |
| `src/app/router.rs` | Empty stub |
| `src/app/state.rs` | Empty stub |
| `src/app/middleware.rs` | Empty stub |
| `src/app/mod.rs` | Empty stub |
| `src/config/env.rs` | Empty stub |
| `src/config/mod.rs` | Empty stub |
| `src/shared/error.rs` | Empty stub |
| `src/shared/response.rs` | Empty stub |
| `src/shared/types.rs` | Empty stub |
| `src/shared/pagination.rs` | Empty stub |
| `src/shared/mod.rs` | Empty stub |
| `Cargo.toml` | No dependencies — bare skeleton |
| `justfile` | Complete — no code changes needed |

> **Finding:** The directory structure is well-planned and matches the expected module organization. All files exist as empty stubs. Zero implementation.

---

## 3. Dependencies

### Depends On
- None — this is the root

### Used By
- Every module in the platform

### External Dependencies
- Tokio (async runtime)
- Axum (web framework)
- `dotenvy` or `config` crate for env loading
- `tracing` / `tracing-subscriber` (logging)
- `tower-http` (CORS, request logging)
- `serde` / `serde_json`

---

## 4. Requirements to Implement

From SRS section 6 (Non-Functional) and section 12 (Logging):

| Requirement | Implementation |
|-------------|---------------|
| NFR-001: 10,000+ concurrent API requests | Tokio multi-thread runtime, Axum async handlers |
| NFR-002: Structured logging | `tracing` JSON subscriber |
| NFR-003: Request IDs | Request ID middleware injects UUID per request |
| NFR-004: Consistent error responses | `AppError` -> `ApiError` response mapping |
| NFR-005: Proper HTTP status codes | `AppError` variants map to 4xx/5xx |
| NFR-006: Versioned API | Router prefix `/api/v1/` (check OpenAPI — spec uses no prefix) |

> **Note:** The OpenAPI spec uses paths like `/auth/register` without a `/api/v1/` prefix. Follow the OpenAPI spec exactly.

---

## 5. API Response Format

All responses follow the envelope format from OpenAPI examples:

```json
// Success
{
  "message": "Resource created successfully.",
  "data": { ... }
}

// Error
{
  "message": "Validation failed.",
  "errors": ["field: error description"]
}
```

---

## 6. AppState Structure

```rust
// Conceptual — actual implementation will evolve
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis: RedisClient,
    pub rabbitmq: Arc<lapin::Connection>,
    pub config: AppConfig,
    pub encryption: EncryptionService,
}
```

---

## 7. AppConfig Fields

| Field | Env Variable | Required |
|-------|-------------|----------|
| database_url | `DATABASE_URL` | Yes |
| redis_url | `REDIS_URL` | Yes |
| rabbitmq_url | `RABBITMQ_URL` | Yes |
| jwt_secret | `JWT_SECRET` | Yes |
| jwt_expiry_seconds | `JWT_EXPIRY_SECONDS` | No (default: 3600) |
| refresh_token_expiry_days | `REFRESH_TOKEN_EXPIRY_DAYS` | No (default: 7) |
| master_encryption_key | `MASTER_ENCRYPTION_KEY` | Yes |
| loki_url | `LOKI_URL` | No (optional for MVP) |
| service_token | `SERVICE_TOKEN` | Yes (Build Worker auth) |
| server_port | `SERVER_PORT` | No (default: 3000) |
| rust_log | `RUST_LOG` | No (default: info) |

---

## 8. Implementation Tasks

### Foundation
- [ ] Fix `Cargo.toml` — add all production dependencies
- [ ] Implement `AppConfig` struct in `src/config/env.rs` with `dotenvy` loading
- [ ] Panic on startup if required env vars are missing
- [ ] Implement `AppState` in `src/app/state.rs`
- [ ] Implement `AppState::new()` that initializes DB, Redis, RabbitMQ, Encryption

### Shared Types
- [ ] Implement `AppError` enum in `src/shared/error.rs` — covers DB errors, validation, auth, not-found, conflict, forbidden
- [ ] Implement `IntoResponse` for `AppError` (maps to appropriate HTTP status codes)
- [ ] Implement `ApiResponse<T>` in `src/shared/response.rs`
- [ ] Implement `PaginatedResponse<T>` with metadata (page, per_page, total, total_pages)
- [ ] Implement `PaginationParams` in `src/shared/pagination.rs` (page, per_page with defaults)
- [ ] Implement shared type aliases in `src/shared/types.rs`

### Application
- [ ] Implement `create_app(state: AppState) -> Router` in `src/app/app.rs`
- [ ] Implement router with all module route registrations in `src/app/router.rs`
- [ ] Implement global 404 handler
- [ ] Implement CORS middleware (configurable origins)
- [ ] Implement request ID middleware in `src/app/middleware.rs`
- [ ] Wire `main.rs` — async Tokio main, load config, initialize state, start Axum server

### Logging
- [ ] Initialize `tracing-subscriber` with JSON format in `main.rs`
- [ ] Inject request ID into tracing span context

### Testing
- [ ] Unit test: `AppConfig` loads correctly from env vars
- [ ] Unit test: `AppError` maps to correct HTTP status codes
- [ ] Unit test: `ApiResponse` serializes correctly
- [ ] Integration test: server starts and responds to `GET /health` (once Health module exists)
- [ ] Integration test: 404 handler returns correct JSON format

---

## 9. Definition of Done

- [ ] `cargo build` succeeds with all dependencies
- [ ] `cargo run` starts Axum server on `SERVER_PORT`
- [ ] Server returns structured JSON 404 for unknown routes
- [ ] Request ID header is present on all responses
- [ ] `AppError` variants map to correct HTTP status codes
- [ ] All shared types compile and serialize correctly
- [ ] Foundation unit tests pass

---

## 10. Logging

| Event | Level | Fields |
|-------|-------|--------|
| Application starting | INFO | port, environment |
| Database connected | INFO | host, pool_size |
| Redis connected | INFO | host |
| RabbitMQ connected | INFO | host, vhost |
| Configuration error | ERROR | field_name, reason |
| Startup failure | ERROR | component, error |

> **Security:** Never log `JWT_SECRET`, `MASTER_ENCRYPTION_KEY`, `DATABASE_URL` passwords, or `SERVICE_TOKEN` values.

---

## 11. Estimated Effort

**Large (3–5 days)**

The Foundation is the longest single step because it requires making all Cargo dependency decisions upfront, implementing the full error/response type system, and wiring the Axum application. Mistakes here propagate to every subsequent module.

---

## 12. Recommendations

**Required:**
- All `AppError` variants must be defined now — modules will add new variants but the core set must exist
- `AppConfig` must validate all required fields at startup and panic with a clear message if missing
- `ApiResponse<T>` serialization format must match the OpenAPI spec examples exactly

**Recommended:**
- Use `anyhow` for internal error context, but convert to `AppError` at handler boundaries
- Use `validator` crate for request body validation (consistent with module documentation)
- Implement `From<sea_orm::DbErr>` for `AppError` to avoid repetitive error mapping

**Future Enhancement:**
- Health check readiness probe before accepting traffic (wait for DB, Redis, RabbitMQ)
- Graceful shutdown with Tokio signal handling
