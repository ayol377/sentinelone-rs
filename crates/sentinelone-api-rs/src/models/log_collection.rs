//! Models for the `Log Collection` tag.
//!
//! Response entities for log collection rules and the per-agent-type rule
//! count. Field nullability mirrors the SentinelOne 2.1 spec exactly: a field
//! is a bare type only when it is in the schema `required` array and not
//! `x-nullable`; everything else is `Option<T>` (the API's "default null"
//! behaviour). Enum-valued strings are kept as `String` for forward
//! compatibility; their allowed values are documented inline.

use serde::Deserialize;

/// A single log collection rule.
///
/// Returned (as the `data` array element) by
/// `GET /web/api/v2.1/log-collection/rules`,
/// `GET /web/api/v2.1/log-collection/rules/{agent_type}`.
///
/// The per-rule schema declares no `required` properties, so every field is
/// `Option<T>` (default null).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionRule {
    /// Log Collection Rule ID. Example: "225494730938493804".
    pub id: Option<String>,
    /// Created at (date/time string). Example: "2018-02-27T04:49:26.257525Z".
    pub created_at: Option<String>,
    /// Created by.
    pub created_by: Option<String>,
    /// Updated at (date/time string). Example: "2018-02-27T04:49:26.257525Z".
    pub updated_at: Option<String>,
    /// Updated by.
    pub updated_by: Option<String>,
    /// Scope id. Example: "225494730938493804".
    pub scope_id: Option<String>,
    /// Scope level.
    pub scope_level: Option<String>,
    /// Scope path.
    pub scope_path: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Agent type. Allowed values (enum): `windows`, `linux`, `macos`, `k8s`.
    pub agent_type: Option<String>,
    /// Enabled.
    pub enabled: Option<bool>,
    /// Uuid.
    pub uuid: Option<String>,
    /// Linux params. Nullable.
    pub linux_params: Option<LogCollectionRuleLinuxParams>,
    /// Windows params. Nullable.
    pub windows_params: Option<LogCollectionRuleWindowsParams>,
    /// Macos params. Nullable.
    pub macos_params: Option<LogCollectionRuleMacosParams>,
    /// K8s params. Nullable.
    pub k8s_params: Option<LogCollectionRuleK8sParams>,
}

/// Linux-specific parameters of a log collection rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionRuleLinuxParams {
    /// Collection path.
    pub collection_path: Option<String>,
    /// Excluded logs.
    pub excluded_logs: Option<Vec<String>>,
    /// Parser. Nullable.
    pub parser: Option<String>,
    /// Log prefix. Nullable.
    pub log_prefix: Option<String>,
}

/// Windows-specific parameters of a log collection rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionRuleWindowsParams {
    /// Collection type. Allowed values (enum): `applicationLog`, `flatFileLog`,
    /// `windowsEventLog`.
    pub collection_type: Option<String>,
    /// Log prefix. Nullable.
    pub log_prefix: Option<String>,
    /// Parser. Nullable.
    pub parser: Option<String>,
    /// Excluded logs. Nullable.
    pub excluded_logs: Option<Vec<String>>,
    /// Collection path or channel. Required in this sub-object, not nullable.
    pub collection_path_or_channel: String,
    /// Collect xml. Nullable.
    pub collect_xml: Option<bool>,
    /// Raw xml. Nullable.
    pub raw_xml: Option<bool>,
    /// Event ids. Nullable.
    pub event_ids: Option<LogCollectionRuleWindowsEventIds>,
    /// Levels and providers. Nullable.
    pub levels_and_providers: Option<LogCollectionRuleWindowsLevelsAndProviders>,
    /// All events. Nullable.
    pub all_events: Option<LogCollectionRuleWindowsAllEvents>,
}

/// Windows `eventIds` sub-object.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionRuleWindowsEventIds {
    /// Event ids. Required in this sub-object, not nullable.
    pub event_ids: Vec<String>,
}

/// Windows `levelsAndProviders` sub-object.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionRuleWindowsLevelsAndProviders {
    /// Levels. Required in this sub-object, not nullable. Items are untyped in
    /// the spec, hence `serde_json::Value`.
    pub levels: Vec<serde_json::Value>,
    /// Providers. Nullable.
    pub providers: Option<Vec<String>>,
    /// Wel excluded ids. Nullable.
    pub wel_excluded_ids: Option<Vec<String>>,
}

/// Windows `allEvents` sub-object.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionRuleWindowsAllEvents {
    /// Wel excluded ids. Nullable.
    pub wel_excluded_ids: Option<Vec<String>>,
}

/// macOS-specific parameters of a log collection rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionRuleMacosParams {
    /// Collection path.
    pub collection_path: Option<String>,
    /// Configuration type. Allowed values (enum): `collectionPath`, `predicate`.
    pub configuration_type: Option<String>,
    /// Collection type. Allowed values (enum): `applicationLog`, `flatFileLog`,
    /// `unifiedLogging`.
    pub collection_type: Option<String>,
    /// Excluded logs.
    pub excluded_logs: Option<Vec<String>>,
    /// Parser. Nullable.
    pub parser: Option<String>,
    /// Info logs.
    pub info_logs: Option<bool>,
    /// Debug logs.
    pub debug_logs: Option<bool>,
    /// Log prefix. Nullable.
    pub log_prefix: Option<String>,
}

/// Kubernetes-specific parameters of a log collection rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionRuleK8sParams {
    /// Collection tags.
    pub collection_tags: Option<Vec<String>>,
    /// Excluded tags. Nullable.
    pub excluded_tags: Option<Vec<String>>,
    /// Parser. Nullable.
    pub parser: Option<String>,
    /// Log prefix. Nullable.
    pub log_prefix: Option<String>,
}

/// Result of an operation that affects a single rule.
///
/// Returned by `POST /web/api/v2.1/log-collection/rules` and
/// `PUT /web/api/v2.1/log-collection/rules/{rule_id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionAffectedResultId {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
    /// Id. Required, not nullable.
    pub id: String,
}

/// Result of an operation that affects multiple rules.
///
/// Returned by `DELETE /web/api/v2.1/log-collection/rules` and
/// `POST /web/api/v2.1/log-collection/rules/activation`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionAffectedResultIds {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
    /// Ids. Required, not nullable.
    pub ids: Vec<String>,
}

/// Per-agent-type rule count summary.
///
/// Returned by `GET /web/api/v2.1/log-collection/agent-type-count`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionAgentTypeCount {
    /// Agent types with their per-type counts. Required, not nullable.
    pub agent_types: Vec<LogCollectionAgentTypeCountItem>,
    /// Total rules. Required, not nullable.
    pub total_rules: i64,
}

/// A single agent-type/count pair.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogCollectionAgentTypeCountItem {
    /// Agent type. Required, not nullable.
    pub agent_type: String,
    /// Count. Required, not nullable.
    pub count: i64,
}
