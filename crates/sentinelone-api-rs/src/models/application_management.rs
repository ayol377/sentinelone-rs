//! Response models for the `Application Management` tag.
//!
//! Field nullability mirrors the SentinelOne 2.1 spec: no `data`-item field is
//! listed as `required`, so every field is `Option<T>` ("default null").
//! Enum-valued fields are kept as `String` for forward-compatibility; the
//! allowed values are documented on each field.

use serde::Deserialize;

/// One row from `GET /web/api/v2.1/application-management/inventory`.
///
/// Application inventory data grouped by application name and vendor.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInventory {
    /// Endpoints count.
    pub endpoints_count: Option<i64>,
    /// Application versions count.
    pub application_versions_count: Option<i64>,
    /// Vendor.
    pub application_vendor: Option<String>,
    /// Name.
    pub application_name: Option<String>,
    /// Estimate.
    pub estimate: Option<bool>,
}

/// One row from `GET /web/api/v2.1/application-management/inventory/applications`.
///
/// An installed application reported for a specific Agent/endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentApplication {
    /// Size.
    pub size: Option<i64>,
    /// Version.
    pub version: Option<String>,
    /// Installed date (date-time).
    pub installed_date: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Publisher.
    pub publisher: Option<String>,
}

/// One row from `GET /web/api/v2.1/application-management/inventory/endpoints`.
///
/// Endpoint data for a specific application.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationInventoryEndpoint {
    /// Account name.
    pub account_name: Option<String>,
    /// Version.
    pub version: Option<String>,
    /// Os version.
    pub os_version: Option<String>,
    /// File size.
    pub file_size: Option<i64>,
    /// Cpu count.
    pub cpu_count: Option<i64>,
    /// Endpoint type.
    pub endpoint_type: Option<String>,
    /// Application name.
    pub application_name: Option<String>,
    /// Application installation date (date-time).
    pub application_installation_date: Option<String>,
    /// Os name.
    pub os_name: Option<String>,
    /// Detection date (date-time).
    pub detection_date: Option<String>,
    /// Endpoint name.
    pub endpoint_name: Option<String>,
    /// Endpoint id.
    pub endpoint_id: Option<String>,
    /// Os arch.
    pub os_arch: Option<String>,
    /// Core count.
    pub core_count: Option<i64>,
    /// Id.
    pub id: Option<String>,
    /// Group name.
    pub group_name: Option<String>,
    /// OS type. Allowed values: `macos`, `windows`, `linux`, `windows_legacy`.
    pub os_type: Option<String>,
    /// Cpe (untyped in spec).
    pub cpe: Option<serde_json::Value>,
    /// Endpoint uuid.
    pub endpoint_uuid: Option<String>,
    /// Site name.
    pub site_name: Option<String>,
    /// Application installation path.
    pub application_installation_path: Option<String>,
}

/// One row from `GET /web/api/v2.1/application-management/risks`.
///
/// CVE vulnerability data for each CVE.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseRisksCve {
    /// Mitigation status reason.
    pub mitigation_status_reason: Option<String>,
    /// Risk mitigation status. Allowed values: `Not mitigated`, `To be patched`,
    /// `On hold`.
    pub mitigation_status: Option<String>,
    /// Report confidence. Available for Singularity Vulnerability Management SKU.
    /// Allowed values: `Not Defined`, `Unknown`, `Reasonable`, `Confirmed`.
    pub report_confidence: Option<String>,
    /// Mitigation status changer.
    pub mitigation_status_changed_by: Option<String>,
    /// Exploit code maturity. Available for Singularity Vulnerability Management
    /// SKU. Allowed values: `Not Defined`, `Unproven`, `Proof of Concept`,
    /// `Functional`, `High`.
    pub exploit_code_maturity: Option<String>,
    /// Application vendor.
    pub application_vendor: Option<String>,
    /// CVE Id.
    pub cve_id: Option<String>,
    /// Application version.
    pub application_version: Option<String>,
    /// Severity.
    pub severity: Option<String>,
    /// Marked by.
    pub marked_by: Option<String>,
    /// Endpoint type.
    pub endpoint_type: Option<String>,
    /// Mitigation status change time (date-time).
    pub mitigation_status_change_time: Option<String>,
    /// Application name.
    pub application_name: Option<String>,
    /// Mark type (untyped in spec).
    pub mark_type: Option<serde_json::Value>,
    /// Detection date (date-time).
    pub detection_date: Option<String>,
    /// Remediation level. Available for Singularity Vulnerability Management SKU.
    /// Allowed values: `Not Defined`, `Official Fix`, `Temporary Fix`,
    /// `Workaround`, `Unavailable`.
    pub remediation_level: Option<String>,
    /// Nvd cvss version. Available for Singularity Vulnerability Management SKU.
    pub nvd_cvss_version: Option<String>,
    /// Reason.
    pub reason: Option<String>,
    /// Composed application name.
    pub application: Option<String>,
    /// Endpoint name.
    pub endpoint_name: Option<String>,
    /// Marked date (date-time).
    pub marked_date: Option<String>,
    /// Nvd base score. Available for Singularity Vulnerability Management SKU.
    pub nvd_base_score: Option<String>,
    /// Endpoint id.
    pub endpoint_id: Option<String>,
    /// Published date (date-time).
    pub published_date: Option<String>,
    /// Last scan date (date-time).
    pub last_scan_date: Option<String>,
    /// Risk status. Allowed values: `Detected`, `Removed`.
    pub status: Option<String>,
    /// Last scan result.
    pub last_scan_result: Option<String>,
    /// Id.
    pub id: Option<String>,
    /// OS type. Allowed values: `macos`, `windows`, `linux`, `windows_legacy`.
    pub os_type: Option<String>,
    /// Base score. Not available with Singularity Vulnerability Management SKU.
    pub base_score: Option<String>,
    /// Risk score. Available for Singularity Vulnerability Management SKU.
    pub risk_score: Option<String>,
    /// Cvss version.
    pub cvss_version: Option<String>,
    /// Days detected.
    pub days_detected: Option<i64>,
}

/// One row from `GET /web/api/v2.1/application-management/risks/cves`.
///
/// CVE data for a specific application.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskyCve {
    /// Severity (untyped in spec).
    pub severity: Option<serde_json::Value>,
    /// Nvd url (untyped in spec).
    pub nvd_url: Option<serde_json::Value>,
    /// Exploited in the wild. Available with Ranger Insights. Allowed values:
    /// `Not Defined`, `Unknown`, `Yes`.
    pub exploited_in_the_wild: Option<String>,
    /// Report confidence. Available with Ranger Insights. Allowed values:
    /// `Not Defined`, `Reasonable`, `Unknown`, `Confirmed`.
    pub report_confidence: Option<String>,
    /// Nvd base score.
    pub nvd_base_score: Option<String>,
    /// Published date (date-time).
    pub published_date: Option<String>,
    /// Mitre url (untyped in spec).
    pub mitre_url: Option<serde_json::Value>,
    /// Risk score. Available with Ranger Insights.
    pub risk_score: Option<String>,
    /// Exploit code maturity. Available with Ranger Insights. Allowed values:
    /// `Unproven`, `High`, `Not Defined`, `Proof of Concept`, `Functional`.
    pub exploit_code_maturity: Option<String>,
    /// Description (untyped in spec).
    pub description: Option<serde_json::Value>,
    /// Fp fn marks.
    pub fp_fn_marks: Option<Vec<serde_json::Value>>,
    /// Cve id.
    pub cve_id: Option<String>,
    /// Remediation level. Available with Ranger Insights. Allowed values:
    /// `Temporary Fix`, `Not Defined`, `Official Fix`, `Workaround`,
    /// `Unavailable`.
    pub remediation_level: Option<String>,
    /// Cvss version.
    pub cvss_version: Option<String>,
}

/// One row from `GET /web/api/v2.1/application-management/risks/aggregated-applications`.
///
/// Aggregated risk data for all applications. Available with Ranger Insights.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregatedApplicationRisk {
    /// Highest nvd base score.
    pub highest_nvd_base_score: Option<String>,
    /// Highest severity.
    pub highest_severity: Option<String>,
    /// Days detected (untyped in spec).
    pub days_detected: Option<serde_json::Value>,
    /// Exploited in the wild. Allowed values: `Not Defined`, `Unknown`, `Yes`.
    pub exploited_in_the_wild: Option<String>,
    /// Vendor.
    pub vendor: Option<String>,
    /// Highest risk score.
    pub highest_risk_score: Option<String>,
    /// Application type.
    pub application_type: Option<String>,
    /// Cve count.
    pub cve_count: Option<i64>,
    /// Version count.
    pub version_count: Option<i64>,
    /// Estimate.
    pub estimate: Option<bool>,
    /// Name.
    pub name: Option<String>,
    /// Exploit code maturity. Allowed values: `Unproven`, `High`, `Not Defined`,
    /// `Proof of Concept`, `Functional`.
    pub exploit_code_maturity: Option<String>,
    /// Statuses.
    pub statuses: Option<Vec<serde_json::Value>>,
    /// Detection date (date-time).
    pub detection_date: Option<String>,
    /// Number of endpoints that are in an integrated scope, but don't have a
    /// ticket created. Available with Ranger Insights.
    pub endpoints_without_ticket: Option<i64>,
    /// Remediation level. Allowed values: `Temporary Fix`, `Not Defined`,
    /// `Official Fix`, `Workaround`, `Unavailable`.
    pub remediation_level: Option<String>,
    /// Endpoint count.
    pub endpoint_count: Option<i64>,
}

/// One row from `GET /web/api/v2.1/application-management/risks/applications`.
///
/// Risk data for each version of all applications.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationRisk {
    /// Highest nvd base score.
    pub highest_nvd_base_score: Option<String>,
    /// Highest severity.
    pub highest_severity: Option<String>,
    /// Days detected (untyped in spec).
    pub days_detected: Option<serde_json::Value>,
    /// Exploited in the wild. Available with Ranger Insights. Allowed values:
    /// `Not Defined`, `Unknown`, `Yes`.
    pub exploited_in_the_wild: Option<String>,
    /// Vendor.
    pub vendor: Option<String>,
    /// Highest risk score. Available with Ranger Insights.
    pub highest_risk_score: Option<String>,
    /// Application type. Available with Ranger Insights.
    pub application_type: Option<String>,
    /// Cve count.
    pub cve_count: Option<i64>,
    /// Application id.
    pub application_id: Option<String>,
    /// Estimate.
    pub estimate: Option<bool>,
    /// Name.
    pub name: Option<String>,
    /// Exploit code maturity. Available with Ranger Insights. Allowed values:
    /// `Unproven`, `High`, `Not Defined`, `Proof of Concept`, `Functional`.
    pub exploit_code_maturity: Option<String>,
    /// Statuses.
    pub statuses: Option<Vec<serde_json::Value>>,
    /// Detection date (date-time).
    pub detection_date: Option<String>,
    /// Number of endpoints that are in an integrated scope, but don't have a
    /// ticket created. Available with Ranger Insights.
    pub endpoints_without_ticket: Option<i64>,
    /// Remediation level. Available with Ranger Insights. Allowed values:
    /// `Temporary Fix`, `Not Defined`, `Official Fix`, `Workaround`,
    /// `Unavailable`.
    pub remediation_level: Option<String>,
    /// Endpoint count.
    pub endpoint_count: Option<i64>,
}

/// One row from `GET /web/api/v2.1/application-management/risks/endpoints`.
///
/// An endpoint installed with a specific vulnerable application.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskyEndpoint {
    /// Group name.
    pub group_name: Option<String>,
    /// Account name.
    pub account_name: Option<String>,
    /// OS type. Allowed values: `macos`, `windows`, `linux`, `windows_legacy`.
    pub os_type: Option<String>,
    /// Version.
    pub application_version: Option<String>,
    /// Endpoint name.
    pub endpoint_name: Option<String>,
    /// Endpoint type.
    pub endpoint_type: Option<String>,
    /// Endpoint uuid.
    pub endpoint_uuid: Option<String>,
    /// Os version.
    pub os_version: Option<String>,
    /// Site name.
    pub site_name: Option<String>,
    /// Last scan result.
    pub last_scan_result: Option<String>,
    /// External ticket system.
    pub external_ticket_system: Option<serde_json::Value>,
    /// Endpoint id.
    pub endpoint_id: Option<String>,
    /// Last scan date (date-time).
    pub last_scan_date: Option<String>,
    /// Status history. Available with Ranger Insights.
    pub status_history: Option<Vec<serde_json::Value>>,
    /// Ticket.
    pub ticket: Option<serde_json::Value>,
    /// Domain.
    pub domain: Option<String>,
    /// Detection date (date-time).
    pub application_detection_date: Option<String>,
    /// Application days detected (untyped in spec).
    pub application_days_detected: Option<serde_json::Value>,
}

/// Response `data` from `POST /web/api/v2.1/application-management/scan`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    /// Indicates a successful operation.
    pub success: Option<bool>,
}
