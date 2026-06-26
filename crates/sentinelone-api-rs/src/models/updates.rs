//! Models for the `Updates` tag (file uploads / Agent packages).

use serde::Deserialize;

/// An Agent update package uploaded to the Management.
///
/// Returned by `GET /web/api/v2.1/update/agent/packages`,
/// `PUT /web/api/v2.1/update/agent/packages/{package_id}` and
/// `POST /web/api/v2.1/upload/agent/software`.
///
/// In the list response no field is in the schema `required` array, so every
/// field is `Option<T>` (default-null behaviour). In the single/`many` upload
/// responses `accounts`, `sites` and `status` are required; they are still
/// modelled as `Option<T>` here so the one struct can be reused across all
/// package endpoints without deserialization failures.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    /// Id. Example: "225494730938493804".
    pub id: Option<String>,
    /// Status. Allowed values: `beta`, `ea`, `ga`, `other`.
    pub status: Option<String>,
    /// Package scope. If "global", it will be available in all sites. If "site",
    /// it will be available only to sites specified in the "siteIds" attribute.
    /// Allowed values: `site`, `account`, `global`.
    pub scope_level: Option<String>,
    /// Agent version. Example: "2.5.1.1320".
    pub version: Option<String>,
    /// Package OS architecture (32/64 bit), applicable to Windows packages only.
    /// Allowed values: `64 bit`, `ARM64`, `32/64 bit`, `32 bit/64 bit/ARM64`,
    /// `32 bit`, `N/A`.
    pub os_arch: Option<String>,
    /// Network Scanner version if applicable. Example: "2.5.1.1320".
    pub ranger_version: Option<String>,
    /// Minor version.
    pub minor_version: Option<String>,
    /// Major version.
    pub major_version: Option<String>,
    /// Supported os versions.
    pub supported_os_versions: Option<String>,
    /// Created at. Example: "2018-02-27T04:49:26.257525Z".
    pub created_at: Option<String>,
    /// Updated at. Example: "2018-02-27T04:49:26.257525Z".
    pub updated_at: Option<String>,
    /// Platform type. Allowed values: `windows_legacy`, `threat_detection_netapp`,
    /// `windows`, `linux_k8s`, `threat_detection_s3`, `macos`, `sdk`, `linux`.
    pub os_type: Option<String>,
    /// Platform type. Allowed values: `windows_legacy`, `threat_detection_netapp`,
    /// `windows`, `linux_k8s`, `threat_detection_s3`, `macos`, `sdk`, `linux`.
    pub platform_type: Option<String>,
    /// File name. Example: "S1Agent_windows_2_5_1_1320".
    pub file_name: Option<String>,
    /// File size (bytes). Example: 1839271.
    pub file_size: Option<i64>,
    /// File extension. Allowed values: `.msi`, `.exe`, `.deb`, `.rpm`, `.bsx`,
    /// `.pkg`, `.img`, `unknown`, `.tar`, `.zip`, `.gz`, `.xz`.
    pub file_extension: Option<String>,
    /// Package hash. Example: "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12".
    pub sha1: Option<String>,
    /// Link (read-only download URL).
    pub link: Option<serde_json::Value>,
    /// Sites where the update package is available for download.
    pub sites: Option<Vec<PackageScopeEntity>>,
    /// Accounts where the update package is available for download.
    pub accounts: Option<Vec<PackageScopeEntity>>,
    /// Package type. Allowed values: `Agent`, `Ranger`, `AgentAndRanger`.
    pub package_type: Option<String>,
}

/// A site or account reference attached to a [`Package`].
///
/// Both `id` and `name` are in the schema `required` array and are not
/// `x-nullable`, so both are bare `String`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageScopeEntity {
    /// Id. Example: "225494730938493804".
    pub id: String,
    /// Name. Example: "SentinelOne".
    pub name: String,
}

/// Response data for `GET /web/api/v2.1/update/agent/latest-packages`
/// (deprecated "Latest Packages by OS").
///
/// No field is required, so everything is `Option<T>`. `osTypes` is a freeform
/// map of os-type -> package list; it is kept as [`serde_json::Value`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestPackages {
    /// Os types — map of `linux`/`macos`/`windows`/`windowsLegacy` to package
    /// lists. Freeform per spec.
    pub os_types: Option<serde_json::Value>,
    /// Registration token.
    pub registration_token: Option<serde_json::Value>,
}

/// Result of an operation that affects a number of entities
/// (`DELETE /web/api/v2.1/update/agent/packages`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResult {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
}

/// Result of an operation that reports a simple success flag
/// (`POST /web/api/v2.1/upload/software`,
/// `POST /web/api/v2.1/upload/software/deploy`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResult {
    /// Indicates a successful operation.
    pub success: Option<bool>,
}
