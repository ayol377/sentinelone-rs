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

use futures::future::join_all;
use serde_json::{json, Map, Value};

use sentinelone::xdr_api::{FacetQueryRequest, TimeseriesQuerySpec};

use crate::mcp::ServerState;
use crate::registry::Tenant;

const TENANT_HELP: &str =
    "Tenant selector: id, name, group label, or \"*\" for all (fan-out). Omit for the default.";

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
}

const ACTION_TOOLS: &[ActionTool] = &[
    ActionTool { name: "s1_isolate_device", desc: "Network-isolate (disconnect) agent(s).", path: "/web/api/v2.1/agents/actions/disconnect", data: &[], path_param: None },
    ActionTool { name: "s1_reconnect_device", desc: "Reconnect previously isolated agent(s).", path: "/web/api/v2.1/agents/actions/connect", data: &[], path_param: None },
    ActionTool { name: "s1_scan_device", desc: "Start a full disk scan on agent(s).", path: "/web/api/v2.1/agents/actions/initiate-scan", data: &[], path_param: None },
    ActionTool { name: "s1_reboot_device", desc: "Reboot agent machine(s).", path: "/web/api/v2.1/agents/actions/restart-machine", data: &[], path_param: None },
    ActionTool { name: "s1_move_device_to_site", desc: "Move agent(s) to another site.", path: "/web/api/v2.1/agents/actions/move-to-site", data: &[("site_id", "targetSiteId", "Destination site id")], path_param: None },
    ActionTool { name: "s1_mitigate_threat", desc: "Mitigate threat(s) with the given action.", path: "/web/api/v2.1/threats/mitigate/{action}", data: &[], path_param: Some(("action", "One of: kill, quarantine, un-quarantine, remediate, rollback-remediation, network-quarantine")) },
    ActionTool { name: "s1_add_threat_note", desc: "Add a note to threat(s).", path: "/web/api/v2.1/threats/notes", data: &[("text", "text", "Note text")], path_param: None },
    ActionTool { name: "s1_set_threat_verdict", desc: "Set analyst verdict on threat(s).", path: "/web/api/v2.1/threats/analyst-verdict", data: &[("verdict", "analystVerdict", "true_positive | false_positive | suspicious | undefined")], path_param: None },
    ActionTool { name: "s1_set_threat_status", desc: "Set incident status on threat(s).", path: "/web/api/v2.1/threats/incident", data: &[("status", "incidentStatus", "unresolved | in_progress | resolved")], path_param: None },
    ActionTool { name: "s1_add_threat_to_blocklist", desc: "Add threat(s) hash to the blocklist.", path: "/web/api/v2.1/threats/add-to-blacklist", data: &[], path_param: None },
    ActionTool { name: "s1_run_remote_script", desc: "Run a RemoteOps script on agent(s). Provide script parameters via `data` (e.g. scriptId, taskDescription, outputDestination). Poll results with s1_remote_script_status.", path: "/web/api/v2.1/remote-scripts/execute", data: &[("script_id", "scriptId", "Id of the script to run")], path_param: None },
];

// ---- definitions ---------------------------------------------------------

/// JSON-Schema tool definitions returned by `tools/list`.
pub fn definitions() -> Vec<Value> {
    let from_p = json!({ "type": "string", "description": "Window start (DataSet time: \"24 hours\", RFC3339, epoch ns). Default \"24 hours\"." });
    let to_p = json!({ "type": "string", "description": "Window end. Default \"now\"." });
    let tenant_p = json!({ "type": "string", "description": TENANT_HELP });

    let mut tools = vec![
        json!({ "name": "s1_list_tenants", "description": "List configured consoles and which backends (mgmt/xdr) each has.",
                "inputSchema": { "type": "object", "properties": {}, "additionalProperties": false } }),
        json!({ "name": "s1_ping", "description": "Health check: for the selected tenant(s), report configured backends.",
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
        json!({ "name": "s1_power_query", "description": "Run a PowerQuery (PQL) against the XDR / Data Lake. Returns columns + rows. Fan-out capable.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "pql": { "type": "string", "description": "PowerQuery text." }, "from": from_p, "to": to_p }, "required": ["pql"], "additionalProperties": false } }),
        json!({ "name": "s1_query", "description": "Run a DataSet log/event query (filter expression). Returns matching events.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "filter": { "type": "string", "description": "DataSet filter expression. Empty = all events." }, "from": from_p, "to": to_p, "limit": { "type": "integer", "description": "Max events." } }, "additionalProperties": false } }),
        json!({ "name": "s1_facet", "description": "Top values of a field over XDR events (group-by + count). Fan-out capable.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "filter": { "type": "string", "description": "DataSet filter. Empty = all." }, "field": { "type": "string", "description": "Field to facet." }, "from": from_p, "to": to_p, "max_count": { "type": "integer" } }, "required": ["field"], "additionalProperties": false } }),
        json!({ "name": "s1_numeric", "description": "Numeric aggregation over XDR events (count, mean:field, p90:field, …), optionally bucketed.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "filter": { "type": "string" }, "function": { "type": "string", "description": "e.g. count, mean:responseTime, p95:latency" }, "from": from_p, "to": to_p, "buckets": { "type": "integer" } }, "required": ["function"], "additionalProperties": false } }),
        json!({ "name": "s1_timeseries", "description": "Time-bucketed counts/aggregation over XDR events (trend / spike detection).",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "filter": { "type": "string" }, "function": { "type": "string", "description": "e.g. count, mean:field" }, "from": from_p, "to": to_p, "buckets": { "type": "integer", "description": "Number of time buckets. Default 24." } }, "required": ["function"], "additionalProperties": false } }),
        // ---- raw escape hatches ----
        json!({ "name": "s1_get_raw", "description": "Escape hatch: raw GET to ANY Management API path (reach endpoints with no curated tool). Read-only.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "path": { "type": "string", "description": "API path, e.g. /web/api/v2.1/ranger/tables" }, "query": { "type": "string", "description": "Pre-serialized querystring, e.g. limit=10&osTypes=windows" } }, "required": ["path"], "additionalProperties": false } }),
        json!({ "name": "s1_post_raw", "description": "[ACTION] Escape hatch: raw POST of a JSON body to ANY Management API path. Gated; single tenant; confirm required.",
                "inputSchema": { "type": "object", "properties": { "tenant": tenant_p, "path": { "type": "string" }, "body": { "type": "object", "additionalProperties": true, "description": "Raw JSON request body." }, "confirm": { "type": "boolean" } }, "required": ["path"], "additionalProperties": false } }),
    ];

    for lt in LIST_TOOLS {
        tools.push(json!({ "name": lt.name, "description": lt.desc, "inputSchema": list_schema(lt.params) }));
    }
    for at in ACTION_TOOLS {
        tools.push(json!({ "name": at.name, "description": format!("[ACTION] {} Requires MCP_ALLOW_ACTIONS=true, a single tenant, and \"confirm\": true.", at.desc), "inputSchema": action_schema(at) }));
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
        "filters": { "type": "object", "additionalProperties": true, "description": "Any additional API query params by exact wire name, e.g. {\"osTypes\":\"windows\"}." }
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
        .map(|t| json!({ "id": t.cfg.id, "name": t.cfg.name, "groups": t.cfg.groups, "backends": backends(t) }))
        .collect();
    Ok(json!({ "actions_enabled": state.settings.allow_actions, "tenants": tenants }))
}

fn ping(state: &ServerState, args: &Value) -> Result<Value, String> {
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let results: Vec<Value> = tenants
        .iter()
        .map(|t| json!({ "tenant": t.cfg.id, "ok": true, "backends": backends(t) }))
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
    let qs = querystring(vec![("ids".into(), id)]);
    let qsr = qs.as_deref();
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_get_json(t, "/web/api/v2.1/threats", qsr).await)
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
    let qs = querystring(pairs);
    let qsr = qs.as_deref();
    let path = format!("/web/api/v2.1/threats/{id}/{suffix}");
    let path = path.as_str();
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_get_json(t, path, qsr).await)
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
    let qs = querystring(pairs);
    let qsr = qs.as_deref();
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_get_json(t, path, qsr).await)
    }))
    .await;
    Ok(fanout(res))
}

async fn get_raw(state: &ServerState, args: &Value) -> Result<Value, String> {
    let path = req_str(args, "path")?;
    let tenants = state.registry.resolve(str_opt(args, "tenant").as_deref())?;
    let query = str_opt(args, "query");
    let (path, query) = (path.as_str(), query.as_deref());
    let res = join_all(tenants.into_iter().map(|t| async move {
        (t.cfg.id.clone(), do_get_json(t, path, query).await)
    }))
    .await;
    Ok(fanout(res))
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
        Some((arg, _)) => act.path.replace("{action}", &req_str(args, arg)?),
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
    let path = req_str(args, "path")?;
    let body = args.get("body").cloned().unwrap_or_else(|| json!({}));
    let mgmt = t.client.management().map_err(|e| e.to_string())?;
    let resp = mgmt.post_json(&path, &body).await.map_err(|e| e.to_string())?;
    Ok(json!({ "tenant": t.cfg.id, "path": path, "result": resp }))
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

async fn do_get_device(t: &Tenant, id: &str) -> Result<Value, String> {
    let agent = t.client.agent(id).await.map_err(|e| e.to_string())?;
    serde_json::to_value(agent.data()).map_err(|e| e.to_string())
}

async fn do_get_json(t: &Tenant, path: &str, qs: Option<&str>) -> Result<Value, String> {
    let mgmt = t.client.management().map_err(|e| e.to_string())?;
    mgmt.get_json(path, qs).await.map_err(|e| e.to_string())
}

async fn do_power_query(t: &Tenant, pql: &str, from: &str, to: &str) -> Result<Value, String> {
    let resp = t.client.power_query(pql).from(from).to(to).run().await.map_err(|e| e.to_string())?;
    Ok(json!({ "columns": resp.columns, "values": resp.values, "matches": resp.matches }))
}

async fn do_log_query(t: &Tenant, filter: &str, from: &str, to: &str) -> Result<Value, String> {
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
    let xdr = t.client.xdr().map_err(|e| e.to_string())?;
    let resp = xdr.numeric_query(filter, function, from, to).await.map_err(|e| e.to_string())?;
    Ok(json!({ "values": resp.values, "warnings": resp.warnings }))
}

async fn do_timeseries(t: &Tenant, filter: &str, function: &str, from: &str, to: &str, buckets: i64) -> Result<Value, String> {
    let xdr = t.client.xdr().map_err(|e| e.to_string())?;
    let spec = TimeseriesQuerySpec::new(filter, function, from, to).buckets(buckets);
    let resp = xdr.timeseries_query(vec![spec]).await.map_err(|e| e.to_string())?;
    let series: Vec<Value> = resp.results.iter().map(|r| json!({ "values": r.values })).collect();
    Ok(json!({ "results": series }))
}

// ---- helpers -------------------------------------------------------------

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

fn window(args: &Value) -> (String, String) {
    (
        str_opt(args, "from").unwrap_or_else(|| "24 hours".into()),
        str_opt(args, "to").unwrap_or_else(|| "now".into()),
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

fn querystring(pairs: Vec<(String, String)>) -> Option<String> {
    if pairs.is_empty() {
        return None;
    }
    serde_urlencoded::to_string(&pairs).ok().filter(|s| !s.is_empty())
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
