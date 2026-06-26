//! Entity models for the `Inventory Governance` tag.
//!
//! Hand-written for 1:1 parity with `swagger_2_1.json`. The governance asset
//! entity (`GovernanceResponse` in the spec) has no declared properties, so its
//! fields are surfaced as a freeform [`serde_json::Value`] map to preserve every
//! attribute the API returns.

use serde::Deserialize;

/// A single governance asset row.
///
/// Resolved from `#/definitions/GovernanceResponse`. The spec declares this
/// object with an empty `properties` map (freeform), so all returned attributes
/// are captured under [`Governance::extra`] rather than being statically typed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Governance {
    /// All asset attributes returned by the API. The spec does not declare a
    /// fixed schema for this object, so every field is captured here verbatim.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Response payload for the "available actions" endpoint.
///
/// Resolved from `#/definitions/AvailableActionWithStatusResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions. Optional/nullable per spec (not in `required`).
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action with its enablement status.
///
/// Resolved from `#/definitions/AvailableAction1`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// Reason the action is disabled, when applicable. Optional/nullable per
    /// spec (not in `required`).
    pub disabled_reason: Option<String>,
    /// Whether this action is currently disabled. Optional/nullable per spec
    /// (not in `required`).
    pub is_disabled: Option<bool>,
    /// Action name. Optional/nullable per spec (not in `required`).
    ///
    /// Allowed values: `Inventory`, `enable_protection`, `disable_protection`,
    /// `start_full_scan`, `stop_full_scan`, `enable_cws_monitoring`,
    /// `enable_cns`, `disable_cns`, `start_vm_scan`, `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    ///
    /// Kept as `String` for forward compatibility.
    pub name: Option<String>,
}
