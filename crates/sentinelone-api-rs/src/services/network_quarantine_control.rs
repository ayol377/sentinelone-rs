//! `Network Quarantine Control` tag — Network Quarantine related endpoints.
//!
//! These endpoints share the Firewall Control endpoint family and are addressed
//! through the `{firewall_rule_category}` path segment. To affect Network
//! Quarantine, pass `network-quarantine` as the `firewall_rule_category`
//! argument.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::network_quarantine_control::*;
use crate::pagination::{Paginated, Response};

/// `Network Quarantine Control` tag — manage Network Quarantine (and Firewall
/// Control) rules (create, delete, reorder, enable/disable), rule tags, scope
/// configuration, import / export, and the list of usable protocols.
///
/// Every endpoint takes a `firewall_rule_category` path argument; pass
/// `network-quarantine` to affect Network Quarantine.
pub struct NetworkQuarantineControlService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Join an iterator of stringy values with commas, as the API expects for array
/// query params.
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

// ===========================================================================
// Query params
// ===========================================================================

/// Query params for
/// `GET /web/api/v2.1/firewall-control/{firewall_rule_category}` — Get Firewall
/// Rules.
///
/// At least one of `actions`, `statuses`, `os_types`, `name`, or a scope ID is
/// required by the API. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFirewallRulesQuery {
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
    /// If true, only the total number of items is returned, without any actual
    /// objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, the total number of items is not calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `action`, `status`, `order`. Optional.
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
    /// Free text search on name, tag, application, protocol. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return only firewall rules in these scopes (comma-joined). Each value is
    /// one of: `global`, `group`, `account`, `site`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    /// Return firewall rules with the filtered firewall class (comma-joined).
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<String>,
    /// Return firewall rules with the filtered name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Return firewall rules with the filtered protocols (comma-joined).
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<String>,
    /// Return firewall rules with the filtered os_type (comma-joined). Each
    /// value is one of: `linux`, `macos`, `windows_legacy`, `windows`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Return firewall rules with the filtered directions (comma-joined). Each
    /// value is one of: `any`, `inbound`, `outbound`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directions: Option<String>,
    /// Return firewall rules with the filtered action (comma-joined). Each
    /// value is one of: `Allow`, `Block`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    /// Return firewall rules with the filtered status (comma-joined). Each
    /// value is one of: `Enabled`, `Disabled`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    /// Return firewall rules created before this timestamp. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Return firewall rules created after this timestamp. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Return firewall rules created before or at this timestamp. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Return firewall rules created after or at this timestamp. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Return firewall rules created within this range (inclusive), e.g.
    /// `1514978764288-1514978999999`. Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// List of ids to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Filter by associated locations (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<String>,
    /// Filter by associated tags (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<String>,
    /// Free-text filter by the rule name (supports multiple values,
    /// comma-joined). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by the tag name (supports multiple values,
    /// comma-joined). Optional.
    #[serde(rename = "tagName__contains", skip_serializing_if = "Option::is_none")]
    pub tag_name_contains: Option<String>,
    /// Free-text filter by application (supports multiple values, comma-joined).
    /// Optional.
    #[serde(rename = "application__contains", skip_serializing_if = "Option::is_none")]
    pub application_contains: Option<String>,
    /// Free-text filter by protocol (supports multiple values, comma-joined).
    /// Optional.
    #[serde(rename = "protocol__contains", skip_serializing_if = "Option::is_none")]
    pub protocol_contains: Option<String>,
    /// Free-text filter by service (supports multiple values, comma-joined).
    /// Optional.
    #[serde(rename = "service__contains", skip_serializing_if = "Option::is_none")]
    pub service_contains: Option<String>,
    /// If true, all rules for the requested scope are returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_pagination: Option<bool>,
}

impl GetFirewallRulesQuery {
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
    /// The column to sort by. Allowed values: `id`, `name`, `action`,
    /// `status`, `order`.
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
    /// Free text search on name, tag, application, protocol.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Return only firewall rules in these scopes. Each value is one of:
    /// `global`, `group`, `account`, `site`.
    pub fn scopes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scopes = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered firewall class.
    pub fn applications<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.applications = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// Return firewall rules with the filtered protocols.
    pub fn protocols<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.protocols = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered os_type. Each value is one of:
    /// `linux`, `macos`, `windows_legacy`, `windows`.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered directions. Each value is one
    /// of: `any`, `inbound`, `outbound`.
    pub fn directions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.directions = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered action. Each value is one of:
    /// `Allow`, `Block`.
    pub fn actions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.actions = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered status. Each value is one of:
    /// `Enabled`, `Disabled`.
    pub fn statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.statuses = Some(join_csv(v));
        self
    }
    /// Return firewall rules created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Return firewall rules created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Return firewall rules created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Return firewall rules created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Return firewall rules created within this range (inclusive).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// List of ids to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// Filter by associated locations.
    pub fn location_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids = Some(join_csv(v));
        self
    }
    /// Filter by associated tags.
    pub fn tag_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tag_ids = Some(join_csv(v));
        self
    }
    /// Free-text filter by the rule name (supports multiple values).
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the tag name (supports multiple values).
    pub fn tag_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tag_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by application (supports multiple values).
    pub fn application_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by protocol (supports multiple values).
    pub fn protocol_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.protocol_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by service (supports multiple values).
    pub fn service_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.service_contains = Some(join_csv(v));
        self
    }
    /// If true, all rules for the requested scope are returned.
    pub fn disable_pagination(mut self, v: bool) -> Self {
        self.disable_pagination = Some(v);
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/firewall-control/{firewall_rule_category}/configuration`
/// — Get Configuration.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationQuery {
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
}

impl GetConfigurationQuery {
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
}

/// Query params for
/// `GET /web/api/v2.1/firewall-control/{firewall_rule_category}/export` —
/// Export Rules.
///
/// Same filter set as [`GetFirewallRulesQuery`] minus the pagination/sort
/// controls. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportRulesQuery {
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
    /// Free text search on name, tag, application, protocol. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return only firewall rules in these scopes (comma-joined). Each value is
    /// one of: `global`, `group`, `account`, `site`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    /// Return firewall rules with the filtered firewall class (comma-joined).
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<String>,
    /// Return firewall rules with the filtered name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Return firewall rules with the filtered protocols (comma-joined).
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocols: Option<String>,
    /// Return firewall rules with the filtered os_type (comma-joined). Each
    /// value is one of: `linux`, `macos`, `windows_legacy`, `windows`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Return firewall rules with the filtered directions (comma-joined). Each
    /// value is one of: `any`, `inbound`, `outbound`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directions: Option<String>,
    /// Return firewall rules with the filtered action (comma-joined). Each
    /// value is one of: `Allow`, `Block`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<String>,
    /// Return firewall rules with the filtered status (comma-joined). Each
    /// value is one of: `Enabled`, `Disabled`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<String>,
    /// Return firewall rules created before this timestamp. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Return firewall rules created after this timestamp. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Return firewall rules created before or at this timestamp. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Return firewall rules created after or at this timestamp. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Return firewall rules created within this range (inclusive). Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// List of ids to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Filter by associated locations (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<String>,
    /// Filter by associated tags (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<String>,
    /// Free-text filter by the rule name (supports multiple values,
    /// comma-joined). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by the tag name (supports multiple values,
    /// comma-joined). Optional.
    #[serde(rename = "tagName__contains", skip_serializing_if = "Option::is_none")]
    pub tag_name_contains: Option<String>,
    /// Free-text filter by application (supports multiple values, comma-joined).
    /// Optional.
    #[serde(rename = "application__contains", skip_serializing_if = "Option::is_none")]
    pub application_contains: Option<String>,
    /// Free-text filter by protocol (supports multiple values, comma-joined).
    /// Optional.
    #[serde(rename = "protocol__contains", skip_serializing_if = "Option::is_none")]
    pub protocol_contains: Option<String>,
    /// Free-text filter by service (supports multiple values, comma-joined).
    /// Optional.
    #[serde(rename = "service__contains", skip_serializing_if = "Option::is_none")]
    pub service_contains: Option<String>,
}

impl ExportRulesQuery {
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
    /// Free text search on name, tag, application, protocol.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Return only firewall rules in these scopes. Each value is one of:
    /// `global`, `group`, `account`, `site`.
    pub fn scopes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scopes = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered firewall class.
    pub fn applications<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.applications = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// Return firewall rules with the filtered protocols.
    pub fn protocols<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.protocols = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered os_type. Each value is one of:
    /// `linux`, `macos`, `windows_legacy`, `windows`.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered directions. Each value is one
    /// of: `any`, `inbound`, `outbound`.
    pub fn directions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.directions = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered action. Each value is one of:
    /// `Allow`, `Block`.
    pub fn actions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.actions = Some(join_csv(v));
        self
    }
    /// Return firewall rules with the filtered status. Each value is one of:
    /// `Enabled`, `Disabled`.
    pub fn statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.statuses = Some(join_csv(v));
        self
    }
    /// Return firewall rules created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Return firewall rules created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Return firewall rules created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Return firewall rules created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Return firewall rules created within this range (inclusive).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// List of ids to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// Filter by associated locations.
    pub fn location_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids = Some(join_csv(v));
        self
    }
    /// Filter by associated tags.
    pub fn tag_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tag_ids = Some(join_csv(v));
        self
    }
    /// Free-text filter by the rule name (supports multiple values).
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the tag name (supports multiple values).
    pub fn tag_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tag_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by application (supports multiple values).
    pub fn application_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by protocol (supports multiple values).
    pub fn protocol_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.protocol_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by service (supports multiple values).
    pub fn service_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.service_contains = Some(join_csv(v));
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/firewall-control/{firewall_rule_category}/protocols` —
/// Get Protocols.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProtocolsQuery {
    /// Skip first number of items (0-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
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
    /// The column to sort the results by. Allowed values: `name`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Full text search on protocols. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// If true, all rules for the requested scope are returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_pagination: Option<bool>,
}

impl GetProtocolsQuery {
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
    /// The column to sort by. Allowed values: `name`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Full text search on protocols.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// If true, all rules for the requested scope are returned.
    pub fn disable_pagination(mut self, v: bool) -> Self {
        self.disable_pagination = Some(v);
        self
    }
}

/// Form-data params for
/// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}/import` —
/// Import Rules.
///
/// NOTE: this endpoint is `multipart/form-data` (the `file` field is a binary
/// upload). The underlying HTTP client only supports JSON bodies, so the scope
/// selectors are sent as query params and the file/body must be supplied as a
/// freeform [`serde_json::Value`]. See [`NetworkQuarantineControlService::import`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportRulesQuery {
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
}

impl ImportRulesQuery {
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
}

// ===========================================================================
// Body params
//
// The Firewall Control / Network Quarantine request bodies share a
// `{ filter, data }` shape, where both `filter` and `data` are deeply-nested /
// polymorphic objects. Following the crate convention for freeform bodies,
// these are kept as `serde_json::Value`, with required-ness expressed by
// `Option<>` vs bare `T`.
// ===========================================================================

/// Request body for
/// `DELETE /web/api/v2.1/firewall-control/{firewall_rule_category}` — Delete
/// Rules.
///
/// Schema: `firewall_control.schemas_RuleDeleteSchema`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleDeleteBody {
    /// Filter selecting the rules to delete (freeform scope/filter object).
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl RuleDeleteBody {
    /// Set the filter (freeform scope/filter object).
    pub fn filter(mut self, v: serde_json::Value) -> Self {
        self.filter = Some(v);
        self
    }
}

/// Request body for
/// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}` — Create
/// Firewall Rule.
///
/// Schema: `firewall_control.schemas_PostFirewallSchema`. Both `data` and
/// `filter` are required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostFirewallBody {
    /// Rule definition (freeform). Required.
    pub data: serde_json::Value,
    /// Scope/filter selecting the target (freeform). Required.
    pub filter: serde_json::Value,
}

impl PostFirewallBody {
    /// Construct from the required `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for
/// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}/add-tags` and
/// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}/remove-tags` —
/// Add/Remove Rule Tags.
///
/// Schema: `firewall_control.schemas_ChangeRulesTagsSchema`. Both `data` and
/// `filter` are required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRulesTagsBody {
    /// Tag change definition (freeform). Required.
    pub data: serde_json::Value,
    /// Filter selecting the affected rules (freeform). Required.
    pub filter: serde_json::Value,
}

impl ChangeRulesTagsBody {
    /// Construct from the required `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for
/// `PUT /web/api/v2.1/firewall-control/{firewall_rule_category}/configuration` —
/// Update Configuration.
///
/// Schema: `firewall_control.schemas_PostFirewallSettingsSchema`. Both `data`
/// and `filter` are required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostFirewallSettingsBody {
    /// New configuration values (freeform). Required.
    pub data: serde_json::Value,
    /// Scope/filter selecting the target (freeform). Required.
    pub filter: serde_json::Value,
}

impl PostFirewallSettingsBody {
    /// Construct from the required `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for
/// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}/copy-rules`
/// (Copy Rules) and
/// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}/move-rules`
/// (Move Rules).
///
/// Schema: `firewall_control.schemas_CopyRuleSchema`. `data` (the list of
/// target scopes) is required; `filter` (the source selector) is optional.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyRuleBody {
    /// Target scope definitions (freeform array). Required.
    pub data: Vec<serde_json::Value>,
    /// Source selector / filter (freeform). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl CopyRuleBody {
    /// Construct from the required `data` array.
    pub fn new(data: Vec<serde_json::Value>) -> Self {
        Self { data, filter: None }
    }
    /// Set the source selector / filter.
    pub fn filter(mut self, v: serde_json::Value) -> Self {
        self.filter = Some(v);
        self
    }
}

/// Request body for
/// `PUT /web/api/v2.1/firewall-control/{firewall_rule_category}/enable` —
/// Enable/Disable Rules.
///
/// Schema: `firewall_control.schemas_EnableRuleSchema`. Both `data` and
/// `filter` are required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableRuleBody {
    /// Status change definition (freeform). Required.
    pub data: serde_json::Value,
    /// Filter selecting the affected rules (freeform). Required.
    pub filter: serde_json::Value,
}

impl EnableRuleBody {
    /// Construct from the required `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for
/// `PUT /web/api/v2.1/firewall-control/{firewall_rule_category}/reorder` —
/// Reorder Rules.
///
/// Schema: `firewall_control.schemas_ReorderSchema`. Both `data` (ordered rule
/// list) and `filter` are required.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderBody {
    /// Ordered rule definitions (freeform array). Required.
    pub data: Vec<serde_json::Value>,
    /// Scope/filter selecting the target (freeform). Required.
    pub filter: serde_json::Value,
}

impl ReorderBody {
    /// Construct from the required `data` array and `filter` object.
    pub fn new(data: Vec<serde_json::Value>, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for
/// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}/set-location` —
/// Set Location.
///
/// Schema: `firewall_control.schemas_SetLocationSchema`. `data` is required;
/// `filter` is optional.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLocationBody {
    /// Location attributes (freeform). Required.
    pub data: serde_json::Value,
    /// Filter selecting the affected rules (freeform). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl SetLocationBody {
    /// Construct from the required `data` object.
    pub fn new(data: serde_json::Value) -> Self {
        Self { data, filter: None }
    }
    /// Set the filter selecting the affected rules.
    pub fn filter(mut self, v: serde_json::Value) -> Self {
        self.filter = Some(v);
        self
    }
}

// ===========================================================================
// Service methods
// ===========================================================================

impl NetworkQuarantineControlService<'_> {
    /// `DELETE /web/api/v2.1/firewall-control/{firewall_rule_category}` — Delete
    /// Rules.
    ///
    /// Delete Firewall Control rules that match the filter.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn delete(
        &self,
        firewall_rule_category: impl Into<String>,
        body: &RuleDeleteBody,
    ) -> Result<Response<AffectedResult>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}",
            firewall_rule_category.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<RuleDeleteBody, Response<AffectedResult>>(
                Method::DELETE,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/firewall-control/{firewall_rule_category}` — Get
    /// Firewall Rules.
    ///
    /// Get the Firewall Control rules for a scope specified by ID (run
    /// "accounts", "sites", "groups", or set "tenant" to "true") that match the
    /// filter. The response is quite long because it includes all the rule
    /// properties, thus at least one of these filters is required: action,
    /// status, osType, name, or scope ID.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn list(
        &self,
        firewall_rule_category: impl Into<String>,
        query: &GetFirewallRulesQuery,
    ) -> Result<Paginated<NetworkQuarantineRule>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}",
            firewall_rule_category.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}` — Create
    /// Firewall Rule.
    ///
    /// Create a Firewall Control rule for a scope specified by ID (run
    /// "accounts", "sites", "groups", or set "tenant" to "true") and specific
    /// OS, to allow or block network traffic to matching endpoints. You can
    /// create one clean-up rule, with the Action of Allow or Block and with no
    /// other parameters defined explicitly. Make this the default rule at the
    /// end of your rule list. Traffic that does not match other rules first will
    /// match this rule. If you do not have a clean-up rule to match all traffic,
    /// the default Firewall Control behavior is to allow traffic that is not
    /// explicitly blocked. Firewall Control requires Control SKU.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn create(
        &self,
        firewall_rule_category: impl Into<String>,
        body: &PostFirewallBody,
    ) -> Result<Response<NetworkQuarantineRule>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}",
            firewall_rule_category.into()
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}/add-tags` —
    /// Add Rule Tags.
    ///
    /// Create a Firewall Rule tag. Create tags to represent Firewall policies -
    /// a set of rules in a specific order. After you create the tag, add rules
    /// to it. Notes: Tags apply to a scope and cannot be linked to rules from
    /// different scopes. Tags must be 2 to 256 characters.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn add_tags(
        &self,
        firewall_rule_category: impl Into<String>,
        body: &ChangeRulesTagsBody,
    ) -> Result<Response<AffectedResult>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/add-tags",
            firewall_rule_category.into()
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `GET
    /// /web/api/v2.1/firewall-control/{firewall_rule_category}/configuration` —
    /// Get Configuration.
    ///
    /// Get the Firewall Control configuration for a given scope. To filter the
    /// results for a scope: Global - make sure "tenant" is "true" and no other
    /// scope ID is given. Account - make sure "tenant" is "false" and at least
    /// one Account ID is given. Site - make sure "tenant" is "false" and at
    /// least one Site ID is given. The response shows if Firewall Control is
    /// enabled for the scope, if Location Awareness is enabled, the higher scope
    /// from which this scope inherited the configuration, and whether a lower
    /// scope inherits this configuration. Firewall Control requires Control SKU.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn get_configuration(
        &self,
        firewall_rule_category: impl Into<String>,
        query: &GetConfigurationQuery,
    ) -> Result<Response<NetworkQuarantineSettings>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/configuration",
            firewall_rule_category.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `PUT
    /// /web/api/v2.1/firewall-control/{firewall_rule_category}/configuration` —
    /// Update Configuration.
    ///
    /// Change the Firewall Control configuration for a given scope. To get the
    /// ID of a scope, run "accounts", "sites", or "groups". To change the Global
    /// configuration, leave the filters empty and set "tenant" to "true". In the
    /// Body, you can set if Firewall Control is enabled for the scope, if
    /// Location Awareness is enabled, the higher scope from which this scope
    /// inherits the configuration ("Global" or a scope ID), whether the lower
    /// scopes inherit this configuration, and whether blocked actions are
    /// reported. Firewall Control requires Control SKU.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn update_configuration(
        &self,
        firewall_rule_category: impl Into<String>,
        body: &PostFirewallSettingsBody,
    ) -> Result<Response<NetworkQuarantineSettings>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/configuration",
            firewall_rule_category.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<PostFirewallSettingsBody, Response<NetworkQuarantineSettings>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}/copy-rules`
    /// — Copy Rules.
    ///
    /// Copy a set of rules to other scopes. In the filter of the body, enter the
    /// properties to define the source. In the data field of the body, define
    /// the targets by ID. To get a scope ID, run "accounts", "sites", or
    /// "groups".
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn copy_rules(
        &self,
        firewall_rule_category: impl Into<String>,
        body: &CopyRuleBody,
    ) -> Result<Response<AffectedResult>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/copy-rules",
            firewall_rule_category.into()
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `PUT /web/api/v2.1/firewall-control/{firewall_rule_category}/enable` —
    /// Enable/Disable Rules.
    ///
    /// Change the status of a set of Firewall Control rules that match the
    /// filter to "Enabled" or "Disabled". In one request, you can set one status
    /// or the other.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn enable(
        &self,
        firewall_rule_category: impl Into<String>,
        body: &EnableRuleBody,
    ) -> Result<Response<AffectedResult>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/enable",
            firewall_rule_category.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<EnableRuleBody, Response<AffectedResult>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/firewall-control/{firewall_rule_category}/export` —
    /// Export Rules.
    ///
    /// Export Firewall Control rules that match the filter to a JSON file from a
    /// scope specified by ID (run "accounts", "sites", "groups", or leave the
    /// scope empty and set "tenant" to "true") and import them to another scope
    /// (with the "import" command). Firewall Control requires Control SKU.
    ///
    /// The endpoint returns a downloadable JSON file rather than the standard
    /// envelope, so the body is surfaced as a freeform [`serde_json::Value`].
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn export(
        &self,
        firewall_rule_category: impl Into<String>,
        query: &ExportRulesQuery,
    ) -> Result<serde_json::Value, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/export",
            firewall_rule_category.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}/import` —
    /// Import Rules.
    ///
    /// Import Firewall Control rules from an exported JSON file to scopes
    /// specified by ID (run "accounts", "sites", "groups", or leave the scope
    /// empty and set "tenant" to "true"). Firewall Control requires Control SKU,
    /// in the target and in the source.
    ///
    /// NOTE: the spec defines this as `multipart/form-data` with a required
    /// `file` upload. The underlying HTTP client only supports JSON bodies, so
    /// the scope selectors are passed as query params via [`ImportRulesQuery`]
    /// and the file/body must be supplied as a freeform [`serde_json::Value`]
    /// (e.g. the previously-exported rules JSON). This is an approximation of
    /// the multipart contract.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn import(
        &self,
        firewall_rule_category: impl Into<String>,
        query: &ImportRulesQuery,
        body: &serde_json::Value,
    ) -> Result<Response<SuccessResponse>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/import",
            firewall_rule_category.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<SuccessResponse>>(
                Method::POST,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/firewall-control/{firewall_rule_category}/move-rules`
    /// — Move Rules.
    ///
    /// Remove Firewall Rules, defined with the ID of the rules (run
    /// "firewall-control"), from scopes specified by ID (run "accounts",
    /// "sites", or "groups") and add the rules to the scope IDs in the data
    /// field. Firewall Control requires Control SKU.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn move_rules(
        &self,
        firewall_rule_category: impl Into<String>,
        body: &CopyRuleBody,
    ) -> Result<Response<AffectedResult>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/move-rules",
            firewall_rule_category.into()
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `GET /web/api/v2.1/firewall-control/{firewall_rule_category}/protocols` —
    /// Get Protocols.
    ///
    /// Get a list of protocols that can be used in Firewall Control rules.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn protocols(
        &self,
        firewall_rule_category: impl Into<String>,
        query: &GetProtocolsQuery,
    ) -> Result<Paginated<NetworkQuarantineProtocol>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/protocols",
            firewall_rule_category.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `POST
    /// /web/api/v2.1/firewall-control/{firewall_rule_category}/remove-tags` —
    /// Remove Rule Tags.
    ///
    /// Remove firewall tags from rules matching the filter. Tags represent
    /// Firewall policies - a set of rules in a specific order. When you remove a
    /// rule with a tag, all scopes that subscribe to the tag get the change.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn remove_tags(
        &self,
        firewall_rule_category: impl Into<String>,
        body: &ChangeRulesTagsBody,
    ) -> Result<Response<AffectedResult>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/remove-tags",
            firewall_rule_category.into()
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `PUT /web/api/v2.1/firewall-control/{firewall_rule_category}/reorder` —
    /// Reorder Rules.
    ///
    /// Change the order of rules for a scope specified by ID (run "accounts",
    /// "sites", or "groups"). The Agent looks at the rules based on their order
    /// in the Firewall Control policy, from the top to the bottom. First it goes
    /// through the Group rules, then the Site rules, then the Account rules,
    /// then the Global rules. When the Agent finds a rule that matches the
    /// parameters of the traffic, that rule is applied. The Agent does not
    /// continue to the lower rules in the list. Thus, the scope and the order of
    /// the rules is important. Firewall Control requires Control SKU.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn reorder(
        &self,
        firewall_rule_category: impl Into<String>,
        body: &ReorderBody,
    ) -> Result<Response<SuccessResponse>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/reorder",
            firewall_rule_category.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<ReorderBody, Response<SuccessResponse>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST
    /// /web/api/v2.1/firewall-control/{firewall_rule_category}/set-location` —
    /// Set Location.
    ///
    /// Set location attributes for a Location Aware Firewall Control rule. These
    /// rules are applied by Agents only if the network parameters of the
    /// endpoint match the properties of the location definition. To get a
    /// Location ID, run "locations". Firewall Control requires Control SKU.
    ///
    /// `firewall_rule_category`: to affect Network Quarantine use
    /// `network-quarantine`.
    pub async fn set_location(
        &self,
        firewall_rule_category: impl Into<String>,
        body: &SetLocationBody,
    ) -> Result<Response<AffectedResult>, Error> {
        let path = format!(
            "/web/api/v2.1/firewall-control/{}/set-location",
            firewall_rule_category.into()
        );
        Ok(self.client.http().post(&path, body).await?)
    }
}
