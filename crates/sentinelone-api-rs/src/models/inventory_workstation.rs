//! Models for the `Inventory Workstation` tag
//! (Inventory Workstation Resource Filters).
//!
//! The SentinelOne spec models the main workstation asset entity
//! (`WorkstationResponse`) as an open (free-form) object with no declared
//! properties, so [`Workstation`] captures every field in a flattened map for
//! forward-compatibility. The filter-metadata entities are fully typed.

use std::collections::HashMap;

use serde::Deserialize;

/// A single workstation asset row.
///
/// The spec models this entity (`WorkstationResponse`) as an open object with
/// no declared properties, so every field is captured in [`extra`](Self::extra)
/// to remain forward-compatible with the API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workstation {
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

/// A single available action with its enabled/disabled status (`AvailableAction1`).
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

/// A single filter-count entry (`CountFiltersResponse`), returned by the
/// `filters/count` endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountFiltersResponse {
    /// A list of filter values with their count. Optional/nullable.
    pub values: Option<Vec<FilterCountValue>>,
    /// A flag to disable the UI filter values sorting by counts, and instead
    /// display values in the provided order. Optional/nullable.
    pub disable_sorting: Option<bool>,
    /// Filter argument key. Optional/nullable.
    pub key: Option<String>,
    /// Filter description. Optional/nullable.
    pub title: Option<String>,
    /// Whether the negation query is enabled for this filter key.
    /// Optional/nullable.
    pub enable_negation: Option<bool>,
}

/// A single filter value with its occurrence count (`FilterCountValue`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCountValue {
    /// Value description. Optional/nullable.
    pub title: Option<String>,
    /// Value. Optional/nullable.
    pub value: Option<String>,
    /// Number of entities matching this value. Optional/nullable.
    pub count: Option<i64>,
}

/// A single free-text filter descriptor (`FreeTextFilterResponse`), returned by
/// the `filters/free-text` endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeTextFilterResponse {
    /// A regular expression for values validation. Optional/nullable.
    pub validation: Option<String>,
    /// Filter argument key. Optional/nullable.
    pub key: Option<String>,
    /// Filter icon. Optional/nullable.
    pub icon: Option<String>,
    /// Filter description. Optional/nullable.
    pub title: Option<String>,
    /// API path for auto-complete query (if applicable). Optional/nullable.
    pub auto_complete: Option<String>,
}

/// Auto-complete suggestions payload (`AutoCompleteResponse`), returned by the
/// `filters/autocomplete` endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteResponse {
    /// Filter description. Optional/nullable.
    pub title: Option<String>,
    /// Auto complete values. Optional/nullable.
    pub values: Option<Vec<AutoCompleteValue>>,
    /// Filter argument key. Optional/nullable.
    pub key: Option<String>,
}

/// A single auto-complete value with its occurrence count (`Values`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteValue {
    /// Value. Optional/nullable.
    pub value: Option<String>,
    /// Number of occurrences. Optional/nullable.
    pub count: Option<i64>,
}
