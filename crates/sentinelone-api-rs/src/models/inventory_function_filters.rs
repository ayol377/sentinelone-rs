//! Models for the `Inventory Function Filters` tag (Inventory Function Resource Filters).
//!
//! Generated for 1:1 parity with the SentinelOne Management API spec
//! (`swagger_2_1.json`). None of these schemas declare a `required` array, so
//! every field is `Option<T>` (the API's default-null behaviour).

use serde::Deserialize;

/// A single filter value together with the number of entities matching it.
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

/// A filter key with its available values and their counts.
///
/// Returned by `GET /web/api/v2.1/xdr/assets/function/filters/count`.
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
    /// This flag indicates whether the negation query is enabled for this
    /// filter key or not. Optional/nullable.
    pub enable_negation: Option<bool>,
}

/// A free-text filter descriptor.
///
/// Returned by `GET /web/api/v2.1/xdr/assets/function/filters/free-text`.
/// Spec definition: `FreeTextFilterResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeTextFilterResponse {
    /// A regular expression for values validation. Optional/nullable.
    pub validation: Option<String>,
    /// Filter argument key (e.g. `computerName__contains`). Optional/nullable.
    pub key: Option<String>,
    /// Filter icon (e.g. `upload`). Optional/nullable.
    pub icon: Option<String>,
    /// Filter description (e.g. `Computer name`). Optional/nullable.
    pub title: Option<String>,
    /// API path for auto-complete query (if applicable). Optional/nullable.
    pub auto_complete: Option<String>,
}

/// A single auto-complete value together with its number of occurrences.
///
/// Spec definition: `Values`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteValue {
    /// Value (e.g. `JohnD_WORKSTATION`). Optional/nullable.
    pub value: Option<String>,
    /// Number of occurrences. Optional/nullable.
    pub count: Option<i64>,
}

/// Auto-complete suggestions for a filter field.
///
/// Returned by `GET /web/api/v2.1/xdr/assets/function/filters/autocomplete`.
/// Spec definition: `AutoCompleteResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteResponse {
    /// Filter description (e.g. `Computer name`). Optional/nullable.
    pub title: Option<String>,
    /// Auto-complete values. Optional/nullable.
    pub values: Option<Vec<AutoCompleteValue>>,
    /// Filter argument key (e.g. `computerName__contains`). Optional/nullable.
    pub key: Option<String>,
}
