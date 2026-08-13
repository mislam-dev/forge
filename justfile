# ================================
# Development
# ================================

start:
    cargo run

dev:
    cargo watch -x run

test:
    cargo test

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
    sea-orm-cli migrate up --migration-dir src/database/migrations

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

# Create a new migration
migration name:
    @if [ -z "{{ name }}" ]; then echo "Error: migration name is required"; exit 1; fi
    sea-orm-cli migrate generate "{{ name }}" --migration-dir src/database/migrations
