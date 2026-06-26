//! Configuration loaded entirely from the environment (`.env` in dev, injected
//! env vars in Docker). Secrets — the per-tenant API tokens — live here too:
//! this server is deployed as a service and reads them from env, not from an OS
//! keychain. (Keychain storage is reserved for a future interactive CLI.)
//!
//! ## Env schema
//!
//! Server:
//! - `MCP_TRANSPORT` = `stdio` (default) | `http`
//! - `MCP_BIND`      = bind address for http transport (default `0.0.0.0:8080`)
//! - `MCP_ALLOW_ACTIONS` = `true` to enable destructive tools (default `false`)
//!
//! Tenants:
//! - `S1_TENANTS` = comma-separated tenant ids, e.g. `acme,globex`
//! - `S1_DEFAULT_TENANT` = id used when a tool omits `tenant` (optional if only one)
//!
//! Per tenant `<ID>` (id upper-cased, non-alphanumerics → `_`):
//! - `S1_TENANT_<ID>_NAME`        (optional, display name)
//! - `S1_TENANT_<ID>_GROUPS`      (optional, comma-separated labels for fan-out)
//! - `S1_TENANT_<ID>_MGMT_HOST`   + `S1_TENANT_<ID>_MGMT_TOKEN`
//! - `S1_TENANT_<ID>_XDR_HOST`    + `S1_TENANT_<ID>_XDR_TOKEN`
//!
//! A tenant may configure only mgmt, only xdr, or both.

use std::env;

/// Transport the server listens on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    Stdio,
    Http,
}

/// Per-tenant configuration (one SentinelOne console).
#[derive(Debug, Clone)]
pub struct TenantConfig {
    pub id: String,
    pub name: String,
    pub groups: Vec<String>,
    pub mgmt_host: Option<String>,
    pub mgmt_token: Option<String>,
    pub xdr_host: Option<String>,
    pub xdr_token: Option<String>,
}

/// Full server settings.
#[derive(Debug, Clone)]
pub struct Settings {
    pub transport: Transport,
    pub bind: String,
    pub allow_actions: bool,
    pub default_tenant: Option<String>,
    pub tenants: Vec<TenantConfig>,
}

impl Settings {
    /// Build settings from the process environment. Loads a `.env` file first if
    /// present (dev convenience; Docker injects env directly).
    pub fn from_env() -> Result<Self, String> {
        load_dotenv_if_present();

        let transport = match env::var("MCP_TRANSPORT").unwrap_or_else(|_| "stdio".into()).as_str() {
            "http" => Transport::Http,
            "stdio" | "" => Transport::Stdio,
            other => return Err(format!("invalid MCP_TRANSPORT: {other} (want stdio|http)")),
        };
        let bind = env::var("MCP_BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
        let allow_actions = env_bool("MCP_ALLOW_ACTIONS");
        let default_tenant = env::var("S1_DEFAULT_TENANT").ok().filter(|s| !s.is_empty());

        let ids: Vec<String> = env::var("S1_TENANTS")
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        if ids.is_empty() {
            return Err("no tenants configured: set S1_TENANTS (e.g. \"acme,globex\")".into());
        }

        let mut tenants = Vec::new();
        for id in ids {
            let key = env_key(&id);
            let mgmt_host = tenant_var(&key, "MGMT_HOST");
            let mgmt_token = tenant_var(&key, "MGMT_TOKEN");
            let xdr_host = tenant_var(&key, "XDR_HOST");
            let xdr_token = tenant_var(&key, "XDR_TOKEN");

            if mgmt_host.is_some() != mgmt_token.is_some() {
                return Err(format!("tenant {id}: MGMT_HOST and MGMT_TOKEN must be set together"));
            }
            if xdr_host.is_some() != xdr_token.is_some() {
                return Err(format!("tenant {id}: XDR_HOST and XDR_TOKEN must be set together"));
            }
            if mgmt_host.is_none() && xdr_host.is_none() {
                return Err(format!("tenant {id}: configure at least one of mgmt or xdr"));
            }

            let name = tenant_var(&key, "NAME").unwrap_or_else(|| id.clone());
            let groups = tenant_var(&key, "GROUPS")
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            tenants.push(TenantConfig {
                id,
                name,
                groups,
                mgmt_host,
                mgmt_token,
                xdr_host,
                xdr_token,
            });
        }

        if let Some(def) = &default_tenant {
            if !tenants.iter().any(|t| &t.id == def) {
                return Err(format!("S1_DEFAULT_TENANT={def} is not in S1_TENANTS"));
            }
        }

        Ok(Settings { transport, bind, allow_actions, default_tenant, tenants })
    }
}

fn env_key(id: &str) -> String {
    id.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_uppercase() } else { '_' })
        .collect()
}

fn tenant_var(key: &str, suffix: &str) -> Option<String> {
    env::var(format!("S1_TENANT_{key}_{suffix}")).ok().filter(|s| !s.is_empty())
}

fn env_bool(name: &str) -> bool {
    matches!(
        env::var(name).unwrap_or_default().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

/// Minimal `.env` loader (no external dependency): `KEY=VALUE` lines, `#`
/// comments, optional surrounding quotes. Does not override variables already
/// present in the environment.
fn load_dotenv_if_present() {
    let path = env::var("MCP_ENV_FILE").unwrap_or_else(|_| ".env".into());
    let Ok(contents) = std::fs::read_to_string(&path) else {
        return;
    };
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line.strip_prefix("export ").unwrap_or(line);
        let Some((k, v)) = line.split_once('=') else { continue };
        let k = k.trim();
        let mut v = v.trim();
        if (v.starts_with('"') && v.ends_with('"') && v.len() >= 2)
            || (v.starts_with('\'') && v.ends_with('\'') && v.len() >= 2)
        {
            v = &v[1..v.len() - 1];
        }
        if env::var(k).is_err() {
            // SAFETY: single-threaded startup, before any tenant clients/tasks spawn.
            unsafe { env::set_var(k, v) };
        }
    }
}
