# sentinelone-mcp — design / tool plan

MCP server exposing SentinelOne to an LLM analyst agent. Built **on top of
`sentinelone-rs`** (the high-level unified facade), never on the raw API crates.
The server holds one configured `SentinelOne` (management + XDR), and each MCP
tool is a thin, safety-shaped wrapper over a facade call.

Status: **historical design plan**, written before any code (June 2026). v0.1
shipped 2026-06-27; [`README.md`](./README.md) and `src/tools.rs` describe what
exists. Where this doc and the code disagree, the code won. Superseded decisions:

| Planned here | Shipped |
|---|---|
| `rmcp` SDK, stdio only | hand-rolled JSON-RPC 2.0 (`src/mcp.rs`); stdio + HTTP (axum) transports |
| TOML tenants file + OS keychain (§2.5) | env-only config, secrets in `mcp/.env` (`src/config.rs`) |
| `--allow-actions` + two-call preview → `confirm_token` (§7, §8.4) | `MCP_ALLOW_ACTIONS=true` + single resolved tenant + `"confirm": true` |
| `s1_use_tenant` / `s1_add_tenant` session state (§6) | not built; `tenant` arg per call, omitted = every tenant |
| `{field, op, value}` filter DSL + discovery tools (§3) | per-tool convenience aliases + a raw `filters` object passing any API param by wire name |
| `s1_pivot_*`, guided hunt builders, resources/prompts (§4.3, §5, §8.5) | not built; `s1_power_query` and `s1_get_raw` / `s1_post_raw` cover the ground |
| normalized, envelope-stripped output | raw Management JSON passthrough, fan-out tagged `{tenant, ok, data\|error}` |

Still accurate: §1 (why search + pivot), the tenant-vs-scope model and fan-out
contract in §2.5, the gating intent of §7, and the CIDR notes in §8.1 (today's
`ip` alias is a `networkInterfaceInet__contains` prefix match, no client-side
CIDR check yet).

---

## 1. Why an MCP layer

An analyst working an investigation runs the same loop dozens of times:

> identify an entity → pull its context → **pivot** (same subnet, same user,
> same hash, same external IP, same storyline) → **ask a custom question** of the
> data → aggregate to spot the outlier → decide a response action.

The two verbs that dominate are **search** and **pivot**. So the tool surface is
built around them, not around a flat list of endpoints. MCP shapes the SDK for an
LLM driver: few intent-named tools, a shared search model, normalized/compact
output, read-first with mutations gated, both backends behind one mental model.

---

## 2. Design principles

| Principle | Rule |
|-----------|------|
| Built on facade | tools call `sentinelone::SentinelOne` only — no raw-crate use |
| Search-first | one shared filter DSL across all entity searches; learnable, composable |
| Discoverable | the agent can ask what fields exist + what values they take, at runtime |
| Naming | tools prefixed `s1_`, snake_case, verb-first |
| Read vs write | every tool tagged `read`/`action`; `action` off unless `--allow-actions` + confirm |
| Output | envelope stripped, sane field subset; `select`/`verbose` to control width |
| Multi-tenant | first-class — one MCP server fronts many consoles; every tool takes `tenant`; `tenant:"*"` fans out (see §2.5) |
| Scope | accept human scope names (account/site/group); resolve to ids internally |
| Bounded | `limit`+`cursor` always; `count_only` for "how many?"; report truncation explicitly |
| Time | human ranges ("24h","7d", RFC3339); default last 24h on event/time tools |
| Errors | unconfigured backend → typed tool error, never panic (facade behavior) |

Transport: **stdio** first. SDK: `rmcp`. Secrets: env + OS keychain (planned
`secrecy`/`keyring`); tokens never on argv.

---

## 2.5 Multitenancy (MSSP) — first-class

The reality: an analyst (or MSSP/SOC) works **many SentinelOne consoles at
once**, each a separate host + token (the working console is literally
`…-mssp.sentinelone.net`). Multitenancy is therefore designed in, not bolted on.

**Two distinct axes — keep them separate:**
- **Tenant** = a whole console: its own *mgmt host+token* and *xdr host+token*.
  Switching tenant = switching credentials/endpoints. → many `SentinelOne`
  facades.
- **Scope** = within ONE console: account / site / group (one token, many
  accounts). Already handled by the `scope` param + `s1_resolve_scope`.
  MSSP parent consoles also expose child accounts as scopes under one token —
  both models supported.

**Server model.** The facade stays single-tenant (clean). The MCP server holds a
`TenantRegistry: Map<TenantId, SentinelOne>` — each entry a cheap `Arc` facade
clone, built lazily. A tenant may configure only mgmt or only xdr; the facade's
`Option` backends + checked accessors make that null-safe automatically.

**`tenant` param on every tool:**
- omitted → the **session default** tenant (set by `s1_use_tenant`, else the
  config default, else the sole tenant);
- a tenant id/name → that console;
- `"*"` or a **tenant group label** (e.g. `"emea"`, `"customer-acme"`) →
  **cross-tenant fan-out**: run the tool against each matching tenant in
  parallel, merge, and tag every result with its `tenant`. This is the MSSP
  killer move — *"find this hash across all 40 customers"* in one call.

**Config & secrets.** A tenants config file (TOML) lists, per tenant:
`id`, `name`, optional `group(s)`, `mgmt_host`, `xdr_host`, and **keychain refs**
for the two tokens — never inline secrets. Tokens resolve from the OS keychain by
tenant id at use. Example:

```toml
default = "acme-prod"
[[tenant]]
id = "acme-prod"
groups = ["customer-acme", "prod"]
mgmt_host = "https://apse1-2111-mssp.sentinelone.net"
xdr_host  = "https://xdr.ap1.sentinelone.net"
# tokens via keychain: service "sentinelone-mcp", account "acme-prod/mgmt" & "/xdr"
```

**Tenant tools** (also see §6): `s1_list_tenants` (enumerate + which backends
each has), `s1_use_tenant` (set session default), `s1_add_tenant` (register at
runtime; tokens to keychain — `action`, gated).

**Output contract.** Every object/row carries `tenant` (and existing `scope`)
so merged cross-tenant results stay attributable and the agent never confuses
hosts. Per-tenant failures in a fan-out are returned as partial results +
an `errors[]` list, never a hard failure of the whole call.

**Concurrency/limits.** Fan-out is bounded (configurable max parallel tenants);
each tenant keeps its own connection pool (separate `reqwest::Client` via the
facade). Rate-limit/`429` from one tenant doesn't stall the others.

---

## 3. The search model (shared by all `s1_search_*` tools)

Three search **modes**, one **filter DSL**, plus **discovery**. This is the core
of the crate — every entity search tool reuses it.

### 3.0 Modes
1. **Structured filter** — `match`: a list of `{field, op, value}` clauses, AND by
   default; nested `{or:[...]}` / `{not:{...}}` for boolean trees.
2. **Free-text** — `q`: substring across the entity's text fields (maps to each
   entity's `free-text-filters`).
3. **Aggregate** — `facet` / `timeseries` / `numeric` (see §3.3), for counts,
   trends, top-N, outliers.

### 3.1 Operators (`op`)
`eq, ne, in, nin, contains, like (wildcards), regex, gt, gte, lt, lte, between,
exists, cidr, before, after`. Each entity tool maps an `(field, op)` pair to the
right backend: mgmt query-param suffix (`__contains`, `__like`, `In`, ranges) or
a PQL/S1QL clause for XDR. Unsupported `(field,op)` → tool error naming the
field, not a silent drop.

### 3.2 Common params (every search tool)
`tenant` (id/name/group/`"*"`; default session tenant — see §2.5), `match`, `q`,
`sort` (`field:asc|desc`, multi), `select` (projected fields), `limit`, `cursor`,
`count_only` (bool), `scope` (account/site/group names or ids), `verbose`.

### 3.3 Aggregation tools (cross-entity)
| Tool | Purpose | Backend |
|------|---------|---------|
| `s1_facet` | group-by + count → top values of a field (top external IPs, OS mix, top parent procs) | mgmt `filters-count` / XDR `facetQuery` |
| `s1_timeseries` | counts over time buckets → trend / spike detection | XDR `timeseriesQuery` |
| `s1_numeric` | sum/avg/min/max/percentile over a numeric field | XDR `numericQuery` |

Each takes the same `match`/`q`/`scope`/time as the matching entity search.

### 3.4 Discovery tools (make the DSL self-teaching)
| Tool | Returns |
|------|---------|
| `s1_describe_filters` | `entity` → filterable fields + types + allowed ops |
| `s1_filter_values` | `entity, field, prefix?` → enumerated/allowed values (autocomplete / facet) |
| `s1_list_entities` | the searchable entities + which backend serves each |

Map to per-entity `filters-autocomplete` + `free-text-filters` + `filters-count`.
Lets the agent construct valid advanced filters without hardcoded field lists.

---

## 4. Entity search tools

Per-entity tools (self-documenting schemas) all consuming §3's model.

### 4.1 `s1_search_devices` — **the deep one**
Devices/agents are the most-filtered entity. Supported `match` fields, grouped
(this is the analyst's pivot palette — *not* just IP):

- **Identity:** `agent_id`, `uuid`, `short_id`, `hostname`(eq/contains/like/regex),
  `serial`, `console_visible`
- **Network:** `local_ip`(+`cidr`/range), `ipv6`, `mac`, `gateway_ip`,
  `gateway_mac`, `external_ip`(+`cidr`), `interface_name`, `dhcp`,
  `network_status`(connected/disconnecting/disconnected), `network_quarantined`
- **OS/platform:** `os_type`(win/mac/linux), `os_name`, `os_version`,
  `os_build`, `os_arch`, `last_reboot`
- **Hardware/machine:** `machine_type`(server/desktop/laptop/k8s/vm/storage),
  `model`, `manufacturer`, `cpu`, `core_count`, `memory_mb`, `is_vm`
- **Agent/health:** `agent_version`(=,<,>,between → "outdated"), `state`,
  `registered_at`, `last_seen`(range → "offline > 30d"), `scan_status`,
  `last_scan_at`, `reboot_required`, `pending_uninstall`, `decommissioned`,
  `missing_permissions`, `up_to_date`, `console_migration`, `update_status`
- **Security state:** `infected`(active threats>0), `active_threats`(count
  range), `isolated`, `mitigation_mode`(detect/protect), `learning_mode`,
  `anti_tamper`, `firewall_enabled`, `device_control_enabled`,
  `remote_shell_available`, `encryption`(bitlocker/filevault),
  `ranger_status`, `rogue`/`unmanaged`
- **User/domain:** `last_user`, `domain`, `workgroup`, `azure_ad_joined`
- **Scope/org:** `account`, `site`, `group`(incl. dynamic), `scope_path`
- **Cloud:** `cloud_provider`, `cloud_account`, `instance_id`, `region`,
  `image_id`, `security_group`, `cloud_tag`(k/v), `k8s_cluster`/`node`/`namespace`
- **Software:** `has_app`(name/vendor/version op), `has_cve`
- **Tags:** `tag`(key/value)
- **Free text:** `q`

Companions:
| Tool | Use |
|------|-----|
| `s1_get_device` | full normalized context by id (`verbose?`) |
| `s1_get_device_network` | interfaces (inet[],inet6[],mac,gateway), external ip |
| `s1_get_device_apps` | installed software (filter by name/cve) |

`s1_get_device` default fields: `id, uuid, hostname, os, os_version,
agent_version, machine_type, model, last_seen, online, network_status, isolated,
external_ip, local_ips[], mac[], domain, last_user, account, site, group,
active_threats, infected, needs_reboot, up_to_date, scan_status, encryption,
tags[]`.

### 4.2 `s1_search_threats`
`match` fields: `device`(host/ip/uuid), `detected_at`/`reported_at`(range),
`classification`(malware/PUA/ransomware/lateral/…), `confidence`(malicious/
suspicious), `verdict`(TP/FP/undefined), `mitigation_status`,
`incident_status`(unresolved/in-progress/resolved), `engine`(static-AI/
behavioral-AI/reputation/STAR/on-write/…), `file_name`, `file_path`,
`sha1`/`sha256`/`md5`, `file_size`, `signer`/`signed`, `process_name`, `cmdline`,
`storyline`, `mitre_technique`/`tactic`, `initiated_by`, `note_contains`,
`external_ticket`, `account`/`site`/`group`, `q`.
Companions: `s1_get_threat` (file/proc-tree/engine/MITRE/indicators),
`s1_threat_timeline`, `s1_threat_notes`.

### 4.3 `s1_search_events` — Deep Visibility / hunt (XDR)
`match` over event fields; key filter `event_type`(process, file, network/ip,
dns, url, registry, scheduled_task, login, cross_process, command/script,
module_load, …) + `host`/`endpoint` filters + time. Plus `s1_query` raw PQL
escape hatch. **Guided builders** (so the agent pivots without PQL fluency),
each a thin preset over `s1_search_events`:
| Tool | Pivot |
|------|-------|
| `s1_process_history` | `host?`,`process?`,`cmdline_contains?` |
| `s1_network_connections` | `host?`,`remote_ip?`(cidr),`port?`,`direction?` |
| `s1_dns_lookups` | `host?`,`domain_contains?` |
| `s1_file_events` | `host?`,`hash?`,`path_contains?` |
| `s1_logons` | `host?`,`user?`,`type?` |
| `s1_hash_sightings` | `hash` → every host/threat that saw it |
| `s1_query` | raw PQL, power-user |

All event/aggregate results: `{columns[], rows[][], matched, truncated,
query_used}` — surface the generated PQL for transparency.

### 4.4 `s1_search_apps_vulns` — software & CVEs
`match`: `app_name`,`vendor`,`version`(op),`cve`,`cve_severity`,
`exploited_in_wild`,`has_patch`,`risk_score`,`endpoints_affected`(range),`scope`.
Drives "who runs Log4j ≤ x", "all endpoints with CVE-2021-44228",
"unpatched criticals by site". Companion `s1_get_cve` (detail + affected hosts).

### 4.5 `s1_search_activity` — audit / activity log
`match`: `device`,`user`(actor),`activity_type`,`target`,`time`(range),`scope`,
`threat_related`,`q`. Use for "who isolated this host", "what changed in 7d".

### 4.6 `s1_search_users` — console users / RBAC / identity
`match`: `email`/`name`,`role`,`last_login`,`2fa`,`sso`,`service_user`,
`api_token_active`,`scope`. (If Identity module present: risky accounts / exposed
creds — P2.)

### 4.7 IOC / intel / blocklist / rules
| Tool | Use |
|------|-----|
| `s1_ioc_lookup` | `value`(hash/ip/domain/url) → reputation + local sightings |
| `s1_search_blocklist` | exclusions / blocklist items by value/type/scope |
| `s1_search_rules` | STAR custom detection rules by name/status/severity |

---

## 5. Pivot tools — "show me related"

Investigation is graph traversal. One tool family turns an entity id into its
neighbors so the agent doesn't rebuild filters each hop:

| Tool | From → To |
|------|-----------|
| `s1_pivot_device` | device → `threats`/`events`/`apps`/`activity`/`same_subnet`/`same_user` |
| `s1_pivot_threat` | threat → `storyline_events`/`same_hash_hosts`/`process_tree`/`same_device` |
| `s1_pivot_hash` | hash → all sightings (hosts, threats, events) |
| `s1_pivot_indicator` | ip/domain/url → connections + hosts that touched it |
| `s1_pivot_user` | user → devices, logons, threats |
| `s1_pivot_storyline` | storyline id → full correlated event graph |

Each = a parameterized search under the hood; named so the agent's intent is
explicit and the result is pre-scoped.

---

## 6. Tenants, scope, saved work, health

| Tool | Use |
|------|-----|
| `s1_list_tenants` | enumerate configured consoles + which backends each has (§2.5) |
| `s1_use_tenant` | set the session default tenant for subsequent calls |
| `s1_add_tenant` | register a console at runtime; tokens → keychain (`action`, gated) |
| `s1_resolve_scope` | name → {account/site/group id}; backs every tool's `scope` |
| `s1_list_sites` / `s1_list_groups` / `s1_list_accounts` | org tree (within a tenant) |
| `s1_saved_searches` | list/run saved queries & hunts (recurring investigations) |
| `s1_ping` | whoami / configured backends / active tenant + scope (health check) |

---

## 7. Response actions (destructive) — gated

Off unless `--allow-actions`. Two-call **preview → confirm**: first call returns
`{would_affect:N, targets:[...], confirm_token}`; re-call with `confirm_token` to
execute. No silent fan-out; never auto-confirm.

| Tool | Params | Maps to |
|------|--------|---------|
| `s1_isolate_device` / `s1_reconnect_device` | `device_id`, `confirm_token?` | agent_actions |
| `s1_scan_device` / `s1_reboot_device` | `device_id`, `confirm_token?` | agent_actions |
| `s1_mitigate_threat` | `threat_id`, `action`(quarantine/kill/remediate/rollback), `confirm_token?` | threats |
| `s1_add_threat_note` | `threat_id`, `text` | threat-notes (low-risk) |
| `s1_tag_device` | `device_id`, `tag`, `confirm_token?` | tags |
| `s1_run_script` | `device_id`, `script_id`, `args?`, `confirm_token?` | RemoteOps (async) — real "run command" path |
| `s1_fetch_script_result` | `task_id` | RemoteOps task output (read) |

`s1_run_script`+`s1_fetch_script_result` = non-interactive execute (RemoteOps
task model), **not** remote-shell (no REST stream; console-WebSocket only —
out of scope).

---

## 8. Implementation notes / open questions

### 8.1 IP & subnet (`cidr` op)
Mgmt agents filter has only substring matchers (`networkInterfaceInet__contains`,
`externalIp__contains`) — **no native CIDR**. `cidr` op strategy:
1. octet-boundary CIDR (`/8/16/24`) → derive prefix string, server `__contains`
   prefilter, then exact in-CIDR client check (kills `10.0.5x` false hits).
2. arbitrary CIDR (`/26`) → coarsest octet prefilter, paginate, client-side
   `ipnet` containment.
3. very large scope → XDR PowerQuery on endpoint IP fields.
Return `method_used` so the agent knows exact-vs-scan-bounded (no silent trunc).
Use the `ipnet` crate.

### 8.2 Backend routing per field
Each searchable field is tagged mgmt / XDR / both. Mgmt = current-state +
inventory + filter-count/autocomplete; XDR = historical events + aggregation +
arbitrary hunt. `s1_search_events`/aggregations → XDR; device/threat/app/user
state → mgmt. Some pivots span both (threat → storyline events: mgmt threat,
XDR events).

### 8.3 Facade gaps this drives
Facade today surfaces only `accounts`/`agents`/`agent_actions`/`power_query`.
MCP forces high-level accessors for: threats, activities, sites, groups,
applications/vulns, users/rbac, tags, blocklist/exclusions, STAR rules,
remoteops, saved-searches, plus `filters-count`/`autocomplete`/`free-text`
discovery per entity, and XDR facet/numeric/timeseries. **MCP is the forcing
function for the facade's ergonomic + search layer.** Add accessors per tool tier.

### 8.4 Confirmation model
Lean two-call preview→confirm_token (transport-agnostic, works in any MCP
client) over MCP elicitation. Decide before building §7.

### 8.5 Resources & prompts (later)
- Resources: `s1://device/{id}`, `s1://threat/{id}`, `s1://sites`.
- Prompt playbooks: `investigate_host`, `triage_threat`, `subnet_sweep`,
  `hash_hunt`, `cve_exposure`.

---

## 9. Build order

1. **Scaffold** — `rmcp` stdio server, **tenant registry + config (TOML) +
   keychain** (§2.5), `Map<TenantId, SentinelOne>`, `tenant` param plumbing +
   fan-out runner, `s1_ping`/`s1_list_tenants`/`s1_use_tenant`, the §3 search
   model + DSL types.
2. **Discovery + device search (P0)** — `s1_describe_filters`,
   `s1_filter_values`, `s1_search_devices` (full field set), `s1_get_device*`,
   `s1_facet`. ← advanced device search end-to-end.
3. **Hunt (P0)** — `s1_search_events` + guided builders + `s1_query` +
   `s1_timeseries`/`s1_numeric`.
4. **Threats + pivots (P1)** — `s1_search_threats`, `s1_get_threat`,
   `s1_pivot_*`.
5. **Breadth (P1)** — apps/vulns, activity, users, IOC/intel, scope, saved.
6. **Actions (P2, gated)** — §7 + RemoteOps.
7. Resources + prompt playbooks.

Each tier may add facade accessors to `sentinelone-rs` (§8.3).
