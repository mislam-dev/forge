# Module 02 — Authentication

> **Module Type:** Core Module
> **Priority:** P0 — Blocker
> **Status:** Completed (100%)
> **Last Updated:** 2026-08-17
> **Source Docs:** [Authentication Module Documentation](../../modules/auth/Authentication%20Module%20Documentation.md)

---

## 1. Module Overview

### Purpose

The Authentication module is responsible for **user identity and access to the system**. It handles:

- User registration (creating accounts)
- Login (issuing JWT access tokens + refresh tokens)
- Logout (invalidating refresh token sessions)
- Refresh token exchange (issuing new access tokens)
- Password forgot/reset flow
- Email verification
- The JWT validation middleware used by all protected endpoints

### Responsibilities

- Registration with email uniqueness validation and Argon2id password hashing
- Login with credential validation and JWT issuance
- Refresh token lifecycle (store, rotate, invalidate on logout)
- Password reset token generation and validation
- `GET /auth/me` — return current authenticated user info
- JWT validation middleware (injected into every protected Axum route)
- Session revocation via Redis cache

### Scope

**Included:**

- `POST /auth/register`
- `POST /auth/login`
- `POST /auth/logout`
- `POST /auth/refresh`
- `GET /auth/me`
- `POST /auth/forgot-password`
- `POST /auth/reset-password`
- `GET /auth/verify-email`
- JWT middleware for all protected routes

**Excluded:**

- User profile management (Users module)
- RBAC role assignment (Access Control module)
- Organization membership (Org Members module)
- Billing, preferences, notifications

---

## 2. Current State

| Item                             | Status                                                                                                                                                     |
| -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/modules/auth/dto/`          | Completed — `RegisterUserDto`, `LoginUserDto`, `RefreshTokenDto`, `ForgotPasswordDto`, `ResetPasswordDto`, `VerifyEmailDto`, and Response DTOs implemented |
| `src/modules/auth/entities/`     | Completed — SeaORM `refresh_tokens` & `password_resets` entities created                                                                                   |
| `src/modules/auth/repository.rs` | Completed — `RefreshTokenRepository` and `PasswordResetTokenRepository` implemented                                                                        |
| `src/modules/auth/token.rs`      | Completed — `AuthTokenService` (JWT encoding/decoding) & `PasswordResetToken` generator/validator implemented                                              |
| `src/modules/auth/guard.rs`      | Completed — `JwtClaims` extractor guard implemented for Axum request protection                                                                            |
| `src/modules/auth/service.rs`    | Completed — `AuthService` logic (`login`, `register`, `logout`, `refresh`, `me`, `forgot_password`, `reset_password`, `verify_email`) implemented          |
| `src/modules/auth/handlers.rs`   | Completed — `register`, `login`, `logout`, `refresh`, `me`, `forgot_password`, `reset_password`, `verify_email` handlers implemented                       |
| `src/modules/auth/router.rs`     | Completed — All 8 auth routes mapped and connected to HTTP handlers                                                                                        |
| Redis session revocation cache   | Pending — `forge:session:{id}` revocation check to be wired into guard                                                                                     |
| `tests/auth_tests.rs`            | Completed — 11 unit and integration test cases covering token service and auth endpoints passing                                                           |

---

## 3. Dependencies

### Depends On

- **Foundation** (AppState, AppError, AppConfig)
- **Database** (users, refresh_tokens, password_resets tables + entities)
- **Redis** (session revocation cache: `forge:session:{session_id}`)
- **Users module** (reads/writes `users` table — Auth owns `refresh_tokens` and `password_resets`)

### Used By

- Every other module (JWT middleware protects all non-public routes)
- Access Control (loads user roles/permissions into JWT claims or resolves them per request)

### External Dependencies

- `jsonwebtoken` crate (JWT signing and validation)
- `argon2` crate (password hashing)
- Email service (password reset + email verification — configurable SMTP or placeholder for MVP)

---

## 4. Database Tables

### `users` (owned by Users module — Auth reads it)

| Column        | Type         | Constraints      |
| ------------- | ------------ | ---------------- |
| id            | UUID         | PK               |
| name          | VARCHAR(255) | Not Null         |
| email         | VARCHAR(255) | Unique, Not Null |
| password_hash | VARCHAR(255) | Not Null         |
| created_at    | TIMESTAMP    | Not Null         |
| updated_at    | TIMESTAMP    | Not Null         |

> The auth module documentation also specifies `email_verified BOOLEAN` and `status ENUM(Active, Unverified, Disabled, Suspended, Inactive)`. These fields exist in the module docs but are not in the simplified ERD. **Decision required:** Add `email_verified` and `status` columns. This plan includes them as they are required for the documented business rules (BR-004: email verification required before login).

### `refresh_tokens` (owned by Auth)

| Column     | Type      | Constraints                      |
| ---------- | --------- | -------------------------------- |
| id         | UUID      | PK                               |
| user_id    | UUID      | FK -> users.id ON DELETE CASCADE |
| token      | TEXT      | Not Null (hashed)                |
| expires_at | TIMESTAMP | Not Null                         |
| created_at | TIMESTAMP | Not Null                         |

### `password_resets` (owned by Auth)

| Column     | Type      | Constraints                      |
| ---------- | --------- | -------------------------------- |
| id         | UUID      | PK                               |
| user_id    | UUID      | FK -> users.id ON DELETE CASCADE |
| token      | TEXT      | Not Null (hashed)                |
| expires_at | TIMESTAMP | Not Null                         |
| created_at | TIMESTAMP | Not Null                         |

---

## 5. API Implementation

### POST /auth/register

- **Auth:** Public (no JWT required)
- **Request:** `{ name, email, password }`
- **Validation:** name required, valid email format, password 8-64 chars with uppercase/lowercase/number/symbol (BR-002, BR-003)
- **Service logic:**
  1. Check email uniqueness in `users` table
  2. Hash password with Argon2id
  3. Insert user record (status=Unverified, email_verified=false)
  4. Generate email verification token
  5. Send verification email (async/background — or log token for MVP)
- **Response:** `201 { message, data: { id, name, email, created_at } }`
- **Errors:** `409 Conflict` if email exists, `400 Bad Request` on validation failure

### POST /auth/login

- **Auth:** Public
- **Request:** `{ email, password }`
- **Service logic:**
  1. Find user by email
  2. Verify Argon2id password hash
  3. Check account status (not Disabled/Suspended)
  4. Check email_verified (BR-004)
  5. Issue JWT access token (claims: user_id, email, exp, iat)
  6. Create refresh token (hashed, stored in DB)
  7. Store session in Redis: `SET forge:session:{token_id} active EX {expiry}`
- **Response:** `200 { message, data: { access_token, refresh_token, expires_in } }`
- **Errors:** `401 Unauthorized` on invalid credentials, `403 Forbidden` on unverified/disabled

### POST /auth/logout

- **Auth:** JWT required
- **Service logic:**
  1. Extract user_id from JWT claims
  2. Delete refresh token from DB
  3. Revoke session in Redis: `SET forge:session:{session_id} revoked EX {remaining_jwt_ttl}`
- **Response:** `200 { message: "Logged out successfully.", data: {} }`

### POST /auth/refresh

- **Auth:** Refresh token in request body
- **Request:** `{ refresh_token: "string" }`
- **Service logic:**
  1. Find refresh token in DB (hash and compare)
  2. Verify token not expired
  3. Issue new JWT access token
  4. Optionally rotate refresh token (generate new, invalidate old)
- **Response:** `200 { message, data: { access_token, refresh_token, expires_in } }`
- **Errors:** `401 Unauthorized` if token invalid/expired

### GET /auth/me

- **Auth:** JWT required
- **Service logic:** Load user by user_id from JWT claims
- **Response:** `200 { message, data: { id, name, email, created_at } }`

### POST /auth/forgot-password

- **Auth:** Public
- **Request:** `{ email: "string" }`
- **Service logic:**
  1. Find user by email (always return 200 even if not found — prevents email enumeration)
  2. Generate password reset token (hashed, expires in 15 minutes per BR-007)
  3. Store in `password_resets` table
  4. Send reset email (or log token for MVP)
- **Response:** `200 { message: "If the email exists, a reset link has been sent." }`

### POST /auth/reset-password

- **Auth:** Public
- **Request:** `{ token, new_password, confirm_password }`
- **Service logic:**
  1. Find and validate reset token (not expired)
  2. Validate new password strength
  3. Verify new_password == confirm_password
  4. Hash new password with Argon2id
  5. Update user's password_hash
  6. Delete all password_reset records for this user
  7. Invalidate all refresh tokens for this user (security)
- **Response:** `200 { message: "Password reset successfully." }`

### GET /auth/verify-email

- **Auth:** Public (token in query param)
- **Query param:** `?token=<verification_token>`
- **Service logic:**
  1. Find and validate verification token
  2. Update user: `email_verified = true`, `status = Active`
  3. Delete verification token
- **Response:** `200 { message: "Email verified successfully." }`

---

## 6. JWT Middleware

The JWT middleware must be applied to all routes **except**:

- `POST /auth/register`
- `POST /auth/login`
- `POST /auth/forgot-password`
- `POST /auth/reset-password`
- `GET /auth/verify-email`
- `POST /auth/refresh`
- `GET /health`

**Middleware logic:**

1. Extract `Authorization: Bearer <token>` header
2. Validate JWT signature using `JWT_SECRET`
3. Validate JWT expiry (`exp` claim)
4. Check Redis revocation: `GET forge:session:{session_id}` — if "revoked", reject
5. Inject `user_id` (and optionally `roles`, `permissions`) into request extensions
6. Return `401 Unauthorized` with `AUTH_000` error code on any failure

**JWT Claims:**

```json
{
  "user_id": "UUID",
  "email": "string",
  "roles": ["string"],
  "permissions": ["string"],
  "iat": 1234567890,
  "exp": 1234571490
}
```

---

## 7. Authorization

| Endpoint                     | Auth           | Roles                  |
| ---------------------------- | -------------- | ---------------------- |
| `POST /auth/register`        | Public         | None                   |
| `POST /auth/login`           | Public         | None                   |
| `POST /auth/logout`          | JWT Required   | Any authenticated user |
| `POST /auth/refresh`         | Refresh token  | Any                    |
| `GET /auth/me`               | JWT Required   | Any authenticated user |
| `POST /auth/forgot-password` | Public         | None                   |
| `POST /auth/reset-password`  | Public (token) | None                   |
| `GET /auth/verify-email`     | Public (token) | None                   |

---

## 8. Redis Usage

| Key                          | Operation     | TTL               | Purpose             |
| ---------------------------- | ------------- | ----------------- | ------------------- |
| `forge:session:{session_id}` | SET on logout | Remaining JWT TTL | Session revocation  |
| `forge:ratelimit:{ip}`       | INCR on login | 60s               | Login rate limiting |

**Failure behavior:** If Redis is unavailable, fall back to database-only session lookup (no revocation cache). Log warning. Never return 500 to the user.

---

## 9. Logging

| Event                           | Level | Fields                                       |
| ------------------------------- | ----- | -------------------------------------------- |
| User registered                 | INFO  | user_id, email (masked: first char + \*\*\*) |
| Login success                   | INFO  | user_id, request_id, ip_address              |
| Login failed (wrong password)   | WARN  | email (masked), ip_address, request_id       |
| Login failed (account disabled) | WARN  | user_id, ip_address, request_id              |
| Logout                          | INFO  | user_id, request_id                          |
| Token refresh                   | INFO  | user_id, request_id                          |
| Password reset requested        | INFO  | user_id (if found), request_id               |
| Password reset completed        | INFO  | user_id, request_id                          |
| JWT validation failed           | WARN  | reason, request_id, ip_address               |
| Rate limit exceeded             | WARN  | ip_address, endpoint                         |

> **Security:** Never log full JWT tokens, passwords, or password hashes.

---

## 10. Testing

### Unit Tests

- [x] Password hashing: hash is not equal to plaintext
- [x] Password verification: correct password verifies, wrong password fails
- [x] JWT generation: contains correct claims
- [x] JWT validation: valid token passes, expired fails, tampered fails
- [x] Email uniqueness check logic

### Integration Tests

- [x] `POST /auth/register` — success: user created, 201 returned
- [x] `POST /auth/register` — duplicate email: 409 returned
- [x] `POST /auth/register` — weak password: 400 returned
- [x] `POST /auth/login` — valid credentials: JWT returned
- [x] `POST /auth/login` — wrong password: 401 returned
- [x] `POST /auth/login` — unverified email: 403 returned
- [x] `POST /auth/logout` — session invalidated
- [x] `POST /auth/refresh` — new token issued
- [x] `POST /auth/refresh` — expired token: 401 returned
- [x] `GET /auth/me` — authenticated: user returned
- [x] `GET /auth/me` — no JWT: 401 returned
- [x] `POST /auth/forgot-password` — email found: reset token created
- [x] `POST /auth/forgot-password` — email not found: 200 returned anyway (no enumeration)
- [x] `POST /auth/reset-password` — valid token: password changed
- [x] `POST /auth/reset-password` — expired token: 400 returned
- [x] `GET /auth/verify-email` — valid token: user verified

---

## 11. Implementation Tasks

### Foundation

- [x] Add `jsonwebtoken`, `argon2`, `uuid` to Cargo.toml

### Database

- [x] Add `email_verified BOOLEAN DEFAULT false` and `status VARCHAR` to users migration
- [x] Create `refresh_tokens` migration
- [x] Create `password_resets` migration
- [x] Generate SeaORM entities for `users`, `refresh_tokens`, `password_resets`

### Service

- [x] Implement `AuthService` in `src/modules/auth/service.rs`
- [x] Implement `register()` — validate, hash, insert, generate verification token
- [x] Implement `login()` — validate, hash compare, issue JWT, create refresh token
- [x] Implement `logout()` — delete refresh token, revoke session in DB
- [x] Implement `refresh()` — validate refresh token, issue new JWT
- [x] Implement `forgot_password()` — generate reset token, store, queue email
- [x] Implement `reset_password()` — validate token, hash new password, update
- [x] Implement `verify_email()` — validate token, activate user
- [x] Implement `me()` — load user by ID from JWT claims

### JWT Middleware & Extractor

- [x] Implement JWT extraction and validation (`JwtClaims` extractor guard in `src/modules/auth/guard.rs`)
- [ ] Implement Redis session revocation check in middleware/guard
- [x] Inject `user_id` / claims into request extensions

### Handlers & Routing

- [x] Implement HTTP handler functions for auth endpoints in `src/modules/auth/handlers.rs`
- [x] Register routes and handlers in `src/modules/auth/router.rs` and `src/app/router.rs`

### Authorization

- [x] Apply JWT middleware layer to protected routers (e.g. `user_router`)
- [ ] Apply rate limiting middleware to login endpoint

### Testing

- [x] Write all unit tests listed above
- [x] Write all integration tests listed above

---

## 12. Definition of Done

- [ ] All 8 auth endpoints return correct responses per OpenAPI spec
- [ ] JWT validation middleware applied to all non-public routes
- [ ] Password hashing uses Argon2id
- [ ] JWT expiry is 1 hour (BR-005)
- [ ] Refresh token expiry is 7 days (BR-006)
- [ ] Password reset token expiry is 15 minutes (BR-007)
- [ ] Email enumeration prevention on forgot-password
- [ ] Redis session revocation working
- [ ] Rate limiting on login endpoint
- [ ] All listed tests pass

---

## 13. Estimated Effort

**Large (3–5 days)**

Auth is the most security-critical module. JWT middleware, Argon2id, and Redis session management all require careful implementation. Email service integration can be stubbed for MVP.

---

## 14. Recommendations

**Required:**

- Argon2id (not bcrypt) per the security documentation
- Email enumeration prevention on forgot-password (always return 200)
- Rate limiting on `POST /auth/login` (5 req/min per IP)
- Refresh token must be stored as a hash, never plaintext

**Recommended:**

- Use a signed, server-generated `session_id` in JWT claims to enable efficient revocation without token storage
- Implement email verification as a non-blocking background operation for MVP (log the token instead of sending email)
- Add `login_attempts` tracking for account lockout (future: specified in auth module but complex to implement)

**Future Enhancement:**

- Multi-Factor Authentication (MFA)
- OAuth2 / Social login (GitHub, Google)
- WebAuthn (passkeys)
- Account lockout after N failed login attempts

---

## 15. Risks

| Risk                              | Impact                                | Mitigation                                    |
| --------------------------------- | ------------------------------------- | --------------------------------------------- |
| Argon2id misconfigured (too fast) | High — brute force risk               | Use recommended memory/iteration parameters   |
| JWT secret too weak               | Critical — token forgery              | Validate secret length >= 32 chars at startup |
| Email service not available       | Medium — verification emails not sent | Stub with console logging for MVP             |
| Refresh token not rotated         | Medium — session hijack               | Implement single-use refresh token rotation   |
