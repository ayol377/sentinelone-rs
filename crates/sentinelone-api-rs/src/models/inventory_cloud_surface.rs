//! Models for the `Inventory Cloud Surface` tag.
//!
//! Field nullability mirrors the SentinelOne `swagger_2_1.json` spec: a field is
//! a bare type only when it appears in the schema `required` array and is not
//! `x-nullable`; otherwise it is `Option<T>` (the spec's "default null"
//! behaviour). Enum-valued strings are kept as `String` for forward
//! compatibility, with the allowed values documented on each field.

use serde::Deserialize;

/// A cloud surface asset (`CloudResponse` in the spec).
///
/// Returned by `GET /web/api/v2.1/xdr/assets/surface/cloud`. No field is marked
/// required in the spec, so every field is `Option`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudSurfaceAsset {
    /// Id.
    pub id: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Category.
    pub category: Option<String>,
    /// Sub category.
    pub sub_category: Option<String>,
    /// Resource type.
    pub resource_type: Option<String>,
    /// S1 group name.
    pub s1_group_name: Option<String>,
    /// S1 group id.
    pub s1_group_id: Option<String>,
    /// S1 site name.
    pub s1_site_name: Option<String>,
    /// S1 site id.
    pub s1_site_id: Option<String>,
    /// S1 account name.
    pub s1_account_name: Option<String>,
    /// S1 account id.
    pub s1_account_id: Option<String>,
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
    /// S1 onboarded group id.
    pub s1_onboarded_group_id: Option<i64>,
    /// S1 onboarded group name.
    pub s1_onboarded_group_name: Option<String>,
    /// S1 onboarded site id.
    pub s1_onboarded_site_id: Option<i64>,
    /// S1 onboarded site name.
    pub s1_onboarded_site_name: Option<String>,
    /// S1 onboarded scope id.
    pub s1_onboarded_scope_id: Option<i64>,
    /// S1 onboarded scope level.
    pub s1_onboarded_scope_level: Option<String>,
    /// S1 onboarded scope path.
    pub s1_onboarded_scope_path: Option<String>,
    /// Asset criticality.
    pub asset_criticality: Option<String>,
    /// Asset status.
    pub asset_status: Option<String>,
    /// Asset environment.
    pub asset_environment: Option<String>,
    /// Asset contact email.
    pub asset_contact_email: Option<String>,
    /// Infection status.
    pub infection_status: Option<String>,
    /// Device review.
    pub device_review: Option<String>,
    /// Device review log.
    pub device_review_log: Option<Vec<DeviceReviewLog>>,
    /// Is rogue.
    pub is_rogue: Option<bool>,
    /// State.
    pub state: Option<String>,
    /// Created time (date/time string).
    pub created_time: Option<String>,
    /// Realtime malware protection enabled time (date/time string).
    pub realtime_malware_protection_enabled_time: Option<String>,
    /// Region.
    pub region: Option<String>,
    /// Instance id.
    pub instance_id: Option<String>,
    /// Instance type.
    pub instance_type: Option<String>,
    /// Instance role.
    pub instance_role: Option<String>,
    /// Image id.
    pub image_id: Option<String>,
    /// Virtual network id.
    pub virtual_network_id: Option<String>,
    /// Subnet id.
    pub subnet_id: Option<Vec<String>>,
    /// Network security groups.
    pub network_security_groups: Option<Vec<String>>,
    /// Encryption type.
    pub encryption_type: Option<Vec<String>>,
    /// Allows unencrypted object uploads.
    pub allows_unencrypted_object_uploads: Option<bool>,
    /// Object count.
    pub object_count: Option<i64>,
    /// Policy document.
    pub policy_document: Option<String>,
    /// Surfaces.
    pub surfaces: Option<Vec<String>>,
    /// Missing coverage.
    pub missing_coverage: Option<Vec<String>>,
    /// Active coverage.
    pub active_coverage: Option<Vec<String>>,
    /// Risk factors.
    pub risk_factors: Option<Vec<String>>,
    /// Cloud resource id.
    pub cloud_resource_id: Option<String>,
    /// Cloud resource uid.
    pub cloud_resource_uid: Option<String>,
    /// Cloud provider account id.
    pub cloud_provider_account_id: Option<String>,
    /// Cloud provider account name.
    pub cloud_provider_account_name: Option<String>,
    /// Cloud provider subscription id.
    pub cloud_provider_subscription_id: Option<String>,
    /// Cloud provider project id.
    pub cloud_provider_project_id: Option<String>,
    /// Cloud provider resource group.
    pub cloud_provider_resource_group: Option<String>,
    /// Cloud provider organization.
    pub cloud_provider_organization: Option<String>,
    /// Cloud provider organization unit.
    pub cloud_provider_organization_unit: Option<String>,
    /// Cloud provider organization unit path.
    pub cloud_provider_organization_unit_path: Option<String>,
    /// Cloud provider url string.
    pub cloud_provider_url_string: Option<String>,
    /// Cloud tags (freeform objects).
    pub cloud_tags: Option<Vec<serde_json::Value>>,
    /// Tags (freeform objects).
    pub tags: Option<Vec<serde_json::Value>>,
    /// Source json (freeform object).
    pub source_json: Option<serde_json::Value>,
    /// Alerts.
    pub alerts: Option<Vec<AlertResponse>>,
    /// Alerts count.
    pub alerts_count: Option<Vec<AlertResponse>>,
    /// Notes.
    pub notes: Option<Vec<NotesResponse>>,
    /// Id secondary.
    pub id_secondary: Option<Vec<String>>,
    /// Kubernetes cluster.
    pub k8s_cluster: Option<String>,
    /// Kubernetes cluster id.
    pub k8s_cluster_id: Option<String>,
    /// Kubernetes namespace.
    pub k8s_namespace: Option<String>,
    /// Kubernetes node.
    pub k8s_node: Option<String>,
    /// Kubernetes node labels.
    pub k8s_node_labels: Option<Vec<String>>,
    /// Kubernetes type.
    pub k8s_type: Option<String>,
    /// Kubernetes version.
    pub k8s_version: Option<String>,
    /// Kubernetes resource id.
    pub k8s_resource_id: Option<String>,
    /// Kubernetes running on nodes.
    pub k8s_running_on_nodes: Option<Vec<String>>,
    /// Kubernetes annotations.
    pub k8s_annotations: Option<Vec<String>>,
    /// Kubernetes annotations unified (freeform objects).
    pub k8s_annotations_unified: Option<Vec<serde_json::Value>>,
    /// Kubernetes labels unified (freeform objects).
    pub k8s_labels_unified: Option<Vec<serde_json::Value>>,
    /// Kubernetes source json (freeform object).
    pub k8s_source_json: Option<serde_json::Value>,
    /// Kubernetes CWS enabled.
    pub k8s_cws_enabled: Option<bool>,
    /// Kubernetes CNS enabled.
    pub k8s_cns_enabled: Option<bool>,
    /// Kubernetes helper agent deployed.
    pub k8s_helper_agent_deployed: Option<bool>,
}

/// A single device-review log entry (`DeviceReviewLog` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceReviewLog {
    /// Updated time (date/time string).
    pub updated_time_dt: Option<String>,
    /// Updated time (epoch).
    pub updated_time: Option<i64>,
    /// Username.
    pub username: Option<String>,
    /// Previous value.
    pub previous: Option<String>,
    /// Current value.
    pub current: Option<String>,
    /// Reason.
    pub reason: Option<String>,
}

/// An alert associated with a cloud asset (`AlertResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlertResponse {
    /// Id.
    pub id: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Classification.
    pub classification: Option<String>,
    /// Severity.
    pub severity: Option<String>,
    /// Status.
    pub status: Option<String>,
    /// Activity.
    pub activity: Option<String>,
    /// Detected at (date/time string).
    pub detected_at: Option<String>,
    /// Time (epoch).
    pub time: Option<i64>,
    /// Count.
    pub count: Option<i64>,
    /// Attacks.
    pub attacks: Option<Vec<AttackResponse>>,
}

/// An attack referenced by an alert (`AttackResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackResponse {
    /// Uid.
    pub uid: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Version.
    pub version: Option<String>,
}

/// A note attached to a cloud asset (`NotesResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotesResponse {
    /// Id.
    pub id: Option<String>,
    /// Resource id.
    pub resource_id: Option<String>,
    /// User id.
    pub user_id: Option<String>,
    /// User name.
    pub user_name: Option<String>,
    /// Note.
    pub note: Option<String>,
    /// Created at (date/time string).
    pub created_at: Option<String>,
    /// Updated at (date/time string).
    pub updated_at: Option<String>,
}

/// Response wrapping the available actions and their status
/// (`AvailableActionWithStatusResponse` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions.
    pub available_actions: Option<Vec<AvailableAction>>,
}

/// A single available action with its enabled/disabled status
/// (`AvailableAction1` in the spec).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableAction {
    /// Action name. Allowed values: `Inventory`, `enable_protection`,
    /// `disable_protection`, `start_full_scan`, `stop_full_scan`,
    /// `enable_cws_monitoring`, `enable_cns`, `disable_cns`, `start_vm_scan`,
    /// `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`,
    /// `add_note`, `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`,
    /// `clear_tags`.
    pub name: Option<String>,
    /// Whether the action is disabled.
    pub is_disabled: Option<bool>,
    /// Reason the action is disabled, if applicable.
    pub disabled_reason: Option<String>,
}
