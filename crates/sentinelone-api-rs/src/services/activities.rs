use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::activities::{Activity, ActivityAsMessage, ActivityType};
use crate::pagination::{Paginated, Response};

/// `Activities` tag — get system activities.
pub struct ActivitiesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/activities` (Get Activities).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitiesListQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: `"YWdlbnRfaWQ6NTgwMjkzODE="`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items will be returned, without any of
    /// the actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, the total number of items will not be calculated, which speeds
    /// up execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: `"id"`. Optional.
    ///
    /// Allowed values: `id`, `activityType`, `createdAt`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: `"asc"`. Optional.
    ///
    /// Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Get activities created before this timestamp.
    /// Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at__lt: Option<String>,
    /// Get activities created after this timestamp.
    /// Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at__gt: Option<String>,
    /// Get activities created before or at this timestamp.
    /// Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at__lte: Option<String>,
    /// Get activities created after or at this timestamp.
    /// Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at__gte: Option<String>,
    /// Get activities created in this range (inclusive) of a start timestamp and
    /// an end timestamp. Example: `"1514978764288-1514978999999"`. Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at__between: Option<String>,
    /// Return only these activity codes (comma-separated list). Optional.
    ///
    /// Example values: `6, 7, 8, 17, 43, 160, 161, 162`. See the `id` field from
    /// the Get activity types command for the full list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_types: Option<String>,
    /// Include internal activities hidden from display. Example: `false`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_hidden: Option<bool>,
    /// Return activities related to specified threats (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_ids: Option<String>,
    /// Return activities related to specified agents (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<String>,
    /// Filter activities by specific activity IDs (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Return activities by specific activity UUIDs (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_uuids: Option<String>,
    /// The user who invoked the activity, by user ID (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<String>,
    /// Email of the user who invoked the activity (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_emails: Option<String>,
    /// Return activities related to specified rules (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<String>,
    /// Return activities related to specified alerts (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_ids: Option<String>,
    /// Return activities related to specified entities (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_ids: Option<String>,
}

impl ActivitiesListQuery {
    /// Skip first number of items (0-1000). Example: `150`.
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Limit number of returned items (1-1000). Example: `10`.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// If true, only the total number of items will be returned.
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }
    /// If true, the total number of items will not be calculated.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }
    /// The column to sort the results by. Allowed: `id`, `activityType`, `createdAt`.
    pub fn sort_by(mut self, s: impl Into<String>) -> Self {
        self.sort_by = Some(s.into());
        self
    }
    /// Sort direction. Allowed: `asc`, `desc`.
    pub fn sort_order(mut self, s: impl Into<String>) -> Self {
        self.sort_order = Some(s.into());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Get activities created before this timestamp.
    pub fn created_at_lt(mut self, ts: impl Into<String>) -> Self {
        self.created_at__lt = Some(ts.into());
        self
    }
    /// Get activities created after this timestamp.
    pub fn created_at_gt(mut self, ts: impl Into<String>) -> Self {
        self.created_at__gt = Some(ts.into());
        self
    }
    /// Get activities created before or at this timestamp.
    pub fn created_at_lte(mut self, ts: impl Into<String>) -> Self {
        self.created_at__lte = Some(ts.into());
        self
    }
    /// Get activities created after or at this timestamp.
    pub fn created_at_gte(mut self, ts: impl Into<String>) -> Self {
        self.created_at__gte = Some(ts.into());
        self
    }
    /// Get activities created in this range (start-end timestamps).
    pub fn created_at_between(mut self, range: impl Into<String>) -> Self {
        self.created_at__between = Some(range.into());
        self
    }
    /// Return only these activity codes.
    pub fn activity_types<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.activity_types = Some(join_csv(types));
        self
    }
    /// Include internal activities hidden from display.
    pub fn include_hidden(mut self, b: bool) -> Self {
        self.include_hidden = Some(b);
        self
    }
    /// Return activities related to specified threats.
    pub fn threat_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_ids = Some(join_csv(ids));
        self
    }
    /// Return activities related to specified agents.
    pub fn agent_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ids = Some(join_csv(ids));
        self
    }
    /// Filter activities by specific activity IDs.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// Return activities by specific activity UUIDs.
    pub fn activity_uuids<I, S>(mut self, uuids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.activity_uuids = Some(join_csv(uuids));
        self
    }
    /// The user who invoked the activity, by user ID.
    pub fn user_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_ids = Some(join_csv(ids));
        self
    }
    /// Email of the user who invoked the activity.
    pub fn user_emails<I, S>(mut self, emails: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_emails = Some(join_csv(emails));
        self
    }
    /// Return activities related to specified rules.
    pub fn rule_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.rule_ids = Some(join_csv(ids));
        self
    }
    /// Return activities related to specified alerts.
    pub fn alert_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_ids = Some(join_csv(ids));
        self
    }
    /// Return activities related to specified entities.
    pub fn related_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.related_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/export/activities` (Export Activities).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitiesExportQuery {
    /// List of Site IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Get activities created before this timestamp.
    /// Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at__lt: Option<String>,
    /// Get activities created after this timestamp.
    /// Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at__gt: Option<String>,
    /// Get activities created before or at this timestamp.
    /// Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at__lte: Option<String>,
    /// Get activities created after or at this timestamp.
    /// Example: `"2018-02-27T04:49:26.257525Z"`. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at__gte: Option<String>,
    /// Get activities created in this range (inclusive) of a start timestamp and
    /// an end timestamp. Example: `"1514978764288-1514978999999"`. Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at__between: Option<String>,
    /// Return only these activity codes (comma-separated list). Optional.
    ///
    /// Example values: `6, 7, 8, 17, 43, 160, 161, 162`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_types: Option<String>,
    /// Include internal activities hidden from display. Example: `false`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_hidden: Option<bool>,
    /// Return activities related to specified threats (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_ids: Option<String>,
    /// Return activities related to specified agents (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<String>,
    /// Filter activities by specific activity IDs (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Return activities by specific activity UUIDs (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_uuids: Option<String>,
    /// The user who invoked the activity, by user ID (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<String>,
    /// Email of the user who invoked the activity (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_emails: Option<String>,
    /// Return activities related to specified rules (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<String>,
    /// Return activities related to specified alerts (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_ids: Option<String>,
    /// Return activities related to specified entities (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_ids: Option<String>,
    /// Limit number of returned items (1-10000). Example: `100`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows_limit: Option<i64>,
}

impl ActivitiesExportQuery {
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Get activities created before this timestamp.
    pub fn created_at_lt(mut self, ts: impl Into<String>) -> Self {
        self.created_at__lt = Some(ts.into());
        self
    }
    /// Get activities created after this timestamp.
    pub fn created_at_gt(mut self, ts: impl Into<String>) -> Self {
        self.created_at__gt = Some(ts.into());
        self
    }
    /// Get activities created before or at this timestamp.
    pub fn created_at_lte(mut self, ts: impl Into<String>) -> Self {
        self.created_at__lte = Some(ts.into());
        self
    }
    /// Get activities created after or at this timestamp.
    pub fn created_at_gte(mut self, ts: impl Into<String>) -> Self {
        self.created_at__gte = Some(ts.into());
        self
    }
    /// Get activities created in this range (start-end timestamps).
    pub fn created_at_between(mut self, range: impl Into<String>) -> Self {
        self.created_at__between = Some(range.into());
        self
    }
    /// Return only these activity codes.
    pub fn activity_types<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.activity_types = Some(join_csv(types));
        self
    }
    /// Include internal activities hidden from display.
    pub fn include_hidden(mut self, b: bool) -> Self {
        self.include_hidden = Some(b);
        self
    }
    /// Return activities related to specified threats.
    pub fn threat_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_ids = Some(join_csv(ids));
        self
    }
    /// Return activities related to specified agents.
    pub fn agent_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ids = Some(join_csv(ids));
        self
    }
    /// Filter activities by specific activity IDs.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// Return activities by specific activity UUIDs.
    pub fn activity_uuids<I, S>(mut self, uuids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.activity_uuids = Some(join_csv(uuids));
        self
    }
    /// The user who invoked the activity, by user ID.
    pub fn user_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_ids = Some(join_csv(ids));
        self
    }
    /// Email of the user who invoked the activity.
    pub fn user_emails<I, S>(mut self, emails: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_emails = Some(join_csv(emails));
        self
    }
    /// Return activities related to specified rules.
    pub fn rule_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.rule_ids = Some(join_csv(ids));
        self
    }
    /// Return activities related to specified alerts.
    pub fn alert_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_ids = Some(join_csv(ids));
        self
    }
    /// Return activities related to specified entities.
    pub fn related_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.related_ids = Some(join_csv(ids));
        self
    }
    /// Limit number of returned items (1-10000). Example: `100`.
    pub fn rows_limit(mut self, n: i64) -> Self {
        self.rows_limit = Some(n);
        self
    }
}

/// Query params for `GET /web/api/v2.1/last-activity-as-syslog`
/// (Last activity as Syslog message).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivitiesLastAsSyslogQuery {
    /// Skip first number of items (0-1000). Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, the total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed: `id`, `activityType`, `createdAt`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Get activities created before this timestamp. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at__lt: Option<String>,
    /// Get activities created after this timestamp. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at__gt: Option<String>,
    /// Get activities created before or at this timestamp. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at__lte: Option<String>,
    /// Get activities created after or at this timestamp. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at__gte: Option<String>,
    /// Get activities created in this range (start-end timestamps). Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at__between: Option<String>,
    /// Return only these activity codes (comma-separated list). Optional.
    ///
    /// Example values: `6, 7, 8, 17, 43, 160, 161, 162`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_types: Option<String>,
    /// Include internal activities hidden from display. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_hidden: Option<bool>,
    /// Return activities related to specified threats (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_ids: Option<String>,
    /// Return activities related to specified agents (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<String>,
    /// Filter activities by specific activity IDs (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Return activities by specific activity UUIDs (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_uuids: Option<String>,
    /// The user who invoked the activity, by user ID (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<String>,
    /// Email of the user who invoked the activity (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_emails: Option<String>,
    /// Return activities related to specified rules (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_ids: Option<String>,
    /// Return activities related to specified alerts (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_ids: Option<String>,
    /// Return activities related to specified entities (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_ids: Option<String>,
}

impl ActivitiesLastAsSyslogQuery {
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
    /// If true, only the total number of items will be returned.
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }
    /// If true, the total number of items will not be calculated.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }
    /// The column to sort the results by. Allowed: `id`, `activityType`, `createdAt`.
    pub fn sort_by(mut self, s: impl Into<String>) -> Self {
        self.sort_by = Some(s.into());
        self
    }
    /// Sort direction. Allowed: `asc`, `desc`.
    pub fn sort_order(mut self, s: impl Into<String>) -> Self {
        self.sort_order = Some(s.into());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Get activities created before this timestamp.
    pub fn created_at_lt(mut self, ts: impl Into<String>) -> Self {
        self.created_at__lt = Some(ts.into());
        self
    }
    /// Get activities created after this timestamp.
    pub fn created_at_gt(mut self, ts: impl Into<String>) -> Self {
        self.created_at__gt = Some(ts.into());
        self
    }
    /// Get activities created before or at this timestamp.
    pub fn created_at_lte(mut self, ts: impl Into<String>) -> Self {
        self.created_at__lte = Some(ts.into());
        self
    }
    /// Get activities created after or at this timestamp.
    pub fn created_at_gte(mut self, ts: impl Into<String>) -> Self {
        self.created_at__gte = Some(ts.into());
        self
    }
    /// Get activities created in this range (start-end timestamps).
    pub fn created_at_between(mut self, range: impl Into<String>) -> Self {
        self.created_at__between = Some(range.into());
        self
    }
    /// Return only these activity codes.
    pub fn activity_types<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.activity_types = Some(join_csv(types));
        self
    }
    /// Include internal activities hidden from display.
    pub fn include_hidden(mut self, b: bool) -> Self {
        self.include_hidden = Some(b);
        self
    }
    /// Return activities related to specified threats.
    pub fn threat_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_ids = Some(join_csv(ids));
        self
    }
    /// Return activities related to specified agents.
    pub fn agent_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ids = Some(join_csv(ids));
        self
    }
    /// Filter activities by specific activity IDs.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// Return activities by specific activity UUIDs.
    pub fn activity_uuids<I, S>(mut self, uuids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.activity_uuids = Some(join_csv(uuids));
        self
    }
    /// The user who invoked the activity, by user ID.
    pub fn user_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_ids = Some(join_csv(ids));
        self
    }
    /// Email of the user who invoked the activity.
    pub fn user_emails<I, S>(mut self, emails: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_emails = Some(join_csv(emails));
        self
    }
    /// Return activities related to specified rules.
    pub fn rule_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.rule_ids = Some(join_csv(ids));
        self
    }
    /// Return activities related to specified alerts.
    pub fn alert_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_ids = Some(join_csv(ids));
        self
    }
    /// Return activities related to specified entities.
    pub fn related_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.related_ids = Some(join_csv(ids));
        self
    }
}

/// Join an iterator of string-like items into a comma-separated string.
fn join_csv<I, S>(items: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    items
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

impl ActivitiesService<'_> {
    /// Get Activities.
    ///
    /// Get the activities, and their data, that match the filters. We recommend
    /// that you set some values for the filters. The full list will be too large
    /// to be useful.
    ///
    /// `GET /web/api/v2.1/activities`
    pub async fn list(
        &self,
        query: &ActivitiesListQuery,
    ) -> Result<Paginated<Activity>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/activities", q).await?)
    }

    /// Get Activity Types.
    ///
    /// Get a list of activity types. This is useful to see valid values to
    /// filter activities in other commands.
    ///
    /// `GET /web/api/v2.1/activities/types`
    pub async fn types(&self) -> Result<Paginated<ActivityType>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/activities/types", None)
            .await?)
    }

    /// Export Activities.
    ///
    /// Export the list of activities. The response is an export payload rather
    /// than a typed envelope, so it is returned as a freeform JSON value.
    ///
    /// `GET /web/api/v2.1/export/activities`
    pub async fn export(
        &self,
        query: &ActivitiesExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/export/activities", q)
            .await?)
    }

    /// Last activity as Syslog message.
    ///
    /// To see examples of Syslog messages, you can get the Syslog message that
    /// corresponds to the last activity that matches the filter. This is not
    /// intended for production purposes. If Syslog messages that you expected to
    /// see are not in the response, make sure you selected "Syslog" for the
    /// activity type in Console > Settings > Notifications. To see your Syslog
    /// settings, run: "settings/notifications". To change the settings, run:
    /// "settings/notifications" with the changes in the body of the request.
    ///
    /// `GET /web/api/v2.1/last-activity-as-syslog`
    pub async fn last_activity_as_syslog(
        &self,
        query: &ActivitiesLastAsSyslogQuery,
    ) -> Result<Response<ActivityAsMessage>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/last-activity-as-syslog", q)
            .await?)
    }
}
