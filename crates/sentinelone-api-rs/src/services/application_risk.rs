use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::pagination::Response;

/// `Application Risk` tag.
///
/// Installed applications and known vulnerabilities and exposures. Available
/// for Complete SKU only.
pub struct ApplicationRiskService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/export/installed-applications`.
///
/// All fields are optional. Array params (e.g. `siteIds`, `osTypes`) are
/// serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportInstalledApplicationsQuery {
    /// List of Site IDs to filter by. Example: `"225494730938493804,225494730938493915"`.
    ///
    /// Optional. Array of strings, comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: `"225494730938493804,225494730938493915"`.
    ///
    /// Optional. Array of strings, comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by. Example: `"225494730938493804,225494730938493915"`.
    ///
    /// Optional. Array of strings, comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Filter by application IDs. Example: `"225494730938493804,225494730938493915"`.
    ///
    /// Optional. Array of strings, comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Filter by OS types. Example: `"linux"`.
    ///
    /// Optional. Array of enum values, comma-joined. Allowed values:
    /// `linux`, `macos`, `windows_legacy`, `windows`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Filter not by OS types. Example: `"linux"`.
    ///
    /// Optional. Array of enum values, comma-joined. Allowed values:
    /// `linux`, `macos`, `windows_legacy`, `windows`.
    #[serde(rename = "osTypesNin", skip_serializing_if = "Option::is_none")]
    pub os_types_nin: Option<String>,
    /// Filter by endpoint machine types. Example: `"unknown"`.
    ///
    /// Optional. Array of enum values, comma-joined. Allowed values:
    /// `unknown`, `desktop`, `laptop`, `server`, `kubernetes node`,
    /// `storage`, `kubernetes pod`, `ecs task`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_machine_types: Option<String>,
    /// Filter not by endpoint machine types. Example: `"unknown"`.
    ///
    /// Optional. Array of enum values, comma-joined. Allowed values:
    /// `unknown`, `desktop`, `laptop`, `server`, `kubernetes node`,
    /// `storage`, `kubernetes pod`, `ecs task`.
    #[serde(
        rename = "agentMachineTypesNin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_machine_types_nin: Option<String>,
    /// Filter by installation date range.
    ///
    /// Optional. String.
    #[serde(rename = "installedAt__between", skip_serializing_if = "Option::is_none")]
    pub installed_at_between: Option<String>,
    /// Filter by application types. Example: `"app"`.
    ///
    /// Optional. Array of enum values, comma-joined. Allowed values:
    /// `app`, `kb`, `patch`, `chromeExtension`, `edgeExtension`,
    /// `firefoxExtension`, `safariExtension`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// Filter not by application types. Example: `"app"`.
    ///
    /// Optional. Array of enum values, comma-joined. Allowed values:
    /// `app`, `kb`, `patch`, `chromeExtension`, `edgeExtension`,
    /// `firefoxExtension`, `safariExtension`.
    #[serde(rename = "typesNin", skip_serializing_if = "Option::is_none")]
    pub types_nin: Option<String>,
    /// Filter by risk. Example: `"none"`.
    ///
    /// Optional. Array of enum values, comma-joined. Allowed values:
    /// `none`, `low`, `medium`, `high`, `critical`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_levels: Option<String>,
    /// Filter not by risk. Example: `"none"`.
    ///
    /// Optional. Array of enum values, comma-joined. Allowed values:
    /// `none`, `low`, `medium`, `high`, `critical`.
    #[serde(rename = "riskLevelsNin", skip_serializing_if = "Option::is_none")]
    pub risk_levels_nin: Option<String>,
    /// Filter by application size range (bytes). Example: `"1024-104856"`.
    ///
    /// Optional. String.
    #[serde(rename = "size__between", skip_serializing_if = "Option::is_none")]
    pub size_between: Option<String>,
    /// Include active agents, decommissioned or both. Example: `"True,False"`.
    ///
    /// Optional. Array of booleans, comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_is_decommissioned: Option<String>,
    /// Free-text filter by application name (supports multiple values).
    /// Example: `"calc"`.
    ///
    /// Optional. Array of strings, comma-joined.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by application version (supports multiple values).
    /// Example: `"1.22.333,build"`.
    ///
    /// Optional. Array of strings, comma-joined.
    #[serde(rename = "version__contains", skip_serializing_if = "Option::is_none")]
    pub version_contains: Option<String>,
    /// Free-text filter by application publisher (supports multiple values).
    /// Example: `"Sentinel"`.
    ///
    /// Optional. Array of strings, comma-joined.
    #[serde(rename = "publisher__contains", skip_serializing_if = "Option::is_none")]
    pub publisher_contains: Option<String>,
    /// Free-text filter by computer name (supports multiple values).
    /// Example: `"john-office,WIN"`.
    ///
    /// Optional. Array of strings, comma-joined.
    #[serde(
        rename = "agentComputerName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_computer_name_contains: Option<String>,
    /// Free-text filter by agent UUID (supports multiple values).
    /// Example: `"e92-01928,b055"`.
    ///
    /// Optional. Array of strings, comma-joined.
    #[serde(
        rename = "agentUuid__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_uuid_contains: Option<String>,
    /// Free-text filter by OS full name and version (supports multiple values).
    /// Example: `"Service Pack 1"`.
    ///
    /// Optional. Array of strings, comma-joined.
    #[serde(
        rename = "agentOsVersion__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_os_version_contains: Option<String>,
}

impl ExportInstalledApplicationsQuery {
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(values));
        self
    }
    /// Filter by application IDs.
    pub fn ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(values));
        self
    }
    /// Filter by OS types. Allowed: `linux`, `macos`, `windows_legacy`, `windows`.
    pub fn os_types<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(values));
        self
    }
    /// Filter not by OS types. Allowed: `linux`, `macos`, `windows_legacy`, `windows`.
    pub fn os_types_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types_nin = Some(join_csv(values));
        self
    }
    /// Filter by endpoint machine types.
    pub fn agent_machine_types<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_machine_types = Some(join_csv(values));
        self
    }
    /// Filter not by endpoint machine types.
    pub fn agent_machine_types_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_machine_types_nin = Some(join_csv(values));
        self
    }
    /// Filter by installation date range.
    pub fn installed_at_between(mut self, v: impl Into<String>) -> Self {
        self.installed_at_between = Some(v.into());
        self
    }
    /// Filter by application types.
    pub fn types<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.types = Some(join_csv(values));
        self
    }
    /// Filter not by application types.
    pub fn types_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.types_nin = Some(join_csv(values));
        self
    }
    /// Filter by risk. Allowed: `none`, `low`, `medium`, `high`, `critical`.
    pub fn risk_levels<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_levels = Some(join_csv(values));
        self
    }
    /// Filter not by risk. Allowed: `none`, `low`, `medium`, `high`, `critical`.
    pub fn risk_levels_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_levels_nin = Some(join_csv(values));
        self
    }
    /// Filter by application size range (bytes).
    pub fn size_between(mut self, v: impl Into<String>) -> Self {
        self.size_between = Some(v.into());
        self
    }
    /// Include active agents, decommissioned or both.
    pub fn agent_is_decommissioned<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_is_decommissioned = Some(join_csv(values));
        self
    }
    /// Free-text filter by application name.
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by application version.
    pub fn version_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.version_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by application publisher.
    pub fn publisher_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.publisher_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by computer name.
    pub fn agent_computer_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_computer_name_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by agent UUID.
    pub fn agent_uuid_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by OS full name and version.
    pub fn agent_os_version_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_os_version_contains = Some(join_csv(values));
        self
    }
}

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

impl ApplicationRiskService<'_> {
    /// `GET /web/api/v2.1/export/installed-applications` — Export Applications.
    ///
    /// Export the list of applications installed on endpoints with Application
    /// Risk-enabled Agents and their properties, including the CVEs for each
    /// application that requires a patch. The CSV file is stored on the
    /// Management. Application Risk requires Complete SKU. This feature is in EA.
    /// To join the EA program, contact your SentinelOne Sales Rep.
    ///
    /// The endpoint triggers a server-side export (the CSV is stored on the
    /// Management console) and returns no structured response body, so the raw
    /// JSON envelope is surfaced as [`serde_json::Value`].
    pub async fn export_installed_applications(
        &self,
        query: &ExportInstalledApplicationsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/export/installed-applications", q)
            .await?)
    }
}
