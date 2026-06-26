//! `sentinelone-mcp` — a multi-tenant MCP server exposing SentinelOne (mgmt +
//! XDR) to LLM analyst agents, built on the high-level `sentinelone` facade.
//!
//! Configured entirely from the environment (`.env` in dev, injected env vars in
//! Docker) — see [`config`]. Run with `MCP_TRANSPORT=http` for a containerized
//! service, or the default `stdio` for desktop/IDE MCP clients.

mod config;
mod http;
mod mcp;
mod registry;
mod stdio;
mod tools;

use std::sync::Arc;

use crate::config::{Settings, Transport};
use crate::mcp::ServerState;
use crate::registry::Registry;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("sentinelone-mcp fatal: {e}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), String> {
    let settings = Settings::from_env()?;
    let registry = Registry::build(&settings)?;
    eprintln!(
        "sentinelone-mcp v{}: {} tenant(s) loaded; actions {}",
        env!("CARGO_PKG_VERSION"),
        registry.all().len(),
        if settings.allow_actions { "ENABLED" } else { "disabled" },
    );

    let transport = settings.transport;
    let bind = settings.bind.clone();
    let state = Arc::new(ServerState { settings, registry });

    match transport {
        Transport::Stdio => stdio::run(state).await.map_err(|e| e.to_string()),
        Transport::Http => http::run(state, &bind).await.map_err(|e| e.to_string()),
    }
}
