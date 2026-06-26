//! Models for the `alerts` tag (SentinelOne Cloud Detection / STAR alerts).
//!
//! Generated at 1:1 parity with `swagger_2_1.json`
//! (`v2_1.alerts.schemas_AlertInformationSchema_many_200`). Every field is
//! `Option<T>` unless the spec marks it required and not `x-nullable`; the data
//! item declares no `required` members, so all top-level info blocks are
//! optional. Enum-constrained strings are kept as `String` for forward-compat;
//! allowed values are documented on each field.

use serde::Deserialize;

/// A single Cloud Detection alert.
///
/// Mirrors one element of the `data` array returned by
/// `GET /web/api/v2.1/cloud-detection/alerts`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    /// Core alert information (timestamps, verdict, indicator, network/login
    /// context). Optional.
    pub alert_info: Option<AlertInfo>,
    /// Detection rule metadata. Optional.
    pub rule_info: Option<AlertRuleInfo>,
    /// Container context for the endpoint. Optional.
    pub container_info: Option<AlertContainerInfo>,
    /// Kubernetes context for the endpoint. Optional.
    pub kubernetes_info: Option<AlertKubernetesInfo>,
    /// Agent / endpoint detection details. Optional.
    pub agent_detection_info: Option<AlertAgentDetectionInfo>,
    /// Source process details. Optional.
    pub source_process_info: Option<AlertProcessInfo>,
    /// Source parent process details. Optional.
    pub source_parent_process_info: Option<AlertProcessInfo>,
    /// Target process / file details. Optional.
    pub target_process_info: Option<AlertTargetProcessInfo>,
}

/// Core alert information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertInfo {
    /// Timestamp the alert was sent for detection. Optional. Date/time string.
    pub created_at: Option<String>,
    /// Timestamp of alert creation in STAR. Optional. Date/time string.
    pub reported_at: Option<String>,
    /// Timestamp the alert was last updated. Optional. Date/time string.
    pub updated_at: Option<String>,
    /// Event type. Optional.
    pub event_type: Option<String>,
    /// Whether the alert is an EDR alert. Optional.
    pub is_edr: Option<bool>,
    /// Deep Visibility event id. Optional.
    pub dv_event_id: Option<String>,
    /// Alert id. Optional.
    pub alert_id: Option<String>,
    /// Alert source. Optional.
    pub source: Option<String>,
    /// Hit type. Optional. Allowed values: `Events`, `Correlation`,
    /// `UEBAFirstSeen`, `Scheduled`.
    pub hit_type: Option<String>,
    /// Analyst verdict. Optional. Allowed values: `Undefined`, `True positive`,
    /// `False positive`, `Suspicious`.
    pub analyst_verdict: Option<String>,
    /// Incident status. Optional. Allowed values: `Unresolved`, `In progress`,
    /// `Resolved`.
    pub incident_status: Option<String>,
    /// Source IP. Optional.
    pub src_ip: Option<String>,
    /// Source port. Optional.
    pub src_port: Option<String>,
    /// Destination IP. Optional.
    pub dst_ip: Option<String>,
    /// Destination port. Optional.
    pub dst_port: Option<String>,
    /// Network event direction. Optional.
    pub net_event_direction: Option<String>,
    /// Threat-intel indicator value. Optional.
    pub ti_indicator_value: Option<String>,
    /// Threat-intel indicator type. Optional.
    pub ti_indicator_type: Option<String>,
    /// Threat-intel indicator comparison method. Optional.
    pub ti_indicator_comparison_method: Option<String>,
    /// Threat-intel indicator source. Optional.
    pub ti_indicator_source: Option<String>,
    /// DNS request. Optional.
    pub dns_request: Option<String>,
    /// DNS response. Optional.
    pub dns_response: Option<String>,
    /// Indicator name. Optional.
    pub indicator_name: Option<String>,
    /// Indicator category. Optional.
    pub indicator_category: Option<String>,
    /// Indicator description. Optional.
    pub indicator_description: Option<String>,
    /// Module path. Optional.
    pub module_path: Option<String>,
    /// Module SHA1. Optional.
    pub module_sha1: Option<String>,
    /// Login user name. Optional.
    pub logins_user_name: Option<String>,
    /// Source machine IP. Optional.
    pub src_machine_ip: Option<String>,
    /// Whether the login was successful. Optional.
    pub login_is_successful: Option<String>,
    /// Login type. Optional.
    pub login_type: Option<String>,
    /// Whether the login account is administrator-equivalent. Optional.
    pub login_is_administrator_equivalent: Option<String>,
    /// Login account SID. Optional.
    pub login_account_sid: Option<String>,
    /// Login account domain. Optional.
    pub login_account_domain: Option<String>,
    /// Registry path. Optional.
    pub registry_path: Option<String>,
    /// Registry key path. Optional.
    pub registry_key_path: Option<String>,
    /// Registry value. Optional.
    pub registry_value: Option<String>,
    /// Registry old value type. Optional.
    pub registry_old_value_type: Option<String>,
    /// Registry old value. Optional.
    pub registry_old_value: Option<String>,
}

/// Detection rule metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertRuleInfo {
    /// Rule id. Optional.
    pub id: Option<String>,
    /// Rule name. Optional.
    pub name: Option<String>,
    /// Rule description. Optional.
    pub description: Option<String>,
    /// S1QL query string. Required (per the spec sub-schema), not nullable.
    pub s1ql: String,
    /// Query language. Optional. Allowed values: `1.0`, `2.0`.
    pub query_lang: Option<String>,
    /// Query type. Optional. Allowed values: `events`, `correlation`,
    /// `uebafirstseen`, `scheduled`.
    pub query_type: Option<String>,
    /// Rule severity. Optional. Allowed values: `Info`, `Low`, `Medium`,
    /// `High`, `Critical`.
    pub severity: Option<String>,
    /// Scope level. Optional. Allowed values: `group`, `site`, `account`,
    /// `global`.
    pub scope_level: Option<String>,
    /// How matches are treated. Optional. Allowed values: `UNDEFINED`,
    /// `Suspicious`, `Malicious`.
    pub treat_as_threat: Option<String>,
}

/// Container context for the endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertContainerInfo {
    /// Container id. Optional.
    pub id: Option<String>,
    /// Container name. Optional.
    pub name: Option<String>,
    /// Container image. Optional.
    pub image: Option<String>,
    /// Container labels. Optional.
    pub labels: Option<String>,
}

/// Kubernetes context for the endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertKubernetesInfo {
    /// Cluster name. Optional.
    pub cluster: Option<String>,
    /// Namespace name. Optional.
    pub namespace: Option<String>,
    /// Pod name. Optional.
    pub pod: Option<String>,
    /// Node name. Optional.
    pub node: Option<String>,
    /// Namespace labels. Optional.
    pub namespace_labels: Option<String>,
    /// Pod labels. Optional.
    pub pod_labels: Option<String>,
    /// Controller kind. Optional.
    pub controller_kind: Option<String>,
    /// Controller name. Optional.
    pub controller_name: Option<String>,
    /// Controller labels. Optional.
    pub controller_labels: Option<String>,
}

/// Agent / endpoint detection details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertAgentDetectionInfo {
    /// Agent name. Optional / nullable.
    pub name: Option<String>,
    /// Site id. Optional / nullable.
    pub site_id: Option<String>,
    /// Account id. Optional / nullable.
    pub account_id: Option<String>,
    /// Agent UUID. Optional / nullable.
    pub uuid: Option<String>,
    /// OS family. Optional / nullable.
    pub os_family: Option<String>,
    /// OS revision. Optional / nullable.
    pub os_revision: Option<String>,
    /// OS name. Optional / nullable.
    pub os_name: Option<String>,
    /// Agent version. Optional / nullable.
    pub version: Option<String>,
    /// Machine type. Optional / nullable.
    pub machine_type: Option<String>,
}

/// Process details (used for both source and source-parent process info).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertProcessInfo {
    /// Process name. Optional.
    pub name: Option<String>,
    /// File signer identity. Optional.
    pub file_signer_identity: Option<String>,
    /// File path. Optional.
    pub file_path: Option<String>,
    /// PID start time. Optional.
    pub pid_starttime: Option<String>,
    /// Command line. Optional.
    pub commandline: Option<String>,
    /// Process id. Optional.
    pub pid: Option<String>,
    /// Storyline id. Optional.
    pub storyline: Option<String>,
    /// File SHA1. Optional.
    pub file_hash_sha1: Option<String>,
    /// File SHA256. Optional.
    pub file_hash_sha256: Option<String>,
    /// File MD5. Optional.
    pub file_hash_md5: Option<String>,
    /// User. Optional.
    pub user: Option<String>,
    /// Effective user. Optional.
    pub effective_user: Option<String>,
    /// Real user. Optional.
    pub real_user: Option<String>,
    /// Login user. Optional.
    pub login_user: Option<String>,
    /// Integrity level. Required (per the spec sub-schema), not nullable.
    /// Allowed values: `unknown`, `untrusted`, `low`, `medium`, `high`,
    /// `system`.
    pub integrity_level: String,
    /// Subsystem. Required (per the spec sub-schema), not nullable. Allowed
    /// values: `unknown`, `wsl`, `sys_win32`.
    pub subsystem: String,
    /// Unique id. Optional.
    pub unique_id: Option<String>,
}

/// Target process / file details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertTargetProcessInfo {
    /// Target file path. Optional.
    pub tgt_file_path: Option<String>,
    /// Target file id. Optional.
    pub tgt_file_id: Option<String>,
    /// Target file SHA1. Optional.
    pub tgt_file_hash_sha1: Option<String>,
    /// Target file SHA256. Optional.
    pub tgt_file_hash_sha256: Option<String>,
    /// Target file created at. Optional. Date/time string.
    pub tgt_file_created_at: Option<String>,
    /// Target file modified at. Optional. Date/time string.
    pub tgt_file_modified_at: Option<String>,
    /// Whether the target file is signed. Optional.
    pub tgt_file_is_signed: Option<String>,
    /// Target file old path. Optional.
    pub tgt_file_old_path: Option<String>,
    /// Target process name. Optional.
    pub tgt_proc_name: Option<String>,
    /// Target process storyline id. Optional.
    pub tgt_proc_storyline_id: Option<String>,
    /// Target process command line. Optional.
    pub tgt_proc_cmd_line: Option<String>,
    /// Target process id. Optional.
    pub tgt_proc_pid: Option<String>,
    /// Target process start time. Optional. Date/time string.
    pub tgt_process_start_time: Option<String>,
    /// Target process image path. Optional.
    pub tgt_proc_image_path: Option<String>,
    /// Target process uid. Optional.
    pub tgt_proc_uid: Option<String>,
    /// Target process integrity level. Optional. Allowed values: `unknown`,
    /// `untrusted`, `low`, `medium`, `high`, `system`.
    pub tgt_proc_integrity_level: Option<String>,
    /// Target process signed status. Optional.
    pub tgt_proc_signed_status: Option<String>,
}

/// Result envelope payload for the alert mutation endpoints
/// (`analyst-verdict`, `incident`): the number of affected entities.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertsAffected {
    /// Number of entities affected by the requested operation. Optional.
    pub affected: Option<i64>,
}
