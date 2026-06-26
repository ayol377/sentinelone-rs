//! Models for the `Datalake Unified Actions` tag.

use serde::Deserialize;

/// Response data for `POST /web/api/v2.1/xdr/action-controller/fetch-unified-actions`.
///
/// Wraps the list of available unified actions for the requested asset/entity
/// types. Returned inside the standard `{ data, errors }` envelope.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionsResponse {
    /// Unified actions available for the affected entities.
    ///
    /// Optional/nullable in the spec (not in `required`) -> `Option`, default null.
    #[serde(default)]
    pub unified_actions: Option<Vec<AvailableAction>>,
}

/// A single available unified action.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// Action name. Required, not nullable -> bare `String`.
    pub name: String,
    /// Action path (identifier used when performing the action).
    /// Required, not nullable -> bare `String`.
    pub path: String,
    /// Human-readable action title. Required, not nullable -> bare `String`.
    pub title: String,
    /// Menu path under which the action is grouped.
    /// Optional/nullable -> `Option`, default null.
    #[serde(default)]
    pub menu_path: Option<String>,
    /// Reason the action is disabled, when applicable.
    /// Optional/nullable -> `Option`, default null.
    #[serde(default)]
    pub disabled_reason: Option<String>,
    /// Ordering hint for display. Optional/nullable -> `Option`, default null.
    #[serde(default)]
    pub order: Option<i64>,
    /// Whether the action is currently disabled.
    /// Optional/nullable -> `Option`, default null.
    #[serde(default)]
    pub is_disabled: Option<bool>,
}
