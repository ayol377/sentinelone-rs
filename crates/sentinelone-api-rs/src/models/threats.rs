//! Models for the `Threats` tag.
//!
//! Field nullability follows the spec exactly: a field is a bare `T` only when it
//! is in the schema `required` array AND is not `x-nullable`; otherwise it is
//! `Option<T>` (the SentinelOne "default null" behaviour). Enum-valued fields are
//! kept as `String` for forward compatibility, with the allowed values documented
//! in their doc comments. Deeply nested / cross-tag objects use
//! [`serde_json::Value`].

use serde::Deserialize;

/// A threat (as returned by `GET /web/api/v2.1/threats`).
///
/// All top-level fields are optional in the spec (`required: []`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Threat {
    /// Threat ID.
    pub id: Option<String>,
    /// Whitening (exclusion) options available for this threat.
    pub whitening_options: Option<Vec<String>>,
    /// Threat mitigation information (one entry per mitigation action).
    pub mitigation_status: Option<Vec<ThreatMitigationStatus>>,
    /// Threat indicators (techniques/tactics observed). Freeform objects.
    pub indicators: Option<Vec<serde_json::Value>>,
    /// Container information (if the threat occurred inside a container).
    pub container_info: Option<ThreatContainerInfo>,
    /// Kubernetes information (if the threat occurred inside a K8s workload).
    pub kubernetes_info: Option<ThreatKubernetesInfo>,
    /// ECS (AWS Elastic Container Service) information.
    pub ecs_info: Option<ThreatEcsInfo>,
    /// Core threat details.
    pub threat_info: Option<ThreatInfo>,
    /// Agent state at the current time.
    pub agent_realtime_info: Option<ThreatAgentRealtimeInfo>,
    /// Agent state at detection time.
    pub agent_detection_info: Option<ThreatAgentDetectionInfo>,
}

/// A single mitigation-status entry of a [`Threat`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatMitigationStatus {
    /// Status. Allowed: `success`, `failed`, `pending-reboot`, `pending`, `sent`,
    /// `partial`.
    pub status: Option<String>,
    /// Action. Allowed: `kill`, `remediate`, `rollback`, `quarantine`,
    /// `unquarantine`, `network_quarantine`, `remove_macros`, `restore_macros`.
    pub action: Option<String>,
    /// ID of the mitigation report.
    pub report_id: Option<String>,
    /// Timestamp of last mitigation status update (ISO-8601).
    pub last_update: Option<String>,
    /// Agent could not find the threat.
    pub group_not_found: Option<bool>,
    /// Report download URL. `None` if there is no report.
    pub latest_report: Option<String>,
    /// Actions counters.
    pub actions_counters: Option<ThreatActionsCounters>,
    /// The Agent generates a full mitigation report.
    pub agent_supports_report: Option<bool>,
    /// The time the Agent started the mitigation (ISO-8601).
    pub mitigation_started_at: Option<String>,
    /// The time the Agent finished the mitigation (ISO-8601).
    pub mitigation_ended_at: Option<String>,
}

/// Per-action counters of a mitigation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatActionsCounters {
    /// Total.
    pub total: Option<i64>,
    /// Success.
    pub success: Option<i64>,
    /// Failed.
    pub failed: Option<i64>,
    /// Pending reboot.
    pub pending_reboot: Option<i64>,
    /// Not found.
    pub not_found: Option<i64>,
}

/// Container information of a [`Threat`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatContainerInfo {
    /// Container ID.
    pub id: Option<String>,
    /// Container name.
    pub name: Option<String>,
    /// Container image.
    pub image: Option<String>,
    /// Container labels.
    pub labels: Option<Vec<String>>,
    /// Whether the container is network-quarantined.
    pub is_container_quarantine: Option<bool>,
}

/// Kubernetes information of a [`Threat`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatKubernetesInfo {
    /// Cluster name.
    pub cluster: Option<String>,
    /// Namespace name.
    pub namespace: Option<String>,
    /// Pod name.
    pub pod: Option<String>,
    /// Node name.
    pub node: Option<String>,
    /// Node labels.
    pub node_labels: Option<Vec<String>>,
    /// Namespace labels.
    pub namespace_labels: Option<Vec<String>>,
    /// Pod labels.
    pub pod_labels: Option<Vec<String>>,
    /// Controller kind.
    pub controller_kind: Option<String>,
    /// Controller name.
    pub controller_name: Option<String>,
    /// Controller labels.
    pub controller_labels: Option<Vec<String>>,
    /// Whether the container is network-quarantined.
    pub is_container_quarantine: Option<bool>,
}

/// AWS ECS information of a [`Threat`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatEcsInfo {
    /// ECS type.
    pub r#type: Option<String>,
    /// ECS version.
    pub version: Option<String>,
    /// Cluster name.
    pub cluster_name: Option<String>,
    /// Task ARN.
    pub task_arn: Option<String>,
    /// Task availability zone.
    pub task_availability_zone: Option<String>,
    /// Service name.
    pub service_name: Option<String>,
    /// Service ARN.
    pub service_arn: Option<String>,
    /// Task definition family.
    pub task_definition_family: Option<String>,
    /// Task definition revision.
    pub task_definition_revision: Option<String>,
    /// Task definition ARN.
    pub task_definition_arn: Option<String>,
}

/// Core threat details (`threatInfo`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatInfo {
    /// Threat ID.
    pub threat_id: Option<String>,
    /// Analyst verdict. Allowed: `undefined`, `true_positive`, `false_positive`,
    /// `suspicious`.
    pub analyst_verdict: Option<String>,
    /// Human-readable analyst verdict (cross-tag object).
    pub analyst_verdict_description: Option<serde_json::Value>,
    /// Browser type.
    pub browser_type: Option<String>,
    /// Certificate ID.
    pub certificate_id: Option<String>,
    /// Threat classification.
    pub classification: Option<String>,
    /// Classification source. Allowed: `Cloud`, `Behavioral`, `Static`, `Engine`.
    /// (nullable in spec)
    pub classification_source: Option<String>,
    /// Collection ID.
    pub collection_id: Option<String>,
    /// Confidence level. Allowed: `malicious`, `suspicious`, `n/a`.
    pub confidence_level: Option<String>,
    /// Created at (ISO-8601).
    pub created_at: Option<String>,
    /// Whether the threat was mitigated pre-execution (vs post-execution).
    pub mitigated_preemptively: Option<bool>,
    /// Detection type. (nullable in spec)
    pub detection_type: Option<String>,
    /// Engines that detected the threat (cross-tag object).
    pub engines: Option<serde_json::Value>,
    /// Detection engines (cross-tag object).
    pub detection_engines: Option<serde_json::Value>,
    /// File extension.
    pub file_extension: Option<String>,
    /// File extension type.
    pub file_extension_type: Option<String>,
    /// File path (cross-tag object).
    pub file_path: Option<serde_json::Value>,
    /// File size in bytes.
    pub file_size: Option<i64>,
    /// File verification type.
    pub file_verification_type: Option<String>,
    /// Identified at (ISO-8601).
    pub identified_at: Option<String>,
    /// Initiating source. Allowed: `agent_policy`, `full_disk_scan`,
    /// `sentinelctl`, `dv_command`, `console_api`, `on_demand_scan`,
    /// `star_active`, `star_manual`.
    pub initiated_by: Option<String>,
    /// Human-readable initiating source (cross-tag object).
    pub initiated_by_description: Option<serde_json::Value>,
    /// Initiating user ID.
    pub initiating_user_id: Option<String>,
    /// Initiating username.
    pub initiating_username: Option<String>,
    /// Whether the signing certificate is valid.
    pub is_valid_certificate: Option<bool>,
    /// Whether the threat is fileless (cross-tag object).
    pub is_fileless: Option<serde_json::Value>,
    /// Whether the events limit was reached.
    pub reached_events_limit: Option<bool>,
    /// Malicious process arguments.
    pub malicious_process_arguments: Option<String>,
    /// MD5 hash.
    pub md5: Option<String>,
    /// Originator process.
    pub originator_process: Option<String>,
    /// Publisher name.
    pub publisher_name: Option<String>,
    /// SHA1 hash. (required + not nullable in spec)
    pub sha1: Option<String>,
    /// SHA256 hash.
    pub sha256: Option<String>,
    /// Mitigation status. Allowed: `not_mitigated`, `mitigated`,
    /// `marked_as_benign`.
    pub mitigation_status: Option<String>,
    /// Human-readable mitigation status (cross-tag object).
    pub mitigation_status_description: Option<serde_json::Value>,
    /// Threat name.
    pub threat_name: Option<String>,
    /// Incident status. Allowed: `unresolved`, `in_progress`, `resolved`.
    pub incident_status: Option<String>,
    /// Human-readable incident status (cross-tag object).
    pub incident_status_description: Option<serde_json::Value>,
    /// Storyline.
    pub storyline: Option<String>,
    /// Updated at (ISO-8601).
    pub updated_at: Option<String>,
    /// External ticket ID.
    pub external_ticket_id: Option<String>,
    /// Whether an external ticket exists (cross-tag object).
    pub external_ticket_exists: Option<serde_json::Value>,
    /// Whether the threat was automatically resolved.
    pub automatically_resolved: Option<bool>,
    /// Cloud files hash verdict.
    pub cloud_files_hash_verdict: Option<String>,
    /// Whether at least one action failed.
    pub failed_actions: Option<bool>,
    /// Whether a reboot is required.
    pub reboot_required: Option<bool>,
    /// Whether at least one action is pending.
    pub pending_actions: Option<bool>,
    /// Process user.
    pub process_user: Option<String>,
    /// Macro modules (freeform objects).
    pub macro_modules: Option<Vec<serde_json::Value>>,
    /// Root process UPN.
    pub root_process_upn: Option<String>,
}

/// Agent state at the current time (`agentRealtimeInfo`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatAgentRealtimeInfo {
    /// Account ID.
    pub account_id: Option<String>,
    /// Account name.
    pub account_name: Option<String>,
    /// Site ID.
    pub site_id: Option<String>,
    /// Site name.
    pub site_name: Option<String>,
    /// Group ID.
    pub group_id: Option<String>,
    /// Group name.
    pub group_name: Option<String>,
    /// Number of active threats on the Agent.
    pub active_threats: Option<i64>,
    /// Agent domain.
    pub agent_domain: Option<String>,
    /// Agent ID.
    pub agent_id: Option<String>,
    /// Whether the Agent is currently connected.
    pub agent_is_active: Option<bool>,
    /// Whether the Agent is decommissioned.
    pub agent_is_decommissioned: Option<bool>,
    /// Decommissioned timestamp (spec types this as boolean).
    pub agent_decommissioned_at: Option<bool>,
    /// Agent machine type. Allowed: `unknown`, `desktop`, `laptop`, `server`,
    /// `kubernetes node`, `storage`, `kubernetes pod`, `ecs task`.
    pub agent_machine_type: Option<String>,
    /// Agent network status.
    pub agent_network_status: Option<String>,
    /// Agent OS name.
    pub agent_os_name: Option<String>,
    /// Agent OS type. Allowed: `linux`, `macos`, `windows_legacy`, `windows`.
    pub agent_os_type: Option<String>,
    /// Agent OS revision.
    pub agent_os_revision: Option<String>,
    /// Agent UUID.
    pub agent_uuid: Option<String>,
    /// Agent computer name.
    pub agent_computer_name: Option<String>,
    /// Agent version.
    pub agent_version: Option<String>,
    /// User actions needed.
    pub user_actions_needed: Option<Vec<String>>,
    /// Scan status.
    pub scan_status: Option<String>,
    /// Scan started at (ISO-8601).
    pub scan_started_at: Option<String>,
    /// Scan finished at (ISO-8601).
    pub scan_finished_at: Option<String>,
    /// Scan aborted at (ISO-8601).
    pub scan_aborted_at: Option<String>,
    /// Agent mitigation mode.
    pub agent_mitigation_mode: Option<String>,
    /// Network interfaces (freeform objects).
    pub network_interfaces: Option<Vec<serde_json::Value>>,
    /// Whether the Agent is infected.
    pub agent_infected: Option<bool>,
    /// Whether a reboot is required.
    pub reboot_required: Option<bool>,
    /// Operational state.
    pub operational_state: Option<String>,
    /// Storage type.
    pub storage_type: Option<String>,
    /// Storage name.
    pub storage_name: Option<String>,
}

/// Agent state at detection time (`agentDetectionInfo`). All fields nullable.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatAgentDetectionInfo {
    /// Group ID.
    pub group_id: Option<String>,
    /// Group name.
    pub group_name: Option<String>,
    /// Site ID.
    pub site_id: Option<String>,
    /// Site name.
    pub site_name: Option<String>,
    /// Account ID.
    pub account_id: Option<String>,
    /// Account name.
    pub account_name: Option<String>,
    /// Agent IPv4 address.
    pub agent_ip_v4: Option<String>,
    /// Agent IPv6 address.
    pub agent_ip_v6: Option<String>,
    /// Agent domain.
    pub agent_domain: Option<String>,
    /// External IP address.
    pub external_ip: Option<String>,
    /// Agent UUID.
    pub agent_uuid: Option<String>,
    /// Last logged-in username.
    pub agent_last_logged_in_user_name: Option<String>,
    /// Last logged-in user e-mail.
    pub agent_last_logged_in_user_mail: Option<String>,
    /// Last logged-in UPN.
    pub agent_last_logged_in_upn: Option<String>,
    /// Agent OS revision.
    pub agent_os_revision: Option<String>,
    /// Agent OS name.
    pub agent_os_name: Option<String>,
    /// Agent registered at (ISO-8601).
    pub agent_registered_at: Option<String>,
    /// Agent version.
    pub agent_version: Option<String>,
    /// Asset version.
    pub asset_version: Option<String>,
    /// Agent mitigation mode.
    pub agent_mitigation_mode: Option<String>,
    /// Agent detection state.
    pub agent_detection_state: Option<String>,
    /// Cloud providers (freeform object).
    pub cloud_providers: Option<serde_json::Value>,
}

/// A quarantined file (`GET /web/api/v2.1/threats/{threat_id}/quarantined-files`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuarantinedFile {
    /// File path.
    pub file_path: Option<String>,
    /// File name.
    pub file_name: Option<String>,
    /// File size in bytes. (nullable in spec)
    pub file_size: Option<i64>,
}

/// A threat timeline entry (`GET /web/api/v2.1/threats/{threat_id}/timeline`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatTimelineEntry {
    /// Activity ID.
    pub id: Option<String>,
    /// Created at (ISO-8601).
    pub created_at: Option<String>,
    /// Updated at (ISO-8601).
    pub updated_at: Option<String>,
    /// Activity type code.
    pub activity_type: Option<i64>,
    /// Activity data (freeform object).
    pub data: Option<serde_json::Value>,
    /// Primary description.
    pub primary_description: Option<String>,
    /// Secondary description.
    pub secondary_description: Option<String>,
    /// OS family. (nullable in spec)
    pub os_family: Option<String>,
    /// Hash.
    pub hash: Option<String>,
    /// Agent updated version.
    pub agent_updated_version: Option<String>,
    /// User ID.
    pub user_id: Option<String>,
    /// Threat ID.
    pub threat_id: Option<String>,
    /// Agent ID.
    pub agent_id: Option<String>,
    /// Account ID.
    pub account_id: Option<String>,
    /// Site ID.
    pub site_id: Option<String>,
    /// Group ID.
    pub group_id: Option<String>,
}

/// A threat event entity
/// (`GET /web/api/v2.1/threats/{threat_id}/explore/events`).
///
/// Fields in the spec `required` array and not nullable are bare `T`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatEvent {
    /// Event ID. (required)
    pub id: String,
    /// Object type. (required)
    pub object_type: String,
    /// Created at (ISO-8601). (required)
    pub created_at: String,
    /// Process name. (required)
    pub process_name: String,
    /// Agent name. (required)
    pub agent_name: String,
    /// Agent group ID. (required)
    pub agent_group_id: String,
    /// Agent ID. (required)
    pub agent_id: String,
    /// Whether the Agent is active. (required)
    pub agent_is_active: bool,
    /// Whether the Agent is decommissioned. (required)
    pub agent_is_decommissioned: bool,
    /// Agent machine type. (required)
    pub agent_machine_type: String,
    /// Agent network status. (required)
    pub agent_network_status: String,
    /// Agent OS. (required)
    pub agent_os: String,
    /// Agent version. (required)
    pub agent_version: String,
    /// Agent UUID. (required)
    pub agent_uuid: String,
    /// Site ID. (required)
    pub site_id: String,
    /// Site name. (required)
    pub site_name: String,
    /// Process PID.
    pub pid: Option<String>,
    /// Source IP.
    pub src_ip: Option<String>,
    /// Source port.
    pub src_port: Option<i64>,
    /// Destination IP.
    pub dst_ip: Option<String>,
    /// Destination port.
    pub dst_port: Option<i64>,
    /// File SHA1.
    pub file_sha1: Option<String>,
    /// File SHA256.
    pub file_sha256: Option<String>,
    /// File MD5.
    pub file_md5: Option<String>,
    /// Old file SHA1.
    pub old_file_sha1: Option<String>,
    /// Old file SHA256.
    pub old_file_sha256: Option<String>,
    /// Old file MD5.
    pub old_file_md5: Option<String>,
    /// Reason a signed signature is invalid.
    pub signature_signed_invalid_reason: Option<String>,
    /// Verified status.
    pub verified_status: Option<String>,
    /// Signed status.
    pub signed_status: Option<String>,
    /// SHA256.
    pub sha256: Option<String>,
    /// SHA1.
    pub sha1: Option<String>,
    /// MD5.
    pub md5: Option<String>,
    /// File full name.
    pub file_full_name: Option<String>,
    /// Old file name.
    pub old_file_name: Option<String>,
    /// Thread ID.
    pub tid: Option<String>,
    /// Relative process ID.
    pub rpid: Option<String>,
    /// DNS request.
    pub dns_request: Option<String>,
    /// DNS response.
    pub dns_response: Option<String>,
    /// Process command line.
    pub process_cmd: Option<String>,
    /// Process group ID.
    pub process_group_id: Option<String>,
    /// Process image path.
    pub process_image_path: Option<String>,
    /// Process user name.
    pub process_user_name: Option<String>,
    /// Process image SHA1 hash.
    pub process_image_sha1_hash: Option<String>,
    /// Process unique key.
    pub process_unique_key: Option<String>,
    /// Process start time.
    pub process_start_time: Option<String>,
    /// Process subsystem.
    pub process_sub_system: Option<String>,
    /// Process session ID.
    pub process_session_id: Option<String>,
    /// Process integrity level.
    pub process_integrity_level: Option<String>,
    /// Process display name.
    pub process_display_name: Option<String>,
    /// Whether the process is WOW64.
    pub process_is_wow64: Option<String>,
    /// Whether the process is a redirected command processor.
    pub process_is_redirected_command_processor: Option<String>,
    /// Process root.
    pub process_root: Option<String>,
    /// Parent process name.
    pub parent_process_name: Option<String>,
    /// Parent PID.
    pub parent_pid: Option<String>,
    /// Parent process unique key.
    pub parent_process_unique_key: Option<String>,
    /// Network source.
    pub network_source: Option<String>,
    /// Network URL.
    pub network_url: Option<String>,
    /// Network method.
    pub network_method: Option<String>,
    /// Direction.
    pub direction: Option<String>,
    /// Event type.
    pub event_type: Option<String>,
    /// Registry path.
    pub registry_path: Option<String>,
    /// Registry ID.
    pub registry_id: Option<String>,
    /// Registry classification.
    pub registry_classification: Option<String>,
    /// Scheduled task name.
    pub task_name: Option<String>,
    /// Scheduled task path.
    pub task_path: Option<String>,
    /// True context.
    pub true_context: Option<String>,
    /// Storyline.
    pub storyline: Option<String>,
    /// File ID.
    pub file_id: Option<String>,
    /// Logins user name.
    pub logins_user_name: Option<String>,
    /// Logins base type.
    pub logins_base_type: Option<String>,
    /// Indicator category.
    pub indicator_category: Option<String>,
    /// Indicator description.
    pub indicator_description: Option<String>,
    /// Indicator metadata.
    pub indicator_metadata: Option<String>,
    /// Indicator name.
    pub indicator_name: Option<String>,
    /// Connection status.
    pub connection_status: Option<String>,
    /// Publisher.
    pub publisher: Option<String>,
    /// Whether the Agent is infected. (required)
    pub agent_infected: bool,
    /// Agent domain. (required)
    pub agent_domain: String,
    /// Agent IP. (required)
    pub agent_ip: String,
    /// User.
    pub user: Option<String>,
    /// Whether the event is related to the threat.
    pub related_to_threat: Option<bool>,
    /// Threat status.
    pub threat_status: Option<String>,
    /// Protocol.
    pub protocol: Option<String>,
    /// Whether the file has active content.
    pub has_active_content: Option<bool>,
    /// Active content file ID.
    pub active_content_file_id: Option<String>,
    /// Active content path.
    pub active_content_path: Option<String>,
    /// Active content hash.
    pub active_content_hash: Option<String>,
    /// Whether the process is malicious.
    pub process_is_malicious: Option<bool>,
    /// Parent process group ID.
    pub parent_process_group_id: Option<String>,
    /// Whether the parent process is malicious.
    pub parent_process_is_malicious: Option<bool>,
    /// File size (spec types this as string).
    pub file_size: Option<String>,
    /// File type.
    pub file_type: Option<String>,
}

/// Number of entities affected by a requested operation (`{ affected }`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
}

/// Generic success flag (`{ success }`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResult {
    /// Indicates a successful operation.
    pub success: Option<bool>,
}

/// Threat-file download URL (`GET /threats/{threat_id}/download-from-cloud`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatFileDownload {
    /// Threat file download URL.
    pub download_url: Option<String>,
    /// Threat file name.
    pub file_name: Option<String>,
}

/// Result of a mitigation action (`POST /threats/mitigate/{action}`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatMitigationActionResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
    /// Single threat mitigation information.
    pub details: Option<Vec<ThreatMitigationActionDetail>>,
}

/// Per-threat detail of a mitigation action.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatMitigationActionDetail {
    /// Threat ID.
    pub threat_id: Option<String>,
    /// List of latest mitigation reports created by the action trigger.
    pub reports: Option<Vec<ThreatMitigationStatus>>,
    /// List of skipped mitigation actions with additional details.
    pub skipped: Option<Vec<ThreatMitigationSkipped>>,
}

/// A skipped mitigation action with the reason it was skipped.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatMitigationSkipped {
    /// Action. Allowed: `kill`, `remediate`, `rollback`, `quarantine`,
    /// `unquarantine`, `network_quarantine`, `remove_macros`, `restore_macros`.
    pub action: Option<String>,
    /// Reason. Allowed: `permissions`, `not_supported`, `precondition`,
    /// `triggered`.
    pub reason: Option<String>,
    /// Description.
    pub description: Option<String>,
}

/// Per-threat result of an add-to-blocklist / add-to-exclusions operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatRestrictionResultDetail {
    /// Threat ID.
    pub threat_id: Option<String>,
    /// Result of adding the threat to blocklist or exclusions. Allowed:
    /// `created`, `already_exists`, `threat_has_no_hash`, `unknown_failure`,
    /// `empty_value`, `disabled_endpoint`.
    pub result: Option<String>,
    /// Result of changing the threat's analyst verdict as part of the operation.
    /// Allowed: `updated`, `not_changed`, `already_set`, `conditions_not_met`,
    /// `missing_permissions`.
    pub analyst_verdict: Option<String>,
}

/// Result of an add-to-blocklist / add-to-exclusions operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatRestrictionResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
    /// Result details for each threat.
    pub details: Option<Vec<ThreatRestrictionResultDetail>>,
}

/// Per-threat result of an incident-update operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatIncidentResultDetail {
    /// Threat ID.
    pub threat_id: Option<String>,
    /// Result of changing the threat's status. Allowed: `updated`, `not_changed`,
    /// `already_set`, `conditions_not_met`, `missing_permissions`.
    pub result: Option<String>,
    /// Result of changing the threat's analyst verdict as part of the operation.
    /// Allowed: `updated`, `not_changed`, `already_set`, `conditions_not_met`,
    /// `missing_permissions`.
    pub analyst_verdict: Option<String>,
}

/// Result of an incident-update operation (`POST /threats/incident`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatIncidentResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
    /// Result details for each threat.
    pub details: Option<Vec<ThreatIncidentResultDetail>>,
}

/// Per-threat result of an analyst-verdict update.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatAnalystVerdictResultDetail {
    /// Threat ID.
    pub threat_id: Option<String>,
    /// Result of changing the threat's analyst verdict. Allowed: `updated`,
    /// `not_changed`, `already_set`, `conditions_not_met`, `missing_permissions`.
    pub result: Option<String>,
}

/// Result of an analyst-verdict update (`POST /threats/analyst-verdict`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatAnalystVerdictResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
    /// Result details for each threat.
    pub details: Option<Vec<ThreatAnalystVerdictResultDetail>>,
}

/// Exclusion (whitening) options for a threat
/// (`GET /threats/{threat_id}/whitening-options`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhiteningOptions {
    /// Available exclusion options.
    pub whitening_options: Option<Vec<String>>,
    /// Threat type.
    pub threat_type: Option<Vec<String>>,
    /// Threat policy.
    pub threat_policy: Option<String>,
}
