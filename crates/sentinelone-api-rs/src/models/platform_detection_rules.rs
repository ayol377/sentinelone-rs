//! Models for the `Platform Detection Rules` tag (Manage Platform Detection
//! Rules).
//!
//! Source of truth: `swagger_2_1.json` definitions
//! `v2_1.gdl.schemas_*` and `_AffectedResultsSchema_200` /
//! `_FreeTextFilterResponseSchema_many_200`. Each response wraps the entity
//! under `.data`; these structs model that inner `data` object (the array item
//! for list endpoints) and are returned via [`crate::pagination::Paginated`] or
//! [`crate::pagination::Response`].
//!
//! Fidelity note: a field is a bare `T` only when it appears in the schema's
//! `required` array (and is not `x-nullable`); every other field is modelled as
//! `Option<T>` (the spec's "default null" behaviour). Enum-typed fields are
//! `String` for forward-compatibility, with allowed values documented inline.

use serde::Deserialize;

// ---------------------------------------------------------------------------
// Shared nested objects
// ---------------------------------------------------------------------------

/// A MITRE tactic (and its techniques) associated with a rule.
///
/// Spec: `data.items.properties.mitre.items`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRuleMitre {
    /// The mitre tactic.
    ///
    /// Optional/nullable -> `Option`.
    pub tactic: Option<String>,
    /// The mitre techniques.
    ///
    /// Optional/nullable -> `Option`.
    pub techniques: Option<Vec<PlatformRuleMitreTechnique>>,
}

/// A MITRE technique nested under a [`PlatformRuleMitre`] tactic.
///
/// Spec: `data.items.properties.mitre.items.properties.techniques.items`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRuleMitreTechnique {
    /// The mitre technique id.
    ///
    /// Optional/nullable -> `Option`.
    pub id: Option<String>,
    /// The mitre technique title.
    ///
    /// Optional/nullable -> `Option`.
    pub title: Option<String>,
    /// The mitre technique link.
    ///
    /// Optional/nullable -> `Option`.
    pub link: Option<String>,
}

/// Cool-off (alert suppression) settings for a rule.
///
/// Receive only one alert and suppress additional alerts when a rule is
/// triggered multiple times during the cool-off period.
///
/// Spec: `data.items.properties.coolOffSettings`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRuleCoolOffSettings {
    /// The period of time in minutes in which alerts will be suppressed after
    /// the rule has been triggered (1-1440).
    ///
    /// Required (inner object `required`) + not `x-nullable` -> bare `i64`.
    pub renotify_minutes: i64,
}

/// A single subquery of a correlation rule.
///
/// Spec: `data.items.properties.correlationParams.properties.subQueries.items`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRuleSubQuery {
    /// A subquery.
    ///
    /// Required (inner object `required`) + not `x-nullable` -> bare `String`.
    pub sub_query: String,
    /// The number of times a subquery must match (1-1000).
    ///
    /// Required (inner object `required`) + not `x-nullable` -> bare `i64`.
    pub matches_required: i64,
}

/// Time window for a correlation rule.
///
/// Spec: `data.items.properties.correlationParams.properties.timeWindow`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRuleTimeWindow {
    /// The period of time in minutes in which subqueries must match to trigger
    /// an alert. Allowed values: `10`, `30`, `60`, `240`, `480`, `720`.
    ///
    /// Optional/nullable -> `Option`.
    pub window_minutes: Option<i64>,
}

/// Correlation parameters for a correlation-type rule.
///
/// Spec: `data.items.properties.correlationParams`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRuleCorrelationParams {
    /// The list of subqueries for the custom detection rule (1-10 items).
    ///
    /// Optional/nullable -> `Option`.
    pub sub_queries: Option<Vec<PlatformRuleSubQuery>>,
    /// A common entity used to group matching events. Allowed values: `user`,
    /// `process`, `ip`, `endpoint`, `storyline`, `custom`, `none`.
    ///
    /// Required (inner object `required`) + not `x-nullable` -> bare `String`.
    pub entity: String,
    /// Set to `true` to require subqueries to match in sequence to trigger an
    /// alert.
    ///
    /// Required (inner object `required`) + not `x-nullable` -> bare `bool`.
    pub match_in_order: bool,
    /// The period of time in minutes in which subqueries must match to trigger
    /// an alert.
    ///
    /// Optional/nullable -> `Option`.
    pub time_window: Option<PlatformRuleTimeWindow>,
}

/// Threshold condition for a scheduled rule.
///
/// Spec: `data.items.properties.scheduledParams.properties.threshold`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRuleThreshold {
    /// Defines the threshold value that, combined with `operator` and the
    /// query, sets the threshold condition for an alert. Default `0`.
    ///
    /// Optional/nullable -> `Option`.
    pub value: Option<i64>,
    /// Defines the operator to use in combination with the query and threshold
    /// value. Allowed values: `Greater`, `Less`, `Equal`. Default `Greater`.
    ///
    /// Optional/nullable -> `Option`.
    pub operator: Option<String>,
}

/// Scheduled parameters for a scheduled-type rule.
///
/// Spec: `data.items.properties.scheduledParams`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRuleScheduledParams {
    /// The PowerQuery to run.
    ///
    /// Required (inner object `required`) + not `x-nullable` -> bare `String`.
    pub query: String,
    /// How often the alert is evaluated, in minutes (1-43200).
    ///
    /// Required (inner object `required`) + not `x-nullable` -> bare `i64`.
    pub run_interval_minutes: i64,
    /// Sets the evaluation lookback window in minutes (1-43200).
    ///
    /// Required (inner object `required`) + not `x-nullable` -> bare `i64`.
    pub lookback_window_minutes: i64,
    /// Receive an alert only if the number of rows in the result of the query
    /// matches the condition in the threshold.
    ///
    /// Optional/nullable -> `Option`.
    pub threshold: Option<PlatformRuleThreshold>,
    /// If `true`, an alert will be generated for each row detected in the
    /// defined lookback window. Default `false`.
    ///
    /// Optional/nullable -> `Option`.
    pub alert_per_row: Option<bool>,
    /// If `true`, disables streak counting logic for scheduled rules. Default
    /// `false`.
    ///
    /// Optional/nullable -> `Option`.
    pub disable_streaks_logic: Option<bool>,
}

// ---------------------------------------------------------------------------
// Managed / Catalog rule entity
// ---------------------------------------------------------------------------

/// A Managed (Platform) / Catalog Detection Rule.
///
/// Spec definition: `v2_1.gdl.schemas_PlatformRuleDetailsSchema_many_200.data.items`.
/// Returned by `GET /detection-library/platform-rules` and
/// `GET /detection-library/rules`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRule {
    /// The platform rule ID.
    ///
    /// Optional/nullable -> `Option`.
    pub id: Option<String>,
    /// The rule name.
    ///
    /// Optional/nullable -> `Option`.
    pub name: Option<String>,
    /// The mitre tactics associated with the rule.
    ///
    /// Optional/nullable -> `Option`.
    pub mitre: Option<Vec<PlatformRuleMitre>>,
    /// The rule description.
    ///
    /// Optional/nullable -> `Option`.
    pub description: Option<String>,
    /// The severity of the rule. Allowed values: `Info`, `Low`, `Medium`,
    /// `High`, `Critical`.
    ///
    /// Optional/nullable -> `Option`.
    pub severity: Option<String>,
    /// The vendor who created the rule.
    ///
    /// Optional/nullable -> `Option`.
    pub created_by: Option<String>,
    /// The attack surfaces associated with the rule.
    ///
    /// Optional/nullable -> `Option`.
    pub attack_surfaces: Option<Vec<String>>,
    /// The sources associated with the rule.
    ///
    /// Optional/nullable -> `Option`.
    pub sources: Option<Vec<String>>,
    /// Cool-off (alert suppression) settings.
    ///
    /// Optional/nullable -> `Option`.
    pub cool_off_settings: Option<PlatformRuleCoolOffSettings>,
    /// The time when the rule was created (date-time string).
    ///
    /// Optional/nullable -> `Option`.
    pub created_at: Option<String>,
    /// The time when the rule was updated (date-time string).
    ///
    /// Optional/nullable -> `Option`.
    pub updated_at: Option<String>,
    /// The time when the last alert was caught (date-time string).
    ///
    /// Optional/nullable -> `Option`.
    pub last_alert_time: Option<String>,
    /// The scope level the rule is enabled for. Allowed values: `group`,
    /// `global`, `site`, `account`.
    ///
    /// Optional/nullable -> `Option`.
    pub scope_level: Option<String>,
    /// Enabled (Activated and sends alerts if triggered) or Disabled. Allowed
    /// values: `Draft`, `Activating`, `Active`, `Disabling`, `Disabled`,
    /// `Deleted`, `Deleting`.
    ///
    /// Required (`data.items.required`) + not `x-nullable` -> bare `String`.
    pub status: String,
    /// The number of alerts generated by the rule.
    ///
    /// Optional/nullable -> `Option`.
    pub generated_alerts: Option<i64>,
    /// The highest scope level the rule inherits from. Allowed values: `group`,
    /// `global`, `site`, `account`.
    ///
    /// Optional/nullable -> `Option`.
    pub highest_inherited_scope_level: Option<String>,
    /// Define the query type: Correlation (made of multiple subqueries), Event
    /// (single query), or Processes (Deprecated). Allowed values: `events`,
    /// `correlation`, `uebafirstseen`, `scheduled`.
    ///
    /// Optional/nullable -> `Option`.
    pub query_type: Option<String>,
    /// The query.
    ///
    /// Optional/nullable -> `Option`.
    pub s1ql: Option<String>,
    /// Correlation params.
    ///
    /// Optional/nullable -> `Option`.
    pub correlation_params: Option<PlatformRuleCorrelationParams>,
    /// Scheduled params.
    ///
    /// Optional/nullable -> `Option`.
    pub scheduled_params: Option<PlatformRuleScheduledParams>,
    /// The associated rule IDs that have not been deleted.
    ///
    /// Optional/nullable -> `Option`.
    pub not_deleted_rule_ids: Option<Vec<String>>,
    /// The associated rule IDs that have already been deleted.
    ///
    /// Optional/nullable -> `Option`.
    pub deleted_rule_ids: Option<Vec<String>>,
    /// The tags associated with the rule. Tags are used to group rules
    /// together.
    ///
    /// Optional/nullable -> `Option`.
    pub tags: Option<Vec<String>>,
    /// Labels (read-only; freeform shape in the spec).
    ///
    /// Optional/nullable -> `Option`.
    pub labels: Option<serde_json::Value>,
    /// The Active Response status of the Rule.
    ///
    /// Optional/nullable -> `Option`.
    pub active_response: Option<bool>,
    /// `true` if the network quarantine is on.
    ///
    /// Optional/nullable -> `Option`.
    pub network_quarantine: Option<bool>,
    /// The Treat-as-threat auto response. Allowed values: `UNDEFINED`,
    /// `Suspicious`, `Malicious`.
    ///
    /// Optional/nullable -> `Option`.
    pub treat_as_threat: Option<String>,
}

// ---------------------------------------------------------------------------
// Template rule entity
// ---------------------------------------------------------------------------

/// A Template Detection Rule.
///
/// Spec definition: `v2_1.gdl.schemas_TemplateRuleDetailsSchema_many_200.data.items`.
/// Returned by `GET /detection-library/template-rules`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateRule {
    /// The platform rule ID.
    ///
    /// Optional/nullable -> `Option`.
    pub id: Option<String>,
    /// The rule name.
    ///
    /// Optional/nullable -> `Option`.
    pub name: Option<String>,
    /// The mitre tactics associated with the rule.
    ///
    /// Optional/nullable -> `Option`.
    pub mitre: Option<Vec<PlatformRuleMitre>>,
    /// The rule description.
    ///
    /// Optional/nullable -> `Option`.
    pub description: Option<String>,
    /// The severity of the rule. Allowed values: `Info`, `Low`, `Medium`,
    /// `High`, `Critical`.
    ///
    /// Optional/nullable -> `Option`.
    pub severity: Option<String>,
    /// The vendor who created the rule.
    ///
    /// Optional/nullable -> `Option`.
    pub created_by: Option<String>,
    /// The attack surfaces associated with the rule.
    ///
    /// Optional/nullable -> `Option`.
    pub attack_surfaces: Option<Vec<String>>,
    /// The sources associated with the rule.
    ///
    /// Optional/nullable -> `Option`.
    pub sources: Option<Vec<String>>,
    /// Cool-off (alert suppression) settings.
    ///
    /// Optional/nullable -> `Option`.
    pub cool_off_settings: Option<PlatformRuleCoolOffSettings>,
    /// The time when the rule was created (date-time string).
    ///
    /// Optional/nullable -> `Option`.
    pub created_at: Option<String>,
    /// The time when the rule was updated (date-time string).
    ///
    /// Optional/nullable -> `Option`.
    pub updated_at: Option<String>,
    /// Correlation params.
    ///
    /// Optional/nullable -> `Option`.
    pub correlation_params: Option<PlatformRuleCorrelationParams>,
    /// Scheduled params.
    ///
    /// Optional/nullable -> `Option`.
    pub scheduled_params: Option<PlatformRuleScheduledParams>,
    /// Define the query type: Correlation (made of multiple subqueries), Event
    /// (single query). Allowed values: `events`, `correlation`,
    /// `uebafirstseen`, `scheduled`.
    ///
    /// Required (`data.items.required`) + not `x-nullable` -> bare `String`.
    pub query_type: String,
    /// The query.
    ///
    /// Optional/nullable -> `Option`.
    pub s1ql: Option<String>,
    /// Indicates if the user has created at least one rule from this template.
    ///
    /// Optional/nullable -> `Option`.
    pub in_use: Option<bool>,
}

// ---------------------------------------------------------------------------
// Action result
// ---------------------------------------------------------------------------

/// Result of an enable/disable action.
///
/// Spec definition: `_AffectedResultsSchema_200.data`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResult {
    /// Number of entities affected by the requested operation.
    ///
    /// Optional/nullable -> `Option`.
    pub affected: Option<i64>,
}

// ---------------------------------------------------------------------------
// Free-text filter metadata
// ---------------------------------------------------------------------------

/// Metadata describing one available free-text filter.
///
/// Spec definition: `_FreeTextFilterResponseSchema_many_200.data.items`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeTextFilter {
    /// Filter argument key. Example: `"computerName__contains"`.
    ///
    /// Optional/nullable -> `Option`.
    pub key: Option<String>,
    /// Filter description. Example: `"Computer name"`.
    ///
    /// Optional/nullable -> `Option`.
    pub title: Option<String>,
    /// API path for auto-complete query (if applicable).
    ///
    /// Optional/nullable -> `Option`.
    pub auto_complete: Option<String>,
    /// A regular expression for values validation.
    ///
    /// Optional/nullable -> `Option`.
    pub validation: Option<String>,
    /// Filter icon. Example: `"upload"`.
    ///
    /// Optional/nullable -> `Option`.
    pub icon: Option<String>,
}

// ---------------------------------------------------------------------------
// Platform settings
// ---------------------------------------------------------------------------

/// Managed Detection Rule settings for a scope.
///
/// Spec definition: `v2_1.gdl.schemas_PlatformSettingsGetResponseSchema_200.data`.
/// Returned by `GET /detection-library/platform-rules/settings`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformSettings {
    /// The Account or Site ID, depending on the scope. `null` if the scope is
    /// Global.
    ///
    /// Optional/nullable -> `Option`.
    pub scope_id: Option<String>,
    /// Scope level. Allowed values: `group`, `global`, `site`, `account`.
    ///
    /// Required (`data.required`) + not `x-nullable` -> bare `String`.
    pub scope_level: String,
    /// Set to `true` to disable Platform Detection settings inheritance from
    /// the parent scope.
    ///
    /// Optional/nullable -> `Option`.
    pub disable_inheritance: Option<bool>,
    /// Set to `On` to enable all smart default Platform Detection Rules. Allowed
    /// values: `Off`, `On`.
    ///
    /// Optional/nullable -> `Option`.
    pub smart_default: Option<String>,
    /// Set to `On` to auto enable all future Emerging Threat Platform Detection
    /// Rules. Allowed values: `Off`, `On`.
    ///
    /// Optional/nullable -> `Option`.
    pub emerging_threat: Option<String>,
    /// Set to `On` to enable all auto default Platform Detection Rules. Allowed
    /// values: `Off`, `On`.
    ///
    /// Optional/nullable -> `Option`.
    pub auto_default: Option<String>,
    /// Set to `On` to enable all Core Platform Detection Rules. Allowed values:
    /// `Off`, `On`.
    ///
    /// Optional/nullable -> `Option`.
    pub core: Option<String>,
    /// Is inheritance configuration available for the provided scope?
    ///
    /// Required (`data.required`) + not `x-nullable` -> bare `bool`.
    pub inheritance_available: bool,
    /// Number of rules tagged as emerging threats.
    ///
    /// Optional/nullable -> `Option`.
    pub emerging_threat_count: Option<i64>,
    /// Number of rules tagged as auto-default.
    ///
    /// Optional/nullable -> `Option`.
    pub auto_default_count: Option<i64>,
    /// Number of rules tagged as core.
    ///
    /// Optional/nullable -> `Option`.
    pub core_count: Option<i64>,
    /// Total number of unique rules with either `emergingThreat` or
    /// `autoDefault` tags.
    ///
    /// Optional/nullable -> `Option`.
    pub combined_tag_count: Option<i64>,
    /// Total number of unique rules with labels (same as `combinedTagCount`,
    /// for frontend compatibility).
    ///
    /// Optional/nullable -> `Option`.
    pub combined_label_count: Option<i64>,
}
