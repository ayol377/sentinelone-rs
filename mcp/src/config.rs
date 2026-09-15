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
//!
//! Per tenant `<ID>` (id upper-cased, non-alphanumerics → `_`):
//! - `S1_TENANT_<ID>_NAME`        (optional, display name)
//! - `S1_TENANT_<ID>_GROUPS`      (optional, comma-separated labels for fan-out)
//! - `S1_TENANT_<ID>_MGMT_HOST`   + `S1_TENANT_<ID>_MGMT_TOKEN`
//! - `S1_TENANT_<ID>_XDR_HOST`    + `S1_TENANT_<ID>_XDR_TOKEN`
//!
//! A tenant may configure only mgmt, only xdr, or both.
//!
//! ## Confinement (MSSP tenancy)
//!
//! The Management API token is typically *account*-scoped: it can read every
//! site in the account, so it cannot by itself stop one customer's investigation
//! from reading another's data. These variables confine a tenant server-side —
//! see [`crate::scope`]:
//!
//! - `S1_TENANT_<ID>_SITE_IDS`    (comma-separated site ids; empty = unrestricted)
//! - `S1_TENANT_<ID>_ACCOUNT_IDS` (comma-separated account ids; for customers
//!   who own a whole account rather than a site)
//! - `S1_TENANT_<ID>_UNSAFE_ALLOW_XDR`      (opt back in to XDR/Data Lake tools
//!   for a confined tenant — PQL/DataSet queries cannot be confined here)
//! - `S1_TENANT_<ID>_UNSAFE_ALLOW_RAW_GET`  (opt back in to `s1_get_raw` for a
//!   confined tenant — best-effort scope injection only)

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
    /// Site ids this tenant may see. Empty = unrestricted (account-wide).
    pub site_ids: Vec<String>,
    /// Account ids this tenant may see. Empty = unrestricted.
    pub account_ids: Vec<String>,
    /// Allow XDR / Data Lake tools even though this tenant is confined.
    pub unsafe_allow_xdr: bool,
    /// Allow `s1_get_raw` even though this tenant is confined.
    pub unsafe_allow_raw_get: bool,
}

impl TenantConfig {
    /// True when this tenant is confined to a subset of the console.
    pub fn is_scoped(&self) -> bool {
        !self.site_ids.is_empty() || !self.account_ids.is_empty()
    }
}

/// Full server settings.
#[derive(Debug, Clone)]
pub struct Settings {
    pub transport: Transport,
    pub bind: String,
    pub allow_actions: bool,
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
            let groups = csv_var(&key, "GROUPS");

            let site_ids = csv_var(&key, "SITE_IDS");
            let account_ids = csv_var(&key, "ACCOUNT_IDS");
            for v in site_ids.iter().chain(account_ids.iter()) {
                if !v.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
                    return Err(format!(
                        "tenant {id}: invalid scope id {v:?} (expected an id like 225494730938493804)"
                    ));
                }
            }
            let unsafe_allow_xdr = env_bool(&format!("S1_TENANT_{key}_UNSAFE_ALLOW_XDR"));
            let unsafe_allow_raw_get = env_bool(&format!("S1_TENANT_{key}_UNSAFE_ALLOW_RAW_GET"));

            tenants.push(TenantConfig {
                id,
                name,
                groups,
                mgmt_host,
                mgmt_token,
                xdr_host,
                xdr_token,
                site_ids,
                account_ids,
                unsafe_allow_xdr,
                unsafe_allow_raw_get,
            });
        }

        Ok(Settings { transport, bind, allow_actions, tenants })
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

/// A comma-separated tenant variable, trimmed, empties dropped.
fn csv_var(key: &str, suffix: &str) -> Vec<String> {
    tenant_var(key, suffix)
        .unwrap_or_default()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
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
