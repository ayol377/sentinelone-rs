//! Models for the `Default Reports` tag.
//!
//! Field nullability follows the spec: a field is a bare `T` only when it is in
//! the schema `required` array and not `x-nullable`; otherwise it is `Option<T>`
//! (default-null behaviour). Enum-typed fields are kept as `String` for
//! forward-compatibility; allowed values are documented inline.

use serde::Deserialize;

/// A generated default report (`GET /web/api/v2.1/reports`).
///
/// None of these fields are marked `required` in the spec, so all are
/// `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    /// Id.
    pub id: Option<String>,
    /// Name of the report.
    pub name: Option<String>,
    /// Scope of the report.
    pub scope: Option<String>,
    /// Report frequency.
    pub frequency: Option<String>,
    /// Interval of the report. Free-form / read-only value.
    pub interval: Option<serde_json::Value>,
    /// Report type. Allowed values: `manually`, `scheduled`.
    pub schedule_type: Option<String>,
    /// Id of the creator.
    pub creator_id: Option<String>,
    /// Name of the creator.
    pub creator_name: Option<String>,
    /// Creation date (date-time string).
    pub created_at: Option<String>,
    /// From date (date-time string).
    pub from_date: Option<String>,
    /// To date (date-time string).
    pub to_date: Option<String>,
    /// Report data: list of Insight types (free-form objects).
    pub insight_types: Option<Vec<serde_json::Value>>,
    /// Type of documents for the report.
    pub attachment_types: Option<Vec<String>>,
    /// Status of the report.
    pub status: Option<String>,
    /// Report sites.
    pub sites: Option<String>,
}

/// A default report task (`GET /web/api/v2.1/report-tasks`,
/// `PUT /web/api/v2.1/report-tasks/{task_id}`).
///
/// None of these fields are marked `required` in the spec, so all are
/// `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportTask {
    /// Id.
    pub id: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Scope. Allowed values: `global`, `group`, `account`, `site`.
    pub scope: Option<String>,
    /// Frequency. Allowed values: `manually`, `weekly`, `monthly`.
    pub frequency: Option<String>,
    /// Day. Free-form / read-only value.
    pub day: Option<serde_json::Value>,
    /// Report type. Allowed values: `manually`, `scheduled`.
    pub schedule_type: Option<String>,
    /// Creator id.
    pub creator_id: Option<String>,
    /// Creator name.
    pub creator_name: Option<String>,
    /// Insight types (free-form objects).
    pub insight_types: Option<Vec<serde_json::Value>>,
    /// Type of documents for the report.
    pub attachment_types: Option<Vec<String>>,
    /// Sites associated to the report.
    pub sites: Option<String>,
    /// From date (date-time string).
    pub from_date: Option<String>,
    /// To date (date-time string).
    pub to_date: Option<String>,
    /// Recipients.
    pub recipients: Option<Vec<String>>,
    /// Is trend.
    pub is_trend: Option<bool>,
}

/// Insight report types (`GET /web/api/v2.1/reports/insights/types`).
///
/// No fields are marked `required` in the spec, so all are `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsightData {
    /// List of reports (free-form objects).
    pub insight_types: Option<Vec<serde_json::Value>>,
}

/// Generic success indicator returned by `POST /web/api/v2.1/report-tasks`.
///
/// No fields are marked `required` in the spec, so all are `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Indicates a successful operation.
    pub success: Option<bool>,
}
