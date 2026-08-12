# syntax=docker/dockerfile:1.7
#
# Build + runtime on Debian Trixie so glibc matches:
# - the Rust toolchain default (rust:*-slim)
# - ffmpeg/yt-dlp binaries auto-downloaded into libs/
#
# cargo-chef stages keep dependency rebuilds cheap; BuildKit cache mounts
# speed up registry/git/target reuse on CI.

# --- chef: toolchain + build deps (cached unless this stage changes) ---
FROM rust:1.93-slim AS chef

WORKDIR /app

# git is required for Cargo git dependencies (yt-dlp fork)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-chef --locked

# --- planner: compute dependency recipe ---
FROM chef AS planner

# Snatchr is a library crate with a binary — both stubs are required
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && echo "" > src/lib.rs
RUN cargo chef prepare --recipe-path recipe.json

# --- builder: cook deps, then compile the real app ---
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo chef cook --release --recipe-path recipe.json

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
# Cache mount for target/ — copy the binary out so the next stage can use it
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/app/target \
    cargo build --release --bin snatchr \
    && cp /app/target/release/snatchr /app/snatchr

# --- runtime: same distro/glibc as the builder (Trixie) ---
FROM debian:trixie-slim

# Runtime deps for yt-dlp / networking / healthchecks
RUN apt-get update && apt-get install -y \
    ca-certificates \
    zlib1g \
    python3 \
    python3-pip \
    wget \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy only the binary from builder stage
COPY --from=builder /app/snatchr .

# Set the host to 0.0.0.0
ENV HOST=0.0.0.0

# Expose the port that the application will listen on
EXPOSE ${PORT:-3000}

# Run the application
CMD ["./snatchr"]

# Healthcheck to verify the app is running
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:${PORT:-3000}/health || exit 1
