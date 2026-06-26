//! Models for the `Inventory Identity Surface Filters` tag (Inventory Identity
//! Surface Resource Filters).
//!
//! None of the underlying spec schemas declare a `required` array, so every
//! field below is `Option<T>` per the "default null" fidelity rule.

use serde::Deserialize;

/// A single auto-complete value with its number of occurrences.
///
/// Spec definition: `Values`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteValue {
    /// Value (e.g. `"JohnD_WORKSTATION"`). Optional/nullable.
    pub value: Option<String>,
    /// Number of occurrences. Optional/nullable.
    pub count: Option<i64>,
}

/// Auto-complete suggestions for a single filter field.
///
/// Returned (as the singular `data` object) by
/// `GET /web/api/v2.1/xdr/assets/surface/identity/filters/autocomplete`.
///
/// Spec definition: `AutoCompleteResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteResponse {
    /// Filter description (e.g. `"Computer name"`). Optional/nullable.
    pub title: Option<String>,
    /// Auto-complete values. Optional/nullable.
    pub values: Option<Vec<AutoCompleteValue>>,
    /// Filter argument key (e.g. `"computerName__contains"`). Optional/nullable.
    pub key: Option<String>,
}

/// A single filter value together with the count of matching entities.
///
/// Spec definition: `FilterCountValue`.
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

/// Filter counts for a single filter key.
///
/// Returned (as elements of the `data` array) by
/// `GET /web/api/v2.1/xdr/assets/surface/identity/filters/count`.
///
/// Spec definition: `CountFiltersResponse`.
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
    /// Whether the negation query is enabled for this filter key or not.
    /// Optional/nullable.
    pub enable_negation: Option<bool>,
}

/// A single free-text filter definition.
///
/// Returned (as elements of the `data` array) by
/// `GET /web/api/v2.1/xdr/assets/surface/identity/filters/free-text`.
///
/// Spec definition: `FreeTextFilterResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeTextFilterResponse {
    /// A regular expression for values validation. Optional/nullable.
    pub validation: Option<String>,
    /// Filter argument key (e.g. `"computerName__contains"`). Optional/nullable.
    pub key: Option<String>,
    /// Filter icon (e.g. `"upload"`). Optional/nullable.
    pub icon: Option<String>,
    /// Filter description (e.g. `"Computer name"`). Optional/nullable.
    pub title: Option<String>,
    /// API path for auto-complete query (if applicable). Optional/nullable.
    pub auto_complete: Option<String>,
}
