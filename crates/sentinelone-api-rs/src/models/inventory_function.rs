//! Models for the `Inventory Function` tag (XDR asset inventory "function" view).
//!
//! Source: `swagger_2_1.json` definitions
//! `v2_1.inventory.function.schemas_FunctionResponseSchema_many` and
//! `v2_1.inventory.schemas_AvailableActionWithStatusResponseSchema`.

use serde::Deserialize;

/// A single asset row returned by the inventory "function" view.
///
/// Source definition: `#/definitions/FunctionResponse`. The schema declares an
/// open object with no fixed properties (`{"type":"object","properties":{}}`),
/// so every field returned by the API is captured in [`Function::extra`] as raw
/// JSON for forward compatibility.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    /// All fields returned by the API. The upstream schema defines no concrete
    /// properties, so the full asset object is preserved here verbatim.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Response payload for `POST /web/api/v2.1/xdr/assets/function/available-actions/with-status`.
///
/// Source definition: `#/definitions/AvailableActionWithStatusResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions for the selected assets, each with its disabled status.
    ///
    /// Optional / nullable: the field is not in the schema `required` array.
    #[serde(default)]
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action and its current status.
///
/// Source definition: `#/definitions/AvailableAction1`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// The action name.
    ///
    /// Enum (documented for forward-compat; kept as `String`): `Inventory`,
    /// `enable_protection`, `disable_protection`, `start_full_scan`,
    /// `stop_full_scan`, `enable_cws_monitoring`, `enable_cns`, `disable_cns`,
    /// `start_vm_scan`, `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`, `add_note`,
    /// `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    ///
    /// Optional / nullable: not in the schema `required` array.
    #[serde(default)]
    pub name: Option<String>,

    /// Whether this action is currently disabled for the selection.
    ///
    /// Optional / nullable: not in the schema `required` array.
    #[serde(default)]
    pub is_disabled: Option<bool>,

    /// Human-readable reason the action is disabled, when applicable.
    ///
    /// Optional / nullable: not in the schema `required` array.
    #[serde(default)]
    pub disabled_reason: Option<String>,
}
