//! Minimal MCP (JSON-RPC 2.0) protocol layer. Transport-agnostic: both the
//! stdio and http transports decode a JSON value, call [`handle_message`], and
//! ship back the returned value (or nothing, for notifications).
//!
//! Implemented methods: `initialize`, `tools/list`, `tools/call`, `ping`, and
//! `notifications/*` (ignored). This is a deliberately small, dependency-free
//! implementation of the parts of MCP this server needs.

use serde_json::{json, Value};

use crate::config::Settings;
use crate::registry::Registry;
use crate::tools;

/// MCP protocol revision this server speaks.
pub const PROTOCOL_VERSION: &str = "2025-06-18";
pub const SERVER_NAME: &str = "sentinelone-mcp";
pub const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Shared, immutable server state handed to every request.
pub struct ServerState {
    pub settings: Settings,
    pub registry: Registry,
}

/// JSON-RPC error codes used here.
pub mod code {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
}

fn ok(id: Value, result: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "result": result })
}

fn err(id: Value, code: i32, message: impl Into<String>) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message.into() } })
}

/// Handle a single decoded JSON-RPC message. Returns `Some(response)` for
/// requests, `None` for notifications (no `id`).
pub async fn handle_message(state: &ServerState, msg: Value) -> Option<Value> {
    let obj = msg.as_object()?;
    let id = obj.get("id").cloned();
    let method = obj.get("method").and_then(Value::as_str).unwrap_or("");
    let params = obj.get("params").cloned().unwrap_or(Value::Null);

    // Notifications have no `id`; never produce a response.
    let is_notification = id.is_none();
    let id = id.unwrap_or(Value::Null);

    if is_notification {
        // e.g. notifications/initialized — nothing to do.
        return None;
    }

    let response = match method {
        "initialize" => ok(id, initialize_result()),
        "ping" => ok(id, json!({})),
        "tools/list" => ok(id, json!({ "tools": tools::definitions() })),
        "tools/call" => match params.as_object() {
            Some(p) => {
                let name = p.get("name").and_then(Value::as_str).unwrap_or_default().to_string();
                let args = p.get("arguments").cloned().unwrap_or(json!({}));
                let result = tools::call(state, &name, args).await;
                ok(id, result)
            }
            None => err(id, code::INVALID_PARAMS, "tools/call requires params object"),
        },
        "" => err(id, code::INVALID_REQUEST, "missing method"),
        other => err(id, code::METHOD_NOT_FOUND, format!("unknown method: {other}")),
    };
    Some(response)
}

/// Process a decoded JSON value that may be a single request or a batch array.
/// Returns the response value (or batch), or `None` if there is nothing to send
/// (all notifications / empty batch).
pub async fn process(state: &ServerState, v: Value) -> Option<Value> {
    if let Some(arr) = v.as_array() {
        let mut out = Vec::new();
        for item in arr {
            if let Some(r) = Box::pin(handle_message(state, item.clone())).await {
                out.push(r);
            }
        }
        (!out.is_empty()).then_some(Value::Array(out))
    } else {
        handle_message(state, v).await
    }
}

/// A JSON-RPC parse-error response (id null), for transports to emit on
/// undecodable input.
pub fn parse_error() -> Value {
    err(Value::Null, code::PARSE_ERROR, "parse error")
}

fn initialize_result() -> Value {
    json!({
        "protocolVersion": PROTOCOL_VERSION,
        "capabilities": { "tools": { "listChanged": false } },
        "serverInfo": { "name": SERVER_NAME, "version": SERVER_VERSION }
    })
}
