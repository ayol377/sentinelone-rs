# sentinelone-mcp

Multi-tenant **MCP server** exposing SentinelOne (Management + XDR/Data Lake) to
LLM analyst agents. Built on the high-level [`sentinelone`] facade. Search-first,
read-by-default, destructive actions gated.

See [`DESIGN.md`](./DESIGN.md) for the full tool/architecture plan. This README
covers running it.

## Quick start (Docker Compose)

Deployment files live at the **repo root** (not in this crate — a crate is Rust
source only). Run from the repo root:

```bash
cp .env.example .env          # fill in tenants + API tokens
docker compose up --build
```

Server comes up on `http://localhost:8080` (JSON-RPC over HTTP):

```bash
curl -s localhost:8080/health                       # -> ok
curl -s localhost:8080/ -H 'content-type: application/json' \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' | jq
```

Files (repo root): `Dockerfile`, `docker-compose.yml`, `.env.example`,
`.dockerignore`. The compose build context is the workspace root because this
crate has path dependencies on its sibling crates.

## Configuration (env / `.env`)

All config — including secrets (API tokens) — is read from the environment.
Compose loads `.env` via `env_file`. (Secrets live in `.env` for this *server*;
OS-keychain storage is reserved for a future interactive CLI, not this service.)

| Var | Meaning |
|-----|---------|
| `MCP_TRANSPORT` | `http` (compose default) or `stdio` |
| `MCP_BIND` | bind addr for http (default `0.0.0.0:8080`) |
| `MCP_ALLOW_ACTIONS` | `true` to enable destructive tools (default `false`) |
| `S1_TENANTS` | comma list of tenant ids, e.g. `acme,globex` |
| `S1_DEFAULT_TENANT` | tenant used when a call omits `tenant` |
| `S1_TENANT_<ID>_NAME` | display name |
| `S1_TENANT_<ID>_GROUPS` | comma labels for fan-out selection |
| `S1_TENANT_<ID>_MGMT_HOST` / `_MGMT_TOKEN` | Management API console + ApiToken |
| `S1_TENANT_<ID>_XDR_HOST` / `_XDR_TOKEN` | XDR/Data Lake console + Bearer |

`<ID>` = tenant id upper-cased (non-alphanumerics → `_`). A tenant may set only
mgmt, only xdr, or both.

## Multitenancy

One server fronts many consoles. Every tool takes an optional `tenant`:

- omitted → `S1_DEFAULT_TENANT`;
- an id / name → that console;
- a **group label** (e.g. `emea`) or `"*"` → **fan-out**: run across all matching
  tenants in parallel; every result is tagged with its `tenant`, and a failure in
  one tenant doesn't fail the whole call.

So *"find this hash across all customers"* is one `s1_power_query` call with
`tenant:"*"`.

## Tools (42)

Every read tool takes the common params `tenant`, `query`, `limit`, `cursor`,
`sort_by`, `sort_order`, and a **`filters`** object accepting any API query param
by its exact wire name (e.g. `{"osTypes":"windows"}`) — so coverage isn't limited
to the convenience aliases below.

**Meta / scope**
| Tool | Purpose |
|------|---------|
| `s1_list_tenants`, `s1_ping` | consoles + configured backends |
| `s1_list_sites`, `s1_list_groups`, `s1_list_accounts` | org/scope tree |

**Devices**
| Tool | Purpose |
|------|---------|
| `s1_get_device` | one agent, full fields (IPs, OS, scope, tags) |
| `s1_search_devices` | hostname, IP/subnet, OS, online, infected, scope, … |
| `s1_count_devices` | fast "how many match" |

**Threats**
| Tool | Purpose |
|------|---------|
| `s1_search_threats`, `s1_get_threat` | detections by host/time/verdict/status/engine |
| `s1_threat_timeline`, `s1_threat_events`, `s1_threat_notes` | forensic detail |

**Activity / inventory / vulns / intel**
| Tool | Purpose |
|------|---------|
| `s1_search_activity`, `s1_activity_types` | audit log |
| `s1_application_inventory` | installed software |
| `s1_search_cves`, `s1_apps_with_risk`, `s1_endpoints_for_vuln` | vulnerabilities ("who has Log4j") |
| `s1_search_exclusions`, `s1_search_blocklist`, `s1_search_iocs` | allow/block/IOC |
| `s1_list_remote_scripts`, `s1_remote_script_status` | RemoteOps scripts |

**Hunting (XDR / Data Lake)**
| Tool | Purpose |
|------|---------|
| `s1_power_query` | arbitrary PQL hunt |
| `s1_query` | DataSet log/event filter query |
| `s1_facet`, `s1_numeric`, `s1_timeseries` | aggregate / trend / spike |

**Escape hatches**
| Tool | Purpose |
|------|---------|
| `s1_get_raw` | raw GET to ANY Management path (read) |
| `s1_post_raw` | raw POST to ANY Management path (**action**) |

**Actions (destructive — gated)**
| Tool | Purpose |
|------|---------|
| `s1_isolate_device`, `s1_reconnect_device` | network isolation |
| `s1_scan_device`, `s1_reboot_device`, `s1_move_device_to_site` | endpoint ops |
| `s1_mitigate_threat`, `s1_add_threat_note`, `s1_set_threat_verdict`, `s1_set_threat_status`, `s1_add_threat_to_blocklist` | threat response |
| `s1_run_remote_script` | run a RemoteOps script |

Actions require `MCP_ALLOW_ACTIONS=true`, a single resolved `tenant`, and
`"confirm": true`. Anything not wrapped by a curated tool is still reachable via
`s1_get_raw` / `s1_post_raw` (all 826 Management endpoints).

## Use as an MCP server in Claude

A ready-to-use **`.mcp.json`** is at the repo root (Claude Code auto-loads
project-scoped `.mcp.json`). It defines two servers — pick one:

- **`sentinelone`** — HTTP. Requires `docker compose up` running. Zero paths.
- **`sentinelone-stdio`** — Claude spawns `docker run -i`. Requires the image
  built (`docker compose build`) and a `.env` in the launch dir.

Tokens are NOT in `.mcp.json` (it's committable) — they stay in `.env`, which the
stdio entry mounts via `--env-file`.

Verify from Claude Code:

```bash
claude mcp list          # shows the servers from .mcp.json
```

**Claude Desktop** uses the same `mcpServers` shape in its own config file
(`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS,
`%APPDATA%\Claude\claude_desktop_config.json` on Windows). Desktop spawns the
process, so use the **stdio** entry (copy the `sentinelone-stdio` block, with an
absolute `--env-file` path).

Without Docker (local binary):

```jsonc
{
  "mcpServers": {
    "sentinelone": {
      "command": "/abs/path/target/release/sentinelone-mcp",
      "env": { "MCP_TRANSPORT": "stdio", "S1_TENANTS": "acme",
               "S1_TENANT_ACME_MGMT_HOST": "https://…", "S1_TENANT_ACME_MGMT_TOKEN": "…" }
    }
  }
}
```

## Local dev (no Docker)

```bash
cp .env.example .env                            # repo root, or set MCP_ENV_FILE
MCP_TRANSPORT=stdio cargo run -p sentinelone-mcp
```

[`sentinelone`]: ../sentinelone-rs
