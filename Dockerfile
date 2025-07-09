# Multi-stage build for optimal image size
FROM rust:1.70 as builder

WORKDIR /app
COPY . .

# Install wasm target
RUN rustup target add wasm32-unknown-unknown

# Build the project
RUN cargo build --release --target wasm32-unknown-unknown

# Runtime image
FROM debian:bullseye-slim

# Install required packages
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Substreams CLI
RUN curl -s https://api.github.com/repos/streamingfast/substreams/releases/latest | \
    grep "browser_download_url.*linux_x86_64.tar.gz" | \
    cut -d : -f 2,3 | \
    tr -d \" | \
    xargs curl -L | \
    tar -xzf - -C /tmp && \
    mv /tmp/substreams /usr/local/bin/

# Copy built artifacts
COPY --from=builder /app/target/wasm32-unknown-unknown/release/usdy_solana_tracker.wasm /app/
COPY --from=builder /app/substreams.yaml /app/
COPY --from=builder /app/proto/ /app/proto/

WORKDIR /app

# Default command
CMD ["substreams", "run", "substreams.yaml", "map_usdy_events", "-e", "mainnet.sol.streamingfast.io:443"]
