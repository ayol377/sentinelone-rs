//! `Custom Detection Rule` tag — managing Custom Detection Rules.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::custom_detection_rule::*;
use crate::pagination::{Paginated, Response};

/// `Custom Detection Rule` tag — create, list, update, delete, enable and
/// disable Custom Detection Rules, and read entity-mapping options.
pub struct CustomDetectionRuleService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ===========================================================================
// Query params
// ===========================================================================

/// Query params for `GET /web/api/v2.1/cloud-detection/rules` — Get Rules.
///
/// Array params are serialized comma-joined, as the API expects. Every field
/// is optional. Many fields are documented in the spec as ignored
/// (kept for backward compatibility only); they are retained here for parity.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRulesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Optional.
    ///
    /// Allowed values: `id`, `name`, `status`, `expirationMode`, `expired`,
    /// `queryType`, `reachedLimit`, `statusReason`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Platform rule ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_rule_ids: Option<String>,
    /// List of entity ids to exclude from select_all (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_ids: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// To filter by scope. Optional.
    ///
    /// Allowed values: `group`, `global`, `site`, `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Comma-joined.
    /// Allowed item values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Comma-joined.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_surfaces: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Comma-joined.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Comma-joined.
    /// Allowed item values: `Activating`, `Active`, `Disabling`, `Disabled`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Comma-joined.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitre_tactics: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1ql_substring: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_substring: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_substring: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Comma-joined.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Comma-joined.
    /// Allowed item values: `Events`, `Correlation`, `UEBAFirstSeen`,
    /// `Scheduled`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    /// Allowed values: `Shown`, `Hidden`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_query: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Comma-joined.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    /// The active response status for the rule. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_response: Option<bool>,
    /// To filter by Rule ID (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// To filter by scope (comma-joined). Optional.
    ///
    /// Allowed item values: `group`, `global`, `site`, `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    /// To filter by status (comma-joined). Optional.
    ///
    /// Allowed item values: `Draft`, `Activating`, `Active`, `Disabling`,
    /// `Disabled`, `Deleted`, `Deleting`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// To filter by severity (comma-joined). Optional.
    ///
    /// Allowed item values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// The expiration mode. Optional. Allowed values: `Permanent`, `Temporary`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_mode: Option<String>,
    /// Rule expired or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// Rule reached limit or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reached_limit: Option<bool>,
    /// Enter a list of query types (comma-joined). Optional.
    ///
    /// Allowed item values: `events`, `correlation`, `uebafirstseen`,
    /// `scheduled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<String>,
    /// Free-text filter by rule name (comma-joined). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by rule description (comma-joined). Optional.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// Free-text filter by rule creator (comma-joined). Optional.
    #[serde(rename = "creator__contains", skip_serializing_if = "Option::is_none")]
    pub creator_contains: Option<String>,
    /// Free-text filter by S1 query (comma-joined). Optional.
    #[serde(rename = "s1ql__contains", skip_serializing_if = "Option::is_none")]
    pub s1ql_contains: Option<String>,
    /// Free-text filter by S1 query (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    /// Allowed values: `Template`, `Platform`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<String>,
    /// If True, all rules for the requested scope will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_pagination: Option<bool>,
    /// If True, alertsCount will be retrieved from Star and not from UAM.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_legacy: Option<bool>,
    /// If True and isLegacy is True, include rules whose logic is hidden.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_hidden_logic_rules: Option<bool>,
    /// If True, includes scheduled rules. Requires isLegacy=false. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_scheduled_rules: Option<bool>,
}

impl GetRulesQuery {
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by.
    ///
    /// Allowed values: `id`, `name`, `status`, `expirationMode`, `expired`,
    /// `queryType`, `reachedLimit`, `statusReason`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Platform rule ids.
    pub fn platform_rule_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.platform_rule_ids = Some(join_csv(v));
        self
    }
    /// List of entity ids to exclude from select_all.
    pub fn exclude_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.exclude_ids = Some(join_csv(v));
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// To filter by scope. Allowed values: `group`, `global`, `site`, `account`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn severities<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.severities = Some(join_csv(v));
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn attack_surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.attack_surfaces = Some(join_csv(v));
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn sources<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sources = Some(join_csv(v));
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.statuses = Some(join_csv(v));
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn mitre_tactics<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mitre_tactics = Some(join_csv(v));
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn s1ql_substring(mut self, v: impl Into<String>) -> Self {
        self.s1ql_substring = Some(v.into());
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn description_substring(mut self, v: impl Into<String>) -> Self {
        self.description_substring = Some(v.into());
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn name_substring(mut self, v: impl Into<String>) -> Self {
        self.name_substring = Some(v.into());
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags = Some(join_csv(v));
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn categories<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.categories = Some(join_csv(v));
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    /// Allowed values: `Shown`, `Hidden`.
    pub fn hide_query(mut self, v: impl Into<String>) -> Self {
        self.hide_query = Some(v.into());
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    pub fn labels<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.labels = Some(join_csv(v));
        self
    }
    /// The active response status for the rule.
    pub fn active_response(mut self, v: bool) -> Self {
        self.active_response = Some(v);
        self
    }
    /// To filter by Rule ID.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// To filter by scope.
    ///
    /// Allowed item values: `group`, `global`, `site`, `account`.
    pub fn scopes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scopes = Some(join_csv(v));
        self
    }
    /// To filter by status.
    ///
    /// Allowed item values: `Draft`, `Activating`, `Active`, `Disabling`,
    /// `Disabled`, `Deleted`, `Deleting`.
    pub fn status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.status = Some(join_csv(v));
        self
    }
    /// To filter by severity.
    ///
    /// Allowed item values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    pub fn severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.severity = Some(join_csv(v));
        self
    }
    /// The expiration mode. Allowed values: `Permanent`, `Temporary`.
    pub fn expiration_mode(mut self, v: impl Into<String>) -> Self {
        self.expiration_mode = Some(v.into());
        self
    }
    /// Rule expired or not.
    pub fn expired(mut self, v: bool) -> Self {
        self.expired = Some(v);
        self
    }
    /// Rule reached limit or not.
    pub fn reached_limit(mut self, v: bool) -> Self {
        self.reached_limit = Some(v);
        self
    }
    /// Enter a list of query types.
    ///
    /// Allowed item values: `events`, `correlation`, `uebafirstseen`,
    /// `scheduled`.
    pub fn query_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.query_type = Some(join_csv(v));
        self
    }
    /// Free-text filter by rule name.
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by rule description.
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by rule creator.
    pub fn creator_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.creator_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by S1 query.
    pub fn s1ql_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.s1ql_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by S1 query.
    pub fn query<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.query = Some(join_csv(v));
        self
    }
    /// Ignored by the API (kept for backward compatibility).
    /// Allowed values: `Template`, `Platform`.
    pub fn filter_by(mut self, v: impl Into<String>) -> Self {
        self.filter_by = Some(v.into());
        self
    }
    /// If True, all rules for the requested scope will be returned.
    pub fn disable_pagination(mut self, v: bool) -> Self {
        self.disable_pagination = Some(v);
        self
    }
    /// If True, alertsCount will be retrieved from Star and not from UAM.
    pub fn is_legacy(mut self, v: bool) -> Self {
        self.is_legacy = Some(v);
        self
    }
    /// If True and isLegacy is True, include rules whose logic is hidden.
    pub fn include_hidden_logic_rules(mut self, v: bool) -> Self {
        self.include_hidden_logic_rules = Some(v);
        self
    }
    /// If True, includes scheduled rules. Requires isLegacy=false.
    pub fn include_scheduled_rules(mut self, v: bool) -> Self {
        self.include_scheduled_rules = Some(v);
        self
    }
}

fn join_csv<I, S>(v: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    v.into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

// ===========================================================================
// Body params
// ===========================================================================

/// Body for `POST` / `PUT` (create / update) — `v2_1.rules.schemas_PostRuleSchema`.
///
/// Both `data` and `filter` are required by the spec.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRuleBody {
    /// The rule definition. Required.
    pub data: PostRuleData,
    /// The scope selector for the rule. Required.
    pub filter: PostRuleFilter,
}

/// The `data` object of a create/update request.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRuleData {
    /// The name of the custom detection rule. Required.
    pub name: String,
    /// A description of the custom detection rule (max 2000). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The rule severity. Required.
    ///
    /// Allowed values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    pub severity: String,
    /// Defines the rule as Permanent or Temporary. Required.
    ///
    /// Allowed values: `Permanent`, `Temporary`.
    pub expiration_mode: String,
    /// If Temporary, the expiration date for the rule (ISO-8601).
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// The query type. Required.
    ///
    /// Allowed values: `events`, `correlation`, `uebafirstseen`, `scheduled`.
    pub query_type: String,
    /// The query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1ql: Option<String>,
    /// Enabled or Disabled. Required.
    ///
    /// Allowed values: `Draft`, `Activating`, `Active`, `Disabling`,
    /// `Disabled`, `Deleted`, `Deleting`.
    pub status: String,
    /// Set to True to automatically quarantine the alerted endpoints
    /// (default false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_quarantine: Option<bool>,
    /// The Treat as a threat auto response. Optional.
    ///
    /// Allowed values: `UNDEFINED`, `Suspicious`, `Malicious`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_as_threat: Option<String>,
    /// The s1ql version query language of the rule (default `1.0`). Optional.
    ///
    /// Allowed values: `1.0`, `2.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_lang: Option<String>,
    /// Correlation params. Optional (free-form object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_params: Option<serde_json::Value>,
    /// Scheduled params. Optional (free-form object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_params: Option<serde_json::Value>,
    /// Cool-off (alert suppression) settings. Optional (free-form object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cool_off_settings: Option<serde_json::Value>,
    /// If true, hide this rule logic from non-privileged users. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_logic: Option<bool>,
    /// List of OCSF entity mappings (max 10). Optional/nullable (free-form
    /// objects).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_mappings: Option<Vec<serde_json::Value>>,
    /// Rule ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Rule Id of template rule from which this rule was created. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_rule_id: Option<String>,
}

/// The `filter` (scope selector) object of a create/update request.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRuleFilter {
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant (Global) scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

/// Body for `DELETE /web/api/v2.1/cloud-detection/rules` —
/// `v2_1.rules.schemas_RuleDeleteSchema`. `filter` is required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleDeleteBody {
    /// The rule selection filter. Required.
    pub filter: RuleFilter,
}

/// Body for `PUT .../enable` and `PUT .../disable` —
/// `v2_1.rules.schemas_FilterRuleSchema`. `filter` is required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterRuleBody {
    /// The rule selection filter. Required.
    pub filter: RuleFilter,
}

/// The shared rule-selection `filter` object used by delete/enable/disable.
///
/// Every field is optional. Several fields are documented as ignored
/// (kept for backward compatibility); retained for parity.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleFilter {
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Platform rule ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_rule_ids: Option<Vec<String>>,
    /// List of entity ids to exclude from select_all. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_ids: Option<Vec<String>>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// To filter by scope. Optional.
    /// Allowed values: `group`, `global`, `site`, `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    /// Allowed item values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<Vec<String>>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_surfaces: Option<Vec<String>>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    /// Allowed item values: `Activating`, `Active`, `Disabling`, `Disabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitre_tactics: Option<Vec<String>>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1ql_substring: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_substring: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_substring: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    /// Allowed item values: `Events`, `Correlation`, `UEBAFirstSeen`,
    /// `Scheduled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    /// Allowed values: `Shown`, `Hidden`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_query: Option<String>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// The active response status for the rule. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_response: Option<bool>,
    /// To filter by Rule ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// To filter by scope. Optional.
    /// Allowed item values: `group`, `global`, `site`, `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// To filter by status. Optional.
    /// Allowed item values: `Draft`, `Activating`, `Active`, `Disabling`,
    /// `Disabled`, `Deleted`, `Deleting`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,
    /// To filter by severity. Optional.
    /// Allowed item values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Vec<String>>,
    /// The expiration mode. Optional. Allowed values: `Permanent`, `Temporary`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_mode: Option<String>,
    /// Rule expired or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,
    /// Rule reached limit or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reached_limit: Option<bool>,
    /// Enter a list of query types. Optional.
    /// Allowed item values: `events`, `correlation`, `uebafirstseen`,
    /// `scheduled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<Vec<String>>,
    /// Free-text filter by rule name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<Vec<String>>,
    /// Free-text filter by rule description. Optional.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<Vec<String>>,
    /// Free-text filter by rule creator. Optional.
    #[serde(rename = "creator__contains", skip_serializing_if = "Option::is_none")]
    pub creator_contains: Option<Vec<String>>,
    /// Free-text filter by S1 query. Optional.
    #[serde(rename = "s1ql__contains", skip_serializing_if = "Option::is_none")]
    pub s1ql_contains: Option<Vec<String>>,
    /// Free-text filter by S1 query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Vec<String>>,
    /// Ignored by the API (kept for backward compatibility). Optional.
    /// Allowed values: `Template`, `Platform`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<String>,
}

// ===========================================================================
// Service methods
// ===========================================================================

impl CustomDetectionRuleService<'_> {
    /// `GET /web/api/v2.1/cloud-detection/rules` — Get Rules.
    ///
    /// Get a list of Custom Detection Rules for a given scope. Note: You can
    /// create and see rules only for your highest available scope. For example,
    /// if your username has an access level of scope Account, you cannot see
    /// rules created for the Global scope or rules created for a specific Site.
    pub async fn list(
        &self,
        query: &GetRulesQuery,
    ) -> Result<Paginated<CustomDetectionRule>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cloud-detection/rules", q)
            .await?)
    }

    /// `POST /web/api/v2.1/cloud-detection/rules` — Create Rule.
    ///
    /// Create a Custom Detection Rule for a scope specified by ID. To get the
    /// ID, run "accounts", "sites", "groups", or set "tenant" to "true" for
    /// Global.
    pub async fn create(
        &self,
        body: &PostRuleBody,
    ) -> Result<Response<CustomDetectionRule>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/cloud-detection/rules", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/cloud-detection/rules` — Delete Rules.
    ///
    /// Deletes Custom Detection Rules that match a filter.
    pub async fn delete(
        &self,
        body: &RuleDeleteBody,
    ) -> Result<Response<CustomDetectionRuleAffected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<RuleDeleteBody, Response<CustomDetectionRuleAffected>>(
                Method::DELETE,
                "/web/api/v2.1/cloud-detection/rules",
                None,
                Some(body),
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/cloud-detection/rules/disable` — Disable Rules.
    ///
    /// Disable Custom Detection Rules based on a filter.
    pub async fn disable(
        &self,
        body: &FilterRuleBody,
    ) -> Result<Response<CustomDetectionRuleAffected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<FilterRuleBody, Response<CustomDetectionRuleAffected>>(
                Method::PUT,
                "/web/api/v2.1/cloud-detection/rules/disable",
                None,
                Some(body),
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/cloud-detection/rules/enable` — Activate Rules.
    ///
    /// Activate Custom Detection Rules based on a filter.
    pub async fn enable(
        &self,
        body: &FilterRuleBody,
    ) -> Result<Response<CustomDetectionRuleAffected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<FilterRuleBody, Response<CustomDetectionRuleAffected>>(
                Method::PUT,
                "/web/api/v2.1/cloud-detection/rules/enable",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/cloud-detection/rules/entity-mapping-options` —
    /// Get Entity Mapping Options.
    ///
    /// Get available entity mapping options. Returns valid asset surfaces,
    /// their properties, and limits.
    pub async fn entity_mapping_options(
        &self,
    ) -> Result<Response<CustomDetectionRuleEntityMappingOptions>, Error> {
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/cloud-detection/rules/entity-mapping-options",
                None,
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/cloud-detection/rules/{rule_id}` — Update Rule.
    ///
    /// Change a Custom Detection rule. This command requires the rule ID
    /// (see Get Rules).
    pub async fn update(
        &self,
        rule_id: impl Into<String>,
        body: &PostRuleBody,
    ) -> Result<Response<CustomDetectionRule>, Error> {
        let path = format!(
            "/web/api/v2.1/cloud-detection/rules/{}",
            rule_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<PostRuleBody, Response<CustomDetectionRule>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }
}
