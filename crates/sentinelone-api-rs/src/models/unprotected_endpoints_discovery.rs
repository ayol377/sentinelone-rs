//! Models for the `Unprotected Endpoints Discovery` tag (the SentinelOne
//! "rogues" endpoints).
//!
//! Field nullability mirrors `swagger_2_1.json` exactly: a field is a bare `T`
//! only when it is listed in the schema `required` array and is not
//! `x-nullable`; otherwise it is `Option<T>` (the API's "default null"
//! behaviour). The `data` item schemas for this tag declare no `required`
//! array, so every field is `Option<T>`. Enum-valued fields are kept as
//! `String` for forward-compat; the allowed values are documented in each
//! field's doc comment.

use serde::Deserialize;

/// A single row of the Unprotected Endpoints Discovery Device Inventory Table.
///
/// Returned by `GET /web/api/v2.1/rogues/table-view` (as a page of rows).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoguesDevice {
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
    /// Array of host names.
    pub hostnames: Option<Vec<String>>,
    /// Manufacturer of the device or network interface.
    pub manufacturer: Option<String>,
    /// Function of the device (e.g. `Server`).
    pub device_function: Option<String>,
}

/// Unprotected Endpoints Discovery ("rogues") settings for an account/scope.
///
/// Returned by `GET` and `PUT /web/api/v2.1/rogues/settings`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoguesSettings {
    /// Is the Network Discovery collection enabled for the account.
    pub enabled: Option<bool>,
    /// Minimum agents required in a network to be listed as selectable for
    /// scan. Valid values are `2`, `10` and `100` if Unprotected Endpoints
    /// Discovery is enabled.
    pub min_agents_in_network_to_scan: Option<i64>,
    /// Account id.
    pub account_id: Option<String>,
    /// A set of IP addresses that should not be scanned in the specific network.
    pub restrictions: Option<Vec<RoguesSettingsRestriction>>,
    /// \[FUTURE\] Use only specific ports defined in specific ports as source
    /// ports of active scans.
    pub use_specific_ports: Option<bool>,
    /// \[FUTURE\] A set of specific ports allowed to be used as source ports for
    /// an active scan.
    pub specific_ports: Option<Vec<RoguesSettingsSpecificPort>>,
}

/// A scan restriction entry in [`RoguesSettings`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoguesSettingsRestriction {
    /// Restriction type. Allowed values: `ip`, `cidr`, `range`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// One IP or one CIDR or two values for a Range.
    pub values: Option<Vec<String>>,
    /// An optional note with the reason for the restriction.
    pub annotation: Option<String>,
}

/// A \[FUTURE\] specific-port entry in [`RoguesSettings`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoguesSettingsSpecificPort {
    /// Port spec type. Allowed values: `single`, `range`.
    #[serde(rename = "type")]
    pub type_: Option<String>,
    /// A single port or two ports `[start, end]` for a Range.
    pub values: Option<Vec<i64>>,
}
