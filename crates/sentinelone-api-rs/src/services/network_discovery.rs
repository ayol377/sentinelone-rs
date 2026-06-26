use sentinelone_http::Method;
use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::network_discovery::{RangerAffectedResults, RangerDevice, RangerSettings};
use crate::pagination::{Paginated, Response};

/// `Network Discovery` tag — Network Discovery views and operations.
///
/// Network Discovery (the "ranger" endpoints) gives visibility of all devices
/// connected to your network, including those not protected by a SentinelOne
/// Agent. Requires a Network Discovery license and cloud-based Management.
pub struct NetworkDiscoveryService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/ranger/report/csv` — Export Network Discovery Data.
///
/// All fields are optional filters. Array params are serialized comma-joined,
/// as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerReportCsvQuery {
    /// Single Account ID to filter by. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Single Site ID to filter by. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// OS type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Included OS types. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Period. Allowed values: `latest`, `last12h`, `last24h`, `last3d`, `last7d`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// Os name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    /// Os version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Search using local IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip: Option<String>,
    /// Search using external IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    /// Search using network name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// List of device ids. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Device type (e.g. `Server`, `Workstation`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// Device types. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_types: Option<String>,
    /// A mac address to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// A gateway mac address to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_mac_address: Option<String>,
    /// Included network domains (e.g. `mybusiness,workgroup`). Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// Included site names (e.g. `Office,Test`). Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_names: Option<String>,
    /// Devices first seen before this timestamp.
    #[serde(rename = "firstSeen__lt", skip_serializing_if = "Option::is_none")]
    pub first_seen_lt: Option<String>,
    /// Devices first seen before or at this timestamp.
    #[serde(rename = "firstSeen__lte", skip_serializing_if = "Option::is_none")]
    pub first_seen_lte: Option<String>,
    /// Devices first seen after this timestamp.
    #[serde(rename = "firstSeen__gt", skip_serializing_if = "Option::is_none")]
    pub first_seen_gt: Option<String>,
    /// Devices first seen after or at this timestamp.
    #[serde(rename = "firstSeen__gte", skip_serializing_if = "Option::is_none")]
    pub first_seen_gte: Option<String>,
    /// Date range for first seen (`<from_timestamp>-<to_timestamp>`, inclusive).
    #[serde(rename = "firstSeen__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_between: Option<String>,
    /// Devices last seen before this timestamp.
    #[serde(rename = "lastSeen__lt", skip_serializing_if = "Option::is_none")]
    pub last_seen_lt: Option<String>,
    /// Devices last seen before or at this timestamp.
    #[serde(rename = "lastSeen__lte", skip_serializing_if = "Option::is_none")]
    pub last_seen_lte: Option<String>,
    /// Devices last seen after this timestamp.
    #[serde(rename = "lastSeen__gt", skip_serializing_if = "Option::is_none")]
    pub last_seen_gt: Option<String>,
    /// Devices last seen after or at this timestamp.
    #[serde(rename = "lastSeen__gte", skip_serializing_if = "Option::is_none")]
    pub last_seen_gte: Option<String>,
    /// Date range for last seen (`<from_timestamp>-<to_timestamp>`, inclusive).
    #[serde(rename = "lastSeen__between", skip_serializing_if = "Option::is_none")]
    pub last_seen_between: Option<String>,
    /// List of agent ids. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<String>,
    /// Is the device managed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_state: Option<String>,
    /// Is the device managed. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_states: Option<String>,
    /// Manufacturer of the device or network interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Discovery methods. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// Known fingerprinting data. Array param (comma-joined). Allowed item values:
    /// `Manufacturer`, `Hostname`, `OS version`, `MAC Address`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_fingerprinting_data: Option<String>,
    /// Hostnames. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<String>,
    /// The device review state. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_reviews: Option<String>,
    /// Free-text filter by visible IP (multiple values). Array param (comma-joined).
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip_contains: Option<String>,
    /// Free-text filter by IP Address (multiple values). Array param (comma-joined).
    #[serde(rename = "localIp__contains", skip_serializing_if = "Option::is_none")]
    pub local_ip_contains: Option<String>,
    /// Free-text filter by Subnet Address (multiple values). Array param (comma-joined).
    #[serde(rename = "subnetAddress__contains", skip_serializing_if = "Option::is_none")]
    pub subnet_address_contains: Option<String>,
    /// Free-text filter by OS full name and version (multiple values). Array param (comma-joined).
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// Free-text filter by manufacturer (multiple values). Array param (comma-joined).
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// Free-text filter by mac address (multiple values). Array param (comma-joined).
    #[serde(rename = "macAddress__contains", skip_serializing_if = "Option::is_none")]
    pub mac_address_contains: Option<String>,
    /// Free-text filter by gateway mac address (multiple values). Array param (comma-joined).
    #[serde(rename = "gatewayMacAddress__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_mac_address_contains: Option<String>,
    /// Free-text filter by tcp port (multiple values). Array param (comma-joined).
    #[serde(rename = "tcpPorts__contains", skip_serializing_if = "Option::is_none")]
    pub tcp_ports_contains: Option<String>,
    /// Free-text filter by udp port (multiple values). Array param (comma-joined).
    #[serde(rename = "udpPorts__contains", skip_serializing_if = "Option::is_none")]
    pub udp_ports_contains: Option<String>,
    /// Free-text filter by hostname (multiple values). Array param (comma-joined).
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// Free-text filter by network name (multiple values). Array param (comma-joined).
    #[serde(rename = "networkName__contains", skip_serializing_if = "Option::is_none")]
    pub network_name_contains: Option<String>,
    /// Free-text filter by device function (multiple values). Array param (comma-joined).
    #[serde(rename = "deviceFunction__contains", skip_serializing_if = "Option::is_none")]
    pub device_function_contains: Option<String>,
    /// Free-text filter by tag name (multiple values). Array param (comma-joined).
    #[serde(rename = "tagName__contains", skip_serializing_if = "Option::is_none")]
    pub tag_name_contains: Option<String>,
    /// Query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl RangerReportCsvQuery {
    /// Single Account ID to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// Single Site ID to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// OS type.
    pub fn os_type(mut self, v: impl Into<String>) -> Self {
        self.os_type = Some(v.into());
        self
    }
    /// Included OS types.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// Period. Allowed values: `latest`, `last12h`, `last24h`, `last3d`, `last7d`.
    pub fn period(mut self, v: impl Into<String>) -> Self {
        self.period = Some(v.into());
        self
    }
    /// Os name.
    pub fn os_name(mut self, v: impl Into<String>) -> Self {
        self.os_name = Some(v.into());
        self
    }
    /// Os version.
    pub fn os_version(mut self, v: impl Into<String>) -> Self {
        self.os_version = Some(v.into());
        self
    }
    /// Search using local IP.
    pub fn local_ip(mut self, v: impl Into<String>) -> Self {
        self.local_ip = Some(v.into());
        self
    }
    /// Search using external IP.
    pub fn external_ip(mut self, v: impl Into<String>) -> Self {
        self.external_ip = Some(v.into());
        self
    }
    /// Search using network name.
    pub fn network_name(mut self, v: impl Into<String>) -> Self {
        self.network_name = Some(v.into());
        self
    }
    /// List of device ids.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// Device type.
    pub fn device_type(mut self, v: impl Into<String>) -> Self {
        self.device_type = Some(v.into());
        self
    }
    /// Device types.
    pub fn device_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_types = Some(join_csv(v));
        self
    }
    /// A mac address to search for.
    pub fn mac_address(mut self, v: impl Into<String>) -> Self {
        self.mac_address = Some(v.into());
        self
    }
    /// A gateway mac address to search for.
    pub fn gateway_mac_address(mut self, v: impl Into<String>) -> Self {
        self.gateway_mac_address = Some(v.into());
        self
    }
    /// Included network domains.
    pub fn domains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains = Some(join_csv(v));
        self
    }
    /// Included site names.
    pub fn site_names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_names = Some(join_csv(v));
        self
    }
    /// Devices first seen before this timestamp.
    pub fn first_seen_lt(mut self, v: impl Into<String>) -> Self {
        self.first_seen_lt = Some(v.into());
        self
    }
    /// Devices first seen before or at this timestamp.
    pub fn first_seen_lte(mut self, v: impl Into<String>) -> Self {
        self.first_seen_lte = Some(v.into());
        self
    }
    /// Devices first seen after this timestamp.
    pub fn first_seen_gt(mut self, v: impl Into<String>) -> Self {
        self.first_seen_gt = Some(v.into());
        self
    }
    /// Devices first seen after or at this timestamp.
    pub fn first_seen_gte(mut self, v: impl Into<String>) -> Self {
        self.first_seen_gte = Some(v.into());
        self
    }
    /// Date range for first seen (`<from_timestamp>-<to_timestamp>`, inclusive).
    pub fn first_seen_between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_between = Some(v.into());
        self
    }
    /// Devices last seen before this timestamp.
    pub fn last_seen_lt(mut self, v: impl Into<String>) -> Self {
        self.last_seen_lt = Some(v.into());
        self
    }
    /// Devices last seen before or at this timestamp.
    pub fn last_seen_lte(mut self, v: impl Into<String>) -> Self {
        self.last_seen_lte = Some(v.into());
        self
    }
    /// Devices last seen after this timestamp.
    pub fn last_seen_gt(mut self, v: impl Into<String>) -> Self {
        self.last_seen_gt = Some(v.into());
        self
    }
    /// Devices last seen after or at this timestamp.
    pub fn last_seen_gte(mut self, v: impl Into<String>) -> Self {
        self.last_seen_gte = Some(v.into());
        self
    }
    /// Date range for last seen (`<from_timestamp>-<to_timestamp>`, inclusive).
    pub fn last_seen_between(mut self, v: impl Into<String>) -> Self {
        self.last_seen_between = Some(v.into());
        self
    }
    /// List of agent ids.
    pub fn agent_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ids = Some(join_csv(v));
        self
    }
    /// Is the device managed.
    pub fn managed_state(mut self, v: impl Into<String>) -> Self {
        self.managed_state = Some(v.into());
        self
    }
    /// Is the device managed.
    pub fn managed_states<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.managed_states = Some(join_csv(v));
        self
    }
    /// Manufacturer of the device or network interface.
    pub fn manufacturer(mut self, v: impl Into<String>) -> Self {
        self.manufacturer = Some(v.into());
        self
    }
    /// Discovery methods.
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(join_csv(v));
        self
    }
    /// Known fingerprinting data. Allowed item values: `Manufacturer`, `Hostname`, `OS version`, `MAC Address`.
    pub fn known_fingerprinting_data<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.known_fingerprinting_data = Some(join_csv(v));
        self
    }
    /// Hostnames.
    pub fn hostnames<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames = Some(join_csv(v));
        self
    }
    /// The device review state.
    pub fn device_reviews<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_reviews = Some(join_csv(v));
        self
    }
    /// Free-text filter by visible IP (multiple values).
    pub fn external_ip_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by IP Address (multiple values).
    pub fn local_ip_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.local_ip_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by Subnet Address (multiple values).
    pub fn subnet_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnet_address_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by OS full name and version (multiple values).
    pub fn os_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by manufacturer (multiple values).
    pub fn manufacturer_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by mac address (multiple values).
    pub fn mac_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_address_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by gateway mac address (multiple values).
    pub fn gateway_mac_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_mac_address_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by tcp port (multiple values).
    pub fn tcp_ports_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by udp port (multiple values).
    pub fn udp_ports_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by hostname (multiple values).
    pub fn hostnames_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by network name (multiple values).
    pub fn network_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by device function (multiple values).
    pub fn device_function_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_function_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by tag name (multiple values).
    pub fn tag_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tag_name_contains = Some(join_csv(v));
        self
    }
    /// Query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/ranger/table-view` — Get Network Discovery Table.
///
/// All fields are optional. Array params are serialized comma-joined, as the
/// API expects. This carries the full Device Inventory filter set plus the
/// pagination/sort controls.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerTableViewQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000 items, use `cursor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up execution time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `osType`, `osName`, `id`,
    /// `deviceType`, `osVersion`, `managedState`, `manufacturer`, `firstSeen`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Single Account ID to filter by. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Single Site ID to filter by. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// OS type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Included OS types. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Period. Allowed values: `latest`, `last12h`, `last24h`, `last3d`, `last7d`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    /// Os name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    /// Os version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Search using local IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip: Option<String>,
    /// Search using external IP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    /// Search using network name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// List of device ids. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Device type (e.g. `Server`, `Workstation`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// Device types. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_types: Option<String>,
    /// A mac address to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// A gateway mac address to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_mac_address: Option<String>,
    /// Included network domains (e.g. `mybusiness,workgroup`). Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// Included site names (e.g. `Office,Test`). Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_names: Option<String>,
    /// Devices first seen before this timestamp.
    #[serde(rename = "firstSeen__lt", skip_serializing_if = "Option::is_none")]
    pub first_seen_lt: Option<String>,
    /// Devices first seen before or at this timestamp.
    #[serde(rename = "firstSeen__lte", skip_serializing_if = "Option::is_none")]
    pub first_seen_lte: Option<String>,
    /// Devices first seen after this timestamp.
    #[serde(rename = "firstSeen__gt", skip_serializing_if = "Option::is_none")]
    pub first_seen_gt: Option<String>,
    /// Devices first seen after or at this timestamp.
    #[serde(rename = "firstSeen__gte", skip_serializing_if = "Option::is_none")]
    pub first_seen_gte: Option<String>,
    /// Date range for first seen (`<from_timestamp>-<to_timestamp>`, inclusive).
    #[serde(rename = "firstSeen__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_between: Option<String>,
    /// Devices last seen before this timestamp.
    #[serde(rename = "lastSeen__lt", skip_serializing_if = "Option::is_none")]
    pub last_seen_lt: Option<String>,
    /// Devices last seen before or at this timestamp.
    #[serde(rename = "lastSeen__lte", skip_serializing_if = "Option::is_none")]
    pub last_seen_lte: Option<String>,
    /// Devices last seen after this timestamp.
    #[serde(rename = "lastSeen__gt", skip_serializing_if = "Option::is_none")]
    pub last_seen_gt: Option<String>,
    /// Devices last seen after or at this timestamp.
    #[serde(rename = "lastSeen__gte", skip_serializing_if = "Option::is_none")]
    pub last_seen_gte: Option<String>,
    /// Date range for last seen (`<from_timestamp>-<to_timestamp>`, inclusive).
    #[serde(rename = "lastSeen__between", skip_serializing_if = "Option::is_none")]
    pub last_seen_between: Option<String>,
    /// List of agent ids. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<String>,
    /// Is the device managed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_state: Option<String>,
    /// Is the device managed. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_states: Option<String>,
    /// Manufacturer of the device or network interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Discovery methods. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// Known fingerprinting data. Array param (comma-joined). Allowed item values:
    /// `Manufacturer`, `Hostname`, `OS version`, `MAC Address`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_fingerprinting_data: Option<String>,
    /// Hostnames. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<String>,
    /// The device review state. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_reviews: Option<String>,
    /// Free-text filter by visible IP (multiple values). Array param (comma-joined).
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip_contains: Option<String>,
    /// Free-text filter by IP Address (multiple values). Array param (comma-joined).
    #[serde(rename = "localIp__contains", skip_serializing_if = "Option::is_none")]
    pub local_ip_contains: Option<String>,
    /// Free-text filter by Subnet Address (multiple values). Array param (comma-joined).
    #[serde(rename = "subnetAddress__contains", skip_serializing_if = "Option::is_none")]
    pub subnet_address_contains: Option<String>,
    /// Free-text filter by OS full name and version (multiple values). Array param (comma-joined).
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// Free-text filter by manufacturer (multiple values). Array param (comma-joined).
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// Free-text filter by mac address (multiple values). Array param (comma-joined).
    #[serde(rename = "macAddress__contains", skip_serializing_if = "Option::is_none")]
    pub mac_address_contains: Option<String>,
    /// Free-text filter by gateway mac address (multiple values). Array param (comma-joined).
    #[serde(rename = "gatewayMacAddress__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_mac_address_contains: Option<String>,
    /// Free-text filter by tcp port (multiple values). Array param (comma-joined).
    #[serde(rename = "tcpPorts__contains", skip_serializing_if = "Option::is_none")]
    pub tcp_ports_contains: Option<String>,
    /// Free-text filter by udp port (multiple values). Array param (comma-joined).
    #[serde(rename = "udpPorts__contains", skip_serializing_if = "Option::is_none")]
    pub udp_ports_contains: Option<String>,
    /// Free-text filter by hostname (multiple values). Array param (comma-joined).
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// Free-text filter by network name (multiple values). Array param (comma-joined).
    #[serde(rename = "networkName__contains", skip_serializing_if = "Option::is_none")]
    pub network_name_contains: Option<String>,
    /// Free-text filter by device function (multiple values). Array param (comma-joined).
    #[serde(rename = "deviceFunction__contains", skip_serializing_if = "Option::is_none")]
    pub device_function_contains: Option<String>,
    /// Free-text filter by tag name (multiple values). Array param (comma-joined).
    #[serde(rename = "tagName__contains", skip_serializing_if = "Option::is_none")]
    pub tag_name_contains: Option<String>,
    /// Query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl RangerTableViewQuery {
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `osType`, `osName`, `id`,
    /// `deviceType`, `osVersion`, `managedState`, `manufacturer`, `firstSeen`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Single Account ID to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// Single Site ID to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// OS type.
    pub fn os_type(mut self, v: impl Into<String>) -> Self {
        self.os_type = Some(v.into());
        self
    }
    /// Included OS types.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// Period. Allowed values: `latest`, `last12h`, `last24h`, `last3d`, `last7d`.
    pub fn period(mut self, v: impl Into<String>) -> Self {
        self.period = Some(v.into());
        self
    }
    /// Os name.
    pub fn os_name(mut self, v: impl Into<String>) -> Self {
        self.os_name = Some(v.into());
        self
    }
    /// Os version.
    pub fn os_version(mut self, v: impl Into<String>) -> Self {
        self.os_version = Some(v.into());
        self
    }
    /// Search using local IP.
    pub fn local_ip(mut self, v: impl Into<String>) -> Self {
        self.local_ip = Some(v.into());
        self
    }
    /// Search using external IP.
    pub fn external_ip(mut self, v: impl Into<String>) -> Self {
        self.external_ip = Some(v.into());
        self
    }
    /// Search using network name.
    pub fn network_name(mut self, v: impl Into<String>) -> Self {
        self.network_name = Some(v.into());
        self
    }
    /// List of device ids.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// Device type.
    pub fn device_type(mut self, v: impl Into<String>) -> Self {
        self.device_type = Some(v.into());
        self
    }
    /// Device types.
    pub fn device_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_types = Some(join_csv(v));
        self
    }
    /// A mac address to search for.
    pub fn mac_address(mut self, v: impl Into<String>) -> Self {
        self.mac_address = Some(v.into());
        self
    }
    /// A gateway mac address to search for.
    pub fn gateway_mac_address(mut self, v: impl Into<String>) -> Self {
        self.gateway_mac_address = Some(v.into());
        self
    }
    /// Included network domains.
    pub fn domains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains = Some(join_csv(v));
        self
    }
    /// Included site names.
    pub fn site_names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_names = Some(join_csv(v));
        self
    }
    /// Devices first seen before this timestamp.
    pub fn first_seen_lt(mut self, v: impl Into<String>) -> Self {
        self.first_seen_lt = Some(v.into());
        self
    }
    /// Devices first seen before or at this timestamp.
    pub fn first_seen_lte(mut self, v: impl Into<String>) -> Self {
        self.first_seen_lte = Some(v.into());
        self
    }
    /// Devices first seen after this timestamp.
    pub fn first_seen_gt(mut self, v: impl Into<String>) -> Self {
        self.first_seen_gt = Some(v.into());
        self
    }
    /// Devices first seen after or at this timestamp.
    pub fn first_seen_gte(mut self, v: impl Into<String>) -> Self {
        self.first_seen_gte = Some(v.into());
        self
    }
    /// Date range for first seen (`<from_timestamp>-<to_timestamp>`, inclusive).
    pub fn first_seen_between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_between = Some(v.into());
        self
    }
    /// Devices last seen before this timestamp.
    pub fn last_seen_lt(mut self, v: impl Into<String>) -> Self {
        self.last_seen_lt = Some(v.into());
        self
    }
    /// Devices last seen before or at this timestamp.
    pub fn last_seen_lte(mut self, v: impl Into<String>) -> Self {
        self.last_seen_lte = Some(v.into());
        self
    }
    /// Devices last seen after this timestamp.
    pub fn last_seen_gt(mut self, v: impl Into<String>) -> Self {
        self.last_seen_gt = Some(v.into());
        self
    }
    /// Devices last seen after or at this timestamp.
    pub fn last_seen_gte(mut self, v: impl Into<String>) -> Self {
        self.last_seen_gte = Some(v.into());
        self
    }
    /// Date range for last seen (`<from_timestamp>-<to_timestamp>`, inclusive).
    pub fn last_seen_between(mut self, v: impl Into<String>) -> Self {
        self.last_seen_between = Some(v.into());
        self
    }
    /// List of agent ids.
    pub fn agent_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ids = Some(join_csv(v));
        self
    }
    /// Is the device managed.
    pub fn managed_state(mut self, v: impl Into<String>) -> Self {
        self.managed_state = Some(v.into());
        self
    }
    /// Is the device managed.
    pub fn managed_states<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.managed_states = Some(join_csv(v));
        self
    }
    /// Manufacturer of the device or network interface.
    pub fn manufacturer(mut self, v: impl Into<String>) -> Self {
        self.manufacturer = Some(v.into());
        self
    }
    /// Discovery methods.
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(join_csv(v));
        self
    }
    /// Known fingerprinting data. Allowed item values: `Manufacturer`, `Hostname`, `OS version`, `MAC Address`.
    pub fn known_fingerprinting_data<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.known_fingerprinting_data = Some(join_csv(v));
        self
    }
    /// Hostnames.
    pub fn hostnames<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames = Some(join_csv(v));
        self
    }
    /// The device review state.
    pub fn device_reviews<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_reviews = Some(join_csv(v));
        self
    }
    /// Free-text filter by visible IP (multiple values).
    pub fn external_ip_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by IP Address (multiple values).
    pub fn local_ip_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.local_ip_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by Subnet Address (multiple values).
    pub fn subnet_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnet_address_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by OS full name and version (multiple values).
    pub fn os_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by manufacturer (multiple values).
    pub fn manufacturer_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by mac address (multiple values).
    pub fn mac_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_address_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by gateway mac address (multiple values).
    pub fn gateway_mac_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_mac_address_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by tcp port (multiple values).
    pub fn tcp_ports_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by udp port (multiple values).
    pub fn udp_ports_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by hostname (multiple values).
    pub fn hostnames_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by network name (multiple values).
    pub fn network_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by device function (multiple values).
    pub fn device_function_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_function_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by tag name (multiple values).
    pub fn tag_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tag_name_contains = Some(join_csv(v));
        self
    }
    /// Query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/ranger/settings` — Get Network Discovery Settings.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RangerSettingsQuery {
    /// List of Account IDs to filter by. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl RangerSettingsQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
}

/// Join an iterator of string-likes into a comma-separated value, as the
/// SentinelOne API expects for array query params.
fn join_csv<I, S>(values: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    values
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

impl NetworkDiscoveryService<'_> {
    /// `POST /web/api/v2.1/ranger/device-review` — Change Device Review in Bulk.
    ///
    /// Change the review state of more than one device.
    ///
    /// The body is the `schemas_DeviceReviewSchema` shape (a `data` object with
    /// `deviceReview` plus an optional `filter`); it is passed as freeform JSON
    /// for full fidelity.
    pub async fn change_device_review_bulk(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<RangerAffectedResults>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/ranger/device-review", body)
            .await?)
    }

    /// `PUT /web/api/v2.1/ranger/device-review/{inventory_id}` — Change Device Review.
    ///
    /// Change the review state of one device.
    ///
    /// `inventory_id` is the device Inventory ID (e.g. `225494730938493804`).
    /// The body is the `schemas_DeviceReviewSchemaPut` shape; passed as freeform
    /// JSON for full fidelity.
    pub async fn change_device_review(
        &self,
        inventory_id: impl Into<String>,
        body: &serde_json::Value,
    ) -> Result<Response<RangerDevice>, Error> {
        let path = format!(
            "/web/api/v2.1/ranger/device-review/{}",
            inventory_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<RangerDevice>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/ranger/report/csv` — Export Network Discovery Data.
    ///
    /// Export Network Discovery data to csv. You can set filters to get only
    /// relevant data. The response sends the csv data as text; it is surfaced
    /// here as freeform JSON.
    pub async fn export_report_csv(
        &self,
        query: &RangerReportCsvQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/ranger/report/csv", q)
            .await?)
    }

    /// `GET /web/api/v2.1/ranger/settings` — Get Network Discovery Settings.
    ///
    /// Network Discovery gives full visibility of all devices connected to your
    /// network, including those not protected by a SentinelOne Agent. Use this
    /// command to get the Network Discovery Settings for the given Account: the
    /// response shows if Network Discovery is enabled, the protocols and ports
    /// of the scans, snapshot behaviour, and more. Requires a Network Discovery
    /// license and cloud-based Management (not supported on-prem).
    pub async fn get_settings(
        &self,
        query: &RangerSettingsQuery,
    ) -> Result<Response<RangerSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/ranger/settings", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/ranger/settings` — Update Network Discovery Settings.
    ///
    /// Change the Network Discovery Settings. Best practice: get the current
    /// settings before you change them (see [`get_settings`]). The body is the
    /// `schemas_PutRangerSchema` shape; passed as freeform JSON for full
    /// fidelity.
    ///
    /// [`get_settings`]: Self::get_settings
    pub async fn update_settings(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<RangerSettings>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<RangerSettings>>(
                Method::PUT,
                "/web/api/v2.1/ranger/settings",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/ranger/table-view` — Get Network Discovery Table.
    ///
    /// Get the data for each row in the Network Discovery Device Inventory
    /// Table. Best practice: set filters. Each row is a set of parameters that
    /// quickly fills the pagination limits.
    pub async fn table_view(
        &self,
        query: &RangerTableViewQuery,
    ) -> Result<Paginated<RangerDevice>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/ranger/table-view", q)
            .await?)
    }

    /// `POST /web/api/v2.1/ranger/tags` — Change Device Tags.
    ///
    /// Change the device tags. The body is the `schemas_DeviceTagsSchema` shape;
    /// passed as freeform JSON for full fidelity.
    pub async fn change_device_tags(
        &self,
        body: &serde_json::Value,
    ) -> Result<Response<RangerAffectedResults>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/ranger/tags", body)
            .await?)
    }

    /// `GET /web/api/v2.1/ranger/{inventory_id}/json` — JSON Raw Data.
    ///
    /// Get a json string with the Network Discovery data for one device, by its
    /// ID in the Device Inventory Data. The `data` payload is freeform, so it is
    /// surfaced as [`serde_json::Value`].
    pub async fn raw_data(
        &self,
        inventory_id: impl Into<String>,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/ranger/{}/json", inventory_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `GET /web/api/v2.1/ranger/{inventory_id}/json/export` — Export JSON Raw Data.
    ///
    /// Export the raw data for one device, by its ID in the Device Inventory
    /// Data. To get the ID, run `ranger/table-view` (see [`table_view`]). Use
    /// this command to get data for Support. The response is surfaced as
    /// freeform JSON.
    ///
    /// [`table_view`]: Self::table_view
    pub async fn export_raw_data(
        &self,
        inventory_id: impl Into<String>,
    ) -> Result<serde_json::Value, Error> {
        let path = format!("/web/api/v2.1/ranger/{}/json/export", inventory_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }
}
