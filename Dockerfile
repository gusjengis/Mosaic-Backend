# --- Stage 1: Build the application ---
FROM rust:latest AS builder

# Set the working directory inside the container.
WORKDIR /app

# Copy the Cargo manifest files to leverage Docker’s caching.
COPY Cargo.toml Cargo.lock ./

# Create a dummy main file so that dependencies can be cached.
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies in release mode (this step is cached until Cargo.toml or Cargo.lock change).
RUN cargo build --release

# Remove the dummy main file.
RUN rm -f src/main.rs

# Copy the full source code.
COPY . .

# Build the actual application in release mode.
RUN cargo build --release

# --- Stage 2: Create a minimal runtime image ---
# Using a newer Debian image (Bookworm) with an updated glibc.
FROM debian:bookworm-slim

# (Optional) Install CA certificates if your API requires HTTPS.
# RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage.
COPY --from=builder /app/target/release/mosaic-backend /usr/local/bin/mosaic-backend

# Expose the port your application listens on (adjust as necessary).
EXPOSE 8080

# Run the application.
CMD ["mosaic-backend"]
