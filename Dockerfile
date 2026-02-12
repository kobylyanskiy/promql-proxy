# --- Stage 1: Build ---
FROM rust:1.93-slim-bookworm AS builder

# Create a dummy project to cache dependencies
WORKDIR /app
COPY Cargo.toml ./
# Create a fake main.rs so cargo can fetch/build dependencies alone
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Now copy your real source code and the index.html
COPY src ./src
# COPY index.html ./index.html 

# Touch main.rs to ensure cargo rebuilds it, then compile the real binary
RUN touch src/main.rs
RUN cargo build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim

# Install CA certificates for HTTPS requests (needed for reqwest)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/promql-proxy /app/promql-proxy

# Copy your settings file (or provide it via Volume)
COPY settings.toml /app/settings.toml

# Expose the port from your ServerConfig
EXPOSE 8081

# Run the proxy
CMD ["./promql-proxy"]
