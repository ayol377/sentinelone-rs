//! `RemoteOps Scripts` tag — RemoteOps scripts operations.
//!
//! Manage the SentinelOne Script Library (Remote Script Orchestration): list,
//! upload, edit and delete scripts; run remote scripts; fetch results; manage
//! pending executions and guardrails configuration; and query remote-script
//! task status.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::remoteops_scripts::*;
use crate::pagination::{Paginated, Response};

/// Join an iterator of stringy values with commas, as the API expects for
/// array query params.
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

/// `RemoteOps Scripts` tag — RemoteOps scripts operations.
///
/// Provides access to the SentinelOne Script Library used for the Remote Script
/// Orchestration feature: script CRUD, remote execution, result download,
/// pending-execution approval, guardrails, and task status.
pub struct RemoteopsScriptsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query structs
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/remote-scripts` — Get Scripts.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScriptsQuery {
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// List of Account IDs to filter by. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of group IDs to filter by. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// List of Site IDs to filter by. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Is the script runnable in Advanced Response Scripts. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_available_for_ars: Option<bool>,
    /// Free-text query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// List of the script types. Optional. Comma-joined. Enum values:
    /// `artifactCollection`, `dataCollection`, `action`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_type: Option<String>,
    /// A list of script IDs. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// The column to sort the results by. Optional. Enum values: `id`,
    /// `createdAt`, `mgmtId`, `scopeId`, `scriptName`, `name`, `osTypes`,
    /// `createdByUserId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Sort direction. Optional. Enum values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of the script OS types. Optional. Comma-joined. Enum values:
    /// `linux`, `macos`, `windows`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
}

impl GetScriptsQuery {
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
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
    /// List of group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
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
    /// Is the script runnable in Advanced Response Scripts.
    pub fn is_available_for_ars(mut self, v: bool) -> Self {
        self.is_available_for_ars = Some(v);
        self
    }
    /// Free-text query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// List of the script types (`artifactCollection`, `dataCollection`,
    /// `action`).
    pub fn script_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.script_type = Some(join_csv(v));
        self
    }
    /// A list of script IDs.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Sort direction (`asc`, `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of the script OS types (`linux`, `macos`, `windows`).
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/remote-scripts/guardrails/configuration`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetGuardrailsConfigurationQuery {
    /// Scope ID. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// Scope level. Required. Enum values: `account`, `site`, `group`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
}

impl GetGuardrailsConfigurationQuery {
    /// Scope ID. Required.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// Scope level (`account`, `site`, `group`). Required.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/remote-scripts/pending-executions`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPendingExecutionsQuery {
    /// If true, total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// List of Account IDs to filter by. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of group IDs to filter by. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Skip first number of items (0-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// List of Site IDs to filter by. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The column to sort the results by. Optional. Enum values: `id`,
    /// `createdAt`, `state`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Sort direction. Optional. Enum values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
}

impl GetPendingExecutionsQuery {
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
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
    /// List of group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
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
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Sort direction (`asc`, `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
}

/// Query params for `GET /web/api/v2.1/remote-scripts/script-content`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScriptContentQuery {
    /// Script ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_id: Option<String>,
}

impl GetScriptContentQuery {
    /// Script ID.
    pub fn script_id(mut self, v: impl Into<String>) -> Self {
        self.script_id = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/remote-scripts/status` — Get Remote
/// Scripts Tasks Status.
///
/// Note: `parent_task_id` or `parent_task_id__in` is mandatory at the API
/// level, although both are declared optional in the spec.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatusQuery {
    /// Skip first number of items (0-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Optional. Enum values: `id`,
    /// `initiatedBy`, `createdAt`, `updatedAt`, `status`, `detailedStatus`,
    /// `agentComputerName`, `parentTaskId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Enum values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of IDs to filter by. Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Created at lesser than (date-time). Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Created at greater than (date-time). Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Created at lesser or equal than (date-time). Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Created at greater or equal than (date-time). Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Updated at lesser than (date-time). Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at_lt: Option<String>,
    /// Updated at greater than (date-time). Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at_gt: Option<String>,
    /// Updated at lesser or equal than (date-time). Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// Updated at greater or equal than (date-time). Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// Free-text query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Status filter. Optional. Comma-joined. Enum values: `created`,
    /// `scheduled`, `pending`, `pending_user_action`, `in_progress`, `failed`,
    /// `completed`, `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Type filter (multiple). Optional. Comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// Single type filter. Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
    /// Free-text filter by agent computer name (supports multiple values).
    /// Optional. Comma-joined.
    #[serde(rename = "computerName__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name_contains: Option<String>,
    /// Free-text filter by agent UUID (supports multiple values). Optional.
    /// Comma-joined.
    #[serde(rename = "uuid__contains", skip_serializing_if = "Option::is_none")]
    pub uuid_contains: Option<String>,
    /// Only include tasks from specific initiating user (supports multiple
    /// values). Optional. Comma-joined.
    #[serde(rename = "initiatedBy__contains", skip_serializing_if = "Option::is_none")]
    pub initiated_by_contains: Option<String>,
    /// Only include tasks with specific detailed status (supports multiple
    /// values). Optional. Comma-joined.
    #[serde(rename = "detailedStatus__contains", skip_serializing_if = "Option::is_none")]
    pub detailed_status_contains: Option<String>,
    /// Only include tasks with specific description (supports multiple values).
    /// Optional. Comma-joined.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// List of parent task IDs to filter by. Optional. Comma-joined.
    #[serde(rename = "parentTaskId__in", skip_serializing_if = "Option::is_none")]
    pub parent_task_id_in: Option<String>,
    /// Parent task id to fetch the status by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<String>,
}

impl GetStatusQuery {
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
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
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction (`asc`, `desc`).
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
    /// List of IDs to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// Created at lesser than (date-time).
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Created at greater than (date-time).
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Created at lesser or equal than (date-time).
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Created at greater or equal than (date-time).
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Updated at lesser than (date-time).
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lt = Some(v.into());
        self
    }
    /// Updated at greater than (date-time).
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gt = Some(v.into());
        self
    }
    /// Updated at lesser or equal than (date-time).
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// Updated at greater or equal than (date-time).
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// Free-text query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Status filter.
    pub fn status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.status = Some(join_csv(v));
        self
    }
    /// Type filter (multiple).
    pub fn types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.types = Some(join_csv(v));
        self
    }
    /// Single type filter.
    pub fn task_type(mut self, v: impl Into<String>) -> Self {
        self.task_type = Some(v.into());
        self
    }
    /// Free-text filter by agent computer name.
    pub fn computer_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by agent UUID.
    pub fn uuid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuid_contains = Some(join_csv(v));
        self
    }
    /// Only include tasks from specific initiating user.
    pub fn initiated_by_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.initiated_by_contains = Some(join_csv(v));
        self
    }
    /// Only include tasks with specific detailed status.
    pub fn detailed_status_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detailed_status_contains = Some(join_csv(v));
        self
    }
    /// Only include tasks with specific description.
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join_csv(v));
        self
    }
    /// List of parent task IDs to filter by.
    pub fn parent_task_id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.parent_task_id_in = Some(join_csv(v));
        self
    }
    /// Parent task id to fetch the status by.
    pub fn parent_task_id(mut self, v: impl Into<String>) -> Self {
        self.parent_task_id = Some(v.into());
        self
    }
}

// ---------------------------------------------------------------------------
// Body structs
// ---------------------------------------------------------------------------

/// Body for `DELETE /web/api/v2.1/remote-scripts` — Delete Scripts.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteScriptsBody {
    /// Filter selecting the scripts to delete. Required.
    pub filter: DeleteScriptsFilter,
    /// Console data. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_data: Option<String>,
    /// Send activity. Optional (default true server-side).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_activity: Option<bool>,
}

/// Filter within [`DeleteScriptsBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteScriptsFilter {
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// Free-text query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// List of the script types. Optional. Enum values: `artifactCollection`,
    /// `dataCollection`, `action`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_type: Option<Vec<String>>,
    /// A list of script IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// List of the script OS types. Optional. Enum values: `linux`, `macos`,
    /// `windows`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<Vec<String>>,
}

/// Body for `POST /web/api/v2.1/remote-scripts` — Upload New Script.
///
/// Note: this endpoint is `multipart/form-data` (file upload). This typed body
/// covers the non-file form fields; binary `file` / `packageFile` parts are not
/// representable as JSON and must be handled separately if needed.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadScriptBody {
    /// Script name. Required.
    pub script_name: String,
    /// Script type. Required. Enum values: `artifactCollection`,
    /// `dataCollection`, `action`.
    pub script_type: String,
    /// Scope level. Required. Enum values: `site`, `account`, `global`.
    pub scope_level: String,
    /// Is input required. Required.
    pub input_required: bool,
    /// OS types. Required. Enum values: `linux`, `macos`, `windows`.
    pub os_types: Vec<String>,
    /// True if script content is encoded. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_script_content_encoded: Option<bool>,
    /// Package max size. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_max_size: Option<String>,
    /// Input example. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_example: Option<String>,
    /// True if script/package files should be taken from an existing script
    /// specified in `original_script_id`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_duplication: Option<bool>,
    /// Package expiration option on endpoint. Optional. Enum values: `None`,
    /// `Immediate`, `OnRestart`, `Time`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_endpoint_expiration: Option<String>,
    /// True if package file should not be copied, applicable only if
    /// `is_duplication` is true. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_removed: Option<bool>,
    /// Script runtime timeout in seconds. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_runtime_timeout_seconds: Option<i64>,
    /// Content of the script file, applicable only if `is_duplication` is true.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_content: Option<String>,
    /// Input instructions. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_instructions: Option<String>,
    /// Console data. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_data: Option<String>,
    /// ID of script from which the script/package files will be copied,
    /// applicable only if `is_duplication` is true. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_script_id: Option<String>,
    /// Send activity. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_activity: Option<bool>,
    /// Script description. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_description: Option<String>,
    /// Package expiration time on endpoint. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_endpoint_expiration_seconds: Option<i64>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

/// Body for `PUT /web/api/v2.1/remote-scripts/edit/{script_id}` — Update a
/// Script (content-aware edit).
///
/// Note: this endpoint is `multipart/form-data` (file upload). This typed body
/// covers the non-file form fields; binary `scriptFile` / `packageFile` parts
/// are not representable as JSON and must be handled separately if needed.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditScriptBody {
    /// Input instructions. Required.
    pub input_instructions: String,
    /// Input example. Required.
    pub input_example: String,
    /// Script name. Required.
    pub script_name: String,
    /// Is input required. Required.
    pub input_required: bool,
    /// Script type. Required. Enum values: `artifactCollection`,
    /// `dataCollection`, `action`.
    pub script_type: String,
    /// OS types. Required. Enum values: `linux`, `macos`, `windows`.
    pub os_types: Vec<String>,
    /// Script runtime timeout in seconds. Required.
    pub script_runtime_timeout_seconds: i64,
    /// New content of a script if the script content was changed on an already
    /// previously uploaded script. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_content: Option<String>,
    /// Is the script content base64 encoded? Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_script_content_encoded: Option<bool>,
    /// Script description. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_description: Option<String>,
    /// Was package removed during edit of the script? Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_removed: Option<bool>,
    /// Package max size. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_max_size: Option<String>,
    /// Package expiration time on endpoint. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_endpoint_expiration_seconds: Option<i64>,
    /// Console data. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_data: Option<String>,
    /// Package expiration option on endpoint. Optional. Enum values: `None`,
    /// `Immediate`, `OnRestart`, `Time`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_endpoint_expiration: Option<String>,
    /// Send activity. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_activity: Option<bool>,
}

/// Body for `POST /web/api/v2.1/remote-scripts/execute` — Run Remote Script.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteScriptBody {
    /// Applied agent filter. Required. The agent filter is deep / freeform; use
    /// a JSON object (e.g. `{ "ids": [...] }`, `{ "siteIds": [...] }`).
    pub filter: serde_json::Value,
    /// Execution data. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ExecuteScriptData>,
}

/// Execution parameters within [`ExecuteScriptBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteScriptData {
    /// Script id. Required.
    pub script_id: String,
    /// Task description. Required.
    pub task_description: String,
    /// Output destination. Required. Enum values: `SentinelCloud`, `Local`,
    /// `None`, `SingularityXDR`.
    pub output_destination: String,
    /// Input params. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_params: Option<String>,
    /// Password. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Used to specify execution where a generic password is used. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_from_scope: Option<ExecuteScriptPasswordScope>,
    /// Output file paths. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_file_paths: Option<Vec<String>>,
    /// Output directory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_directory: Option<String>,
    /// Script runtime timeout in seconds for current execution. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_runtime_timeout_seconds: Option<i64>,
    /// SingularityXDR URL. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singularityxdr_url: Option<String>,
    /// SingularityXDR keyword. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singularityxdr_keyword: Option<String>,
    /// Api key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// If set to true, execution will require approval. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_approval: Option<bool>,
    /// Id of destination profile to use. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_profile_id: Option<String>,
    /// Destination profile keyword. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_profile_keyword: Option<String>,
}

/// Generic-password scope reference within [`ExecuteScriptData`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteScriptPasswordScope {
    /// User scope. Required. Enum values: `tenant`, `account`, `site`.
    pub scope_level: String,
    /// String repr. of scope id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

/// Body for `POST /web/api/v2.1/remote-scripts/fetch-files` — Get Script
/// Results.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchScriptsResultsBody {
    /// Data selector. Required.
    pub data: FetchScriptsResultsData,
}

/// Selector within [`FetchScriptsResultsBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchScriptsResultsData {
    /// A list of task ids to get a download link for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_ids: Option<Vec<String>>,
    /// A list of partial or whole computer names, which ran scripts, to get a
    /// download link for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_names: Option<Vec<String>>,
}

/// Body for `POST /web/api/v2.1/remote-scripts/guardrails/check` — Check
/// whether guardrail applies to an execution.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuardrailCheckBody {
    /// Data payload. Required.
    pub data: GuardrailCheckData,
}

/// Payload within [`GuardrailCheckBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuardrailCheckData {
    /// Script id. Required.
    pub script_id: String,
    /// Agent ids. Required.
    pub agent_ids: Vec<String>,
}

/// Body for `DELETE /web/api/v2.1/remote-scripts/guardrails/configuration` —
/// Deletes a specific guardrails configuration.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteGuardrailsBody {
    /// Data payload. Required.
    pub data: DeleteGuardrailsData,
}

/// Payload within [`DeleteGuardrailsBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteGuardrailsData {
    /// Scope ID. Required.
    pub scope_id: String,
    /// Scope level. Required. Enum values: `account`, `site`, `group`.
    pub scope_level: String,
}

/// Body for `POST /web/api/v2.1/remote-scripts/guardrails/configuration` —
/// Updates or inserts a guardrails configuration.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostGuardrailsBody {
    /// Data payload. Required.
    pub data: PostGuardrailsData,
}

/// Payload within [`PostGuardrailsBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostGuardrailsData {
    /// Whether guardrail is active. Required.
    pub enabled: bool,
    /// Threshold for number of endpoints. Required.
    pub endpoints_quantity: i64,
    /// Scope ID. Required.
    pub scope_id: String,
    /// Scope level. Required. Enum values: `account`, `site`, `group`.
    pub scope_level: String,
    /// List of script types that the guardrail relates to. Required.
    pub script_types: Vec<String>,
}

/// Body for
/// `PUT /web/api/v2.1/remote-scripts/pending-executions/{pending_execution_id}`
/// — Approve/decline pending execution.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveDeclinePendingExecutionBody {
    /// Data payload. Required.
    pub data: ApproveDeclinePendingExecutionData,
}

/// Payload within [`ApproveDeclinePendingExecutionBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveDeclinePendingExecutionData {
    /// Action. Required. Enum values: `approve`, `decline`.
    pub action: String,
}

/// Body for `PUT /web/api/v2.1/remote-scripts/{script_id}` — Update a Script
/// (metadata-only edit).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScriptBody {
    /// Data payload. Required.
    pub data: UpdateScriptData,
    /// Console data. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_data: Option<String>,
    /// Send activity. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_activity: Option<bool>,
}

/// Payload within [`UpdateScriptBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScriptData {
    /// Script name. Required.
    pub script_name: String,
    /// Script type. Required. Enum values: `artifactCollection`,
    /// `dataCollection`, `action`.
    pub script_type: String,
    /// Is input required. Required.
    pub input_required: bool,
    /// Input example. Required.
    pub input_example: String,
    /// Input instructions. Required.
    pub input_instructions: String,
    /// OS types. Required. Enum values: `linux`, `macos`, `windows`.
    pub os_types: Vec<String>,
    /// Script runtime timeout in seconds. Required.
    pub script_runtime_timeout_seconds: i64,
    /// Script description. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_description: Option<String>,
    /// Package expiration option on endpoint. Optional. Enum values: `None`,
    /// `Immediate`, `OnRestart`, `Time`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_endpoint_expiration: Option<String>,
    /// Package expiration time on endpoint. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_endpoint_expiration_seconds: Option<i64>,
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl RemoteopsScriptsService<'_> {
    /// `DELETE /web/api/v2.1/remote-scripts` — Delete Scripts.
    ///
    /// Deletes scripts that match a filter.
    pub async fn delete_scripts(
        &self,
        body: &DeleteScriptsBody,
    ) -> Result<Paginated<EnrichedScript>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteScriptsBody, Paginated<EnrichedScript>>(
                Method::DELETE,
                "/web/api/v2.1/remote-scripts",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/remote-scripts` — Get Scripts.
    ///
    /// Get data of the scripts in the SentinelOne Script Library. The
    /// SentinelOne Script Library, used for the Remote Script Orchestration
    /// feature, gives you a wide range of scripts to collect various forensic
    /// artifacts, parse them, and show them in formats that are easy to analyze.
    /// Use the scripts to collect information such as hardware and software
    /// inventory and configuration, running applications and processes, files
    /// and directories, network connections, and more.
    pub async fn list(
        &self,
        query: &GetScriptsQuery,
    ) -> Result<Paginated<EnrichedScript>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/remote-scripts", q).await?)
    }

    /// `POST /web/api/v2.1/remote-scripts` — Upload New Script.
    ///
    /// Upload a new script file. The file and various properties are required.
    /// To see the mandatory and optional parameters and their valid values, see
    /// the Body Schema or click Run On Console.
    ///
    /// Note: the API expects `multipart/form-data`; [`UploadScriptBody`] covers
    /// the non-file form fields only.
    pub async fn upload(
        &self,
        body: &UploadScriptBody,
    ) -> Result<Response<EnrichedScript>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/remote-scripts", body).await?)
    }

    /// `PUT /web/api/v2.1/remote-scripts/edit/{script_id}` — Update a Script.
    ///
    /// Change the properties of a given script: runtime timeout, name, and
    /// whether input is required (if true, input example and instructions are
    /// required), or script content itself. This command requires the script
    /// ID, which you can get from the Get Scripts API.
    ///
    /// Note: the API expects `multipart/form-data`; [`EditScriptBody`] covers
    /// the non-file form fields only.
    pub async fn edit(
        &self,
        script_id: impl Into<String>,
        body: &EditScriptBody,
    ) -> Result<Response<EnrichedScript>, Error> {
        let path = format!(
            "/web/api/v2.1/remote-scripts/edit/{}",
            script_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<EditScriptBody, Response<EnrichedScript>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/remote-scripts/execute` — Run Remote Script.
    ///
    /// Run a remote script that was uploaded to the SentinelOne Script Library.
    pub async fn execute(
        &self,
        body: &ExecuteScriptBody,
    ) -> Result<Response<RemoteScriptExecuteResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/remote-scripts/execute", body)
            .await?)
    }

    /// `POST /web/api/v2.1/remote-scripts/fetch-files` — Get Script Results.
    ///
    /// Get scripts results URLs. Accessible via API only.
    pub async fn fetch_files(
        &self,
        body: &FetchScriptsResultsBody,
    ) -> Result<Response<DownloadScriptsResults>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/remote-scripts/fetch-files", body)
            .await?)
    }

    /// `GET /web/api/v2.1/remote-scripts/fetch-upload-limits` — Get upload
    /// limit for Package.
    ///
    /// Get upload limit for Package. The response shape is freeform.
    pub async fn fetch_upload_limits(
        &self,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-scripts/fetch-upload-limits", None)
            .await?)
    }

    /// `POST /web/api/v2.1/remote-scripts/guardrails/check` — Check whether
    /// guardrail applies to an execution.
    pub async fn guardrails_check(
        &self,
        body: &GuardrailCheckBody,
    ) -> Result<Response<GuardrailCheckResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/remote-scripts/guardrails/check", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/remote-scripts/guardrails/configuration` — Deletes
    /// a specific guardrails configuration.
    pub async fn delete_guardrails_configuration(
        &self,
        body: &DeleteGuardrailsBody,
    ) -> Result<Response<OperationResultStatus>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteGuardrailsBody, Response<OperationResultStatus>>(
                Method::DELETE,
                "/web/api/v2.1/remote-scripts/guardrails/configuration",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/remote-scripts/guardrails/configuration` — Gets a
    /// guardrails configuration for a given scope.
    pub async fn get_guardrails_configuration(
        &self,
        query: &GetGuardrailsConfigurationQuery,
    ) -> Result<Response<GuardrailsConfiguration>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-scripts/guardrails/configuration", q)
            .await?)
    }

    /// `POST /web/api/v2.1/remote-scripts/guardrails/configuration` — Updates
    /// or inserts (if record does not exist) a guardrails configuration.
    pub async fn upsert_guardrails_configuration(
        &self,
        body: &PostGuardrailsBody,
    ) -> Result<Response<OperationResultStatus>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/remote-scripts/guardrails/configuration", body)
            .await?)
    }

    /// `GET /web/api/v2.1/remote-scripts/pending-executions` — Get paginated
    /// pending executions.
    pub async fn list_pending_executions(
        &self,
        query: &GetPendingExecutionsQuery,
    ) -> Result<Paginated<PendingExecution>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-scripts/pending-executions", q)
            .await?)
    }

    /// `PUT
    /// /web/api/v2.1/remote-scripts/pending-executions/{pending_execution_id}`
    /// — Approve/decline pending execution.
    pub async fn approve_decline_pending_execution(
        &self,
        pending_execution_id: impl Into<String>,
        body: &ApproveDeclinePendingExecutionBody,
    ) -> Result<Response<OperationResultStatus>, Error> {
        let path = format!(
            "/web/api/v2.1/remote-scripts/pending-executions/{}",
            pending_execution_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<ApproveDeclinePendingExecutionBody, Response<OperationResultStatus>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/remote-scripts/script-content` — Get script content.
    ///
    /// Get Script content by script id.
    pub async fn get_script_content(
        &self,
        query: &GetScriptContentQuery,
    ) -> Result<Response<ScriptContent>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-scripts/script-content", q)
            .await?)
    }

    /// `GET /web/api/v2.1/remote-scripts/status` — Get Remote Scripts Tasks
    /// Status.
    ///
    /// Get remote scripts tasks using a variety of filters. Accessible via API
    /// only. `parent_task_id` or `parent_task_id__in` query parameter is
    /// mandatory.
    pub async fn status(
        &self,
        query: &GetStatusQuery,
    ) -> Result<Paginated<RemoteScriptTaskStatus>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-scripts/status", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/remote-scripts/{script_id}` — Update a Script.
    ///
    /// Change the properties of a given script: runtime timeout, name, and
    /// whether input is required (if true, input example and instructions are
    /// required). This command requires the script ID, which you can get from
    /// the Get Scripts API.
    pub async fn update(
        &self,
        script_id: impl Into<String>,
        body: &UpdateScriptBody,
    ) -> Result<Response<EnrichedScript>, Error> {
        let path = format!(
            "/web/api/v2.1/remote-scripts/{}",
            script_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<UpdateScriptBody, Response<EnrichedScript>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }
}
