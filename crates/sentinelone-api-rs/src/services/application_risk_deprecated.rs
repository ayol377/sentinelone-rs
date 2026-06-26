//! Service for the `Application Risk (Deprecated)` tag.
//!
//! Installed applications and known vulnerabilities and exposures. Available
//! for Complete SKU only.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::application_risk_deprecated::{Application, Cve};
use crate::pagination::Paginated;

/// `Application Risk (Deprecated)` tag.
///
/// Installed applications and known vulnerabilities and exposures. Available
/// for Complete SKU only.
pub struct ApplicationRiskDeprecatedService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/installed-applications`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationsQuery {
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
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. One of: `id`, `installedAt`, `type`,
    /// `name`, `publisher`, `version`, `size`, `agentComputerName`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. One of: `asc`, `desc`. Optional.
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
    /// Filter by application IDs (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Filter by OS types (comma-separated). Each value one of: `linux`,
    /// `macos`, `windows_legacy`, `windows`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Filter not by OS types (comma-separated). Each value one of: `linux`,
    /// `macos`, `windows_legacy`, `windows`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types_nin: Option<String>,
    /// Filter by endpoint machine types (comma-separated). Each value one of:
    /// `unknown`, `desktop`, `laptop`, `server`, `kubernetes node`, `storage`,
    /// `kubernetes pod`, `ecs task`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_machine_types: Option<String>,
    /// Filter not by endpoint machine types (comma-separated). Each value one
    /// of: `unknown`, `desktop`, `laptop`, `server`, `kubernetes node`,
    /// `storage`, `kubernetes pod`, `ecs task`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_machine_types_nin: Option<String>,
    /// Filter by installation date range. Optional.
    #[serde(rename = "installedAt__between", skip_serializing_if = "Option::is_none")]
    pub installed_at__between: Option<String>,
    /// Filter by application types (comma-separated). Each value one of: `app`,
    /// `kb`, `patch`, `chromeExtension`, `edgeExtension`, `firefoxExtension`,
    /// `safariExtension`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// Filter not by application types (comma-separated). Each value one of:
    /// `app`, `kb`, `patch`, `chromeExtension`, `edgeExtension`,
    /// `firefoxExtension`, `safariExtension`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types_nin: Option<String>,
    /// Filter by risk (comma-separated). Each value one of: `none`, `low`,
    /// `medium`, `high`, `critical`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_levels: Option<String>,
    /// Filter not by risk (comma-separated). Each value one of: `none`, `low`,
    /// `medium`, `high`, `critical`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_levels_nin: Option<String>,
    /// Filter by application size range (bytes). Example: `1024-104856`. Optional.
    #[serde(rename = "size__between", skip_serializing_if = "Option::is_none")]
    pub size__between: Option<String>,
    /// Include active agents, decommissioned or both (comma-separated booleans).
    /// Example: `True,False`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_is_decommissioned: Option<String>,
    /// Free-text filter by application name (comma-separated, supports multiple
    /// values). Example: `calc`. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
    /// Free-text filter by application version (comma-separated, supports
    /// multiple values). Example: `1.22.333,build`. Optional.
    #[serde(rename = "version__contains", skip_serializing_if = "Option::is_none")]
    pub version__contains: Option<String>,
    /// Free-text filter by application publisher (comma-separated, supports
    /// multiple values). Example: `Sentinel`. Optional.
    #[serde(rename = "publisher__contains", skip_serializing_if = "Option::is_none")]
    pub publisher__contains: Option<String>,
    /// Free-text filter by computer name (comma-separated, supports multiple
    /// values). Example: `john-office,WIN`. Optional.
    #[serde(rename = "agentComputerName__contains", skip_serializing_if = "Option::is_none")]
    pub agent_computer_name__contains: Option<String>,
    /// Free-text filter by agent UUID (comma-separated, supports multiple
    /// values). Example: `e92-01928,b055`. Optional.
    #[serde(rename = "agentUuid__contains", skip_serializing_if = "Option::is_none")]
    pub agent_uuid__contains: Option<String>,
    /// Free-text filter by OS full name and version (comma-separated, supports
    /// multiple values). Example: `Service Pack 1`. Optional.
    #[serde(rename = "agentOsVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_os_version__contains: Option<String>,
}

impl ApplicationsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`.
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
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }
    /// If true, the total number of items is not calculated.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }
    /// The column to sort the results by. One of: `id`, `installedAt`, `type`,
    /// `name`, `publisher`, `version`, `size`, `agentComputerName`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. One of: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter by application IDs.
    pub fn ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.ids = Some(vals.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
    /// Filter by OS types. Each value one of: `linux`, `macos`,
    /// `windows_legacy`, `windows`.
    pub fn os_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.os_types = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter not by OS types. Each value one of: `linux`, `macos`,
    /// `windows_legacy`, `windows`.
    pub fn os_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.os_types_nin = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter by endpoint machine types. Each value one of: `unknown`,
    /// `desktop`, `laptop`, `server`, `kubernetes node`, `storage`,
    /// `kubernetes pod`, `ecs task`.
    pub fn agent_machine_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.agent_machine_types = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter not by endpoint machine types. Each value one of: `unknown`,
    /// `desktop`, `laptop`, `server`, `kubernetes node`, `storage`,
    /// `kubernetes pod`, `ecs task`.
    pub fn agent_machine_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.agent_machine_types_nin = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter by installation date range.
    pub fn installed_at_between(mut self, v: impl Into<String>) -> Self {
        self.installed_at__between = Some(v.into());
        self
    }
    /// Filter by application types. Each value one of: `app`, `kb`, `patch`,
    /// `chromeExtension`, `edgeExtension`, `firefoxExtension`, `safariExtension`.
    pub fn types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.types = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter not by application types. Each value one of: `app`, `kb`, `patch`,
    /// `chromeExtension`, `edgeExtension`, `firefoxExtension`, `safariExtension`.
    pub fn types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.types_nin = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter by risk. Each value one of: `none`, `low`, `medium`, `high`,
    /// `critical`.
    pub fn risk_levels<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.risk_levels = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter not by risk. Each value one of: `none`, `low`, `medium`, `high`,
    /// `critical`.
    pub fn risk_levels_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.risk_levels_nin = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter by application size range (bytes). Example: `1024-104856`.
    pub fn size_between(mut self, v: impl Into<String>) -> Self {
        self.size__between = Some(v.into());
        self
    }
    /// Include active agents, decommissioned or both. Example: `True,False`.
    pub fn agent_is_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.agent_is_decommissioned = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Free-text filter by application name (supports multiple values).
    pub fn name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.name__contains = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Free-text filter by application version (supports multiple values).
    pub fn version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.version__contains = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Free-text filter by application publisher (supports multiple values).
    pub fn publisher_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.publisher__contains = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Free-text filter by computer name (supports multiple values).
    pub fn agent_computer_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.agent_computer_name__contains = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Free-text filter by agent UUID (supports multiple values).
    pub fn agent_uuid_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.agent_uuid__contains = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Free-text filter by OS full name and version (supports multiple values).
    pub fn agent_os_version_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.agent_os_version__contains = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query params for `GET /web/api/v2.1/installed-applications/cves`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CvesQuery {
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
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. One of: `id`, `publishedAt`,
    /// `agentId`, `applicationId`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. One of: `asc`, `desc`. Optional.
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
    /// Filter by internal CVE IDs (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Filter by global CVE ids (comma-separated). Example:
    /// `CVE-2018-3182,CVE-2018-1087`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cve_ids: Option<String>,
    /// Filter by application IDs (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_ids: Option<String>,
    /// Created at lesser than. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at__lt: Option<String>,
    /// Created at greater than. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at__gt: Option<String>,
    /// Created at lesser or equal than. Example: `2018-02-27T04:49:26.257525Z`.
    /// Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at__lte: Option<String>,
    /// Created at greater or equal than. Example:
    /// `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at__gte: Option<String>,
    /// Updated at lesser than. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at__lt: Option<String>,
    /// Updated at greater than. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at__gt: Option<String>,
    /// Updated at lesser or equal than. Example: `2018-02-27T04:49:26.257525Z`.
    /// Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at__lte: Option<String>,
    /// Updated at greater or equal than. Example:
    /// `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at__gte: Option<String>,
}

impl CvesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`.
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
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }
    /// If true, the total number of items is not calculated.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }
    /// The column to sort the results by. One of: `id`, `publishedAt`,
    /// `agentId`, `applicationId`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. One of: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter by internal CVE IDs.
    pub fn ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.ids = Some(vals.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
    /// Filter by global CVE ids. Example: `CVE-2018-3182,CVE-2018-1087`.
    pub fn cve_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.cve_ids = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Filter by application IDs.
    pub fn application_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.application_ids = Some(
            vals.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Created at lesser than. Example: `2018-02-27T04:49:26.257525Z`.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at__lt = Some(v.into());
        self
    }
    /// Created at greater than. Example: `2018-02-27T04:49:26.257525Z`.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at__gt = Some(v.into());
        self
    }
    /// Created at lesser or equal than. Example: `2018-02-27T04:49:26.257525Z`.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at__lte = Some(v.into());
        self
    }
    /// Created at greater or equal than. Example:
    /// `2018-02-27T04:49:26.257525Z`.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at__gte = Some(v.into());
        self
    }
    /// Updated at lesser than. Example: `2018-02-27T04:49:26.257525Z`.
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lt = Some(v.into());
        self
    }
    /// Updated at greater than. Example: `2018-02-27T04:49:26.257525Z`.
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gt = Some(v.into());
        self
    }
    /// Updated at lesser or equal than. Example: `2018-02-27T04:49:26.257525Z`.
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lte = Some(v.into());
        self
    }
    /// Updated at greater or equal than. Example:
    /// `2018-02-27T04:49:26.257525Z`.
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gte = Some(v.into());
        self
    }
}

impl ApplicationRiskDeprecatedService<'_> {
    /// `GET /web/api/v2.1/installed-applications` — [DEPRECATED] Get Applications.
    ///
    /// Get the applications, and their data (such as risk level), installed on
    /// endpoints with Application Risk-enabled Agents that match the filter.
    /// SentinelOne Application Risk lets you monitor applications installed on
    /// endpoints. Applications not updated with the latest patches are
    /// vulnerable to exploits. With SentinelOne Application Risk you can see all
    /// applications to be patched, on all endpoints or on a specific endpoint.
    /// The Agent takes a snapshot of the endpoint application data and checks
    /// for vulnerabilities in the SentinelOne Cloud. When the Agent detects a
    /// change to the application data, it sends a diff to the Management.
    ///
    /// Application Risk requires Complete SKU. This feature is in EA. To join
    /// the EA program, contact your SentinelOne Sales Rep.
    pub async fn list_applications(
        &self,
        query: &ApplicationsQuery,
    ) -> Result<Paginated<Application>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/installed-applications", q)
            .await?)
    }

    /// `GET /web/api/v2.1/installed-applications/cves` — [DEPRECATED] Get CVEs.
    ///
    /// Get known CVEs for applications that are installed on endpoints with
    /// Application Risk-enabled Agents.
    ///
    /// Application Risk requires Complete SKU. This feature is in EA. To join
    /// the EA program, contact your SentinelOne Sales Rep.
    pub async fn list_cves(&self, query: &CvesQuery) -> Result<Paginated<Cve>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/installed-applications/cves", q)
            .await?)
    }
}
