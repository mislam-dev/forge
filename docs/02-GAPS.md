# Forge Platform — Gaps Analysis (Production & Portfolio)

> **Date:** October 15, 2026  
> **Auditor:** Senior Staff Engineer & Technical Hiring Reviewer  
> **Target Project:** Forge — Self-Hosted Developer Platform  

---

## Part A: Production Gaps

The following table categorizes all missing, weak, or regressed components blocking production deployment.

| Gap ID | Title | Severity | Impacted Component | Evidence File / Location |
| :--- | :--- | :---: | :--- | :--- |
| **GAP-P01** | Pseudo-Encryption of GitHub PATs & Env Secrets | **Blocker** | Security / Crypto | `src/modules/projects/repositories/utils/access_token_service.rs:4-18` |
| **GAP-P02** | Container Healthcheck 404 Failure | **Blocker** | Deployment / Docker | `Dockerfile:59` vs `src/app/app.rs:20-30` |
| **GAP-P03** | Route Aliasing Regressions & 37 Broken Tests | **Blocker** | Routing / Integration | `src/app/app.rs:21-29` vs `tests/auth_tests.rs:150` |
| **GAP-P04** | Missing DELETE Route in User Profile Router | **High** | API / Profile | `src/modules/users/profile/router.rs:11-14` |
| **GAP-P05** | Global 404 Response Contract Mismatch | **High** | Foundation / Error | `src/app/app.rs:46` vs `tests/foundation_tests.rs:57` |
| **GAP-P06** | Secret Value Leak in Non-Secret Env Var DTO | **High** | Projects / Env Vars | `src/modules/projects/environment_variables/dto/response.rs:25` |
| **GAP-P07** | Duplicate RabbitMQ Connection Initialization | **High** | Infrastructure / Queue | `src/main.rs:25, 29` & `src/app/state.rs:25` |
| **GAP-P08** | Hardcoded Loki URL in Tracing Subscriber | **Medium** | Observability / Logging | `src/shared/logger.rs:39` |
| **GAP-P09** | Unimplemented Email Verification & Reset Delivery | **Medium** | Auth / Notifications | `src/modules/auth/service.rs:201, 245` |
| **GAP-P10** | Orphaned Redis Infrastructure (ADR-003 Omission) | **Medium** | Architecture / Cache | `src/modules/auth/service.rs:71, 108` & `Cargo.toml` |
| **GAP-P11** | Zero API Rate Limiting Middleware | **Medium** | Security / Resilience | `src/app/app.rs:32-38` (Missing Governor/RateLimit) |
| **GAP-P12** | Absence of CI/CD Automation Workflows | **Medium** | DevOps / Quality Gate | Repository root (Missing `.github/workflows/`) |
| **GAP-P13** | 138 Clippy Warnings & Strict CI Failure | **Low** | Code Quality / Lint | `cargo clippy --all-targets --all-features` output |
| **GAP-P14** | Code Formatting Inconsistencies | **Low** | Repository Hygiene | `cargo fmt --check` exit code 1 |

---

### Detailed Production Gap Specifications

#### GAP-P01: Pseudo-Encryption of GitHub PATs & Env Secrets
- **Why It Matters:** GitHub Personal Access Tokens and custom environment variables (database passwords, external API keys) grant access to critical client systems. The system claims to use AES-256-GCM encryption, but actually uses hexadecimal encoding. If the PostgreSQL database is breached or inspected via read replicas/logs, all credentials are instantly compromised.
- **Evidence:** 
  - [`src/modules/projects/repositories/utils/access_token_service.rs:4-6`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/repositories/utils/access_token_service.rs#L4-L6):
    ```rust
    pub fn encrypt(raw_token: &str) -> String {
        raw_token.bytes().map(|b| format!("{:02x}", b)).collect()
    }
    ```
  - [`src/modules/projects/environment_variables/service.rs:49-51`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/environment_variables/service.rs#L49-L51):
    ```rust
    fn encrypt_value(raw_value: &str) -> String {
        raw_value.bytes().map(|b| format!("{:02x}", b)).collect()
    }
    ```
  - [`src/infrastructure/encryption/mod.rs`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/infrastructure/encryption/mod.rs) is completely empty (0 bytes).
- **Severity:** **Blocker**
- **What "Done" Looks Like:** Implement true AES-256-GCM authenticated encryption in `src/infrastructure/encryption/` using `aes-gcm` or `ring`, keyed by `AppConfig.secrets.master_encryption_key`, with randomized nonces prepended to the ciphertext and authenticated decryption.

---

#### GAP-P02: Container Healthcheck 404 Failure
- **Why It Matters:** Docker, Kubernetes, AWS ECS, and Docker Compose rely on container healthchecks to determine service readiness. Because `/health` returns 404, container orchestrators report the container as `unhealthy`, drop it from ingress routing, or enter crash restart loops in production.
- **Evidence:** 
  - [`Dockerfile:58-59`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/Dockerfile#L58-L59):
    ```dockerfile
    HEALTHCHECK --interval=15s --timeout=5s --start-period=10s --retries=3 \
        CMD curl -f http://localhost:9000/health || exit 1
    ```
  - Command output:
    ```bash
    curl -i http://localhost:9000/health
    # Output: HTTP/1.1 404 Not Found
    # {"code":"SYS_004","errors":null,"is_error":true,"message":"The requested resource was not found"}
    ```
- **Severity:** **Blocker**
- **What "Done" Looks Like:** Mount `.nest("/health", health_router())` alongside `/api/v1/health` in `src/app/app.rs`, and update the Dockerfile HEALTHCHECK command to hit `http://localhost:9000/health/live` or `/api/v1/health/live` with an HTTP 200 response.

---

#### GAP-P03: Route Aliasing Regressions & 37 Broken Tests
- **Why It Matters:** In commit `8d7a8e7`, route aliases (`/api/*` and `/*`) were removed in favor of `/api/v1/*`. Consequently, 37 automated integration tests across Auth, Teams, Notifications, Dashboard, and Health fail. Furthermore, external webhook receivers or client libraries expecting `/api/...` break.
- **Evidence:**
  - [`src/app/app.rs:21-29`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/app/app.rs#L21-L29) mounts only:
    ```rust
    .nest("/api/v1/auth", auth_router())
    .nest("/api/v1/users", user_router())
    .nest("/api/v1/teams", teams_router())
    ...
    ```
  - Test suites (`tests/auth_tests.rs:150, 199`, `tests/teams_tests.rs:39, 55`, `tests/notifications_tests.rs:28`, `tests/dashboard_tests.rs:28`) hit `/api/...` and panic with `assertion failed: left: 404, right: 400/401`.
- **Severity:** **Blocker**
- **What "Done" Looks Like:** Restore route aliases in `src/app/app.rs` by aliasing or merging router definitions (e.g. mounting both `/api/v1/*` and `/api/*`, or aligning integration tests and updating documentation contracts), ensuring `cargo test --no-fail-fast` runs with 100% pass rate.

---

#### GAP-P04: Missing DELETE Route in User Profile Router
- **Why It Matters:** An endpoint expected by the API contract and user deletion lifecycle fails with HTTP 405 Method Not Allowed because the HTTP verb is missing from the router registration.
- **Evidence:**
  - [`src/modules/users/profile/router.rs:11-14`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/users/profile/router.rs#L11-L14):
    ```rust
    Router::new()
        .route("/{id}/profile", get(get_profile))
        .route("/{id}/profile", put(update_profile))
        .route_layer(middleware::from_extractor::<AccessTokenClaims>())
    ```
  - [`tests/user_profile_tests.rs:138`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/tests/user_profile_tests.rs#L138) sends `DELETE /{id}/profile` and panics with:
    `assertion left == right failed (left: 405, right: 403)`.
- **Severity:** **High**
- **What "Done" Looks Like:** Register `.delete(delete_profile)` handler on the profile route with proper RBAC permission checks (`admin` or owner self-delete guard).

---

#### GAP-P05: Global 404 Response Contract Mismatch
- **Why It Matters:** API error consistency is crucial for automated API clients. Commit `55e3996` changed the not-found message, but did not reconcile `foundation_tests.rs`.
- **Evidence:**
  - [`src/app/app.rs:46`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/app/app.rs#L46): returns `"The requested resource was not found"`.
  - [`tests/foundation_tests.rs:57`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/tests/foundation_tests.rs#L57): asserts `"you requested resource not found"`, causing test panic.
- **Severity:** **High**
- **What "Done" Looks Like:** Harmonize the response contract in both `app.rs` and `foundation_tests.rs` to match the OpenAPI specification standard error message.

---

#### GAP-P06: Secret Value Leak in Non-Secret Env Var DTO
- **Why It Matters:** When a user marks an environment variable as non-secret, the API returns the raw database storage string. Because the database stores values as hex strings, the API returns hex bytes rather than the plain value, corrupting build worker injections and UI displays.
- **Evidence:**
  - [`src/modules/projects/environment_variables/dto/response.rs:22-26`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/projects/environment_variables/dto/response.rs#L22-L26):
    ```rust
    let value = if is_secret {
        "••••••••".to_string()
    } else {
        model.value_encrypted
    };
    ```
- **Severity:** **High**
- **What "Done" Looks Like:** Decrypt the value using the encryption service when returning non-secret variables, and properly mask with `"••••••••"` only when `is_secret == true`.

---

#### GAP-P07: Duplicate RabbitMQ Connection Initialization
- **Why It Matters:** Redundant TCP connections to message brokers exhaust server and broker file descriptors. Furthermore, `AppState::new()` lacks connection retry logic; if RabbitMQ takes 2 seconds to initialize, the web server crashes immediately on startup.
- **Evidence:**
  - [`src/main.rs:25`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/main.rs#L25): `let app_state = AppState::new().await?;` connects to RabbitMQ via [`src/app/state.rs:25`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/app/state.rs#L25).
  - [`src/main.rs:29`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/main.rs#L29): `match RabbitMq::connect(&rmq_config).await` connects to RabbitMQ a second time.
- **Severity:** **High**
- **What "Done" Looks Like:** Consolidate RabbitMQ connection instantiation to a single shared `Arc<RabbitMq>` instance in `main.rs`, verify topology, and inject it into `AppState::from_parts`.

---

#### GAP-P08: Hardcoded Loki URL in Tracing Subscriber
- **Why It Matters:** In containerized deployments (Docker Compose / Kubernetes / ECS), Loki runs on a service hostname like `http://loki:3100`. Because the URL is hardcoded to `http://127.0.0.1:3100`, log shipping silently fails or crashes in production environments.
- **Evidence:**
  - [`src/shared/logger.rs:39`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/shared/logger.rs#L39):
    ```rust
    .build_url(Url::parse("http://127.0.0.1:3100").unwrap())?;
    ```
- **Severity:** **Medium**
- **What "Done" Looks Like:** Refactor `init_tracing` to accept `loki_url: Option<&str>` parsed from `AppConfig` or `std::env::var("LOKI_URL")`.

---

#### GAP-P09: Unimplemented Email Verification & Reset Delivery
- **Why It Matters:** User onboarding and password self-service recovery are non-functional without email dispatch. Currently, reset tokens are dumped to server standard output (`println!`) and verification reuses reset tokens.
- **Evidence:**
  - [`src/modules/auth/service.rs:201`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/auth/service.rs#L201): `// todo send email to the user`
  - [`src/modules/auth/service.rs:245`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/auth/service.rs#L245): `// todo: implement this`
- **Severity:** **Medium**
- **What "Done" Looks Like:** Integrate an email provider driver (e.g. `lettre` with SMTP or transactional email API like SendGrid/Resend) with email verification and password reset HTML/text templates.

---

#### GAP-P10: Orphaned Redis Infrastructure (ADR-003 Omission)
- **Why It Matters:** ADR-003 explicitly documents Redis for query caching, token revocation on logout, and distributed rate limiting. Running Redis in Docker Compose while having zero application code connected to it wastes 50 MB+ memory and misrepresents architectural completion.
- **Evidence:**
  - `Cargo.toml` contains no `redis` or `deadpool-redis` dependency.
  - [`src/modules/auth/service.rs:71, 108`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/modules/auth/service.rs#L71) leave TODO comments for Redis token blacklist.
- **Severity:** **Medium**
- **What "Done" Looks Like:** Either wire `deadpool-redis` to store revoked access token IDs on `/auth/logout`, or formally update ADR-003 to document the deferral of Redis in favor of in-memory caching.

---

#### GAP-P11: Zero API Rate Limiting Middleware
- **Why It Matters:** Public endpoints such as `/api/v1/auth/login` and `/api/v1/auth/register` are completely unprotected against brute-force attacks and volumetric DDoS.
- **Evidence:**
  - No rate limiting crate (`tower_governor` or `governor`) is imported in `Cargo.toml`.
  - [`src/app/app.rs:32-38`](file:///Users/mislamdev/Desktop/projects/personal/rust/forge/src/app/app.rs#L32-L38) applies CORS, Timeout, and Logging, but no rate limiting layer.
- **Severity:** **Medium**
- **What "Done" Looks Like:** Add `tower_governor` middleware to `/api/v1/auth/*` restricting attempts to 5 requests per minute per IP.

---

#### GAP-P12: Absence of CI/CD Automation Workflows
- **Why It Matters:** Without continuous integration, regressions like `8d7a8e7` are committed undetected to the `main` branch.
- **Evidence:**
  - Command `ls -la .github` returns `No such file or directory`.
- **Severity:** **Medium**
- **What "Done" Looks Like:** Create `.github/workflows/ci.yml` running `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, and building the Docker container on pull requests.

---

#### GAP-P13: 138 Clippy Warnings & Strict CI Failure
- **Why It Matters:** Running the repository's documented CI recipe (`just ci`) invokes `cargo clippy --all-targets --all-features -- -D warnings` and fails immediately due to 138 lints.
- **Evidence:** Command output from `cargo clippy` identified 138 compiler lints across `src/lib.rs`.
- **Severity:** **Low**
- **What "Done" Looks Like:** Run `cargo clippy --fix` and resolve remaining lints (unit bindings, redundant borrows, manual div_ceil) so `cargo clippy -- -D warnings` exits 0.

---

#### GAP-P14: Code Formatting Inconsistencies
- **Why It Matters:** Code style discrepancies trigger CI pipeline failures and cause messy git diffs across PRs.
- **Evidence:** `cargo fmt --check` failed across 6 files (`users/seeder.rs`, `users/service.rs`, `tests/health_tests.rs`, etc.).
- **Severity:** **Low**
- **What "Done" Looks Like:** Run `cargo fmt` to apply standard rustfmt rules across the workspace.

---

## Part B: Resume & Portfolio Gaps

When evaluated by a technical hiring manager or senior engineering recruiter, Forge has immense architectural merit (Rust, Tokio, Docker engine bindings, message queues, SeaORM), but suffers from missing portfolio packaging.

### 1. Root README Quality Evaluation
- **Current Status: MISSING (0 / 10).**
  - There is literally **no `README.md` at the project root**.
  - A recruiter visiting GitHub sees only a raw file tree.
- **Required Improvements:**
  - **Hero Section:** Clean title, one-line value proposition ("Self-Hosted Rust Developer Platform with Automated Container Builds, Real-Time Log Streaming, and 5-Tier RBAC").
  - **Badges:** Rust 2024, License (MIT/Apache), CI build status, Docker, OpenAPI 3.0, Newman 93/93 passing.
  - **Visual Architecture Diagram:** High-level ASCII or Mermaid flowchart showing client requests, Axum API, PostgreSQL, RabbitMQ, Docker Daemon (Bollard), and Grafana Loki.
  - **Quickstart in 60 Seconds:** Two-command setup (`docker compose up -d` + `cargo run` or `just docker-dev`).
  - **Key Technical Decisions & Trade-Offs:** Explaining why Rust was chosen over Go/Node.js, why Bollard was selected over Docker CLI subprocesses, and why RabbitMQ was chosen for build queueing.

### 2. Live Demo & Local Developer Experience
- **Current Status: PARTIAL (5 / 10).**
  - Running `just docker-dev` or `docker compose up -d` starts PostgreSQL, Redis, RabbitMQ, Loki, and Grafana.
  - However, no seeded demo tenant or sample project is automatically pre-configured. A reviewer cannot immediately click "Deploy" to watch a build run without manually executing `cargo run --bin seed` and authenticating via Postman.
- **Required Improvements:**
  - Add a one-command demo script: `just demo` that spins up the stack, executes seeds, creates a demo project pointing to a public GitHub repo, triggers a build, and outputs the live logs URL.

### 3. Measurable Results & Engineering Metrics
- **Current Status: WEAK (3 / 10).**
  - Resume bullet points cannot be credible without verified benchmarks.
  - While k6 scripts exist in `tests/load/`, no benchmark data, throughput statistics, or latency percentiles are documented in the repository.
- **Empirically Verified Numbers to Feature:**
  - **Throughput:** 265+ requests/sec handled by a single core with **0.00% error rate** on smoke tests.
  - **Latency:** **3.59ms average latency**, **1.42ms median**, **13.02ms p(95)** across 2,679 requests.
  - **Test Coverage:** **368+ automated test cases** (275 Rust unit tests + 93 Newman API tests).
  - **API Contract:** Validated 83+ endpoints conforming to OpenAPI 3.0.3 specification with zero lint errors.

### 4. Proof of Engineering Depth
- **Current Status: GOOD (7 / 10), but hidden in `docs/system/`.**
  - Outstanding ADRs exist in `docs/system/09-adr/` (ADR-001 through ADR-005) justifying database engine, ORM, message broker, and logging choices.
  - However, they are hidden in deep subfolders. Recruiters will never click into `docs/system/09-adr/` unless surfaced in the root README.
- **Required Improvements:**
  - Feature a "Deep Dives & Architecture Decisions" section in the root README linking directly to ADRs.
  - Highlight the Docker sandbox isolation mechanism: how `Bollard` streams tarballs into container contexts and isolates builds from host memory.

### 5. Missing Portfolio Artifacts Checklist

| Artifact | Current Status | Required Action |
| :--- | :---: | :--- |
| **Root README.md** | ❌ Missing | Author high-impact technical portfolio README. |
| **LICENSE** | ❌ Missing | Add `MIT` or `Apache 2.0` license file. |
| **.env.example** | ❌ Missing | Generate documented `.env.example` template. |
| **Architecture Diagram** | ❌ Missing in Root | Add Mermaid or SVG diagram in README. |
| **CI GitHub Actions Badge** | ❌ Missing | Create `.github/workflows/ci.yml` and embed badge. |
| **Interactive API Documentation** | ⚠️ Partially Mounted | Verify `/swagger-ui/` loads seamlessly in development. |
| **Git Release Tag / Changelog** | ❌ Missing | Create `v0.1.0` release tag and `CHANGELOG.md`. |
