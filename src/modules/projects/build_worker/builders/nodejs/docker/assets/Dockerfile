# Stage 0: Base Stage
FROM node:22-alpine AS base
WORKDIR /app

# Stage 1: Dependencies Stage
FROM base AS dependencies
COPY package.json {lock_file} {ts_config} ./
RUN {pkg_installer}


# Stage 2: Building Stage
FROM base AS builder
COPY package.json {lock_file} {ts_config} ./
COPY --from=dependencies /app/node_modules ./node_modules
COPY . .
RUN {pkg_builder}
RUN {pkg_prune}

# Stage 3: Production Runner Stage
FROM node:22-alpine AS runner
WORKDIR /app

ENV NODE_ENV=production
ENV PORT={exposed_port}

# Run as non-root user for security
USER node

# Copy only production dependencies and built application
COPY --chown=node:node {lock_file} ./
COPY --from=builder --chown=node:node /app/node_modules ./node_modules
COPY --from=builder --chown=node:node /app/dist ./dist

EXPOSE {exposed_port}

# Run compiled JS directly so SIGTERM/SIGINT signals reach the app for graceful shutdown
CMD {pkg_start}
