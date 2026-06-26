//! Models for the `Inventory Application Integration` tag.
//!
//! The SentinelOne spec defines the asset entity returned by the
//! application-integration endpoints as an open (free-form) object, so the
//! main [`ApplicationIntegration`] entity captures all fields in a flattened
//! map for forward-compatibility.

use std::collections::HashMap;

use serde::Deserialize;

/// A single application-integration asset row.
///
/// The spec models this entity (`ApplicationIntegrationResponse`) as an open
/// object with no declared properties, so every field is captured in
/// [`extra`](Self::extra) to remain forward-compatible with the API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationIntegration {
    /// All fields returned by the API. The asset schema is open/free-form in
    /// the spec, so no individual properties are typed.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Response payload for the available-actions-with-status endpoint
/// (`AvailableActionWithStatusResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions. Optional/nullable in the spec.
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action with its enabled/disabled status
/// (`AvailableAction1`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// The reason the action is disabled, when applicable. Optional/nullable.
    pub disabled_reason: Option<String>,
    /// Whether the action is currently disabled. Optional/nullable.
    pub is_disabled: Option<bool>,
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
    pub name: Option<String>,
}
