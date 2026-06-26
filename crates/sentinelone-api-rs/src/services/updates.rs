//! Service for the `Updates` tag.
//!
//! File uploads related endpoints (Agent packages).

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::updates::{AffectedResult, LatestPackages, Package, SuccessResult};
use crate::pagination::{Paginated, Response};

/// `Updates` tag — file uploads related endpoints (Agent packages).
pub struct UpdatesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/update/agent/latest-packages`.
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestPackagesByOsQuery {
    /// List of Site IDs to filter by. Optional.
    /// Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Optional.
    /// Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Package type. Optional. Allowed values: `Agent`, `Ranger`,
    /// `AgentAndRanger`. Example: "Agent".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
}

impl LatestPackagesByOsQuery {
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
    /// Package type. Allowed values: `Agent`, `Ranger`, `AgentAndRanger`.
    pub fn package_type(mut self, v: impl Into<String>) -> Self {
        self.package_type = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/update/agent/packages` (Get Latest
/// Packages).
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use "cursor". Optional. Example: "150".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional. Example: "10".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional. Example: "YWdlbnRfaWQ6NTgwMjkzODE=".
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
    /// The column to sort the results by. Optional. Allowed values: `id`,
    /// `createdAt`, `updatedAt`, `osType`, `platformType`, `fileName`,
    /// `fileSize`, `fileExtension`. Example: "id".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`. Example: "asc".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Free-text filter by file name. Optional.
    #[serde(rename = "fileName__contains", skip_serializing_if = "Option::is_none")]
    pub file_name_contains: Option<String>,
    /// Free-text filter by file size. Optional.
    #[serde(rename = "fileSize__contains", skip_serializing_if = "Option::is_none")]
    pub file_size_contains: Option<String>,
    /// Free-text filter by version string. Optional.
    #[serde(rename = "versionStr__contains", skip_serializing_if = "Option::is_none")]
    pub version_str_contains: Option<String>,
    /// Free-text filter by Network Scanner version. Optional.
    #[serde(rename = "rangerVersion__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_version_contains: Option<String>,
    /// Free-text filter by SHA1 hash. Optional.
    #[serde(rename = "sha1__contains", skip_serializing_if = "Option::is_none")]
    pub sha1_contains: Option<String>,
    /// Free-text filter by site name. Optional.
    #[serde(rename = "siteName__contains", skip_serializing_if = "Option::is_none")]
    pub site_name_contains: Option<String>,
    /// Free-text filter by account name. Optional.
    #[serde(rename = "accountName__contains", skip_serializing_if = "Option::is_none")]
    pub account_name_contains: Option<String>,
    /// List of Site IDs to filter by. Optional.
    /// Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Optional.
    /// Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// A free-text search term, will match applicable attributes. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Package ID list. Optional.
    /// Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Os type in. Optional. Allowed values: `windows_legacy`,
    /// `threat_detection_netapp`, `windows`, `linux_k8s`, `threat_detection_s3`,
    /// `macos`, `sdk`, `linux`. Example: "windows_legacy".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Platform type. Optional. Allowed values: `windows_legacy`,
    /// `threat_detection_netapp`, `windows`, `linux_k8s`, `threat_detection_s3`,
    /// `macos`, `sdk`, `linux`. Example: "windows_legacy".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<String>,
    /// Platform type in. Optional. Allowed values: `windows_legacy`,
    /// `threat_detection_netapp`, `windows`, `linux_k8s`, `threat_detection_s3`,
    /// `macos`, `sdk`, `linux`. Example: "windows_legacy".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_types: Option<String>,
    /// Status in. Optional. Allowed values: `beta`, `ea`, `ga`, `other`.
    /// Example: "beta".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Package OS architecture (32/64 bit), applicable to Windows packages only.
    /// Optional. Allowed values: `64 bit`, `ARM64`, `32/64 bit`,
    /// `32 bit/64 bit/ARM64`, `32 bit`, `N/A`. Example: "64 bit".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_arches: Option<String>,
    /// File extension. Optional. Allowed values: `.msi`, `.exe`, `.deb`, `.rpm`,
    /// `.bsx`, `.pkg`, `.img`, `unknown`. Example: ".msi".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extension: Option<String>,
    /// File extension in. Optional. Allowed values: `.msi`, `.exe`, `.deb`,
    /// `.rpm`, `.bsx`, `.pkg`, `.img`, `unknown`. Example: ".msi".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_extensions: Option<String>,
    /// Agent version. Optional. Example: "2.5.1.1320".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Package major versions. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_versions: Option<String>,
    /// Package minor versions. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_versions: Option<String>,
    /// Package minor version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    /// Package hash. Optional.
    /// Example: "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha1: Option<String>,
    /// Network Scanner version. Optional. Example: "2.5.1.1320".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_version: Option<String>,
    /// Package type. Optional. Allowed values: `Agent`, `Ranger`,
    /// `AgentAndRanger`. Example: "Agent".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    /// Package type in. Optional. Allowed values: `Agent`, `Ranger`,
    /// `AgentAndRanger`. Example: "Agent".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_types: Option<String>,
    /// Agent os revision. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_revision: Option<String>,
}

impl PackagesQuery {
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
    /// The column to sort the results by. Allowed values: `id`, `createdAt`,
    /// `updatedAt`, `osType`, `platformType`, `fileName`, `fileSize`,
    /// `fileExtension`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Free-text filter by file name.
    pub fn file_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.file_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by file size.
    pub fn file_size_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.file_size_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by version string.
    pub fn version_str_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.version_str_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by Network Scanner version.
    pub fn ranger_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_version_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by SHA1 hash.
    pub fn sha1_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sha1_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by site name.
    pub fn site_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by account name.
    pub fn account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_name_contains = Some(join_csv(v));
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
    /// A free-text search term, will match applicable attributes.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Package ID list.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// Os type in. Allowed values: `windows_legacy`, `threat_detection_netapp`,
    /// `windows`, `linux_k8s`, `threat_detection_s3`, `macos`, `sdk`, `linux`.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// Platform type. Allowed values: `windows_legacy`,
    /// `threat_detection_netapp`, `windows`, `linux_k8s`, `threat_detection_s3`,
    /// `macos`, `sdk`, `linux`.
    pub fn platform_type(mut self, v: impl Into<String>) -> Self {
        self.platform_type = Some(v.into());
        self
    }
    /// Platform type in. Allowed values: `windows_legacy`,
    /// `threat_detection_netapp`, `windows`, `linux_k8s`, `threat_detection_s3`,
    /// `macos`, `sdk`, `linux`.
    pub fn platform_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.platform_types = Some(join_csv(v));
        self
    }
    /// Status in. Allowed values: `beta`, `ea`, `ga`, `other`.
    pub fn status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.status = Some(join_csv(v));
        self
    }
    /// Package OS architecture (32/64 bit), applicable to Windows packages only.
    /// Allowed values: `64 bit`, `ARM64`, `32/64 bit`, `32 bit/64 bit/ARM64`,
    /// `32 bit`, `N/A`.
    pub fn os_arches<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches = Some(join_csv(v));
        self
    }
    /// File extension. Allowed values: `.msi`, `.exe`, `.deb`, `.rpm`, `.bsx`,
    /// `.pkg`, `.img`, `unknown`.
    pub fn file_extension(mut self, v: impl Into<String>) -> Self {
        self.file_extension = Some(v.into());
        self
    }
    /// File extension in. Allowed values: `.msi`, `.exe`, `.deb`, `.rpm`,
    /// `.bsx`, `.pkg`, `.img`, `unknown`.
    pub fn file_extensions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.file_extensions = Some(join_csv(v));
        self
    }
    /// Agent version.
    pub fn version(mut self, v: impl Into<String>) -> Self {
        self.version = Some(v.into());
        self
    }
    /// Package major versions.
    pub fn major_versions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.major_versions = Some(join_csv(v));
        self
    }
    /// Package minor versions.
    pub fn minor_versions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.minor_versions = Some(join_csv(v));
        self
    }
    /// Package minor version.
    pub fn minor_version(mut self, v: impl Into<String>) -> Self {
        self.minor_version = Some(v.into());
        self
    }
    /// Package hash.
    pub fn sha1(mut self, v: impl Into<String>) -> Self {
        self.sha1 = Some(v.into());
        self
    }
    /// Network Scanner version.
    pub fn ranger_version(mut self, v: impl Into<String>) -> Self {
        self.ranger_version = Some(v.into());
        self
    }
    /// Package type. Allowed values: `Agent`, `Ranger`, `AgentAndRanger`.
    pub fn package_type(mut self, v: impl Into<String>) -> Self {
        self.package_type = Some(v.into());
        self
    }
    /// Package type in. Allowed values: `Agent`, `Ranger`, `AgentAndRanger`.
    pub fn package_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.package_types = Some(join_csv(v));
        self
    }
    /// Agent os revision.
    pub fn os_revision(mut self, v: impl Into<String>) -> Self {
        self.os_revision = Some(v.into());
        self
    }
}

/// Body for `DELETE /web/api/v2.1/update/agent/packages` (Delete Packages).
///
/// Wraps the required `data` object. Mirrors
/// `packages.schemas_DeletePackagesSchema`.
#[derive(Debug, Serialize)]
pub struct DeletePackagesBody {
    /// Data. Required.
    pub data: DeletePackagesData,
}

/// Inner `data` object for [`DeletePackagesBody`].
#[derive(Debug, Default, Serialize)]
pub struct DeletePackagesData {
    /// Package IDs to delete (max 5000). Required.
    /// Example: ["225494730938493804", "225494730938493915"].
    pub ids: Vec<String>,
}

/// Body for `PUT /web/api/v2.1/update/agent/packages/{package_id}` (Update
/// package).
///
/// Wraps the required `data` object. Mirrors
/// `packages.schemas_PutPackageSchema`.
#[derive(Debug, Serialize)]
pub struct PutPackageBody {
    /// Data. Required.
    pub data: PutPackageData,
}

/// Inner `data` object for [`PutPackageBody`]. Every field is optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutPackageData {
    /// List of sites to make the package available in. Applicable only if
    /// scopeLevel is set to "site" (max 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of accounts to make the package available in. Applicable only if
    /// scopeLevel is set to "account" (max 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Status. Optional. Allowed values: `beta`, `ea`, `ga`, `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Package scope. If "global", it will be available in all sites. If
    /// "site", it will be available only to sites specified in the "siteIds"
    /// attribute. Optional. Allowed values: `site`, `account`, `global`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Agent version. Optional. Example: "2.5.1.1320".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Package OS architecture (32/64 bit), applicable to Windows packages only.
    /// Optional. Allowed values: `64 bit`, `ARM64`, `32/64 bit`,
    /// `32 bit/64 bit/ARM64`, `32 bit`, `N/A`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<String>,
    /// Network Scanner version if applicable. Optional. Example: "2.5.1.1320".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_version: Option<String>,
    /// Minor version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    /// Supported os versions. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_os_versions: Option<String>,
}

/// Body for `POST /web/api/v2.1/upload/agent/software` (Upload Agent Package).
///
/// The endpoint is a `multipart/form-data` upload. The shared HTTP core only
/// serializes JSON bodies, so the binary `file` field is NOT carried here; this
/// struct models the form metadata fields only. Supply the file via a transport
/// that supports multipart if required.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadAgentSoftwareBody {
    /// List of sites to make the package available in. Applicable only if
    /// scopeLevel is set to "site". Optional.
    /// Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of accounts to make the package available in. Applicable only if
    /// scopeLevel is set to "account". Optional.
    /// Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Platform type. Optional. Allowed values: `windows_legacy`,
    /// `threat_detection_netapp`, `windows`, `linux_k8s`, `threat_detection_s3`,
    /// `macos`, `sdk`, `linux`. Example: "windows_legacy".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Platform type. Optional. Allowed values: `windows_legacy`,
    /// `threat_detection_netapp`, `windows`, `linux_k8s`, `threat_detection_s3`,
    /// `macos`, `sdk`, `linux`. Example: "windows_legacy".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_type: Option<String>,
    /// Version. Optional. Example: "2.5.1.1320".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Status. Required. Allowed values: `beta`, `ea`, `ga`, `other`.
    /// Example: "beta".
    pub status: String,
    /// Package minor version. Optional. Example: "SP1".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    /// Package scope. If "global", it will be available in all sites. Otherwise,
    /// it will only be available to the sites/accounts specified in
    /// "siteIds"/"accountIds" attribute. Optional. Allowed values: `site`,
    /// `account`, `global`. Example: "site".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
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

impl UpdatesService<'_> {
    /// `GET /web/api/v2.1/update/agent/download/{package_id}` — Download Agent
    /// Package.
    ///
    /// [DEPRECATED] Download an agent package by package ID. Rate limit: 2 calls
    /// per minute for each different user token.
    ///
    /// The endpoint streams a package file; with the JSON transport the body is
    /// surfaced as [`serde_json::Value`].
    ///
    /// `package_id`: Package ID. Required. Example: "225494730938493804".
    pub async fn download_agent_package(
        &self,
        package_id: impl Into<String>,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/update/agent/download/{}", package_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `GET /web/api/v2.1/update/agent/download/{site_id}/{package_id}` —
    /// Download Package.
    ///
    /// Download a package by site_id ("sites") and filename. Rate limit: 2 calls
    /// per minute for each user token. Use this command to manually deploy Agent
    /// updates that cannot be deployed with the update-software command (see
    /// Agent Actions > Update Software) or through the Console.
    ///
    /// The endpoint streams a package file; with the JSON transport the body is
    /// surfaced as [`serde_json::Value`].
    ///
    /// `site_id`: Site ID. Required. Example: "225494730938493804".
    /// `package_id`: Package ID. Required. Example: "225494730938493804".
    pub async fn download_package(
        &self,
        site_id: impl Into<String>,
        package_id: impl Into<String>,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/update/agent/download/{}/{}",
            site_id.into(),
            package_id.into()
        );
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `GET /web/api/v2.1/update/agent/latest-packages` — Latest Packages by OS.
    ///
    /// [DEPRECATED] Use "Latest packages" API call instead
    /// (`GET /web/api/v2.1/update/agent/packages`).
    pub async fn latest_packages_by_os(
        &self,
        query: &LatestPackagesByOsQuery,
    ) -> Result<Response<LatestPackages>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/update/agent/latest-packages", q)
            .await?)
    }

    /// `DELETE /web/api/v2.1/update/agent/packages` — Delete Packages.
    ///
    /// Delete Agent packages from your Management. Use the IDs from Get Latest
    /// Packages.
    pub async fn delete_packages(
        &self,
        body: &DeletePackagesBody,
    ) -> Result<Response<AffectedResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeletePackagesBody, Response<AffectedResult>>(
                Method::DELETE,
                "/web/api/v2.1/update/agent/packages",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/update/agent/packages` — Get Latest Packages.
    ///
    /// Get the Agent packages that are uploaded to your Management. The response
    /// shows the data of each package, including the IDs, which you can use in
    /// other commands.
    pub async fn list_packages(
        &self,
        query: &PackagesQuery,
    ) -> Result<Paginated<Package>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/update/agent/packages", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/update/agent/packages/{package_id}` — Update package.
    ///
    /// Update the metadata for an existing package.
    ///
    /// `package_id`: Package ID. Required. Example: "225494730938493804".
    pub async fn update_package(
        &self,
        package_id: impl Into<String>,
        body: &PutPackageBody,
    ) -> Result<Response<Package>, Error> {
        let path = format!(
            "/web/api/v2.1/update/agent/packages/{}",
            package_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<PutPackageBody, Response<Package>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/upload/agent/software` — Upload Agent Package.
    ///
    /// If you have an On-Prem Management or you are a participant in the Beta
    /// program, you can use this command to upload an Agent package to the
    /// Management. Then you can deploy the Agent to update endpoints.
    ///
    /// Note: this is a `multipart/form-data` upload. The shared JSON transport
    /// does not attach the binary `file` part; [`UploadAgentSoftwareBody`]
    /// carries only the form metadata fields.
    pub async fn upload_agent_software(
        &self,
        body: &UploadAgentSoftwareBody,
    ) -> Result<Response<Vec<Package>>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/upload/agent/software", body)
            .await?)
    }

    /// `POST /web/api/v2.1/upload/software` — Upload System Package.
    ///
    /// If you have an On-Prem Management or otherwise require a manual package
    /// upload, use this command to upload an Agent package or a Management
    /// package. Then you can deploy the update (see Deploy System Package).
    ///
    /// Note: this is a `multipart/form-data` upload (single `file` field). The
    /// shared JSON transport cannot attach the binary part; an empty JSON object
    /// is sent. Supply the file via a multipart-capable transport if required.
    pub async fn upload_software(&self) -> Result<Response<SuccessResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/upload/software", &serde_json::json!({}))
            .await?)
    }

    /// `POST /web/api/v2.1/upload/software/deploy` — Deploy System Package.
    ///
    /// If you have an On-Prem Management or you are a participant in the Beta
    /// program, you can upload a Management package and then use this command to
    /// deploy the new Management. You must first upload the package (see Upload
    /// System Package).
    pub async fn deploy_software(&self) -> Result<Response<SuccessResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/upload/software/deploy", &serde_json::json!({}))
            .await?)
    }
}
