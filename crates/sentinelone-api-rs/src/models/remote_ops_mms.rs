//! `Remote Ops MMS` entity models.
//!
//! Field nullability mirrors `swagger_2_1.json`: a field is a bare type only
//! when it is in the schema `required` array and not `x-nullable`; everything
//! else is `Option<T>` (the SentinelOne "default null" behaviour). Enum-valued
//! fields are kept as `String` for forward-compatibility; the allowed values
//! are documented in each field's doc comment. Date/time fields are `String`.

use serde::Deserialize;

/// A data-exporter Destination profile.
///
/// Returned by
/// `GET /web/api/v2.1/remote-ops/data-exporter/destination-profiles` (list) and
/// `GET /web/api/v2.1/remote-ops/data-exporter/destination-profiles/{profile_id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinationProfile {
    /// Destination profile ID. Required, non-nullable.
    pub id: String,
    /// Destination profile name. Required, non-nullable.
    pub name: String,
    /// Path of scope where the Destination profile is stored. Required,
    /// non-nullable.
    pub scope_path: String,
    /// Write key of destination account to upload data. Required, non-nullable.
    pub api_key: String,
    /// URL of destination instance to upload data. Required, non-nullable.
    pub api_url: String,
    /// Flag if the Destination profile is default for the scope. Required,
    /// non-nullable.
    pub is_default: bool,
    /// Email of user who created the Destination profile. Required but
    /// nullable.
    pub creator: Option<String>,
    /// ID of user who created the Destination profile. Required but nullable.
    pub creator_id: Option<String>,
    /// Email of user who updated the Destination profile. Required but
    /// nullable.
    pub updater: Option<String>,
    /// ID of user who updated the Destination profile. Required but nullable.
    pub updater_id: Option<String>,
    /// Destination type where the results will be uploaded. Required but
    /// nullable.
    pub destination: Option<String>,
}

/// Response data for creating/updating a Destination profile.
///
/// Returned by `POST` and `PUT`
/// `/web/api/v2.1/remote-ops/data-exporter/destination-profiles`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileIdResult {
    /// Profile ID. Required, non-nullable.
    pub profile_id: String,
}

/// Response data for deleting multiple Destination profiles.
///
/// Returned by
/// `DELETE /web/api/v2.1/remote-ops/data-exporter/destination-profiles`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedDestinationProfilesResult {
    /// Deleted Destination profile IDs. Optional.
    pub affected: Option<Vec<String>>,
}

/// Results uploaded to the data exporter for a single result kind (task,
/// threat or agent).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkylightUploadResultEntry {
    /// Indicates if no results were returned. Required, non-nullable.
    pub is_empty: bool,
    /// Indicates if a failure occurred during upload. Required, non-nullable.
    pub has_failures: bool,
    /// Url to relevant data source. Required, non-nullable.
    pub url: String,
    /// Last error message. Optional.
    pub error_message: Option<String>,
    /// Query filter to find the result. Optional.
    pub query: Option<String>,
    /// Starting point of the results timeframe (date-time). Optional.
    pub start_time: Option<String>,
    /// Ending point of the results timeframe (date-time). Optional.
    pub end_time: Option<String>,
}

/// Results sent to the data exporter.
///
/// Returned by `GET /web/api/v2.1/remote-ops/data-exporter/results`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkylightUploadResults {
    /// Task results. Optional.
    pub task_results: Option<SkylightUploadResultEntry>,
    /// Threat results. Optional.
    pub threat_results: Option<SkylightUploadResultEntry>,
    /// Agent results. Optional.
    pub agent_results: Option<SkylightUploadResultEntry>,
}

/// Response data for scheduling a remote script.
///
/// Returned by `POST /web/api/v2.1/remote-ops/schedule/remote-script`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRemoteScriptResult {
    /// Scheduled Task ID. Required, non-nullable.
    pub scheduled_task_id: String,
    /// Number of affected endpoints that matched the filter. Required,
    /// non-nullable.
    pub affected: i64,
    /// Pending execution id. Optional.
    pub pending_execution_id: Option<String>,
}

/// Response data for scheduling a forensics collection.
///
/// Returned by `POST /web/api/v2.1/remote-ops/schedule/forensics`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleForensicsResult {
    /// Scheduled Task ID. Required, non-nullable.
    pub scheduled_task_id: String,
    /// Number of affected endpoints that matched the filter. Required,
    /// non-nullable.
    pub affected: i64,
}

/// Response data for deleting multiple scheduled tasks.
///
/// Returned by `DELETE /web/api/v2.1/remote-ops/scheduled-tasks`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeletedScheduledTasksResult {
    /// Deleted Scheduled task IDs. Optional.
    pub affected: Option<Vec<String>>,
}

/// Response data for updating a scheduled task.
///
/// Returned by `PUT /web/api/v2.1/remote-ops/scheduled-tasks/{scheduled_task_id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledTaskIdResult {
    /// Scheduled Task ID. Required, non-nullable.
    pub scheduled_task_id: String,
}

/// A timezone-aware timestamp used by scheduled tasks.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledTaskTime {
    /// Timezone. Required, non-nullable.
    pub tz: String,
    /// UTC timestamp (date-time). Required, non-nullable.
    pub utc: String,
}

/// Recurrence configuration of a scheduled task.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledTaskRecurrence {
    /// The unit of the recurrence. Optional/nullable.
    /// Allowed values: `day`, `week`, `month`, `year`.
    pub unit: Option<String>,
    /// The day of the month. Accepts only 1 or None. Optional/nullable.
    pub day_of_month: Option<i64>,
    /// The interval of the units. Optional/nullable.
    pub interval: Option<i64>,
}

/// Target agents filter (scopes or IDs) of a scheduled task.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledTaskEndpointsFilter {
    /// Group ids. Optional.
    pub group_ids: Option<Vec<String>>,
    /// Account ids. Optional.
    pub account_ids: Option<Vec<String>>,
    /// Ids. Optional.
    pub ids: Option<Vec<String>>,
    /// Tenant. Optional.
    pub tenant: Option<bool>,
    /// Site ids. Optional.
    pub site_ids: Option<Vec<String>>,
}

/// A scheduled task.
///
/// Returned by `GET /web/api/v2.1/remote-ops/scheduled-tasks` (list).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledTask {
    /// Scheduled task ID. Required, non-nullable.
    pub id: String,
    /// Name. Required, non-nullable.
    pub name: String,
    /// Description. Required, non-nullable.
    pub description: String,
    /// name/email of creating user. Required, non-nullable.
    pub creator: String,
    /// Initiation (creation) time (date-time). Required, non-nullable.
    pub created_at: String,
    /// Latest update time (date-time). Required, non-nullable.
    pub updated_at: String,
    /// Output destination. Required, non-nullable.
    /// Allowed values: `SentinelCloud`, `Local`, `None`, `SingularityXDR`.
    pub output_destination: String,
    /// Scheduled time. Required, non-nullable.
    pub scheduled_time: ScheduledTaskTime,
    /// Status of the task. Required, non-nullable.
    /// Allowed values: `scheduled`, `waiting_approval`, `declined`.
    pub status: String,
    /// List of endpoint names targeted in the current scope. Optional.
    pub target_endpoint_names: Option<Vec<String>>,
    /// Count of targeted endpoints. Optional.
    pub endpoints_count: Option<i64>,
    /// OS types. Optional. Allowed values: `linux`, `windows`, `macos`.
    pub os_types: Option<Vec<String>>,
    /// If a scope is targeted, the user-readable scope path; otherwise null.
    /// Optional.
    pub target_scope_path: Option<String>,
    /// Endpoints filter, JSON-encoded. Optional.
    pub dynamic_endpoints_filter: Option<String>,
    /// Expire time. Optional.
    pub expire_time: Option<ScheduledTaskTime>,
    /// Target agents filter (scopes or IDs). Optional.
    pub endpoints_filter: Option<ScheduledTaskEndpointsFilter>,
    /// Recurrence. Optional.
    pub recurrence: Option<ScheduledTaskRecurrence>,
    /// Name key. Optional.
    pub name_key: Option<String>,
    /// Type. Optional.
    /// Allowed values: `action`, `dataCollection`, `artifactCollection`,
    /// `forensicsProfile`.
    pub r#type: Option<String>,
    /// Number of endpoints that will be targeted in the current scope.
    /// Optional.
    pub target_endpoints_count: Option<i64>,
}
