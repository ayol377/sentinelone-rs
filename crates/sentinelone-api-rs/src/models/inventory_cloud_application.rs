//! Models for the `Inventory Cloud Application` tag.
//!
//! Field nullability mirrors the SentinelOne OpenAPI spec: a field is a bare
//! `T` only when it is in the schema `required` array and not `x-nullable`;
//! everything else is `Option<T>` (the API's "default null" behaviour).

use serde::Deserialize;

/// A cloud application inventory asset.
///
/// Spec definition: `CloudApplicationResponse`. The spec declares this as a
/// free-form object (no fixed properties), so the full payload is preserved as
/// a [`serde_json::Value`]. Common keys observed in practice include the asset
/// `id`, `name`, `category`, `subCategory`, `region`, `assetStatus`,
/// `assetCriticality`, cloud-provider metadata, coverage and tag information.
pub type CloudApplication = serde_json::Value;

/// Response data for
/// `POST /web/api/v2.1/xdr/assets/cloud-application/available-actions/with-status`.
///
/// Spec definition: `AvailableActionWithStatusResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions.
    ///
    /// Optional/nullable in the spec (not in `required`).
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action together with its enabled/disabled status.
///
/// Spec definition: `AvailableAction1`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// The action name.
    ///
    /// Enum in the spec (kept as `String` for forward-compatibility). Allowed
    /// values: `Inventory`, `enable_protection`, `disable_protection`,
    /// `start_full_scan`, `stop_full_scan`, `enable_cws_monitoring`,
    /// `enable_cns`, `disable_cns`, `start_vm_scan`, `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    ///
    /// Optional/nullable in the spec (not in `required`).
    pub name: Option<String>,
    /// Whether the action is disabled.
    ///
    /// Optional/nullable in the spec (not in `required`).
    pub is_disabled: Option<bool>,
    /// Human-readable reason why the action is disabled, when applicable.
    ///
    /// Optional/nullable in the spec (not in `required`).
    pub disabled_reason: Option<String>,
}
