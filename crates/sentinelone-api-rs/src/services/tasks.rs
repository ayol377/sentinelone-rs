use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::tasks::{
    FlexibleTaskConfiguration, TaskBooleanFlag, TaskConfiguration, TaskSubscope,
};
use crate::pagination::{Paginated, Response};

/// `Tasks` tag — task related operations.
///
/// Read and update the task configuration (concurrency limits and maintenance
/// windows) of a scope, inspect child-scope configurations, work with the
/// flexible maintenance-window policy format, and export maintenance windows as
/// CSV.
pub struct TasksService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Joins an iterator of strings into a comma-separated value, as SentinelOne
/// array query params expect.
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

/// Query params for `GET /web/api/v2.1/tasks-configuration` (Get Task
/// Configuration).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTaskConfigurationQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use "cursor". Example: "150". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: "id".
    ///
    /// Enum: `scopeId`, `taskType`, `createdAt`, `userId`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Enum: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Task type. Example: "agents_upgrade".
    ///
    /// Enum: `agents_upgrade`, `agent_version_change`, `auto_deploy`,
    /// `script_execution`, `cis_scan`, `gad`, `forensics_collection`. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

impl GetTaskConfigurationQuery {
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
    /// The column to sort the results by. Enum: `scopeId`, `taskType`,
    /// `createdAt`, `userId`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Enum: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
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
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Task type (required). Enum: `agents_upgrade`, `agent_version_change`,
    /// `auto_deploy`, `script_execution`, `cis_scan`, `gad`,
    /// `forensics_collection`.
    pub fn task_type(mut self, v: impl Into<String>) -> Self {
        self.task_type = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/tasks-configuration/explicit-subscopes`
/// (Get Child Scope Task Configuration).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetChildScopeTaskConfigurationQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use "cursor". Example: "150". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Example:
    /// "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: "id".
    ///
    /// Enum: `scopeId`, `taskType`, `createdAt`, `userId`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Enum: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Task type. Example: "agents_upgrade".
    ///
    /// Enum: `agents_upgrade`, `agent_version_change`, `auto_deploy`,
    /// `script_execution`, `cis_scan`, `gad`, `forensics_collection`. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// Query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl GetChildScopeTaskConfigurationQuery {
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
    /// The column to sort the results by. Enum: `scopeId`, `taskType`,
    /// `createdAt`, `userId`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Enum: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
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
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Task type (required). Enum: `agents_upgrade`, `agent_version_change`,
    /// `auto_deploy`, `script_execution`, `cis_scan`, `gad`,
    /// `forensics_collection`.
    pub fn task_type(mut self, v: impl Into<String>) -> Self {
        self.task_type = Some(v.into());
        self
    }
    /// Query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/tasks-configuration/flexible` (Get Task
/// Configuration (Flexible MW)).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFlexibleTaskConfigurationQuery {
    /// List of Account IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Task type. Example: "agents_upgrade".
    ///
    /// Enum: `agents_upgrade`, `agent_version_change`, `auto_deploy`,
    /// `script_execution`, `cis_scan`, `gad`, `forensics_collection`. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

impl GetFlexibleTaskConfigurationQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
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
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Task type (required). Enum: `agents_upgrade`, `agent_version_change`,
    /// `auto_deploy`, `script_execution`, `cis_scan`, `gad`,
    /// `forensics_collection`.
    pub fn task_type(mut self, v: impl Into<String>) -> Self {
        self.task_type = Some(v.into());
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/tasks-configuration/has-explicit-subscope` (Has Child
/// Scopes).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HasChildScopesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use "cursor". Example: "150". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Example:
    /// "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: "id".
    ///
    /// Enum: `scopeId`, `taskType`, `createdAt`, `userId`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Enum: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Task type. Example: "agents_upgrade".
    ///
    /// Enum: `agents_upgrade`, `agent_version_change`, `auto_deploy`,
    /// `script_execution`, `cis_scan`, `gad`, `forensics_collection`. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

impl HasChildScopesQuery {
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
    /// The column to sort the results by. Enum: `scopeId`, `taskType`,
    /// `createdAt`, `userId`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Enum: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
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
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Task type (required). Enum: `agents_upgrade`, `agent_version_change`,
    /// `auto_deploy`, `script_execution`, `cis_scan`, `gad`,
    /// `forensics_collection`.
    pub fn task_type(mut self, v: impl Into<String>) -> Self {
        self.task_type = Some(v.into());
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/tasks-configuration/maintenance-windows/export` (Export
/// Maintenance Windows as CSV).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportMaintenanceWindowsQuery {
    /// List of Account IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-separated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Task type. Example: "agents_upgrade".
    ///
    /// Enum: `agents_upgrade`, `agent_version_change`, `auto_deploy`,
    /// `script_execution`, `cis_scan`, `gad`, `forensics_collection`. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

impl ExportMaintenanceWindowsQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
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
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Task type (required). Enum: `agents_upgrade`, `agent_version_change`,
    /// `auto_deploy`, `script_execution`, `cis_scan`, `gad`,
    /// `forensics_collection`.
    pub fn task_type(mut self, v: impl Into<String>) -> Self {
        self.task_type = Some(v.into());
        self
    }
}

// ---- Body types ----

/// Body for `PUT /web/api/v2.1/tasks-configuration` (Create Task)
/// (`tasks.schemas_PutTaskSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskBody {
    /// Data. Required.
    pub data: CreateTaskBodyData,
    /// Filter. Required.
    pub filter: CreateTaskBodyFilter,
}

/// `data` object for [`CreateTaskBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskBodyData {
    /// Inherit parent's scope Max Concurrent configuration. Required.
    pub inherit_parent_concurrency_config: bool,
    /// Inherit parent's scope Maintenance windows configuration. Required.
    pub inherit_parent_maintenance_config: bool,
    /// Max concurrent. Required.
    pub max_concurrent: i64,
    /// Timezone gmt. Required.
    pub timezone_gmt: String,
    /// Stores the maintenance time for each day. Required.
    ///
    /// Freeform object keyed by day; values are arbitrary objects.
    pub maintenance_windows_by_day: serde_json::Value,
}

/// `filter` object for [`CreateTaskBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskBodyFilter {
    /// Task type. Required.
    ///
    /// Enum: `agents_upgrade`, `agent_version_change`, `auto_deploy`,
    /// `script_execution`, `cis_scan`, `gad`, `forensics_collection`.
    pub task_type: String,
    /// List of Account IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

/// Body for `PUT /web/api/v2.1/tasks-configuration/flexible` (Update Task
/// Configuration (Flexible MW)) (`tasks.schemas_PutFlexibleTaskSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFlexibleTaskBody {
    /// Data. Required.
    pub data: UpdateFlexibleTaskBodyData,
    /// Filter. Required.
    pub filter: UpdateFlexibleTaskBodyFilter,
}

/// `data` object for [`UpdateFlexibleTaskBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFlexibleTaskBodyData {
    /// Inherit parent's scope Max Concurrent configuration. Required.
    pub inherit_parent_concurrency_config: bool,
    /// Inherit parent's scope Maintenance windows configuration. Required.
    pub inherit_parent_maintenance_config: bool,
    /// Max concurrent tasks. Required.
    pub max_concurrent: i64,
    /// IANA timezone identifier (e.g., 'America/New_York'). Required.
    pub timezone: String,
    /// Policy frequency. Required.
    ///
    /// Enum: `daily`, `weekly`, `monthly`, `yearly`, `custom`.
    pub frequency: String,
    /// End configuration. Required.
    pub end: UpdateFlexibleTaskBodyEnd,
    /// Maintenance rules (minimum 1). Required.
    ///
    /// Each rule is a freeform object (see spec
    /// `tasks.schemas_PutFlexibleTaskSchema` for `byWeekday`, `byMonthDay`,
    /// `byMonth`, `bySetPos`, `frequency`, `interval`, `effectiveStartOn`,
    /// `effectiveEndOn`, `timeWindows`, `absolute`).
    pub rules: Vec<serde_json::Value>,
    /// Policy-level recurrence interval (minimum 1; default 1). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// Policy start date in ISO format (computed for custom if omitted).
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_on: Option<String>,
    /// Whether the maintenance window is active (default true). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// `end` object for [`UpdateFlexibleTaskBodyData`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFlexibleTaskBodyEnd {
    /// End mode. Required. Enum: `never`, `on_date`, `after_count`.
    pub mode: String,
    /// End date in ISO format (required when mode=on_date). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_date: Option<String>,
    /// Max occurrences (required when mode=after_count; minimum 1).
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_occurrences: Option<i64>,
}

/// `filter` object for [`UpdateFlexibleTaskBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFlexibleTaskBodyFilter {
    /// Task type. Required.
    ///
    /// Enum: `agents_upgrade`, `agent_version_change`, `auto_deploy`,
    /// `script_execution`, `cis_scan`, `gad`, `forensics_collection`.
    pub task_type: String,
    /// List of Account IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by (1-500). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl TasksService<'_> {
    /// `GET /web/api/v2.1/tasks-configuration` — Get Task Configuration.
    ///
    /// Get the task configuration of a scope.
    pub async fn get_task_configuration(
        &self,
        query: &GetTaskConfigurationQuery,
    ) -> Result<Response<TaskConfiguration>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/tasks-configuration", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/tasks-configuration` — Create Task.
    ///
    /// Create a task configuration.
    pub async fn create_task(
        &self,
        body: &CreateTaskBody,
    ) -> Result<Response<TaskConfiguration>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<CreateTaskBody, Response<TaskConfiguration>>(
                Method::PUT,
                "/web/api/v2.1/tasks-configuration",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/tasks-configuration/explicit-subscopes` — Get Child
    /// Scope Task Configuration.
    ///
    /// Get the task configuration of child scopes of the given scope, if the
    /// tasks are not inherited.
    pub async fn get_child_scope_task_configuration(
        &self,
        query: &GetChildScopeTaskConfigurationQuery,
    ) -> Result<Paginated<TaskSubscope>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/tasks-configuration/explicit-subscopes", q)
            .await?)
    }

    /// `GET /web/api/v2.1/tasks-configuration/flexible` — Get Task
    /// Configuration (Flexible MW).
    ///
    /// Get task configuration with flexible maintenance window format. Returns
    /// policy_payload when flexible MW is configured.
    pub async fn get_flexible_task_configuration(
        &self,
        query: &GetFlexibleTaskConfigurationQuery,
    ) -> Result<Response<FlexibleTaskConfiguration>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/tasks-configuration/flexible", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/tasks-configuration/flexible` — Update Task
    /// Configuration (Flexible MW).
    ///
    /// Update task configuration with flexible maintenance window format.
    /// Requires mw_aup_advanced_flexibility or mw_lsu_advanced_flexibility
    /// global switch.
    pub async fn update_flexible_task_configuration(
        &self,
        body: &UpdateFlexibleTaskBody,
    ) -> Result<Response<FlexibleTaskConfiguration>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<UpdateFlexibleTaskBody, Response<FlexibleTaskConfiguration>>(
                Method::PUT,
                "/web/api/v2.1/tasks-configuration/flexible",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/tasks-configuration/has-explicit-subscope` — Has
    /// Child Scopes.
    ///
    /// From a given scope, see if there are scopes under it that have local,
    /// explicit tasks. The response returns True if a sub-scope has a local
    /// (not inherited) task configuration.
    pub async fn has_child_scopes(
        &self,
        query: &HasChildScopesQuery,
    ) -> Result<Response<TaskBooleanFlag>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/tasks-configuration/has-explicit-subscope", q)
            .await?)
    }

    /// `GET /web/api/v2.1/tasks-configuration/maintenance-windows/export` —
    /// Export Maintenance Windows as CSV.
    ///
    /// Export all maintenance window occurrences for a specific scope as CSV.
    /// Only supports flexible (policy_payload) maintenance window format. Date
    /// range: policies with end dates generate up to that date; policies with no
    /// end date generate for 5 years from start date. Returns a CSV file with
    /// columns: Date, Start Time, End Time. Maximum report size: 5MB.
    ///
    /// Note: the endpoint returns a CSV file. This SDK decodes the HTTP body as
    /// JSON (the transport always parses JSON), so the result is exposed as a
    /// freeform [`serde_json::Value`].
    pub async fn export_maintenance_windows(
        &self,
        query: &ExportMaintenanceWindowsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/tasks-configuration/maintenance-windows/export",
                q,
            )
            .await?)
    }
}
