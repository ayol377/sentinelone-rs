//! Models for the `Inventory Developer Tool` tag
//! (`inventory_developer_tool` service).
//!
//! Hand-written for strict 1:1 spec parity with `swagger_2_1.json`.
//! Field nullability follows the spec: a field is bare `T` only when it is in
//! the schema `required` array and not `x-nullable`; otherwise `Option<T>`.

use std::collections::HashMap;

use serde::Deserialize;

/// A developer-tool inventory asset (`DeveloperToolResponse`).
///
/// The spec defines this entity as a free-form object with no fixed
/// properties (`"properties": {}`), so all fields are captured dynamically.
/// Use [`Asset::extra`] to access the returned key/value pairs.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    /// All fields returned for the asset. The spec models `DeveloperToolResponse`
    /// as an open object with no declared properties, so the concrete shape is
    /// captured here as arbitrary JSON key/value pairs.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Response payload for the "Available actions" endpoint
/// (`AvailableActionWithStatusResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatus {
    /// Available actions for the selected assets.
    ///
    /// Optional/nullable in the spec (not in `required`) -> `Option`.
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action with its status (`AvailableAction1`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// Reason the action is disabled, when applicable.
    ///
    /// Optional/nullable in the spec (not in `required`) -> `Option`.
    pub disabled_reason: Option<String>,
    /// Whether the action is currently disabled.
    ///
    /// Optional/nullable in the spec (not in `required`) -> `Option`.
    pub is_disabled: Option<bool>,
    /// The action name.
    ///
    /// Enum (documented for forward-compat; modelled as `String`). Allowed
    /// values: `Inventory`, `enable_protection`, `disable_protection`,
    /// `start_full_scan`, `stop_full_scan`, `enable_cws_monitoring`,
    /// `enable_cns`, `disable_cns`, `start_vm_scan`, `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    ///
    /// Optional/nullable in the spec (not in `required`) -> `Option`.
    pub name: Option<String>,
}
