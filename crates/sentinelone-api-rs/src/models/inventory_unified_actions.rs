//! Models for the `Inventory Unified Actions` tag.
//!
//! Response entities for the unified-actions endpoints:
//! - `POST /web/api/v2.1/xdr/assets/actions/fetch-agent-ids`
//!   -> [`FetchAgentIdsResponse`]
//! - `POST /web/api/v2.1/xdr/assets/actions/fetch-unified-actions`
//!   -> [`AvailableActionsResponse`]
//! - `POST /web/api/v2.1/xdr/assets/actions/perform-unified-action`
//!   -> untyped (`serde_json::Value`); the spec declares no `200` body schema.
//!
//! Field optionality follows the spec: a field is a bare `T` only when it is
//! listed in the schema `required` array and is not `x-nullable`; otherwise it
//! is `Option<T>` (the "default null" behaviour).

use serde::Deserialize;

/// Response data for
/// `POST /web/api/v2.1/xdr/assets/actions/fetch-agent-ids`
/// ("Loads all agent ids for the unified actions").
///
/// Spec definition: `FetchAgentIdsResponse` (the `data` object of
/// `FetchAgentIdsResponseSchema`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchAgentIdsResponse {
    /// Agent ids. Array of string. Optional/nullable in the spec.
    pub agent_ids: Option<Vec<String>>,
}

/// Response data for
/// `POST /web/api/v2.1/xdr/assets/actions/fetch-unified-actions`
/// ("Get Available Actions by Asset/Entity Type").
///
/// Spec definition: `AvailableActionsResponse1` (the `data` object of
/// `AvailableActionsResponseSchema`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionsResponse {
    /// Unified actions. Array of [`AvailableAction`]. Optional/nullable in the
    /// spec.
    pub unified_actions: Option<Vec<AvailableAction>>,
}

/// A single available unified action.
///
/// Spec definition: `AvailableAction2`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// Action name. Required (in the spec `required` array, not nullable).
    pub name: String,
    /// Action title. Required (in the spec `required` array, not nullable).
    pub title: String,
    /// Action path. Required (in the spec `required` array, not nullable).
    pub path: String,
    /// Display order. Integer (`int32`). Optional/nullable in the spec.
    pub order: Option<i64>,
    /// Reason the action is disabled, when applicable. Optional/nullable.
    pub disabled_reason: Option<String>,
    /// Whether the action is currently disabled. Optional/nullable in the spec.
    pub is_disabled: Option<bool>,
}
