//! Tenant registry: one [`SentinelOne`] facade per configured console, plus the
//! `tenant` selector resolution that powers cross-tenant fan-out (§2.5 of the
//! crate DESIGN).

use sentinelone::SentinelOne;

use crate::config::{Settings, TenantConfig};

/// A configured console: its static config + a built facade client.
pub struct Tenant {
    pub cfg: TenantConfig,
    pub client: SentinelOne,
}

impl Tenant {
    pub fn has_mgmt(&self) -> bool {
        self.cfg.mgmt_host.is_some()
    }
    pub fn has_xdr(&self) -> bool {
        self.cfg.xdr_host.is_some()
    }
}

/// All tenants, keyed for selection by id / name / group / `"*"`.
pub struct Registry {
    tenants: Vec<Tenant>,
    default: Option<String>,
}

impl Registry {
    /// Build a facade per tenant from settings.
    pub fn build(settings: &Settings) -> Result<Self, String> {
        let mut tenants = Vec::with_capacity(settings.tenants.len());
        for cfg in &settings.tenants {
            let mut builder = SentinelOne::builder();
            if let (Some(h), Some(t)) = (&cfg.mgmt_host, &cfg.mgmt_token) {
                builder = builder.management(h, t);
            }
            if let (Some(h), Some(t)) = (&cfg.xdr_host, &cfg.xdr_token) {
                builder = builder.xdr(h, t);
            }
            let client = builder
                .build()
                .map_err(|e| format!("tenant {}: {e}", cfg.id))?;
            tenants.push(Tenant { cfg: cfg.clone(), client });
        }

        // Default: explicit, or the sole tenant when there is exactly one.
        let default = settings.default_tenant.clone().or_else(|| {
            if tenants.len() == 1 {
                Some(tenants[0].cfg.id.clone())
            } else {
                None
            }
        });

        Ok(Registry { tenants, default })
    }

    pub fn all(&self) -> &[Tenant] {
        &self.tenants
    }

    fn get(&self, id: &str) -> Option<&Tenant> {
        self.tenants.iter().find(|t| t.cfg.id == id)
    }

    /// Resolve a `tenant` selector to one or more tenants.
    ///
    /// - `None` → the session/config default (error if ambiguous);
    /// - `"*"` / `"all"` → every tenant (fan-out);
    /// - an exact id or name → that one;
    /// - otherwise, every tenant carrying the value as a group label (fan-out).
    pub fn resolve(&self, selector: Option<&str>) -> Result<Vec<&Tenant>, String> {
        match selector.map(str::trim) {
            None | Some("") => {
                let def = self.default.as_deref().ok_or_else(|| {
                    "multiple tenants configured; specify `tenant` (id, name, group, or \"*\") \
                     or set S1_DEFAULT_TENANT"
                        .to_string()
                })?;
                let t = self
                    .get(def)
                    .ok_or_else(|| format!("default tenant {def} not found"))?;
                Ok(vec![t])
            }
            Some("*") | Some("all") => {
                if self.tenants.is_empty() {
                    return Err("no tenants configured".into());
                }
                Ok(self.tenants.iter().collect())
            }
            Some(sel) => {
                // exact id or name
                if let Some(t) = self.tenants.iter().find(|t| t.cfg.id == sel || t.cfg.name == sel) {
                    return Ok(vec![t]);
                }
                // group label
                let group: Vec<&Tenant> = self
                    .tenants
                    .iter()
                    .filter(|t| t.cfg.groups.iter().any(|g| g == sel))
                    .collect();
                if group.is_empty() {
                    Err(format!("no tenant matches `{sel}` (not an id, name, or group)"))
                } else {
                    Ok(group)
                }
            }
        }
    }
}
