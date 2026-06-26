//! Models for the `Inventory Network Discovery Surface` tag.
//!
//! Field nullability mirrors the SentinelOne spec: a field is a bare `T` only
//! when it is in the schema `required` array and not `x-nullable`; otherwise it
//! is `Option<T>` (default-null behaviour). Cross-tag `$ref`s and freeform
//! objects are typed as [`serde_json::Value`] for forward-compatibility.

use serde::Deserialize;

/// A single Network Discovery surface asset.
///
/// Spec entity: `NetworkDiscoveryResponse`. The schema declares no `required`
/// fields, so every field is optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDiscovery {
    /// Id.
    pub id: Option<String>,
    /// Id secondary.
    pub id_secondary: Option<Vec<String>>,
    /// Name.
    pub name: Option<String>,
    /// S1 group name.
    pub s1_group_name: Option<String>,
    /// S1 group id.
    pub s1_group_id: Option<String>,
    /// Cpu.
    pub cpu: Option<String>,
    /// Core count.
    pub core_count: Option<i64>,
    /// Memory (raw integer).
    pub memory: Option<i64>,
    /// Memory readable (human readable format).
    pub memory_readable: Option<String>,
    /// Legacy identity policy name.
    pub legacy_identity_policy_name: Option<String>,
    /// Previous os type.
    pub previous_os_type: Option<String>,
    /// Previous os version.
    pub previous_os_version: Option<String>,
    /// Previous device function.
    pub previous_device_function: Option<String>,
    /// Os.
    pub os: Option<String>,
    /// Os family.
    pub os_family: Option<String>,
    /// Os version.
    pub os_version: Option<String>,
    /// Os name version.
    pub os_name_version: Option<String>,
    /// Architecture.
    pub architecture: Option<String>,
    /// Manufacturer.
    pub manufacturer: Option<String>,
    /// Serial number.
    pub serial_number: Option<String>,
    /// Domain.
    pub domain: Option<String>,
    /// Network name.
    pub network_name: Option<String>,
    /// Network names.
    pub network_names: Option<Vec<String>>,
    /// Subnets.
    pub subnets: Option<Vec<String>>,
    /// Ip address.
    pub ip_address: Option<String>,
    /// Internal ips.
    pub internal_ips: Option<Vec<String>>,
    /// Internal ips v6.
    pub internal_ips_v6: Option<Vec<String>>,
    /// Gateway ips.
    pub gateway_ips: Option<Vec<String>>,
    /// Gateway macs.
    pub gateway_macs: Option<Vec<String>>,
    /// Mac addresses.
    pub mac_addresses: Option<Vec<String>>,
    /// Hostnames.
    pub hostnames: Option<Vec<String>>,
    /// Tcp ports.
    pub tcp_ports: Option<Vec<String>>,
    /// Udp ports.
    pub udp_ports: Option<Vec<String>>,
    /// Resource type.
    pub resource_type: Option<String>,
    /// Category.
    pub category: Option<String>,
    /// Sub category.
    pub sub_category: Option<String>,
    /// Asset criticality.
    ///
    /// Spec enum values: `critical`, `high`, `medium`, `low`, `--`.
    pub asset_criticality: Option<String>,
    /// Asset status.
    ///
    /// Spec enum values: `Active`, `Inactive`.
    pub asset_status: Option<String>,
    /// Asset environment (AWS | Azure | GCP | Active Directory).
    pub asset_environment: Option<String>,
    /// Asset contact email.
    pub asset_contact_email: Option<String>,
    /// Infection status.
    ///
    /// Spec enum values: `Infected`, `Healthy`.
    pub infection_status: Option<String>,
    /// Surfaces.
    ///
    /// Spec enum values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    pub surfaces: Option<Vec<String>>,
    /// Missing coverage.
    ///
    /// Spec enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    pub missing_coverage: Option<Vec<String>>,
    /// Active coverage.
    ///
    /// Spec enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    pub active_coverage: Option<Vec<String>>,
    /// Risk factors.
    ///
    /// Spec enum values: `Unresolved Alerts`, `High Value`.
    pub risk_factors: Option<Vec<String>>,
    /// Discovery methods.
    pub discovery_methods: Option<Vec<String>>,
    /// Detected from site.
    pub detected_from_site: Option<String>,
    /// Epp unsupported unknown.
    pub epp_unsupported_unknown: Option<String>,
    /// Device review.
    ///
    /// Spec enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`.
    pub device_review: Option<String>,
    /// Device review log.
    pub device_review_log: Option<Vec<NetworkDiscoveryDeviceReviewLog>>,
    /// Tags (freeform objects).
    pub tags: Option<Vec<serde_json::Value>>,
    /// Ranger tags.
    pub ranger_tags: Option<Vec<String>>,
    /// Is ad connector.
    pub is_ad_connector: Option<bool>,
    /// Is dc server.
    pub is_dc_server: Option<bool>,
    /// Ads enabled.
    pub ads_enabled: Option<bool>,
    /// First seen dt.
    pub first_seen_dt: Option<String>,
    /// Last active dt.
    pub last_active_dt: Option<String>,
    /// Last reboot dt.
    pub last_reboot_dt: Option<String>,
    /// Last update dt.
    pub last_update_dt: Option<String>,
    /// S1 updated at.
    pub s1_updated_at: Option<String>,
    /// S1 management id.
    pub s1_management_id: Option<i64>,
    /// S1 site id.
    pub s1_site_id: Option<String>,
    /// S1 site name.
    pub s1_site_name: Option<String>,
    /// S1 account id.
    pub s1_account_id: Option<String>,
    /// S1 account name.
    pub s1_account_name: Option<String>,
    /// S1 scope type.
    pub s1_scope_type: Option<i64>,
    /// S1 scope level.
    pub s1_scope_level: Option<String>,
    /// S1 scope id.
    pub s1_scope_id: Option<String>,
    /// S1 scope path.
    pub s1_scope_path: Option<String>,
    /// S1 onboarded account id.
    pub s1_onboarded_account_id: Option<i64>,
    /// S1 onboarded account name.
    pub s1_onboarded_account_name: Option<String>,
    /// S1 onboarded site id.
    pub s1_onboarded_site_id: Option<i64>,
    /// S1 onboarded site name.
    pub s1_onboarded_site_name: Option<String>,
    /// S1 onboarded group id.
    pub s1_onboarded_group_id: Option<i64>,
    /// S1 onboarded group name.
    pub s1_onboarded_group_name: Option<String>,
    /// S1 onboarded scope id.
    pub s1_onboarded_scope_id: Option<i64>,
    /// S1 onboarded scope level.
    pub s1_onboarded_scope_level: Option<String>,
    /// S1 onboarded scope path.
    pub s1_onboarded_scope_path: Option<String>,
    /// Agent (cross-tag `AgentResponse` ref; freeform).
    pub agent: Option<serde_json::Value>,
    /// Identity (cross-tag `IdentityResponse` ref; freeform).
    pub identity: Option<serde_json::Value>,
    /// Network interfaces (cross-tag `NetworkInterfaceResponse` refs; freeform).
    pub network_interfaces: Option<Vec<serde_json::Value>>,
    /// Alerts (cross-tag `AlertResponse` refs; freeform).
    pub alerts: Option<Vec<serde_json::Value>>,
    /// Alerts count (cross-tag `AlertResponse` refs; freeform).
    pub alerts_count: Option<Vec<serde_json::Value>>,
    /// Notes (cross-tag `NotesResponse` refs; freeform).
    pub notes: Option<Vec<serde_json::Value>>,
}

/// A single device-review audit entry for a Network Discovery asset.
///
/// Spec entity: `DeviceReviewLog`. No `required` fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDiscoveryDeviceReviewLog {
    /// Updated time dt.
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

/// Response wrapper for available actions with status.
///
/// Spec entity: `AvailableActionWithStatusResponse`. No `required` fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDiscoveryAvailableActions {
    /// Available actions.
    pub available_actions: Option<Vec<NetworkDiscoveryAvailableAction>>,
}

/// A single available action and its enablement status.
///
/// Spec entity: `AvailableAction1`. No `required` fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkDiscoveryAvailableAction {
    /// Action name.
    ///
    /// Spec enum values: `Inventory`, `enable_protection`,
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
    /// The reason the action is disabled, if applicable.
    pub disabled_reason: Option<String>,
}
