//! Entity models for the `Graph Query Builder` tag.
//!
//! Field nullability mirrors `swagger_2_1.json`: a field is a bare `T` only
//! when it is in the schema `required` array and not `x-nullable`; otherwise it
//! is `Option<T>` (the API's "default null" behaviour). Enum-typed strings are
//! kept as `String` for forward-compatibility; allowed values are documented on
//! the relevant field.

use serde::Deserialize;

/// Graph query builder initial metadata.
///
/// Returned by `GET /web/api/v2.1/xdr/graph-explorer/query/builder/metadata`
/// and `GET /web/api/v2.1/xdr/assets/query/builder/metadata`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQueryBuilderMetadata {
    /// Categories. Optional / nullable.
    pub categories: Option<Vec<GraphQueryBuilderCategory>>,
}

/// A top-level category in the query builder metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQueryBuilderCategory {
    /// Whether the category is marked as popular. Optional / nullable.
    pub is_popular: Option<bool>,
    /// Sub-categories. Optional / nullable.
    pub sub_categories: Option<Vec<GraphQueryBuilderSubCategory>>,
    /// Category id. Optional / nullable.
    pub id: Option<String>,
    /// Category name. Optional / nullable.
    pub name: Option<String>,
}

/// A sub-category nested inside a [`GraphQueryBuilderCategory`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQueryBuilderSubCategory {
    /// Whether the sub-category is marked as popular. Optional / nullable.
    pub is_popular: Option<bool>,
    /// Asset types under this sub-category. Optional / nullable.
    pub asset_types: Option<Vec<GraphQueryBuilderAssetType>>,
    /// Sub-category id. Optional / nullable.
    pub id: Option<String>,
    /// Sub-category name. Optional / nullable.
    pub name: Option<String>,
}

/// An asset type nested inside a [`GraphQueryBuilderSubCategory`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQueryBuilderAssetType {
    /// Whether the asset type is marked as popular. Optional / nullable.
    pub is_popular: Option<bool>,
    /// Asset type id. Optional / nullable.
    pub id: Option<String>,
    /// Asset type name. Optional / nullable.
    pub name: Option<String>,
}

/// Query builder metadata for requested cloud asset types.
///
/// Returned by
/// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/metadata/available-options/v2`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAvailableOptionMetadataSchemaV2 {
    /// Asset properties. Optional / nullable.
    pub properties: Option<Vec<AssetPropertySchemaV2>>,
    /// Relations. Optional / nullable.
    pub relations: Option<Vec<GraphQueryBuilderRelations>>,
}

/// An asset property descriptor for a requested asset type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetPropertySchemaV2 {
    /// Supported filter operators for this property. Optional / nullable.
    pub supported_filters: Option<Vec<String>>,
    /// Property name. Optional / nullable.
    pub name: Option<String>,
    /// Whether this property is an enum. Optional / nullable.
    pub is_enum: Option<bool>,
    /// Property id. Optional / nullable.
    pub id: Option<String>,
    /// Property data type. Optional / nullable.
    pub data_type: Option<String>,
}

/// A relation descriptor for a requested asset type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQueryBuilderRelations {
    /// Relation id. Optional / nullable.
    pub id: Option<String>,
    /// Singular entities reachable via this relation. Optional / nullable.
    pub singular_entities: Option<Vec<BasicQueryBuilderSchemaItem>>,
    /// Relation name. Optional / nullable.
    pub name: Option<String>,
    /// Categorized entities reachable via this relation. Optional / nullable.
    pub categorized_entities: Option<Vec<CategorizedEntities>>,
}

/// A basic id/name pair used throughout the relations metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicQueryBuilderSchemaItem {
    /// Entity id. Optional / nullable.
    pub id: Option<String>,
    /// Entity name. Optional / nullable.
    pub name: Option<String>,
}

/// A categorized group of entities reachable via a relation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategorizedEntities {
    /// Categories of entities. Optional / nullable.
    pub categories: Option<Vec<RelationSubCategory>>,
    /// Categorized-entities id. Optional / nullable.
    pub id: Option<String>,
    /// Categorized-entities name. Optional / nullable.
    pub name: Option<String>,
}

/// A relation category containing sub-categories.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationSubCategory {
    /// Sub-categories. Optional / nullable.
    pub sub_categories: Option<Vec<RelationAssetType>>,
    /// Category id. Optional / nullable.
    pub id: Option<String>,
    /// Category name. Optional / nullable.
    pub name: Option<String>,
}

/// A relation sub-category containing asset types.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RelationAssetType {
    /// Asset types under this sub-category. Optional / nullable.
    pub asset_types: Option<Vec<BasicQueryBuilderSchemaItem>>,
    /// Sub-category id. Optional / nullable.
    pub id: Option<String>,
    /// Sub-category name. Optional / nullable.
    pub name: Option<String>,
}

/// Auto-complete suggestions for a query builder field.
///
/// Returned by
/// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/autocomplete` and
/// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/autocomplete/v2`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteResponse {
    /// Filter description (e.g. `Computer name`). Optional / nullable.
    pub title: Option<String>,
    /// Auto-complete values. Optional / nullable.
    pub values: Option<Vec<AutoCompleteValue>>,
    /// Filter argument key (e.g. `computerName__contains`). Optional / nullable.
    pub key: Option<String>,
}

/// A single auto-complete value with an occurrence count.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteValue {
    /// The value (e.g. `JohnD_WORKSTATION`). Optional / nullable.
    pub value: Option<String>,
    /// Number of occurrences. Optional / nullable.
    pub count: Option<i64>,
}

/// Tag auto-complete suggestions for keys or values.
///
/// Returned by
/// `POST /web/api/v2.1/xdr/graph-explorer/query/builder/tag/autocomplete`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagAutoCompleteResponse {
    /// Tag filtered based on key or value (e.g. `key`). Optional / nullable.
    pub title: Option<String>,
    /// Suggested values. Optional / nullable.
    pub values: Option<Vec<String>>,
}

/// Feature toggles for graph services.
///
/// Returned by `GET /web/api/v2.1/xdr/private/graph-services-features`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphFeatureToggleResponse {
    /// Graph feature toggles. The spec declares no concrete shape (read-only),
    /// so the value is exposed as an opaque [`serde_json::Value`].
    /// Optional / nullable.
    pub graph_feature_toggles: Option<serde_json::Value>,
}

/// An ontology relation between graph entities.
///
/// Returned (as an array) by
/// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/metadata/available-relations`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OntologyRelations {
    /// Inverse. Required.
    pub inverse: String,
    /// Id. Required.
    pub id: String,
    /// Name. Required.
    pub name: String,
    /// Inverse id. Required.
    pub inverse_id: String,
}
