# Multi-stage Dockerfile for production-ready Rust + SPFresh service
# Build stage: compile the Rust binary (assumes Rust 1.91 toolchain compatibility)
FROM rust:1.91-bullseye as builder

WORKDIR /usr/src/app

# Install system build deps for C/C++ parts (SPFresh wrapper) and pkg tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential cmake pkg-config git ca-certificates curl \
    libgomp1 libtbb-dev libnuma-dev zlib1g-dev pkg-config ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy Cargo manifests and download dependencies (layer caching)
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "// placeholder" > src/lib.rs
RUN cargo fetch --locked

# Copy the full source and (optionally) SPFresh Release folder
COPY . .

# Build in release mode
RUN cargo build --release -p vector-search-api || cargo build --release


### Runtime image
FROM debian:bookworm-slim

ENV RUST_BACKTRACE=1
ENV RUST_LOG=info

# Create a non-root user
RUN useradd -m -u 1000 appuser

# Install runtime libs required by SPFresh and the binary
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl libstdc++6 libgomp1 libtbb2 libnuma1 libzstd1 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/vector-search-api /usr/local/bin/vector-search-api

# Copy configuration (if present) and companion files
COPY --from=builder /usr/src/app/config.json ./config.json

# Copy SPFresh Release native libs if present in the repo (optional)
COPY --from=builder /usr/src/app/SPFresh/SPFresh/Release /usr/local/lib/spfresh-release

# Ensure dynamic loader can see the SPFresh libs (we'll still export LD_LIBRARY_PATH)
ENV LD_LIBRARY_PATH=/usr/local/lib/spfresh-release:$LD_LIBRARY_PATH

# Create directories for data and logs and set permissions
RUN mkdir -p /app/data && chown -R appuser:appuser /app

USER appuser

EXPOSE 3000

# Healthcheck — uses curl to hit the local /health endpoint
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -fsS http://127.0.0.1:3000/health || exit 1

# Run the server; allow overriding via env (e.g., VECTOR_CONFIG_PATH)
CMD ["/usr/local/bin/vector-search-api"]
