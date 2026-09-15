# sentinelone-mcp

Multi-tenant **MCP server** exposing SentinelOne (Management + XDR/Data Lake) to
LLM analyst agents. Built on the high-level [`sentinelone`] facade. Search-first,
read-by-default, destructive actions gated.

See [`DESIGN.md`](./DESIGN.md) for the original design plan (its status note
lists what the implementation superseded). This README covers running it.

## Quick start (Docker Compose)

Everything the server needs — Rust source, `Dockerfile`, `docker-compose.yml`,
`.env.example` — lives in this directory (`mcp/`), separate from the SDK crates
under `crates/`. Run from `mcp/`:

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

Files here: `Dockerfile`, `docker-compose.yml`, `.env.example`,
`Dockerfile.dockerignore`. The compose build context is the workspace root
(`..`) because this crate depends on the SDK crates by path; the ignore file
sits next to the Dockerfile (BuildKit's `<Dockerfile>.dockerignore` convention)
so nothing MCP-specific has to live at the repo root.

### Build memory

A release build of the 826-endpoint `sentinelone-api-rs` crate needs about
3 GB of RAM in a single `rustc` process (measured; `codegen-units`, `opt-level`
and `-j` move it by under 10%). On a host with less, Docker kills the build
(`signal: 9, SIGKILL`). Build where the memory is and ship the result.

**Option A — static binary (simplest, no Docker on the server):**

```bash
./build-static.sh                      # from mcp/; static musl binary, runs on any x86_64 Linux
scp ../target/musl/release/sentinelone-mcp SERVER:/usr/local/bin/
# on SERVER, with mcp/.env next to it (or MCP_ENV_FILE=/path/.env):
MCP_TRANSPORT=http sentinelone-mcp     # add a systemd unit to keep it running
```

**Option B — prebuilt image:**

```bash
docker compose build                   # from mcp/, on a machine with RAM
docker save sentinelone-mcp:0.1.0 | gzip | ssh SERVER 'gunzip | docker load'
# on SERVER, from mcp/ (with its own .env):
docker compose up -d                   # no --build
```

## Configuration (env / `.env`)

All config — including secrets (API tokens) — is read from the environment.
Compose loads `mcp/.env` via `env_file`. (Secrets live in `.env` for this *server*;
OS-keychain storage is reserved for a future interactive CLI, not this service.)

| Var | Meaning |
|-----|---------|
| `MCP_TRANSPORT` | `http` (compose default) or `stdio` |
| `MCP_BIND` | bind addr for http (default `0.0.0.0:8080`) |
| `MCP_ALLOW_ACTIONS` | `true` to enable destructive tools (default `false`) |
| `S1_TENANTS` | comma list of tenant ids, e.g. `acme,globex` |
| `S1_TENANT_<ID>_NAME` | display name |
| `S1_TENANT_<ID>_GROUPS` | comma labels for fan-out selection |
| `S1_TENANT_<ID>_MGMT_HOST` / `_MGMT_TOKEN` | Management API console + ApiToken |
| `S1_TENANT_<ID>_XDR_HOST` / `_XDR_TOKEN` | XDR/Data Lake console + Bearer |
| `S1_TENANT_<ID>_SITE_IDS` | **comma-separated site ids this tenant may see. Empty/unset = unrestricted (today's behaviour).** |
| `S1_TENANT_<ID>_ACCOUNT_IDS` | comma-separated account ids this tenant may see, for a customer that owns a whole account rather than one site |
| `S1_TENANT_<ID>_UNSAFE_ALLOW_XDR` | `true` re-enables XDR/Data Lake tools for a confined tenant (they cannot be confined — see below) |
| `S1_TENANT_<ID>_UNSAFE_ALLOW_RAW_GET` | `true` re-enables `s1_get_raw` for a confined tenant (best-effort scoping only) |

`<ID>` = tenant id upper-cased (non-alphanumerics → `_`). A tenant may set only
mgmt, only xdr, or both.

At startup the server prints one line per tenant reporting its confinement, so
an operator can see it is on without reading the config:

```
sentinelone-mcp v0.1.0: 2 tenant(s) loaded; actions disabled
  tenant acme             unrestricted (account-wide)
  tenant globex           confined to 1 site(s)
```

## Multitenancy

One server fronts many consoles. Every tool takes an optional `tenant`:

- omitted (or `"*"`) → **every tenant**, in parallel — a device lookup with no
  `tenant` searches every console the server holds credentials for, each one
  confined to its own scope;
- an id / name → that console;
- a **group label** (e.g. `emea`) → the matching subset.

Fan-out results are tagged with their `tenant`; a failure in one tenant doesn't
fail the whole call. So *"find this hash across all customers"* is one
`s1_power_query` call with no `tenant` at all. Destructive actions are the
exception: they refuse to fan out and require an explicit single `tenant`.

### Site confinement (MSSP tenancy)

**Threat model.** In an MSSP deployment one SentinelOne *account* holds many
customers, one **site** each, and the service-user API token reports
`scope: "account"` — it can read every site. The token therefore cannot confine
a tenant, and neither can the calling application: `site_ids` injected into tool
arguments is client-side and one bug (or one creative LLM) away from being
bypassed. `S1_TENANT_<ID>_SITE_IDS` is what confines a tenant to its customer,
enforced by this server on every API call.

Set it and the tenant becomes **confined**. On every Management API request the
server:

1. **strips** every caller-supplied `siteIds` / `accountIds` / `groupIds` /
   `tenant` parameter — from named tool arguments *and* from the free-form
   `filters` object, under any casing or separator (`site_ids`, `SITEIDS`,
   `Site-Ids`, `siteIds__contains`, …);
2. **intersects** any sites the caller did ask for with the allowed set — a
   caller can narrow within its scope but never widen it;
3. **refuses**, naming the allowed sites, if the intersection is empty, rather
   than silently returning the whole scope (which would read to an agent as
   "that customer has no such data");
4. **appends** the resulting allowed `siteIds` (and `accountIds`, if configured).

This happens in one place — `scope::apply` in `src/scope.rs`, the only function
that builds a Management querystring — reached through `do_get_json`, the only
Management `GET` in the server. A tool handler cannot construct a query that
skipped it.

`s1_list_tenants` and `s1_ping` report each tenant's `scope`, so an agent can
see its boundary instead of inferring it from empty results.

#### Actions and other non-query targets

An action (isolate, mitigate, blocklist, run script, move device, set verdict, …)
names its target by id in a POST body, where a querystring filter does not reach.
For a confined tenant the server therefore resolves every target id through a
**confined read** first (`/agents?ids=…&siteIds=<allowed>` or the threats
equivalent); an id belonging to another customer simply does not come back and
the action is refused. If the verification lookup itself fails, the action is
refused — it fails closed. The same pre-check guards `s1_threat_timeline`,
`s1_threat_events` and `s1_threat_notes`, whose threat id lives in the URL path.

Additionally, for a confined tenant:

- `s1_move_device_to_site` refuses a `targetSiteId` outside the allowed sites —
  otherwise the action would push a device into another customer's site;
- `s1_add_threat_to_blocklist` pins `targetScope` to `site` (or `group`) and
  refuses `account`/`global`, which would apply the block to every customer in
  the account;
- any other body field that sets API scope is refused.

#### What is refused rather than scoped

| Tool | Confined tenant | Why |
|------|-----------------|-----|
| `s1_post_raw` | **always refused**, no opt-out | an arbitrary POST body cannot be confined |
| `s1_get_raw` | refused unless `S1_TENANT_<ID>_UNSAFE_ALLOW_RAW_GET=true` | it can reach paths that name their target in the URL (e.g. `/sites/{id}`) and ignore the injected `siteIds` |
| `s1_power_query`, `s1_query`, `s1_facet`, `s1_numeric`, `s1_timeseries` | refused unless `S1_TENANT_<ID>_UNSAFE_ALLOW_XDR=true` | PQL / DataSet query text is caller-authored against a separate backend; this server has no safe way to rewrite it to one site |
| `s1_list_accounts` | refused unless `_ACCOUNT_IDS` is set | `/accounts` honours `accountIds` but not `siteIds` |

Two further deliberate narrowings for a confined tenant: caller-supplied
`groupIds` are **dropped** (a group id cannot be validated against a site
allow-list without a second lookup, and the API is not documented to AND it with
`siteIds`), so group filtering is unavailable under site confinement; and
`s1_activity_types` is served unconfined because it returns a static enum of
activity type ids with no customer data in it.

Independently of confinement, caller-supplied API paths are now validated:
`s1_get_raw` / `s1_post_raw` reject anything that is not a plain absolute path
on the configured console (the HTTP layer resolves paths with `Url::join`, which
would otherwise follow `https://elsewhere/…` or `//elsewhere/…` to another host
and send the tenant's API token there), and the `{action}` segment of
`s1_mitigate_threat` is restricted to `[A-Za-z0-9_-]` so it cannot add path
segments.

#### Known limits

- Confinement rides on the API honouring `siteIds`. Every curated read tool's
  endpoint is documented to accept it, but an endpoint reached through
  `s1_get_raw` with `UNSAFE_ALLOW_RAW_GET` may ignore an unknown query
  parameter rather than rejecting it — which is exactly why that flag is named
  `UNSAFE_`.
- The XDR / Data Lake backend is not confined at all; a tenant with
  `UNSAFE_ALLOW_XDR` set can hunt across the whole data lake.
- A confined tenant is still selectable by group label or `"*"` fan-out. That is
  safe (each tenant applies its own scope), but it means a `"*"` call mixes
  confined and unconfined tenants' results in one response, each tagged with its
  `tenant`.

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
| `s1_get_raw` | raw GET to ANY Management path (read) — refused for a confined tenant unless `UNSAFE_ALLOW_RAW_GET` |
| `s1_post_raw` | raw POST to ANY Management path (**action**) — always refused for a confined tenant |

**Actions (destructive — gated)**
| Tool | Purpose |
|------|---------|
| `s1_isolate_device`, `s1_reconnect_device` | network isolation |
| `s1_scan_device`, `s1_reboot_device`, `s1_move_device_to_site` | endpoint ops |
| `s1_mitigate_threat`, `s1_add_threat_note`, `s1_set_threat_verdict`, `s1_set_threat_status`, `s1_add_threat_to_blocklist` | threat response |
| `s1_run_remote_script` | run a RemoteOps script |

Actions require `MCP_ALLOW_ACTIONS=true`, a single resolved `tenant`, and
`"confirm": true` — and, for a site-confined tenant, that every target id
resolves inside the tenant's sites (see
[Site confinement](#site-confinement-mssp-tenancy)). Anything not wrapped by a curated tool is still reachable via
`s1_get_raw` / `s1_post_raw` (all 826 Management endpoints).

## Use as an MCP server in Claude

A **`.mcp.json`** at the repo root registers the server for Claude Code (which
only auto-loads a project-scoped `.mcp.json` from the project root — it is the
one MCP-related file outside `mcp/`). It defines a single server:

- **`sentinelone`** — HTTP at `http://localhost:8080/`. Requires
  `docker compose up` (from `mcp/`) to be running. No paths, no secrets.

For a client that must spawn the server itself, add a stdio entry like this
instead. Tokens stay in `mcp/.env` (mounted via `--env-file`), never in the
committable `.mcp.json`; the image must be built first (`docker compose build`
from `mcp/`):

```jsonc
"sentinelone-stdio": {
  "command": "docker",
  "args": ["run", "-i", "--rm", "--env-file", "/abs/path/to/mcp/.env",
           "-e", "MCP_TRANSPORT=stdio", "sentinelone-mcp:0.1.0"]
}
```

Verify from Claude Code:

```bash
claude mcp list          # shows the servers from .mcp.json
```

**Claude Desktop** uses the same `mcpServers` shape in its own config file
(`~/Library/Application Support/Claude/claude_desktop_config.json` on macOS,
`%APPDATA%\Claude\claude_desktop_config.json` on Windows). Desktop spawns the
process, so use the **stdio** form above with an absolute `--env-file` path.

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
cd mcp
cp .env.example .env                            # or point MCP_ENV_FILE at a file
MCP_TRANSPORT=stdio cargo run
# from the repo root instead: MCP_ENV_FILE=mcp/.env MCP_TRANSPORT=stdio cargo run -p sentinelone-mcp
```

[`sentinelone`]: ../crates/sentinelone-rs
