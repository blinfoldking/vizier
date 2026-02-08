# Use the official Rust image for building
FROM rust:1.86-bookworm AS builder

# Create a new empty shell project
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs and lib.rs to satisfy cargo
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs

# Build dependencies only (this layer will be cached)
RUN cargo build --release

# Now copy the actual source code
COPY src ./src
COPY templates ./templates

# Rebuild with actual source code
RUN touch src/main.rs && cargo build --release

# Use a smaller runtime image
FROM debian:bookworm-slim

# Install runtime dependencies if needed
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/vizier /usr/local/bin/vizier

# Copy templates directory
COPY --from=builder /app/templates /app/templates

# Create a non-root user to run the application
RUN useradd -m -u 1000 appuser
USER appuser

# Set the working directory
WORKDIR /home/appuser

# Expose any necessary ports (update based on your application's needs)
# EXPOSE 8080

# Run the application
CMD ["vizier"]