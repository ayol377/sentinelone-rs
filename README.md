# sentinelone-rs

Async Rust SDK for SentinelOne. Workspace of four crates:

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

High-level: `Agent` (fetch/disconnect/refresh); xdr `power_query`. Remaining: more active-record entities, xdr LRQ + extra query endpoints, secret storage (keychain), `sentinelone-mcp` (see its `DESIGN.md`).

```bash
cargo test          # no-network unit tests (builder + null-client behavior)
cargo run -p sentinelone-rs --example quickstart -- <AGENT_UUID>
```

> Built clean-room from the published API specs. Not affiliated with or endorsed by SentinelOne.
