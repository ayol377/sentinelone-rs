//! Response models for the `Firewall Control` tag.
//!
//! Field optionality follows the spec: a field is a bare `T` only when it is
//! listed in the schema `required` array *and* is not `x-nullable`; otherwise
//! it is `Option<T>` (the API's "default null" behaviour). Enum-typed strings
//! are kept as `String` for forward-compatibility, with the allowed values
//! documented on each field.

use serde::Deserialize;

/// A Firewall Control rule.
///
/// Returned by `GET /web/api/v2.1/firewall-control`,
/// `GET /web/api/v2.1/firewall-control/tag-rules/{tag_id}`,
/// `POST /web/api/v2.1/firewall-control` and
/// `PUT /web/api/v2.1/firewall-control/{firewall_rule_category}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRule {
    /// Rule ID.
    pub id: Option<String>,
    /// Position in the list of rules.
    pub order: Option<i64>,
    /// Defines the direction of the Firewall rule. Allowed values: `any`,
    /// `inbound`, `outbound`. Nullable.
    pub direction: Option<String>,
    /// \[DEPRECATED\] First os_type (use `os_types` instead). Allowed values:
    /// `linux`, `macos`, `windows_legacy`, `windows`.
    pub os_type: Option<String>,
    /// OS types. Each value is one of: `linux`, `macos`, `windows_legacy`,
    /// `windows`.
    pub os_types: Option<Vec<String>>,
    /// \[DEPRECATED\] Free text to describe the rule. Use `description` instead.
    pub tag: Option<String>,
    /// Application for the rule (read-only; freeform).
    pub application: Option<serde_json::Value>,
    /// The name of the firewall rule.
    pub name: Option<String>,
    /// The protocol.
    pub protocol: Option<String>,
    /// Product identifier. Unique for a specific product module, per vendor ID,
    /// interface.
    pub product_id: Option<String>,
    /// Local host (read-only; freeform).
    pub local_host: Option<serde_json::Value>,
    /// Local ports (read-only; freeform).
    pub local_port: Option<serde_json::Value>,
    /// \[DEPRECATED\] First remote host in the rule (read-only; freeform). Full
    /// list in `remote_hosts`.
    pub remote_host: Option<serde_json::Value>,
    /// List of remote hosts.
    pub remote_hosts: Option<Vec<FirewallRuleRemoteHost>>,
    /// Remote ports (read-only; freeform).
    pub remote_port: Option<serde_json::Value>,
    /// Location associated with the rule.
    pub location: Option<FirewallRuleLocation>,
    /// Defines if the agent shall block or allow traffic matching the rule
    /// parameters. Allowed values: `Allow`, `Block`.
    pub action: Option<String>,
    /// Defines if the rule is enabled or disabled. Allowed values: `Enabled`,
    /// `Disabled`.
    pub status: Option<String>,
    /// Scope of the rule. Allowed values: `global`, `group`, `account`, `site`.
    pub scope: Option<String>,
    /// The group or site id depending on the scope. `null` if it is global.
    pub scope_id: Option<String>,
    /// True if the rule can be modified at this scope level.
    pub editable: Option<bool>,
    /// Date of rule creation (ISO-8601 timestamp).
    pub created_at: Option<String>,
    /// Date of last update (ISO-8601 timestamp).
    pub updated_at: Option<String>,
    /// Full name of the creating user.
    pub creator: Option<String>,
    /// Id of the creating user.
    pub creator_id: Option<String>,
    /// Tags (IDs and names) linked to this rule.
    pub tags: Option<Vec<FirewallRuleTag>>,
    /// List of tag IDs this firewall rule is linked to.
    pub tag_ids: Option<Vec<String>>,
    /// List of tag names this firewall rule is linked to.
    pub tag_names: Option<Vec<String>>,
    /// Description. Nullable.
    pub description: Option<String>,
    /// Network quarantine rule or standard firewall rule. Allowed values:
    /// `firewall`, `network_quarantine`.
    pub rule_category: Option<String>,
}

/// A remote host entry on a [`FirewallRule`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRuleRemoteHost {
    /// Type of the host. Allowed values: `any`, `cidr`, `range`, `addresses`,
    /// `fqdn`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Value(s) of the host. Nullable.
    pub values: Option<Vec<String>>,
}

/// Location associated with a [`FirewallRule`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRuleLocation {
    /// Location type. Allowed values: `all`, `specific`, `fallback`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Location IDs (applicable for the `specific` location type only).
    /// Nullable.
    pub values: Option<Vec<FirewallRuleLocationValue>>,
}

/// A single location reference inside [`FirewallRuleLocation::values`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRuleLocationValue {
    /// Location ID. Required.
    pub id: String,
    /// Location name. Nullable.
    pub name: Option<String>,
    /// Location scope. Allowed values: `global`, `group`, `account`, `site`.
    /// Nullable.
    pub scope: Option<String>,
}

/// A tag (ID + name) linked to a [`FirewallRule`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallRuleTag {
    /// Tag name.
    pub name: Option<String>,
    /// Tag ID.
    pub id: Option<String>,
}

/// Firewall Control configuration for a scope.
///
/// Returned by `GET` / `PUT /web/api/v2.1/firewall-control/configuration`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallSettings {
    /// Firewall control enabled for the scope.
    pub enabled: Option<bool>,
    /// Firewall control supports location awareness for the scope.
    pub location_aware: Option<bool>,
    /// Agent should report blocked events.
    pub report_blocked: Option<bool>,
    /// True if rules are decoupled from parent rules.
    pub inherits: Option<bool>,
    /// If `null`, this is the scope's own policy; otherwise the ancestor scope
    /// the policy is inherited from. For groups: `null`/`Site`/`Global`; for
    /// sites: `null`/`Global`. Nullable.
    pub inherited_from: Option<String>,
    /// Selected (enabled) tag IDs.
    pub selected_tags: Option<Vec<String>>,
    /// Inherit firewall settings from the parent scope.
    pub inherit_settings: Option<bool>,
    /// Inherit all the rules and tags from the parent scope. Expands on
    /// `inherits`.
    pub inherit_all_firewall_rules: Option<bool>,
}

/// A protocol usable in Firewall Control rules.
///
/// Returned by `GET /web/api/v2.1/firewall-control/protocols`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FirewallProtocol {
    /// List of Account IDs to filter by.
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by.
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by.
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request.
    pub tenant: Option<bool>,
    /// Short code identifying the protocol (read-only).
    pub value: Option<String>,
    /// Description of the protocol.
    pub name: Option<String>,
}

/// Number of entities affected by a mutation.
///
/// Returned by the delete / enable / copy / move / set-location / add-tags /
/// remove-tags endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
}

/// Generic success indicator.
///
/// Returned by the reorder and import endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Indicates a successful operation.
    pub success: Option<bool>,
}
