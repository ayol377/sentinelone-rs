# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build --workspace                 # must stay green AND warning-free
cargo test                              # no-network unit tests (facade builder/null-client; MCP scope confinement + gating)
cargo test -p sentinelone-rs calling_xdr_without_config_errors_not_panics   # single test
cargo run -p sentinelone-rs --example quickstart -- <AGENT_UUID>            # needs S1_MGMT_HOST/S1_MGMT_TOKEN

# MCP server — lives in mcp/, not crates/; run these from mcp/
cd mcp && MCP_TRANSPORT=stdio cargo run              # reads mcp/.env (override: MCP_ENV_FILE=path)
cd mcp && docker compose up --build                  # http on :8080; build context is the workspace root (..)
# release build needs ~3 GB RAM for one rustc (api crate); on smaller hosts build the image elsewhere and `docker load` it
curl -s localhost:8080/health
curl -s localhost:8080/ -H 'content-type: application/json' -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}'

# Regenerate the API reference markdown from the vendor spec
python3 s1_swagger_to_md.py swagger_2_1.json -o docs
# swagger_2_1.json, docs/ and sentinel-mgmt-sdk/ are gitignored, so a fresh clone has none of them.
# The spec is a browser download from the console's login-gated /apidoc/swagger_2_1.json.
```

## Workspace layout

One workspace (edition 2024) in two halves: four SDK library crates under `crates/`, and one application, `sentinelone-mcp`, under `mcp/`. **Package name ≠ lib name** — always import by lib name:

| Package (`-p …`) | `use …` | Role |
|---|---|---|
| `sentinelone-http` | `sentinelone_http` | reqwest wrapper: base URL + `Auth::{ApiToken,Bearer}` + `HttpError` |
| `sentinelone-api-rs` | `sentinelone_api` | Management REST, service-per-tag, 117 tags / 826 endpoints |
| `sentinelone-xdr-api-rs` | `sentinelone_xdr_api` | XDR / Singularity Data Lake (DataSet HTTP API), 15 methods; each typed query method has a `*_with` twin taking the full request struct |
| `sentinelone-rs` | `sentinelone` | high-level facade + active-record entities |
| `sentinelone-mcp` (in `mcp/`) | (bin) | multi-tenant MCP server over the facade |

Dependency direction is strictly one-way: `mcp → sentinelone-rs → {api, xdr-api} → http`. The two low-level API crates never see each other; they are genuinely different APIs (different host, different auth scheme).

## Non-negotiable design rules

- **Clean-room.** Everything is hand-written from the published specs. `sentinel-mgmt-sdk/` (extracted Python SDK) is licensed "SentinelOne — Corporate IP": read it for *shape* only, never copy code. It, `swagger_2_1.json`, and generated `docs/` are gitignored build aids.
- **Async (tokio) only.** No blocking API.
- **Never panic on an unconfigured backend.** `SentinelOne` holds `Option<ManagementClient>` + `Option<XdrClient>`; all access goes through the checked accessors `s1.management()?` / `s1.xdr()?`, returning `Error::ManagementNotConfigured` / `Error::XdrNotConfigured`. Not typestate — deliberately a runtime error.
- **1:1 spec parity in `sentinelone-api-rs`.** Path ids are required args (`&str`/`i64`, never `Option`); optional/`x-nullable` params are `Option<T>` + `#[serde(skip_serializing_if = "Option::is_none")]` so unset == omitted == the API's null default; `serde_json::Value` only for genuinely free-form `object`/export payloads (say so in the rustdoc). Every method carries a `///` doc starting with verb + path + spec summary.
- **Warning-free.** `sentinelone-api-rs` has `#![allow(non_snake_case)]` crate-wide because filter fields mirror the wire names including operator suffixes (`created_at__gt`, `computer_name__contains`). Don't rename those fields to "fix" them.

## Management crate conventions

Canonical file to copy when adding a service: `crates/sentinelone-api-rs/src/services/activities.rs`.

- One module per spec tag in `src/services/`, one in `src/models/`, both registered in the respective `mod.rs`, plus an accessor on `ManagementClient` in `src/client.rs` (`client.activities()`).
- Service structs are borrowed: `pub struct XService<'a> { pub(crate) client: &'a ManagementClient }`.
- Query params: one `#[derive(Debug, Default, Serialize)] #[serde(rename_all = "camelCase")]` struct per endpoint plus chainable `fn foo(mut self, …) -> Self` builders. Array params serialize comma-joined (`join_csv`), not repeated keys. `join_csv` is a *private per-file helper* (there is no shared util module) — copy it into a new service file.
- `limit`/`skip` builders take `i64` in the generated services; the two skeleton services `accounts` and `agents` take `u32`. The facade contract below pins `agents` at `u32` — don't normalize in either direction.
- `Error::Api` is declared but nothing constructs it. Non-2xx responses arrive as `Error::Http(HttpError::Status { status, body })` with the raw body text; `Error::NotFound` is only produced by facade-level lookups.
- Serialize with `serde_urlencoded::to_string(query)`, pass `None` when empty, then `self.client.http().get(path, q)`.
- Envelopes live in `pagination.rs` and are reused everywhere: `Paginated<T>` (`{data: Vec<T>, pagination{nextCursor,totalItems}}`) for lists, `Response<T>` for single resources. Never re-declare an envelope in a service module.
- Escape hatches on `ManagementClient`: `get_json` / `post_json` return the untyped envelope for endpoints with no typed wrapper. The MCP server relies on these.
- Per-endpoint reference for hand-writing lives in `docs/<tag>.md` (generated; see command above). Verify exact paths there or in the swagger before wiring — don't guess.
- The generated services were produced by a Workflow script, `.claude/workflows/write-s1-mgmt-api.js` (local, gitignored): one agent per tag reading `docs/<tag>.md`, then a build-and-repair loop. Reuse it for any bulk rewrite instead of hand-editing 100+ files.

**Compile contract the facade depends on** (breaking these breaks `sentinelone-rs`): `agents::AgentsQuery::default().ids([..]).limit(u32)`, `agents::AgentsService::list(&AgentsQuery) -> Paginated<Agent>`, `agent_actions::AgentFilter::ids([..])`, `agent_actions::disconnect(&AgentFilter) -> i64`, `models::{Account, Agent}`, and `XdrClient::power_query(pql, start, end)`.

XDR responses are intentionally lenient: envelopes carry `#[serde(flatten)] extra` so unmodeled server fields survive. `models::Agent` does the same, which is why MCP device output keeps full server fields (IPs, etc.). Preserve that when editing models.

## High-level facade (`sentinelone-rs`)

`SentinelOne` is `Arc`-backed and cheap to clone. Active-record entities (`entities/agent.rs` is the reference pattern) bundle a raw model + a `SentinelOne` clone so `agent.disconnect()` routes itself. Note the mgmt API has no get-by-id route for agents — `Agent::fetch` filters the list endpoint by `ids` and takes the first result. Raw crates are re-exported as `sentinelone::api` / `sentinelone::xdr_api` for escape-hatch use.

## MCP server (`sentinelone-mcp`)

Read `mcp/DESIGN.md` for intent and roadmap before changing tool surface; its status note tabulates what the code superseded (`rmcp`, TOML + keychain, `confirm_token`, the filter DSL, pivots). `mcp/README.md` and `mcp/src/tools.rs` are the source of truth for the current 42-tool surface.

- **No `rmcp`/SDK dependency** — a deliberately small JSON-RPC 2.0 implementation in `mcp.rs` (methods: `initialize`, `tools/list`, `tools/call`, `ping`; notifications ignored). Protocol version `2025-06-18`. Transports: `stdio.rs` (newline-delimited JSON) and `http.rs` (axum 0.8, POST `/` or `/mcp`, `GET /health`).
- **Config is 100% env** (`config.rs`, with its own tiny `.env` loader — no dotenvy). Tenants are `S1_TENANTS=acme,globex` + `S1_TENANT_<ID>_{NAME,GROUPS,MGMT_HOST,MGMT_TOKEN,XDR_HOST,XDR_TOKEN}`, `<ID>` upper-cased with non-alphanumerics → `_`. See `.env.example`.
- **Multitenancy is first-class.** Keep the two axes separate: TENANT = a whole console (own host + token per backend) vs SCOPE = account/site/group inside one console. The facade stays single-tenant; `registry.rs` holds one `SentinelOne` per tenant and `Registry::resolve` turns the `tenant` arg into 1..n tenants (omitted / `"*"` / `"all"` → every tenant in parallel, id/name → one, group label → that subset). Omitting `tenant` is deliberately the fan-out case: a device lookup searches every console, each with its own token and confinement. Fan-out results are `{results: [{tenant, ok, data|error}]}` — one tenant failing must never fail the whole call.
- `tools/call` always answers with a `{content, isError}` result: tool-level failures (bad args, gated action, API error) set `isError: true`; JSON-RPC error objects are reserved for protocol faults (parse, unknown method).
- **Tools are table-driven** (`tools.rs`): 17 `LIST_TOOLS` + 11 `ACTION_TOOLS` const rows over raw Management JSON passthrough, plus 14 hand-coded tools (tenant meta, get-by-id, XDR queries, raw escape hatches). Adding a read tool is usually a new `ListTool` row, not new handler code; a hand-coded tool needs both a `definitions()` entry and an `if name == …` branch in `call()`. Every read tool gets the common params (`tenant`, `query`, `limit`, `cursor`, `sort_by`, `sort_order`) plus a `filters` object passing any API param by exact wire name — so coverage is never limited to the convenience aliases. `s1_get_raw` / `s1_post_raw` keep all 826 endpoints reachable.
- **Destructive tools are triple-gated**: `MCP_ALLOW_ACTIONS=true` AND a single resolved tenant (so `tenant` must be named explicitly whenever more than one is configured) AND `"confirm": true` in the args. Keep all three when adding actions.
- **Confinement chokepoint** (`mcp/src/scope.rs`): `scope::apply` is the only place a Management querystring is built. A tenant with `S1_TENANT_<ID>_SITE_IDS` / `_ACCOUNT_IDS` is confined server-side: caller scope params are stripped and the allow-list appended; XDR and `s1_get_raw` are off for confined tenants unless the `UNSAFE_ALLOW_*` vars opt back in. Route every new Management read through `do_get_json`, never a hand-built query.
- **Everything MCP-specific lives in `mcp/`**: Rust source plus `Dockerfile`, `docker-compose.yml`, `Dockerfile.dockerignore`, `.env.example`, `.env`, `README.md`, `DESIGN.md`. Keep the repo root and `crates/` SDK-only — no MCP deploy files or MCP-only dependencies there. Compose runs from `mcp/` with `context: ..` (path deps need the workspace root); the ignore file uses BuildKit's `<Dockerfile>.dockerignore` next-to-Dockerfile convention. The one sanctioned exception is root `.mcp.json` (Claude Code only auto-loads it from the project root): a single HTTP server `sentinelone` at `localhost:8080`, no secrets.
- Secrets stay in `.env`. OS-keychain storage is reserved for a future interactive CLI, explicitly *not* this server.

## Hosts & auth

Management: `https://<region>-<tenant>.sentinelone.net`, `Authorization: ApiToken <t>`.
XDR / Data Lake: `https://xdr.<region>.sentinelone.net`, `Authorization: Bearer <t>`.
