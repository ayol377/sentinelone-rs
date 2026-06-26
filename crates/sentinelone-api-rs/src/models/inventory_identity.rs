//! Models for the `Inventory Identity` tag.
//!
//! Response entities are sourced from `swagger_2_1.json`. The primary asset
//! entity (`IdentityResponse1`) is declared with no fixed properties in the
//! spec (freeform), so [`Identity`] captures every returned field via a
//! flattened [`serde_json::Value`] to preserve forward-compatibility.

use serde::Deserialize;

/// An identity inventory asset.
///
/// Spec entity: `IdentityResponse1`. The schema defines no fixed properties,
/// so all returned fields are captured in [`Identity::extra`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    /// All fields returned by the API for the asset. The spec declares this
    /// entity as a freeform object, so fields are not statically typed.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// A single available action with its enabled/disabled status.
///
/// Spec entity: `AvailableAction1`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// The action name.
    ///
    /// Enum (forward-compat as `String`): `Inventory`, `enable_protection`,
    /// `disable_protection`, `start_full_scan`, `stop_full_scan`,
    /// `enable_cws_monitoring`, `enable_cns`, `disable_cns`, `start_vm_scan`,
    /// `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`,
    /// `add_note`, `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`,
    /// `clear_tags`.
    ///
    /// Optional / nullable -> `Option`.
    pub name: Option<String>,
    /// Whether the action is currently disabled. Optional / nullable -> `Option`.
    pub is_disabled: Option<bool>,
    /// The reason the action is disabled, if any. Optional / nullable -> `Option`.
    pub disabled_reason: Option<String>,
}

/// Response payload for the available-actions endpoint.
///
/// Spec entity: `AvailableActionWithStatusResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatus {
    /// The list of available actions with their statuses.
    /// Optional / nullable -> `Option`.
    pub available_actions: Option<Vec<AvailableAction>>,
}
