//! `Exclusions and Blocklist` tag — Exclusions related endpoints.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::exclusions_and_blocklist::*;
use crate::pagination::{Paginated, Response};

/// `Exclusions and Blocklist` tag.
///
/// Exclusions related endpoints, covering both the Exclusions list and the
/// Blocklist (restrictions): list/create/update/delete, CSV import and its
/// validation report, item validation, and CSV export.
pub struct ExclusionsAndBlocklistService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query params
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/exclusions` — Get Exclusions.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExclusionsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: `YWdlbnRfaWQ6NTgwMjkzODE=`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items is returned, without the actual
    /// objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, the total number of items is not calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `id`, `createdAt`,
    /// `updatedAt`, `mode`, `source`, `description`, `pathExclusionType`,
    /// `osType`. Example: `id`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Example: `asc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Comma-separated list of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Comma-separated list of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Comma-separated list of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Comma-separated list of IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Unified. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified: Option<bool>,
    /// Value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Created before this timestamp. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Created before or at this timestamp. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Created after this timestamp. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Created after or at this timestamp. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Date range for creation time (format: `<from>-<to>`, inclusive).
    /// Example: `1514978890136-1514978650130`. Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// Updated before this timestamp. Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at_lt: Option<String>,
    /// Updated before or at this timestamp. Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// Updated after this timestamp. Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at_gt: Option<String>,
    /// Updated after or at this timestamp. Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// Date range for update time (format: `<from>-<to>`, inclusive). Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at_between: Option<String>,
    /// A free-text search term, will match applicable attributes. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Type. Allowed values: `path`, `certificate`, `browser`, `file_type`,
    /// `white_hash`, `dv_exclusions`. Example: `path`. Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Comma-separated list of OS types to filter by. Allowed item values:
    /// `linux`, `macos`, `windows_legacy`, `windows`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Comma-separated list of sources to filter by. Allowed item values:
    /// `user`, `cloud`, `action_from_threat`, `catalog`, `performance_insight`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Comma-separated list of user ids to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<String>,
    /// Comma-separated list of recommendations to filter by. Allowed item
    /// values: `Not recommended`, `Not allowed`, `NONE`, `duplicated_value_sha1`,
    /// `duplicated_value_sha256`, `duplicated_value_sha1_sha256`, `Duplication`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<String>,
    /// Free-text filter by value (comma-separated). Optional.
    #[serde(rename = "value__contains", skip_serializing_if = "Option::is_none")]
    pub value_contains: Option<String>,
    /// Free-text filter by description (comma-separated). Optional.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// Free-text filter by user name (comma-separated). Optional.
    #[serde(rename = "user__contains", skip_serializing_if = "Option::is_none")]
    pub user_contains: Option<String>,
    /// Type in. Allowed item values: `path`, `certificate`, `browser`,
    /// `file_type`, `white_hash`, `dv_exclusions`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// Comma-separated list of modes to filter by (Path exclusions only).
    /// Allowed item values: `suppress`, `suppress_dynamic_only`,
    /// `suppress_dfi_only`, `disable_in_process_monitor`,
    /// `disable_in_process_monitor_deep`, `disable_all_monitors`,
    /// `disable_all_monitors_deep`, `suppress_app_control`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<String>,
    /// Comma-separated list of excluded paths in an exclusion (Path exclusions
    /// only). Allowed item values: `file`, `folder`, `subfolders`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_exclusion_types: Option<String>,
    /// Return filters from parent scope levels (default: false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels (default: false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// Filter by import status (comma-separated): `true` (imported), `false`
    /// (not imported), or both for multiselect. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<String>,
    /// Free-text filter by application name (comma-separated). Optional.
    #[serde(rename = "applicationName__contains", skip_serializing_if = "Option::is_none")]
    pub application_name_contains: Option<String>,
    /// Found or Not found - whether this exclusion is related to an application
    /// found in the scope's Application Inventory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_inventory: Option<bool>,
    /// Mode type. Allowed values: `all`, `suppression`, `agent_interoperability`,
    /// `binary_vault`. Example: `all`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_type: Option<String>,
    /// Child process. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_process: Option<bool>,
    /// Threat type in (comma-separated). Allowed item values: `EDR`, `IDR`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_type: Option<String>,
    /// Engine in (comma-separated). Allowed item values: `suppress`,
    /// `suppress_dfi_only`, `suppress_dynamic_only`, `suppress_app_control`,
    /// `suppress_drift_detection`, `ad_secure_ep`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<String>,
    /// Interaction level in (comma-separated). Allowed item values:
    /// `disable_all_monitors`, `disable_in_process_monitor`, `identity_only`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_level: Option<String>,
    /// Conditions in (comma-separated). Allowed item values: `white_hash`,
    /// `path`, `certificate`, `file_type`, `browser`, `commandline`,
    /// `container_native`, `user`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<String>,
    /// Exclusion name like any (comma-separated). Optional.
    #[serde(rename = "exclusionName__contains", skip_serializing_if = "Option::is_none")]
    pub exclusion_name_contains: Option<String>,
    /// Not recommended in (comma-separated). Allowed item values:
    /// `Not recommended`, `Not allowed`, `NONE`, `duplicated_value_sha1`,
    /// `duplicated_value_sha256`, `duplicated_value_sha1_sha256`, `Duplication`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_recommended: Option<String>,
    /// Scope path like any (comma-separated). Optional.
    #[serde(rename = "scopePath__contains", skip_serializing_if = "Option::is_none")]
    pub scope_path_contains: Option<String>,
}

/// Joins an iterator of string-likes into a comma-separated string.
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

impl ExclusionsQuery {
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
    /// If true, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, the total number of items is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Sort column. See struct docs for allowed values.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction: `asc` or `desc`.
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
    /// Unified.
    pub fn unified(mut self, v: bool) -> Self {
        self.unified = Some(v);
        self
    }
    /// Value.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    /// Created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Date range for creation time.
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// Updated before this timestamp.
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lt = Some(v.into());
        self
    }
    /// Updated before or at this timestamp.
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// Updated after this timestamp.
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gt = Some(v.into());
        self
    }
    /// Updated after or at this timestamp.
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// Date range for update time.
    pub fn updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.updated_at_between = Some(v.into());
        self
    }
    /// A free-text search term.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Type. See struct docs for allowed values.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    /// List of OS types to filter by.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// List of sources to filter by.
    pub fn source<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source = Some(join_csv(v));
        self
    }
    /// List of user ids to filter by.
    pub fn user_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_ids = Some(join_csv(v));
        self
    }
    /// List of recommendations to filter by.
    pub fn recommendations<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.recommendations = Some(join_csv(v));
        self
    }
    /// Free-text filter by value.
    pub fn value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.value_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by description.
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by user name.
    pub fn user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_contains = Some(join_csv(v));
        self
    }
    /// Type in.
    pub fn types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.types = Some(join_csv(v));
        self
    }
    /// List of modes to filter by (Path exclusions only).
    pub fn modes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.modes = Some(join_csv(v));
        self
    }
    /// List of excluded path types (Path exclusions only).
    pub fn path_exclusion_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.path_exclusion_types = Some(join_csv(v));
        self
    }
    /// Return filters from parent scope levels.
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Return filters from children scope levels.
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
    /// Filter by import status.
    pub fn imported<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.imported = Some(join_csv(v));
        self
    }
    /// Free-text filter by application name.
    pub fn application_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name_contains = Some(join_csv(v));
        self
    }
    /// Found or Not found in Application Inventory.
    pub fn in_app_inventory(mut self, v: bool) -> Self {
        self.in_app_inventory = Some(v);
        self
    }
    /// Mode type. See struct docs for allowed values.
    pub fn mode_type(mut self, v: impl Into<String>) -> Self {
        self.mode_type = Some(v.into());
        self
    }
    /// Child process.
    pub fn child_process(mut self, v: bool) -> Self {
        self.child_process = Some(v);
        self
    }
    /// Threat type in.
    pub fn threat_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_type = Some(join_csv(v));
        self
    }
    /// Engine in.
    pub fn engines<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.engines = Some(join_csv(v));
        self
    }
    /// Interaction level in.
    pub fn interaction_level<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.interaction_level = Some(join_csv(v));
        self
    }
    /// Conditions in.
    pub fn conditions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.conditions = Some(join_csv(v));
        self
    }
    /// Exclusion name like any.
    pub fn exclusion_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.exclusion_name_contains = Some(join_csv(v));
        self
    }
    /// Not recommended in.
    pub fn not_recommended<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.not_recommended = Some(join_csv(v));
        self
    }
    /// Scope path like any.
    pub fn scope_path_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_path_contains = Some(join_csv(v));
        self
    }
}

/// Query params for `GET /web/api/v2.1/export/exclusions` — Export Exclusions.
///
/// Identical filter set to [`ExclusionsQuery`] minus the pagination/sort
/// controls (`skip`, `limit`, `cursor`, `countOnly`, `skipCount`, `sortBy`,
/// `sortOrder`), since the export streams a CSV of all matching items.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportExclusionsQuery {
    /// Comma-separated list of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Comma-separated list of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Comma-separated list of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Comma-separated list of IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Unified. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified: Option<bool>,
    /// Value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Created before this timestamp. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Created before or at this timestamp. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Created after this timestamp. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Created after or at this timestamp. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Date range for creation time. Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// Updated before this timestamp. Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at_lt: Option<String>,
    /// Updated before or at this timestamp. Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// Updated after this timestamp. Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at_gt: Option<String>,
    /// Updated after or at this timestamp. Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// Date range for update time. Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at_between: Option<String>,
    /// A free-text search term. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Type. Allowed values: `path`, `certificate`, `browser`, `file_type`,
    /// `white_hash`, `dv_exclusions`. Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Comma-separated list of OS types. Allowed item values: `linux`, `macos`,
    /// `windows_legacy`, `windows`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Comma-separated list of sources. Allowed item values: `user`, `cloud`,
    /// `action_from_threat`, `catalog`, `performance_insight`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Comma-separated list of user ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<String>,
    /// Comma-separated list of recommendations. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<String>,
    /// Free-text filter by value. Optional.
    #[serde(rename = "value__contains", skip_serializing_if = "Option::is_none")]
    pub value_contains: Option<String>,
    /// Free-text filter by description. Optional.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// Free-text filter by user name. Optional.
    #[serde(rename = "user__contains", skip_serializing_if = "Option::is_none")]
    pub user_contains: Option<String>,
    /// Type in. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// List of modes to filter by (Path exclusions only). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<String>,
    /// List of excluded path types (Path exclusions only). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_exclusion_types: Option<String>,
    /// Return filters from parent scope levels. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// Filter by import status. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<String>,
    /// Free-text filter by application name. Optional.
    #[serde(rename = "applicationName__contains", skip_serializing_if = "Option::is_none")]
    pub application_name_contains: Option<String>,
    /// Found or Not found in Application Inventory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_inventory: Option<bool>,
    /// Mode type. Allowed values: `all`, `suppression`, `agent_interoperability`,
    /// `binary_vault`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_type: Option<String>,
    /// Child process. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_process: Option<bool>,
    /// Threat type in. Allowed item values: `EDR`, `IDR`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_type: Option<String>,
    /// Engine in. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<String>,
    /// Interaction level in. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_level: Option<String>,
    /// Conditions in. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<String>,
    /// Exclusion name like any. Optional.
    #[serde(rename = "exclusionName__contains", skip_serializing_if = "Option::is_none")]
    pub exclusion_name_contains: Option<String>,
    /// Not recommended in. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_recommended: Option<String>,
    /// Scope path like any. Optional.
    #[serde(rename = "scopePath__contains", skip_serializing_if = "Option::is_none")]
    pub scope_path_contains: Option<String>,
}

impl ExportExclusionsQuery {
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
    /// Unified.
    pub fn unified(mut self, v: bool) -> Self {
        self.unified = Some(v);
        self
    }
    /// Value.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    /// Type. See struct docs for allowed values.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    /// A free-text search term.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// List of OS types to filter by.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// List of sources to filter by.
    pub fn source<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source = Some(join_csv(v));
        self
    }
    /// List of user ids to filter by.
    pub fn user_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_ids = Some(join_csv(v));
        self
    }
    /// List of recommendations to filter by.
    pub fn recommendations<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.recommendations = Some(join_csv(v));
        self
    }
    /// Types in.
    pub fn types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.types = Some(join_csv(v));
        self
    }
    /// List of modes to filter by (Path exclusions only).
    pub fn modes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.modes = Some(join_csv(v));
        self
    }
    /// List of excluded path types (Path exclusions only).
    pub fn path_exclusion_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.path_exclusion_types = Some(join_csv(v));
        self
    }
    /// Return filters from parent scope levels.
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Return filters from children scope levels.
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
    /// Filter by import status.
    pub fn imported<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.imported = Some(join_csv(v));
        self
    }
    /// Mode type.
    pub fn mode_type(mut self, v: impl Into<String>) -> Self {
        self.mode_type = Some(v.into());
        self
    }
    /// Child process.
    pub fn child_process(mut self, v: bool) -> Self {
        self.child_process = Some(v);
        self
    }
    /// Found or Not found in Application Inventory.
    pub fn in_app_inventory(mut self, v: bool) -> Self {
        self.in_app_inventory = Some(v);
        self
    }
    /// Threat type in.
    pub fn threat_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_type = Some(join_csv(v));
        self
    }
    /// Engine in.
    pub fn engines<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.engines = Some(join_csv(v));
        self
    }
    /// Interaction level in.
    pub fn interaction_level<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.interaction_level = Some(join_csv(v));
        self
    }
    /// Conditions in.
    pub fn conditions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.conditions = Some(join_csv(v));
        self
    }
    /// Free-text filter by value.
    pub fn value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.value_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by description.
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by user name.
    pub fn user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by application name.
    pub fn application_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name_contains = Some(join_csv(v));
        self
    }
    /// Exclusion name like any.
    pub fn exclusion_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.exclusion_name_contains = Some(join_csv(v));
        self
    }
    /// Not recommended in.
    pub fn not_recommended<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.not_recommended = Some(join_csv(v));
        self
    }
    /// Scope path like any.
    pub fn scope_path_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_path_contains = Some(join_csv(v));
        self
    }
}

/// Query params for `GET /web/api/v2.1/restrictions` — Get Blocklist.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestrictionsQuery {
    /// Skip first number of items (0-1000). Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items is returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, the total number of items is not calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `id`, `createdAt`,
    /// `updatedAt`, `osType`, `description`, `scope`, `value`, `userName`.
    /// Example: `id`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Comma-separated list of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Comma-separated list of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Comma-separated list of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Comma-separated list of IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Unified. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified: Option<bool>,
    /// Value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Created before this timestamp. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Created before or at this timestamp. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Created after this timestamp. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Created after or at this timestamp. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Date range for creation time. Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// Updated before this timestamp. Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at_lt: Option<String>,
    /// Updated before or at this timestamp. Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// Updated after this timestamp. Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at_gt: Option<String>,
    /// Updated after or at this timestamp. Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// Date range for update time. Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at_between: Option<String>,
    /// A free-text search term. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Type. Allowed values: `black_hash`. Example: `black_hash`. Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Comma-separated list of OS types. Allowed item values: `linux`, `macos`,
    /// `windows_legacy`, `windows`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Comma-separated list of sources. Allowed item values: `user`, `cloud`,
    /// `action_from_threat`, `catalog`, `performance_insight`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Comma-separated list of user ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<String>,
    /// Comma-separated list of recommendations. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<String>,
    /// Free-text filter by value. Optional.
    #[serde(rename = "value__contains", skip_serializing_if = "Option::is_none")]
    pub value_contains: Option<String>,
    /// Free-text filter by description. Optional.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// Free-text filter by user name. Optional.
    #[serde(rename = "user__contains", skip_serializing_if = "Option::is_none")]
    pub user_contains: Option<String>,
    /// Type in. Allowed item values: `black_hash`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// Return filters from parent scope levels. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// Filter by import status. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<String>,
    /// List of modes to filter by (Path exclusions only). Allowed item values:
    /// `suppress`, `suppress_dynamic_only`, `suppress_dfi_only`,
    /// `disable_in_process_monitor`, `disable_in_process_monitor_deep`,
    /// `disable_all_monitors`, `disable_all_monitors_deep`, `suppress_app_control`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modes: Option<String>,
}

impl RestrictionsQuery {
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
    /// If true, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, the total number of items is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Sort column. See struct docs for allowed values.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction: `asc` or `desc`.
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
    /// Unified.
    pub fn unified(mut self, v: bool) -> Self {
        self.unified = Some(v);
        self
    }
    /// Value.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    /// Created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Date range for creation time.
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// Updated before this timestamp.
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lt = Some(v.into());
        self
    }
    /// Updated before or at this timestamp.
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// Updated after this timestamp.
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gt = Some(v.into());
        self
    }
    /// Updated after or at this timestamp.
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// Date range for update time.
    pub fn updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.updated_at_between = Some(v.into());
        self
    }
    /// A free-text search term.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Type. Allowed values: `black_hash`.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    /// List of OS types to filter by.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// List of sources to filter by.
    pub fn source<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source = Some(join_csv(v));
        self
    }
    /// List of user ids to filter by.
    pub fn user_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_ids = Some(join_csv(v));
        self
    }
    /// List of recommendations to filter by.
    pub fn recommendations<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.recommendations = Some(join_csv(v));
        self
    }
    /// Free-text filter by value.
    pub fn value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.value_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by description.
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by user name.
    pub fn user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_contains = Some(join_csv(v));
        self
    }
    /// Type in. Allowed item values: `black_hash`.
    pub fn types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.types = Some(join_csv(v));
        self
    }
    /// Return filters from parent scope levels.
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Return filters from children scope levels.
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
    /// Filter by import status.
    pub fn imported<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.imported = Some(join_csv(v));
        self
    }
    /// List of modes to filter by (Path exclusions only).
    pub fn modes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.modes = Some(join_csv(v));
        self
    }
}

/// Query params for `GET /web/api/v2.1/export/restrictions` — Export Blocklist.
///
/// Filter set for the CSV export of Blocklist items (no pagination/sort
/// controls).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRestrictionsQuery {
    /// Comma-separated list of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Comma-separated list of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Comma-separated list of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Comma-separated list of IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Unified. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified: Option<bool>,
    /// Value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Created before this timestamp. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Created before or at this timestamp. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Created after this timestamp. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Created after or at this timestamp. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Date range for creation time. Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// Updated before this timestamp. Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at_lt: Option<String>,
    /// Updated before or at this timestamp. Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// Updated after this timestamp. Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at_gt: Option<String>,
    /// Updated after or at this timestamp. Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// Date range for update time. Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at_between: Option<String>,
    /// A free-text search term. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Type. Allowed values: `black_hash`. Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// Comma-separated list of OS types. Allowed item values: `linux`, `macos`,
    /// `windows_legacy`, `windows`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Comma-separated list of sources. Allowed item values: `user`, `cloud`,
    /// `action_from_threat`, `catalog`, `performance_insight`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Comma-separated list of user ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<String>,
    /// Comma-separated list of recommendations. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<String>,
    /// Free-text filter by value. Optional.
    #[serde(rename = "value__contains", skip_serializing_if = "Option::is_none")]
    pub value_contains: Option<String>,
    /// Free-text filter by description. Optional.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// Free-text filter by user name. Optional.
    #[serde(rename = "user__contains", skip_serializing_if = "Option::is_none")]
    pub user_contains: Option<String>,
    /// Type in. Allowed item values: `black_hash`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// Return filters from parent scope levels. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// Filter by import status. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<String>,
}

impl ExportRestrictionsQuery {
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
    /// Unified.
    pub fn unified(mut self, v: bool) -> Self {
        self.unified = Some(v);
        self
    }
    /// Value.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    /// Type. Allowed values: `black_hash`.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    /// A free-text search term.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// List of OS types to filter by.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// List of sources to filter by.
    pub fn source<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source = Some(join_csv(v));
        self
    }
    /// List of user ids to filter by.
    pub fn user_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_ids = Some(join_csv(v));
        self
    }
    /// List of recommendations to filter by.
    pub fn recommendations<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.recommendations = Some(join_csv(v));
        self
    }
    /// Types in. Allowed item values: `black_hash`.
    pub fn types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.types = Some(join_csv(v));
        self
    }
    /// Free-text filter by value.
    pub fn value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.value_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by description.
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by user name.
    pub fn user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_contains = Some(join_csv(v));
        self
    }
    /// Return filters from parent scope levels.
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Return filters from children scope levels.
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
    /// Filter by import status.
    pub fn imported<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.imported = Some(join_csv(v));
        self
    }
}

// ---------------------------------------------------------------------------
// Request bodies
// ---------------------------------------------------------------------------

/// Body for `DELETE /web/api/v2.1/exclusions` — Delete Exclusions.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteExclusionBody {
    /// Data. Required. Freeform object identifying the exclusion(s) to delete
    /// (e.g. `{ "ids": ["..."] }`).
    pub data: serde_json::Value,
}

/// Body for `POST /web/api/v2.1/exclusions` — Create Exclusion.
#[derive(Debug, Clone, Serialize)]
pub struct PostExclusionBody {
    /// Data. Required. Freeform exclusion definition.
    pub data: serde_json::Value,
    /// Filter. Required. Freeform scope selector (e.g. `{ "tenant": true }`,
    /// `{ "accountIds": ["..."] }`, `{ "siteIds": ["..."] }`,
    /// `{ "groupIds": ["..."] }`).
    pub filter: serde_json::Value,
}

/// Body for `PUT /web/api/v2.1/exclusions` — Update Exclusions.
#[derive(Debug, Clone, Serialize)]
pub struct PutExclusionBody {
    /// Data. Required. Freeform object with the updated exclusion fields.
    pub data: serde_json::Value,
}

/// Body for `POST /web/api/v2.1/exclusions/validate` — Validate Exclusion Item.
#[derive(Debug, Clone, Serialize)]
pub struct ValidateExclusionBody {
    /// Data. Required. Freeform exclusion definition to validate.
    pub data: serde_json::Value,
}

/// Body for `DELETE /web/api/v2.1/restrictions` — Delete Blocklist Item.
#[derive(Debug, Clone, Serialize)]
pub struct DeleteRestrictionBody {
    /// Data. Required. Freeform object identifying the blocklist item(s) to delete.
    pub data: serde_json::Value,
}

/// Body for `POST /web/api/v2.1/restrictions` — Create Blocklist Item.
#[derive(Debug, Clone, Serialize)]
pub struct PostRestrictionBody {
    /// Data. Required. Freeform blocklist item definition. The `type` must be
    /// `black_hash`.
    pub data: serde_json::Value,
    /// Filter. Required. Freeform scope selector.
    pub filter: serde_json::Value,
}

/// Body for `PUT /web/api/v2.1/restrictions` — Update Blocklist Item.
#[derive(Debug, Clone, Serialize)]
pub struct PutRestrictionBody {
    /// Data. Required. Freeform object with the updated blocklist item fields.
    pub data: serde_json::Value,
    /// Filter. Optional freeform scope selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

/// Body for `POST /web/api/v2.1/restrictions/validate` — Validate Blocklist Item.
#[derive(Debug, Clone, Serialize)]
pub struct ValidateRestrictionBody {
    /// Data. Required. Freeform blocklist item definition to validate.
    pub data: serde_json::Value,
    /// Filter. Optional freeform scope selector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl ExclusionsAndBlocklistService<'_> {
    /// `GET /web/api/v2.1/exclusions` — Get Exclusions.
    ///
    /// Get a list of all the Exclusions that match the filter. Note: to filter
    /// the results for a scope: Global - make sure `tenant` is `true` and no
    /// other scope ID is given; Account - make sure `tenant` is `false` and at
    /// least one Account ID is given; Site - make sure `tenant` is `false` and
    /// at least one Site ID is given.
    pub async fn list_exclusions(
        &self,
        query: &ExclusionsQuery,
    ) -> Result<Paginated<Exclusion>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/exclusions", q).await?)
    }

    /// `POST /web/api/v2.1/exclusions` — Create Exclusion.
    ///
    /// Create Exclusions to make your Agents suppress alerts and mitigation for
    /// items that you consider to be benign or which you require for
    /// interoperability. IMPORTANT! Every Exclusion is a possible security hole.
    /// Do not create Exclusions unless you are sure this hash, path, certificate
    /// signer, file type, or browser is always benign. When you create an
    /// Exclusion, make sure you set the filter to the smallest possible scope.
    pub async fn create_exclusion(
        &self,
        body: &PostExclusionBody,
    ) -> Result<Response<Vec<Exclusion>>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/exclusions", body)
            .await?)
    }

    /// `PUT /web/api/v2.1/exclusions` — Update Exclusions.
    ///
    /// Change the properties of an Exclusion through the data fields. To get the
    /// original data, run "exclusions" with a filter to give the item you want.
    pub async fn update_exclusions(
        &self,
        body: &PutExclusionBody,
    ) -> Result<Response<Vec<Exclusion>>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<PutExclusionBody, Response<Vec<Exclusion>>>(
                Method::PUT,
                "/web/api/v2.1/exclusions",
                None,
                Some(body),
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/exclusions` — Delete Exclusions.
    ///
    /// Every Exclusion opens a possible security hole. If you decide that an
    /// Exclusion (or multiple Exclusions) is not required, use this command to
    /// delete it. To get the ID of the Exclusion to delete, run the "exclusions"
    /// command.
    pub async fn delete_exclusions(
        &self,
        body: &DeleteExclusionBody,
    ) -> Result<Response<AffectedResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteExclusionBody, Response<AffectedResult>>(
                Method::DELETE,
                "/web/api/v2.1/exclusions",
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/exclusions/import` — Import Exclusions.
    ///
    /// Upload a CSV file that contains exclusion entries to import to a scope in
    /// your Management.
    ///
    /// This endpoint expects a `multipart/form-data` upload with a `filter`
    /// field (scope JSON; optional) and a required `file` field (the input CSV).
    /// Multipart uploads are not modeled by the JSON transport, so the body is
    /// passed through as a freeform [`serde_json::Value`].
    pub async fn import_exclusions(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<ImportResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/exclusions/import", body)
            .await?)
    }

    /// `GET /web/api/v2.1/exclusions/report/{report_id}` — Get Exclusion Import
    /// Validation Report.
    ///
    /// Get the Validation Report generated for the import to help you fix entries
    /// that did not import successfully.
    ///
    /// `report_id`: The ID of the requested Validation Report. Example:
    /// "225494730938493804".
    pub async fn get_exclusion_import_report(
        &self,
        report_id: impl Into<String>,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/exclusions/report/{}", report_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `POST /web/api/v2.1/exclusions/validate` — Validate Exclusion Item.
    ///
    /// Check if an exclusion is on the list of SentinelOne items that are "Not
    /// Allowed" or "Not Recommended". Returns one of: Not Recommended, Not
    /// Allowed, or None.
    pub async fn validate_exclusion(
        &self,
        body: &ValidateExclusionBody,
    ) -> Result<Response<ValidateResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/exclusions/validate", body)
            .await?)
    }

    /// `GET /web/api/v2.1/export/exclusions` — Export Exclusions.
    ///
    /// Get a CSV of all the items in the Exclusions that match the filter. Note:
    /// to see items from the Global Exclusion scope, make sure `tenant` is `true`
    /// and no other scope ID is given.
    ///
    /// The response is a CSV stream; it is returned as a freeform
    /// [`serde_json::Value`] envelope.
    pub async fn export_exclusions(
        &self,
        query: &ExportExclusionsQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/export/exclusions", q).await?)
    }

    /// `GET /web/api/v2.1/restrictions` — Get Blocklist.
    ///
    /// Get a list of all the items in the Blocklist that match the filter. To
    /// filter the results for a scope: Global - make sure `tenant` is `true` and
    /// no other scope ID is given; Account - make sure `tenant` is `false` and at
    /// least one Account ID is given; Site - make sure `tenant` is `false` and at
    /// least one Site ID is given.
    pub async fn list_restrictions(
        &self,
        query: &RestrictionsQuery,
    ) -> Result<Paginated<Restriction>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/restrictions", q).await?)
    }

    /// `POST /web/api/v2.1/restrictions` — Create Blocklist Item.
    ///
    /// Create a blocklist item for a SHA1 or SHA256 hash or both, for the scopes
    /// you enter in the filter fields. IMPORTANT: the type must be `black_hash` -
    /// any other value will create an Exclusion rather than a Blocklist item.
    /// Users with the IT role do not have permissions to run this.
    pub async fn create_restriction(
        &self,
        body: &PostRestrictionBody,
    ) -> Result<Response<Vec<Restriction>>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/restrictions", body)
            .await?)
    }

    /// `PUT /web/api/v2.1/restrictions` — Update Blocklist Item.
    ///
    /// Change the properties of a Blocklist item through the data fields. To get
    /// the original data, run "restrictions" with a filter to give the item you
    /// want.
    pub async fn update_restriction(
        &self,
        body: &PutRestrictionBody,
    ) -> Result<Response<Vec<Restriction>>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<PutRestrictionBody, Response<Vec<Restriction>>>(
                Method::PUT,
                "/web/api/v2.1/restrictions",
                None,
                Some(body),
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/restrictions` — Delete Blocklist Item.
    ///
    /// Agents immediately identify files on the blocklist and block them from
    /// executing. If there is a conflict you can delete the hash from the
    /// Blocklist. Users with the IT role do not have permissions to run this
    /// command.
    pub async fn delete_restriction(
        &self,
        body: &DeleteRestrictionBody,
    ) -> Result<Response<AffectedResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteRestrictionBody, Response<AffectedResult>>(
                Method::DELETE,
                "/web/api/v2.1/restrictions",
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/restrictions/import` — Import Blocklist Items.
    ///
    /// Upload a CSV file that contains blocklist entries to import to a scope in
    /// your Management.
    ///
    /// This endpoint expects a `multipart/form-data` upload with a `filter`
    /// field (scope JSON; optional) and a required `file` field (the input CSV).
    /// Multipart uploads are not modeled by the JSON transport, so the body is
    /// passed through as a freeform [`serde_json::Value`].
    pub async fn import_restrictions(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<ImportResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/restrictions/import", body)
            .await?)
    }

    /// `GET /web/api/v2.1/restrictions/report/{report_id}` — Get Blocklist Import
    /// Validation Report.
    ///
    /// Get the Validation Report generated for the import to help you fix entries
    /// that did not import successfully.
    ///
    /// `report_id`: The ID of the requested Validation Report. Example:
    /// "225494730938493804".
    pub async fn get_restriction_import_report(
        &self,
        report_id: impl Into<String>,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/restrictions/report/{}", report_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `POST /web/api/v2.1/restrictions/validate` — Validate Blocklist Item.
    ///
    /// Check if a hash is on the list of SentinelOne items that are "Not Allowed"
    /// or "Not Recommended". Returns one of: Not Recommended, Not Allowed, or
    /// None.
    pub async fn validate_restriction(
        &self,
        body: &ValidateRestrictionBody,
    ) -> Result<Response<ValidateResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/restrictions/validate", body)
            .await?)
    }

    /// `GET /web/api/v2.1/export/restrictions` — Export Blocklist.
    ///
    /// Get a CSV of all the items in the Blocklist that match the filter. Note:
    /// to see items from the Global Blocklist, make sure `tenant` is `true` and
    /// no other scope ID is given.
    ///
    /// The response is a CSV stream; it is returned as a freeform
    /// [`serde_json::Value`] envelope.
    pub async fn export_restrictions(
        &self,
        query: &ExportRestrictionsQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/export/restrictions", q).await?)
    }
}
