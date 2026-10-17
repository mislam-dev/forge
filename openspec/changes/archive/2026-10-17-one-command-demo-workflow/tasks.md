# Tasks

## 1. Automation Script Implementation

- [x] 1.1 Create `scripts/demo.sh` executable script with pre-flight checks for `docker`, `cargo`, and `curl`, and verify executable permissions with `chmod +x scripts/demo.sh`
- [x] 1.2 Implement backing infrastructure startup (`docker compose up -d postgres redis rabbitmq loki`) and health readiness polling loop for PostgreSQL and RabbitMQ in `scripts/demo.sh`
- [x] 1.3 Implement automated database migration execution (`just db-up`) and baseline account seeding (`just seed`) in `scripts/demo.sh`
- [x] 1.4 Implement API server process management and health polling targeting `http://localhost:9000/health/ready` with a 30-second timeout in `scripts/demo.sh`
- [x] 1.5 Implement authenticated API demonstration flow (login as `admin1@forge.local`, acquire JWT token, create demo project, trigger sample deployment) with JSON response parsing in `scripts/demo.sh`
- [x] 1.6 Implement cross-platform browser opener for `http://localhost:9000/docs` with headless/fallback URL printing in `scripts/demo.sh`

## 2. Justfile Integration

- [x] 2.1 Add the `demo` recipe to `justfile` delegating to `bash scripts/demo.sh` and verify with `just --list`

## 3. End-to-End Verification & Task Tracking

- [x] 3.1 Execute `just demo` on local environment and verify all milestones (containers, migrations, seeding, API health, curl deployment dispatch, Swagger UI launch) complete successfully under 60 seconds
- [x] 3.2 Update `docs/03-TASKS.md` marking task T-019 as COMPLETED with execution details and verification notes
