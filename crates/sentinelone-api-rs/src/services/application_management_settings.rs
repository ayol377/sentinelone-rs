use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::application_management_settings::ApplicationManagementSettings;
use crate::pagination::Response;

/// `Application Management Settings` tag.
///
/// Application Management Policy Settings.
pub struct ApplicationManagementSettingsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/application-management/settings`.
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSettingsQuery {
    /// Single Site ID to filter by. Example: `225494730938493804`.
    ///
    /// Array<string> query param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Single Group ID to filter by. Example: `225494730938493804`.
    ///
    /// Array<string> query param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Single Account ID to filter by. Example: `225494730938493804`.
    ///
    /// Array<string> query param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl GetSettingsQuery {
    /// Single Site ID to filter by. Example: `225494730938493804`.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = ids
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.site_ids = Some(joined);
        self
    }
    /// Single Group ID to filter by. Example: `225494730938493804`.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = ids
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.group_ids = Some(joined);
        self
    }
    /// Single Account ID to filter by. Example: `225494730938493804`.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = ids
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.account_ids = Some(joined);
        self
    }
}

/// Scope filter for the `POST /web/api/v2.1/application-management/settings` body.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsFilter {
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of Site IDs to filter by (1..=500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by (1..=500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by (1..=500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

impl UpdateSettingsFilter {
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// List of Site IDs to filter by (1..=500 items).
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// List of Group IDs to filter by (1..=500 items).
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// List of Account IDs to filter by (1..=500 items).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
}

/// Scan schedule configuration for the update body.
///
/// Per the spec these fields are required when `scanSchedule` is supplied.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsScanSchedule {
    /// Number of weeks between scans (must be between 1 and 4). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_every: Option<i64>,
    /// A day of the week. Allowed values: `Tuesday`, `Friday`, `Wednesday`,
    /// `Monday`, `Sunday`, `Saturday`, `Thursday`. Example: `Tuesday`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_on: Option<String>,
    /// Timezone for the scan time. Example: `Europe/Berlin`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Scan start time in 24-hour format (pattern `^(?:[01]\d|2[0-3]):[0-5]\d$`).
    /// Example: `20:15`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

impl UpdateSettingsScanSchedule {
    /// Number of weeks between scans (must be between 1 and 4).
    pub fn scan_every(mut self, n: i64) -> Self {
        self.scan_every = Some(n);
        self
    }
    /// A day of the week. Allowed values: `Tuesday`, `Friday`, `Wednesday`,
    /// `Monday`, `Sunday`, `Saturday`, `Thursday`.
    pub fn repeat_on(mut self, v: impl Into<String>) -> Self {
        self.repeat_on = Some(v.into());
        self
    }
    /// Timezone for the scan time. Example: `Europe/Berlin`.
    pub fn timezone(mut self, v: impl Into<String>) -> Self {
        self.timezone = Some(v.into());
        self
    }
    /// Scan start time in 24-hour format. Example: `20:15`.
    pub fn time(mut self, v: impl Into<String>) -> Self {
        self.time = Some(v.into());
        self
    }
}

/// The `data` portion of the update body.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsData {
    /// Extensive scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensive_scan_enabled: Option<bool>,
    /// Determines if the policy is overridden on the scope level. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_policy: Option<bool>,
    /// Scan schedule configuration. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_schedule: Option<UpdateSettingsScanSchedule>,
    /// Extensive Linux scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensive_linux_scan_enabled: Option<bool>,
    /// Vulnerabilities scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerabilities_scan_enabled: Option<bool>,
}

impl UpdateSettingsData {
    /// Extensive scan enabled.
    pub fn extensive_scan_enabled(mut self, v: bool) -> Self {
        self.extensive_scan_enabled = Some(v);
        self
    }
    /// Determines if the policy is overridden on the scope level.
    pub fn is_default_policy(mut self, v: bool) -> Self {
        self.is_default_policy = Some(v);
        self
    }
    /// Scan schedule configuration.
    pub fn scan_schedule(mut self, v: UpdateSettingsScanSchedule) -> Self {
        self.scan_schedule = Some(v);
        self
    }
    /// Extensive Linux scan enabled.
    pub fn extensive_linux_scan_enabled(mut self, v: bool) -> Self {
        self.extensive_linux_scan_enabled = Some(v);
        self
    }
    /// Vulnerabilities scan enabled.
    pub fn vulnerabilities_scan_enabled(mut self, v: bool) -> Self {
        self.vulnerabilities_scan_enabled = Some(v);
        self
    }
}

/// Request body for `POST /web/api/v2.1/application-management/settings`.
///
/// Both `data` and `filter` are required by the spec.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsBody {
    /// Data. The settings to apply. Required.
    pub data: UpdateSettingsData,
    /// Filter. The scope the settings apply to. Required.
    pub filter: UpdateSettingsFilter,
}

impl ApplicationManagementSettingsService<'_> {
    /// `GET /web/api/v2.1/application-management/settings` — Get Application Management Settings.
    ///
    /// Get Application Management settings.
    pub async fn get_settings(
        &self,
        query: &GetSettingsQuery,
    ) -> Result<Response<ApplicationManagementSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/application-management/settings", q)
            .await?)
    }

    /// `POST /web/api/v2.1/application-management/settings` — Update Application Management Settings.
    ///
    /// Update Application Management Settings.
    pub async fn update_settings(
        &self,
        body: &UpdateSettingsBody,
    ) -> Result<Response<ApplicationManagementSettings>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/application-management/settings", body)
            .await?)
    }
}
