//! Models for the `Auto Upgrade Policy` tag.
//!
//! Hand-written for 1:1 spec parity with `swagger_2_1.json`. None of the
//! definitions in this tag declare a `required` array or `x-nullable`, so every
//! field defaults to `Option<T>` (the "default null" behaviour).

use serde::Deserialize;

/// An auto upgrade policy (`v2_1.models.Policy`).
///
/// Returned inside [`PoliciesCollection`] by the `GET .../policies` and
/// `GET .../parent-policies` endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Policy {
    /// Timestamp the policy was activated at (ISO-8601 date/time string).
    pub activated_at: Option<String>,
    /// `true` if the policy is applied to all endpoints.
    pub all_endpoints: Option<bool>,
    /// Timestamp the policy was created at (ISO-8601 date/time string).
    pub created_at: Option<String>,
    /// Policy description.
    pub description: Option<String>,
    /// Policy ID.
    pub id: Option<String>,
    /// `true` if the policy is active, `false` if disabled.
    pub is_active: Option<bool>,
    /// `true` if the upgrade is scheduled for a maintenance window.
    pub is_scheduled: Option<bool>,
    /// Maximum number of upgrade retries for an endpoint on retriable failure.
    pub max_retries: Option<i64>,
    /// Policy name.
    pub name: Option<String>,
    /// OS type. Allowed values: `linux`, `macos`, `windows`.
    pub os_type: Option<String>,
    /// The package to be sent to an Agent during the upgrade.
    pub package: Option<Package>,
    /// Policy priority / ordering.
    pub priority: Option<i64>,
    /// Scope ID.
    pub scope_id: Option<String>,
    /// Scope level. Allowed values: `account`, `group`, `site`, `tenant`.
    pub scope_level: Option<String>,
    /// Tags the policy applies to (when `all_endpoints` is `false`).
    pub tags: Option<Vec<String>>,
    /// Timestamp the policy was last updated at (ISO-8601 date/time string).
    pub updated_at: Option<String>,
}

/// The package to be sent to an Agent during the upgrade (`v2_1.models.Package`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    /// Package build identifier.
    pub build: Option<String>,
    /// File ID of the package.
    pub file_id: Option<String>,
    /// Major version component.
    pub major: Option<String>,
    /// Minor version component.
    pub minor: Option<String>,
}

/// A collection of policies plus inheritance metadata
/// (`v2_1.models.PoliciesCollection`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoliciesCollection {
    /// `true` if policies are inherited from a higher scope.
    pub is_inherited: Option<bool>,
    /// The policies in this scope.
    pub policies: Option<Vec<Policy>>,
    /// `true` if there are policies in a child scope.
    pub policies_in_child_scope: Option<bool>,
}

/// Pagination metadata (`v2_1.models.PaginationInfo`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationInfo {
    /// Total number of items across all pages.
    pub total_items: Option<i64>,
}

/// Response of `GET .../policies` and `GET .../parent-policies`
/// (`v2_1.models.GetPoliciesResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPoliciesResponse {
    /// The policies collection.
    pub data: Option<PoliciesCollection>,
    /// Pagination metadata.
    pub pagination: Option<PaginationInfo>,
}

/// Per-OS counts (`v2_1.models.OsCountResult`).
///
/// Returned by `GET .../all-policies-count` and `GET .../policies-count`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OsCountResult {
    /// Number of policies for Linux.
    pub linux: Option<i64>,
    /// Number of policies for macOS.
    pub macos: Option<i64>,
    /// Number of policies for Windows.
    pub windows: Option<i64>,
}

/// A single available package file (`v2_1.models.PackageFile`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageFile {
    /// File ID.
    pub id: Option<String>,
    /// File name.
    pub name: Option<String>,
}

/// An available package (`v2_1.models.PackageResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageResponse {
    /// Package build identifier.
    pub build: Option<String>,
    /// Human-readable package name, e.g. `22.1 GA`.
    pub display_name: Option<String>,
    /// The package files.
    pub file_names: Option<Vec<PackageFile>>,
    /// Major version component.
    pub major: Option<String>,
    /// Minor version component.
    pub minor: Option<String>,
}

/// A collection of available packages (`v2_1.models.PackagesCollection`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackagesCollection {
    /// The available packages.
    pub packages: Option<Vec<PackageResponse>>,
}

/// Response of `GET .../available-packages`
/// (`v2_1.models.GetAvailablePackagesResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAvailablePackagesResponse {
    /// The packages collection.
    pub data: Option<PackagesCollection>,
}

/// Response of `POST .../has-policy` (`v2_1.models.HasPoliciesResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HasPoliciesResponse {
    /// `true` if a matching policy exists.
    pub has_policies: Option<bool>,
}

/// Generic empty/acknowledgement response (`v2_1.models.EmptyResponse`).
///
/// Returned by the mutation endpoints (create/update/action/reorder/etc.).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyResponse {
    /// Opaque response string.
    pub response: Option<String>,
}
