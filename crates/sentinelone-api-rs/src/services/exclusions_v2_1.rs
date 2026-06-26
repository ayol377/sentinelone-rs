//! `Exclusions v2.1` tag — unified-exclusion management APIs.
//!
//! Exclusions make Agents suppress alerts and mitigation for items considered
//! benign or required for interoperability. This module covers listing,
//! creating, updating, deleting, bulk-creating, computing available actions,
//! exporting and importing unified exclusions.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::exclusions_v2_1::*;
use crate::pagination::{Paginated, Response};

/// `Exclusions v2.1` tag — unified-exclusion management (list/create/update/
/// delete/bulk/available-actions/export/import).
pub struct ExclusionsV21Service<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ===========================================================================
// Query params
// ===========================================================================

/// Query params for `GET /web/api/v2.1/unified-exclusions` — Get Exclusions.
///
/// All fields are optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
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
    /// The column to sort the results by. Allowed values: `id`, `createdAt`,
    /// `updatedAt`, `source`, `description`, `osType`, `scope`, `userName`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of IDs to exclude from filter (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids_nin: Option<String>,
    /// Filter by the stable exclusion identifier. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_identifier: Option<String>,
    /// Created before this timestamp. Example: `2018-02-27T04:49:26.257525Z`.
    /// Optional.
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
    /// Optional.
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
    /// List of OS types to filter by (comma-joined). Allowed values: `linux`,
    /// `macos`, `windows_legacy`, `windows`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// List of sources to filter by (comma-joined). Allowed values: `user`,
    /// `cloud`, `action_from_threat`, `catalog`, `performance_insight`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// List of user IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<String>,
    /// List of Exclusion Type to filter by (comma-joined). Allowed values:
    /// `all`, `suppression`, `agent_interoperability`, `binary_vault`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_type: Option<String>,
    /// List of recommendations to filter by (comma-joined). Allowed values:
    /// `Not recommended`, `Not allowed`, `NONE`, `duplicated_value_sha1`,
    /// `duplicated_value_sha256`, `duplicated_value_sha1_sha256`, `Duplication`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_recommended: Option<String>,
    /// List of warnings to filter by (comma-joined). Allowed values:
    /// `Not allowed`, `Not recommended`, `Duplication`, `NONE`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<String>,
    /// List of excluded paths in an exclusion to filter by (comma-joined).
    /// Applies only to EDR exclusions of type path. Allowed values: `file`,
    /// `folder`, `subfolders`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_exclusion_types: Option<String>,
    /// List of threat types to filter by (comma-joined). Allowed values: `EDR`,
    /// `IDR`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_type: Option<String>,
    /// List of engines to filter by (comma-joined). Allowed values: `suppress`,
    /// `suppress_dfi_only`, `suppress_dynamic_only`, `suppress_app_control`,
    /// `suppress_drift_detection`, `ad_secure_ep`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<String>,
    /// List of interaction levels to filter by (comma-joined). Allowed values:
    /// `disable_all_monitors`, `disable_in_process_monitor`, `identity_only`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_level: Option<String>,
    /// Return filters from parent scope levels (Default: false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels (Default: false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// Indicates if the exclusion was imported by a bulk operation or not.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<bool>,
    /// List of conditions to filter by (comma-joined). Allowed values:
    /// `white_hash`, `path`, `certificate`, `file_type`, `browser`,
    /// `commandline`, `container_native`, `user`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<String>,
    /// Indicates if the exclusion applies to child processes or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_process: Option<bool>,
    /// Indicates if the exclusion is related to an application found in the
    /// scope's Application Inventory or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_inventory: Option<bool>,
    /// Free-text filter by exclusion name (comma-joined). Optional.
    #[serde(rename = "exclusionName__contains", skip_serializing_if = "Option::is_none")]
    pub exclusion_name_contains: Option<String>,
    /// Free-text filter by application name (comma-joined). Optional.
    #[serde(rename = "applicationName__contains", skip_serializing_if = "Option::is_none")]
    pub application_name_contains: Option<String>,
    /// Free-text filter by value (comma-joined). Optional.
    #[serde(rename = "value__contains", skip_serializing_if = "Option::is_none")]
    pub value_contains: Option<String>,
    /// Free-text filter by pod name (comma-joined). Optional.
    #[serde(rename = "podName__contains", skip_serializing_if = "Option::is_none")]
    pub pod_name_contains: Option<String>,
    /// Free-text filter by container name (comma-joined). Optional.
    #[serde(rename = "containerName__contains", skip_serializing_if = "Option::is_none")]
    pub container_name_contains: Option<String>,
    /// Free-text filter by namespace (comma-joined). Optional.
    #[serde(rename = "namespace__contains", skip_serializing_if = "Option::is_none")]
    pub namespace_contains: Option<String>,
    /// Free-text filter by SHA-256 of an image (comma-joined). Optional.
    #[serde(rename = "sha256Image__contains", skip_serializing_if = "Option::is_none")]
    pub sha256_image_contains: Option<String>,
    /// Free-text filter by full image URI (comma-joined). Optional.
    #[serde(rename = "fullImageName__contains", skip_serializing_if = "Option::is_none")]
    pub full_image_name_contains: Option<String>,
    /// Free-text filter by label key (comma-joined). Optional.
    #[serde(rename = "labelsKey__contains", skip_serializing_if = "Option::is_none")]
    pub labels_key_contains: Option<String>,
    /// Free-text filter by label value (comma-joined). Optional.
    #[serde(rename = "labelsValue__contains", skip_serializing_if = "Option::is_none")]
    pub labels_value_contains: Option<String>,
    /// Free-text filter by nested cmdline value (comma-joined). Optional.
    #[serde(rename = "cmdlineValue__contains", skip_serializing_if = "Option::is_none")]
    pub cmdline_value_contains: Option<String>,
    /// Free-text filter by nested path value (comma-joined). Optional.
    #[serde(rename = "pathValue__contains", skip_serializing_if = "Option::is_none")]
    pub path_value_contains: Option<String>,
    /// Free-text filter by description (comma-joined). Optional.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// Free-text filter by username (comma-joined). Optional.
    #[serde(rename = "user__contains", skip_serializing_if = "Option::is_none")]
    pub user_contains: Option<String>,
    /// Free-text filter by creator name (comma-joined). Optional.
    #[serde(rename = "creator__contains", skip_serializing_if = "Option::is_none")]
    pub creator_contains: Option<String>,
    /// Free-text filter by scope path (comma-joined). Optional.
    #[serde(rename = "scopePath__contains", skip_serializing_if = "Option::is_none")]
    pub scope_path_contains: Option<String>,
    /// Filter exclusions by their assigned tags. JSON where each key is a tag
    /// key and each value is a list of values; use `__nin` suffix on a key to
    /// filter by unassigned tag values. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// Include only Exclusions that have any tags assigned if true, or none if
    /// false. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_tags: Option<bool>,
    /// List of tag IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<String>,
    /// Comma-separated list of fields to be shown. Example:
    /// `threatType,interactionLevel`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// Group exclusions by their assigned tags (JSON). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_by_group: Option<String>,
    /// Include Exclusions that have any tags assigned if true, or none if false.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_tags_by_group: Option<bool>,
    /// Date range for last hit time (format: `<from>-<to>`, inclusive).
    /// Optional.
    #[serde(rename = "lastHit__between", skip_serializing_if = "Option::is_none")]
    pub last_hit_between: Option<String>,
    /// Inclusive range for number of hits within the past 30 days. Example:
    /// `6-9`. Optional.
    #[serde(rename = "hits30d__between", skip_serializing_if = "Option::is_none")]
    pub hits30d_between: Option<String>,
    /// Inclusive range for number of hits within the past 90 days. Example:
    /// `13-37`. Optional.
    #[serde(rename = "hits90d__between", skip_serializing_if = "Option::is_none")]
    pub hits90d_between: Option<String>,
    /// Inclusive range for all-time number of hits. Example: `678-9998212`.
    /// Optional.
    #[serde(rename = "hitsAllTime__between", skip_serializing_if = "Option::is_none")]
    pub hits_all_time_between: Option<String>,
}

/// Joins an iterator of stringy values into a comma-separated string.
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

impl ListQuery {
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
    /// The column to sort the results by. Allowed values: `id`, `createdAt`,
    /// `updatedAt`, `source`, `description`, `osType`, `scope`, `userName`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
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
    /// List of IDs to exclude from filter.
    pub fn ids_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids_nin = Some(join_csv(v));
        self
    }
    /// Filter by the stable exclusion identifier.
    pub fn exclusion_identifier(mut self, v: impl Into<String>) -> Self {
        self.exclusion_identifier = Some(v.into());
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
    /// Date range for creation time (format: `<from>-<to>`, inclusive).
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
    /// Date range for update time (format: `<from>-<to>`, inclusive).
    pub fn updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.updated_at_between = Some(v.into());
        self
    }
    /// List of OS types to filter by. Allowed values: `linux`, `macos`,
    /// `windows_legacy`, `windows`.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// List of sources to filter by. Allowed values: `user`, `cloud`,
    /// `action_from_threat`, `catalog`, `performance_insight`.
    pub fn source<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source = Some(join_csv(v));
        self
    }
    /// List of user IDs to filter by.
    pub fn user_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_ids = Some(join_csv(v));
        self
    }
    /// List of Exclusion Type to filter by. Allowed values: `all`,
    /// `suppression`, `agent_interoperability`, `binary_vault`.
    pub fn mode_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mode_type = Some(join_csv(v));
        self
    }
    /// List of recommendations to filter by. Allowed values: `Not recommended`,
    /// `Not allowed`, `NONE`, `duplicated_value_sha1`, `duplicated_value_sha256`,
    /// `duplicated_value_sha1_sha256`, `Duplication`.
    pub fn not_recommended<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.not_recommended = Some(join_csv(v));
        self
    }
    /// List of warnings to filter by. Allowed values: `Not allowed`,
    /// `Not recommended`, `Duplication`, `NONE`.
    pub fn warnings<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.warnings = Some(join_csv(v));
        self
    }
    /// List of excluded paths in an exclusion to filter by. Allowed values:
    /// `file`, `folder`, `subfolders`.
    pub fn path_exclusion_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.path_exclusion_types = Some(join_csv(v));
        self
    }
    /// List of threat types to filter by. Allowed values: `EDR`, `IDR`.
    pub fn threat_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_type = Some(join_csv(v));
        self
    }
    /// List of engines to filter by. Allowed values: `suppress`,
    /// `suppress_dfi_only`, `suppress_dynamic_only`, `suppress_app_control`,
    /// `suppress_drift_detection`, `ad_secure_ep`.
    pub fn engines<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.engines = Some(join_csv(v));
        self
    }
    /// List of interaction levels to filter by. Allowed values:
    /// `disable_all_monitors`, `disable_in_process_monitor`, `identity_only`.
    pub fn interaction_level<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.interaction_level = Some(join_csv(v));
        self
    }
    /// Return filters from parent scope levels (Default: false).
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Return filters from children scope levels (Default: false).
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
    /// Indicates if the exclusion was imported by a bulk operation or not.
    pub fn imported(mut self, v: bool) -> Self {
        self.imported = Some(v);
        self
    }
    /// List of conditions to filter by. Allowed values: `white_hash`, `path`,
    /// `certificate`, `file_type`, `browser`, `commandline`, `container_native`,
    /// `user`.
    pub fn conditions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.conditions = Some(join_csv(v));
        self
    }
    /// Indicates if the exclusion applies to child processes or not.
    pub fn child_process(mut self, v: bool) -> Self {
        self.child_process = Some(v);
        self
    }
    /// Indicates if the exclusion is related to an application in the scope's
    /// Application Inventory or not.
    pub fn in_app_inventory(mut self, v: bool) -> Self {
        self.in_app_inventory = Some(v);
        self
    }
    /// Free-text filter by exclusion name.
    pub fn exclusion_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.exclusion_name_contains = Some(join_csv(v));
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
    /// Free-text filter by value.
    pub fn value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.value_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by pod name.
    pub fn pod_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.pod_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by container name.
    pub fn container_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.container_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by namespace.
    pub fn namespace_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.namespace_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by SHA-256 of an image.
    pub fn sha256_image_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sha256_image_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by full image URI.
    pub fn full_image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.full_image_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by label key.
    pub fn labels_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.labels_key_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by label value.
    pub fn labels_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.labels_value_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by nested cmdline value.
    pub fn cmdline_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cmdline_value_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by nested path value.
    pub fn path_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.path_value_contains = Some(join_csv(v));
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
    /// Free-text filter by username.
    pub fn user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by creator name.
    pub fn creator_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.creator_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by scope path.
    pub fn scope_path_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_path_contains = Some(join_csv(v));
        self
    }
    /// Filter exclusions by their assigned tags (JSON).
    pub fn tags(mut self, v: impl Into<String>) -> Self {
        self.tags = Some(v.into());
        self
    }
    /// Include only Exclusions that have any tags assigned if true, or none if
    /// false.
    pub fn has_tags(mut self, v: bool) -> Self {
        self.has_tags = Some(v);
        self
    }
    /// List of tag IDs to filter by.
    pub fn tag_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tag_ids = Some(join_csv(v));
        self
    }
    /// Comma-separated list of fields to be shown.
    pub fn counts_for(mut self, v: impl Into<String>) -> Self {
        self.counts_for = Some(v.into());
        self
    }
    /// Group exclusions by their assigned tags (JSON).
    pub fn tags_by_group(mut self, v: impl Into<String>) -> Self {
        self.tags_by_group = Some(v.into());
        self
    }
    /// Include Exclusions that have any tags assigned if true, or none if false.
    pub fn has_tags_by_group(mut self, v: bool) -> Self {
        self.has_tags_by_group = Some(v);
        self
    }
    /// Date range for last hit time (format: `<from>-<to>`, inclusive).
    pub fn last_hit_between(mut self, v: impl Into<String>) -> Self {
        self.last_hit_between = Some(v.into());
        self
    }
    /// Inclusive range for number of hits within the past 30 days.
    pub fn hits30d_between(mut self, v: impl Into<String>) -> Self {
        self.hits30d_between = Some(v.into());
        self
    }
    /// Inclusive range for number of hits within the past 90 days.
    pub fn hits90d_between(mut self, v: impl Into<String>) -> Self {
        self.hits90d_between = Some(v.into());
        self
    }
    /// Inclusive range for all-time number of hits.
    pub fn hits_all_time_between(mut self, v: impl Into<String>) -> Self {
        self.hits_all_time_between = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/unified-exclusions/available-actions`
/// — Get Exclusion Actions.
///
/// The only fields unique to this endpoint are the required `create` flag and
/// the optional `selectAll` flag; every other filter/pagination parameter is
/// the same as the list endpoint and is supplied via a [`ListQuery`].
///
/// (`serde_urlencoded` does not support `#[serde(flatten)]`, so the two parts
/// are serialized separately and concatenated by the service method.)
#[derive(Debug, Default)]
pub struct AvailableActionsQuery {
    /// Create. Required.
    pub create: bool,
    /// When true, calculate actions for all exclusions matching filters (not
    /// just selected IDs). Optional.
    pub select_all: Option<bool>,
    /// Shared exclusion filter / pagination params. See [`ListQuery`] for the
    /// full documented field set.
    pub filters: ListQuery,
}

/// Serialized form of the extra (non-filter) available-actions params.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AvailableActionsExtra {
    create: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    select_all: Option<bool>,
}

impl AvailableActionsQuery {
    /// Construct with the required `create` flag set.
    pub fn new(create: bool) -> Self {
        Self {
            create,
            select_all: None,
            filters: ListQuery::default(),
        }
    }
    /// Set the required `create` flag.
    pub fn create(mut self, v: bool) -> Self {
        self.create = v;
        self
    }
    /// When true, calculate actions for all exclusions matching filters.
    pub fn select_all(mut self, v: bool) -> Self {
        self.select_all = Some(v);
        self
    }
    /// Set the shared exclusion filter / pagination params.
    pub fn filters(mut self, v: ListQuery) -> Self {
        self.filters = v;
        self
    }

    /// Build the full querystring (filters + the `create`/`selectAll` flags).
    fn to_query_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        let extra = AvailableActionsExtra {
            create: self.create,
            select_all: self.select_all,
        };
        let extra_qs = serde_urlencoded::to_string(&extra).unwrap_or_default();
        if !extra_qs.is_empty() {
            parts.push(extra_qs);
        }
        let filters_qs = serde_urlencoded::to_string(&self.filters).unwrap_or_default();
        if !filters_qs.is_empty() {
            parts.push(filters_qs);
        }
        parts.join("&")
    }
}

/// Query params for `GET /web/api/v2.1/unified-exclusions/export`
/// — Export Unified Exclusions.
///
/// The export endpoint accepts the same exclusion filter set as the list
/// endpoint (without pagination/sort controls). Populate it with the
/// [`ListQuery`] builders.
#[derive(Debug, Default)]
pub struct ExportQuery {
    /// Shared exclusion filter params. See [`ListQuery`] for the full
    /// documented field set.
    pub filters: ListQuery,
}

impl ExportQuery {
    /// Set the shared exclusion filter params.
    pub fn filters(mut self, v: ListQuery) -> Self {
        self.filters = v;
        self
    }

    /// Build the querystring from the filter params.
    fn to_query_string(&self) -> String {
        serde_urlencoded::to_string(&self.filters).unwrap_or_default()
    }
}

// ===========================================================================
// Body params
// ===========================================================================

/// A single exclusion reference for deletion (id + type).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteExclusionRef {
    /// Exclusion ID. Required.
    pub id: String,
    /// Exclusion type. Required. Allowed values: `white_hash`, `path`,
    /// `certificate`, `file_type`, `browser`, `commandline`,
    /// `container_native`, `idr`.
    #[serde(rename = "type")]
    pub r#type: String,
}

impl DeleteExclusionRef {
    /// Construct a delete reference from an id and type.
    pub fn new(id: impl Into<String>, r#type: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            r#type: r#type.into(),
        }
    }
}

/// Inner `data` object of [`DeleteBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteBodyData {
    /// Exclusions to delete. Required.
    pub exclusions: Vec<DeleteExclusionRef>,
}

/// Request body for `DELETE /web/api/v2.1/unified-exclusions`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteBody {
    /// Data. Required.
    pub data: DeleteBodyData,
}

impl DeleteBody {
    /// Construct a delete body from the exclusion references to delete.
    pub fn new(exclusions: Vec<DeleteExclusionRef>) -> Self {
        Self {
            data: DeleteBodyData { exclusions },
        }
    }
}

/// Scope filter for a bulk-create operation.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkFilter {
    /// The scope level of the exclusion. Required. Allowed values: `group`,
    /// `site`, `account`, `tenant`.
    pub scope_level: String,
    /// The identifier of the scope level. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level_id: Option<i64>,
}

impl BulkFilter {
    /// Construct with the required scope level.
    pub fn new(scope_level: impl Into<String>) -> Self {
        Self {
            scope_level: scope_level.into(),
            scope_level_id: None,
        }
    }
    /// Set the scope level identifier.
    pub fn scope_level_id(mut self, v: i64) -> Self {
        self.scope_level_id = Some(v);
        self
    }
}

/// Request body for `POST /web/api/v2.1/unified-exclusions/bulk`.
///
/// `data` is freeform (the spec leaves it open) and holds the exclusion(s) to
/// create; `filter` selects the target scope.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkBody {
    /// Data (the exclusion payload; freeform). Required.
    pub data: serde_json::Value,
    /// Scope filter. Required.
    pub filter: BulkFilter,
}

impl BulkBody {
    /// Construct a bulk body from the freeform `data` payload and a scope
    /// filter.
    pub fn new(data: serde_json::Value, filter: BulkFilter) -> Self {
        Self { data, filter }
    }
}

// ===========================================================================
// Service methods
// ===========================================================================

impl ExclusionsV21Service<'_> {
    /// `DELETE /web/api/v2.1/unified-exclusions` — Delete Exclusions.
    ///
    /// Delete the exclusions referenced (by id + type) in the request body.
    pub async fn delete(
        &self,
        body: &DeleteBody,
    ) -> Result<Response<UnifiedExclusionDeleteResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteBody, Response<UnifiedExclusionDeleteResult>>(
                Method::DELETE,
                "/web/api/v2.1/unified-exclusions",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/unified-exclusions` — Get Exclusions.
    ///
    /// Get a list of all the Exclusions that match the filter. Note: to filter
    /// the results for a scope: Global — make sure `tenant` is `true` and no
    /// other scope ID is given; Account — make sure `tenant` is `false` and at
    /// least one Account ID is given; Site — make sure `tenant` is `false` and
    /// at least one Site ID is given.
    pub async fn list(
        &self,
        query: &ListQuery,
    ) -> Result<Paginated<UnifiedExclusion>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/unified-exclusions", q)
            .await?)
    }

    /// `POST /web/api/v2.1/unified-exclusions` — Create Unified Exclusion.
    ///
    /// Create Exclusions to make your Agents suppress alerts and mitigation for
    /// items that you consider benign or which you require for interoperability.
    /// IMPORTANT! Every Exclusion is a possible security hole. Excluding by hash
    /// or path is much more secure than excluding all detections of a specific
    /// signer, file type, or browser. Set the filter to the smallest possible
    /// scope.
    ///
    /// The request body is the polymorphic create payload (`oneOf` IDR/EDR
    /// variants), so it is passed as freeform [`serde_json::Value`].
    pub async fn create(
        &self,
        body: &serde_json::Value,
    ) -> Result<Paginated<UnifiedExclusion>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/unified-exclusions", body)
            .await?)
    }

    /// `PUT /web/api/v2.1/unified-exclusions` — Update Exclusions.
    ///
    /// Change the properties of an Exclusion through the data fields. To get the
    /// original data, run the list endpoint with a filter to find the item you
    /// want.
    ///
    /// The request body is the polymorphic update payload (`oneOf` IDR/EDR
    /// variants), so it is passed as freeform [`serde_json::Value`].
    pub async fn update(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<UnifiedExclusion>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<UnifiedExclusion>>(
                Method::PUT,
                "/web/api/v2.1/unified-exclusions",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/unified-exclusions/available-actions` — Get Exclusion
    /// Actions.
    ///
    /// Get a list of available actions for exclusions that match the filter
    /// criteria. The `create` query flag is required.
    pub async fn available_actions(
        &self,
        query: &AvailableActionsQuery,
    ) -> Result<Response<UnifiedExclusionsActions>, Error> {
        let qs = query.to_query_string();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/unified-exclusions/available-actions", q)
            .await?)
    }

    /// `POST /web/api/v2.1/unified-exclusions/bulk` — Create Bulk Unified
    /// Exclusion.
    ///
    /// Create Bulk Exclusions to make your Agents suppress alerts and mitigation
    /// for items that you consider benign or which you require for
    /// interoperability. IMPORTANT! Every Exclusion is a possible security hole.
    /// Excluding by hash or path is much more secure than excluding all
    /// detections of a specific signer, file type, or browser. Set the filter to
    /// the smallest possible scope.
    pub async fn create_bulk(
        &self,
        body: &BulkBody,
    ) -> Result<Paginated<UnifiedExclusion>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/unified-exclusions/bulk", body)
            .await?)
    }

    /// `GET /web/api/v2.1/unified-exclusions/export` — Export Unified
    /// Exclusions.
    ///
    /// Export the currently filtered exclusions to a JSON file. You can use the
    /// export file to import the exclusions to a different scope.
    ///
    /// The endpoint returns a file payload (no documented JSON envelope), so the
    /// raw JSON is surfaced as a [`serde_json::Value`].
    pub async fn export(
        &self,
        query: &ExportQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = query.to_query_string();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/unified-exclusions/export", q)
            .await?)
    }

    /// `POST /web/api/v2.1/unified-exclusions/import` — Import Unified
    /// Exclusions.
    ///
    /// Import exclusions to a specified scope in the Console. Use an exclusion
    /// JSON file exported from a different scope; exported exclusion CSV files
    /// are also supported.
    ///
    /// This endpoint expects a `multipart/form-data` body with a `filter` field
    /// (scope JSON) and a `file` field (the JSON/CSV upload). The shared HTTP
    /// client sends JSON bodies only, so the multipart payload is passed as a
    /// freeform [`serde_json::Value`]; callers must build the appropriate
    /// multipart request when wiring this up.
    pub async fn import(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<ImportUnifiedExclusionsResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/unified-exclusions/import", body)
            .await?)
    }
}
