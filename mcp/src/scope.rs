//! Server-side tenant confinement.
//!
//! ## Threat model
//!
//! In an MSSP deployment one SentinelOne *account* holds many customers, one
//! **site** each, and the service-user API token reports `scope: "account"` —
//! it can read every site. The token therefore cannot confine a tenant. Nor can
//! the calling application: anything it injects into tool arguments is
//! client-side and one bug (or one creative LLM) away from being bypassed.
//!
//! This module is the server-side answer. A tenant configured with
//! `S1_TENANT_<ID>_SITE_IDS` is *confined*: every Management API request built
//! by this server has caller-supplied scope parameters removed and the allowed
//! scope appended, so a request for another customer's data cannot be
//! constructed at all.
//!
//! ## Chokepoint
//!
//! [`apply`] is the single place a Management querystring is produced. The
//! plain `querystring` serializer is private to this module, so no tool handler
//! can build a query that skipped confinement — the type system routes every
//! Management `GET` through here. Requests whose target is named in the *path*
//! rather than the query (a threat id, an agent id for an action) cannot be
//! confined by a querystring at all; those are checked separately by the
//! caller-side helpers in `tools.rs`, which resolve the id through a confined
//! read first.

use crate::config::TenantConfig;

/// What confinement is possible for a given Management API path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathPolicy {
    /// The endpoint honours `siteIds` / `accountIds`; append the allowed scope.
    QueryScoped,
    /// The endpoint exposes no per-customer data (e.g. an enum of activity
    /// type ids); safe to serve unconfined.
    NoCustomerData,
    /// The endpoint cannot be confined by query parameters; refuse it.
    Unscopeable,
}

/// Classification of a query-parameter name for confinement purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Key {
    Site,
    Account,
    /// Group scoping — cannot be intersected against a site allow-list without
    /// a second lookup, and the API may not AND it with `siteIds`. Dropped.
    Group,
    /// Parameters that ask the API to widen scope (S1's `tenant=true` means
    /// "account-level request").
    Widen,
    Other,
}

/// Canonicalise a parameter name: drop an S1 filter suffix (`__contains`),
/// lower-case, and strip every non-alphanumeric character. This collapses
/// `siteIds`, `site_ids`, `SITE-IDS`, `Site.Ids` and `siteIds__contains` onto
/// one spelling so no casing or separator trick survives.
fn canon(name: &str) -> String {
    let base = name.split("__").next().unwrap_or(name);
    base.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn classify(name: &str) -> Key {
    match canon(name).as_str() {
        "siteids" | "siteid" | "sites" | "site" => Key::Site,
        "accountids" | "accountid" | "accounts" | "account" => Key::Account,
        "groupids" | "groupid" | "groups" | "group" => Key::Group,
        "tenant" | "scope" | "scopelevel" | "globalscope" | "targetscope" => Key::Widen,
        _ => Key::Other,
    }
}

/// True if this parameter name would set or widen scope and must never reach
/// the API from caller input. Used for request *bodies* too (see `tools.rs`).
pub fn is_scope_key(name: &str) -> bool {
    !matches!(classify(name), Key::Other)
}

/// Looser check for request *bodies*, where field names vary more than query
/// params do (`targetSiteId`, `targetSiteIds`, `filterSiteIds`, …): true if the
/// name mentions a site, account or group id at all.
pub fn mentions_scope_id(name: &str) -> bool {
    let c = canon(name);
    is_scope_key(name)
        || c.contains("siteid")
        || c.contains("accountid")
        || c.contains("groupid")
}

/// Split a comma-separated id list (the API's array convention).
fn split_csv(v: &str) -> Vec<String> {
    v.split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Confinement policy for a Management path under this tenant's config.
pub fn policy(cfg: &TenantConfig, path: &str) -> PathPolicy {
    let p = path.trim_end_matches('/');
    // Static enumerations: no customer data, no scope params.
    if p.ends_with("/activities/types") {
        return PathPolicy::NoCustomerData;
    }
    // Polling a PowerQuery by id: scope was fixed (and confined) when the query
    // was created via POST /dv/events/pq; the ping takes only `queryId`.
    if p.ends_with("/dv/events/pq-ping") {
        return PathPolicy::NoCustomerData;
    }
    // /accounts takes `accountIds` but not `siteIds`; a site-scoped tenant
    // cannot confine it, so it is refused unless account ids are configured.
    if p.ends_with("/accounts") || p.contains("/accounts/") {
        return if cfg.account_ids.is_empty() {
            PathPolicy::Unscopeable
        } else {
            PathPolicy::QueryScoped
        };
    }
    PathPolicy::QueryScoped
}

/// Intersect a caller's requested ids with the allowed set.
///
/// No request → the full allowed set. A request → only the allowed members of
/// it. A disjoint request is an error naming the allowed ids, so the caller
/// learns it asked outside its scope instead of silently receiving everything
/// in scope (which would read as "that customer has no such data").
pub(crate) fn intersect(requested: &[String], allowed: &[String], what: &str) -> Result<Vec<String>, String> {
    if requested.is_empty() {
        return Ok(allowed.to_vec());
    }
    // Keep allow-list order and drop duplicates: a caller repeating an allowed
    // id under six spellings must not produce six copies in the query.
    let keep: Vec<String> = allowed
        .iter()
        .filter(|a| requested.iter().any(|r| r == *a))
        .cloned()
        .collect();
    if keep.is_empty() {
        return Err(format!(
            "request is outside this tenant's scope: requested {what} [{}] but this server is \
             confined to {what} [{}]",
            requested.join(", "),
            allowed.join(", "),
        ));
    }
    Ok(keep)
}

/// **The chokepoint.** Turn a tool's parameter pairs into the final Management
/// querystring, enforcing this tenant's confinement.
///
/// For an unconfined tenant the pairs are serialized unchanged (today's
/// behaviour). For a confined tenant:
///
/// 1. every caller-supplied scope parameter is removed — from named tool
///    arguments and from the free-form `filters` splat alike, under any casing
///    or separator spelling;
/// 2. requested site/account ids are intersected with the allowed set, and a
///    disjoint request is an error;
/// 3. the resulting allowed scope is appended.
pub fn apply(
    cfg: &TenantConfig,
    path: &str,
    pairs: Vec<(String, String)>,
) -> Result<Option<String>, String> {
    if !cfg.is_scoped() {
        return Ok(querystring(pairs));
    }

    match policy(cfg, path) {
        PathPolicy::NoCustomerData => {
            // Still strip scope keys: nothing legitimate needs them here.
            let kept: Vec<(String, String)> = pairs
                .into_iter()
                .filter(|(k, _)| !is_scope_key(k))
                .collect();
            return Ok(querystring(kept));
        }
        PathPolicy::Unscopeable => {
            return Err(format!(
                "refusing {path}: this tenant is confined to sites [{}] and that endpoint cannot \
                 be restricted by scope parameters",
                cfg.site_ids.join(", "),
            ));
        }
        PathPolicy::QueryScoped => {}
    }

    let mut kept: Vec<(String, String)> = Vec::with_capacity(pairs.len() + 2);
    let mut req_sites: Vec<String> = Vec::new();
    let mut req_accounts: Vec<String> = Vec::new();

    for (k, v) in pairs {
        match classify(&k) {
            Key::Site => req_sites.extend(split_csv(&v)),
            Key::Account => req_accounts.extend(split_csv(&v)),
            // Dropped outright: group scoping cannot be validated against a
            // site allow-list here, and `tenant=true` widens to the account.
            Key::Group | Key::Widen => {}
            Key::Other => kept.push((k, v)),
        }
    }

    if !cfg.site_ids.is_empty() {
        let eff = intersect(&req_sites, &cfg.site_ids, "sites")?;
        kept.push(("siteIds".into(), eff.join(",")));
    }
    if !cfg.account_ids.is_empty() {
        let eff = intersect(&req_accounts, &cfg.account_ids, "accounts")?;
        kept.push(("accountIds".into(), eff.join(",")));
    }

    Ok(querystring(kept))
}

/// Serialize parameter pairs. **Private on purpose**: every Management query
/// must be produced by [`apply`] so confinement cannot be skipped.
fn querystring(pairs: Vec<(String, String)>) -> Option<String> {
    if pairs.is_empty() {
        return None;
    }
    serde_urlencoded::to_string(&pairs).ok().filter(|s| !s.is_empty())
}

/// Ids the caller asked to act on that a confined read did not return — i.e.
/// ids that are not in this tenant's scope (or do not exist).
pub fn unresolved_ids(requested: &[String], found: &[String]) -> Vec<String> {
    requested
        .iter()
        .filter(|r| !found.iter().any(|f| f == *r))
        .cloned()
        .collect()
}

/// A one-line summary of a tenant's confinement, for logs and `s1_list_tenants`.
pub fn describe(cfg: &TenantConfig) -> String {
    if !cfg.is_scoped() {
        return "unrestricted (account-wide)".into();
    }
    let mut parts = Vec::new();
    if !cfg.site_ids.is_empty() {
        parts.push(format!("{} site(s)", cfg.site_ids.len()));
    }
    if !cfg.account_ids.is_empty() {
        parts.push(format!("{} account(s)", cfg.account_ids.len()));
    }
    format!("confined to {}", parts.join(" + "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(site_ids: &[&str], account_ids: &[&str]) -> TenantConfig {
        TenantConfig {
            id: "t".into(),
            name: "t".into(),
            groups: vec![],
            mgmt_host: Some("https://example.invalid".into()),
            mgmt_token: Some("tok".into()),
            xdr_host: None,
            xdr_token: None,
            site_ids: site_ids.iter().map(|s| (*s).to_string()).collect(),
            account_ids: account_ids.iter().map(|s| (*s).to_string()).collect(),
            unsafe_allow_xdr: false,
            unsafe_allow_raw_get: false,
        }
    }

    fn pairs(kv: &[(&str, &str)]) -> Vec<(String, String)> {
        kv.iter().map(|(k, v)| ((*k).to_string(), (*v).to_string())).collect()
    }

    const AGENTS: &str = "/web/api/v2.1/agents";

    #[test]
    fn unscoped_tenant_is_unchanged() {
        let c = cfg(&[], &[]);
        let qs = apply(&c, AGENTS, pairs(&[("siteIds", "999"), ("limit", "50")])).unwrap();
        assert_eq!(qs.as_deref(), Some("siteIds=999&limit=50"));
    }

    #[test]
    fn scoped_tenant_replaces_caller_site_ids() {
        let c = cfg(&["100", "200"], &[]);
        // Caller asks for a site it may not see -> disjoint -> refused.
        let err = apply(&c, AGENTS, pairs(&[("siteIds", "999")])).unwrap_err();
        assert!(err.contains("outside this tenant's scope"), "{err}");
        assert!(err.contains("100") && err.contains("200"), "error must name allowed sites: {err}");

        // Caller asks for nothing -> full allowed scope is appended.
        let qs = apply(&c, AGENTS, pairs(&[("limit", "10")])).unwrap().unwrap();
        assert_eq!(qs, "limit=10&siteIds=100%2C200");

        // Caller asks for one allowed site -> narrowed to it, never widened.
        let qs = apply(&c, AGENTS, pairs(&[("siteIds", "100")])).unwrap().unwrap();
        assert_eq!(qs, "siteIds=100");

        // A mixed request keeps only the allowed member.
        let qs = apply(&c, AGENTS, pairs(&[("siteIds", "100,999")])).unwrap().unwrap();
        assert_eq!(qs, "siteIds=100");
    }

    #[test]
    fn filters_splat_cannot_widen_scope_under_any_spelling() {
        let c = cfg(&["100"], &[]);
        // Everything below arrives via `merge_filters` in a real call: the
        // `filters` object copies arbitrary keys straight into the query.
        // Every spelling of a scope key is recognised and removed; what is
        // left is only the caller's non-scope filter plus our own scope.
        let qs = apply(
            &c,
            AGENTS,
            pairs(&[
                ("SITEIDS", "100"),
                ("site_ids", "100"),
                ("Site-Ids", "100"),
                ("site.ids", "100"),
                ("siteId", "100"),
                ("siteIds__contains", "100"),
                ("accountIds", "555"),
                ("groupIds", "777"),
                ("tenant", "true"),
                ("scope", "account"),
                ("osTypes", "windows"),
            ]),
        )
        .unwrap()
        .unwrap();
        assert_eq!(qs, "osTypes=windows&siteIds=100");
        assert!(!qs.contains("555"), "accountIds must not survive: {qs}");
        assert!(!qs.contains("777"), "groupIds must not survive: {qs}");
        assert!(!qs.contains("tenant"), "tenant=true widens to the account: {qs}");

        // ...and a `filters` object naming somebody else's site cannot widen
        // scope by any spelling either — it is refused outright.
        for k in ["siteIds", "SITEIDS", "site_ids", "Site-Ids", "site.ids", "siteId", "siteIds__contains"] {
            let err = apply(&c, AGENTS, pairs(&[(k, "999")])).unwrap_err();
            assert!(err.contains("outside this tenant's scope"), "{k}: {err}");
        }
    }

    #[test]
    fn disjoint_request_errors_rather_than_returning_everything() {
        let c = cfg(&["100", "200"], &[]);
        let err = apply(&c, AGENTS, pairs(&[("siteIds", "300,400")])).unwrap_err();
        assert!(err.contains("300"), "{err}");
        assert!(err.contains("100, 200"), "{err}");
    }

    #[test]
    fn account_scope_is_enforced_the_same_way() {
        let c = cfg(&[], &["42"]);
        let qs = apply(&c, AGENTS, pairs(&[("accountIds", "42")])).unwrap().unwrap();
        assert_eq!(qs, "accountIds=42");
        let err = apply(&c, AGENTS, pairs(&[("accountIds", "43")])).unwrap_err();
        assert!(err.contains("outside this tenant's scope"), "{err}");
    }

    #[test]
    fn accounts_endpoint_is_refused_for_a_site_only_scope() {
        let c = cfg(&["100"], &[]);
        assert_eq!(policy(&c, "/web/api/v2.1/accounts"), PathPolicy::Unscopeable);
        let err = apply(&c, "/web/api/v2.1/accounts", vec![]).unwrap_err();
        assert!(err.contains("refusing"), "{err}");

        // With account ids configured it becomes confinable.
        let c = cfg(&["100"], &["42"]);
        assert_eq!(policy(&c, "/web/api/v2.1/accounts"), PathPolicy::QueryScoped);
    }

    #[test]
    fn activity_types_is_served_unconfined_but_still_stripped() {
        let c = cfg(&["100"], &[]);
        let qs = apply(&c, "/web/api/v2.1/activities/types", pairs(&[("siteIds", "999")]))
            .unwrap();
        assert_eq!(qs, None);
    }

    #[test]
    fn unresolved_ids_reports_out_of_scope_targets() {
        let req = vec!["a".to_string(), "b".to_string()];
        assert_eq!(unresolved_ids(&req, &["a".to_string()]), vec!["b".to_string()]);
        assert!(unresolved_ids(&req, &["a".into(), "b".into()]).is_empty());
        assert_eq!(unresolved_ids(&req, &[]), req);
    }

    #[test]
    fn scope_key_detection_covers_body_fields() {
        for k in ["siteIds", "site_ids", "SITEIDS", "accountId", "groupIds", "tenant", "targetScope"] {
            assert!(is_scope_key(k), "{k} should be a scope key");
        }
        for k in ["osTypes", "computerName__contains", "limit", "siteName"] {
            assert!(!is_scope_key(k), "{k} should not be a scope key");
        }
        // Body fields name scope more loosely than query params do.
        for k in ["targetSiteId", "targetSiteIds", "filterSiteIds", "sourceAccountId"] {
            assert!(mentions_scope_id(k), "{k} should be caught in a body");
        }
        for k in ["scriptId", "analystVerdict", "text"] {
            assert!(!mentions_scope_id(k), "{k} should not be caught in a body");
        }
    }
}
