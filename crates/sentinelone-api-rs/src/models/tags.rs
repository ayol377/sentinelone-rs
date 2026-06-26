//! Response models for the `Tags` tag.

use serde::Deserialize;

/// A SentinelOne Tag (`tags.schemas_GetTagSchema`).
///
/// Returned by `GET /web/api/v2.1/tags` (as a list), and by
/// `POST /web/api/v2.1/tags` / `PUT /web/api/v2.1/tags/{tag_id}` (as a single
/// resource). In the spec only `type` is in the `required` array (and it is not
/// `x-nullable`), so it is the only non-`Option` field; every other field is
/// optional/nullable and defaults to `None`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// Type. Required (not nullable). Allowed values: `firewall`,
    /// `network-quarantine`, `device-inventory`.
    #[serde(rename = "type")]
    pub type_: String,
    /// Name. Optional/nullable.
    #[serde(default)]
    pub name: Option<String>,
    /// Id. Optional/nullable.
    #[serde(default)]
    pub id: Option<String>,
    /// Description. Optional/nullable.
    #[serde(default)]
    pub description: Option<String>,
    /// Kind is a MGMT side indication to categorize special tags like
    /// `vulnerability`. Optional/nullable.
    #[serde(default)]
    pub kind: Option<String>,
    /// Timestamp of tag creation (date-time string, e.g.
    /// `2018-02-27T04:49:26.257525Z`). Optional/nullable.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Timestamp of last update (date-time string, e.g.
    /// `2018-02-27T04:49:26.257525Z`). Optional/nullable.
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Location creator name (e.g. `John Doe`). Optional/nullable.
    #[serde(default)]
    pub creator: Option<String>,
    /// Location creator ID. Optional/nullable.
    #[serde(default)]
    pub creator_id: Option<String>,
    /// Location updater name (e.g. `John Doe`). Optional/nullable.
    #[serde(default)]
    pub updater: Option<String>,
    /// Location updater ID. Optional/nullable.
    #[serde(default)]
    pub updater_id: Option<String>,
    /// Linked rules. Optional/nullable.
    #[serde(default)]
    pub linked_rules: Option<i64>,
    /// Affected scopes. Optional/nullable.
    #[serde(default)]
    pub affected_scopes: Option<i64>,
    /// Scope. Optional/nullable. Allowed values: `global`, `group`, `account`,
    /// `site`.
    #[serde(default)]
    pub scope: Option<String>,
    /// Scope id. Optional/nullable.
    #[serde(default)]
    pub scope_id: Option<String>,
    /// Scope name. Optional/nullable.
    #[serde(default)]
    pub scope_name: Option<String>,
}

/// Number of entities affected by a requested operation
/// (`_AffectedResultsSchema`).
///
/// Returned by the `DELETE /web/api/v2.1/tags` and
/// `DELETE /web/api/v2.1/tags/{tag_id}` endpoints. The spec marks no field as
/// required, so `affected` is optional/nullable.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagAffected {
    /// Number of entities affected by the requested operation.
    /// Optional/nullable.
    #[serde(default)]
    pub affected: Option<i64>,
}
