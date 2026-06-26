//! Models for the `Inventory Identity Surface` tag.
//!
//! Response entities for the XDR identity surface inventory endpoints. Every
//! field follows the SentinelOne "default null" convention: a field is bare `T`
//! only when the spec marks it `required` and not `x-nullable`; otherwise it is
//! `Option<T>`. None of the `IdentitySurfaceResponse` properties are listed in a
//! `required` array, so all are optional. Enum-valued fields are kept as
//! `String` for forward compatibility (allowed values are documented inline).

use serde::Deserialize;

/// A single identity surface asset returned by
/// `GET /web/api/v2.1/xdr/assets/surface/identity`.
///
/// Identity inventory asset describing an Active Directory / cloud identity
/// entity and its associated SentinelOne scope, coverage and risk metadata.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentitySurfaceResponse {
    /// Id.
    pub id: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Display name.
    pub display_name: Option<String>,
    /// S1 group name.
    pub s1_group_name: Option<String>,
    /// S1 group id.
    pub s1_group_id: Option<String>,
    /// S1 site id.
    pub s1_site_id: Option<String>,
    /// S1 site name.
    pub s1_site_name: Option<String>,
    /// S1 account id.
    pub s1_account_id: Option<String>,
    /// S1 account name.
    pub s1_account_name: Option<String>,
    /// S1 management id.
    pub s1_management_id: Option<i64>,
    /// S1 scope type.
    pub s1_scope_type: Option<i64>,
    /// S1 scope level.
    pub s1_scope_level: Option<String>,
    /// S1 scope id.
    pub s1_scope_id: Option<String>,
    /// S1 scope path.
    pub s1_scope_path: Option<String>,
    /// S1 updated at (date/time string).
    pub s1_updated_at: Option<String>,
    /// S1 onboarded account id.
    pub s1_onboarded_account_id: Option<i64>,
    /// S1 onboarded account name.
    pub s1_onboarded_account_name: Option<String>,
    /// S1 onboarded scope level.
    pub s1_onboarded_scope_level: Option<String>,
    /// S1 onboarded scope id.
    pub s1_onboarded_scope_id: Option<i64>,
    /// S1 onboarded scope path.
    pub s1_onboarded_scope_path: Option<String>,
    /// S1 onboarded group name.
    pub s1_onboarded_group_name: Option<String>,
    /// S1 onboarded group id.
    pub s1_onboarded_group_id: Option<i64>,
    /// S1 onboarded site name.
    pub s1_onboarded_site_name: Option<String>,
    /// S1 onboarded site id.
    pub s1_onboarded_site_id: Option<i64>,
    /// Recycled.
    pub recycled: Option<bool>,
    /// Group type.
    pub group_type: Option<i64>,
    /// Service principal name.
    pub service_principal_name: Option<Vec<String>>,
    /// Missing coverage. Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`,
    /// `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    pub missing_coverage: Option<Vec<String>>,
    /// Active coverage. Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`,
    /// `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    pub active_coverage: Option<Vec<String>>,
    /// Tags (freeform objects).
    pub tags: Option<Vec<serde_json::Value>>,
    /// Object category.
    pub object_category: Option<String>,
    /// Member.
    pub member: Option<Vec<String>>,
    /// Members guid.
    pub members_guid: Option<Vec<String>>,
    /// Member of.
    pub member_of: Option<Vec<String>>,
    /// Member of guid.
    pub member_of_guid: Option<Vec<String>>,
    /// Member of transitive.
    pub member_of_transitive: Option<Vec<String>>,
    /// Resultant pso.
    pub resultant_pso: Option<String>,
    /// Logon count.
    pub logon_count: Option<i64>,
    /// User principal name.
    pub user_principal_name: Option<String>,
    /// Allowed to delegate to.
    pub allowed_to_delegate_to: Option<Vec<String>>,
    /// Distinguished name.
    pub distinguished_name: Option<String>,
    /// Creator sid.
    pub creator_sid: Option<String>,
    /// Mail.
    pub mail: Option<String>,
    /// Last logon time (date/time string).
    pub last_logon_time: Option<String>,
    /// Object sid.
    pub object_sid: Option<String>,
    /// Consistency guid.
    pub consistency_guid: Option<String>,
    /// Sam account name.
    pub sam_account_name: Option<String>,
    /// Sam account type.
    pub sam_account_type: Option<i64>,
    /// Surfaces. Enum values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    pub surfaces: Option<Vec<String>>,
    /// Forest.
    pub forest: Option<String>,
    /// Privileged.
    pub privileged: Option<bool>,
    /// Device review log.
    pub device_review_log: Option<Vec<IdentitySurfaceDeviceReviewLog>>,
    /// Password never expire.
    pub password_never_expire: Option<bool>,
    /// Category.
    pub category: Option<String>,
    /// Alerts.
    pub alerts: Option<Vec<IdentitySurfaceAlert>>,
    /// Alerts count.
    pub alerts_count: Option<Vec<IdentitySurfaceAlert>>,
    /// Deleted.
    pub deleted: Option<bool>,
    /// Sid history.
    pub sid_history: Option<Vec<String>>,
    /// Last modified time (date/time string).
    pub last_modified_time: Option<String>,
    /// Parent dist name.
    pub parent_dist_name: Option<String>,
    /// Primary group id.
    pub primary_group_id: Option<i64>,
    /// Last known parent.
    pub last_known_parent: Option<String>,
    /// Cn (LDAP common name).
    pub cn: Option<String>,
    /// Password last set time (date/time string).
    pub password_last_set_time: Option<String>,
    /// Asset contact email.
    pub asset_contact_email: Option<String>,
    /// Risk factors. Enum values: `Unresolved Alerts`, `High Value`.
    pub risk_factors: Option<Vec<String>>,
    /// Asset environment - AWS | Azure | GCP | Active Directory.
    pub asset_environment: Option<String>,
    /// Bad password count.
    pub bad_password_count: Option<i64>,
    /// Account expires (date/time string).
    pub account_expires: Option<String>,
    /// Bad password time (date/time string).
    pub bad_password_time: Option<String>,
    /// Token groups.
    pub token_groups: Option<Vec<String>>,
    /// Created time (date/time string).
    pub created_time: Option<String>,
    /// Id secondary.
    pub id_secondary: Option<Vec<String>>,
    /// User account control.
    pub user_account_control: Option<i64>,
    /// Asset criticality. Enum values: `critical`, `high`, `medium`, `low`, `--`.
    pub asset_criticality: Option<String>,
    /// Resource type (canonical resource type name).
    pub resource_type: Option<String>,
    /// Service account.
    pub service_account: Option<bool>,
    /// Infection status. Enum values: `Infected`, `Healthy`.
    pub infection_status: Option<String>,
    /// Object guid.
    pub object_guid: Option<String>,
    /// Nt security descriptor.
    pub nt_security_descriptor: Option<String>,
    /// Allowed to act on behalf of other identity.
    pub allowed_to_act_on_behalf_of_other_identity: Option<bool>,
    /// Sub category.
    pub sub_category: Option<String>,
    /// Notes.
    pub notes: Option<Vec<IdentitySurfaceNote>>,
    /// Device review. Enum values: `Not Reviewed`, `Under Analysis`,
    /// `Not Trusted`, `Allowed`.
    pub device_review: Option<String>,
    /// Usn changed.
    pub usn_changed: Option<i64>,
    /// Logon hours.
    pub logon_hours: Option<String>,
    /// Object class.
    pub object_class: Option<Vec<String>>,
    /// Principal name.
    pub principal_name: Option<String>,
    /// Lock out time (date/time string).
    pub lock_out_time: Option<String>,
    /// Asset status. Enum values: `Active`, `Inactive`.
    pub asset_status: Option<String>,
    /// When changed (date/time string).
    pub when_changed: Option<String>,
    /// User password expiry time computed (date/time string).
    pub user_password_expiry_time_computed: Option<String>,
    /// Admin count.
    pub admin_count: Option<i64>,
    /// Usn created.
    pub usn_created: Option<i64>,
    /// Domain (AD domain name).
    pub domain: Option<String>,
    /// Enabled.
    pub enabled: Option<bool>,
}

/// A device review log entry on an [`IdentitySurfaceResponse`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentitySurfaceDeviceReviewLog {
    /// Updated time as a date/time string.
    pub updated_time_dt: Option<String>,
    /// Username that performed the review.
    pub username: Option<String>,
    /// Previous review state.
    pub previous: Option<String>,
    /// Current review state.
    pub current: Option<String>,
    /// Updated time (epoch).
    pub updated_time: Option<i64>,
    /// Reason for the review change.
    pub reason: Option<String>,
}

/// An alert associated with an [`IdentitySurfaceResponse`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentitySurfaceAlert {
    /// Alert id.
    pub id: Option<String>,
    /// Alert name.
    pub name: Option<String>,
    /// Classification.
    pub classification: Option<String>,
    /// Detected at (date/time string).
    pub detected_at: Option<String>,
    /// Count.
    pub count: Option<i64>,
    /// Attacks.
    pub attacks: Option<Vec<IdentitySurfaceAttack>>,
    /// Severity.
    pub severity: Option<String>,
    /// Time (epoch).
    pub time: Option<i64>,
    /// Status.
    pub status: Option<String>,
    /// Activity.
    pub activity: Option<String>,
}

/// An attack referenced by an [`IdentitySurfaceAlert`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentitySurfaceAttack {
    /// Attack uid.
    pub uid: Option<String>,
    /// Attack name.
    pub name: Option<String>,
    /// Attack version.
    pub version: Option<String>,
}

/// A note attached to an [`IdentitySurfaceResponse`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentitySurfaceNote {
    /// Note id.
    pub id: Option<String>,
    /// Resource id the note belongs to.
    pub resource_id: Option<String>,
    /// Author user name.
    pub user_name: Option<String>,
    /// Author user id.
    pub user_id: Option<String>,
    /// Created at (date/time string).
    pub created_at: Option<String>,
    /// Updated at (date/time string).
    pub updated_at: Option<String>,
    /// Note body.
    pub note: Option<String>,
}

/// Response data for
/// `POST /web/api/v2.1/xdr/assets/surface/identity/available-actions/with-status`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions, each with their enablement status.
    pub available_actions: Option<Vec<IdentitySurfaceAvailableAction>>,
}

/// A single available action with its current enablement status.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentitySurfaceAvailableAction {
    /// Action name. Enum values: `Inventory`, `enable_protection`,
    /// `disable_protection`, `start_full_scan`, `stop_full_scan`,
    /// `enable_cws_monitoring`, `enable_cns`, `disable_cns`, `start_vm_scan`,
    /// `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`,
    /// `add_note`, `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`,
    /// `clear_tags`.
    pub name: Option<String>,
    /// Whether the action is currently disabled.
    pub is_disabled: Option<bool>,
    /// Reason the action is disabled, when applicable.
    pub disabled_reason: Option<String>,
}
