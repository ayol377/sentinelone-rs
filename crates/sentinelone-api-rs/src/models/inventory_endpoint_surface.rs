//! Models for the `Inventory Endpoint Surface` tag.
//!
//! Generated at 1:1 parity with `swagger_2_1.json`. Every field is optional
//! (`Option<T>`) unless it appears in the schema `required` array and is not
//! `x-nullable`; enum-typed fields are represented as `String` for
//! forward-compatibility (allowed values are documented in the doc comments).

use serde::Deserialize;

/// An inventory endpoint asset.
///
/// Definition: `EndpointResponse`. No fields are marked required in the spec,
/// so every field is `Option<T>` ("default null" behaviour).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointResponse {
    /// S1 group name.
    pub s1_group_name: Option<String>,
    /// Cpu.
    pub cpu: Option<String>,
    /// Legacy identity policy name.
    pub legacy_identity_policy_name: Option<String>,
    /// Previous os type.
    pub previous_os_type: Option<String>,
    /// Previous os version.
    pub previous_os_version: Option<String>,
    /// Missing coverage.
    pub missing_coverage: Option<Vec<String>>,
    /// Tags (freeform objects).
    pub tags: Option<Vec<serde_json::Value>>,
    /// Applications installed on the asset.
    pub applications: Option<Vec<EndpointApplication>>,
    /// S1 updated at (date/time string).
    pub s1_updated_at: Option<String>,
    /// Memory in human readable format.
    pub memory_readable: Option<String>,
    /// S1 management id.
    pub s1_management_id: Option<i64>,
    /// Subnets.
    pub subnets: Option<Vec<String>>,
    /// Is ad connector.
    pub is_ad_connector: Option<bool>,
    /// Os.
    pub os: Option<String>,
    /// Agent details.
    pub agent: Option<EndpointAgent>,
    /// Network name.
    pub network_name: Option<String>,
    /// S1 site id.
    pub s1_site_id: Option<String>,
    /// Surfaces. Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    pub surfaces: Option<Vec<String>>,
    /// S1 site name.
    pub s1_site_name: Option<String>,
    /// Serial number.
    pub serial_number: Option<String>,
    /// Detected from site.
    pub detected_from_site: Option<String>,
    /// Epp unsupported unknown.
    pub epp_unsupported_unknown: Option<String>,
    /// Device review log.
    pub device_review_log: Option<Vec<EndpointDeviceReviewLog>>,
    /// Category.
    pub category: Option<String>,
    /// S1 group id.
    pub s1_group_id: Option<String>,
    /// Alerts.
    pub alerts: Option<Vec<EndpointAlert>>,
    /// Ip address.
    pub ip_address: Option<String>,
    /// Last active dt (date/time string).
    pub last_active_dt: Option<String>,
    /// S1 onboarded account id.
    pub s1_onboarded_account_id: Option<i64>,
    /// Network names.
    pub network_names: Option<Vec<String>>,
    /// Asset contact email.
    pub asset_contact_email: Option<String>,
    /// Risk factors. Allowed values: `Unresolved Alerts`, `High Value`.
    pub risk_factors: Option<Vec<String>>,
    /// S1 scope type.
    pub s1_scope_type: Option<i64>,
    /// Asset environment - AWS | Azure | GCP | Active Directory.
    pub asset_environment: Option<String>,
    /// Previous device function.
    pub previous_device_function: Option<String>,
    /// Os family.
    pub os_family: Option<String>,
    /// S1 account id.
    pub s1_account_id: Option<String>,
    /// Alerts count.
    pub alerts_count: Option<Vec<EndpointAlert>>,
    /// Ads enabled.
    pub ads_enabled: Option<bool>,
    /// Id.
    pub id: Option<String>,
    /// S1 account name.
    pub s1_account_name: Option<String>,
    /// S1 scope level.
    pub s1_scope_level: Option<String>,
    /// Network interfaces.
    pub network_interfaces: Option<Vec<EndpointNetworkInterface>>,
    /// Memory.
    pub memory: Option<i64>,
    /// S1 onboarded account name.
    pub s1_onboarded_account_name: Option<String>,
    /// S1 onboarded scope level.
    pub s1_onboarded_scope_level: Option<String>,
    /// Is dc server.
    pub is_dc_server: Option<bool>,
    /// Ranger tags.
    pub ranger_tags: Option<Vec<String>>,
    /// Id secondary.
    pub id_secondary: Option<Vec<String>>,
    /// Gateway ips.
    pub gateway_ips: Option<Vec<String>>,
    /// Name.
    pub name: Option<String>,
    /// Asset criticality. Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    pub asset_criticality: Option<String>,
    /// S1 scope path.
    pub s1_scope_path: Option<String>,
    /// Os version.
    pub os_version: Option<String>,
    /// S1 onboarded group name.
    pub s1_onboarded_group_name: Option<String>,
    /// Mac addresses.
    pub mac_addresses: Option<Vec<String>>,
    /// Internal ips.
    pub internal_ips: Option<Vec<String>>,
    /// Last reboot dt (date/time string).
    pub last_reboot_dt: Option<String>,
    /// S1 onboarded site name.
    pub s1_onboarded_site_name: Option<String>,
    /// Hostnames.
    pub hostnames: Option<Vec<String>>,
    /// S1 scope id.
    pub s1_scope_id: Option<String>,
    /// Manufacturer.
    pub manufacturer: Option<String>,
    /// Resource type (canonical resource type name).
    pub resource_type: Option<String>,
    /// Tcp ports.
    pub tcp_ports: Option<Vec<String>>,
    /// Core count.
    pub core_count: Option<i64>,
    /// Discovery methods.
    pub discovery_methods: Option<Vec<String>>,
    /// Infection status. Allowed values: `Infected`, `Healthy`.
    pub infection_status: Option<String>,
    /// Udp ports.
    pub udp_ports: Option<Vec<String>>,
    /// Active coverage. Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`,
    /// `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    pub active_coverage: Option<Vec<String>>,
    /// S1 onboarded site id.
    pub s1_onboarded_site_id: Option<i64>,
    /// Sub category.
    pub sub_category: Option<String>,
    /// S1 onboarded scope id.
    pub s1_onboarded_scope_id: Option<i64>,
    /// Notes.
    pub notes: Option<Vec<EndpointNote>>,
    /// Device review. Allowed values: `Not Reviewed`, `Under Analysis`,
    /// `Not Trusted`, `Allowed`, ``.
    pub device_review: Option<String>,
    /// Identity (AD) details.
    pub identity: Option<EndpointIdentity>,
    /// S1 onboarded scope path.
    pub s1_onboarded_scope_path: Option<String>,
    /// Gateway macs.
    pub gateway_macs: Option<Vec<String>>,
    /// Architecture.
    pub architecture: Option<String>,
    /// Asset status. Allowed values: `Active`, `Inactive`.
    pub asset_status: Option<String>,
    /// S1 onboarded group id.
    pub s1_onboarded_group_id: Option<i64>,
    /// First seen dt (date/time string).
    pub first_seen_dt: Option<String>,
    /// Domain.
    pub domain: Option<String>,
    /// Last update dt (date/time string).
    pub last_update_dt: Option<String>,
    /// Internal ips v6.
    pub internal_ips_v6: Option<Vec<String>>,
    /// Os name version.
    pub os_name_version: Option<String>,
}

/// An application installed on an endpoint asset.
///
/// Definition: `ApplicationResponse`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointApplication {
    /// Vendor name.
    pub vendor_name: Option<String>,
    /// Installed time dt (date/time string).
    pub installed_time_dt: Option<String>,
    /// Application name.
    pub name: Option<String>,
    /// Application version.
    pub version: Option<String>,
}

/// Agent details for an endpoint asset.
///
/// Definition: `AgentResponse`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointAgent {
    /// Installer type.
    pub installer_type: Option<String>,
    /// K8s namespace.
    pub k8s_namespace: Option<String>,
    /// Vss volumes.
    pub vss_volumes: Option<Vec<EndpointVssVolume>>,
    /// Has local config.
    pub has_local_config: Option<bool>,
    /// Disk encryption.
    pub disk_encryption: Option<bool>,
    /// Uninstalled.
    pub uninstalled: Option<bool>,
    /// Decommissioned.
    pub decommissioned: Option<bool>,
    /// Pending uninstall.
    pub pending_uninstall: Option<bool>,
    /// Ranger version (network scanner version).
    pub ranger_version: Option<String>,
    /// Customer identifier.
    pub customer_identifier: Option<String>,
    /// Missing permissions.
    pub missing_permissions: Option<Vec<String>>,
    /// Configurable network quarantine.
    pub configurable_network_quarantine: Option<bool>,
    /// Location.
    pub location: Option<Vec<String>>,
    /// Operational state.
    pub operational_state: Option<String>,
    /// Anti tampering status.
    pub anti_tampering_status: Option<String>,
    /// Disk metrics.
    pub disk_metrics: Option<Vec<EndpointDiskMetrics>>,
    /// Agent version.
    pub agent_version: Option<String>,
    /// Full disk scan dt (date/time string).
    pub full_disk_scan_dt: Option<String>,
    /// Pending actions.
    pub pending_actions: Option<Vec<String>>,
    /// Vss rollback status.
    pub vss_rollback_status: Option<String>,
    /// Console migration status.
    pub console_migration_status: Option<String>,
    /// Pending upgrade.
    pub pending_upgrade: Option<bool>,
    /// Uuid.
    pub uuid: Option<String>,
    /// Up to date.
    pub up_to_date: Option<bool>,
    /// Dv (SDL) connectivity.
    pub dv_connectivity: Option<String>,
    /// Health status.
    pub health_status: Option<String>,
    /// Console connectivity.
    pub console_connectivity: Option<bool>,
    /// Last logged in user.
    pub last_logged_in_user: Option<String>,
    /// Firewall status.
    pub firewall_status: Option<bool>,
    /// Operational state expiration time dt (date/time string).
    pub operational_state_expiration_time_dt: Option<String>,
    /// Vss last snapshot dt (date/time string).
    pub vss_last_snapshot_dt: Option<String>,
    /// Idr connectivity.
    pub idr_connectivity: Option<bool>,
    /// Location awareness.
    pub location_awareness: Option<bool>,
    /// Ranger status (network scanner status).
    pub ranger_status: Option<String>,
    /// Network status.
    pub network_status: Option<String>,
    /// Detection state.
    pub detection_state: Option<String>,
    /// Subscribe on dt (date/time string).
    pub subscribe_on_dt: Option<String>,
    /// Id.
    pub id: Option<String>,
    /// Vss protection status.
    pub vss_protection_status: Option<String>,
    /// K8s pod.
    pub k8s_pod: Option<String>,
    /// Dv connectivity last updated dt (date/time string).
    pub dv_connectivity_last_updated_dt: Option<String>,
    /// Vss service status.
    pub vss_service_status: Option<String>,
}

/// A VSS volume entry for an agent.
///
/// Definition: `VssVolume`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointVssVolume {
    /// Diff area current used bytes.
    pub diff_area_current_used_bytes: Option<i64>,
    /// Diff area name.
    pub diff_area_name: Option<String>,
    /// Diff area free percentage.
    pub diff_area_free_percentage: Option<f64>,
    /// Diff area current allocated bytes.
    pub diff_area_current_allocated_bytes: Option<i64>,
    /// Diff area max limit bytes.
    pub diff_area_max_limit_bytes: Option<i64>,
}

/// Disk metrics for an agent volume.
///
/// Definition: `DiskMetrics`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointDiskMetrics {
    /// Volume type.
    pub volume_type: Option<String>,
    /// Total number of free bytes.
    pub total_number_of_free_bytes: Option<i64>,
    /// Free bytes available to caller.
    pub free_bytes_available_to_caller: Option<i64>,
    /// Total number of bytes.
    pub total_number_of_bytes: Option<i64>,
    /// Free percentage.
    pub free_percentage: Option<f64>,
    /// Path.
    pub path: Option<String>,
}

/// A device review log entry.
///
/// Definition: `DeviceReviewLog`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointDeviceReviewLog {
    /// Updated time dt (date/time string).
    pub updated_time_dt: Option<String>,
    /// Username.
    pub username: Option<String>,
    /// Previous review value.
    pub previous: Option<String>,
    /// Current review value.
    pub current: Option<String>,
    /// Updated time (epoch).
    pub updated_time: Option<i64>,
    /// Reason.
    pub reason: Option<String>,
}

/// An alert associated with an endpoint asset.
///
/// Definition: `AlertResponse`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointAlert {
    /// Classification.
    pub classification: Option<String>,
    /// Detected at (date/time string).
    pub detected_at: Option<String>,
    /// Alert name.
    pub name: Option<String>,
    /// Count.
    pub count: Option<i64>,
    /// Attacks.
    pub attacks: Option<Vec<EndpointAttack>>,
    /// Severity.
    pub severity: Option<String>,
    /// Id.
    pub id: Option<String>,
    /// Time (epoch).
    pub time: Option<i64>,
    /// Status.
    pub status: Option<String>,
    /// Activity.
    pub activity: Option<String>,
}

/// An attack technique referenced by an alert.
///
/// Definition: `AttackResponse`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointAttack {
    /// Uid.
    pub uid: Option<String>,
    /// Name.
    pub name: Option<String>,
    /// Version.
    pub version: Option<String>,
}

/// A network interface on an endpoint asset.
///
/// Definition: `NetworkInterfaceResponse`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointNetworkInterface {
    /// Subnet.
    pub subnet: Option<String>,
    /// Ip.
    pub ip: Option<String>,
    /// Network name.
    pub network_name: Option<String>,
    /// Mac.
    pub mac: Option<String>,
    /// Gateway mac.
    pub gateway_mac: Option<String>,
    /// Interface name.
    pub name: Option<String>,
    /// Gateway ip.
    pub gateway_ip: Option<String>,
}

/// A note attached to an endpoint asset.
///
/// Definition: `NotesResponse`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointNote {
    /// Resource id.
    pub resource_id: Option<String>,
    /// User name.
    pub user_name: Option<String>,
    /// Created at (date/time string).
    pub created_at: Option<String>,
    /// Id.
    pub id: Option<String>,
    /// User id.
    pub user_id: Option<String>,
    /// Updated at (date/time string).
    pub updated_at: Option<String>,
    /// Note text.
    pub note: Option<String>,
}

/// AD identity details for an endpoint asset.
///
/// Definition: `IdentityResponse`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndpointIdentity {
    /// Ad machine membership.
    pub ad_machine_membership: Option<Vec<String>>,
    /// Ad machine distinguished name.
    pub ad_machine_distinguished_name: Option<String>,
    /// Ad user distinguished name.
    pub ad_user_distinguished_name: Option<String>,
    /// Ad user membership.
    pub ad_user_membership: Option<Vec<String>>,
}

/// Response payload for available actions with status.
///
/// Definition: `AvailableActionWithStatusResponse`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatusResponse {
    /// Available actions with their status.
    pub available_actions: Option<Vec<AvailableActionWithStatus>>,
}

/// A single available action with its enabled/disabled status.
///
/// Definition: `AvailableAction1`. All fields optional.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionWithStatus {
    /// Reason the action is disabled (if any).
    pub disabled_reason: Option<String>,
    /// Whether the action is disabled.
    pub is_disabled: Option<bool>,
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
}
