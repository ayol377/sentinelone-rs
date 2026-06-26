//! Models for the `Inventory Network` tag (`/web/api/v2.1/xdr/assets/network`).
//!
//! The network asset entity (`NetworkResponse` in the spec) is defined as an
//! open/empty object (`{ "type": "object", "properties": {} }`), so its fields
//! are not enumerated by the spec. It is therefore modelled as a freeform
//! [`serde_json::Value`] via the [`NetworkAsset`] alias to preserve every field
//! the API returns.

use serde::Deserialize;

/// A single network inventory asset.
///
/// The spec defines `NetworkResponse` as an open object with no declared
/// properties, so this is a freeform JSON value carrying whatever fields the
/// API returns for an asset.
pub type NetworkAsset = serde_json::Value;

/// Response payload for
/// `POST /web/api/v2.1/xdr/assets/network/available-actions/with-status`.
///
/// Corresponds to `AvailableActionWithStatusResponse` in the spec.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatus {
    /// The list of actions available for the selected assets, each with its
    /// enabled/disabled status. Optional / may be absent (not in the schema
    /// `required` array).
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action with its status.
///
/// Corresponds to `AvailableAction1` in the spec.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// The action identifier. Optional / nullable (not in `required`).
    ///
    /// Allowed values: `Inventory`, `enable_protection`, `disable_protection`,
    /// `start_full_scan`, `stop_full_scan`, `enable_cws_monitoring`,
    /// `enable_cns`, `disable_cns`, `start_vm_scan`, `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    pub name: Option<String>,
    /// Whether the action is disabled for the current selection. Optional /
    /// nullable (not in `required`).
    pub is_disabled: Option<bool>,
    /// The reason the action is disabled, when applicable. Optional / nullable
    /// (not in `required`).
    pub disabled_reason: Option<String>,
}
