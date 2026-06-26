//! Models for the `Locations` tag.
//!
//! A location defines parameters of Agents in a scope filter (IP addresses,
//! DNS servers, DNS lookup, network interfaces, server connectivity and
//! registry keys). Agents detect their location and apply Firewall Control
//! rules whose Location Aware parameters match.

use serde::Deserialize;

/// A location definition.
///
/// Returned by `GET /web/api/v2.1/locations` (as a list), and by
/// `POST`/`PUT /web/api/v2.1/locations` (as a single resource).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// Location name (should be unique per scope). Required.
    pub name: String,
    /// Logical operator to apply between the set of identifiers.
    /// Allowed values: `all`, `any`, `none`. Required.
    pub operator: String,
    /// Location description. Optional/nullable.
    #[serde(default)]
    pub description: Option<String>,
    /// Identify a location by DNS lookup results. Optional/nullable.
    #[serde(default)]
    pub dns_lookup: Option<LocationDnsLookup>,
    /// Identify a location by DNS servers defined on the endpoint.
    /// Optional/nullable.
    #[serde(default)]
    pub dns_servers: Option<LocationAddressMatch>,
    /// Identify a location by a registry key or value. Optional/nullable.
    #[serde(default)]
    pub registry_keys: Option<LocationRegistryKeys>,
    /// Identify a location by connectivity to the management server.
    /// Optional/nullable.
    #[serde(default)]
    pub server_connectivity: Option<LocationServerConnectivity>,
    /// Identify a location by available network interface types.
    /// Optional/nullable.
    #[serde(default)]
    pub network_interfaces: Option<LocationNetworkInterfaces>,
    /// Identify a location by the assigned IP addresses. Optional/nullable.
    #[serde(default)]
    pub ip_addresses: Option<LocationAddressMatch>,
    /// Id. Optional/nullable.
    #[serde(default)]
    pub id: Option<String>,
    /// Created at (ISO-8601 timestamp). Optional/nullable.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Updated at (ISO-8601 timestamp). Optional/nullable.
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Location creator name. Optional/nullable.
    #[serde(default)]
    pub creator: Option<String>,
    /// Location creator ID. Optional/nullable.
    #[serde(default)]
    pub creator_id: Option<String>,
    /// Location updater name. Optional/nullable.
    #[serde(default)]
    pub updater: Option<String>,
    /// Location updater ID. Optional/nullable.
    #[serde(default)]
    pub updater_id: Option<String>,
    /// Scope. Allowed values: `global`, `group`, `account`, `site`.
    /// Optional/nullable.
    #[serde(default)]
    pub scope: Option<String>,
    /// Scope id. Optional/nullable.
    #[serde(default)]
    pub scope_id: Option<String>,
    /// Scope name. Optional/nullable.
    #[serde(default)]
    pub scope_name: Option<String>,
    /// Is location editable in current scope. Optional/nullable.
    #[serde(default)]
    pub editable: Option<bool>,
    /// Is fallback. Optional/nullable.
    #[serde(default)]
    pub is_fallback: Option<bool>,
    /// Number of agents in the location. Optional/nullable.
    #[serde(default)]
    pub reporting_agents: Option<i64>,
    /// Number of active firewall rules defined in the location.
    /// Optional/nullable.
    #[serde(default)]
    pub active_firewall_rules: Option<i64>,
}

/// Identify a location by DNS lookup results.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationDnsLookup {
    /// A list of DNS lookup identifiers (max 5). Required.
    pub identifiers: Vec<LocationDnsLookupIdentifier>,
    /// Logical operator to apply between the set of identifiers.
    /// Allowed values: `all`, `any`, `none`. Required.
    pub operator: String,
}

/// A single DNS lookup identifier (hostname + resolved IP).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationDnsLookupIdentifier {
    /// Hostname to resolve. Required.
    pub host: String,
    /// Resolved IP address. Required.
    pub ip: String,
}

/// Identify a location by a set of address identifiers.
///
/// Shared shape used by both `ipAddresses` and `dnsServers`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationAddressMatch {
    /// A list of identifiers (1-5 for IP addresses / DNS servers).
    /// Optional/nullable.
    #[serde(default)]
    pub identifiers: Option<Vec<LocationAddressIdentifier>>,
    /// Logical operator to apply between the set of identifiers.
    /// Allowed values: `all`, `any`, `none`. Required.
    pub operator: String,
}

/// A single address identifier (an IP address, CIDR or range).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationAddressIdentifier {
    /// IP address, CIDR or range of two addresses (IPv4 or IPv6); 1-2 values.
    /// Required.
    pub values: Vec<String>,
    /// Address type. Allowed values: `address`, `cidr`, `range`. Required.
    #[serde(rename = "type")]
    pub type_: String,
}

/// Identify a location by a registry key or value.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationRegistryKeys {
    /// Registry key path to match. Must start with
    /// `HKEY_LOCAL_MACHINE\SOFTWARE\`. Required.
    pub key: String,
    /// Value name in the registry key path to match (optional).
    /// Optional/nullable.
    #[serde(default)]
    pub value: Option<String>,
    /// Content of the value to match (a string or 64-bit integer, optional).
    /// Optional/nullable.
    #[serde(default)]
    pub data: Option<String>,
}

/// Identify a location by connectivity to the management server.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationServerConnectivity {
    /// Use or discard this location identifier. Required.
    pub enabled: bool,
    /// Server connectivity status. Allowed values: `connected`,
    /// `disconnected`. Optional/nullable.
    #[serde(default)]
    pub value: Option<String>,
}

/// Identify a location by available network interface types.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationNetworkInterfaces {
    /// Use or discard this location identifier. Required.
    pub enabled: bool,
    /// Network interface type. Allowed values: `wired`, `wireless`.
    /// Optional/nullable.
    #[serde(default)]
    pub value: Option<String>,
}

/// Result of a `DELETE /web/api/v2.1/locations` operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationAffected {
    /// Number of entities affected by the requested operation.
    /// Optional/nullable.
    #[serde(default)]
    pub affected: Option<i64>,
}
