# Forge Platform Load Testing Guide

This guide describes the automated multi-tier load testing framework for the Forge platform using [Grafana k6](https://k6.io/). The framework benchmarks system throughput, validates latency budgets, ensures database connection pool resilience, and analyzes graceful degradation across 1,000, 10,000, and 100,000 concurrent user scales.

---

## 1. Concurrency Tiers & Requirements

| Tier | Concurrency (VUs) | Target Purpose | Threshold: p95 Latency | Threshold: Error Rate | Requirement Mapping |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Tier 1** | **1,000** | Baseline operating throughput & smoke test | `< 50 ms` | `< 0.1 %` | Baseline Health |
| **Tier 2** | **10,000** | Target concurrent load compliance | `< 200 ms` | `< 1.0 %` | **SRS Section 6 & NFR-001** |
| **Tier 3** | **100,000** | Extreme stress, saturation limit, backpressure | `< 1,000 ms` | `< 20.0 %` (allows 429/503) | Resilience & Recovery |

---

## 2. Directory Structure

```
tests/load/
├── config.js              # Global base URLs, default test credentials, and tier thresholds
├── helpers/
│   ├── auth.js            # JWT login, token prewarming, and auth headers
│   └── metrics.js         # Custom latency trends (health, auth, read, mutation) & counters
├── scenarios/
│   ├── health.js          # Unauthenticated health probes (/health, /health/live, /health/ready)
│   ├── auth.js            # Authentication stress testing (POST /api/v1/auth/login)
│   ├── reads.js           # Authenticated cached reads (/organizations, /projects, /teams, etc.)
│   └── full_journey.js    # Mixed read and mutation user lifecycle flow
└── tiers/
    ├── tier_1k.js         # Tier 1 runner: 1,000 VUs staged ramp-up
    ├── tier_10k.js        # Tier 2 runner: 10,000 VUs target compliance
    └── tier_100k.js       # Tier 3 runner: 100,000 VUs extreme stress
```

---

## 3. Pre-requisites & OS Tuning

### Native k6 or Docker
- **macOS (Homebrew):** `brew install k6`
- **Linux:** `sudo apt-get install k6` or equivalent package manager
- **Docker:** Available automatically via `just load-test-docker` using `grafana/k6:latest`

### Operating System File Descriptor & Socket Limits
Simulating 10,000 to 100,000 concurrent network connections requires tuning local operating system limits to avoid `socket: too many open files` or ephemeral port exhaustion.

#### macOS:
```bash
# Increase file descriptor limit for the current shell session:
ulimit -n 65536

# Expand ephemeral port range (optional, for Tier 2 and Tier 3):
sudo sysctl -w net.inet.ip.portrange.first=1024
```

#### Linux:
```bash
# Increase open file limits:
ulimit -n 1048576

# Enable fast socket recycling:
sudo sysctl -w net.ipv4.tcp_tw_reuse=1
sudo sysctl -w net.ipv4.ip_local_port_range="1024 65535"
```

---

## 4. Execution Commands (`justfile`)

Start the Forge backend server first (e.g. `just start` or `cargo run`), then run any of the following targets:

### Smoke Check (10 VUs for 10s)
```bash
just load-test-smoke
```

### Tier 1: 1,000 Concurrent Virtual Users
```bash
just load-test-1k
```

### Tier 2: 10,000 Concurrent Virtual Users (SRS NFR-001 Compliance)
```bash
just load-test-10k
```

### Tier 3: 100,000 Concurrent Virtual Users (Extreme Stress & Degradation)
```bash
just load-test-100k
```

### Running with Docker
```bash
# Default runs tier_1k.js
just load-test-docker

# Run specific tier inside Docker:
just load-test-docker tests/load/tiers/tier_10k.js
```

---

## 5. Environment Overrides

You can customize targets and credentials using environment variables:

| Variable | Default Value | Description |
| :--- | :--- | :--- |
| `BASE_URL` | `http://localhost:9000` | Target host base URL |
| `TEST_EMAIL` | `admin1@forge.local` | Test user login email |
| `TEST_PASSWORD` | `Password123!` | Test user login password |
| `ORG_ID` | `00000000-0000-0000-0000-000000000001` | Active Organization UUID |

Example:
```bash
BASE_URL="http://192.168.1.100:9000" just load-test-1k
```

---

## 6. Output & Reporting

Each tier run outputs real-time console metrics and writes a structured JSON summary report upon completion:
- **Tier 1:** `tests/load/summary-tier-1k.json`
- **Tier 2:** `tests/load/summary-tier-10k.json`
- **Tier 3:** `tests/load/summary-tier-100k.json`

The reports include:
- Total requests and Requests Per Second (RPS)
- Percentile latencies: p50, p90, p95, p99
- Threshold pass/fail status
- Breakdown of HTTP status codes (200, 429 rate-limited, 503 service unavailable)
