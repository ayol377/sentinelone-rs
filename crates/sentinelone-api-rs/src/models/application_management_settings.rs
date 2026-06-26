//! Models for the `Application Management Settings` tag.
//!
//! Application Management Policy Settings.

use serde::Deserialize;

/// Application Management settings for a scope.
///
/// Returned by `GET`/`POST /web/api/v2.1/application-management/settings`
/// (the `data` object of the response envelope).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationManagementSettings {
    /// Extensive scan enabled. Optional/nullable.
    pub extensive_scan_enabled: Option<bool>,
    /// Determines if the policy is overridden on the scope level. Optional/nullable.
    pub is_default_policy: Option<bool>,
    /// Scan schedule configuration. Optional/nullable.
    pub scan_schedule: Option<ApplicationManagementSettingsScanSchedule>,
    /// Determines if settings are overridden on any descendant scope. Optional/nullable.
    pub has_breaking_inheritance: Option<bool>,
    /// Extensive Linux scan enabled. Optional/nullable.
    pub extensive_linux_scan_enabled: Option<bool>,
    /// Inherited from. Optional/nullable.
    pub inherited_from: Option<String>,
    /// Vulnerabilities scan enabled. Optional/nullable.
    pub vulnerabilities_scan_enabled: Option<bool>,
}

/// Scan schedule configuration for Application Management settings.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationManagementSettingsScanSchedule {
    /// Number of weeks between scans (must be between 1 and 4).
    ///
    /// Required by the nested schema and not nullable -> bare `i64`.
    pub scan_every: i64,
    /// A day of the week. Allowed values: `Tuesday`, `Friday`, `Wednesday`,
    /// `Monday`, `Sunday`, `Saturday`, `Thursday`. Example: `Tuesday`.
    ///
    /// Required by the nested schema and not nullable -> bare `String`.
    /// Modeled as `String` for forward compatibility.
    pub repeat_on: String,
    /// Timezone for the scan time. Example: `Europe/Berlin`.
    ///
    /// Required by the nested schema and not nullable -> bare `String`.
    pub timezone: String,
    /// Scan start time in 24-hour format (pattern `^(?:[01]\d|2[0-3]):[0-5]\d$`).
    /// Example: `20:15`.
    ///
    /// Required by the nested schema and not nullable -> bare `String`.
    pub time: String,
}
