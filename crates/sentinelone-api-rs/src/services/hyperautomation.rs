use sentinelone_http::Method;
use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::hyperautomation::*;
use crate::pagination::{Paginated, Response};

/// `Hyperautomation` tag — Hyperautomation Public APIs.
///
/// Workflows, workflow versions, workflow executions, expression evaluation and
/// the import/export of workflows and custom integrations.
pub struct HyperautomationService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query structs
// ---------------------------------------------------------------------------

/// Query params for `GET .../custom-integration-import-export/export`.
///
/// `groupIds` / `siteIds` / `accountIds` are scope filters serialized
/// comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportCustomIntegrationsQuery {
    /// Comma-separated custom integration ids to export. **Required** by the API
    /// (passed via [`HyperautomationService::export_custom_integrations`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_integration_ids: Option<String>,
    /// Scope filter: group ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Scope filter: site ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Scope filter: account ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl ExportCustomIntegrationsQuery {
    /// Comma-separated custom integration ids to export.
    pub fn custom_integration_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.custom_integration_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: group ids.
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: site ids.
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: account ids.
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }
}

/// Scope-only query params (`groupIds` / `siteIds` / `accountIds`), shared by
/// the many endpoints that accept just scope filters.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeQuery {
    /// Scope filter: group ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Scope filter: site ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Scope filter: account ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl ScopeQuery {
    /// Scope filter: group ids.
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: site ids.
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: account ids.
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }
}

/// Query params for `GET .../workflow-execution` (list all workflow executions).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWorkflowExecutionsQuery {
    /// Filter by trigger types (comma-joined). Allowed values: `http_trigger`,
    /// `scheduled_trigger`, `email_trigger`, `manual_trigger`,
    /// `singularity_response_trigger`, `snippet_trigger`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_types: Option<String>,
    /// Filter by execution states (comma-joined). Allowed values: `Running`,
    /// `Pending`, `Stuck`, `Completed`, `Error`, `Waiting`, `Aborted`,
    /// `CompletedWithErrors`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<String>,
    /// Filter by scope ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<String>,
    /// Versions count filter. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions_count: Option<String>,
    /// Created at, greater-than-or-equal (date-time). Optional.
    #[serde(rename = "created_at__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Created at, less-than (date-time). Optional.
    #[serde(rename = "created_at__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Maximum number of results. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of results to skip. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Filter by workflow name substring. Optional.
    #[serde(
        rename = "workflow_name__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub workflow_name_contains: Option<String>,
    /// Filter by integrations. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<String>,
    /// Filter by workflow id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
    /// Whether to include snippets. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_snippet: Option<bool>,
    /// Filter by tags (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Field to sort by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort order (e.g. `asc` / `desc`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Scope filter: group ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Scope filter: site ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Scope filter: account ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl ListWorkflowExecutionsQuery {
    /// Filter by trigger types (comma-joined).
    pub fn trigger_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.trigger_types = Some(join_csv(vals));
        self
    }
    /// Filter by execution states (comma-joined).
    pub fn states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states = Some(join_csv(vals));
        self
    }
    /// Filter by scope ids (comma-joined).
    pub fn scope_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_ids = Some(join_csv(vals));
        self
    }
    /// Versions count filter.
    pub fn versions_count(mut self, v: impl Into<String>) -> Self {
        self.versions_count = Some(v.into());
        self
    }
    /// Created at, greater-than-or-equal (date-time).
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Created at, less-than (date-time).
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Maximum number of results.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Number of results to skip.
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Filter by workflow name substring.
    pub fn workflow_name_contains(mut self, v: impl Into<String>) -> Self {
        self.workflow_name_contains = Some(v.into());
        self
    }
    /// Filter by integrations.
    pub fn integrations(mut self, v: impl Into<String>) -> Self {
        self.integrations = Some(v.into());
        self
    }
    /// Filter by workflow id.
    pub fn workflow_id(mut self, v: impl Into<String>) -> Self {
        self.workflow_id = Some(v.into());
        self
    }
    /// Whether to include snippets.
    pub fn is_snippet(mut self, v: bool) -> Self {
        self.is_snippet = Some(v);
        self
    }
    /// Filter by tags (comma-joined).
    pub fn tags<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags = Some(join_csv(vals));
        self
    }
    /// Field to sort by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort order (`asc` / `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Scope filter: group ids.
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: site ids.
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: account ids.
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }
}

/// Query params for `GET .../workflow-import-export/export` (batch export).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportWorkflowsQuery {
    /// Filter by workflow ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_ids: Option<String>,
    /// Filter by integrations (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<String>,
    /// Filter by trigger types (comma-joined). Allowed values: `http_trigger`,
    /// `scheduled_trigger`, `email_trigger`, `manual_trigger`,
    /// `singularity_response_trigger`, `snippet_trigger`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_types: Option<String>,
    /// Filter by core actions (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_actions: Option<String>,
    /// Filter by workflow states (comma-joined). Allowed values: `active`,
    /// `inactive`, `deactivated`, `draft`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<String>,
    /// Filter by scope ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<String>,
    /// Filter by name substring. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Filter by description substring. Optional.
    #[serde(
        rename = "description__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub description_contains: Option<String>,
    /// Filter by exact name. Optional.
    #[serde(rename = "name__eq", skip_serializing_if = "Option::is_none")]
    pub name_eq: Option<String>,
    /// Oversight filter. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversight: Option<bool>,
    /// Filter by tags (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Scope filter: group ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Scope filter: site ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Scope filter: account ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl ExportWorkflowsQuery {
    /// Filter by workflow ids (comma-joined).
    pub fn workflow_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.workflow_ids = Some(join_csv(vals));
        self
    }
    /// Filter by integrations (comma-joined).
    pub fn integrations<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.integrations = Some(join_csv(vals));
        self
    }
    /// Filter by trigger types (comma-joined).
    pub fn trigger_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.trigger_types = Some(join_csv(vals));
        self
    }
    /// Filter by core actions (comma-joined).
    pub fn core_actions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_actions = Some(join_csv(vals));
        self
    }
    /// Filter by workflow states (comma-joined).
    pub fn states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states = Some(join_csv(vals));
        self
    }
    /// Filter by scope ids (comma-joined).
    pub fn scope_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_ids = Some(join_csv(vals));
        self
    }
    /// Filter by name substring.
    pub fn name_contains(mut self, v: impl Into<String>) -> Self {
        self.name_contains = Some(v.into());
        self
    }
    /// Filter by description substring.
    pub fn description_contains(mut self, v: impl Into<String>) -> Self {
        self.description_contains = Some(v.into());
        self
    }
    /// Filter by exact name.
    pub fn name_eq(mut self, v: impl Into<String>) -> Self {
        self.name_eq = Some(v.into());
        self
    }
    /// Oversight filter.
    pub fn oversight(mut self, v: bool) -> Self {
        self.oversight = Some(v);
        self
    }
    /// Filter by tags (comma-joined).
    pub fn tags<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags = Some(join_csv(vals));
        self
    }
    /// Scope filter: group ids.
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: site ids.
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: account ids.
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }
}

/// Query params for `GET .../workflows` (list all workflows).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListWorkflowsQuery {
    /// Filter by integrations (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<String>,
    /// Filter by trigger types (comma-joined). Allowed values: `http_trigger`,
    /// `scheduled_trigger`, `email_trigger`, `manual_trigger`,
    /// `singularity_response_trigger`, `snippet_trigger`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_types: Option<String>,
    /// Filter by core actions (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_actions: Option<String>,
    /// Filter by workflow states (comma-joined). Allowed values: `active`,
    /// `inactive`, `deactivated`, `draft`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<String>,
    /// Filter by scope ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<String>,
    /// Maximum number of results. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Number of results to skip. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Whether to include snippets. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_snippet: Option<bool>,
    /// Filter by name substring. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Filter by description substring. Optional.
    #[serde(
        rename = "description__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub description_contains: Option<String>,
    /// Filter by exact name. Optional.
    #[serde(rename = "name__eq", skip_serializing_if = "Option::is_none")]
    pub name_eq: Option<String>,
    /// Filter by tags (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Field to sort by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort order (`asc` / `desc`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Oversight filter. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oversight: Option<bool>,
    /// Filter by workflow ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_ids: Option<String>,
    /// Scope filter: group ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Scope filter: site ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Scope filter: account ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl ListWorkflowsQuery {
    /// Filter by integrations (comma-joined).
    pub fn integrations<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.integrations = Some(join_csv(vals));
        self
    }
    /// Filter by trigger types (comma-joined).
    pub fn trigger_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.trigger_types = Some(join_csv(vals));
        self
    }
    /// Filter by core actions (comma-joined).
    pub fn core_actions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_actions = Some(join_csv(vals));
        self
    }
    /// Filter by workflow states (comma-joined).
    pub fn states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states = Some(join_csv(vals));
        self
    }
    /// Filter by scope ids (comma-joined).
    pub fn scope_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_ids = Some(join_csv(vals));
        self
    }
    /// Maximum number of results.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Number of results to skip.
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Whether to include snippets.
    pub fn is_snippet(mut self, v: bool) -> Self {
        self.is_snippet = Some(v);
        self
    }
    /// Filter by name substring.
    pub fn name_contains(mut self, v: impl Into<String>) -> Self {
        self.name_contains = Some(v.into());
        self
    }
    /// Filter by description substring.
    pub fn description_contains(mut self, v: impl Into<String>) -> Self {
        self.description_contains = Some(v.into());
        self
    }
    /// Filter by exact name.
    pub fn name_eq(mut self, v: impl Into<String>) -> Self {
        self.name_eq = Some(v.into());
        self
    }
    /// Filter by tags (comma-joined).
    pub fn tags<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags = Some(join_csv(vals));
        self
    }
    /// Field to sort by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort order (`asc` / `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Oversight filter.
    pub fn oversight(mut self, v: bool) -> Self {
        self.oversight = Some(v);
        self
    }
    /// Filter by workflow ids (comma-joined).
    pub fn workflow_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.workflow_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: group ids.
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: site ids.
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: account ids.
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }
}

/// Query params for `POST .../workflows/{workflow_id}/deactivate`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivateWorkflowQuery {
    /// The workflow version id to deactivate. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    /// Scope filter: group ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Scope filter: site ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Scope filter: account ids (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl DeactivateWorkflowQuery {
    /// The workflow version id to deactivate.
    pub fn version_id(mut self, v: impl Into<String>) -> Self {
        self.version_id = Some(v.into());
        self
    }
    /// Scope filter: group ids.
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: site ids.
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }
    /// Scope filter: account ids.
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }
}

// ---------------------------------------------------------------------------
// Body structs
// ---------------------------------------------------------------------------

/// Scope filter embedded in `S1ApiBody` request bodies.
#[derive(Debug, Clone, Serialize)]
pub struct Filter {
    /// Filter type. **Required.**
    #[serde(rename = "type")]
    pub type_: String,
    /// Filter value. **Required.**
    pub value: serde_json::Value,
}

/// Body for `POST .../custom-integration-import-export/import`
/// (`S1ApiBody[CustomIntegrationImportExport]`).
#[derive(Debug, Default, Serialize)]
pub struct ImportCustomIntegrationBody {
    /// The custom integration to import (previously exported). Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Scope filter. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

impl ImportCustomIntegrationBody {
    /// The custom integration payload to import.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
    /// Scope filter.
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Body for `POST .../custom-integration-import-export/import/batch`.
///
/// The spec models this as a `multipart/form-data` upload (`file` binary +
/// `filter` string). The JSON transport only sends JSON, so the file content is
/// passed as a string field; supply a base64 / text payload as appropriate.
#[derive(Debug, Serialize)]
pub struct ImportCustomIntegrationBatchBody {
    /// The exported file contents. **Required.**
    pub file: String,
    /// The scope filter, as a string. **Required.**
    pub filter: String,
}

/// Input for an expression evaluation / breakdown
/// (`S1ApiBody[ExpressionEvaluationInput]`).
#[derive(Debug, Default, Serialize)]
pub struct ExpressionEvaluationBody {
    /// The expression-evaluation input. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ExpressionEvaluationInput>,
    /// Scope filter. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

impl ExpressionEvaluationBody {
    /// The expression-evaluation input.
    pub fn data(mut self, data: ExpressionEvaluationInput) -> Self {
        self.data = Some(data);
        self
    }
    /// Scope filter.
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// The `data` of an expression evaluation request.
#[derive(Debug, Default, Serialize)]
pub struct ExpressionEvaluationInput {
    /// The expression to evaluate. **Required.**
    pub expression: String,
    /// Loop context (freeform object). Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_context: Option<serde_json::Value>,
}

impl ExpressionEvaluationInput {
    /// Create with the required expression.
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
            loop_context: None,
        }
    }
    /// Loop context.
    pub fn loop_context(mut self, ctx: serde_json::Value) -> Self {
        self.loop_context = Some(ctx);
        self
    }
}

/// Body for `POST .../workflow-execution/manual/{workflow_id}/{version_id}`
/// (`S1ApiBody[WorkflowExecutionCreate]`).
#[derive(Debug, Default, Serialize)]
pub struct TriggerWorkflowBody {
    /// The execution-create input. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WorkflowExecutionCreate>,
    /// Scope filter. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

impl TriggerWorkflowBody {
    /// The execution-create input.
    pub fn data(mut self, data: WorkflowExecutionCreate) -> Self {
        self.data = Some(data);
        self
    }
    /// Scope filter.
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// The `data` of a manual workflow-execution request.
#[derive(Debug, Default, Serialize)]
pub struct WorkflowExecutionCreate {
    /// Execution payload (string). Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    /// Singularity response event id. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singularity_response_event_id: Option<String>,
    /// Singularity response event type. Enum: `alert`, `incident`,
    /// `misconfiguration`, `vulnerability`, `activity`. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singularity_response_event_type: Option<String>,
    /// Whether this is a downstream execution. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_downstream_execution: Option<bool>,
    /// Parent execution id. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_execution_id: Option<String>,
    /// Whether this is a snippet execution. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_snippet_execution: Option<bool>,
    /// Singularity response execution source (enum string). Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singularity_response_execution_source: Option<String>,
    /// Singularity response event dedup id. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singularity_response_event_dedup_id: Option<String>,
}

/// Body for `POST .../workflow-import-export/import`
/// (`S1ApiBody[WorkflowImportExport]`).
#[derive(Debug, Default, Serialize)]
pub struct ImportWorkflowBody {
    /// The workflow to import (previously exported). Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Scope filter. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

impl ImportWorkflowBody {
    /// The workflow payload to import.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
    /// Scope filter.
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Body for `POST .../workflow-import-export/import/batch`.
///
/// The spec models this as a `multipart/form-data` upload (`file` binary +
/// `filter` string). The JSON transport only sends JSON, so the file content is
/// passed as a string field; supply a base64 / text payload as appropriate.
#[derive(Debug, Serialize)]
pub struct ImportWorkflowBatchBody {
    /// The exported file contents. **Required.**
    pub file: String,
    /// The scope filter, as a string. **Required.**
    pub filter: String,
}

/// Body for `POST .../workflows/{workflow_id}/{version_id}/activation`
/// (`S1ApiBody[WorkflowPatch]`).
#[derive(Debug, Default, Serialize)]
pub struct ActivateWorkflowBody {
    /// The workflow patch to apply on activation. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WorkflowPatch>,
    /// Scope filter. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

impl ActivateWorkflowBody {
    /// The workflow patch.
    pub fn data(mut self, data: WorkflowPatch) -> Self {
        self.data = Some(data);
        self
    }
    /// Scope filter.
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// A partial workflow update applied on activation.
#[derive(Debug, Default, Serialize)]
pub struct WorkflowPatch {
    /// Version description. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
    /// Execution timeout (seconds). Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// Daily maximum executions. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_max_executions: Option<i64>,
    /// Maximum concurrency. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<i64>,
    /// Notification recipients. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_to: Option<Vec<String>>,
    /// Time saved. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_saved: Option<i64>,
    /// Time saved unit. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_saved_unit: Option<String>,
    /// Whether the workflow is a snippet. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_snippet: Option<bool>,
    /// Canvas dimensions (freeform object). Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<serde_json::Value>,
}

/// Generic `S1ApiBody` body (`{ data, filter }`) used by the deactivate
/// endpoint, where `data` is unconstrained.
#[derive(Debug, Default, Serialize)]
pub struct DeactivateWorkflowBody {
    /// Freeform request data. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Scope filter. Optional / nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<Filter>,
}

impl DeactivateWorkflowBody {
    /// Freeform request data.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
    /// Scope filter.
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl HyperautomationService<'_> {
    /// `GET /web/api/v2.1/hyper-automate/api/public/custom-integration-import-export/export`
    /// — Batch export custom custom integrations.
    ///
    /// Returns the exported custom integrations (freeform export document).
    pub async fn export_custom_integrations(
        &self,
        query: &ExportCustomIntegrationsQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/hyper-automate/api/public/custom-integration-import-export/export",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/hyper-automate/api/public/custom-integration-import-export/export/{custom_integration_id}`
    /// — Export custom integration.
    ///
    /// Export a specific custom integration with its actions.
    ///
    /// `custom_integration_id` — The custom integration id. **Required.**
    pub async fn export_custom_integration(
        &self,
        custom_integration_id: impl Into<String>,
        query: &ScopeQuery,
    ) -> Result<Response<CustomIntegrationImportExport>, Error> {
        let path = format!(
            "/web/api/v2.1/hyper-automate/api/public/custom-integration-import-export/export/{}",
            custom_integration_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `POST /web/api/v2.1/hyper-automate/api/public/custom-integration-import-export/import`
    /// — Import custom integration.
    ///
    /// Import a custom integration that has been previously exported.
    pub async fn import_custom_integration(
        &self,
        query: &ScopeQuery,
        body: &ImportCustomIntegrationBody,
    ) -> Result<Response<PublicIntegrationReadScopeMeta>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<ImportCustomIntegrationBody, Response<PublicIntegrationReadScopeMeta>>(
                Method::POST,
                "/web/api/v2.1/hyper-automate/api/public/custom-integration-import-export/import",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/hyper-automate/api/public/custom-integration-import-export/import/batch`
    /// — Batch import custom integrations.
    ///
    /// Import custom integrations that have been previously exported.
    pub async fn import_custom_integrations_batch(
        &self,
        body: &ImportCustomIntegrationBatchBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post(
                "/web/api/v2.1/hyper-automate/api/public/custom-integration-import-export/import/batch",
                body,
            )
            .await?)
    }

    /// `POST /web/api/v2.1/hyper-automate/api/public/workflow-action-expressions/{base_action_id}/evaluate-expression`
    /// — Evaluate Expression.
    ///
    /// `base_action_id` — The base action id (uuid). **Required.**
    pub async fn evaluate_expression(
        &self,
        base_action_id: impl Into<String>,
        query: &ScopeQuery,
        body: &ExpressionEvaluationBody,
    ) -> Result<Response<ExpressionEvaluationResult>, Error> {
        let path = format!(
            "/web/api/v2.1/hyper-automate/api/public/workflow-action-expressions/{}/evaluate-expression",
            base_action_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<ExpressionEvaluationBody, Response<ExpressionEvaluationResult>>(
                Method::POST,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/hyper-automate/api/public/workflow-action-expressions/{base_action_id}/expression-breakdown`
    /// — Expression Breakdown.
    ///
    /// `base_action_id` — The base action id (uuid). **Required.**
    pub async fn expression_breakdown(
        &self,
        base_action_id: impl Into<String>,
        query: &ScopeQuery,
        body: &ExpressionEvaluationBody,
    ) -> Result<Response<ExpressionEvaluationResult>, Error> {
        let path = format!(
            "/web/api/v2.1/hyper-automate/api/public/workflow-action-expressions/{}/expression-breakdown",
            base_action_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<ExpressionEvaluationBody, Response<ExpressionEvaluationResult>>(
                Method::POST,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/hyper-automate/api/public/workflow-execution`
    /// — List all workflow executions.
    pub async fn list_workflow_executions(
        &self,
        query: &ListWorkflowExecutionsQuery,
    ) -> Result<Paginated<WorkflowExecutionInfratable>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/hyper-automate/api/public/workflow-execution",
                q,
            )
            .await?)
    }

    /// `POST /web/api/v2.1/hyper-automate/api/public/workflow-execution/manual/{workflow_id}/{version_id}`
    /// — Trigger a workflow that uses a manual trigger or a scheduled trigger.
    ///
    /// `workflow_id` — The workflow id (uuid). **Required.**
    /// `version_id` — The workflow version id (uuid). **Required.**
    pub async fn trigger_workflow(
        &self,
        workflow_id: impl Into<String>,
        version_id: impl Into<String>,
        query: &ScopeQuery,
        body: &TriggerWorkflowBody,
    ) -> Result<Response<WorkflowExecutionRead>, Error> {
        let path = format!(
            "/web/api/v2.1/hyper-automate/api/public/workflow-execution/manual/{}/{}",
            workflow_id.into(),
            version_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<TriggerWorkflowBody, Response<WorkflowExecutionRead>>(
                Method::POST,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/hyper-automate/api/public/workflow-execution/output/{execution_id}/raw`
    /// — Get workflow execution output.
    ///
    /// `execution_id` — The execution id. **Required.**
    pub async fn get_workflow_execution_output(
        &self,
        execution_id: impl Into<String>,
        query: &ScopeQuery,
    ) -> Result<Response<ExecutionFinalResponse>, Error> {
        let path = format!(
            "/web/api/v2.1/hyper-automate/api/public/workflow-execution/output/{}/raw",
            execution_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `GET /web/api/v2.1/hyper-automate/api/public/workflow-execution/{workflow_execution_id}`
    /// — Get a workflow execution by its ID.
    ///
    /// `workflow_execution_id` — The workflow execution id (uuid). **Required.**
    pub async fn get_workflow_execution(
        &self,
        workflow_execution_id: impl Into<String>,
        query: &ScopeQuery,
    ) -> Result<Response<PublicWorkflowExecutionResponse>, Error> {
        let path = format!(
            "/web/api/v2.1/hyper-automate/api/public/workflow-execution/{}",
            workflow_execution_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `GET /web/api/v2.1/hyper-automate/api/public/workflow-import-export/export`
    /// — Batch export workflows.
    ///
    /// Returns the exported workflows (freeform export document).
    pub async fn export_workflows(
        &self,
        query: &ExportWorkflowsQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/hyper-automate/api/public/workflow-import-export/export",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/hyper-automate/api/public/workflow-import-export/export/{workflow_id}/{version_id}`
    /// — Export workflow.
    ///
    /// Export a specific workflow version.
    ///
    /// `workflow_id` — The workflow id (uuid). **Required.**
    /// `version_id` — The workflow version id (uuid). **Required.**
    pub async fn export_workflow(
        &self,
        workflow_id: impl Into<String>,
        version_id: impl Into<String>,
        query: &ScopeQuery,
    ) -> Result<Response<WorkflowImportExport>, Error> {
        let path = format!(
            "/web/api/v2.1/hyper-automate/api/public/workflow-import-export/export/{}/{}",
            workflow_id.into(),
            version_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `POST /web/api/v2.1/hyper-automate/api/public/workflow-import-export/import`
    /// — Import workflow.
    ///
    /// Import workflows that have been previously exported from Hyperautomation.
    pub async fn import_workflow(
        &self,
        query: &ScopeQuery,
        body: &ImportWorkflowBody,
    ) -> Result<Response<WorkflowReadScopeMeta>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<ImportWorkflowBody, Response<WorkflowReadScopeMeta>>(
                Method::POST,
                "/web/api/v2.1/hyper-automate/api/public/workflow-import-export/import",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/hyper-automate/api/public/workflow-import-export/import/batch`
    /// — Batch import workflows.
    ///
    /// Import workflows that have been previously exported from Hyperautomation.
    pub async fn import_workflows_batch(
        &self,
        body: &ImportWorkflowBatchBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post(
                "/web/api/v2.1/hyper-automate/api/public/workflow-import-export/import/batch",
                body,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/hyper-automate/api/public/workflows`
    /// — List all workflows.
    pub async fn list_workflows(
        &self,
        query: &ListWorkflowsQuery,
    ) -> Result<Paginated<WorkflowWithActionsMeta>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/hyper-automate/api/public/workflows", q)
            .await?)
    }

    /// `GET /web/api/v2.1/hyper-automate/api/public/workflows/versions/list/{workflow_id}`
    /// — List workflow versions.
    ///
    /// `workflow_id` — The workflow id (uuid). **Required.**
    pub async fn list_workflow_versions(
        &self,
        workflow_id: impl Into<String>,
        query: &ScopeQuery,
    ) -> Result<Response<PublicWorkflowVersionData>, Error> {
        let path = format!(
            "/web/api/v2.1/hyper-automate/api/public/workflows/versions/list/{}",
            workflow_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `POST /web/api/v2.1/hyper-automate/api/public/workflows/{workflow_id}/deactivate`
    /// — Deactivate The active workflow.
    ///
    /// Returns HTTP 204 (no content) on success.
    ///
    /// `workflow_id` — The workflow id (uuid). **Required.**
    pub async fn deactivate_workflow(
        &self,
        workflow_id: impl Into<String>,
        query: &DeactivateWorkflowQuery,
        body: &DeactivateWorkflowBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/hyper-automate/api/public/workflows/{}/deactivate",
            workflow_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<DeactivateWorkflowBody, Response<serde_json::Value>>(
                Method::POST,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/hyper-automate/api/public/workflows/{workflow_id}/{version_id}/activation`
    /// — Activate a workflow version.
    ///
    /// Returns HTTP 204 (no content) on success.
    ///
    /// `workflow_id` — The workflow id (uuid). **Required.**
    /// `version_id` — The workflow version id (uuid). **Required.**
    pub async fn activate_workflow(
        &self,
        workflow_id: impl Into<String>,
        version_id: impl Into<String>,
        query: &ScopeQuery,
        body: &ActivateWorkflowBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/hyper-automate/api/public/workflows/{}/{}/activation",
            workflow_id.into(),
            version_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<ActivateWorkflowBody, Response<serde_json::Value>>(
                Method::POST,
                &path,
                q,
                Some(body),
            )
            .await?)
    }
}

/// Join an iterator of string-like values into a comma-separated string, as the
/// SentinelOne API expects for array-valued query params.
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
