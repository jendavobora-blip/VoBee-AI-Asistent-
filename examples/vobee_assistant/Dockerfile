# Dockerfile for VoBee API Server
# This allows easy deployment of the VoBee API to any container platform

# Build stage
FROM rust:1.91-bookworm as builder

# Copy the entire workspace (build context should be the repository root)
WORKDIR /workspace
COPY . /workspace

# Build the API binary
RUN cargo build --release --bin vobee_api --manifest-path examples/vobee_assistant/Cargo.toml

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /workspace/target/release/vobee_api /usr/local/bin/vobee_api

# Expose the default port
EXPOSE 3000

# Set environment variables
ENV PORT=3000

# Run the API server
CMD ["vobee_api"]
