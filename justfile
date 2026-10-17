set dotenv-load

# ================================
# Demonstration & Evaluation
# ================================

# Run complete one-command live demonstration workflow (<60s)
demo:
    bash scripts/demo.sh

# ================================
# Development
# ================================

start:
    cargo run

dev:
    cargo watch -c -w src -d 0.1 -x run

test:
    cargo test
clippy:
    cargo clippy --all-targets --all-features -- -D warnings

check:
    cargo check

fmt:
    cargo fmt

lint:
    cargo clippy --all-targets --all-features
ci:
    cargo fmt --check
    cargo check
    cargo clippy --all-targets --all-features -- -D warnings
    cargo test

# ================================
# API Testing & Postman Automation
# ================================

# Bundle Postman v12 Local Workspace collection into Collection v2.1 JSON
bundle-collection:
    node scripts/bundle-collection.js

# Run API tests headlessly with Newman against local environment
test-api: bundle-collection
    npx newman run postman/forge-platform-api.collection.json -e environment.json

# Run API tests headlessly with Newman against a specific environment (e.g., just test-api-env admin-with-org)
test-api-env env: bundle-collection
    npx newman run postman/forge-platform-api.collection.json -e "postman/environments/{{env}}.environment.json"

# Run multi-role RBAC matrix tests across all 6 role & tenancy environments
test-api-matrix: bundle-collection
    node scripts/run-role-matrix.js

# Generate visual interactive HTML test reports with newman-reporter-htmlextra
test-api-report: bundle-collection
    @mkdir -p reports/postman
    npx newman run postman/forge-platform-api.collection.json -e environment.json -r cli,htmlextra --reporter-htmlextra-export reports/postman/index.html --reporter-htmlextra-title "Forge Platform API Test Report"

# ================================
# Load Testing (k6 Multi-Tier)
# ================================

# Run load test smoke check (10 VUs for 10s)
load-test-smoke:
    k6 run tests/load/scenarios/health.js

# Run Tier 1 load test: 1,000 Concurrent Virtual Users
load-test-1k:
    k6 run tests/load/tiers/tier_1k.js

# Run Tier 2 load test: 10,000 Concurrent Virtual Users (SRS NFR-001)
load-test-10k:
    k6 run tests/load/tiers/tier_10k.js

# Run Tier 3 load test: 100,000 Concurrent Virtual Users (Stress & Degradation)
load-test-100k:
    k6 run --no-connection-reuse=false tests/load/tiers/tier_100k.js

# Run load test in Docker container (e.g. just load-test-docker tests/load/tiers/tier_1k.js)
load-test-docker script="tests/load/tiers/tier_1k.js":
    docker run --rm -i --network=host -v "$PWD:/work" -w /work grafana/k6:latest run {{ script }}

# ================================
# SeaORM
# ================================

# Generate entities for a specific module.
#
# Usage:
#   just entity users users
#
# Arguments:
#   module  - module name
#   tables  - comma-separated table names
#
#
#
setup:
    cargo check
    just db-up

entity path tables:
    @if [ -z "{{ path }}" ]; then echo "Error: module path is required"; exit 1; fi
    @if [ -z "{{ tables }}" ]; then echo "Error: table names are required"; exit 1; fi
    sea-orm-cli generate entity \
        -u "$DATABASE_URL" \
        -o "src/modules/{{ path }}/entities" \
        --tables "{{ tables }}" \
        --with-serde both \
        --date-time-crate chrono





# ================================
# Database Seed
# ================================
seed: 
    cargo run --bin seed

# ================================
# Database Migrations
# ================================

# Run all pending migrations
db-up:
    sea-orm-cli migrate up --migration-dir src/database/migrations

# Rollback the latest migration
db-down:
    sea-orm-cli migrate down --migration-dir src/database/migrations

# Rollback all migrations
db-reset:
    sea-orm-cli migrate reset --migration-dir src/database/migrations

db-fresh:
    sea-orm-cli migrate fresh -u "$DATABASE_URL" --migration-dir src/database/migrations

# Create a new migration
migration name:
    @if [ -z "{{ name }}" ]; then echo "Error: migration name is required"; exit 1; fi
    sea-orm-cli migrate generate "{{ name }}" --migration-dir src/database/migrations

# generating migrations scaffolding
migration-cli init:
    sea-orm-cli migrate init -d src/database/migrations

# ================================
# Docker & Container Orchestration
# ================================

# Start infrastructure services (Postgres, Redis, RabbitMQ, Loki, Grafana)
docker-infra-up:
    docker compose up -d postgres redis rabbitmq loki grafana

# Start hot-reloading development container with dependencies
docker-dev:
    docker compose --profile dev up --build -d 

# Run cargo test in Docker with automated limits (nofile: 65535)
docker-test:
    docker compose --profile test run --rm forge-test

# Run Postman/Newman API matrix tests in Docker container
docker-test-api:
    docker compose --profile test run --rm forge-test npm run test:matrix

# Run load test in Docker container (e.g. just docker-load-test tests/load/tiers/tier_1k.js)
docker-load-test script="tests/load/tiers/tier_1k.js":
    docker compose --profile test run --rm forge-test npx k6 run {{ script }}

# Build and start full production stack in Docker
docker-prod-up:
    docker compose --profile prod up -d --build

# Stop all containers and remove networks
docker-down:
    docker compose --profile dev --profile prod --profile test down

