# Multi-stage production build using cargo-chef for dependency caching

FROM rust:1-slim-bookworm AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    cmake \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Stage 1: Generate dependency recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Cook dependencies and build release binary
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies layer (cached unless Cargo.lock/Cargo.toml change)
RUN cargo chef cook --release --recipe-path recipe.json
# Copy source and compile application
COPY . .
RUN cargo build --release --bin forge && \
    strip /app/target/release/forge

# Stage 3: Minimal hardened runtime image
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create unprivileged application user and writable log directory
RUN groupadd -g 10001 appgroup && \
    useradd -u 10001 -g appgroup -s /bin/false appuser && \
    mkdir -p /app/logs && \
    chown -R appuser:appgroup /app

# Copy compiled and stripped binary
COPY --from=builder --chown=appuser:appgroup /app/target/release/forge /usr/local/bin/forge

USER appuser

ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=9000
ENV RUST_LOG=info

EXPOSE 9000

HEALTHCHECK --interval=15s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:9000/health/live || exit 1

ENTRYPOINT ["/usr/local/bin/forge"]
