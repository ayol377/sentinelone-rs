//! Models for the `Gateways` tag.
//!
//! Gateway views and misc. operations. Gateways are returned by Network
//! Discovery scans (Network Discovery requires a Network Discovery license).

use serde::Deserialize;

/// A single gateway entity discovered by Network Discovery.
///
/// Returned by `GET /web/api/v2.1/ranger/gateways` (as a list) and by
/// `PUT /web/api/v2.1/ranger/gateways/{gateway_id}` (as a single resource).
///
/// The response entity object declares no `required` fields in the spec, so
/// every field is modelled as `Option` (the "default null" behaviour).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gateway {
    /// The gateway id. Optional/nullable -> `Option`.
    pub id: Option<String>,
    /// The gateway local ip. Optional/nullable -> `Option`.
    pub ip: Option<String>,
    /// The gateway mac address. Optional/nullable -> `Option`.
    pub mac_address: Option<String>,
    /// The gateway external Ip. Optional/nullable -> `Option`.
    pub external_ip: Option<String>,
    /// Do we allow scanning in this network. Optional/nullable -> `Option`.
    pub allow_scan: Option<bool>,
    /// Allowed TCP ports. Optional/nullable -> `Option`.
    pub tcp_ports: Option<Vec<i64>>,
    /// Allowed UDP ports. Optional/nullable -> `Option`.
    pub udp_ports: Option<Vec<i64>>,
    /// The Account Id. Optional/nullable -> `Option`.
    pub account_id: Option<i64>,
    /// Account name. Optional/nullable -> `Option`.
    pub account_name: Option<String>,
    /// The gateway manufacturer obtained from the mac address.
    /// Optional/nullable -> `Option`.
    pub manufacturer: Option<String>,
    /// The number of non decommissioned agents in this network.
    /// Optional/nullable -> `Option`.
    pub number_of_agents: Option<i64>,
    /// The number of scanners in this network. Optional/nullable -> `Option`.
    pub number_of_rangers: Option<i64>,
    /// The number of active scanners. Optional/nullable -> `Option`.
    pub connected_rangers: Option<i64>,
    /// The total of non decommissioned agents in the account.
    /// Optional/nullable -> `Option`.
    pub total_agents: Option<i64>,
    /// The network name. Optional/nullable -> `Option`.
    pub network_name: Option<String>,
    /// TCP Port scan enabled. Optional/nullable -> `Option`.
    pub tcp_port_scan: Option<bool>,
    /// ICMP scan enabled. Optional/nullable -> `Option`.
    pub icmp_scan: Option<bool>,
    /// SMB scan enabled. Optional/nullable -> `Option`.
    pub smb_scan: Option<bool>,
    /// MDNS scan enabled. Optional/nullable -> `Option`.
    pub mdns_scan: Option<bool>,
    /// RDNS scan enabled. Optional/nullable -> `Option`.
    pub rdns_scan: Option<bool>,
    /// SNMP scan enabled. Optional/nullable -> `Option`.
    pub snmp_scan: Option<bool>,
    /// UDP Port scan enabled. Optional/nullable -> `Option`.
    pub udp_port_scan: Option<bool>,
    /// Archived network. Optional/nullable -> `Option`.
    pub archived: Option<bool>,
    /// Discovery method. Optional/nullable. Allowed values: `Automatic`,
    /// `User`. Kept as `String` for forward-compatibility.
    pub discovery_method: Option<String>,
    /// Created at (date/time string). Optional/nullable -> `Option`.
    pub created_at: Option<String>,
    /// A set of IP addresses that should not be scanned in the specific
    /// network. Optional/nullable -> `Option`.
    pub restrictions: Option<Vec<GatewayRestriction>>,
    /// True if this network was first seen some days ago, 3 by default.
    /// Optional/nullable -> `Option`.
    pub new: Option<bool>,
    /// Percentage of agents of the account in this network calculated as
    /// `numberOfAgents / totalAgents * 100`. Optional/nullable -> `Option`.
    pub agent_percentage: Option<f64>,
    /// Allow remote tasks form this network. Optional/nullable -> `Option`.
    pub scan_only_local_subnets: Option<bool>,
    /// Date when this network will expire, null if it won't expire (date/time
    /// string). Optional and explicitly `x-nullable` -> `Option`.
    pub expiry_date: Option<String>,
    /// True if inherited values are taken from account settings.
    /// Optional/nullable -> `Option`.
    pub inherit_settings: Option<bool>,
    /// Multicast SSDP scan enabled. Optional/nullable -> `Option`.
    pub multi_scan_ssdp: Option<bool>,
    /// The Site Id. Optional/nullable -> `Option`.
    pub site_id: Option<i64>,
}

/// A single restriction entry on a [`Gateway`]: a set of IP addresses that
/// should not be scanned in the specific network.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayRestriction {
    /// Restriction type. Optional/nullable. Allowed values: `ip`, `cidr`,
    /// `range`. Kept as `String` for forward-compatibility.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// It will be one IP or one CIDR or two values for a Range.
    /// Optional/nullable -> `Option`.
    pub values: Option<Vec<String>>,
    /// An optional note with the reason for the restriction.
    /// Optional/nullable -> `Option`.
    pub annotation: Option<String>,
}

/// Result of `POST /web/api/v2.1/ranger/gateways/update`.
///
/// The response entity object declares no `required` fields in the spec, so
/// `affected` is modelled as `Option`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayAffected {
    /// Number of entities affected by the requested operation.
    /// Optional/nullable -> `Option`.
    pub affected: Option<i64>,
}
