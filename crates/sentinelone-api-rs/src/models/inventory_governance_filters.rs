//! Models for the `Inventory Governance Filters` tag (Inventory Governance
//! Resource Filters).
//!
//! All fields are `Option<T>`: none of the underlying spec definitions declare
//! a `required` array, so every property follows the API's "default null"
//! behaviour.

use serde::Deserialize;

/// A single filter value paired with the number of entities matching it.
///
/// Nested under [`CountFiltersResponse::values`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCountValue {
    /// Value description.
    ///
    /// Optional/nullable.
    pub title: Option<String>,
    /// Value.
    ///
    /// Optional/nullable.
    pub value: Option<String>,
    /// Number of entities matching this value.
    ///
    /// Optional/nullable.
    pub count: Option<i64>,
}

/// Filter counts response entity.
///
/// Returned (as a list) from `GET
/// /web/api/v2.1/xdr/assets/governance/filters/count`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountFiltersResponse {
    /// A list of filter values with their count.
    ///
    /// Optional/nullable.
    pub values: Option<Vec<FilterCountValue>>,
    /// A flag to disable the UI filter values sorting by counts, and instead
    /// display values in the provided order.
    ///
    /// Optional/nullable.
    pub disable_sorting: Option<bool>,
    /// Filter argument key.
    ///
    /// Optional/nullable.
    pub key: Option<String>,
    /// Filter description.
    ///
    /// Optional/nullable.
    pub title: Option<String>,
    /// This flag indicates whether the negation query is enabled for this
    /// filter key or not.
    ///
    /// Optional/nullable.
    pub enable_negation: Option<bool>,
}

/// Free-text filter response entity.
///
/// Returned (as a list) from `GET
/// /web/api/v2.1/xdr/assets/governance/filters/free-text`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeTextFilterResponse {
    /// A regular expression for values validation.
    ///
    /// Optional/nullable.
    pub validation: Option<String>,
    /// Filter argument key (e.g. `computerName__contains`).
    ///
    /// Optional/nullable.
    pub key: Option<String>,
    /// Filter icon (e.g. `upload`).
    ///
    /// Optional/nullable.
    pub icon: Option<String>,
    /// Filter description (e.g. `Computer name`).
    ///
    /// Optional/nullable.
    pub title: Option<String>,
    /// API path for auto-complete query (if applicable).
    ///
    /// Optional/nullable.
    pub auto_complete: Option<String>,
}

/// A single auto-complete value paired with its occurrence count.
///
/// Nested under [`AutoCompleteResponse::values`]. Mirrors the spec `Values`
/// definition; named entity-prefixed to avoid a bare `Values` type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteValue {
    /// Value (e.g. `JohnD_WORKSTATION`).
    ///
    /// Optional/nullable.
    pub value: Option<String>,
    /// Number of occurrences.
    ///
    /// Optional/nullable.
    pub count: Option<i64>,
}

/// Auto-complete response entity.
///
/// Returned (as a single object) from `GET
/// /web/api/v2.1/xdr/assets/governance/filters/autocomplete`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteResponse {
    /// Filter description (e.g. `Computer name`).
    ///
    /// Optional/nullable.
    pub title: Option<String>,
    /// Auto-complete values.
    ///
    /// Optional/nullable.
    pub values: Option<Vec<AutoCompleteValue>>,
    /// Filter argument key (e.g. `computerName__contains`).
    ///
    /// Optional/nullable.
    pub key: Option<String>,
}
