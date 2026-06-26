//! Models for the `System` tag (general system information).
//!
//! Hand-written at 1:1 parity with `swagger_2_1.json`. The response `data`
//! objects for these endpoints declare no `required` array, so every field is
//! `Option<T>` (default-null behaviour).

use serde::Deserialize;

/// Console build, version, patch, and release information.
///
/// Response data of `GET /web/api/v2.1/system/info`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    /// Release.
    pub release: Option<String>,
    /// Version.
    pub version: Option<String>,
    /// Build.
    pub build: Option<String>,
    /// Patch.
    pub patch: Option<String>,
    /// Latest agent version.
    pub latest_agent_version: Option<String>,
}

/// System health indicator.
///
/// Response data of `GET /web/api/v2.1/system/status` (and the deprecated
/// `/status/cache` and `/status/db`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatus {
    /// System health indicator. Always returns `"ok"` when it is up and running.
    pub health: Option<String>,
}

/// Environment details of the system.
///
/// Response data of `GET /web/api/v2.1/system/env`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemEnv {
    /// Env.
    pub env: Option<String>,
    /// Is prod.
    pub is_prod: Option<bool>,
    /// Url.
    pub url: Option<String>,
}

/// An allowed domain entry for user creation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemConfigurationAllowedDomain {
    /// Allowed domain name for user creation.
    pub domain: Option<String>,
    /// True if this is an inherited domain.
    pub inherited: Option<bool>,
}

/// A min/max integer range (minutes / seconds / days, depending on the field).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemConfigurationRange {
    /// Minimum value of the range.
    pub min: Option<i64>,
    /// Maximum value of the range.
    pub max: Option<i64>,
}

/// Configuration of the SentinelOne system.
///
/// Response data of `GET`/`PUT` `/web/api/v2.1/system/configuration`. Shows
/// basic information of the deployed SKUs and licenses, 2FA, and the
/// Management URL.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemConfiguration {
    /// \[DEPRECATED\] Cloud intelligence on.
    pub cloud_intelligence_on: Option<bool>,
    /// Global two fa enabled.
    pub global_two_fa_enabled: Option<bool>,
    /// Cloud last connection time (ISO-8601 timestamp string).
    pub cloud_last_connection_time: Option<String>,
    /// Time in minutes until a user session expires.
    pub remember_me_length: Option<i64>,
    /// Remember me length range.
    pub remember_me_length_range: Option<SystemConfigurationRange>,
    /// Length of UI inactivity period, in seconds.
    pub ui_inactivity_timeout_seconds: Option<i64>,
    /// UI inactivity timeout range, in seconds.
    pub ui_inactivity_timeout_seconds_range: Option<SystemConfigurationRange>,
    /// External DNS name of the management.
    pub accessible_url: Option<String>,
    /// The Scalyr URL that sends data to this Console.
    pub scalyr_url: Option<String>,
    /// The region of the management.
    pub region: Option<String>,
    /// True if advanced mode is enabled.
    pub advanced_mode: Option<bool>,
    /// True if advanced mode value can be updated from this scope.
    pub advanced_mode_allow_changes: Option<bool>,
    /// True if early access mode is enabled.
    pub early_access: Option<bool>,
    /// Early access platforms. Allowed item values: `windows_legacy`,
    /// `threat_detection_netapp`, `windows`, `linux_k8s`,
    /// `threat_detection_s3`, `macos`, `sdk`, `linux`.
    pub early_access_platforms: Option<Vec<String>>,
    /// \[DEPRECATED\] Core licenses.
    pub max_core_licenses: Option<i64>,
    /// \[DEPRECATED\] Control licenses.
    pub max_control_licenses: Option<i64>,
    /// \[DEPRECATED\] Complete licenses.
    pub max_complete_licenses: Option<i64>,
    /// \[DEPRECATED\] True if Core licenses is unlimited.
    pub unlimited_core: Option<bool>,
    /// \[DEPRECATED\] True if Control licenses is unlimited.
    pub unlimited_control: Option<bool>,
    /// \[DEPRECATED\] True if Complete licenses is unlimited.
    pub unlimited_complete: Option<bool>,
    /// \[DEPRECATED\] Allow site admins to duplicate sites in their accounts.
    pub allow_duplicate_site: Option<bool>,
    /// List of allowed domains for user creation.
    pub allowed_domains: Option<Vec<SystemConfigurationAllowedDomain>>,
    /// List of available licenses (bundles, modules, settings). Freeform —
    /// represented as raw JSON.
    pub licenses: Option<serde_json::Value>,
    /// 2FA enrollment expiration period, in minutes.
    pub tfa_enrollment_expiration: Option<i64>,
    /// Shared console field of tenant.
    pub global_shared_console: Option<bool>,
    /// Salesforce id field of tenant.
    pub global_salesforce_id: Option<String>,
    /// Time in days until a user password expires.
    pub password_expiration: Option<i64>,
    /// Password expiration range (read-only), in days.
    pub password_expiration_range: Option<SystemConfigurationRange>,
}
