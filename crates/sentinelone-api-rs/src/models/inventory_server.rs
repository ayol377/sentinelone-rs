//! Models for the `Inventory Server` tag (XDR asset inventory — server assets).
//!
//! Generated to match `swagger_2_1.json` with envelope stripping: the
//! `{ data, pagination, errors }` wrappers are handled by
//! [`crate::pagination::Paginated`] / [`crate::pagination::Response`], so the
//! models here describe only the inner entities.

use serde::Deserialize;

/// A single server asset returned by the inventory endpoints.
///
/// In `swagger_2_1.json` the `ServerResponse` definition is an open/freeform
/// object (`{ "type": "object", "properties": {} }`) — the API returns a wide,
/// dynamically-shaped record per asset. It is therefore modelled as a transparent
/// JSON object to preserve every field without lossy typing.
pub type Server = serde_json::Value;

/// Response body of
/// `POST /web/api/v2.1/xdr/assets/server/available-actions/with-status`.
///
/// Wraps the list of actions that can be performed on the selected assets,
/// each annotated with whether it is currently disabled and why.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatus {
    /// Available actions for the selected assets.
    ///
    /// Optional / not in the schema `required` array -> `Option`, default null.
    #[serde(default)]
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action together with its enabled/disabled status.
///
/// All fields are optional (none appear in the schema `required` array).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// Human-readable reason the action is disabled, when applicable.
    ///
    /// Optional / not in `required` -> `Option`, default null.
    #[serde(default)]
    pub disabled_reason: Option<String>,
    /// Whether this action is currently disabled for the selected assets.
    ///
    /// Optional / not in `required` -> `Option`, default null.
    #[serde(default)]
    pub is_disabled: Option<bool>,
    /// The action name.
    ///
    /// Optional / not in `required` -> `Option`, default null.
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
    #[serde(default)]
    pub name: Option<String>,
}
