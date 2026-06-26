//! Models for the `Mobile Integration` tag.
//!
//! Hand-written at 1:1 spec parity with `swagger_2_1.json`. Envelope
//! (`data` / `pagination` / `errors`) is stripped and handled by
//! [`crate::pagination::Paginated`] / [`crate::pagination::Response`].
//!
//! Field nullability rule: a field is a bare `T` only when it is in the
//! schema `required` array AND not `x-nullable`; otherwise it is
//! `Option<T>` (default-null behaviour). Enum-typed strings are kept as
//! `String` for forward-compatibility, with allowed values documented.

use serde::Deserialize;

// ===========================================================================
// Devices (GET /web/api/v2.1/mobile-integration/devices)
// ===========================================================================

/// A mobile device entity returned by the devices list endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    /// Id. (required)
    pub id: i64,
    /// MDM device ID. (required)
    pub device_id: String,
    /// A reference to the containing account. (required)
    pub account_id: String,
    /// Name of the containing account. (required)
    pub account_name: String,
    /// A reference to the containing site. (required)
    pub site_id: String,
    /// Name of the containing site. (required)
    pub site_name: String,
    /// A reference to the containing network group. (required)
    pub group_id: String,
    /// Name of the containing network group. (required)
    pub group_name: String,
    /// MDM name or initiator in case of activations. (required)
    pub registration_source: String,
    /// Registration type. (required)
    ///
    /// Allowed values: `mdm`, `invite`, `global_link`.
    pub registration_type: String,
    /// Alert counts for the device. (required)
    pub alert_counts: DeviceCounts,
    /// Threat counts for the device. (required)
    pub threat_counts: DeviceCounts,
    /// Either rooted or jailbroken for devices with privileges. Otherwise none. (optional)
    ///
    /// Allowed values: `none`, `rooted`, `jailbroken`.
    pub privileges: Option<String>,
    /// When the ZippApp registered. (optional, date-time string)
    pub registered_on: Option<String>,
    /// When the corresponding policy was updated. (optional, date-time string)
    pub policy_updated_at: Option<String>,
    /// Debug mode. (optional)
    pub debug_mode: Option<bool>,
    /// Version of the ZippApp. (optional)
    pub app_version: Option<String>,
    /// External tracking ID of device. (optional)
    pub tracking_id1: Option<String>,
    /// When the activation for this device was created. (optional, date-time string)
    pub registration_date: Option<String>,
    /// Encrypted. (optional)
    pub encrypted: Option<bool>,
    /// Another external tracking ID of device. (optional)
    pub tracking_id2: Option<String>,
    /// Highest health state of the device. (optional)
    ///
    /// Allowed values: `normal`, `low`, `medium`, `high`, `critical`, `not_activated`.
    pub health_state: Option<String>,
    /// Unofficial appstore. (optional)
    pub unofficial_appstore: Option<bool>,
    /// Device manufacturer and model. (optional)
    pub model: Option<String>,
    /// UEM state of the device. (optional)
    ///
    /// Allowed values: `unknown`, `synced`, `deleted`.
    pub managed_state: Option<String>,
    /// Device os version. (optional)
    pub os_version: Option<String>,
    /// When we received last heartbeat. (optional, date-time string)
    pub last_active_on: Option<String>,
    /// Screen locked. (optional)
    pub screen_locked: Option<bool>,
    /// Developer mode. (optional)
    pub developer_mode: Option<bool>,
    /// Stagefreight vulnerable. (optional)
    pub stagefreight_vulnerable: Option<bool>,
    /// Device platform. (optional)
    ///
    /// Allowed values: `android`, `ios`, `chrome_os`.
    pub platform: Option<String>,
    /// User email. (optional)
    pub owner: Option<String>,
    /// Protected. (optional)
    pub protected: Option<bool>,
    /// ZipApp state. (optional)
    ///
    /// Allowed values: `unknown`, `registered`.
    pub app_state: Option<String>,
}

/// Alert / threat counts for a device.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceCounts {
    /// How many threats are not mitigated. (required)
    pub not_mitigated_count: i64,
    /// How many threats is in conditional access. (required)
    pub conditional_access_count: i64,
    /// How many threats are resolved by admin. (required)
    pub admin_resolved_count: i64,
    /// How many threats are mitigated. (required)
    pub mitigated_count: i64,
}

// ===========================================================================
// Incidents (GET /web/api/v2.1/mobile-integration/incidents)
// ===========================================================================

/// A mobile incident entity returned by the incidents list endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Incident {
    /// Id. (required)
    pub id: i64,
    /// A reference to the containing account. (required)
    pub account_id: String,
    /// Name of the containing account. (required)
    pub account_name: String,
    /// A reference to the containing site. (required)
    pub site_id: String,
    /// Name of the containing site. (required)
    pub site_name: String,
    /// A reference to the containing network group. (required)
    pub group_id: String,
    /// Name of the containing network group. (required)
    pub group_name: String,
    /// Device ID. (required)
    pub device_id: String,
    /// Tracking ID 1. (required)
    pub tracking_id1: String,
    /// Tracking ID 2. (required)
    pub tracking_id2: String,
    /// Kind. (required)
    pub kind: String,
    /// Type. (required)
    #[serde(rename = "type")]
    pub type_: String,
    /// Status. (required)
    pub status: String,
    /// Status action. (required)
    pub status_action: String,
    /// Incident status. (required)
    pub incident_status: String,
    /// Severity. (required)
    pub severity: String,
    /// Analyst verdict. (required)
    pub analyst_verdict: String,
    /// Investigation. (required)
    pub investigation: String,
    /// Description. (required)
    pub description: String,
    /// Detection type. (required)
    pub detection_type: String,
    /// Detection engine. (required)
    pub detection_engine: String,
    /// OS type. (required)
    pub os_type: String,
    /// OS version. (required)
    pub os_version: String,
    /// User email. (required)
    pub user_email: String,
    /// User notified. (required)
    pub user_notified: bool,
    /// Remediation step. (required)
    pub remediation_step: String,
    /// Reported time. (required, date-time string)
    pub reported_time: String,
    /// Incident details. (required)
    pub details: IncidentDetails,
    /// Notes. (required)
    pub notes: Vec<IncidentNote>,
    /// Network. (optional)
    pub network: Option<String>,
    /// Detail. (optional)
    pub detail: Option<String>,
}

/// Detail fields for an incident.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncidentDetails {
    /// Application developer. (required)
    pub application_developer: String,
    /// Malware threat name. (required)
    pub malware_threat_name: String,
    /// File name. (required)
    pub file_name: String,
    /// Application name. (required)
    pub application_name: String,
    /// Application package. (required)
    pub application_package: String,
    /// Suspected URL. (required)
    pub suspected_url: String,
    /// Process name. (required)
    pub process_name: String,
    /// Device time. (required, date-time string)
    pub device_time: String,
    /// Network interface. (required)
    pub network_interface: String,
    /// File hash. (required)
    pub file_hash: String,
    /// Installer source. (required)
    pub installer_source: String,
    /// Router SSID. (optional)
    #[serde(rename = "routerSSID")]
    pub router_ssid: Option<String>,
    /// Router BSSID. (optional)
    #[serde(rename = "routerBSSID")]
    pub router_bssid: Option<String>,
}

/// A note attached to an incident.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IncidentNote {
    /// Note ID. (required)
    pub id: i64,
    /// Author display name. (required)
    pub author: String,
    /// Author ID. (required)
    pub author_id: String,
    /// Note text. (required)
    pub text: String,
    /// Creation timestamp. (required, date-time string)
    pub created_at: String,
    /// Last update timestamp. (required, date-time string)
    pub updated_at: String,
    /// Whether the note was edited. (required)
    pub edited: bool,
}

// ===========================================================================
// Provisioning users (shared by tenant & MSSP responses)
// ===========================================================================

/// A user (admin user or tenant user) within a provisioning response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvisioningUser {
    /// User ID. (required)
    pub id: String,
    /// User's email address. (required)
    pub email: String,
    /// User's first name. (required)
    pub first_name: String,
    /// User's last name. (required)
    pub last_name: String,
    /// User's creation date. (required, date-time string)
    pub created: String,
    /// User's role. (required)
    pub role: ProvisioningUserRole,
    /// Notification email for sending details. (optional)
    pub notification_email: Option<String>,
}

/// The role of a [`ProvisioningUser`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProvisioningUserRole {
    /// Role ID. (required)
    pub id: String,
    /// Role name. (required)
    pub name: String,
}

// ===========================================================================
// Tenant (GET /web/api/v2.1/mobile-integration/provisioning/tenant)
// ===========================================================================

/// A tenant with its users.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TenantWithUsers {
    /// Tenant ID. (required)
    pub id: String,
    /// Tenant name. (required)
    pub name: String,
    /// Tenant users. (optional)
    pub users: Option<Vec<ProvisioningUser>>,
    /// Tenant admin user. (optional)
    pub admin_user: Option<ProvisioningUser>,
}

// ===========================================================================
// MSSP partner (GET /web/api/v2.1/mobile-integration/mssp-provisioning/partner)
// ===========================================================================

/// An MSSP partner with its admin user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MsspWithUsers {
    /// MSSP partner ID. (required)
    pub id: String,
    /// MSSP partner name. (required)
    pub name: String,
    /// MSSP partner admin user. (optional)
    pub admin_user: Option<ProvisioningUser>,
}

// ===========================================================================
// Can-provision-tenant
// (GET /web/api/v2.1/mobile-integration/provisioning/can-provision-tenant)
// ===========================================================================

/// Whether a tenant can be provisioned for the requested scope.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanProvisionTenant {
    /// Can provision tenant. (required)
    pub can_provision: bool,
    /// Is MSSP scope. (optional)
    pub mssp_scope: Option<bool>,
    /// Reason for not being able to provision tenant. (optional)
    pub reason: Option<String>,
    /// Reason code for not being able to provision tenant. (optional)
    pub reason_code: Option<String>,
    /// Is under MSSP scope. (optional)
    #[serde(rename = "underMSSPScope")]
    pub under_mssp_scope: Option<bool>,
    /// Affecting scopes. (optional)
    pub affecting_scopes: Option<Vec<CanProvisionScope>>,
}

/// A scope that affects whether provisioning is allowed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CanProvisionScope {
    /// Scope ID. (required)
    pub id: String,
    /// Scope level. (required)
    pub level: String,
}

// ===========================================================================
// Partner key (GET /web/api/v2.1/mobile-integration/provisioning/partner-key)
// ===========================================================================

/// An MSSP partner key (client ID only; secret is never returned).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartnerKey {
    /// Partner client ID. (required)
    pub client_id: String,
}
