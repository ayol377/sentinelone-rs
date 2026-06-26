//! Models for the `Inventory Storage` tag.
//!
//! Field nullability mirrors the SentinelOne `swagger_2_1.json` spec: a field is
//! a bare type only when it appears in the schema `required` array and is not
//! `x-nullable`; otherwise it is `Option<T>` (the spec's "default null"
//! behaviour). Enum-valued strings are kept as `String` for forward
//! compatibility, with the allowed values documented on each field.

use serde::Deserialize;

/// A storage asset (`StorageResponse` in the spec).
///
/// Returned in the `data` array of `GET`/`POST
/// /web/api/v2.1/xdr/assets/storage`. The spec defines this response object with
/// no fixed properties, so all returned fields are captured verbatim in
/// [`extra`](Self::extra) for forward compatibility.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageAsset {
    /// All fields returned for the asset. The spec declares no fixed schema for
    /// this object, so every key is preserved here as-is.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Response wrapping the available actions and their status
/// (`AvailableActionWithStatusResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions.
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action with its enabled/disabled status
/// (`AvailableAction1` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// Action name. Allowed values: `Inventory`, `enable_protection`,
    /// `disable_protection`, `start_full_scan`, `stop_full_scan`,
    /// `enable_cws_monitoring`, `enable_cns`, `disable_cns`, `start_vm_scan`,
    /// `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`,
    /// `add_note`, `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`,
    /// `clear_tags`.
    pub name: Option<String>,
    /// Whether the action is disabled.
    pub is_disabled: Option<bool>,
    /// Reason the action is disabled, if applicable.
    pub disabled_reason: Option<String>,
}
