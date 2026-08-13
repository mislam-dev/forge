# Infrastructure Plan: Redis

> **Plan Type:** Infrastructure
> **Priority:** P0 — Blocker
> **Status:** Not Started
> **Last Updated:** 2026-08-13
> **ADR:** [ADR-003](../../system/09-adr/ADR-003-redis-caching-layer.md)

---

## 1. Overview

Redis is adopted **exclusively as an in-memory caching, rate-limiting, and session revocation layer** for the Forge Platform (ADR-003). It is **not** a primary data store. It is **not** used for message queuing (that is RabbitMQ per ADR-004).

**Three use cases only:**
1. **Dashboard metrics caching** — key: `forge:dashboard:{org_id}`, TTL: 60–300s
2. **Rate limiting** — key: `forge:ratelimit:{ip}` or `forge:ratelimit:{user_id}`, TTL: 60s
3. **Session revocation** — key: `forge:session:{session_id}`, TTL: matches JWT expiry

Redis client lives in `src/infrastructure/` and is shared via `AppState`.

---

## 2. Current State

| Item | Status |
|------|--------|
| `src/infrastructure/` directory | Exists |
| Redis client implementation | Not implemented |
| Redis connection in AppState | Not implemented |
| Rate limiting middleware | Not implemented |
| Dashboard cache helpers | Not implemented |
| Session revocation helpers | Not implemented |

---

## 3. Dependencies

### Depends On
- Foundation (Cargo.toml, AppState)
- Redis 7+ server (Docker Compose service)

### Used By
- Authentication module (session revocation)
- Dashboard module (metrics caching)
- All modules (rate limiting middleware)

---

## 4. Required Cargo Dependencies

```toml
[dependencies]
# Redis async client
redis = { version = "0.26", features = ["tokio-comp", "connection-manager"] }

# JSON serialization for cached values
serde_json = "1"
serde = { version = "1", features = ["derive"] }
```

---

## 5. Key Naming Convention

Per ADR-003, all Redis keys must follow this namespace hierarchy:

| Key Pattern | Module | Purpose | TTL |
|-------------|--------|---------|-----|
| `forge:dashboard:{org_id}` | Dashboard | Cached org metrics | 60–300s |
| `forge:ratelimit:{ip}` | Security | IP-based rate counter | 60s |
| `forge:ratelimit:{user_id}` | Security | User-based rate counter | 60s |
| `forge:session:{session_id}` | Auth | Session revocation status | Matches JWT expiry (~3600s) |
| `forge:lock:dashboard:{org_id}` | Dashboard | Stampede prevention mutex | 5s |

> **Rule:** Every key MUST have an explicit TTL. Permanent keys without TTL are forbidden per ADR-003.

---

## 6. Failure Behavior (Fail-Open)

Per ADR-003, Redis is a **non-critical performance dependency**:

- If Redis is unavailable during a cache read: log `WARN`, fall back to PostgreSQL query.
- If Redis is unavailable during rate-limit check: log `WARN`, allow request through (fail-open).
- If Redis is unavailable during session check: fall back to database session lookup.
- Redis outage must NOT cause HTTP 500 errors for end users.
- Health probe reports Redis as `Degraded` (not `Critical`) on failure.

---

## 7. Implementation Tasks

### Cargo Setup
- [ ] Add `redis`, `serde_json`, `serde` to Cargo.toml

### Redis Client
- [ ] Implement `RedisClient` wrapper in `src/infrastructure/` (or name the module appropriately)
- [ ] Connection string from environment variable `REDIS_URL`
- [ ] Use `ConnectionManager` for automatic reconnection
- [ ] Expose `RedisClient` via `AppState`

### Cache Helpers
- [ ] `cache_get(key: &str) -> Option<T>` — deserialize from JSON
- [ ] `cache_set(key: &str, value: &T, ttl_seconds: u64)` — serialize to JSON with TTL
- [ ] `cache_delete(key: &str)` — invalidate
- [ ] `cache_lock(key: &str, ttl_seconds: u64) -> bool` — mutex for stampede prevention

### Rate Limiting
- [ ] Implement rate limit middleware using `INCR` + `EXPIRE` pattern
- [ ] Configurable thresholds per endpoint category (auth endpoints: 5/min; API: 100/min)
- [ ] Inject `X-RateLimit-Limit`, `X-RateLimit-Remaining`, `X-RateLimit-Reset` headers
- [ ] Return `429 Too Many Requests` on threshold breach

### Session Revocation
- [ ] `revoke_session(session_id, ttl)` — set key to mark as revoked
- [ ] `is_session_revoked(session_id) -> bool` — check key existence
- [ ] Integrate into JWT validation middleware

### Testing
- [ ] Unit test: cache get/set/delete cycle
- [ ] Unit test: TTL expiry (use short TTL in tests)
- [ ] Unit test: rate limit counter increments correctly
- [ ] Unit test: rate limit 429 returned at threshold
- [ ] Unit test: session revocation check
- [ ] Integration test: Redis unavailable -> fallback behavior (no 500 error)

---

## 8. Definition of Done

- [ ] Redis client connects from environment variable
- [ ] `AppState` exposes Redis client
- [ ] Cache get/set/delete helpers implemented and tested
- [ ] Rate limiting middleware applied to auth and API routes
- [ ] Session revocation integration with JWT middleware
- [ ] Redis failure does NOT crash the API (fail-open verified)
- [ ] All tests pass

---

## 9. Estimated Effort

**Medium (1–2 days)**

The Redis client setup is straightforward. Rate limiting middleware requires careful integration with Axum middleware stack.

---

## 10. Recommendations

**Required:**
- All three use cases from ADR-003 must be implemented.
- No plaintext secrets must ever be stored in Redis values.
- All keys must have explicit TTLs.

**Recommended:**
- Use `ConnectionManager` instead of `Client` for production-grade reconnection handling.
- Serialize cached DTOs as JSON strings (not binary) for debuggability via `redis-cli`.

**Future Enhancement:**
- Redis Sentinel or Redis Cluster support for high-availability production environments.
- Sliding window rate limiting instead of fixed window (more accurate but more complex).
