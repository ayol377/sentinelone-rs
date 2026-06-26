//! Models for the `Network Discovery` tag (the SentinelOne "ranger" endpoints).
//!
//! Field nullability mirrors `swagger_2_1.json` exactly: a field is a bare `T`
//! only when it is listed in the schema `required` array and is not
//! `x-nullable`; otherwise it is `Option<T>` (the API's "default null"
//! behaviour). Enum-valued fields are kept as `String` for forward-compat;
//! the allowed values are documented in each field's doc comment.

use serde::Deserialize;

/// A single row of the Network Discovery Device Inventory Table.
///
/// Returned by `GET /web/api/v2.1/ranger/table-view` (as a page of rows) and by
/// `PUT /web/api/v2.1/ranger/device-review/{inventory_id}` (as the updated row).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerDevice {
    /// Local ip of the device.
    pub local_ip: Option<String>,
    /// Id of the device.
    pub id: Option<String>,
    /// Os Type of the device.
    pub os_type: Option<String>,
    /// OS Name/Version of the device.
    pub os_name: Option<String>,
    /// OS Version of the device.
    pub os_version: Option<String>,
    /// Role of the device (e.g. `Server`).
    pub device_type: Option<String>,
    /// Main Gateway Visible IP.
    pub external_ip: Option<String>,
    /// Mac address of the device.
    pub mac_address: Option<String>,
    /// Time the device was first seen (ISO-8601 timestamp).
    pub first_seen: Option<String>,
    /// Time the device was last seen (ISO-8601 timestamp).
    pub last_seen: Option<String>,
    /// The agent id if this is a known managed device.
    pub agent_id: Option<String>,
    /// Protection state of the device (nullable).
    pub managed_state: Option<String>,
    /// Methods used to discover the device.
    pub discovery_methods: Option<Vec<String>>,
    /// Main subnet address.
    pub subnet_address: Option<String>,
    /// Main gateway IP address.
    pub gateway_ip_address: Option<String>,
    /// Main gateway MAC address.
    pub gateway_mac_address: Option<String>,
    /// TCP Ports.
    pub tcp_ports: Option<Vec<i64>>,
    /// UDP Ports.
    pub udp_ports: Option<Vec<i64>>,
    /// Array of host names.
    pub hostnames: Option<Vec<String>>,
    /// The confidence for this fingerprinting result.
    pub finger_print_score: Option<i64>,
    /// Manufacturer of the device or network interface.
    pub manufacturer: Option<String>,
    /// All the networks associated to the device. When not combined it is always one element.
    pub networks: Option<Vec<RangerDeviceNetwork>>,
    /// A list of ip addresses. When not combined it is always one element.
    pub ip_addresses: Option<Vec<String>>,
    /// The network name.
    pub network_name: Option<String>,
    /// The device review state.
    pub device_review: Option<String>,
    /// Would we be able to identify this device over time.
    pub has_identity: Option<bool>,
    /// Log of actions for this device.
    pub device_review_log: Option<Vec<RangerDeviceReviewLogEntry>>,
    /// Function of the device (e.g. `Server`).
    pub device_function: Option<String>,
    /// The tags.
    pub tags: Option<Vec<RangerDeviceTag>>,
    /// The domain of the device.
    pub domain: Option<String>,
    /// Site name.
    pub site_name: Option<String>,
    /// True if it has a user label.
    pub has_user_label: Option<bool>,
    /// Previous Function of the device if manually changed.
    pub previous_device_function: Option<String>,
    /// Previous Os Type of the device if manually changed.
    pub previous_os_type: Option<String>,
    /// Previous OS Version of the device if manually changed.
    pub previous_os_version: Option<String>,
    /// The user that changed the label.
    pub label_user_name: Option<String>,
    /// The date of the last label update (ISO-8601 timestamp).
    pub label_updated_at: Option<String>,
}

/// One network a [`RangerDevice`] is associated with.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerDeviceNetwork {
    /// Main subnet address.
    pub subnet_address: Option<String>,
    /// Main gateway MAC address.
    pub gateway_mac_address: Option<String>,
    /// Main gateway IP address.
    pub gateway_ip_address: Option<String>,
    /// Main Gateway Visible IP.
    pub external_ip: Option<String>,
    /// The IP of the device in the network.
    pub ip: Option<String>,
    /// The network name.
    pub network_name: Option<String>,
}

/// One entry in a device's review action log.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerDeviceReviewLogEntry {
    /// Previous review state.
    pub previous: Option<String>,
    /// Current review state.
    pub current: Option<String>,
    /// When the change was made.
    pub updated_at: Option<String>,
    /// User who made the change.
    pub username: Option<String>,
    /// Reason for the change.
    pub reason: Option<String>,
    /// Reason details for the change.
    pub reason_details: Option<String>,
}

/// A tag attached to a [`RangerDevice`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerDeviceTag {
    /// The tag id (required, not nullable).
    pub id: String,
    /// The tag name.
    pub name: Option<String>,
    /// The tag description (nullable).
    pub description: Option<String>,
    /// Kind of tag if relevant (e.g. `Vulnerability`; nullable).
    pub kind: Option<String>,
}

/// Network Discovery ("ranger") settings for an account/scope.
///
/// Returned by `GET` and `PUT /web/api/v2.1/ranger/settings`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerSettings {
    /// Network Discovery views are generated periodically by the snapshot period.
    pub use_periodic_snapshots: Option<bool>,
    /// Period in minutes for each snapshot.
    pub snapshot_period: Option<i64>,
    /// Is the Network Discovery collection enabled for the account.
    pub enabled: Option<bool>,
    /// The number of days to archive a network which was not enabled for scan (1-7).
    pub network_decommission_value: Option<i64>,
    /// Minimum agents required in a network to be listed as selectable for scan.
    pub min_agents_in_network_to_scan: Option<i64>,
    /// TCP Ports.
    pub tcp_ports: Option<Vec<i64>>,
    /// UDP Ports.
    pub udp_ports: Option<Vec<i64>>,
    /// TCP Port scan enabled.
    pub tcp_port_scan: Option<bool>,
    /// UDP Port scan enabled.
    pub udp_port_scan: Option<bool>,
    /// ICMP scan enabled.
    pub icmp_scan: Option<bool>,
    /// SMB scan enabled.
    pub smb_scan: Option<bool>,
    /// MDNS scan enabled.
    pub mdns_scan: Option<bool>,
    /// RDNS scan enabled.
    pub rdns_scan: Option<bool>,
    /// SNMP scan enabled.
    pub snmp_scan: Option<bool>,
    /// Networks are going to be marked as new for this period.
    pub new_network_in_hours: Option<i64>,
    /// Account id.
    pub account_id: Option<String>,
    /// Scope id.
    pub scope_id: Option<String>,
    /// Scan only local subnets.
    pub scan_only_local_subnets: Option<bool>,
    /// All networks that match the min agents configuration will be enabled automatically.
    pub auto_enable_networks: Option<bool>,
    /// Combine devices as one among multiple networks.
    pub combine_devices: Option<bool>,
    /// A set of IP addresses that should not be scanned in the specific network.
    pub restrictions: Option<Vec<RangerSettingsRestriction>>,
    /// SSDP Multicast scan enabled.
    pub multi_scan_ssdp: Option<bool>,
    /// DNS Full scan enabled.
    pub use_full_dns_scan: Option<bool>,
    /// \[FUTURE\] Use only specific ports defined in specific ports as source ports of active scans.
    pub use_specific_ports: Option<bool>,
    /// \[FUTURE\] A set of specific ports allowed to be used as source ports for an active scan.
    pub specific_ports: Option<Vec<RangerSettingsSpecificPort>>,
}

/// A scan restriction entry in [`RangerSettings`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerSettingsRestriction {
    /// Restriction type. Allowed values: `ip`, `cidr`, `range`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// One IP or one CIDR or two values for a Range.
    pub values: Option<Vec<String>>,
    /// An optional note with the reason for the restriction.
    pub annotation: Option<String>,
}

/// A \[FUTURE\] specific-port entry in [`RangerSettings`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerSettingsSpecificPort {
    /// Port spec type. Allowed values: `single`, `range`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// A single port or two ports `[start, end]` for a Range.
    pub values: Option<Vec<i64>>,
}

/// Result of a bulk mutation (device-review / tags).
///
/// Returned by `POST /web/api/v2.1/ranger/device-review` and
/// `POST /web/api/v2.1/ranger/tags`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerAffectedResults {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
}
