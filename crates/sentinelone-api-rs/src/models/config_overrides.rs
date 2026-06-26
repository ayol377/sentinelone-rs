//! Models for the `Config Overrides` tag.
//!
//! Agents configuration override entities. There are different ways to
//! override the configuration of an Agent, and the priority of changes depends
//! on the endpoint OS and the version of the installed Agent.

use serde::Deserialize;

/// A single configuration override entity.
///
/// Returned by `GET /web/api/v2.1/config-override` (as a list),
/// `POST /web/api/v2.1/config-override` and
/// `PUT /web/api/v2.1/config-override/{override_id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigOverride {
    /// Id. Optional/nullable in the spec (not in `required`) -> `Option`.
    pub id: Option<String>,
    /// Name. Required (in `required`) and not nullable -> bare `String`.
    pub name: String,
    /// Description. Required (in `required`) and not nullable -> bare `String`.
    pub description: String,
    /// Config. Required (in `required`); freeform object -> `serde_json::Value`.
    pub config: serde_json::Value,
    /// OS type. Optional/nullable. Allowed values: `linux`, `macos`,
    /// `windows_legacy`, `windows`.
    pub os_type: Option<String>,
    /// Agent version. Optional/nullable -> `Option`.
    pub agent_version: Option<String>,
    /// Version option. Optional/nullable. Allowed values: `ALL`, `SPECIFIC`.
    pub version_option: Option<String>,
    /// Scope level. Optional/nullable. Allowed values: `group`, `site`,
    /// `account`, `tenant`.
    pub scope: Option<String>,
    /// Site reference. Optional/nullable -> `Option`.
    pub site: Option<ConfigOverrideSite>,
    /// Group reference. Optional/nullable -> `Option`.
    pub group: Option<ConfigOverrideGroup>,
    /// Account reference. Optional/nullable -> `Option`.
    pub account: Option<ConfigOverrideAccount>,
    /// Agent reference. Optional/nullable -> `Option`.
    pub agent: Option<ConfigOverrideAgent>,
    /// Created at (date-time string). Optional/nullable -> `Option`.
    pub created_at: Option<String>,
    /// Updated at (date-time string). Optional/nullable -> `Option`.
    pub updated_at: Option<String>,
}

/// Site reference embedded in a [`ConfigOverride`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigOverrideSite {
    /// Id. Required (in `required`) and not nullable -> bare `String`.
    pub id: String,
    /// Name. Optional/nullable -> `Option`.
    pub name: Option<String>,
}

/// Group reference embedded in a [`ConfigOverride`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigOverrideGroup {
    /// Id. Required (in `required`) and not nullable -> bare `String`.
    pub id: String,
    /// Name. Optional/nullable -> `Option`.
    pub name: Option<String>,
}

/// Account reference embedded in a [`ConfigOverride`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigOverrideAccount {
    /// Id. Required (in `required`) and not nullable -> bare `String`.
    pub id: String,
    /// Name. Optional/nullable -> `Option`.
    pub name: Option<String>,
}

/// Agent reference embedded in a [`ConfigOverride`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigOverrideAgent {
    /// Id. Required (in `required`) and not nullable -> bare `String`.
    pub id: String,
}

/// `data` payload of `DELETE /web/api/v2.1/config-override`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigOverrideAffected {
    /// Number of entities affected by the requested operation.
    /// Optional/nullable in the spec -> `Option`.
    pub affected: Option<i64>,
}

/// `data` payload of `DELETE /web/api/v2.1/config-override/{override_id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigOverrideSuccess {
    /// Indicates a successful operation. Optional/nullable in the spec ->
    /// `Option`.
    pub success: Option<bool>,
}
