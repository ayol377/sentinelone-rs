//! Models for the `Policies` tag.
//!
//! The single entity is the enriched policy ([`Policy`]) returned from every
//! `GET`/`PUT .../policy` endpoint (envelope-stripped from
//! `policies_schemas_EnrichedPolicySchema_200`). The response schema declares
//! no `required` fields, so every field is `Option<T>` ("default null").
//!
//! Enum-typed string fields keep `String` (not Rust enums) for forward
//! compatibility; the allowed values are documented on each field.

use serde::Deserialize;

/// An enriched SentinelOne policy (scope: tenant, account, site, or group).
///
/// Source: `policies_schemas_EnrichedPolicySchema_200.data`. No field is in the
/// schema `required` array, so all fields are optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    /// Network quarantine on.
    pub network_quarantine_on: Option<bool>,
    /// Automatic immune on/off - this value must be true since all policies are
    /// immune by default.
    pub auto_immune_on: Option<bool>,
    /// Auto decommission on.
    pub auto_decommission_on: Option<bool>,
    /// True if this is the tenant policy.
    pub is_default: Option<bool>,
    /// Cloud validation on. (Spec declares no explicit type; defaults to `true`.)
    pub cloud_validation_on: Option<serde_json::Value>,
    /// Share data with SentinelOne.
    pub research_on: Option<bool>,
    /// Default action for auto mitigation.
    pub auto_mitigation_action: Option<String>,
    /// Automatic decommission period in days (1-99).
    pub auto_decommission_days: Option<i64>,
    /// Mitigation modes. Allowed values: `detect`, `protect`.
    pub mitigation_mode: Option<String>,
    /// Timestamp of policy creation (ISO-8601, e.g. `2018-02-27T04:49:26.257525Z`).
    pub created_at: Option<String>,
    /// \[DEPRECATED\] Show endpoint notification on suspicious. Replaced by
    /// `show_suspicious` in the agent UI section.
    pub agent_notification: Option<bool>,
    /// The engines statuses.
    pub engines: Option<serde_json::Value>,
    /// Mitigation mode (suspicious). Allowed values: `detect`, `protect`.
    pub mitigation_mode_suspicious: Option<String>,
    /// If true, initiate full disk scan upon first registration.
    pub scan_new_agents: Option<bool>,
    /// Anti tampering on/off.
    pub anti_tampering_on: Option<bool>,
    /// Suspicious signed driver blocking on/off.
    pub signed_driver_blocking_on: Option<bool>,
    /// Suspicious unsigned driver blocking on/off.
    pub unsigned_driver_blocking_on: Option<bool>,
    /// Suspicious driver blocking engine on/off.
    pub driver_blocking: Option<bool>,
    /// Informational alerts on/off.
    pub informational_alerts_on: Option<bool>,
    /// \[DEPRECATED\] Show/hide Agent UI. Moved inside the agent UI section.
    pub agent_ui_on: Option<bool>,
    /// True if snapshots are enabled.
    pub snapshots_on: Option<bool>,
    /// True if logging is enabled in the agent.
    pub agent_logging_on: Option<bool>,
    /// Monitor on execute on/off.
    pub monitor_on_execute: Option<bool>,
    /// Monitor on write.
    pub monitor_on_write: Option<bool>,
    /// True if IOC is enabled.
    pub ioc: Option<bool>,
    /// The IOC attributes.
    pub ioc_attributes: Option<serde_json::Value>,
    /// The DV attributes.
    pub dv_attributes_per_event_type: Option<serde_json::Value>,
    /// IOC supported for the scope.
    pub ioc_supported: Option<bool>,
    /// Indicates the parent scope from which this policy is inherited, or `null`
    /// if it is not inherited (modified specifically for the current scope).
    /// Allowed values: `site`, `account`, `global`. Nullable.
    pub inherited_from: Option<String>,
    /// Time of the last update to the policy.
    pub updated_at: Option<String>,
    /// The user id.
    pub user_id: Option<String>,
    /// The user that created the policy.
    pub user_full_name: Option<String>,
    /// True if Remote Shell is enabled for the scope.
    pub allow_remote_shell: Option<bool>,
    /// Automatic file upload configuration. Nullable.
    pub auto_file_upload: Option<serde_json::Value>,
    /// Agent UI.
    pub agent_ui: Option<serde_json::Value>,
    /// Remote script orchestration upload limits configuration. Nullable.
    pub remote_script_orchestration: Option<serde_json::Value>,
    /// FE indication as to how to display DV policy.
    pub is_dv_policy_per_event_type: Option<bool>,
    /// Remote ops forensics configuration. Nullable.
    pub remote_ops_forensics: Option<serde_json::Value>,
    /// Determines if macros should be removed from macro threats.
    pub remove_macros: Option<bool>,
    /// Determines if malicious macros should be mitigated using ML models.
    pub remove_macros_ml: Option<bool>,
    /// Identity update interval in minutes.
    pub identity_update_interval: Option<i64>,
    /// Identity telemetry report interval in minutes.
    pub identity_report_interval: Option<i64>,
    /// Identity duplicate command consolidation interval in minutes.
    pub identity_throttling_interval: Option<i64>,
    /// Endpoint reporting level. Allowed values: `disabled`, `conservative`,
    /// `moderate`, `aggressive`.
    pub identity_endpoint_reporting: Option<String>,
    /// Identity module on/off.
    pub identity_on: Option<bool>,
    /// Forensics auto triggering configuration. Nullable.
    pub forensics_auto_triggering: Option<serde_json::Value>,
    /// Allow Unprotect By Approved Process on/off.
    pub allow_unprotect_by_approved_process: Option<bool>,
    /// Identity configuration. Nullable.
    pub identity_configuration_settings: Option<serde_json::Value>,
    /// Log collector on/off.
    pub log_collector_enabled: Option<bool>,
    /// Drift detection delay time in seconds.
    pub drift_detection_delay_time: Option<i64>,
    /// Network Protection Infrastructure on/off.
    pub network_protection_infra: Option<bool>,
    /// SMB Lateral Movement Mitigation.
    pub smb_lateral_movement_mitigation: Option<bool>,
    /// True if Firewall Control for Network Quarantine is enabled.
    pub fw_for_network_quarantine_enabled: Option<bool>,
}
