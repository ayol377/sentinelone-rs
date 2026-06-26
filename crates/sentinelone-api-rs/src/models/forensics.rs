//! Models for the `Forensics` tag.
//!
//! Field nullability mirrors the SentinelOne 2.1 spec: a field is a bare type
//! only when it is in the schema `required` array *and* not `x-nullable`;
//! otherwise it is `Option<T>` (the API's "default null" behaviour). None of
//! the Forensics response schemas declare a `required` array, so every field
//! below is `Option<T>`.
//!
//! All Forensics endpoints are **DEPRECATED**.

use serde::Deserialize;

// ===========================================================================
// GET /web/api/v2.1/applications/{application_id}/forensics
// (forensics_ApplicationContentFields_200)
// ===========================================================================

/// `data` wrapper returned by
/// `GET /web/api/v2.1/applications/{application_id}/forensics`.
///
/// This endpoint uses a non-standard envelope: the response `data` is an
/// object `{ success, result }` rather than a bare entity or array.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationForensicsData {
    /// Success flag. Optional/nullable per spec.
    pub success: Option<bool>,
    /// The forensics result payload. Optional/nullable per spec.
    pub result: Option<ApplicationForensicsResult>,
}

/// `result` object for [`ApplicationForensicsData`].
///
/// No field is in the schema `required` array, so every field is `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationForensicsResult {
    /// Application id.
    pub application_id: Option<String>,
    /// Application created (date/time string, e.g. `"2018-02-27T04:49:26.257525Z"`).
    pub application_created: Option<String>,
    /// Fetch story status.
    pub fetch_story_status: Option<String>,
    /// Agent.
    pub agent: Option<String>,
    /// File details.
    pub file: Option<ForensicsFile>,
    /// Process details.
    pub process: Option<ForensicsProcess>,
    /// Process created at (date/time string, e.g. `"2018-02-27T04:49:26.257525Z"`).
    pub process_created_at: Option<String>,
    /// Process display name.
    pub process_display_name: Option<String>,
    /// Seen on network.
    pub seen_on_network: Option<i64>,
    /// Malicious process arguments.
    pub malicious_process_arguments: Option<String>,
}

/// File details shared by the forensics result objects.
///
/// No field is in the schema `required` array, so every field is `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsFile {
    /// Content hash.
    pub content_hash: Option<String>,
    /// Created date.
    pub created_date: Option<String>,
    /// Display name.
    pub display_name: Option<String>,
    /// Is system.
    pub is_system: Option<bool>,
    /// Object id.
    pub object_id: Option<String>,
    /// Path.
    pub path: Option<String>,
    /// Permission.
    pub permission: Option<String>,
    /// Size.
    pub size: Option<i64>,
}

/// Process details shared by the forensics result objects.
///
/// No field is in the schema `required` array, so every field is `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsProcess {
    /// Bundle id.
    pub bundle_id: Option<String>,
    /// Created date.
    pub created_date: Option<String>,
    /// Display name.
    pub display_name: Option<String>,
    /// Executable file id.
    pub executable_file_id: Option<String>,
    /// Is primary.
    pub is_primary: Option<bool>,
    /// Is root.
    pub is_root: Option<bool>,
    /// Object id.
    pub object_id: Option<String>,
    /// Pid.
    pub pid: Option<i64>,
    /// Username.
    pub username: Option<String>,
}

// ===========================================================================
// GET /web/api/v2.1/applications/{application_id}/forensics/details
// (forensics_ApplicationDetailsContentFields_200)
// ===========================================================================

/// `data` wrapper returned by
/// `GET /web/api/v2.1/applications/{application_id}/forensics/details`.
///
/// Non-standard envelope: the response `data` is an object `{ success, result }`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationForensicsDetailsData {
    /// Success flag. Optional/nullable per spec.
    pub success: Option<bool>,
    /// The detailed forensics result payload. Optional/nullable per spec.
    pub result: Option<ApplicationForensicsDetailsResult>,
}

/// `result` object for [`ApplicationForensicsDetailsData`].
///
/// No field is in the schema `required` array, so every field is `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationForensicsDetailsResult {
    /// Agent.
    pub agent: Option<String>,
    /// Application created (date/time string, e.g. `"2018-02-27T04:49:26.257525Z"`).
    pub application_created: Option<String>,
    /// Application duration.
    pub application_duration: Option<String>,
    /// Application id.
    pub application_id: Option<String>,
    /// Category scores (freeform; no declared type in the spec).
    pub category_scores: Option<serde_json::Value>,
    /// Fetch story error at (date/time string, e.g. `"2018-02-27T04:49:26.257525Z"`).
    pub fetch_story_error_at: Option<String>,
    /// Fetch story sent at (date/time string, e.g. `"2018-02-27T04:49:26.257525Z"`).
    pub fetch_story_sent_at: Option<String>,
    /// Fetch story status.
    pub fetch_story_status: Option<String>,
    /// File details.
    pub file: Option<ForensicsFile>,
    /// Graph (freeform; no declared type in the spec).
    pub graph: Option<serde_json::Value>,
    /// Last event seen at (date/time string, e.g. `"2018-02-27T04:49:26.257525Z"`).
    pub last_event_seen_at: Option<String>,
    /// Process details.
    pub process: Option<ForensicsProcess>,
    /// Process created at (date/time string, e.g. `"2018-02-27T04:49:26.257525Z"`).
    pub process_created_at: Option<String>,
    /// Process display name.
    pub process_display_name: Option<String>,
    /// Raw data (freeform; no declared type in the spec).
    pub raw_data: Option<serde_json::Value>,
    /// Seen on network.
    pub seen_on_network: Option<i64>,
    /// Summary (freeform; no declared type in the spec).
    pub summary: Option<serde_json::Value>,
    /// Summary overview (file/network/registry event counts).
    pub summary_overview: Option<ForensicsSummaryOverview>,
}

/// `summary_overview` object for [`ApplicationForensicsDetailsResult`].
///
/// No field is in the schema `required` array, so every field is `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsSummaryOverview {
    /// File event counts.
    pub file: Option<ForensicsSummaryFile>,
    /// Network event counts.
    pub network: Option<ForensicsSummaryNetwork>,
    /// Registry event counts.
    pub registry: Option<ForensicsSummaryRegistry>,
}

/// File event counts for [`ForensicsSummaryOverview`].
///
/// No field is in the schema `required` array, so every field is `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsSummaryFile {
    /// Create count.
    pub create: Option<i64>,
    /// Write count.
    pub write: Option<i64>,
    /// Delete count.
    pub delete: Option<i64>,
}

/// Network event counts for [`ForensicsSummaryOverview`].
///
/// No field is in the schema `required` array, so every field is `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsSummaryNetwork {
    /// Connections count.
    pub connections: Option<i64>,
    /// DNS count.
    pub dns: Option<i64>,
}

/// Registry event counts for [`ForensicsSummaryOverview`].
///
/// No field is in the schema `required` array, so every field is `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsSummaryRegistry {
    /// Persistence count.
    pub persistence: Option<i64>,
    /// Security count.
    pub security: Option<i64>,
    /// Stealth count.
    pub stealth: Option<i64>,
}

// ===========================================================================
// GET /web/api/v2.1/applications/{application_id}/forensics/connections
// (forensics_ConnectionsSchema_200)
// ===========================================================================

/// `data` wrapper returned by
/// `GET /web/api/v2.1/applications/{application_id}/forensics/connections`.
///
/// Non-standard envelope: the response `data` is an object that itself contains
/// an inner `data` array of freeform connection objects. **[DEPRECATED] Returns
/// an empty array.**
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationConnectionsData {
    /// The connections array (freeform objects; no declared item type in the
    /// spec). Optional/nullable per spec.
    pub data: Option<Vec<serde_json::Value>>,
}
