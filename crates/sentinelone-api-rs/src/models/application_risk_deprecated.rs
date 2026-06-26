//! Models for the `Application Risk (Deprecated)` tag.
//!
//! Installed applications and known vulnerabilities and exposures (CVEs)
//! reported by Application Risk-enabled Agents. Available for Complete SKU only.

use serde::Deserialize;

/// An application installed on an endpoint, as reported by an
/// Application Risk-enabled Agent.
///
/// Returned by `GET /web/api/v2.1/installed-applications`.
///
/// No fields are marked `required` in the response schema, so every field is
/// `Option<T>` (default-null behaviour).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    /// Application ID.
    pub id: Option<String>,
    /// Created at (date/time string, e.g. `2018-02-27T04:49:26.257525Z`).
    pub created_at: Option<String>,
    /// Updated at (date/time string, e.g. `2018-02-27T04:49:26.257525Z`).
    pub updated_at: Option<String>,
    /// Type. One of: `app`, `kb`, `patch`, `chromeExtension`, `edgeExtension`,
    /// `firefoxExtension`, `safariExtension`.
    pub r#type: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Version.
    pub version: Option<String>,
    /// Publisher.
    pub publisher: Option<String>,
    /// OS type. One of: `linux`, `macos`, `windows_legacy`, `windows`.
    pub os_type: Option<String>,
    /// Installed at (date/time string, e.g. `2018-02-27T04:49:26.257525Z`).
    pub installed_at: Option<String>,
    /// Application size (bytes).
    pub size: Option<i64>,
    /// Signed.
    pub signed: Option<bool>,
    /// Risk level (nullable). One of: `none`, `low`, `medium`, `high`, `critical`.
    pub risk_level: Option<String>,
    /// Agent id.
    pub agent_id: Option<String>,
    /// Agent machine type. One of: `unknown`, `desktop`, `laptop`, `server`,
    /// `kubernetes node`, `storage`, `kubernetes pod`, `ecs task`,
    /// `kubernetes helper`.
    pub agent_machine_type: Option<String>,
    /// Agent uuid.
    pub agent_uuid: Option<String>,
    /// Agent computer name.
    pub agent_computer_name: Option<String>,
    /// Agent domain.
    pub agent_domain: Option<String>,
    /// Agent version.
    pub agent_version: Option<String>,
    /// Agent OS type. One of: `linux`, `macos`, `windows_legacy`, `windows`.
    pub agent_os_type: Option<String>,
    /// Agent network status. One of: `connected`, `disconnected`, `connecting`,
    /// `disconnecting`.
    pub agent_network_status: Option<String>,
    /// Agent is decommissioned.
    pub agent_is_decommissioned: Option<bool>,
    /// Agent is active.
    pub agent_is_active: Option<bool>,
    /// Agent infected.
    pub agent_infected: Option<bool>,
    /// Agent operational state.
    pub agent_operational_state: Option<String>,
}

/// A known CVE (Common Vulnerabilities and Exposures) for an application
/// installed on an endpoint with an Application Risk-enabled Agent.
///
/// Returned by `GET /web/api/v2.1/installed-applications/cves`.
///
/// No fields are marked `required` in the response schema, so every field is
/// `Option<T>` (default-null behaviour).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cve {
    /// Internal CVE ID.
    pub id: Option<String>,
    /// Created at (date/time string, e.g. `2018-02-27T04:49:26.257525Z`).
    pub created_at: Option<String>,
    /// Updated at (date/time string, e.g. `2018-02-27T04:49:26.257525Z`).
    pub updated_at: Option<String>,
    /// Global CVE ID (e.g. `CVE-2018-3204`).
    pub cve_id: Option<String>,
    /// Score.
    pub score: Option<f64>,
    /// Risk level. One of: `none`, `low`, `medium`, `high`, `critical`.
    pub risk_level: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Published at (date/time string).
    pub published_at: Option<String>,
    /// Link.
    pub link: Option<String>,
}
