//! Models for the `ISPM` (Identity Security Posture Management) tag.
//!
//! Field nullability/optionality mirrors the SentinelOne `swagger_2_1.json`
//! spec exactly: a field is a bare type only when it is in the schema
//! `required` array and not `x-nullable`; otherwise it is `Option<T>`.

use serde::Deserialize;

// ---------------------------------------------------------------------------
// GET /web/api/v2.1/ranger-ad/assessment-status
// ---------------------------------------------------------------------------

/// AD assessment status for an account (`data` object of
/// `GET /web/api/v2.1/ranger-ad/assessment-status`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentStatus {
    /// List of assessment status for Azure tenants. Optional/nullable.
    pub tenant_wise_current_status_list: Option<Vec<AssessmentStatusTenant>>,
    /// Overall assessment status. Optional/nullable.
    ///
    /// Allowed values: `PENDING`, `IN_PROGRESS`, `COMPLETED`.
    pub status: Option<String>,
    /// List of assessment status for AD domains. Optional/nullable.
    pub domain_wise_current_status_list: Option<Vec<AssessmentStatusDomain>>,
}

/// Per-Azure-tenant assessment status entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentStatusTenant {
    /// Whether the assessment is completed for this tenant. Optional/nullable.
    pub tenant_completed_status: Option<bool>,
    /// Total number of assessment jobs for this tenant. Optional/nullable.
    pub total_jobs: Option<i64>,
    /// Unique identifier of the Azure tenant. Optional/nullable.
    pub tenant_id: Option<String>,
    /// Number of assessment jobs completed for this tenant. Optional/nullable.
    pub completed_jobs: Option<i64>,
}

/// Per-AD-domain assessment status entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssessmentStatusDomain {
    /// Total number of assessment jobs for this domain. Optional/nullable.
    pub total_jobs: Option<i64>,
    /// Name of the Active Directory forest containing the domain. Optional/nullable.
    pub forest_name: Option<String>,
    /// Whether the assessment is completed for this domain. Optional/nullable.
    pub domain_completed_status: Option<bool>,
    /// Number of assessment jobs completed for this domain. Optional/nullable.
    pub completed_jobs: Option<i64>,
    /// Name of the Active Directory domain. Optional/nullable.
    pub domain_name: Option<String>,
}

// ---------------------------------------------------------------------------
// POST /web/api/v2.1/ranger-ad/get-exposures
// ---------------------------------------------------------------------------

/// An AD/Azure exposure (one item of the `data` array of
/// `POST /web/api/v2.1/ranger-ad/get-exposures`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exposure {
    /// Numeric identifier of the detection rule. Required.
    pub detection_id: i64,
    /// Whether the exposure can be automatically remediated. Required.
    pub remediable: bool,
    /// Source of the detection. Required.
    ///
    /// Allowed values: `OnPremAD`, `AzureAD`.
    pub source: String,
    /// Human-readable reason why the exposure was skipped. Optional/nullable.
    pub skip_reason: Option<String>,
    /// Number of vulnerable objects from the previous assessment. Optional/nullable.
    pub prev_vulnerable_count: Option<i64>,
    /// Active Directory forest containing the domain. Required.
    pub forest_name: String,
    /// Severity level of the exposure. Required.
    ///
    /// Allowed values: `Critical`, `High`, `Medium`, `Low`.
    pub severity: String,
    /// Unix timestamp of the re-run, if applicable. Optional/nullable.
    pub special_run_timestamp: Option<i64>,
    /// Unix timestamp when the detection was last run. Required.
    pub run_timestamp: i64,
    /// Whether the re-run was performed. Optional/nullable.
    pub special_run: Option<bool>,
    /// Unique identifier for the exposure. Required.
    pub id: String,
    /// Current status of the exposure detection. Required.
    ///
    /// Allowed values: `Vulnerable`, `Not_Vulnerable`, `Skipped`,
    /// `In_Progress`, `Pending`, `Mitigated`.
    pub detection_status: String,
    /// Number of vulnerable objects affected by this exposure. Required.
    pub vulnerable_count: i64,
    /// Whether the exposure has been acknowledged by a user. Required.
    pub acknowledged: bool,
    /// Name of the detection rule that identified this exposure. Required.
    pub detection_name: String,
    /// Code indicating why the exposure was skipped, if applicable. Optional/nullable.
    pub skip_code: Option<String>,
    /// Whether the exposure contains objects that can be excluded. Optional/nullable.
    pub has_excludable_objects: Option<bool>,
    /// Active Directory domain where the exposure was found. Required.
    pub domain_name: String,
}

// ---------------------------------------------------------------------------
// POST /web/api/v2.1/ranger-ad/get-affected-objects
// ---------------------------------------------------------------------------

/// An object affected by an exposure (one item of the `data` array of
/// `POST /web/api/v2.1/ranger-ad/get-affected-objects`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedObject {
    /// Unique identifier for the affected object. Required.
    pub id: i64,
    /// Identifier of the detection run that found this object. Required.
    pub run_id: i64,
    /// Distinguished Name (DN) of the object in Active Directory. Optional/nullable.
    pub dn: Option<String>,
    /// Number of bad password attempts. Optional/nullable.
    pub bad_pwd_count: Option<i64>,
    /// Whether the user is a guest user in Azure AD. Optional/nullable.
    pub guest_user: Option<bool>,
    /// Unix timestamp of the last logon. Optional/nullable.
    pub last_logon_timestamp: Option<i64>,
    /// Name of the Certificate Authority. Optional/nullable.
    pub cert_authority_name: Option<String>,
    /// End user consent settings for the application. Optional/nullable.
    pub end_user_consent_setting: Option<String>,
    /// Name of the permission granted. Optional/nullable.
    pub permission_name: Option<String>,
    /// Description of the Azure AD object. Optional/nullable.
    pub description: Option<String>,
    /// Multi-factor authentication configuration for joined devices. Optional/nullable.
    pub join_device_mfa_config: Option<String>,
    /// Permission level or access rights. Optional/nullable.
    pub permission: Option<String>,
    /// Reserved attribute field for exposure specific data. Optional/nullable.
    pub attr1: Option<String>,
    /// User Principal Name of the object. Optional/nullable.
    pub upn: Option<String>,
    /// DNS hostname of the computer object. Optional/nullable.
    #[serde(rename = "dNSHostName")]
    pub d_ns_host_name: Option<String>,
    /// Certificate Authority that issued the certificate. Optional/nullable.
    pub certificate_authority: Option<String>,
    /// Unix timestamp when the object was created. Optional/nullable.
    pub when_created: Option<i64>,
    /// Unix timestamp when the password expires. Optional/nullable.
    pub password_expiry_time: Option<i64>,
    /// Certificate template name. Optional/nullable.
    pub template_name: Option<String>,
    /// Group Policy Container file system path. Optional/nullable.
    #[serde(rename = "gPCFileSysPath")]
    pub g_pc_file_sys_path: Option<String>,
    /// Current status of the user account (enabled/disabled). Optional/nullable.
    pub account_status: Option<String>,
    /// Service Principal Name (SPN) associated with the account. Optional/nullable.
    pub spn: Option<String>,
    /// Display name of the Active Directory object. Optional/nullable.
    pub display_name: Option<String>,
    /// Reserved attribute field for exposure specific data. Optional/nullable.
    pub attr7: Option<String>,
    /// Whether the user is registered for multi-factor authentication. Optional/nullable.
    pub mfa_registered: Option<bool>,
    /// Name of the container holding the object. Optional/nullable.
    pub container_name: Option<String>,
    /// Type of Active Directory object (User, Computer, Group, etc.). Optional/nullable.
    pub object_type: Option<String>,
    /// Reserved attribute field for exposure specific data. Optional/nullable.
    pub attr2: Option<String>,
    /// Certificate Authority server name. Optional/nullable.
    pub ca_server_name: Option<String>,
    /// Subject name of the certificate. Optional/nullable.
    pub cert_subject_name: Option<String>,
    /// Unix timestamp when password was last set. Optional/nullable.
    pub pwd_last_set: Option<i64>,
    /// Security Account Manager (SAM) account name. Optional/nullable.
    pub sam_account_name: Option<String>,
    /// Unix timestamp of the last bad password attempt. Optional/nullable.
    pub bad_password_time: Option<i64>,
    /// Access level granted to guest users. Optional/nullable.
    pub guest_user_access: Option<String>,
    /// Serial number of the certificate. Optional/nullable.
    pub cert_serial_number: Option<String>,
    /// Resource to which the permission applies. Optional/nullable.
    pub resource: Option<String>,
    /// Unix timestamp when the object was last modified. Optional/nullable.
    pub when_changed: Option<i64>,
    /// Managed Service Account (MSA) group membership. Optional/nullable.
    pub msds_group_msa_membership: Option<String>,
    /// Type of permission (Application, Delegated, etc.). Optional/nullable.
    pub permission_type: Option<String>,
    /// Reserved attribute field for exposure specific data. Optional/nullable.
    pub attr3: Option<String>,
    /// Whether the account is currently locked out. Optional/nullable.
    pub lockout_status: Option<bool>,
    /// Username of the Azure AD user. Optional/nullable.
    pub username: Option<String>,
    /// Certificate Authority issuer name. Optional/nullable.
    pub ca_issuer_name: Option<String>,
    /// Unix timestamp when the object was first detected. Optional/nullable.
    pub first_seen: Option<i64>,
    /// Common name (CN) of the Active Directory object. Optional/nullable.
    pub common_name: Option<String>,
    /// Unix timestamp when the detection was run. Optional/nullable.
    pub run_time: Option<i64>,
    /// Globally Unique Identifier (GUID) of the object. Optional/nullable.
    #[serde(rename = "objectGUID")]
    pub object_guid: Option<String>,
    /// Reserved attribute field for exposure specific data. Optional/nullable.
    pub attr5: Option<String>,
    /// Reserved attribute field for exposure specific data. Optional/nullable.
    pub attr4: Option<String>,
    /// Operating system information for computer objects. Optional/nullable.
    pub os: Option<String>,
    /// Security Identifier (SID) of the object. Optional/nullable.
    pub object_sid: Option<String>,
    /// Type of Active Directory group (Security, Distribution, etc.). Optional/nullable.
    pub group_type: Option<String>,
    /// Reserved attribute field for exposure specific data. Optional/nullable.
    pub attr6: Option<String>,
    /// Delegation settings for the account. Optional/nullable.
    pub delegation: Option<String>,
}

// ---------------------------------------------------------------------------
// Shared success envelope for trigger / set-skipped / set-ack
// ---------------------------------------------------------------------------

/// Generic success message returned by ISPM mutation endpoints
/// (`data` object of trigger-assessment, set-skipped-exposures, set-ack-status).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessMessage {
    /// Indicates a successful operation. Optional/nullable.
    pub success: Option<bool>,
    /// Success message describing the completed operation. Optional/nullable.
    pub message: Option<String>,
}
