//! `RemoteOps Scripts` tag — response models.
//!
//! Models for the SentinelOne Script Library (Remote Script Orchestration):
//! script metadata, script execution results, pending executions, guardrails
//! configuration and remote-script task status.
//!
//! Field nullability mirrors the OpenAPI spec exactly: a field is a bare `T`
//! only when it is in the schema `required` array *and* not `x-nullable`;
//! otherwise it is `Option<T>` (the spec's "default null" behaviour). Enum
//! fields are modelled as `String` for forward-compatibility; the allowed
//! values are documented on each field.

use serde::Deserialize;

/// A script in the SentinelOne Script Library (enriched representation).
///
/// Returned by `GET`/`POST`/`DELETE`/`PUT /web/api/v2.1/remote-scripts` and the
/// edit endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedScript {
    /// Script ID.
    pub id: Option<String>,
    /// Script name. Required.
    pub script_name: String,
    /// Script type. Required. Enum value (e.g. `artifactCollection`,
    /// `dataCollection`, `action`).
    pub script_type: String,
    /// Script description.
    pub script_description: Option<String>,
    /// Version. Required.
    pub version: String,
    /// OS types. Required. Each value is an OS type string (e.g. `linux`,
    /// `macos`, `windows`).
    pub os_types: Vec<String>,
    /// Is input required. Required.
    pub input_required: bool,
    /// Input example. Required.
    pub input_example: String,
    /// Input instructions. Required.
    pub input_instructions: String,
    /// Created by user id. Required.
    pub created_by_user_id: String,
    /// Created by user.
    pub created_by_user: Option<String>,
    /// Name of the creating user.
    pub creator: Option<String>,
    /// Id of the creating user.
    pub creator_id: Option<String>,
    /// Name of the updating user. Nullable.
    pub updater: Option<String>,
    /// Id of the updating user. Nullable.
    pub updater_id: Option<String>,
    /// Created at (date-time string).
    pub created_at: Option<String>,
    /// Updated at (date-time string).
    pub updated_at: Option<String>,
    /// Mgmt id.
    pub mgmt_id: Option<i64>,
    /// Scope level. Enum value: `tenant`, `account`, `site`, `group`,
    /// `sentinel`, `global`.
    pub scope_level: Option<String>,
    /// Scope ID.
    pub scope_id: Option<String>,
    /// The scripts scope name.
    pub scope_name: Option<String>,
    /// The path of the scripts scope.
    pub scope_path: Option<String>,
    /// File name.
    pub short_file_name: Option<String>,
    /// File name with full path.
    pub file_name: Option<String>,
    /// File size.
    pub file_size: Option<i64>,
    /// Bucket name.
    pub bucket_name: Option<String>,
    /// Signature.
    pub signature: Option<String>,
    /// Signature type.
    pub signature_type: Option<String>,
    /// Script runtime timeout in seconds.
    pub script_runtime_timeout_seconds: Option<i64>,
    /// Is the script runnable in Advanced Response Scripts.
    pub is_available_for_ars: Option<bool>,
    /// Is the script runnable in Lite version. Nullable.
    pub is_available_for_lite: Option<bool>,
    /// Supported destinations for the script output. Nullable.
    pub supported_destinations: Option<Vec<String>>,
    /// Output file paths. Nullable.
    pub output_file_paths: Option<Vec<String>>,
    /// Script name in camel case representation used for localization, only
    /// relevant for builtin-scripts.
    pub name_key: Option<String>,
    /// Package metadata.
    pub package: Option<EnrichedScriptPackage>,
}

/// Package metadata attached to an [`EnrichedScript`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrichedScriptPackage {
    /// Package ID.
    pub id: Option<String>,
    /// File name with full path.
    pub file_name: Option<String>,
    /// File size.
    pub file_size: Option<i64>,
    /// Bucket name.
    pub bucket_name: Option<String>,
    /// Signature.
    pub signature: Option<String>,
    /// Signature type.
    pub signature_type: Option<String>,
    /// Package expiration option on endpoint.
    pub endpoint_expiration: Option<String>,
    /// Package expiration time on endpoint. Nullable.
    pub endpoint_expiration_seconds: Option<i64>,
}

/// Result data of `POST /web/api/v2.1/remote-scripts/execute`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteScriptExecuteResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
    /// The parent task id of the script execution task, null in case of pending
    /// execution. Nullable.
    pub parent_task_id: Option<String>,
    /// Flag indicating if requested script execution requires approval and is
    /// created as pending execution.
    pub pending: Option<bool>,
    /// ID of created pending execution, present only if `pending` flag is true.
    pub pending_execution_id: Option<String>,
}

/// Result data of `POST /web/api/v2.1/remote-scripts/fetch-files`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadScriptsResults {
    /// Inner data payload.
    pub data: Option<DownloadScriptsResultsData>,
}

/// Inner payload of [`DownloadScriptsResults`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadScriptsResultsData {
    /// List of download links.
    pub download_links: Option<Vec<DownloadScriptsLink>>,
    /// Task ids and detailed errors for tasks which a download link couldn't be
    /// fetched.
    pub errors: Option<Vec<DownloadScriptsError>>,
}

/// A single download link entry within [`DownloadScriptsResultsData`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadScriptsLink {
    /// Download link for the file.
    pub download_url: Option<String>,
    /// The name of the file.
    pub file_name: Option<String>,
    /// The task id related to the download link.
    pub task_id: Option<String>,
}

/// A single fetch error within [`DownloadScriptsResultsData`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadScriptsError {
    /// The failed-to-fetch file task id.
    pub task_id: Option<String>,
    /// The error string for failing to fetch a download link.
    pub error_string: Option<String>,
}

/// Result data of `GET /web/api/v2.1/remote-scripts/script-content`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScriptContent {
    /// Script content. Required.
    pub script_content: String,
}

/// Generic operation-result data, returned by guardrails mutation endpoints and
/// the approve/decline pending-execution endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationResultStatus {
    /// Operation result.
    pub success: Option<bool>,
}

/// Guardrails configuration, returned by
/// `GET /web/api/v2.1/remote-scripts/guardrails/configuration`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuardrailsConfiguration {
    /// Whether guardrail is active. Required.
    pub enabled: bool,
    /// Whether guardrail is inherited. Required.
    pub inherited: bool,
    /// Threshold for number of endpoints. Required (but nullable).
    pub endpoints_quantity: Option<i64>,
    /// List of script types that the guardrail relates to. Required.
    pub script_types: Vec<String>,
}

/// Result data of `POST /web/api/v2.1/remote-scripts/guardrails/check`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuardrailCheckResult {
    /// Whether the guardrail check requires approval.
    pub requires_approval: Option<bool>,
}

/// A pending script execution, returned by
/// `GET /web/api/v2.1/remote-scripts/pending-executions`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingExecution {
    /// Pending execution id. Required.
    pub pending_execution_id: String,
    /// State. Required. Enum value: `waiting`, `approved`, `declined`,
    /// `expired`.
    pub state: String,
    /// Execution data. Required.
    pub execution_data: PendingExecutionData,
    /// Script data. Required.
    pub script_data: EnrichedScript,
    /// Creator id. Required.
    pub creator_id: String,
    /// Creator. Required (but nullable).
    pub creator: Option<String>,
    /// Reviewer id. Required.
    pub reviewer_id: String,
    /// Reviewer. Required (but nullable).
    pub reviewer: Option<String>,
    /// Can approve or decline. Required.
    pub can_approve_or_decline: bool,
    /// Total endpoints. Required.
    pub total_endpoints: i64,
    /// Endpoints by scope. Required.
    pub endpoints_by_scope: Vec<PendingExecutionScopeEndpoints>,
    /// Created at (date-time string).
    pub created_at: Option<String>,
    /// Scheduled task id. Nullable.
    pub scheduled_task_id: Option<String>,
}

/// Per-scope endpoint counts within a [`PendingExecution`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingExecutionScopeEndpoints {
    /// Scope name. Required.
    pub scope_name: String,
    /// Total endpoints. Required.
    pub total_endpoints: i64,
}

/// Execution parameters for a [`PendingExecution`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingExecutionData {
    /// Script id. Required.
    pub script_id: String,
    /// Task description. Required.
    pub task_description: String,
    /// Output destination. Required. Enum value: `None`, `Local`,
    /// `SentinelCloud`, `SingularityXDR`.
    pub output_destination: String,
    /// Input params.
    pub input_params: Option<String>,
    /// If set to true, execution will require approval.
    pub requires_approval: Option<bool>,
    /// Destination profile keyword.
    pub destination_profile_keyword: Option<String>,
    /// Id of destination profile to use.
    pub destination_profile_id: Option<String>,
    /// Api key.
    pub api_key: Option<String>,
    /// Password.
    pub password: Option<String>,
    /// Used to specify execution where a generic password is used.
    pub password_from_scope: Option<PendingExecutionPasswordScope>,
    /// Output destination URL for SingularityXDR.
    pub singularityxdr_url: Option<String>,
    /// SingularityXDR keyword.
    pub singularityxdr_keyword: Option<String>,
    /// Output directory.
    pub output_directory: Option<String>,
    /// Output file paths.
    pub output_file_paths: Option<Vec<String>>,
    /// Script runtime timeout in seconds for current execution.
    pub script_runtime_timeout_seconds: Option<i64>,
}

/// Generic-password scope reference within [`PendingExecutionData`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PendingExecutionPasswordScope {
    /// User scope. Required. Enum value: `group`, `site`, `account`, `tenant`.
    pub scope_level: String,
    /// String repr. of scope id.
    pub scope_id: Option<String>,
}

/// A remote-script task status row, returned by
/// `GET /web/api/v2.1/remote-scripts/status`.
///
/// All fields are optional (the schema declares no `required` set).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteScriptTaskStatus {
    /// Task ID.
    pub id: Option<String>,
    /// Parent task ID.
    pub parent_task_id: Option<String>,
    /// Task type.
    #[serde(rename = "type")]
    pub task_type: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Initiating user name.
    pub initiated_by: Option<String>,
    /// Initiating user id.
    pub initiated_by_id: Option<String>,
    /// Agent computer name.
    pub agent_computer_name: Option<String>,
    /// Status. Enum value: `created`, `scheduled`, `pending`,
    /// `pending_user_action`, `in_progress`, `failed`, `completed`, `canceled`,
    /// `expired`, `partially_completed`.
    pub status: Option<String>,
    /// Status code.
    pub status_code: Option<i64>,
    /// Status description (freeform).
    pub status_description: Option<serde_json::Value>,
    /// Detailed status.
    pub detailed_status: Option<String>,
    /// Created at (date-time string).
    pub created_at: Option<String>,
    /// Updated at (date-time string).
    pub updated_at: Option<String>,
    /// Account name.
    pub account_name: Option<String>,
    /// Account id.
    pub account_id: Option<String>,
    /// Agent OS type. Nullable. Enum value: `linux`, `macos`, `windows_legacy`,
    /// `windows`.
    pub agent_os_type: Option<String>,
    /// Agent machine type. Nullable.
    pub agent_machine_type: Option<String>,
    /// Agent id. Nullable.
    pub agent_id: Option<String>,
    /// Whether the agent is active. Nullable.
    pub agent_is_active: Option<bool>,
    /// Whether the agent is decommissioned. Nullable.
    pub agent_is_decommissioned: Option<bool>,
    /// Agent UUID. Nullable.
    pub agent_uuid: Option<String>,
    /// Group id.
    pub group_id: Option<String>,
    /// Group name.
    pub group_name: Option<String>,
    /// Site id.
    pub site_id: Option<String>,
    /// Site name.
    pub site_name: Option<String>,
    /// Script results bucket.
    pub script_results_bucket: Option<String>,
    /// Script results path.
    pub script_results_path: Option<String>,
    /// Script results signature.
    pub script_results_signature: Option<String>,
}
