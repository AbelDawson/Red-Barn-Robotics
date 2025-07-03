# --- Build stage ---
FROM rust:latest AS builder

# Create app directory
WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs
RUN cargo build --release && rm -rf src

# Copy actual source
COPY . .

# Build actual binary
RUN cargo build --release

# --- Runtime stage ---
FROM debian:bookworm-slim

# Install required runtime packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/crop_tracking_system /usr/local/bin/crop_tracking_system

# Set working directory
WORKDIR /data

# Define entrypoint (❗fixed syntax)
ENTRYPOINT ["crop_tracking_system"]

