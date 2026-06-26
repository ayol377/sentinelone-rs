//! Response models for the `Filters` tag (saved network filters).

use serde::Deserialize;

/// A saved filter (`filters.filters_FilterViewSchema`).
///
/// Returned by `GET /web/api/v2.1/filters`, `POST /web/api/v2.1/filters`, and
/// `PUT /web/api/v2.1/filters/{filter_id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    /// Id. Required.
    pub id: String,
    /// Created at (date-time string, e.g. `2018-02-27T04:49:26.257525Z`).
    /// Required.
    pub created_at: String,
    /// Updated at (date-time string). Required.
    pub updated_at: String,
    /// Name. Required.
    pub name: String,
    /// Filter scope. Required. Allowed values: `site`, `account`, `global`.
    pub scope_level: String,
    /// [DEPRECATED] Use `scope_id` instead. Optional/nullable.
    #[serde(default)]
    pub site_id: Option<String>,
    /// Associated site/account. Optional/nullable.
    #[serde(default)]
    pub scope_id: Option<String>,
    /// A set of arguments composing the filter (freeform object).
    /// Optional/nullable.
    #[serde(default)]
    pub filter_fields: Option<serde_json::Value>,
}

/// A saved Deep Visibility filter
/// (`filters.filters_DeepVisibilityFilterViewSchema`).
///
/// Returned by `PUT /web/api/v2.1/filters/dv/{filter_id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepVisibilityFilter {
    /// Id. Required.
    pub id: String,
    /// Created at (date-time string). Required.
    pub created_at: String,
    /// Updated at (date-time string). Required.
    pub updated_at: String,
    /// Name. Required.
    pub name: String,
    /// Filter scope. Required. Allowed values: `site`, `account`, `global`,
    /// `group`, `tenant`.
    pub scope_level: String,
    /// [DEPRECATED] Use `scope_id` instead. Optional/nullable.
    #[serde(default)]
    pub site_id: Option<String>,
    /// Associated site/account. Optional/nullable.
    #[serde(default)]
    pub scope_id: Option<String>,
    /// A set of arguments composing the filter (freeform object).
    /// Optional/nullable.
    #[serde(default)]
    pub filter_fields: Option<serde_json::Value>,
    /// Notification recipients. Optional/nullable.
    #[serde(default)]
    pub recipients: Option<Vec<String>>,
    /// Notification frequency. Optional/nullable.
    #[serde(default)]
    pub frequency: Option<i64>,
    /// Whether notifications are enabled. Optional/nullable.
    #[serde(default)]
    pub notifications: Option<bool>,
    /// Scope level name. Optional/nullable.
    #[serde(default)]
    pub scope_level_name: Option<String>,
}

/// Result of uploading a CSV filter file
/// (`filters.schemas_CsvFilterViewSchema`).
///
/// Returned by `POST /web/api/v2.1/filters/csv-filter`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CsvFilter {
    /// Number of rows in the uploaded file. Optional/nullable.
    #[serde(default)]
    pub rows_count: Option<i64>,
    /// Number of unique input values. Optional/nullable.
    #[serde(default)]
    pub unique_input_values_count: Option<i64>,
    /// Number of endpoints found. Optional/nullable.
    #[serde(default)]
    pub endpoint_found_count: Option<i64>,
    /// Number of active endpoints found. Optional/nullable.
    #[serde(default)]
    pub active_endpoint_found_count: Option<i64>,
    /// Endpoints that were not found. Optional/nullable.
    #[serde(default)]
    pub not_found_endpoints: Option<Vec<String>>,
    /// Active endpoints that were not found. Optional/nullable.
    #[serde(default)]
    pub not_found_active_endpoints: Option<Vec<String>>,
    /// The created filter ID. Optional/nullable.
    #[serde(default)]
    pub filter_id: Option<String>,
}

/// An XDR saved filter (`v2_1.config.schemas_FilterViewSchema` -> `FilterView`).
///
/// Returned by `POST /web/api/v2.1/xdr/filters` and
/// `PUT /web/api/v2.1/xdr/filters/{filter_id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XdrFilter {
    /// Id. Required.
    pub id: String,
    /// Created at (date-time string). Required.
    pub created_at: String,
    /// Updated at (date-time string). Required.
    pub updated_at: String,
    /// Name. Required.
    pub name: String,
    /// Filter scope. Required. Allowed values: `site`, `account`, `global`.
    pub scope_level: String,
    /// Management ID. Optional/nullable.
    #[serde(default)]
    pub mgmt_id: Option<String>,
    /// Associated site/account. Optional/nullable.
    #[serde(default)]
    pub scope_id: Option<String>,
    /// A set of arguments composing the filter (freeform object).
    /// Optional/nullable.
    #[serde(default)]
    pub filter_fields: Option<serde_json::Value>,
}

/// A single enriched filter field value (`FilterFieldValue`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XdrFilterFieldValue {
    /// The filter argument title. Optional/nullable.
    #[serde(default)]
    pub title: Option<String>,
    /// The filter argument value. Optional/nullable.
    #[serde(default)]
    pub value: Option<String>,
}

/// A single enriched filter field (`FilterField`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XdrFilterField {
    /// The filter field title. Optional/nullable.
    #[serde(default)]
    pub title: Option<String>,
    /// The filter field key. Optional/nullable.
    #[serde(default)]
    pub key: Option<String>,
    /// Whether this field is free text. Optional/nullable.
    #[serde(default)]
    pub free_text: Option<bool>,
    /// The filter field values. Optional/nullable.
    #[serde(default)]
    pub values: Option<Vec<XdrFilterFieldValue>>,
}

/// An enriched saved filter, with metadata
/// (`v2_1.config.schemas_FilterEnrichedSchema` -> `FilterEnriched`).
///
/// Returned by `GET /web/api/v2.1/xdr/filters` and
/// `GET /web/api/v2.1/xdr/private/filters/enriched`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XdrFilterEnriched {
    /// Id. Required.
    pub id: String,
    /// Created at (date-time string). Required.
    pub created_at: String,
    /// Updated at (date-time string). Required.
    pub updated_at: String,
    /// Name. Required.
    pub name: String,
    /// Filter scope. Required. Allowed values: `site`, `account`, `global`.
    pub scope_level: String,
    /// Management ID. Optional/nullable.
    #[serde(default)]
    pub mgmt_id: Option<String>,
    /// Associated site/account. Optional/nullable.
    #[serde(default)]
    pub scope_id: Option<String>,
    /// The enriched set of arguments composing the filter. Optional/nullable.
    #[serde(default)]
    pub filter_fields: Option<Vec<XdrFilterField>>,
}

/// Generic success response (`SuccessResponse`).
///
/// Returned by the delete endpoints
/// (`DELETE /web/api/v2.1/filters/{filter_id}`,
/// `DELETE /web/api/v2.1/filters/dv/{filter_id}`,
/// `DELETE /web/api/v2.1/xdr/filters/{filter_id}`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterSuccess {
    /// Indicates a successful operation. Optional/nullable.
    #[serde(default)]
    pub success: Option<bool>,
}
