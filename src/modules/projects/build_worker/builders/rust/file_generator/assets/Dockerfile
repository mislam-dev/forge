# Build stage
FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
RUN cargo install --path . --root /app/dist

# Production stage
FROM alpine:latest
WORKDIR /app
RUN apk --no-cache add ca-certificates
COPY --from=builder /app/dist/bin/ /app/bin/

ENV PORT=8080
EXPOSE 8080
CMD ["sh", "-c", "exec /app/bin/*"]
