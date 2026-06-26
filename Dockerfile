# Builds the sentinelone-mcp server. Context = repo root (workspace).
#
#   docker build -t sentinelone-mcp .

FROM rust:1-bookworm AS builder
WORKDIR /build
COPY . .
RUN cargo build --release -p sentinelone-mcp

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/target/release/sentinelone-mcp /usr/local/bin/sentinelone-mcp

# Default to the HTTP transport for a long-lived containerized service.
ENV MCP_TRANSPORT=http \
    MCP_BIND=0.0.0.0:8080
EXPOSE 8080

# Run unprivileged.
USER 1000:1000

ENTRYPOINT ["/usr/local/bin/sentinelone-mcp"]
