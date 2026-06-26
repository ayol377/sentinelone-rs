//! Models for the `Tag Manager` tag.
//!
//! Response entity types for the SentinelOne Management Tag Manager API.
//! Field nullability follows the spec: a field is bare `T` only when it is in
//! the schema `required` array and not `x-nullable`; otherwise it is
//! `Option<T>` (the "default null" behaviour). The `data` object of
//! `TagsViewSchema_200` declares no `required` array, so every field of [`Tag`]
//! is optional.

use serde::Deserialize;

/// An endpoint tag, as returned by the create (`POST`) and edit (`PUT`)
/// Tag Manager endpoints (the `data` object of `TagsViewSchema_200`).
///
/// All fields are optional/nullable: the spec declares no `required` array on
/// the response `data` object.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// Tag ID. Example: `"225494730938493804"`.
    pub id: Option<String>,
    /// Key.
    pub key: Option<String>,
    /// Value.
    pub value: Option<String>,
    /// Type. e.g: `manual-tagging`.
    #[serde(rename = "type")]
    pub tag_type: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Name of scope.
    pub scope_level: Option<String>,
    /// An ID reference to the containing scope.
    /// Example: `"225494730938493804"`.
    pub scope_id: Option<String>,
    /// A reference to the user which created the tag.
    /// Example: `"225494730938493804"`.
    pub created_by_id: Option<String>,
    /// A reference to the user which updated the tag.
    /// Example: `"225494730938493804"`.
    pub updated_by_id: Option<String>,
    /// Created at (date-time string).
    /// Example: `"2018-02-27T04:49:26.257525Z"`.
    pub created_at: Option<String>,
    /// Updated at (date-time string).
    /// Example: `"2018-02-27T04:49:26.257525Z"`.
    pub updated_at: Option<String>,
}

/// The `data` object of `_AffectedResultsSchema_200`, returned by the delete
/// (`DELETE`) Tag Manager endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
}
