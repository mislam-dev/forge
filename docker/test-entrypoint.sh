#!/usr/bin/env bash
set -e

echo "=== Forge Containerized Test Environment ==="
echo "Current File Descriptor Limit (nofile): $(ulimit -n)"

# Ensure limits meet or exceed requirement (65535)
CURRENT_LIMIT=$(ulimit -n)
if [ "$CURRENT_LIMIT" != "unlimited" ] && [ "$CURRENT_LIMIT" -lt 65535 ]; then
    echo "Attempting to increase ulimit -n to 65535..."
    ulimit -n 65535 || echo "Warning: Could not raise ulimit -n beyond $CURRENT_LIMIT"
fi

echo "Effective nofile limit: $(ulimit -n)"
echo "Target Database: ${DATABASE_URL:-not set}"
echo "Target Redis: ${REDIS_URL:-not set}"
echo "Target RabbitMQ: ${RABBITMQ_URL:-not set}"
echo "==========================================="

if [ "$#" -gt 0 ]; then
    if [ "$1" = "ulimit" ]; then
        ulimit "${@:2}"
    else
        exec "$@"
    fi
else
    echo "Running default test suite (Cargo Test)..."
    cargo test -- --nocapture
fi
