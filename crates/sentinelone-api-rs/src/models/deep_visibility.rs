//! Models for the `Deep Visibility` tag.
//!
//! Response envelopes (`data` / `pagination` / `errors`) are stripped by
//! [`crate::pagination::Paginated`] and [`crate::pagination::Response`]; the
//! structs here describe the inner `data` payloads only.
//!
//! Field fidelity follows the spec: a field is a bare `T` only when it is in the
//! schema `required` array and not `x-nullable`; otherwise it is `Option<T>`
//! (default-null behaviour). Spec enums are kept as `String` for forward
//! compatibility, with the allowed values documented inline.

use serde::Deserialize;

/// Query-mode information attached to query-id and query-status responses.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepVisibilityQueryModeInfo {
    /// The query mode (e.g. `presto`). Required.
    pub mode: String,
    /// The query mode `last_activated_at` date (date-time string). Optional/nullable.
    pub last_activated_at: Option<String>,
}

/// `data` payload of `POST /web/api/v2.1/dv/init-query` — the created query id.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepVisibilityQueryId {
    /// A query unique identifier (e.g. `q1xx2xx3`). Required.
    pub query_id: String,
    /// Query mode info. Optional/nullable.
    pub query_mode_info: Option<DeepVisibilityQueryModeInfo>,
}

/// `data` payload of `POST /web/api/v2.1/dv/cancel-query` — request success status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepVisibilitySuccess {
    /// Request success status. Required (returned as a string in the spec).
    pub success: String,
}

/// `data` payload of `GET /web/api/v2.1/dv/query-status` — the query status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepVisibilityQueryStatus {
    /// Response state. Required.
    ///
    /// Allowed values: `PROCESS_RUNNING`, `EVENTS_RUNNING`, `FAILED`,
    /// `FAILED_CLIENT`, `FINISHED`, `RUNNING`, `ERROR`, `QUERY_CANCELLED`,
    /// `TIMED_OUT`, `QUERY_EXPIRED`. (The endpoint doc also lists `EMPTY_RESULTS`,
    /// `PLANNING`, `QUERY_CANCEL`, `QUERY_NOT_FOUND`, `QUERY_RUNNING`.)
    pub response_state: String,
    /// Query loading status in percentage. Required.
    pub progress_status: i64,
    /// Relevant only for `FAILED` and `FAILED_CLIENT` DV errors. Optional/nullable.
    pub response_error: Option<String>,
    /// Query mode info. Optional/nullable.
    pub query_mode_info: Option<DeepVisibilityQueryModeInfo>,
    /// Warnings. Optional/nullable.
    pub warnings: Option<String>,
}

/// `data` payload of `GET /web/api/v2.1/dv/fetch-file` — a file download link.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepVisibilityFileDownloadLink {
    /// Download link. Optional/nullable.
    pub download_url: Option<String>,
    /// File name. Optional/nullable.
    pub file_name: Option<String>,
}

/// `data` payload of the Power Query endpoints
/// (`POST /web/api/v2.1/dv/events/pq` and `GET /web/api/v2.1/dv/events/pq-ping`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepVisibilityPowerQuery {
    /// Query Id (e.g. `q1xx2xx3`). Required.
    pub query_id: String,
    /// External Query Id. Optional/nullable.
    pub external_id: Option<String>,
    /// Status. Required.
    ///
    /// Allowed values: `PROCESS_RUNNING`, `EVENTS_RUNNING`, `FAILED`,
    /// `FAILED_CLIENT`, `FINISHED`, `RUNNING`, `ERROR`, `QUERY_CANCELLED`,
    /// `TIMED_OUT`, `QUERY_EXPIRED`.
    pub status: String,
    /// Query loading status in percentage. Required.
    pub progress: i64,
    /// Column metadata — each entry includes the name of the column and its type.
    /// Optional/nullable. Freeform objects, kept as JSON.
    pub columns: Option<Vec<serde_json::Value>>,
    /// Actual searched data — an array of rows, each row an array of cells.
    /// Optional/nullable. Cell types vary, kept as JSON.
    pub data: Option<Vec<Vec<serde_json::Value>>>,
    /// Possible action items to improve query results. Optional/nullable.
    pub recommendations: Option<Vec<String>>,
}

/// A single Deep Visibility event (item in the `data` array of the events
/// endpoints `GET /web/api/v2.1/dv/events` and
/// `GET /web/api/v2.1/dv/events/{event_type}`).
///
/// The required (bare-`T`) fields per the spec are: `agentDomain`,
/// `agentGroupId`, `agentId`, `agentInfected`, `agentIp`, `agentIsActive`,
/// `agentIsDecommissioned`, `agentMachineType`, `agentName`,
/// `agentNetworkStatus`, `agentOs`, `agentUuid`, `agentVersion`, `createdAt`,
/// `id`, `objectType`, `processName`, `siteName`, `user`. All other fields are
/// optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepVisibilityEvent {
    /// Id. Required.
    pub id: String,
    /// Object type. Required.
    pub object_type: String,
    /// User. Required.
    pub user: String,
    /// Created at (date-time string, e.g. `2018-02-27T04:49:26.257525Z`). Required.
    pub created_at: String,
    /// Process name. Required.
    pub process_name: String,
    /// Agent name. Required.
    pub agent_name: String,
    /// Agent domain. Required.
    pub agent_domain: String,
    /// Agent group id. Required.
    pub agent_group_id: String,
    /// Agent id. Required.
    pub agent_id: String,
    /// Agent infected. Required.
    pub agent_infected: bool,
    /// Agent ip. Required.
    pub agent_ip: String,
    /// Agent is active. Required.
    pub agent_is_active: bool,
    /// Agent is decommissioned. Required.
    pub agent_is_decommissioned: bool,
    /// Agent machine type. Required.
    pub agent_machine_type: String,
    /// Agent network status. Required.
    pub agent_network_status: String,
    /// OS type. Required. Allowed values: `linux`, `macos`, `windows_legacy`, `windows`.
    pub agent_os: String,
    /// Agent version. Required.
    pub agent_version: String,
    /// Agent uuid. Required.
    pub agent_uuid: String,
    /// Site name. Required.
    pub site_name: String,
    /// Pid. Optional/nullable.
    pub pid: Option<String>,
    /// Src ip. Optional/nullable.
    pub src_ip: Option<String>,
    /// Src port. Optional/nullable.
    pub src_port: Option<i64>,
    /// Dst ip. Optional/nullable.
    pub dst_ip: Option<String>,
    /// Dst port. Optional/nullable.
    pub dst_port: Option<i64>,
    /// File sha1. Optional/nullable.
    pub file_sha1: Option<String>,
    /// File sha256. Optional/nullable.
    pub file_sha256: Option<String>,
    /// File md5. Optional/nullable.
    pub file_md5: Option<String>,
    /// Old file sha1. Optional/nullable.
    pub old_file_sha1: Option<String>,
    /// Old file sha256. Optional/nullable.
    pub old_file_sha256: Option<String>,
    /// Old file md5. Optional/nullable.
    pub old_file_md5: Option<String>,
    /// Publisher. Optional/nullable.
    pub publisher: Option<String>,
    /// Signature signed invalid reason. Optional/nullable.
    pub signature_signed_invalid_reason: Option<String>,
    /// Signed status. Optional/nullable.
    pub signed_status: Option<String>,
    /// Verified status. Optional/nullable.
    pub verified_status: Option<String>,
    /// Sha1. Optional/nullable.
    pub sha1: Option<String>,
    /// Sha256. Optional/nullable.
    pub sha256: Option<String>,
    /// Md5. Optional/nullable.
    pub md5: Option<String>,
    /// File full name. Optional/nullable.
    pub file_full_name: Option<String>,
    /// Old file name. Optional/nullable.
    pub old_file_name: Option<String>,
    /// Tid. Optional/nullable.
    pub tid: Option<String>,
    /// Rpid. Optional/nullable.
    pub rpid: Option<String>,
    /// Dns request. Optional/nullable.
    pub dns_request: Option<String>,
    /// Dns response. Optional/nullable.
    pub dns_response: Option<String>,
    /// Process cmd. Optional/nullable.
    pub process_cmd: Option<String>,
    /// Process group id. Optional/nullable.
    pub process_group_id: Option<String>,
    /// Process image path. Optional/nullable.
    pub process_image_path: Option<String>,
    /// Process user name. Optional/nullable.
    pub process_user_name: Option<String>,
    /// Process image sha1 hash. Optional/nullable.
    pub process_image_sha1_hash: Option<String>,
    /// Process is malicious. Optional/nullable.
    pub process_is_malicious: Option<bool>,
    /// Process unique key. Optional/nullable.
    pub process_unique_key: Option<String>,
    /// Process start time. Optional/nullable.
    pub process_start_time: Option<String>,
    /// Process sub system. Optional/nullable.
    pub process_sub_system: Option<String>,
    /// Process session id. Optional/nullable.
    pub process_session_id: Option<String>,
    /// Process integrity level. Optional/nullable.
    pub process_integrity_level: Option<String>,
    /// Process display name. Optional/nullable.
    pub process_display_name: Option<String>,
    /// Process is wow64. Optional/nullable.
    pub process_is_wow64: Option<String>,
    /// Process is redirected command processor. Optional/nullable.
    pub process_is_redirected_command_processor: Option<String>,
    /// Process root. Optional/nullable.
    pub process_root: Option<String>,
    /// Parent process group id. Optional/nullable.
    pub parent_process_group_id: Option<String>,
    /// Parent process is malicious. Optional/nullable.
    pub parent_process_is_malicious: Option<bool>,
    /// Parent process name. Optional/nullable.
    pub parent_process_name: Option<String>,
    /// Parent pid. Optional/nullable.
    pub parent_pid: Option<String>,
    /// Parent process start time. Optional/nullable.
    pub parent_process_start_time: Option<String>,
    /// Parent process unique key. Optional/nullable.
    pub parent_process_unique_key: Option<String>,
    /// Network method. Optional/nullable.
    pub network_method: Option<String>,
    /// Network source. Optional/nullable.
    pub network_source: Option<String>,
    /// Network url. Optional/nullable.
    pub network_url: Option<String>,
    /// Direction. Optional/nullable.
    pub direction: Option<String>,
    /// Event type. Optional/nullable.
    pub event_type: Option<String>,
    /// Registry path. Optional/nullable.
    pub registry_path: Option<String>,
    /// Registry id. Optional/nullable.
    pub registry_id: Option<String>,
    /// Task name. Optional/nullable.
    pub task_name: Option<String>,
    /// Task path. Optional/nullable.
    pub task_path: Option<String>,
    /// True context. Optional/nullable.
    pub true_context: Option<String>,
    /// File id. Optional/nullable.
    pub file_id: Option<String>,
    /// Related to threat. Optional/nullable.
    pub related_to_threat: Option<String>,
    /// Forensic url. Optional/nullable.
    pub forensic_url: Option<String>,
    /// Threat status. Optional/nullable.
    pub threat_status: Option<String>,
    /// Logins user name. Optional/nullable.
    pub logins_user_name: Option<String>,
    /// Logins base type. Optional/nullable.
    pub logins_base_type: Option<String>,
    /// Indicator category. Optional/nullable.
    pub indicator_category: Option<String>,
    /// Indicator description. Optional/nullable.
    pub indicator_description: Option<String>,
    /// Indicator metadata. Optional/nullable.
    pub indicator_metadata: Option<String>,
    /// Indicator name. Optional/nullable.
    pub indicator_name: Option<String>,
    /// Connection status. Optional/nullable.
    pub connection_status: Option<String>,
    /// File size. Optional/nullable.
    pub file_size: Option<String>,
    /// File type. Optional/nullable.
    pub file_type: Option<String>,
    /// Src proc download token. Optional/nullable.
    pub src_proc_download_token: Option<String>,
    /// Is agent version fully supported for pg. Optional/nullable.
    pub is_agent_version_fully_supported_for_pg: Option<bool>,
    /// Is agent version fully supported for pg message. Optional/nullable.
    pub is_agent_version_fully_supported_for_pg_message: Option<String>,
}
