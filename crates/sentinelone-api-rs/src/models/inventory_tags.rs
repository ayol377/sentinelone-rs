//! Entity models for the `Inventory Tags` tag.
//!
//! Hand-written at 1:1 parity with `swagger_2_1.json`. Per the "default null"
//! rule, every field below is `Option<T>` because none of the source schemas
//! declare a `required` array for the entity objects (and the `_many` envelopes
//! only mark `pagination` as required, which lives in the shared
//! [`crate::pagination`] envelope).

use serde::Deserialize;

/// A unique inventory tag.
///
/// Returned by `GET /web/api/v2.1/xdr/assets/tags` (entity: `TagsResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// Id.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub id: Option<String>,
    /// Key.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub key: Option<String>,
    /// Value.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub value: Option<String>,
    /// Source.
    ///
    /// Allowed values (per the related query enum): `System`, `ThirdParty`,
    /// `User`. Kept as `String` for forward-compatibility. Nullable -> `Option`.
    pub source: Option<String>,
    /// Read only.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub read_only: Option<bool>,
    /// Reserved.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub reserved: Option<bool>,
    /// Asset count.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub asset_count: Option<i64>,
    /// List of Account IDs to filter by.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub group_ids: Option<Vec<String>>,
}

/// The asset count for a given tag id.
///
/// Returned by `POST /web/api/v2.1/xdr/assets/tags/count`
/// (entity: `InventoryTagsCountResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryTagsCount {
    /// Tag id.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub tag_id: Option<String>,
    /// Asset count.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub asset_count: Option<i64>,
    /// List of Account IDs to filter by.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub group_ids: Option<Vec<String>>,
}

/// Tags info for an asset.
///
/// Returned by `POST /web/api/v2.1/xdr/assets/fetch-tags`
/// (entity: `TagsInfoResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagsInfo {
    /// Id.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub id: Option<String>,
    /// Key.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub key: Option<String>,
    /// Value.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub value: Option<String>,
    /// List of Account IDs to filter by.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub group_ids: Option<Vec<String>>,
}

/// A single filter value with its matching-entity count.
///
/// Nested under [`CountFilters::values`] (entity: `FilterCountValue`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCountValue {
    /// Value.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub value: Option<String>,
    /// Value description.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub title: Option<String>,
    /// Number of entities matching this value.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub count: Option<i64>,
}

/// Asset-tags filter counts for a single filter key.
///
/// Returned by `GET /web/api/v2.1/xdr/assets/tags/filters-count`
/// (entity: `CountFiltersResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountFilters {
    /// Filter argument key.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub key: Option<String>,
    /// Filter description.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub title: Option<String>,
    /// A list of filter values with their count.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub values: Option<Vec<FilterCountValue>>,
    /// A flag to disable the UI filter values sorting by counts, and instead
    /// display values in the provided order.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub disable_sorting: Option<bool>,
    /// This flag indicates whether the negation query is enabled for this
    /// filter key or not.
    ///
    /// Nullable/optional in spec -> `Option`.
    pub enable_negation: Option<bool>,
}
