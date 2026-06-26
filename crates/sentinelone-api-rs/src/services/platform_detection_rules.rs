use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::platform_detection_rules::{
    AffectedResult, FreeTextFilter, PlatformRule, PlatformSettings, TemplateRule,
};
use crate::pagination::{Paginated, Response};

/// `Platform Detection Rules` tag — Manage Platform Detection Rules.
///
/// Covers Managed (Platform) Detection Rules, the Catalog Rules listing,
/// Template Detection Rules, the per-scope rule settings (smart/auto/core
/// default and emerging-threat toggles), enable/disable actions, and the
/// supporting metadata endpoints (data sources, severities, statuses, surfaces,
/// free-text filters).
pub struct PlatformDetectionRulesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query param structs
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/detection-library/platform-rules`
/// (Get Managed Detection Rules).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPlatformRulesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `"150"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `"10"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: `"YWdlbnRfaWQ6NTgwMjkzODE="`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If `true`, only the total number of items is returned, without any of
    /// the actual objects.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If `true`, total number of items is not calculated, which speeds up
    /// execution time.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Platform rule ids. Example: `"225494730938493804,225494730938493915"`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_rule_ids: Option<String>,
    /// List of entity ids to exclude from select_all.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_ids: Option<String>,
    /// The Account, Site, or Group ID, depending on the scope. Null if the
    /// scope is Global. Example: `"225494730938493804"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// To filter by scope, enter one or more scopes, separated by commas.
    /// Allowed values: `group`, `global`, `site`, `account`. Example:
    /// `"group"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Severities. Allowed values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<String>,
    /// To filter by attack surfaces associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_surfaces: Option<String>,
    /// To filter by sources associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<String>,
    /// Statuses. Allowed values: `Activating`, `Active`, `Disabling`,
    /// `Disabled`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    /// To filter by mitre tactics associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitre_tactics: Option<String>,
    /// To filter by a substring of the query content.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "s1ql__contains", skip_serializing_if = "Option::is_none")]
    pub s1ql_contains: Option<String>,
    /// To filter by a substring of the rule description.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// To filter by a substring of the rule name.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name_substring: Option<String>,
    /// To filter by tags associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Categories. Allowed values: `Events`, `Correlation`, `UEBAFirstSeen`,
    /// `Scheduled`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
    /// To filter by `hideQuery` of the rule params. Allowed values: `Shown`,
    /// `Hidden`. Example: `"Shown"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_query: Option<String>,
    /// To filter by labels.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    /// The active response status for the rule.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_response: Option<bool>,
}

impl ListPlatformRulesQuery {
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
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// If `true`, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If `true`, total number of items is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
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
    /// The Account, Site, or Group ID, depending on the scope.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// Scope level. Allowed values: `group`, `global`, `site`, `account`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Severities. Allowed values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    pub fn severities<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.severities = Some(join_csv(v));
        self
    }
    /// To filter by attack surfaces associated with the rule.
    pub fn attack_surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.attack_surfaces = Some(join_csv(v));
        self
    }
    /// To filter by sources associated with the rule.
    pub fn sources<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sources = Some(join_csv(v));
        self
    }
    /// Statuses. Allowed values: `Activating`, `Active`, `Disabling`,
    /// `Disabled`.
    pub fn statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.statuses = Some(join_csv(v));
        self
    }
    /// To filter by mitre tactics associated with the rule.
    pub fn mitre_tactics<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mitre_tactics = Some(join_csv(v));
        self
    }
    /// To filter by a substring of the query content.
    pub fn s1ql_contains(mut self, v: impl Into<String>) -> Self {
        self.s1ql_contains = Some(v.into());
        self
    }
    /// To filter by a substring of the rule description.
    pub fn description_contains(mut self, v: impl Into<String>) -> Self {
        self.description_contains = Some(v.into());
        self
    }
    /// To filter by a substring of the rule name.
    pub fn rule_name_substring(mut self, v: impl Into<String>) -> Self {
        self.rule_name_substring = Some(v.into());
        self
    }
    /// To filter by tags associated with the rule.
    pub fn tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags = Some(join_csv(v));
        self
    }
    /// Categories. Allowed values: `Events`, `Correlation`, `UEBAFirstSeen`,
    /// `Scheduled`.
    pub fn categories<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.categories = Some(join_csv(v));
        self
    }
    /// To filter by `hideQuery`. Allowed values: `Shown`, `Hidden`.
    pub fn hide_query(mut self, v: impl Into<String>) -> Self {
        self.hide_query = Some(v.into());
        self
    }
    /// To filter by labels.
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
}

/// Query params for `GET /web/api/v2.1/detection-library/rules`
/// (Get Managed Detection Rules — Catalog Rules).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListRulesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `"150"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `"10"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Example:
    /// `"YWdlbnRfaWQ6NTgwMjkzODE="`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If `true`, only the total number of items is returned.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If `true`, total number of items is not calculated.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `name`, `status`,
    /// `severity`, `description`, `category`, `generatedAlerts`,
    /// `raisedIssues`, `lastAlertTime`. Example: `"id"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Example: `"asc"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Account scope level id. Example: `"225494730938493804"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Site scope level id. Example: `"225494730938493804"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Platform rule ids. Example: `"225494730938493804,225494730938493915"`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_rule_ids: Option<String>,
    /// Custom rule ids. Example: `"225494730938493804,225494730938493915"`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_rule_ids: Option<String>,
    /// Severities. Allowed values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<String>,
    /// To filter by attack surfaces associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_surfaces: Option<String>,
    /// To filter by sources associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<String>,
    /// Statuses. Allowed values: `Activating`, `Active`, `Disabling`,
    /// `Disabled`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    /// To filter by mitre tactics associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitre_tactics: Option<String>,
    /// To filter by a substring of the rule description.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// To filter by a substring of the query content.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "s1ql__contains", skip_serializing_if = "Option::is_none")]
    pub s1ql_contains: Option<String>,
    /// To filter by a substring of the rule name.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// To filter by `hideQuery` of the rule params. Allowed values: `Shown`,
    /// `Hidden`. Example: `"Shown"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_query: Option<String>,
    /// Categories. Allowed values: `Events`, `Correlation`, `UEBAFirstSeen`,
    /// `Scheduled`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
    /// To filter by tags.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// To filter by labels.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    /// To filter by created-at greater than or equal to a datetime. Example:
    /// `"2018-02-27T04:49:26.257525Z"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// To filter by created-at less than or equal to a datetime. Example:
    /// `"2018-02-27T04:49:26.257525Z"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// To filter by updated-at greater than or equal to a datetime. Example:
    /// `"2018-02-27T04:49:26.257525Z"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// To filter by updated-at less than or equal to a datetime. Example:
    /// `"2018-02-27T04:49:26.257525Z"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// The active response status for the rule.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_response: Option<bool>,
    /// Free-text filter by all fields (name, description, query content). You
    /// can enter multiple values, separated by commas. Example:
    /// `"Service Pack 1"`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// If `true`, use Labels naming instead of Tags (e.g. "Labels" instead of
    /// "Tags", "core" instead of "autoDefault").
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_labels: Option<bool>,
}

impl ListRulesQuery {
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
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// If `true`, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If `true`, total number of items is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `name`, `status`,
    /// `severity`, `description`, `category`, `generatedAlerts`,
    /// `raisedIssues`, `lastAlertTime`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Account scope level id.
    pub fn account_ids(mut self, v: impl Into<String>) -> Self {
        self.account_ids = Some(v.into());
        self
    }
    /// Site scope level id.
    pub fn site_ids(mut self, v: impl Into<String>) -> Self {
        self.site_ids = Some(v.into());
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
    /// Custom rule ids.
    pub fn custom_rule_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.custom_rule_ids = Some(join_csv(v));
        self
    }
    /// Severities. Allowed values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    pub fn severities<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.severities = Some(join_csv(v));
        self
    }
    /// To filter by attack surfaces associated with the rule.
    pub fn attack_surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.attack_surfaces = Some(join_csv(v));
        self
    }
    /// To filter by sources associated with the rule.
    pub fn sources<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sources = Some(join_csv(v));
        self
    }
    /// Statuses. Allowed values: `Activating`, `Active`, `Disabling`,
    /// `Disabled`.
    pub fn statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.statuses = Some(join_csv(v));
        self
    }
    /// To filter by mitre tactics associated with the rule.
    pub fn mitre_tactics<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mitre_tactics = Some(join_csv(v));
        self
    }
    /// To filter by a substring of the rule description.
    pub fn description_contains(mut self, v: impl Into<String>) -> Self {
        self.description_contains = Some(v.into());
        self
    }
    /// To filter by a substring of the query content.
    pub fn s1ql_contains(mut self, v: impl Into<String>) -> Self {
        self.s1ql_contains = Some(v.into());
        self
    }
    /// To filter by a substring of the rule name.
    pub fn name_contains(mut self, v: impl Into<String>) -> Self {
        self.name_contains = Some(v.into());
        self
    }
    /// To filter by `hideQuery`. Allowed values: `Shown`, `Hidden`.
    pub fn hide_query(mut self, v: impl Into<String>) -> Self {
        self.hide_query = Some(v.into());
        self
    }
    /// Categories. Allowed values: `Events`, `Correlation`, `UEBAFirstSeen`,
    /// `Scheduled`.
    pub fn categories<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.categories = Some(join_csv(v));
        self
    }
    /// To filter by tags.
    pub fn tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags = Some(join_csv(v));
        self
    }
    /// To filter by labels.
    pub fn labels<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.labels = Some(join_csv(v));
        self
    }
    /// To filter by created-at greater than or equal to a datetime.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// To filter by created-at less than or equal to a datetime.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// To filter by updated-at greater than or equal to a datetime.
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// To filter by updated-at less than or equal to a datetime.
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// The active response status for the rule.
    pub fn active_response(mut self, v: bool) -> Self {
        self.active_response = Some(v);
        self
    }
    /// Free-text filter by all fields (name, description, query content).
    pub fn query<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.query = Some(join_csv(v));
        self
    }
    /// If `true`, use Labels naming instead of Tags.
    pub fn use_labels(mut self, v: bool) -> Self {
        self.use_labels = Some(v);
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/detection-library/platform-rules/settings`
/// (Get settings for Managed Detection Rules).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSettingsQuery {
    /// The Account or Site ID, depending on the scope. Null if the scope is
    /// Global. Example: `"225494730938493804"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// Scope level. Allowed values: `group`, `global`, `site`, `account`.
    /// Example: `"group"`.
    ///
    /// Required=yes. Kept as `Option` for builder ergonomics, but must be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
}

impl GetSettingsQuery {
    /// Scope level (required). Allowed values: `group`, `global`, `site`,
    /// `account`.
    pub fn new(scope_level: impl Into<String>) -> Self {
        Self {
            scope_id: None,
            scope_level: Some(scope_level.into()),
        }
    }
    /// The Account or Site ID, depending on the scope.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// Scope level (required). Allowed values: `group`, `global`, `site`,
    /// `account`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
}

/// Query params for the Template Detection Rules listing
/// (`GET /web/api/v2.1/detection-library/template-rules`).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTemplateRulesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `"150"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `"10"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Example:
    /// `"YWdlbnRfaWQ6NTgwMjkzODE="`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If `true`, only the total number of items is returned.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If `true`, total number of items is not calculated.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Platform rule ids. Example: `"225494730938493804,225494730938493915"`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_rule_ids: Option<String>,
    /// List of entity ids to exclude from select_all.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_ids: Option<String>,
    /// The Account, Site, or Group ID, depending on the scope. Null if the
    /// scope is Global. Example: `"225494730938493804"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// To filter by scope, enter one or more scopes, separated by commas.
    /// Allowed values: `group`, `global`, `site`, `account`. Example:
    /// `"group"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Severities. Allowed values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<String>,
    /// To filter by attack surfaces associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_surfaces: Option<String>,
    /// To filter by sources associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<String>,
    /// Statuses. Allowed values: `Activating`, `Active`, `Disabling`,
    /// `Disabled`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    /// To filter by mitre tactics associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitre_tactics: Option<String>,
    /// To filter by a substring of the query content.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "s1ql__contains", skip_serializing_if = "Option::is_none")]
    pub s1ql_contains: Option<String>,
    /// To filter by a substring of the rule description.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// To filter by a substring of the rule name.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name_substring: Option<String>,
    /// To filter by tags associated with the rule.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Categories. Allowed values: `Events`, `Correlation`, `UEBAFirstSeen`,
    /// `Scheduled`.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<String>,
    /// To filter by `hideQuery` of the rule params. Allowed values: `Shown`,
    /// `Hidden`. Example: `"Shown"`.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_query: Option<String>,
    /// To filter by labels.
    ///
    /// Array param (comma-joined). Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    /// The active response status for the rule.
    ///
    /// Optional (Required=no) -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_response: Option<bool>,
}

impl ListTemplateRulesQuery {
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
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// If `true`, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If `true`, total number of items is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
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
    /// The Account, Site, or Group ID, depending on the scope.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// Scope level. Allowed values: `group`, `global`, `site`, `account`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Severities. Allowed values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    pub fn severities<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.severities = Some(join_csv(v));
        self
    }
    /// To filter by attack surfaces associated with the rule.
    pub fn attack_surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.attack_surfaces = Some(join_csv(v));
        self
    }
    /// To filter by sources associated with the rule.
    pub fn sources<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sources = Some(join_csv(v));
        self
    }
    /// Statuses. Allowed values: `Activating`, `Active`, `Disabling`,
    /// `Disabled`.
    pub fn statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.statuses = Some(join_csv(v));
        self
    }
    /// To filter by mitre tactics associated with the rule.
    pub fn mitre_tactics<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mitre_tactics = Some(join_csv(v));
        self
    }
    /// To filter by a substring of the query content.
    pub fn s1ql_contains(mut self, v: impl Into<String>) -> Self {
        self.s1ql_contains = Some(v.into());
        self
    }
    /// To filter by a substring of the rule description.
    pub fn description_contains(mut self, v: impl Into<String>) -> Self {
        self.description_contains = Some(v.into());
        self
    }
    /// To filter by a substring of the rule name.
    pub fn rule_name_substring(mut self, v: impl Into<String>) -> Self {
        self.rule_name_substring = Some(v.into());
        self
    }
    /// To filter by tags associated with the rule.
    pub fn tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags = Some(join_csv(v));
        self
    }
    /// Categories. Allowed values: `Events`, `Correlation`, `UEBAFirstSeen`,
    /// `Scheduled`.
    pub fn categories<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.categories = Some(join_csv(v));
        self
    }
    /// To filter by `hideQuery`. Allowed values: `Shown`, `Hidden`.
    pub fn hide_query(mut self, v: impl Into<String>) -> Self {
        self.hide_query = Some(v.into());
        self
    }
    /// To filter by labels.
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
}

// ---------------------------------------------------------------------------
// Body structs
// ---------------------------------------------------------------------------

/// Request body for `PUT /web/api/v2.1/detection-library/platform-rules/enable`
/// (Enable a Managed Detection Rule) and
/// `PUT /web/api/v2.1/detection-library/platform-rules/disable`
/// (Disable a Managed Detection Rule).
///
/// Spec definition: `v2_1.gdl.schemas_PlatformRuleSchemaWithValidation`. The
/// schema declares no `required` fields, so every field is optional. This is a
/// rule-selection/filter payload (it selects which rules to enable/disable).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformRuleSelectionBody {
    /// Platform rule ids. Example: `["225494730938493804"]`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_rule_ids: Option<Vec<String>>,
    /// List of entity ids to exclude from select_all.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_ids: Option<Vec<String>>,
    /// The Account, Site, or Group ID, depending on the scope. Null if the
    /// scope is Global.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// To filter by scope. Allowed values: `group`, `global`, `site`,
    /// `account`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Severities. Allowed values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<Vec<String>>,
    /// To filter by attack surfaces associated with the rule.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_surfaces: Option<Vec<String>>,
    /// To filter by sources associated with the rule.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<String>>,
    /// Statuses. Allowed values: `Activating`, `Active`, `Disabling`,
    /// `Disabled`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
    /// To filter by mitre tactics associated with the rule.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitre_tactics: Option<Vec<String>>,
    /// To filter by a substring of the query content.
    ///
    /// Optional -> `Option`.
    #[serde(rename = "s1ql__contains", skip_serializing_if = "Option::is_none")]
    pub s1ql_contains: Option<String>,
    /// To filter by a substring of the rule description.
    ///
    /// Optional -> `Option`.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// To filter by a substring of the rule name.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name_substring: Option<String>,
    /// To filter by tags associated with the rule.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Categories. Allowed values: `Events`, `Correlation`, `UEBAFirstSeen`,
    /// `Scheduled`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<String>>,
    /// To filter by `hideQuery` of the rule params. Allowed values: `Shown`,
    /// `Hidden`.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_query: Option<String>,
    /// To filter by labels.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// The active response status for the rule.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_response: Option<bool>,
    /// To filter by a substring of the rule name.
    ///
    /// Optional -> `Option`.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by all fields (name, description, query content). You
    /// can enter multiple values, separated by commas.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Vec<String>>,
    /// `true` if the network quarantine is on.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_quarantine: Option<bool>,
    /// Defines the Treat-as-a-threat auto response. Allowed values:
    /// `UNDEFINED`, `Suspicious`, `Malicious`.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_as_threat: Option<String>,
}

impl PlatformRuleSelectionBody {
    /// Platform rule ids.
    pub fn platform_rule_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.platform_rule_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// List of entity ids to exclude from select_all.
    pub fn exclude_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.exclude_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// The Account, Site, or Group ID, depending on the scope.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// Scope level. Allowed values: `group`, `global`, `site`, `account`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Severities. Allowed values: `Info`, `Low`, `Medium`, `High`, `Critical`.
    pub fn severities<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.severities = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// To filter by attack surfaces associated with the rule.
    pub fn attack_surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.attack_surfaces = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// To filter by sources associated with the rule.
    pub fn sources<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.sources = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Statuses. Allowed values: `Activating`, `Active`, `Disabling`,
    /// `Disabled`.
    pub fn statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.statuses = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// To filter by mitre tactics associated with the rule.
    pub fn mitre_tactics<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.mitre_tactics = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// To filter by a substring of the query content.
    pub fn s1ql_contains(mut self, v: impl Into<String>) -> Self {
        self.s1ql_contains = Some(v.into());
        self
    }
    /// To filter by a substring of the rule description.
    pub fn description_contains(mut self, v: impl Into<String>) -> Self {
        self.description_contains = Some(v.into());
        self
    }
    /// To filter by a substring of the rule name.
    pub fn rule_name_substring(mut self, v: impl Into<String>) -> Self {
        self.rule_name_substring = Some(v.into());
        self
    }
    /// To filter by tags associated with the rule.
    pub fn tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Categories. Allowed values: `Events`, `Correlation`, `UEBAFirstSeen`,
    /// `Scheduled`.
    pub fn categories<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.categories = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// To filter by `hideQuery`. Allowed values: `Shown`, `Hidden`.
    pub fn hide_query(mut self, v: impl Into<String>) -> Self {
        self.hide_query = Some(v.into());
        self
    }
    /// To filter by labels.
    pub fn labels<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.labels = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// The active response status for the rule.
    pub fn active_response(mut self, v: bool) -> Self {
        self.active_response = Some(v);
        self
    }
    /// To filter by a substring of the rule name.
    pub fn name_contains(mut self, v: impl Into<String>) -> Self {
        self.name_contains = Some(v.into());
        self
    }
    /// Free-text filter by all fields (name, description, query content).
    pub fn query<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.query = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// `true` if the network quarantine is on.
    pub fn network_quarantine(mut self, v: bool) -> Self {
        self.network_quarantine = Some(v);
        self
    }
    /// Treat-as-threat auto response. Allowed values: `UNDEFINED`,
    /// `Suspicious`, `Malicious`.
    pub fn treat_as_threat(mut self, v: impl Into<String>) -> Self {
        self.treat_as_threat = Some(v.into());
        self
    }
}

/// Request body for
/// `PUT /web/api/v2.1/detection-library/platform-rules/settings`
/// (Update settings for Managed Detection Rules).
///
/// Spec definition: `v2_1.gdl.schemas_PlatformSettingsSchema`. Only
/// `scopeLevel` is `required`; the rest are optional/nullable.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsBody {
    /// The Account or Site ID, depending on the scope. Null if the scope is
    /// Global.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// Scope level. Allowed values: `group`, `global`, `site`, `account`.
    ///
    /// Required (`required`) + not `x-nullable` -> bare `String`.
    pub scope_level: String,
    /// Set to `true` to disable Platform Detection settings inheritance from
    /// the parent scope.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_inheritance: Option<bool>,
    /// Set to `On` to enable all smart default Platform Detection Rules. Allowed
    /// values: `Off`, `On`.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_default: Option<String>,
    /// Set to `On` to auto enable all future Emerging Threat Platform Detection
    /// Rules. Allowed values: `Off`, `On`.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emerging_threat: Option<String>,
    /// Set to `On` to enable all auto default Platform Detection Rules. Allowed
    /// values: `Off`, `On`.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_default: Option<String>,
    /// Set to `On` to enable all Core Platform Detection Rules. Allowed values:
    /// `Off`, `On`.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core: Option<String>,
}

impl UpdateSettingsBody {
    /// New body with the required `scopeLevel`. Allowed values: `group`,
    /// `global`, `site`, `account`.
    pub fn new(scope_level: impl Into<String>) -> Self {
        Self {
            scope_id: None,
            scope_level: scope_level.into(),
            disable_inheritance: None,
            smart_default: None,
            emerging_threat: None,
            auto_default: None,
            core: None,
        }
    }
    /// The Account or Site ID, depending on the scope.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// Set to `true` to disable Platform Detection settings inheritance.
    pub fn disable_inheritance(mut self, v: bool) -> Self {
        self.disable_inheritance = Some(v);
        self
    }
    /// Smart default toggle. Allowed values: `Off`, `On`.
    pub fn smart_default(mut self, v: impl Into<String>) -> Self {
        self.smart_default = Some(v.into());
        self
    }
    /// Emerging threat toggle. Allowed values: `Off`, `On`.
    pub fn emerging_threat(mut self, v: impl Into<String>) -> Self {
        self.emerging_threat = Some(v.into());
        self
    }
    /// Auto default toggle. Allowed values: `Off`, `On`.
    pub fn auto_default(mut self, v: impl Into<String>) -> Self {
        self.auto_default = Some(v.into());
        self
    }
    /// Core toggle. Allowed values: `Off`, `On`.
    pub fn core(mut self, v: impl Into<String>) -> Self {
        self.core = Some(v.into());
        self
    }
}

/// Join an iterator of string-like values into a comma-separated string.
fn join_csv<I, S>(values: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    values
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl PlatformDetectionRulesService<'_> {
    /// `GET /web/api/v2.1/detection-library/data-sources` — Get Data Sources.
    ///
    /// Get Data Sources valid for Managed Detection Rules.
    ///
    /// The spec defines no `200` response body for this endpoint, so the
    /// response data is returned as freeform `serde_json::Value`.
    pub async fn get_data_sources(&self) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/detection-library/data-sources", None)
            .await?)
    }

    /// `GET /web/api/v2.1/detection-library/platform-rules` — Get Managed
    /// Detection Rules.
    ///
    /// Return Managed Detection Rules for the given scope.
    pub async fn list_platform_rules(
        &self,
        query: &ListPlatformRulesQuery,
    ) -> Result<Paginated<PlatformRule>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/detection-library/platform-rules", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/detection-library/platform-rules/disable` — Disable a
    /// Managed Detection Rule.
    pub async fn disable_platform_rule(
        &self,
        body: &PlatformRuleSelectionBody,
    ) -> Result<Response<AffectedResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<PlatformRuleSelectionBody, Response<AffectedResult>>(
                Method::PUT,
                "/web/api/v2.1/detection-library/platform-rules/disable",
                None,
                Some(body),
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/detection-library/platform-rules/enable` — Enable a
    /// Managed Detection Rule.
    ///
    /// Enable a Managed Detection Rule creates a new rule and activates it.
    pub async fn enable_platform_rule(
        &self,
        body: &PlatformRuleSelectionBody,
    ) -> Result<Response<AffectedResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<PlatformRuleSelectionBody, Response<AffectedResult>>(
                Method::PUT,
                "/web/api/v2.1/detection-library/platform-rules/enable",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/detection-library/platform-rules/settings` — Get
    /// settings for Managed Detection Rules.
    ///
    /// Get settings for Managed Detection Rules for the given scope.
    pub async fn get_settings(
        &self,
        query: &GetSettingsQuery,
    ) -> Result<Response<PlatformSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/detection-library/platform-rules/settings", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/detection-library/platform-rules/settings` — Update
    /// settings for Managed Detection Rules.
    ///
    /// Update settings for Managed Detection Rules.
    ///
    /// The spec's `200` response declares an empty `data` object, so the
    /// response data is returned as freeform `serde_json::Value`.
    pub async fn update_settings(
        &self,
        body: &UpdateSettingsBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<UpdateSettingsBody, Response<serde_json::Value>>(
                Method::PUT,
                "/web/api/v2.1/detection-library/platform-rules/settings",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/detection-library/rules` — Get Managed Detection
    /// Rules.
    ///
    /// Return Catalog Rules for the given scope.
    pub async fn list_rules(
        &self,
        query: &ListRulesQuery,
    ) -> Result<Paginated<PlatformRule>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/detection-library/rules", q)
            .await?)
    }

    /// `GET /web/api/v2.1/detection-library/rules/free-text-filters` —
    /// Free-Text Filters.
    ///
    /// Get a metadata list of the available free-text filters.
    pub async fn list_free_text_filters(&self) -> Result<Paginated<FreeTextFilter>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/detection-library/rules/free-text-filters", None)
            .await?)
    }

    /// `GET /web/api/v2.1/detection-library/severities` — Get Severities.
    ///
    /// Get Severities valid for Managed Detection Rules.
    ///
    /// The spec defines no `200` response body for this endpoint, so the
    /// response data is returned as freeform `serde_json::Value`.
    pub async fn get_severities(&self) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/detection-library/severities", None)
            .await?)
    }

    /// `GET /web/api/v2.1/detection-library/statuses` — Get Statuses.
    ///
    /// Get all rule statuses valid for Managed Detection Rules.
    ///
    /// The spec defines no `200` response body for this endpoint, so the
    /// response data is returned as freeform `serde_json::Value`.
    pub async fn get_statuses(&self) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/detection-library/statuses", None)
            .await?)
    }

    /// `GET /web/api/v2.1/detection-library/surfaces` — Get Surfaces.
    ///
    /// Get all Surfaces valid for Managed Detection Rules.
    ///
    /// The spec defines no `200` response body for this endpoint, so the
    /// response data is returned as freeform `serde_json::Value`.
    pub async fn get_surfaces(&self) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/detection-library/surfaces", None)
            .await?)
    }

    /// `GET /web/api/v2.1/detection-library/template-rules` — Get Template
    /// Detection Rules.
    ///
    /// Return Template Detection Rules for the given scope.
    pub async fn list_template_rules(
        &self,
        query: &ListTemplateRulesQuery,
    ) -> Result<Paginated<TemplateRule>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/detection-library/template-rules", q)
            .await?)
    }
}
