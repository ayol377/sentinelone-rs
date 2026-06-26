//! Models for the `Exclusions and Blocklist` tag.
//!
//! Field optionality follows the spec: a field is a bare `T` only when it is in
//! the schema `required` array and not `x-nullable`; otherwise it is wrapped in
//! `Option<T>` ("default null"). Enum-valued fields are kept as `String` for
//! forward compatibility, with the allowed values documented inline.

use serde::Deserialize;

/// Scope descriptor attached to an exclusion or blocklist item.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExclusionScope {
    /// Group ids the item applies to.
    pub group_ids: Option<Vec<String>>,
    /// Site ids the item applies to.
    pub site_ids: Option<Vec<String>>,
    /// Account ids the item applies to.
    pub account_ids: Option<Vec<String>>,
    /// Whether the item is scoped to the tenant (global).
    pub tenant: Option<bool>,
}

/// An Exclusion entry, as returned by `GET /web/api/v2.1/exclusions`.
///
/// Note: list and create/update responses share most fields. This struct
/// covers the union; fields absent from a given response deserialize to `None`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exclusion {
    /// The ID of the exclusion (read-only).
    pub id: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// ID of the creating user.
    pub user_id: Option<String>,
    /// Name of the creating user.
    pub user_name: Option<String>,
    /// Timestamp of item creation (ISO-8601). Example: `2018-02-27T04:49:26.257525Z`.
    pub created_at: Option<String>,
    /// Timestamp of item update (ISO-8601). Example: `2018-02-27T04:49:26.257525Z`.
    pub updated_at: Option<String>,
    /// OS type. Allowed values: `linux`, `macos`, `windows_legacy`, `windows`.
    pub os_type: Option<String>,
    /// Exclusion list type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Scope name.
    pub scope_name: Option<String>,
    /// Scope descriptor. Required (and non-null) on create/update responses;
    /// optional on the list response.
    pub scope: Option<ExclusionScope>,
    /// Not recommended indicator.
    pub not_recommended: Option<String>,
    /// Scope path (list response only).
    pub scope_path: Option<String>,
    /// Return filters from parent scope levels (default: false).
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels (default: false).
    pub include_children: Option<bool>,
    /// SHA1 if hash type, or value according to the exclusion list type.
    pub value: Option<String>,
    /// \[DEPRECATED\] Path exclusion monitor mode.
    pub inject: Option<bool>,
    /// Exclusion mode (path exclusion only). Allowed values: `suppress`,
    /// `suppress_dynamic_only`, `suppress_dfi_only`, `disable_in_process_monitor`,
    /// `disable_in_process_monitor_deep`, `disable_all_monitors`,
    /// `disable_all_monitors_deep`, `suppress_app_control`,
    /// `suppress_drift_detection`. Nullable.
    pub mode: Option<String>,
    /// Excluded path for a path exclusion list. Allowed values: `file`, `folder`,
    /// `subfolders`. Nullable.
    pub path_exclusion_type: Option<String>,
    /// Actions to perform. Allowed item values: `upload`, `detect`.
    pub actions: Option<Vec<String>>,
    /// Source. Allowed values: `user`, `cloud`, `action_from_threat`, `catalog`,
    /// `performance_insight`.
    pub source: Option<String>,
    /// Whether the exclusion was imported by a bulk operation.
    pub imported: Option<bool>,
    /// The Application name of exclusions created from the Exclusion Catalog. Nullable.
    pub application_name: Option<String>,
    /// Found or Not found - whether this exclusion is related to an application
    /// found in the scope's Application Inventory.
    pub in_app_inventory: Option<bool>,
}

/// A Blocklist (restriction) entry, as returned by `GET /web/api/v2.1/restrictions`.
///
/// Covers the union of list and create/update response fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restriction {
    /// The ID of the restriction (read-only).
    pub id: Option<String>,
    /// Hash id (create/update response only).
    pub hash_id: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// ID of the creating user.
    pub user_id: Option<String>,
    /// Name of the creating user.
    pub user_name: Option<String>,
    /// Source. Allowed values: `user`, `cloud`, `action_from_threat`, `catalog`,
    /// `performance_insight`.
    pub source: Option<String>,
    /// SHA1 hash.
    pub value: Option<String>,
    /// SHA256 hash value.
    pub sha256_value: Option<String>,
    /// Timestamp of item creation (ISO-8601). Example: `2018-02-27T04:49:26.257525Z`.
    pub created_at: Option<String>,
    /// Timestamp of item update (ISO-8601). Example: `2018-02-27T04:49:26.257525Z`.
    pub updated_at: Option<String>,
    /// OS type. Allowed values: `linux`, `macos`, `windows_legacy`, `windows`.
    pub os_type: Option<String>,
    /// Blocklist type.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// Scope name.
    pub scope_name: Option<String>,
    /// Scope descriptor.
    pub scope: Option<ExclusionScope>,
    /// Not recommended indicator.
    pub not_recommended: Option<String>,
    /// Scope path (list response only).
    pub scope_path: Option<String>,
    /// Return filters from parent scope levels (default: false).
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels (default: false).
    pub include_children: Option<bool>,
    /// Whether the item was imported by a bulk operation.
    pub imported: Option<bool>,
}

/// Result of an import operation
/// (`POST /web/api/v2.1/exclusions/import`, `POST /web/api/v2.1/restrictions/import`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    /// The number of rows in the file.
    pub total: Option<i64>,
    /// The number of entries that imported successfully.
    pub succeeded: Option<i64>,
    /// The ID of the Validation Report generated for the import. It can help you
    /// fix entries that did not import successfully.
    pub report_id: Option<String>,
}

/// Number of entities affected by a delete operation
/// (`DELETE /web/api/v2.1/exclusions`, `DELETE /web/api/v2.1/restrictions`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
}

/// A single validation detail entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateDetail {
    /// Field. Required.
    pub field: String,
    /// Error. Required.
    pub error: String,
}

/// Result of validating an exclusion or blocklist item
/// (`POST /web/api/v2.1/exclusions/validate`, `POST /web/api/v2.1/restrictions/validate`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateResult {
    /// Recommendation for the exclusion/blocklist item. Allowed values:
    /// `Not recommended`, `Not allowed`, `NONE`, `duplicated_value_sha1`,
    /// `duplicated_value_sha256`, `duplicated_value_sha1_sha256`, `Duplication`.
    pub status: Option<String>,
    /// Validation details.
    pub details: Option<Vec<ValidateDetail>>,
}
