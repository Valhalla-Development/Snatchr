# Build stage
FROM rust:1.88-slim AS builder

# Install build dependencies and cargo-chef for better caching
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-chef

WORKDIR /app

# Step 1: Copy dependency files and create minimal project structure
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Step 2: Generate dependency recipe
RUN cargo chef prepare --recipe-path recipe.json

# Step 3: Build dependencies (this layer cached unless dependencies change)
RUN cargo chef cook --release --recipe-path recipe.json

# Step 4: Build the actual application
COPY src/ ./src/
RUN cargo build --release --bin snatchr

# Runtime stage - debian-slim with required libraries
FROM debian:bookworm-slim

# Install runtime dependencies for yt-dlp and the application
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
COPY --from=builder /app/target/release/snatchr .

# Set the host to 0.0.0.0
ENV HOST=0.0.0.0

# Expose the port that the application will listen on
EXPOSE ${PORT:-3000}

# Run the application
CMD ["./snatchr"]

# Healthcheck to verify the app is running
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:${PORT:-3000}/health || exit 1
