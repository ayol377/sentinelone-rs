# sentinelone-rs

Async Rust SDK for SentinelOne, plus a multi-tenant MCP server built on it. One
Cargo workspace in two halves:

- `crates/` — the SDK: four library crates (table below).
- `mcp/` — `sentinelone-mcp`, a standalone server application over the SDK. It
  owns its own `Dockerfile`, `docker-compose.yml` and `.env.example`; see
  [`mcp/README.md`](mcp/README.md). Nothing MCP-specific lives at the repo root
  except `.mcp.json` (Claude Code project config).

SDK crates:

| Crate | Role | Auth | Source |
|-------|------|------|--------|
| `sentinelone-http` | shared transport (reqwest, auth, errors) | — | hand-written |
| `sentinelone-api-rs` (`use sentinelone_api`) | low-level **Management** API | `ApiToken` | hand-written, 1:1 with `swagger_2_1.json` |
| `sentinelone-xdr-api-rs` (`use sentinelone_xdr_api`) | low-level **XDR / Data Lake** API | `Bearer` | hand-written (DataSet HTTP API) |
| `sentinelone-rs` (`use sentinelone`) | high-level unified facade, active-record entities | — | hand-written |

Hosts: Management `https://<region>-<tenant>.sentinelone.net`; XDR `https://xdr.<region>.sentinelone.net`.

```rust
let s1 = SentinelOne::builder()
    .management(mgmt_host, mgmt_token)   // optional
    .xdr(xdr_host, bearer)               // optional
    .build()?;

let agent = s1.agent("UUID").await?;
agent.disconnect().await?;

let rows = s1.power_query("dataset='endpoint' | limit 100").from("24 hours").to("now").run().await?;
```

Calling an unconfigured backend returns `Error::ManagementNotConfigured` / `Error::XdrNotConfigured` — never panics.

## Status

**Management API: complete.** All 117 service tags / **826 endpoints** hand-written at 1:1 spec parity — path ids required, optional params `Option` (omitted when unset), typed `Response<T>`/`Paginated<T>` envelopes, full rustdoc on every method. `cargo build --workspace` is green.

**XDR / Data Lake API: complete.** 15 methods — query / powerQuery / facet / numeric / timeseries (each with a `*_with` full-request form), addEvents, uploadLogs, and Long Running Query launch / poll / cancel.

**MCP server: v0.1** in `mcp/` — 42 tools, multi-tenant fan-out, gated destructive actions, stdio + HTTP transports.

High-level facade: `Agent` (fetch/disconnect/refresh); xdr `power_query`. Remaining: more active-record entities, secret storage (keychain) for a future interactive CLI.

```bash
cargo test          # no-network unit tests (builder + null-client behavior)
cargo run -p sentinelone-rs --example quickstart -- <AGENT_UUID>
cd mcp && docker compose up --build     # MCP server on :8080
```

> Built clean-room from the published API specs. Not affiliated with or endorsed by SentinelOne.
