//! Models for the `overview` tag.
//!
//! Field nullability mirrors the SentinelOne `swagger_2_1.json` spec: a field is
//! a bare type only when it appears in the schema `required` array and is not
//! `x-nullable`; otherwise it is `Option<T>` (the spec's "default null"
//! behaviour). Enum-valued strings are kept as `String` for forward
//! compatibility, with the allowed values documented on each field.

use serde::Deserialize;

/// Cloud inventory resource overview
/// (`InventoryOverviewResponse` in the spec).
///
/// Returned as the `data` field of
/// `POST /web/api/v2.1/xdr/assets/overview`. The spec marks no field as
/// required, so every field is `Option`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryOverview {
    /// Properties (`InventoryAllCategoriesResponse` in the spec).
    ///
    /// The spec defines this as a free-form object with no declared
    /// properties, so it is kept as a `serde_json::Value` to preserve
    /// whatever the API returns. Optional/nullable.
    pub properties: Option<serde_json::Value>,
}
