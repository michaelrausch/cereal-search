# Build stage
FROM rust:alpine3.21 AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

WORKDIR /usr/src/app
COPY . .

# Build the application with release optimizations
RUN cargo build --release

# Runtime stage
FROM alpine:3.19

# Install necessary runtime dependencies
RUN apk add --no-cache ca-certificates curl

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/bang_search /app/bang_search
# Copy the config file
COPY config.yml /app/config.yml

# Expose the port the app runs on
EXPOSE 9876

# Set the config path environment variable
ENV CONFIG_PATH=/app/config.yml

# Add health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:9876/health || exit 1

# Run the binary
CMD ["/app/bang_search"] 