//! Models for the `Inventory Data Store` tag.
//!
//! Source: `swagger_2_1.json` (envelope-stripped via [`crate::pagination`]).
//!
//! The primary asset entity (`DataStoreResponse` in the spec) is declared as a
//! freeform object (`"properties": {}`), so [`DataStoreAsset`] is a transparent
//! wrapper around [`serde_json::Value`] that preserves the full asset payload
//! while keeping a named model for the public API surface.

use serde::Deserialize;

/// A single asset returned by the Inventory Data Store endpoints.
///
/// The spec models this entity (`DataStoreResponse`) as a freeform object with
/// no declared properties, so the entire asset payload is preserved verbatim as
/// a JSON value. Access fields via the inner [`serde_json::Value`].
#[derive(Debug, Clone, Deserialize)]
#[serde(transparent)]
pub struct DataStoreAsset(pub serde_json::Value);

/// Response payload for
/// `POST /web/api/v2.1/xdr/assets/data-store/available-actions/with-status`.
///
/// Wraps the list of actions that can be performed on the matched assets,
/// together with their availability status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions.
    ///
    /// Optional/nullable in the spec (not in the parent `required` array) -> `Option`.
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action and its current availability status.
///
/// None of these fields are in the schema `required` array, so all are `Option`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// Human-readable reason explaining why the action is disabled, when applicable.
    ///
    /// Optional/nullable -> `Option<String>`.
    pub disabled_reason: Option<String>,

    /// Whether the action is currently disabled for the matched assets.
    ///
    /// Optional/nullable -> `Option<bool>`.
    pub is_disabled: Option<bool>,

    /// Action name (enum in the spec; kept as `String` for forward-compatibility).
    ///
    /// Allowed values: `Inventory`, `enable_protection`, `disable_protection`,
    /// `start_full_scan`, `stop_full_scan`, `enable_cws_monitoring`, `enable_cns`,
    /// `disable_cns`, `start_vm_scan`, `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    ///
    /// Optional/nullable -> `Option<String>`.
    pub name: Option<String>,
}
