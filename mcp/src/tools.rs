//! Tool catalog and handlers. Tools are thin, multi-tenant wrappers over the
//! high-level `sentinelone` facade (mgmt + XDR). Read tools fan out across the
//! selected tenant(s); destructive tools are gated.
//!
//! Breadth strategy: most read/search tools are table-driven over the raw
//! Management JSON passthrough (`ManagementClient::get_json`), each with a
//! `filters` object that accepts any documented API query param by its exact
//! wire name. Two escape hatches — `s1_get_raw` / `s1_post_raw` — guarantee that
//! *any* of the 826 Management endpoints is reachable even if no curated tool
//! wraps it.
//!
//! ## Confinement
//!
//! Every Management `GET` in this module goes through [`do_get_json`], which
//! calls [`scope::apply`] — the single place a querystring is built. A tenant
//! configured with `S1_TENANT_<ID>_SITE_IDS` therefore cannot have an
//! out-of-scope request constructed for it, no matter what the caller passes.
//! Targets named in a *path* (threat sub-resources) or in a POST body (actions)
//! are confined instead by resolving the id through a confined read first and
//! refusing when it does not come back.

use futures::future::join_all;
use serde_json::{json, Map, Value};

use sentinelone::xdr_api::{FacetQueryRequest, TimeseriesQuerySpec};

use crate::mcp::ServerState;
use crate::registry::Tenant;
use crate::scope;

const TENANT_HELP: &str =
    "Tenant selector: id, name, group label, or \"*\" for all (fan-out). Omit to run against every tenant.";

// ---- table-driven read tools --------------------------------------------

/// A read/search tool backed by a raw Management `GET`.
struct ListTool {
    name: &'static str,
    desc: &'static str,
    path: &'static str,
    /// (arg name, API query-param wire name, description) convenience filters.
    params: &'static [(&'static str, &'static str, &'static str)],
}

/// Device/agent filters — shared by `s1_search_devices` and `s1_count_devices`.
const DEVICE_PARAMS: &[(&str, &str, &str)] = &[
    ("computer_name", "computerName__contains", "Substring match on hostname"),
    ("ip", "networkInterfaceInet__contains", "Local IP substring; for a /24 subnet pass the prefix, e.g. \"10.0.5.\""),
    ("external_ip", "externalIp__contains", "External/public IP substring"),
    ("os_types", "osTypes", "OS type(s): windows, macos, linux (csv or array)"),
    ("machine_types", "machineTypes", "Machine type(s): server, desktop, laptop, kubernetes node, …"),
    ("network_statuses", "networkStatuses", "connected, disconnecting, disconnected"),
    ("last_user", "lastLoggedInUserName__contains", "Last logged-in user substring"),
    ("uuids", "uuids", "Agent UUID(s)"),
    ("ids", "ids", "Agent id(s)"),
    ("infected", "infected", "true = has active threats"),
    ("is_active", "isActive", "true = currently online"),
    ("scan_statuses", "scanStatuses", "none, started, finished, aborted"),
    ("agent_versions", "agentVersions", "Agent version(s)"),
    ("site_ids", "siteIds", "Scope: site id(s)"),
    ("group_ids", "groupIds", "Scope: group id(s)"),
    ("account_ids", "accountIds", "Scope: account id(s)"),
];

const THREAT_PARAMS: &[(&str, &str, &str)] = &[
    ("computer_name", "computerName__contains", "Host substring"),
    ("ids", "ids", "Threat id(s)"),
    ("mitigation_statuses", "mitigationStatuses", "mitigated, active, blocked, …"),
    ("incident_statuses", "incidentStatuses", "unresolved, in_progress, resolved"),
    ("analyst_verdicts", "analystVerdicts", "true_positive, false_positive, suspicious, undefined"),
    ("classifications", "classifications", "Malware, PUA, Ransomware, …"),
    ("confidence_levels", "confidenceLevels", "malicious, suspicious"),
    ("engines", "engines", "Detection engine(s)"),
    ("from", "createdAt__gte", "Detected at/after (RFC3339)"),
    ("to", "createdAt__lte", "Detected at/before (RFC3339)"),
    ("site_ids", "siteIds", "Scope: site id(s)"),
    ("group_ids", "groupIds", "Scope: group id(s)"),
    ("account_ids", "accountIds", "Scope: account id(s)"),
];

const LIST_TOOLS: &[ListTool] = &[
    ListTool { name: "s1_list_sites", desc: "List sites (scope tree). Filter by name; any other param via `filters`.", path: "/web/api/v2.1/sites", params: &[("name", "name__contains", "Substring on site name"), ("states", "states", "active, deleted, expired")] },
    ListTool { name: "s1_list_groups", desc: "List groups within scope.", path: "/web/api/v2.1/groups", params: &[("name", "name__contains", "Substring on group name"), ("site_ids", "siteIds", "Restrict to site id(s)"), ("type", "type", "static or dynamic")] },
    ListTool { name: "s1_list_accounts", desc: "List accounts.", path: "/web/api/v2.1/accounts", params: &[("states", "states", "active, deleted, expired")] },
    ListTool { name: "s1_search_devices", desc: "Search agents/devices: free-text + rich filters (hostname, IP/subnet, OS, online, infected, scope, …). Fan-out capable.", path: "/web/api/v2.1/agents", params: DEVICE_PARAMS },
    ListTool { name: "s1_count_devices", desc: "Count agents matching a device filter (fast 'how many?'). Same filters as s1_search_devices.", path: "/web/api/v2.1/agents/count", params: DEVICE_PARAMS },
    ListTool { name: "s1_search_threats", desc: "Search threats/detections by host, time, classification, verdict, mitigation/incident status, engine, scope.", path: "/web/api/v2.1/threats", params: THREAT_PARAMS },
    ListTool { name: "s1_search_activity", desc: "Search the activity/audit log (who did what, when).", path: "/web/api/v2.1/activities", params: &[("ids", "ids", "Activity id(s)"), ("from", "createdAt__gte", "At/after (RFC3339)"), ("to", "createdAt__lte", "At/before (RFC3339)"), ("activity_types", "activityTypes", "Numeric activity type id(s)"), ("user_ids", "userIds", "Actor user id(s)"), ("agent_ids", "agentIds", "Related agent id(s)")] },
    ListTool { name: "s1_activity_types", desc: "List activity type ids and their meanings (for filtering s1_search_activity).", path: "/web/api/v2.1/activities/types", params: &[] },
    ListTool { name: "s1_application_inventory", desc: "Installed software inventory across endpoints. Use `filters` for exact params (e.g. applicationName, ids).", path: "/web/api/v2.1/application-management/inventory", params: &[("ids", "ids", "Agent id(s)")] },
    ListTool { name: "s1_search_cves", desc: "Known CVEs / application vulnerabilities. Filters: cveIds, severities, exploitedInTheWild, hasFix.", path: "/web/api/v2.1/application-management/risks/cves", params: &[] },
    ListTool { name: "s1_apps_with_risk", desc: "Applications carrying risk/CVEs (which software is vulnerable).", path: "/web/api/v2.1/application-management/risks/applications", params: &[] },
    ListTool { name: "s1_endpoints_for_vuln", desc: "Endpoints affected by a vulnerable app/CVE (\"who has Log4j?\"). Filters: cveId, applicationName.", path: "/web/api/v2.1/application-management/risks/endpoints", params: &[] },
    ListTool { name: "s1_search_exclusions", desc: "Whitelist / exclusion items.", path: "/web/api/v2.1/exclusions", params: &[("type", "type", "path, hash, certificate, browser, file_type, …"), ("os_types", "osTypes", "OS type(s)")] },
    ListTool { name: "s1_search_blocklist", desc: "Blocklist (restriction) items — blocked hashes etc.", path: "/web/api/v2.1/restrictions", params: &[("type", "type", "black_hash, …"), ("os_types", "osTypes", "OS type(s)")] },
    ListTool { name: "s1_search_iocs", desc: "Threat-intelligence IOCs (hash/ip/domain/url indicators).", path: "/web/api/v2.1/threat-intelligence/iocs", params: &[("type", "type", "IOC type: DNS, IPV4, SHA1, SHA256, MD5, URL, …")] },
    ListTool { name: "s1_list_remote_scripts", desc: "Available RemoteOps scripts (for s1_run_remote_script).", path: "/web/api/v2.1/remote-scripts", params: &[] },
    ListTool { name: "s1_remote_script_status", desc: "Status of RemoteOps script tasks. Filters: computerName, parentTaskId, status.", path: "/web/api/v2.1/remote-scripts/status", params: &[] },
];

// ---- table-driven action tools (gated) ----------------------------------

/// What kind of entity an action targets — used to confine it by resolving the
/// target ids through a scoped read before acting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Agent,
    Threat,
}

impl Target {
    /// The scoped list endpoint used to prove a target id is in scope.
    fn list_path(self) -> &'static str {
        match self {
            Target::Agent => "/web/api/v2.1/agents",
            Target::Threat => "/web/api/v2.1/threats",
        }
    }
    fn noun(self) -> &'static str {
        match self {
            Target::Agent => "agent",
            Target::Threat => "threat",
        }
    }
}

/// A destructive/mutating tool backed by a raw Management `POST`.
struct ActionTool {
    name: &'static str,
    desc: &'static str,
    /// May contain `{action}` to be substituted from `path_param`.
    path: &'static str,
    /// (arg name, body `data` field wire name, description).
    data: &'static [(&'static str, &'static str, &'static str)],
    /// (arg name, description) for a `{action}` path segment, if any.
    path_param: Option<(&'static str, &'static str)>,
    /// Entity kind the `ids` refer to, so confinement can verify them.
    target: Target,
}

const ACTION_TOOLS: &[ActionTool] = &[
    ActionTool { name: "s1_isolate_device", desc: "Network-isolate (disconnect) agent(s).", path: "/web/api/v2.1/agents/actions/disconnect", data: &[], path_param: None , target: Target::Agent },
    ActionTool { name: "s1_reconnect_device", desc: "Reconnect previously isolated agent(s).", path: "/web/api/v2.1/agents/actions/connect", data: &[], path_param: None , target: Target::Agent },
    ActionTool { name: "s1_scan_device", desc: "Start a full disk scan on agent(s).", path: "/web/api/v2.1/agents/actions/initiate-scan", data: &[], path_param: None , target: Target::Agent },
    ActionTool { name: "s1_reboot_device", desc: "Reboot agent machine(s).", path: "/web/api/v2.1/agents/actions/restart-machine", data: &[], path_param: None , target: Target::Agent },
    ActionTool { name: "s1_move_device_to_site", desc: "Move agent(s) to another site.", path: "/web/api/v2.1/agents/actions/move-to-site", data: &[("site_id", "targetSiteId", "Destination site id")], path_param: None , target: Target::Agent },
    ActionTool { name: "s1_mitigate_threat", desc: "Mitigate threat(s) with the given action.", path: "/web/api/v2.1/threats/mitigate/{action}", data: &[], path_param: Some(("action", "One of: kill, quarantine, un-quarantine, remediate, rollback-remediation, network-quarantine")) , target: Target::Threat },
    ActionTool { name: "s1_add_threat_note", desc: "Add a note to threat(s).", path: "/web/api/v2.1/threats/notes", data: &[("text", "text", "Note text")], path_param: None , target: Target::Threat },
    ActionTool { name: "s1_set_threat_verdict", desc: "Set analyst verdict on threat(s).", path: "/web/api/v2.1/threats/analyst-verdict", data: &[("verdict", "analystVerdict", "true_positive | false_positive | suspicious | undefined")], path_param: None , target: Target::Threat },
    ActionTool { name: "s1_set_threat_status", desc: "Set incident status on threat(s).", path: "/web/api/v2.1/threats/incident", data: &[("status", "incidentStatus", "unresolved | in_progress | resolved")], path_param: None , target: Target::Threat },
    ActionTool { name: "s1_add_threat_to_blocklist", desc: "Add threat(s) hash to the blocklist.", path: "/web/api/v2.1/threats/add-to-blacklist", data: &[], path_param: None , target: Target::Threat },
    ActionTool { name: "s1_run_remote_script", desc: "Run a RemoteOps script on agent(s). Provide script parameters via `data` (e.g. scriptId, taskDescription, outputDestination). Poll results with s1_remote_script_status.", path: "/web/api/v2.1/remote-scripts/execute", data: &[("script_id", "scriptId", "Id of the script to run")], path_param: None , target: Target::Agent },
];

// ---- definitions ---------------------------------------------------------

/// JSON-Schema tool definitions returned by `tools/list`.
pub fn definitions() -> Vec<Value> {
    let from_p = json!({ "type": "string", "description": "Window start: relative like \"24h\" / \"7d\", RFC3339, or epoch ms. Default \"24h\"." });
    let to_p = json!({ "type": "string", "description": "Window end: RFC3339 or epoch ms (\"now\" is NOT accepted by the XDR host). Default: current time." });
    let tenant_p = json!({ "type": "string", "description": TENANT_HELP });

    let mut tools = vec![
        json!({ "name": "s1_list_tenants", "description": "List configured consoles, which backends (mgmt/xdr) each has, and the site scope each is confined to. Check this first: a confined tenant can only ever return data for its own sites.",
                "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false } }),
        json!({ "name": "s1_ping", "description": "Health check: for the selected tenant(s), report configured backends and site confinement.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p }, "additionalProperties": false } }),
        json!({ "name": "s1_get_device", "description": "Fetch one agent by id/UUID with full fields (IPs, OS, scope, tags). Fan-out capable.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "id": { "type": "string", "description": "Agent id or UUID." } }, "required": ["id"], "additionalProperties": false } }),
        json!({ "name": "s1_get_threat", "description": "Fetch one threat by id (full detection context).",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "id": { "type": "string", "description": "Threat id." } }, "required": ["id"], "additionalProperties": false } }),
        json!({ "name": "s1_threat_timeline", "description": "Forensic timeline of a threat.",
                "inputSchema": threat_sub_schema() }),
        json!({ "name": "s1_threat_events", "description": "Deep Visibility events explored from a threat (process tree etc.).",
                "inputSchema": threat_sub_schema() }),
        json!({ "name": "s1_threat_notes", "description": "Notes attached to a threat.",
                "inputSchema": threat_sub_schema() }),
        // ---- XDR / Data Lake hunting ----
        json!({ "name": "s1_power_query", "description": "Run a PowerQuery (PQL) against the XDR / Data Lake host. Returns columns + rows. Fan-out capable. NOTE: on a multi-account console this host can return zero rows with no error; prefer s1_dv_power_query there.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "pql": { "type": "string", "description": "PowerQuery text." }, "from": from_p, "to": to_p }, "required": ["pql"], "additionalProperties": false } }),
        json!({ "name": "s1_dv_power_query", "description": "Run a PowerQuery (PQL) through the Management console (POST /dv/events/pq + pq-ping) instead of the XDR host. Same ApiToken as every other tool; honours the token's multi-account scope and takes optional account_ids / site_ids. Preferred over s1_power_query on multi-account consoles. Polls up to ~2 min. Fan-out capable.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "pql": { "type": "string", "description": "PowerQuery text." },
                    "from": { "type": "string", "description": "RFC3339 start, e.g. 2026-09-14T00:00:00Z (required; DataSet relative syntax is NOT accepted here)." },
                    "to": { "type": "string", "description": "RFC3339 end." },
                    "account_ids": { "type": "array", "items": { "type": "string" }, "description": "Account id(s) to scope to." },
                    "site_ids": { "type": "array", "items": { "type": "string" }, "description": "Site id(s) to scope to." },
                    "limit": { "type": "integer", "description": "Max rows (1-100000). Default 1000." } },
                  "required": ["pql", "from", "to"], "additionalProperties": false } }),
        json!({ "name": "s1_query", "description": "Run a DataSet log/event query (filter expression). Returns matching events.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "filter": { "type": "string", "description": "DataSet filter expression. Empty = all events." }, "from": from_p, "to": to_p, "limit": { "type": "integer", "description": "Max events." } }, "additionalProperties": false } }),
        json!({ "name": "s1_facet", "description": "Top values of a field over XDR events (group-by + count). Fan-out capable.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "filter": { "type": "string", "description": "DataSet filter. Empty = all." }, "field": { "type": "string", "description": "Field to facet." }, "from": from_p, "to": to_p, "max_count": { "type": "integer" } }, "required": ["field"], "additionalProperties": false } }),
        json!({ "name": "s1_numeric", "description": "Numeric aggregation over XDR events (count, mean:field, p90:field, …), optionally bucketed.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "filter": { "type": "string" }, "function": { "type": "string", "description": "e.g. count, mean:responseTime, p95:latency" }, "from": from_p, "to": to_p, "buckets": { "type": "integer" } }, "required": ["function"], "additionalProperties": false } }),
        json!({ "name": "s1_timeseries", "description": "Time-bucketed counts/aggregation over XDR events (trend / spike detection).",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "filter": { "type": "string" }, "function": { "type": "string", "description": "e.g. count, mean:field" }, "from": from_p, "to": to_p, "buckets": { "type": "integer", "description": "Number of time buckets. Default 24." } }, "required": ["function"], "additionalProperties": false } }),
        // ---- raw escape hatches ----
        json!({ "name": "s1_get_raw", "description": "Escape hatch: raw GET to ANY Management API path (reach endpoints with no curated tool). Read-only. Disabled for a site-confined tenant (an arbitrary path may ignore the injected siteIds filter) unless the operator opts in.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "path": { "type": "string", "description": "API path, e.g. /web/api/v2.1/ranger/tables" }, "query": { "type": "string", "description": "Pre-serialized querystring, e.g. limit=10&osTypes=windows" } }, "required": ["path"], "additionalProperties": false } }),
        json!({ "name": "s1_post_raw", "description": "[ACTION] Escape hatch: raw POST of a JSON body to ANY Management API path. Gated; single tenant; confirm required. Always refused for a site-confined tenant — use a curated action tool, which verifies its targets are in scope.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "path": { "type": "string" }, "body": { "type": "object", "additionalProperties": true, "description": "Raw JSON request body." }, "confirm": { "type": "boolean" } }, "required": ["path"], "additionalProperties": false } }),
    ];

    for lt in LIST_TOOLS {
        tools.push(json!({ "name": lt.name, "description": lt.desc, "inputSchema": list_schema(lt.params) }));
    }
    for at in ACTION_TOOLS {
        tools.push(json!({ "name": at.name, "description": format!("[ACTION] {} Requires MCP_ALLOW_ACTIONS=true, a single tenant, and \"confirm\": true. For a site-confined tenant every target id is resolved through a scoped read first and the action is refused if it belongs elsewhere.", at.desc), "inputSchema": action_schema(at) }));
    }
    tools
}

fn threat_sub_schema() -> Value {
    json!({ "type": "object", "properties": {
        "tenant": { "type": "string", "description": TENANT_HELP },
        "id": { "type": "string", "description": "Threat id." },
        "limit": { "type": "integer" }, "cursor": { "type": "string" }
    }, "required": ["id"], "additionalProperties": false })
}

fn list_schema(params: &[(&str, &str, &str)]) -> Value {
    let mut props = json!({
        "tenant": { "type": "string", "description": TENANT_HELP },
        "query": { "type": "string", "description": "Free-text search." },
        "limit": { "type": "integer", "description": "Max items (1-1000). Default 50." },
        "cursor": { "type": "string", "description": "Pagination cursor from a prior response." },
        "sort_by": { "type": "string", "description": "Field to sort by." },
        "sort_order": { "type": "string", "description": "asc or desc." },
        "filters": { "type": "object", "additionalProperties": true, "description": "Any additional API query params by exact wire name, e.g. {\"osTypes\":\"windows\"}. Scope keys (siteIds/accountIds/groupIds/tenant) are stripped here and replaced by the server's configured scope." }
    });
    let m = props.as_object_mut().unwrap();
    for (arg, _p, d) in params {
        m.insert((*arg).to_string(), json!({ "description": d }));
    }
    json!({ "type": "object", "properties": props, "additionalProperties": false })
}

fn action_schema(at: &ActionTool) -> Value {
    let mut props = json!({
        "tenant": { "type": "string", "description": TENANT_HELP },
        "id": { "type": "string", "description": "Target id (agent or threat)." },
        "ids": { "type": "array", "items": { "type": "string" }, "description": "Multiple target ids (alternative to `id`)." },
        "data": { "type": "object", "additionalProperties": true, "description": "Extra raw body `data` fields." },
        "confirm": { "type": "boolean", "description": "Must be true to execute." }
    });
    let m = props.as_object_mut().unwrap();
    for (arg, _f, d) in at.data {
        m.insert((*arg).to_string(), json!({ "description": d }));
    }
    if let Some((arg, d)) = at.path_param {
        m.insert(arg.to_string(), json!({ "type": "string", "description": d }));
    }
    json!({ "type": "object", "properties": props, "required": ["id"], "additionalProperties": false })
}

// ---- dispatch ------------------------------------------------------------

/// Dispatch a `tools/call`. Always returns a tools/call result object
/// (`{content, isError}`); tool-level failures use `isError`, not protocol errors.
pub async fn call(state: &ServerState, name: &str, args: Value) -> Value {
    let result: Result<Value, String> = if name == "s1_list_tenants" {
        list_tenants(state)
    } else if name == "s1_ping" {
        ping(state, &args)
    } else if name == "s1_get_device" {
        get_device(state, &args).await
    } else if name == "s1_get_threat" {
        get_threat(state, &args).await
    } else if name == "s1_threat_timeline" {
        threat_sub(state, &args, "timeline").await
    } else if name == "s1_threat_events" {
        threat_sub(state, &args, "explore/events").await
    } else if name == "s1_threat_notes" {
        threat_sub(state, &args, "notes").await
    } else if name == "s1_power_query" {
        power_query(state, &args).await
    } else if name == "s1_dv_power_query" {
        dv_power_query(state, &args).await
    } else if name == "s1_query" {
        log_query(state, &args).await
    } else if name == "s1_facet" {
        facet(state, &args).await
    } else if name == "s1_numeric" {
        numeric(state, &args).await
    } else if name == "s1_timeseries" {
        timeseries(state, &args).await
    } else if name == "s1_get_raw" {
        get_raw(state, &args).await
    } else if name == "s1_post_raw" {
        post_raw(state, &args).await
    } else if let Some(lt) = LIST_TOOLS.iter().find(|l| l.name == name) {
        list_endpoint(state, &args, lt.path, lt.params).await
    } else if let Some(at) = ACTION_TOOLS.iter().find(|a| a.name == name) {
        run_action(state, &args, at).await
    } else {
        Err(format!("unknown tool: {name}"))
    };
    match result {
        Ok(v) => text_result(v),
        Err(e) => error_result(e),
    }
}

// ---- non-fanout / meta tools --------------------------------------------

fn list_tenants(state: &ServerState) -> Result<Value, String> {
    let tenants: Vec<Value> = state
        .registry
        .all()
        .iter()
        .map(|t| json!({
            "id": t.cfg.id,
            "name": t.cfg.name,
            "groups": t.cfg.groups,
            "backends": backends(t),
            "scope": scope_report(t),
        }))
        .collect();
    Ok(json!({ "actions_enabled": state.settings.allow_actions, "tenants": tenants }))
}

fn ping(state: &ServerState, args: &Value) -> Result<Value, String> {
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let results: Vec<Value> = tenants
        .iter()
        .map(|t| json!({ "tenant": t.cfg.id, "ok": true, "backends": backends(t), "scope": scope_report(t) }))
        .collect();
    Ok(json!({ "results": results }))
}

// ---- read tools (fan-out) ------------------------------------------------

async fn get_device(state: &ServerState, args: &Value) -> Result<Value, String> {
    let id = req_str(args, "id")?;
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let id = id.as_str();
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_get_device(t, id).await)
    }))
    .await;
    Ok(fanout(res))
}

async fn get_threat(state: &ServerState, args: &Value) -> Result<Value, String> {
    let id = req_str(args, "id")?;
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let pairs = vec![("ids".to_string(), id)];
    let pairs = &pairs;
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_get_json(t, "/web/api/v2.1/threats", pairs).await)
    }))
    .await;
    Ok(fanout(res))
}

async fn threat_sub(state: &ServerState, args: &Value, suffix: &str) -> Result<Value, String> {
    let id = req_str(args, "id")?;
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let mut pairs = vec![];
    if let Some(c) = str_opt(args, "cursor") {
        pairs.push(("cursor".into(), c));
    }
    if let Some(l) = args.get("limit").and_then(Value::as_i64) {
        pairs.push(("limit".into(), l.clamp(1, 1000).to_string()));
    }
    let path = format!("/web/api/v2.1/threats/{id}/{suffix}");
    let (path, pairs, id) = (path.as_str(), &pairs, id.as_str());
    let res = join_all(tenants.into_iter().map(|t| async move {
        // The threat id lives in the *path*, so a site filter in the query
        // cannot confine this. Prove the threat is in scope first.
        let out = match verify_targets_in_scope(t, Target::Threat, &[id.to_string()]).await {
            Err(e) => Err(e),
            Ok(()) => do_get_json(t, path, pairs).await,
        };
        (t.cfg.id.clone(), out)
    }))
    .await;
    Ok(fanout(res))
}

async fn list_endpoint(
    state: &ServerState,
    args: &Value,
    path: &str,
    params: &[(&str, &str, &str)],
) -> Result<Value, String> {
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let mut pairs = common_pairs(args);
    for (arg, param, _) in params {
        if let Some(v) = args.get(*arg) {
            push_param(&mut pairs, param, v);
        }
    }
    merge_filters(args, &mut pairs);
    let pairs = &pairs;
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_get_json(t, path, pairs).await)
    }))
    .await;
    Ok(fanout(res))
}

async fn get_raw(state: &ServerState, args: &Value) -> Result<Value, String> {
    let path = req_str(args, "path")?;
    validate_path(&path)?;
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let query = str_opt(args, "query").unwrap_or_default();
    let (path, query) = (path.as_str(), query.as_str());
    let res = join_all(tenants.into_iter().map(|t| async move {
        let out = match gate_raw_get(t) {
            Err(e) => Err(e),
            Ok(()) => match parse_query(query) {
                Err(e) => Err(e),
                // Re-serialized through the chokepoint, so the caller's
                // querystring is subject to the same stripping and intersection
                // as a curated tool's parameters.
                Ok(pairs) => do_get_json(t, path, &pairs).await,
            },
        };
        (t.cfg.id.clone(), out)
    }))
    .await;
    Ok(fanout(res))
}

/// `s1_get_raw` can reach *any* Management path, including ones that do not
/// implement `siteIds` (e.g. `/sites/{id}`), where forcing the scope into the
/// query would not confine anything. Refuse it for a confined tenant unless the
/// operator explicitly accepts that risk.
fn gate_raw_get(t: &Tenant) -> Result<(), String> {
    if t.cfg.is_scoped() && !t.cfg.unsafe_allow_raw_get {
        return Err(format!(
            "s1_get_raw is disabled for tenant {}: it is confined to sites [{}], and an arbitrary \
             Management path may ignore the siteIds filter this server injects. Use a curated \
             s1_* tool, or set S1_TENANT_<ID>_UNSAFE_ALLOW_RAW_GET=true to accept best-effort \
             scoping.",
            t.cfg.id,
            t.cfg.site_ids.join(", "),
        ));
    }
    Ok(())
}

/// Reject a caller-supplied API path that is not a plain absolute path on the
/// configured console. The HTTP layer resolves the path with `Url::join`, which
/// would happily follow an absolute (`https://elsewhere/…`) or protocol-relative
/// (`//elsewhere/…`) value to another host — sending the tenant's API token
/// there and escaping confinement entirely.
fn validate_path(path: &str) -> Result<(), String> {
    if !path.starts_with('/') || path.starts_with("//") || path.contains("://") {
        return Err(format!(
            "invalid API path {path:?}: must be an absolute path on the configured console, \
             e.g. /web/api/v2.1/agents"
        ));
    }
    Ok(())
}

/// A caller-supplied value interpolated into a URL path (e.g. the mitigation
/// action). Must not be able to introduce new path segments.
fn validate_path_segment(v: &str) -> Result<(), String> {
    if v.is_empty() || !v.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err(format!(
            "invalid path segment {v:?}: expected letters, digits, `-` or `_` only"
        ));
    }
    Ok(())
}

fn parse_query(q: &str) -> Result<Vec<(String, String)>, String> {
    if q.is_empty() {
        return Ok(vec![]);
    }
    serde_urlencoded::from_str::<Vec<(String, String)>>(q)
        .map_err(|e| format!("could not parse `query` as a querystring: {e}"))
}

// ---- XDR / Data Lake tools (fan-out) ------------------------------------

async fn power_query(state: &ServerState, args: &Value) -> Result<Value, String> {
    let pql = req_str(args, "pql")?;
    let (from, to) = window(args);
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let (pql, from, to) = (pql.as_str(), from.as_str(), to.as_str());
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_power_query(t, pql, from, to).await)
    }))
    .await;
    Ok(fanout(res))
}

async fn dv_power_query(state: &ServerState, args: &Value) -> Result<Value, String> {
    let pql = req_str(args, "pql")?;
    let from = req_str(args, "from")?;
    let to = req_str(args, "to")?;
    let limit = args.get("limit").and_then(Value::as_i64).unwrap_or(1000).clamp(1, 100_000);
    let accounts = str_array(args, "account_ids");
    let sites = str_array(args, "site_ids");
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let (pql, from, to, accounts, sites) = (pql.as_str(), from.as_str(), to.as_str(), &accounts, &sites);
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_dv_power_query(t, pql, from, to, limit, accounts, sites).await)
    }))
    .await;
    Ok(fanout(res))
}

async fn log_query(state: &ServerState, args: &Value) -> Result<Value, String> {
    let filter = str_opt(args, "filter").unwrap_or_default();
    let (from, to) = window(args);
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let (filter, from, to) = (filter.as_str(), from.as_str(), to.as_str());
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_log_query(t, filter, from, to).await)
    }))
    .await;
    Ok(fanout(res))
}

async fn facet(state: &ServerState, args: &Value) -> Result<Value, String> {
    let field = req_str(args, "field")?;
    let filter = str_opt(args, "filter").unwrap_or_default();
    let (from, to) = window(args);
    let max_count = args.get("max_count").and_then(Value::as_i64);
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let (field, filter, from, to) = (field.as_str(), filter.as_str(), from.as_str(), to.as_str());
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_facet(t, filter, field, from, to, max_count).await)
    }))
    .await;
    Ok(fanout(res))
}

async fn numeric(state: &ServerState, args: &Value) -> Result<Value, String> {
    let function = req_str(args, "function")?;
    let filter = str_opt(args, "filter").unwrap_or_default();
    let (from, to) = window(args);
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let (function, filter, from, to) = (function.as_str(), filter.as_str(), from.as_str(), to.as_str());
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_numeric(t, filter, function, from, to).await)
    }))
    .await;
    Ok(fanout(res))
}

async fn timeseries(state: &ServerState, args: &Value) -> Result<Value, String> {
    let function = req_str(args, "function")?;
    let filter = str_opt(args, "filter").unwrap_or_default();
    let (from, to) = window(args);
    let buckets = args.get("buckets").and_then(Value::as_i64).unwrap_or(24);
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let (function, filter, from, to) = (function.as_str(), filter.as_str(), from.as_str(), to.as_str());
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_timeseries(t, filter, function, from, to, buckets).await)
    }))
    .await;
    Ok(fanout(res))
}

// ---- action tools (gated, single tenant) --------------------------------

async fn run_action(state: &ServerState, args: &Value, act: &ActionTool) -> Result<Value, String> {
    let t = gate_single_tenant(state, args)?;
    let ids = ids_from_args(args)?;

    let path = match act.path_param {
        Some((arg, _)) => {
            let seg = req_str(args, arg)?;
            validate_path_segment(&seg)?;
            act.path.replace("{action}", &seg)
        }
        None => act.path.to_string(),
    };

    let mut data = Map::new();
    for (arg, field, _) in act.data {
        if let Some(v) = args.get(*arg) {
            data.insert((*field).to_string(), v.clone());
        }
    }
    if let Some(obj) = args.get("data").and_then(Value::as_object) {
        for (k, v) in obj {
            data.insert(k.clone(), v.clone());
        }
    }

    // Confinement: an action targets entities by id, so the site filter this
    // server injects into read queries does not reach it. Refuse unless the
    // body is scope-neutral *and* every target resolves inside our scope.
    if t.cfg.is_scoped() {
        enforce_action_body_scope(t, act, &mut data)?;
        verify_targets_in_scope(t, act.target, &ids).await?;
    }

    let mut body = Map::new();
    body.insert("filter".into(), json!({ "ids": ids }));
    if !data.is_empty() {
        body.insert("data".into(), Value::Object(data));
    }

    let mgmt = t.client.management().map_err(|e| e.to_string())?;
    let resp = mgmt.post_json(&path, &Value::Object(body)).await.map_err(|e| e.to_string())?;
    Ok(json!({ "tenant": t.cfg.id, "action": act.name, "path": path, "result": resp }))
}

async fn post_raw(state: &ServerState, args: &Value) -> Result<Value, String> {
    let t = gate_single_tenant(state, args)?;
    if t.cfg.is_scoped() {
        return Err(format!(
            "s1_post_raw is refused for tenant {}: it is confined to sites [{}], and an arbitrary \
             POST body cannot be confined by this server. Use a curated s1_* action tool, which \
             verifies its targets are in scope.",
            t.cfg.id,
            t.cfg.site_ids.join(", "),
        ));
    }
    let path = req_str(args, "path")?;
    validate_path(&path)?;
    let body = args.get("body").cloned().unwrap_or_else(|| json!({}));
    let mgmt = t.client.management().map_err(|e| e.to_string())?;
    let resp = mgmt.post_json(&path, &body).await.map_err(|e| e.to_string())?;
    Ok(json!({ "tenant": t.cfg.id, "path": path, "result": resp }))
}

/// Reject a mutating body that would set or widen API scope, and validate the
/// one destination field that legitimately names a site.
fn enforce_action_body_scope(
    t: &Tenant,
    act: &ActionTool,
    data: &mut Map<String, Value>,
) -> Result<(), String> {
    // Destination of a device move: the target site must be one of ours, or the
    // action would push a device into another customer's site.
    if let Some(v) = data.get("targetSiteId") {
        let dest = v.as_str().unwrap_or_default().to_string();
        if !t.cfg.site_ids.iter().any(|s| *s == dest) {
            return Err(format!(
                "refusing to move device(s) to site {dest}: this server is confined to sites [{}]",
                t.cfg.site_ids.join(", "),
            ));
        }
    }

    // A blocklist entry created at account or global scope would apply to every
    // customer in the account. Pin it to the agent's own site/group.
    if act.name == "s1_add_threat_to_blocklist" {
        let ts = data
            .get("targetScope")
            .and_then(Value::as_str)
            .unwrap_or("site")
            .to_ascii_lowercase();
        if ts != "site" && ts != "group" {
            return Err(format!(
                "refusing targetScope={ts:?}: a blocklist entry at that scope would apply to every \
                 customer in this account. Use \"site\" or \"group\"."
            ));
        }
        data.insert("targetScope".into(), Value::String(ts));
    }

    // Nothing else in the body may name or widen a scope. `targetScope` and
    // `targetSiteId` are the two fields validated above.
    let offenders: Vec<String> = data
        .keys()
        .filter(|k| {
            scope::mentions_scope_id(k)
                && k.as_str() != "targetScope"
                && k.as_str() != "targetSiteId"
        })
        .cloned()
        .collect();
    if let Some(k) = offenders.first() {
        return Err(format!(
            "refusing action: body field `{k}` sets API scope, which a confined tenant may not do"
        ));
    }
    Ok(())
}

/// Prove that every target id belongs to this tenant's scope by resolving it
/// through a *confined* read: the lookup carries our `siteIds`, so an id that
/// belongs to another customer simply does not come back.
async fn verify_targets_in_scope(t: &Tenant, target: Target, ids: &[String]) -> Result<(), String> {
    if !t.cfg.is_scoped() {
        return Ok(());
    }
    let pairs = vec![
        ("ids".to_string(), ids.join(",")),
        ("limit".to_string(), ids.len().clamp(1, 1000).to_string()),
    ];
    let resp = do_get_json(t, target.list_path(), &pairs).await.map_err(|e| {
        format!(
            "refusing: could not verify the {} target(s) are inside this tenant's scope ({e})",
            target.noun()
        )
    })?;
    let found = ids_in_response(&resp);
    let missing = scope::unresolved_ids(ids, &found);
    if !missing.is_empty() {
        return Err(format!(
            "refusing: {} id(s) [{}] are not in this tenant's scope (sites [{}])",
            target.noun(),
            missing.join(", "),
            t.cfg.site_ids.join(", "),
        ));
    }
    Ok(())
}

/// Collect `data[].id` from a Management list envelope.
fn ids_in_response(v: &Value) -> Vec<String> {
    v.get("data")
        .and_then(Value::as_array)
        .map(|a| {
            a.iter()
                .filter_map(|d| d.get("id").and_then(Value::as_str).map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

/// XDR / Data Lake queries are caller-authored PQL / DataSet filter text against
/// a separate backend; this server has no safe way to rewrite them so they only
/// touch one site. A confined tenant therefore cannot use them by default.
fn gate_xdr(t: &Tenant) -> Result<(), String> {
    if t.cfg.is_scoped() && !t.cfg.unsafe_allow_xdr {
        return Err(format!(
            "XDR / Data Lake tools are disabled for tenant {}: it is confined to sites [{}], but \
             PowerQuery / DataSet query text is caller-authored and this server cannot confine it \
             to a site. Set S1_TENANT_<ID>_UNSAFE_ALLOW_XDR=true to accept the risk.",
            t.cfg.id,
            t.cfg.site_ids.join(", "),
        ));
    }
    Ok(())
}

fn gate_single_tenant<'a>(state: &'a ServerState, args: &Value) -> Result<&'a Tenant, String> {
    if !state.settings.allow_actions {
        return Err("destructive actions are disabled; start the server with MCP_ALLOW_ACTIONS=true".into());
    }
    if !args.get("confirm").and_then(Value::as_bool).unwrap_or(false) {
        return Err("refusing to run a destructive action without \"confirm\": true".into());
    }
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    if tenants.len() != 1 {
        return Err("destructive actions require a single tenant; specify `tenant` explicitly".into());
    }
    Ok(tenants[0])
}

// ---- per-tenant workers --------------------------------------------------

/// Fetch one agent through the *confined* list endpoint (the Management API has
/// no get-by-id route). Tries `ids` then `uuids`; a device outside this tenant's
/// scope is simply not found.
async fn do_get_device(t: &Tenant, id: &str) -> Result<Value, String> {
    const AGENTS: &str = "/web/api/v2.1/agents";
    for key in ["ids", "uuids"] {
        let pairs = vec![(key.to_string(), id.to_string()), ("limit".to_string(), "1".to_string())];
        let resp = do_get_json(t, AGENTS, &pairs).await?;
        if let Some(first) = resp.get("data").and_then(Value::as_array).and_then(|a| a.first()) {
            return Ok(first.clone());
        }
    }
    Err(format!("agent {id} not found in this tenant's scope"))
}

/// **The only** Management `GET` in this server. The querystring is produced by
/// [`scope::apply`], so confinement is applied to every read without each tool
/// handler having to remember it.
async fn do_get_json(t: &Tenant, path: &str, pairs: &[(String, String)]) -> Result<Value, String> {
    let qs = scope::apply(&t.cfg, path, pairs.to_vec())?;
    let mgmt = t.client.management().map_err(|e| e.to_string())?;
    mgmt.get_json(path, qs.as_deref()).await.map_err(|e| e.to_string())
}

async fn do_power_query(t: &Tenant, pql: &str, from: &str, to: &str) -> Result<Value, String> {
    gate_xdr(t)?;
    let resp = t.client.power_query(pql).from(from).to(to).run().await.map_err(|e| e.to_string())?;
    Ok(json!({ "columns": resp.columns, "values": resp.values, "matches": resp.matches }))
}

/// PowerQuery via the Management console (`/dv/events/pq`), polled to
/// completion. Scope lives in the JSON body rather than a querystring, so
/// confinement is applied here with the same intersection rule `scope::apply`
/// uses: a confined tenant's allow-list wins, an unconfined one passes the
/// caller's ids through.
async fn do_dv_power_query(
    t: &Tenant,
    pql: &str,
    from: &str,
    to: &str,
    limit: i64,
    account_ids: &[String],
    site_ids: &[String],
) -> Result<Value, String> {
    let cfg = &t.cfg;
    let mut body = json!({ "query": pql, "fromDate": from, "toDate": to, "limit": limit });
    let (accounts, sites) = if cfg.is_scoped() {
        (
            if cfg.account_ids.is_empty() { vec![] } else { scope::intersect(account_ids, &cfg.account_ids, "accounts")? },
            if cfg.site_ids.is_empty() { vec![] } else { scope::intersect(site_ids, &cfg.site_ids, "sites")? },
        )
    } else {
        (account_ids.to_vec(), site_ids.to_vec())
    };
    if !accounts.is_empty() {
        body["accountIds"] = json!(accounts);
    }
    if !sites.is_empty() {
        body["siteIds"] = json!(sites);
    }
    let mgmt = t.client.management().map_err(|e| e.to_string())?;
    let mut resp = mgmt.post_json("/web/api/v2.1/dv/events/pq", &body).await.map_err(|e| e.to_string())?;
    // ponytail: fixed 2s poll, 60 tries (~2 min); make it configurable if analysts hit the ceiling.
    for _ in 0..60 {
        let status = resp["data"]["status"].as_str().unwrap_or("").to_ascii_uppercase();
        if status != "RUNNING" {
            break;
        }
        let Some(qid) = resp["data"]["queryId"].as_str().map(String::from) else { break };
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let qs = scope::apply(cfg, "/web/api/v2.1/dv/events/pq-ping", vec![("queryId".into(), qid)])?;
        resp = mgmt.get_json("/web/api/v2.1/dv/events/pq-ping", qs.as_deref()).await.map_err(|e| e.to_string())?;
    }
    Ok(json!({ "scope_sent": { "accountIds": accounts, "siteIds": sites }, "result": resp["data"] }))
}

async fn do_log_query(t: &Tenant, filter: &str, from: &str, to: &str) -> Result<Value, String> {
    gate_xdr(t)?;
    let xdr = t.client.xdr().map_err(|e| e.to_string())?;
    let resp = xdr.query(filter, from, to).await.map_err(|e| e.to_string())?;
    Ok(json!({
        "status": resp.status,
        "matching_events": resp.matching_events,
        "matches": resp.matches,
        "continuation_token": resp.continuation_token,
    }))
}

async fn do_facet(t: &Tenant, filter: &str, field: &str, from: &str, to: &str, max_count: Option<i64>) -> Result<Value, String> {
    gate_xdr(t)?;
    let xdr = t.client.xdr().map_err(|e| e.to_string())?;
    let mut req = FacetQueryRequest::new(filter, field, from, to);
    if let Some(n) = max_count {
        req = req.max_count(n);
    }
    let resp = xdr.facet_query_with(&req).await.map_err(|e| e.to_string())?;
    let values: Vec<Value> = resp.values.iter().map(|f| json!({ "value": f.value, "count": f.count })).collect();
    Ok(json!({ "matching_events": resp.matching_events, "values": values }))
}

async fn do_numeric(t: &Tenant, filter: &str, function: &str, from: &str, to: &str) -> Result<Value, String> {
    gate_xdr(t)?;
    let xdr = t.client.xdr().map_err(|e| e.to_string())?;
    let resp = xdr.numeric_query(filter, function, from, to).await.map_err(|e| e.to_string())?;
    Ok(json!({ "values": resp.values, "warnings": resp.warnings }))
}

async fn do_timeseries(t: &Tenant, filter: &str, function: &str, from: &str, to: &str, buckets: i64) -> Result<Value, String> {
    gate_xdr(t)?;
    let xdr = t.client.xdr().map_err(|e| e.to_string())?;
    let spec = TimeseriesQuerySpec::new(filter, function, from, to).buckets(buckets);
    let resp = xdr.timeseries_query(vec![spec]).await.map_err(|e| e.to_string())?;
    let series: Vec<Value> = resp.results.iter().map(|r| json!({ "values": r.values })).collect();
    Ok(json!({ "results": series }))
}

// ---- helpers -------------------------------------------------------------

/// What this tenant is confined to, for `s1_list_tenants` / `s1_ping` so an
/// agent can see its boundary instead of inferring it from empty results.
fn scope_report(t: &Tenant) -> Value {
    json!({
        "confined": t.cfg.is_scoped(),
        "site_ids": t.cfg.site_ids,
        "account_ids": t.cfg.account_ids,
        "summary": scope::describe(&t.cfg),
        "xdr_tools": if !t.cfg.is_scoped() || t.cfg.unsafe_allow_xdr { "allowed" } else { "refused (cannot be confined)" },
        "raw_get": if !t.cfg.is_scoped() || t.cfg.unsafe_allow_raw_get { "allowed" } else { "refused (cannot be confined)" },
        "raw_post": if t.cfg.is_scoped() { "refused (cannot be confined)" } else { "allowed" },
    })
}

fn backends(t: &Tenant) -> Vec<&'static str> {
    let mut b = Vec::new();
    if t.has_mgmt() {
        b.push("mgmt");
    }
    if t.has_xdr() {
        b.push("xdr");
    }
    b
}

/// XDR-host time window. S1's XDR host rejects DataSet's `now` ("Can't parse
/// date [now]") but accepts `24h`-style relative starts, RFC3339, and epoch ms,
/// so the default end is the current time as epoch milliseconds.
fn window(args: &Value) -> (String, String) {
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis().to_string())
        .unwrap_or_default();
    (
        str_opt(args, "from").unwrap_or_else(|| "24h".into()),
        str_opt(args, "to").unwrap_or(now_ms),
    )
}

fn common_pairs(args: &Value) -> Vec<(String, String)> {
    let mut p = vec![];
    if let Some(q) = str_opt(args, "query") {
        p.push(("query".into(), q));
    }
    if let Some(c) = str_opt(args, "cursor") {
        p.push(("cursor".into(), c));
    }
    if let Some(l) = args.get("limit").and_then(Value::as_i64) {
        p.push(("limit".into(), l.clamp(1, 1000).to_string()));
    }
    if let Some(s) = str_opt(args, "sort_by") {
        p.push(("sortBy".into(), s));
    }
    if let Some(s) = str_opt(args, "sort_order") {
        p.push(("sortOrder".into(), s));
    }
    p
}

fn merge_filters(args: &Value, pairs: &mut Vec<(String, String)>) {
    if let Some(f) = args.get("filters").and_then(Value::as_object) {
        for (k, v) in f {
            push_param(pairs, k, v);
        }
    }
}

fn push_param(pairs: &mut Vec<(String, String)>, name: &str, v: &Value) {
    if let Some(s) = value_to_param(v) {
        pairs.push((name.to_string(), s));
    }
}

/// Stringify a JSON value for use as a query param. Arrays become comma-joined
/// (the API's convention). `null`/objects are skipped.
fn value_to_param(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => (!s.is_empty()).then(|| s.clone()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Number(n) => Some(n.to_string()),
        Value::Array(a) => {
            let parts: Vec<String> = a.iter().filter_map(value_to_param).collect();
            (!parts.is_empty()).then(|| parts.join(","))
        }
        _ => None,
    }
}

fn ids_from_args(args: &Value) -> Result<Vec<String>, String> {
    if let Some(id) = str_opt(args, "id") {
        return Ok(vec![id]);
    }
    let ids = str_array(args, "ids");
    if ids.is_empty() {
        Err("provide `id` or `ids`".into())
    } else {
        Ok(ids)
    }
}

/// Wrap per-tenant `(id, Result)` pairs into a uniform, attributable shape.
fn fanout(pairs: Vec<(String, Result<Value, String>)>) -> Value {
    let results: Vec<Value> = pairs
        .into_iter()
        .map(|(tenant, r)| match r {
            Ok(data) => json!({ "tenant": tenant, "ok": true, "data": data }),
            Err(error) => json!({ "tenant": tenant, "ok": false, "error": error }),
        })
        .collect();
    json!({ "results": results })
}

fn text_result(value: Value) -> Value {
    let text = serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string());
    json!({ "content": [{ "type": "text", "text": text }], "isError": false })
}

fn error_result(message: String) -> Value {
    json!({ "content": [{ "type": "text", "text": message }], "isError": true })
}

fn str_opt(args: &Value, key: &str) -> Option<String> {
    args.get(key).and_then(Value::as_str).filter(|s| !s.is_empty()).map(String::from)
}

fn req_str(args: &Value, key: &str) -> Result<String, String> {
    str_opt(args, key).ok_or_else(|| format!("missing required argument: {key}"))
}

fn str_array(args: &Value, key: &str) -> Vec<String> {
    args.get(key)
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Settings, TenantConfig, Transport};
    use crate::registry::Registry;

    /// A tenant pointed at a port nothing listens on: every check that must
    /// happen *before* the network is reached is observable, and every check
    /// that needs the network fails closed (connection refused) rather than
    /// succeeding unconfined.
    fn tenant(id: &str, sites: &[&str]) -> TenantConfig {
        TenantConfig {
            id: id.into(),
            name: id.into(),
            groups: vec![],
            mgmt_host: Some("http://127.0.0.1:1".into()),
            mgmt_token: Some("tok".into()),
            xdr_host: Some("http://127.0.0.1:1".into()),
            xdr_token: Some("tok".into()),
            site_ids: sites.iter().map(|s| (*s).to_string()).collect(),
            account_ids: vec![],
            unsafe_allow_xdr: false,
            unsafe_allow_raw_get: false,
        }
    }

    fn state(cfg: TenantConfig, allow_actions: bool) -> ServerState {
        let settings = Settings {
            transport: Transport::Stdio,
            bind: "0.0.0.0:0".into(),
            allow_actions,
            tenants: vec![cfg],
        };
        let registry = Registry::build(&settings).unwrap();
        ServerState { settings, registry }
    }

    /// The text of a tools/call result, plus whether it was an error.
    fn outcome(v: &Value) -> (bool, String) {
        (
            v.get("isError").and_then(Value::as_bool).unwrap_or(false),
            v["content"][0]["text"].as_str().unwrap_or_default().to_string(),
        )
    }

    #[tokio::test]
    async fn scoped_tenant_refuses_post_raw() {
        let st = state(tenant("acme", &["100"]), true);
        let r = call(&st, "s1_post_raw", json!({"path": "/web/api/v2.1/agents/actions/disconnect", "body": {}, "confirm": true})).await;
        let (is_err, text) = outcome(&r);
        assert!(is_err, "{text}");
        assert!(text.contains("s1_post_raw is refused"), "{text}");
    }

    #[tokio::test]
    async fn unscoped_tenant_still_allows_post_raw() {
        let st = state(tenant("acme", &[]), true);
        let r = call(&st, "s1_post_raw", json!({"path": "/web/api/v2.1/x", "body": {}, "confirm": true})).await;
        let (is_err, text) = outcome(&r);
        // It fails (nothing is listening) but NOT because of confinement.
        assert!(is_err);
        assert!(!text.contains("refused"), "unscoped tenant must be unchanged: {text}");
    }

    #[tokio::test]
    async fn scoped_tenant_refuses_raw_get() {
        let st = state(tenant("acme", &["100"]), false);
        let r = call(&st, "s1_get_raw", json!({"path": "/web/api/v2.1/sites/999"})).await;
        let (_, text) = outcome(&r);
        assert!(text.contains("s1_get_raw is disabled"), "{text}");
    }

    #[tokio::test]
    async fn scoped_tenant_refuses_xdr_queries() {
        let st = state(tenant("acme", &["100"]), false);
        let r = call(&st, "s1_power_query", json!({"pql": "dataset = 'endpoint' | limit 1"})).await;
        let (_, text) = outcome(&r);
        assert!(text.contains("XDR / Data Lake tools are disabled"), "{text}");
    }

    #[tokio::test]
    async fn action_moving_a_device_out_of_scope_is_refused() {
        let st = state(tenant("acme", &["100"]), true);
        let r = call(
            &st,
            "s1_move_device_to_site",
            json!({"id": "abc", "site_id": "999", "confirm": true}),
        )
        .await;
        let (is_err, text) = outcome(&r);
        assert!(is_err, "{text}");
        assert!(text.contains("refusing to move device(s) to site 999"), "{text}");
    }

    #[tokio::test]
    async fn action_body_may_not_set_scope() {
        let st = state(tenant("acme", &["100"]), true);
        let r = call(
            &st,
            "s1_isolate_device",
            json!({"id": "abc", "data": {"siteIds": "999"}, "confirm": true}),
        )
        .await;
        let (is_err, text) = outcome(&r);
        assert!(is_err, "{text}");
        assert!(text.contains("sets API scope"), "{text}");
    }

    #[tokio::test]
    async fn action_against_an_unverifiable_target_fails_closed() {
        let st = state(tenant("acme", &["100"]), true);
        // The scoped lookup that would prove `abc` is in scope cannot complete,
        // so the action must be refused rather than attempted.
        let r = call(&st, "s1_isolate_device", json!({"id": "abc", "confirm": true})).await;
        let (is_err, text) = outcome(&r);
        assert!(is_err, "{text}");
        assert!(text.contains("could not verify the agent target(s)"), "{text}");
    }

    #[tokio::test]
    async fn blocklist_at_account_scope_is_refused() {
        let st = state(tenant("acme", &["100"]), true);
        let r = call(
            &st,
            "s1_add_threat_to_blocklist",
            json!({"id": "t1", "data": {"targetScope": "account"}, "confirm": true}),
        )
        .await;
        let (_, text) = outcome(&r);
        assert!(text.contains("refusing targetScope"), "{text}");
    }

    #[tokio::test]
    async fn raw_tools_cannot_be_pointed_at_another_host() {
        let st = state(tenant("acme", &[]), true);
        for bad in ["https://evil.example/x", "//evil.example/x", "web/api/v2.1/agents"] {
            let r = call(&st, "s1_get_raw", json!({ "path": bad })).await;
            let (is_err, text) = outcome(&r);
            assert!(is_err && text.contains("invalid API path"), "{bad}: {text}");
            let r = call(&st, "s1_post_raw", json!({ "path": bad, "confirm": true })).await;
            let (is_err, text) = outcome(&r);
            assert!(is_err && text.contains("invalid API path"), "{bad}: {text}");
        }
    }

    #[tokio::test]
    async fn action_path_segment_cannot_add_segments() {
        let st = state(tenant("acme", &[]), true);
        let r = call(
            &st,
            "s1_mitigate_threat",
            json!({"id": "t1", "action": "../../../web/api/v2.1/users", "confirm": true}),
        )
        .await;
        let (is_err, text) = outcome(&r);
        assert!(is_err, "{text}");
        assert!(text.contains("invalid path segment"), "{text}");
    }

    #[tokio::test]
    async fn list_tenants_reports_the_confinement() {
        let st = state(tenant("acme", &["100", "200"]), false);
        let r = call(&st, "s1_list_tenants", json!({})).await;
        let (_, text) = outcome(&r);
        let v: Value = serde_json::from_str(&text).unwrap();
        let sc = &v["tenants"][0]["scope"];
        assert_eq!(sc["confined"], json!(true));
        assert_eq!(sc["site_ids"], json!(["100", "200"]));
        assert!(sc["summary"].as_str().unwrap().contains("2 site(s)"));
        assert!(sc["raw_post"].as_str().unwrap().starts_with("refused"));
    }

    #[tokio::test]
    async fn unscoped_tenant_reports_no_confinement() {
        let st = state(tenant("acme", &[]), false);
        let r = call(&st, "s1_ping", json!({})).await;
        let (_, text) = outcome(&r);
        let v: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["results"][0]["scope"]["confined"], json!(false));
        assert_eq!(v["results"][0]["scope"]["raw_post"], json!("allowed"));
    }
}
