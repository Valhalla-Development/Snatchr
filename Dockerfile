# Use the official Rust image as a base
FROM rust:1.88

# Set the working directory to /app
WORKDIR /app

# Copy the Cargo files first for better caching
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src/ ./src/

# Build the Rust application
RUN cargo build --release

# Set the host to 0.0.0.0
ENV HOST=0.0.0.0

# Expose the port that the application will listen on (configurable via PORT env var)
EXPOSE ${PORT:-3000}

# Run the command to start the application when the container is launched
CMD ["./target/release/snatchr"]

HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:${PORT:-3000}/health || exit 1
