//! Models for the `Inventory Container` tag.
//!
//! These types map the response envelopes documented under
//! `/web/api/v2.1/xdr/assets/container`. The inventory "container" asset
//! object (`ContainerResponse`) is defined in the spec as a free-form object
//! with no fixed properties, so it is represented as [`serde_json::Value`].

use serde::Deserialize;

/// A single inventory container asset.
///
/// In `swagger_2_1.json` the `ContainerResponse` definition is an empty object
/// (`{ "type": "object", "properties": {} }`) — the asset shape is dynamic and
/// varies by surface/resource type, so it is modelled as raw JSON for
/// forward-compatibility.
pub type Container = serde_json::Value;

/// Response payload for `POST
/// `/web/api/v2.1/xdr/assets/container/available-actions/with-status` —
/// the set of actions available for the selected assets, each annotated with
/// its enabled/disabled status.
///
/// Maps `#/definitions/AvailableActionWithStatusResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions. Optional (not in the schema `required` array).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action plus its current availability status.
///
/// Maps `#/definitions/AvailableAction1`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// The action name. Optional/nullable.
    ///
    /// Allowed values (enum, kept as `String` for forward-compat):
    /// `Inventory`, `enable_protection`, `disable_protection`,
    /// `start_full_scan`, `stop_full_scan`, `enable_cws_monitoring`,
    /// `enable_cns`, `disable_cns`, `start_vm_scan`, `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<String>,
    /// Whether this action is currently disabled for the selection. Optional.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub is_disabled: Option<bool>,
    /// Human-readable reason the action is disabled, when applicable. Optional.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub disabled_reason: Option<String>,
}
