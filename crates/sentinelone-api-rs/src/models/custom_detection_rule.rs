//! Models for the `Custom Detection Rule` tag.
//!
//! The list endpoint (`RuleViewSchema`) and the single-resource create/update
//! endpoints (`RuleResponseSchema`) return the same logical entity. A handful
//! of fields (e.g. `generatedAlerts`, `siteName`) only appear on the list view;
//! they are modelled as `Option<T>` here so a single [`CustomDetectionRule`]
//! struct can deserialize both shapes.

use serde::Deserialize;

/// A Custom Detection Rule.
///
/// Field nullability follows the spec: a field is bare `T` only when it is in
/// the schema `required` array and not `x-nullable`; everything else is
/// `Option<T>` (default-null behaviour).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRule {
    /// Rule ID. Optional/nullable (not in `required`).
    pub id: Option<String>,
    /// The name of the custom detection rule. Required.
    pub name: String,
    /// The description of the custom detection rule. Optional/nullable.
    pub description: Option<String>,
    /// The rule severity in your environment. Required.
    ///
    /// Allowed values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    pub severity: String,
    /// Whether the rule is Temporary or Permanent. Required.
    ///
    /// Allowed values: `Permanent`, `Temporary`.
    pub expiration_mode: String,
    /// If Temporary, the expiration date for the rule (ISO-8601). Optional/nullable.
    pub expiration: Option<String>,
    /// The query type. Optional/nullable.
    ///
    /// Allowed values: `events`, `correlation`, `uebafirstseen`, `scheduled`.
    pub query_type: Option<String>,
    /// The query. Optional (not in `required`).
    pub s1ql: Option<String>,
    /// Enabled (Activated and sends alerts if triggered) or Disabled. Required.
    ///
    /// Allowed values: `Draft`, `Activating`, `Active`, `Disabling`,
    /// `Disabled`, `Deleted`, `Deleting`.
    pub status: String,
    /// The scope of the rule. Optional (read-only).
    ///
    /// Allowed values: `group`, `global`, `site`, `account`.
    pub scope: Option<String>,
    /// The Account, Site, or Group ID, depending on the scope. Null if the
    /// scope is Global. Optional.
    pub scope_id: Option<Vec<String>>,
    /// True if the rule can be modified at this scope level. Optional.
    pub editable: Option<bool>,
    /// The date the rule was created (ISO-8601). Optional.
    pub created_at: Option<String>,
    /// The date the rule was last updated (ISO-8601). Optional.
    pub updated_at: Option<String>,
    /// The full name of the user that created the rule. Optional.
    pub creator: Option<String>,
    /// The ID of the user that created the rule. Optional.
    pub creator_id: Option<String>,
    /// The ID of the user that last updated the rule. Optional.
    pub updater_id: Option<String>,
    /// True if the Rule has expired. Optional.
    pub expired: Option<bool>,
    /// True if the Rule reached the 5k/hour or 10k/day alert limit. Optional.
    pub reached_limit: Option<bool>,
    /// The reason why the Rule has its current status. Optional.
    pub status_reason: Option<String>,
    /// True if the network quarantine is on. Optional.
    pub network_quarantine: Option<bool>,
    /// The Treat as threat auto response. Optional.
    ///
    /// Allowed values: `UNDEFINED`, `Suspicious`, `Malicious`.
    pub treat_as_threat: Option<String>,
    /// The s1ql version query language of the rule. Optional/nullable.
    ///
    /// Allowed values: `1.0`, `2.0`.
    pub query_lang: Option<String>,
    /// Correlation params. Optional.
    pub correlation_params: Option<CustomDetectionRuleCorrelationParams>,
    /// Scheduled params. Optional.
    pub scheduled_params: Option<CustomDetectionRuleScheduledParams>,
    /// Rule Id of template rule from which this rule was created. Optional
    /// (read-only, untyped in spec).
    pub template_rule_id: Option<serde_json::Value>,
    /// Cool-off (alert suppression) settings. Optional.
    pub cool_off_settings: Option<CustomDetectionRuleCoolOffSettings>,
    /// If true, this rule's logic is hidden from non-privileged users. Optional.
    pub hide_logic: Option<bool>,
    /// List of OCSF entity mappings to associate alerts with specific assets.
    /// Optional.
    pub entity_mappings: Option<Vec<CustomDetectionRuleEntityMapping>>,
    /// The number of alerts generated for the Rule (list view only). Optional.
    pub generated_alerts: Option<i64>,
    /// The time of the last alert for the Rule (list view only). Optional.
    pub last_alert_time: Option<String>,
    /// The Active Response status of the Rule (list view only). Optional.
    pub active_response: Option<bool>,
    /// The source of the rule. Present for Detection-as-Code managed rules.
    /// Optional. Allowed values: `DAC`.
    pub source: Option<String>,
    /// Enrichment information (list view only). Optional.
    pub enrichment: Option<CustomDetectionRuleEnrichment>,
    /// The site ID (list view only). Optional.
    pub site_id: Option<String>,
    /// The name of the site (list view only). Optional.
    pub site_name: Option<String>,
    /// The account ID (list view only). Optional.
    pub account_id: Option<String>,
    /// The name of the account (list view only). Optional.
    pub account_name: Option<String>,
}

/// Correlation params for a Custom Detection Rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRuleCorrelationParams {
    /// The list of subqueries for the custom detection rule. Optional.
    pub sub_queries: Option<Vec<CustomDetectionRuleSubQuery>>,
    /// A common entity used to group matching events. Required.
    ///
    /// Allowed values: `user`, `process`, `ip`, `endpoint`, `storyline`,
    /// `custom`, `none`.
    pub entity: String,
    /// Set to True to require subqueries to match in sequence. Required.
    pub match_in_order: bool,
    /// The period of time in which subqueries must match. Optional.
    pub time_window: Option<CustomDetectionRuleTimeWindow>,
}

/// A correlation subquery.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRuleSubQuery {
    /// A subquery. Required.
    pub sub_query: String,
    /// The number of times a subquery must match (1-1000). Required.
    pub matches_required: i64,
}

/// Correlation time window.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRuleTimeWindow {
    /// The period of time in minutes in which subqueries must match. Optional.
    ///
    /// Allowed values: `10`, `30`, `60`, `240`, `480`, `720`.
    pub window_minutes: Option<i64>,
}

/// Scheduled params for a Custom Detection Rule.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRuleScheduledParams {
    /// The PowerQuery to run. Required.
    pub query: String,
    /// How often the alert is evaluated, in minutes (1-43200). Required.
    pub run_interval_minutes: i64,
    /// The evaluation lookback window, in minutes (1-43200). Required.
    pub lookback_window_minutes: i64,
    /// Threshold condition for an alert to be triggered. Optional.
    pub threshold: Option<CustomDetectionRuleThreshold>,
    /// If True, an alert is generated for each row detected. Optional.
    pub alert_per_row: Option<bool>,
    /// If True, disables streak counting logic for scheduled rules. Optional.
    pub disable_streaks_logic: Option<bool>,
}

/// Scheduled-rule threshold.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRuleThreshold {
    /// The threshold value (default 0). Optional.
    pub value: Option<i64>,
    /// The operator. Optional. Allowed values: `Greater`, `Less`, `Equal`.
    pub operator: Option<String>,
}

/// Cool-off (alert suppression) settings.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRuleCoolOffSettings {
    /// The period of time in minutes in which alerts are suppressed after the
    /// rule has been triggered (1-1440). Required.
    pub renotify_minutes: i64,
}

/// An OCSF entity mapping associating alerts with specific assets.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRuleEntityMapping {
    /// Unique case-insensitive column name from PowerQuery table result
    /// (required for Scheduled rules). Optional.
    pub column_name: Option<String>,
    /// Asset surface. Required. Allowed values: `Endpoint`, `Cloud`, `Identity`.
    pub asset_surface: String,
    /// Asset unique identifier property (e.g., `device_uuid`, `hostname_fqdn`
    /// for Endpoint). Required.
    pub asset_property: String,
}

/// Enrichment information attached to a rule (list view only).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRuleEnrichment {
    /// The ID of the user that created the Rule. Optional.
    pub creator: Option<String>,
    /// The ID of the user that last updated the Rule. Optional.
    pub updater: Option<String>,
    /// Scope name. Optional.
    pub scope_name: Option<String>,
}

/// Response data for
/// `GET /web/api/v2.1/cloud-detection/rules/entity-mapping-options`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRuleEntityMappingOptions {
    /// List of valid asset surfaces. Optional.
    ///
    /// Allowed item values: `Endpoint`, `Cloud`, `Identity`.
    pub asset_surfaces: Option<Vec<String>>,
    /// Map of asset surface to list of valid properties. When creating an
    /// entity mapping, select one surface and one property from that surface.
    /// Optional (free-form object).
    pub asset_properties: Option<serde_json::Value>,
    /// Maximum number of entity mappings allowed per rule. Optional.
    pub max_entity_mappings: Option<i64>,
}

/// Result of an action that affects a number of rules
/// (delete/enable/disable). Mirrors `{ "affected": <int> }`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomDetectionRuleAffected {
    /// Number of entities affected by the requested operation. Optional.
    pub affected: Option<i64>,
}
