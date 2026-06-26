//! Response models for the `Network Quarantine Control` tag.
//!
//! Network Quarantine Control shares the Firewall Control endpoint family,
//! addressed via the `{firewall_rule_category}` path segment (use
//! `network-quarantine` to affect Network Quarantine). The response schemas are
//! therefore the Firewall Control schemas.
//!
//! Field optionality follows the spec: a field is a bare `T` only when it is
//! listed in the schema `required` array *and* is not `x-nullable`; otherwise it
//! is `Option<T>` (the API's "default null" behaviour). Enum-typed strings are
//! kept as `String` for forward-compatibility, with the allowed values
//! documented on each field.

use serde::Deserialize;

/// A Network Quarantine / Firewall Control rule.
///
/// Returned by `GET /web/api/v2.1/firewall-control/{firewall_rule_category}` and
/// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}`.
///
/// No property is in the schema `required` array, so every field is
/// `Option<T>` ("default null").
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkQuarantineRule {
    /// Rule ID. Optional.
    pub id: Option<String>,
    /// Position in the list of rules. Optional.
    pub order: Option<i64>,
    /// Defines the direction of the rule. Allowed values: `any`, `inbound`,
    /// `outbound`. Nullable.
    pub direction: Option<String>,
    /// \[DEPRECATED\] First os_type (use `os_types` instead). Allowed values:
    /// `linux`, `macos`, `windows_legacy`, `windows`. Optional.
    pub os_type: Option<String>,
    /// OS types. Each value is one of: `linux`, `macos`, `windows_legacy`,
    /// `windows`. Optional.
    pub os_types: Option<Vec<String>>,
    /// \[DEPRECATED\] Free text to describe the rule. Use `description` instead.
    /// Optional.
    pub tag: Option<String>,
    /// Application for the rule (read-only; freeform). Optional.
    pub application: Option<serde_json::Value>,
    /// The name of the rule. Optional.
    pub name: Option<String>,
    /// The protocol. Optional.
    pub protocol: Option<String>,
    /// Product identifier. Unique for a specific product module, per vendor ID,
    /// interface. Optional.
    pub product_id: Option<String>,
    /// Local host (read-only; freeform). Optional.
    pub local_host: Option<serde_json::Value>,
    /// Local ports (read-only; freeform). Optional.
    pub local_port: Option<serde_json::Value>,
    /// \[DEPRECATED\] First remote host in the rule (read-only; freeform). Full
    /// list in `remote_hosts`. Optional.
    pub remote_host: Option<serde_json::Value>,
    /// List of remote hosts. Optional.
    pub remote_hosts: Option<Vec<NetworkQuarantineRuleRemoteHost>>,
    /// Remote ports (read-only; freeform). Optional.
    pub remote_port: Option<serde_json::Value>,
    /// Location associated with the rule. Optional.
    pub location: Option<NetworkQuarantineRuleLocation>,
    /// Defines if the agent shall block or allow traffic matching the rule
    /// parameters. Allowed values: `Allow`, `Block`. Optional.
    pub action: Option<String>,
    /// Defines if the rule is enabled or disabled. Allowed values: `Enabled`,
    /// `Disabled`. Optional.
    pub status: Option<String>,
    /// Scope of the rule. Allowed values: `global`, `group`, `account`, `site`.
    /// Optional.
    pub scope: Option<String>,
    /// The group or site id depending on the scope. `null` if it is global.
    /// Optional.
    pub scope_id: Option<String>,
    /// True if the rule can be modified at this scope level. Optional.
    pub editable: Option<bool>,
    /// Date of rule creation (ISO-8601 timestamp). Optional.
    pub created_at: Option<String>,
    /// Date of last update (ISO-8601 timestamp). Optional.
    pub updated_at: Option<String>,
    /// Full name of the creating user. Optional.
    pub creator: Option<String>,
    /// Id of the creating user. Optional.
    pub creator_id: Option<String>,
    /// Tags (IDs and names) linked to this rule. Optional.
    pub tags: Option<Vec<NetworkQuarantineRuleTag>>,
    /// List of tag IDs this rule is linked to. Optional.
    pub tag_ids: Option<Vec<String>>,
    /// List of tag names this rule is linked to. Optional.
    pub tag_names: Option<Vec<String>>,
    /// Description. Nullable.
    pub description: Option<String>,
    /// Network quarantine rule or standard firewall rule. Allowed values:
    /// `firewall`, `network_quarantine`. Optional.
    pub rule_category: Option<String>,
}

/// A remote host entry on a [`NetworkQuarantineRule`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkQuarantineRuleRemoteHost {
    /// Type of the host. Allowed values: `any`, `cidr`, `range`, `addresses`,
    /// `fqdn`. Optional.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Value(s) of the host. Nullable.
    pub values: Option<Vec<String>>,
}

/// Location associated with a [`NetworkQuarantineRule`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkQuarantineRuleLocation {
    /// Location type. Allowed values: `all`, `specific`, `fallback`. Optional.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Location IDs (applicable for the `specific` location type only).
    /// Nullable.
    pub values: Option<Vec<NetworkQuarantineRuleLocationValue>>,
}

/// A single location reference inside
/// [`NetworkQuarantineRuleLocation::values`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkQuarantineRuleLocationValue {
    /// Location ID. Required.
    pub id: String,
    /// Location name. Nullable.
    pub name: Option<String>,
    /// Location scope. Allowed values: `global`, `group`, `account`, `site`.
    /// Nullable.
    pub scope: Option<String>,
}

/// A tag (ID + name) linked to a [`NetworkQuarantineRule`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkQuarantineRuleTag {
    /// Tag name. Optional.
    pub name: Option<String>,
    /// Tag ID. Optional.
    pub id: Option<String>,
}

/// Network Quarantine / Firewall Control configuration for a scope.
///
/// Returned by `GET` / `PUT`
/// `/web/api/v2.1/firewall-control/{firewall_rule_category}/configuration`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkQuarantineSettings {
    /// Firewall control enabled for the scope. Optional.
    pub enabled: Option<bool>,
    /// Firewall control supports location awareness for the scope. Optional.
    pub location_aware: Option<bool>,
    /// Agent should report blocked events. Optional.
    pub report_blocked: Option<bool>,
    /// True if rules are decoupled from parent rules. Optional.
    pub inherits: Option<bool>,
    /// If `null`, this is the scope's own policy; otherwise the ancestor scope
    /// the policy is inherited from. For groups: `null`/`Site`/`Global`; for
    /// sites: `null`/`Global`. Nullable.
    pub inherited_from: Option<String>,
    /// Selected (enabled) tag IDs. Optional.
    pub selected_tags: Option<Vec<String>>,
    /// Inherit firewall settings from the parent scope. Optional.
    pub inherit_settings: Option<bool>,
    /// Inherit all the rules and tags from the parent scope. Expands on
    /// `inherits`. Optional.
    pub inherit_all_firewall_rules: Option<bool>,
}

/// A protocol usable in Network Quarantine / Firewall Control rules.
///
/// Returned by
/// `GET /web/api/v2.1/firewall-control/{firewall_rule_category}/protocols`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkQuarantineProtocol {
    /// List of Account IDs to filter by. Optional.
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by. Optional.
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by. Optional.
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request. Optional.
    pub tenant: Option<bool>,
    /// Short code identifying the protocol (read-only). Optional.
    pub value: Option<String>,
    /// Description of the protocol. Optional.
    pub name: Option<String>,
}

/// Number of entities affected by a mutation.
///
/// Returned by the delete / enable / copy / move / set-location / add-tags /
/// remove-tags endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResult {
    /// Number of entities affected by the requested operation. Optional.
    pub affected: Option<i64>,
}

/// Generic success indicator.
///
/// Returned by the reorder and import endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Indicates a successful operation. Optional.
    pub success: Option<bool>,
}
