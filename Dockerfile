# Stage 1: Build Rust application
FROM rust:latest AS builder

WORKDIR /app

# Copy entire project, including Cargo.toml and source files
COPY . .

# Ensure dependencies are locked
RUN cargo generate-lockfile

# Fetch dependencies ahead of time
RUN cargo fetch

# Build the Rust binary
RUN cargo build --release

# Stage 2: Create a minimal final image
FROM debian:bookworm-slim

WORKDIR /app

# Copy the compiled Rust binary from the builder stage
COPY --from=builder /app/target/release/rustflare /app/rustflare

# Copy the config.yaml from the docker directory
COPY docker/config.yaml /app/config.yaml

# Install necessary dependencies
RUN apt-get update && apt-get install -y ca-certificates openssl && rm -rf /var/lib/apt/lists/*

# Set execution command
CMD ["/app/rustflare"]
