#!/bin/sh
# Build a fully static x86_64 musl binary of sentinelone-mcp using the
# rust:1-alpine image (no musl toolchain needed on the host). Needs ~3 GB RAM.
# Output: target/musl/release/sentinelone-mcp — copy it to any Linux box.
set -eu
cd "$(dirname "$0")/.."
docker run --rm -v "$PWD":/build:z -w /build \
  -v s1-cargo-registry:/usr/local/cargo/registry rust:1-alpine \
  sh -c 'apk add -q musl-dev && cargo build --release -p sentinelone-mcp --target-dir target/musl'
ls -la target/musl/release/sentinelone-mcp
