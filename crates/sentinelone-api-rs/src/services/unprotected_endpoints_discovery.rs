use sentinelone_http::Method;
use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::unprotected_endpoints_discovery::{RoguesDevice, RoguesSettings};
use crate::pagination::{Paginated, Response};

/// `Unprotected Endpoints Discovery` tag — views and operations.
///
/// Unprotected Endpoints Discovery (the SentinelOne "rogues" endpoints) gives
/// full visibility of all unsecured devices connected to your network, even
/// those not protected by or supported by SentinelOne. Selected Windows Agents
/// act as scanners that find connected devices with passive and active scan
/// techniques; the collected data is fingerprinted to identify and classify
/// unique devices in the Management Console Device Inventory.
pub struct UnprotectedEndpointsDiscoveryService<'a> {
    pub(crate) client: &'a ManagementClient,
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

/// Query params for `GET /web/api/v2.1/rogues/report/csv` — Export Unprotected
/// Endpoints Discovery Data.
///
/// All fields are optional filters. Array params are serialized comma-joined,
/// as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportCsvQuery {
    /// List of Account IDs to filter by (e.g. `225494730938493804,225494730938493915`).
    /// Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (e.g. `225494730938493804,225494730938493915`).
    /// Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by (e.g. `225494730938493804,225494730938493915`).
    /// Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// OS type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Included OS types. Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Os name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    /// Os version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Search using local IP. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip: Option<String>,
    /// Search using external IP. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    /// List of device ids (e.g. `225494730938493804,225494730938493915`).
    /// Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Device type (e.g. `Server`, `Workstation`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// Device types. Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_types: Option<String>,
    /// A mac address to search for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Devices first seen before this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "firstSeen__lt", skip_serializing_if = "Option::is_none")]
    pub first_seen_lt: Option<String>,
    /// Devices first seen before or at this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "firstSeen__lte", skip_serializing_if = "Option::is_none")]
    pub first_seen_lte: Option<String>,
    /// Devices first seen after this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "firstSeen__gt", skip_serializing_if = "Option::is_none")]
    pub first_seen_gt: Option<String>,
    /// Devices first seen after or at this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "firstSeen__gte", skip_serializing_if = "Option::is_none")]
    pub first_seen_gte: Option<String>,
    /// Date range for first seen (`<from_timestamp>-<to_timestamp>`, inclusive;
    /// e.g. `1514978890136-1514978650130`). Optional.
    #[serde(rename = "firstSeen__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_between: Option<String>,
    /// Devices last seen before this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "lastSeen__lt", skip_serializing_if = "Option::is_none")]
    pub last_seen_lt: Option<String>,
    /// Devices last seen before or at this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "lastSeen__lte", skip_serializing_if = "Option::is_none")]
    pub last_seen_lte: Option<String>,
    /// Devices last seen after this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "lastSeen__gt", skip_serializing_if = "Option::is_none")]
    pub last_seen_gt: Option<String>,
    /// Devices last seen after or at this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "lastSeen__gte", skip_serializing_if = "Option::is_none")]
    pub last_seen_gte: Option<String>,
    /// Date range for last seen (`<from_timestamp>-<to_timestamp>`, inclusive;
    /// e.g. `1514978890136-1514978650130`). Optional.
    #[serde(rename = "lastSeen__between", skip_serializing_if = "Option::is_none")]
    pub last_seen_between: Option<String>,
    /// Manufacturer of the device or network interface. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Hostnames. Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<String>,
    /// Free-text filter by visible IP (supports multiple values; e.g.
    /// `192.168.0.1/24,10.1`). Array param (comma-joined). Optional.
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip_contains: Option<String>,
    /// Free-text filter by IP Address (supports multiple values; e.g.
    /// `192.168.0.1/24,10.1`). Array param (comma-joined). Optional.
    #[serde(rename = "localIp__contains", skip_serializing_if = "Option::is_none")]
    pub local_ip_contains: Option<String>,
    /// Free-text filter by OS full name and version (supports multiple values;
    /// e.g. `Service Pack 1`). Array param (comma-joined). Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// Free-text filter by manufacturer (supports multiple values; e.g.
    /// `Company`). Array param (comma-joined). Optional.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// Free-text filter by mac address (supports multiple values; e.g.
    /// `aa:ee:b1`). Array param (comma-joined). Optional.
    #[serde(rename = "macAddress__contains", skip_serializing_if = "Option::is_none")]
    pub mac_address_contains: Option<String>,
    /// Free-text filter by hostname (supports multiple values; e.g.
    /// `s1_host,SomeHost`). Array param (comma-joined). Optional.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// Query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl ReportCsvQuery {
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
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
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
    /// Manufacturer of the device or network interface.
    pub fn manufacturer(mut self, v: impl Into<String>) -> Self {
        self.manufacturer = Some(v.into());
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
    /// Free-text filter by visible IP (supports multiple values).
    pub fn external_ip_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by IP Address (supports multiple values).
    pub fn local_ip_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.local_ip_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by OS full name and version (supports multiple values).
    pub fn os_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by manufacturer (supports multiple values).
    pub fn manufacturer_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by mac address (supports multiple values).
    pub fn mac_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_address_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by hostname (supports multiple values).
    pub fn hostnames_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(join_csv(v));
        self
    }
    /// Query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/rogues/settings` — Get Unprotected
/// Endpoints Discovery Settings.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsQuery {
    /// List of Account IDs to filter by (e.g. `225494730938493804,225494730938493915`).
    /// Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (e.g. `225494730938493804,225494730938493915`).
    /// Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl SettingsQuery {
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

/// Query params for `GET /web/api/v2.1/rogues/table-view` — Get Unprotected
/// Endpoints Discovery Table.
///
/// All fields are optional. Array params are serialized comma-joined, as the
/// API expects. This carries the full Device Inventory filter set plus the
/// pagination/sort controls.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableViewQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor` (e.g. `150`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000; e.g. `10`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items (e.g. `YWdlbnRfaWQ6NTgwMjkzODE=`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by (e.g. `id`). Allowed values: `osType`,
    /// `osName`, `id`, `deviceType`, `osVersion`, `manufacturer`, `firstSeen`,
    /// `lastSeen`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction (e.g. `asc`). Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by (e.g. `225494730938493804,225494730938493915`).
    /// Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (e.g. `225494730938493804,225494730938493915`).
    /// Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by (e.g. `225494730938493804,225494730938493915`).
    /// Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// OS type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Included OS types. Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Os name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name: Option<String>,
    /// Os version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Search using local IP. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_ip: Option<String>,
    /// Search using external IP. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    /// List of device ids (e.g. `225494730938493804,225494730938493915`).
    /// Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Device type (e.g. `Server`, `Workstation`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// Device types. Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_types: Option<String>,
    /// A mac address to search for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Devices first seen before this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "firstSeen__lt", skip_serializing_if = "Option::is_none")]
    pub first_seen_lt: Option<String>,
    /// Devices first seen before or at this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "firstSeen__lte", skip_serializing_if = "Option::is_none")]
    pub first_seen_lte: Option<String>,
    /// Devices first seen after this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "firstSeen__gt", skip_serializing_if = "Option::is_none")]
    pub first_seen_gt: Option<String>,
    /// Devices first seen after or at this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "firstSeen__gte", skip_serializing_if = "Option::is_none")]
    pub first_seen_gte: Option<String>,
    /// Date range for first seen (`<from_timestamp>-<to_timestamp>`, inclusive;
    /// e.g. `1514978890136-1514978650130`). Optional.
    #[serde(rename = "firstSeen__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_between: Option<String>,
    /// Devices last seen before this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "lastSeen__lt", skip_serializing_if = "Option::is_none")]
    pub last_seen_lt: Option<String>,
    /// Devices last seen before or at this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "lastSeen__lte", skip_serializing_if = "Option::is_none")]
    pub last_seen_lte: Option<String>,
    /// Devices last seen after this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "lastSeen__gt", skip_serializing_if = "Option::is_none")]
    pub last_seen_gt: Option<String>,
    /// Devices last seen after or at this timestamp (e.g. `2018-02-27T04:49:26.257525Z`). Optional.
    #[serde(rename = "lastSeen__gte", skip_serializing_if = "Option::is_none")]
    pub last_seen_gte: Option<String>,
    /// Date range for last seen (`<from_timestamp>-<to_timestamp>`, inclusive;
    /// e.g. `1514978890136-1514978650130`). Optional.
    #[serde(rename = "lastSeen__between", skip_serializing_if = "Option::is_none")]
    pub last_seen_between: Option<String>,
    /// Manufacturer of the device or network interface. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Hostnames. Array param (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<String>,
    /// Free-text filter by visible IP (supports multiple values; e.g.
    /// `192.168.0.1/24,10.1`). Array param (comma-joined). Optional.
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip_contains: Option<String>,
    /// Free-text filter by IP Address (supports multiple values; e.g.
    /// `192.168.0.1/24,10.1`). Array param (comma-joined). Optional.
    #[serde(rename = "localIp__contains", skip_serializing_if = "Option::is_none")]
    pub local_ip_contains: Option<String>,
    /// Free-text filter by OS full name and version (supports multiple values;
    /// e.g. `Service Pack 1`). Array param (comma-joined). Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// Free-text filter by manufacturer (supports multiple values; e.g.
    /// `Company`). Array param (comma-joined). Optional.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// Free-text filter by mac address (supports multiple values; e.g.
    /// `aa:ee:b1`). Array param (comma-joined). Optional.
    #[serde(rename = "macAddress__contains", skip_serializing_if = "Option::is_none")]
    pub mac_address_contains: Option<String>,
    /// Free-text filter by hostname (supports multiple values; e.g.
    /// `s1_host,SomeHost`). Array param (comma-joined). Optional.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// Query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl TableViewQuery {
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
    /// The column to sort the results by. Allowed values: `osType`, `osName`,
    /// `id`, `deviceType`, `osVersion`, `manufacturer`, `firstSeen`, `lastSeen`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
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
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
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
    /// Manufacturer of the device or network interface.
    pub fn manufacturer(mut self, v: impl Into<String>) -> Self {
        self.manufacturer = Some(v.into());
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
    /// Free-text filter by visible IP (supports multiple values).
    pub fn external_ip_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by IP Address (supports multiple values).
    pub fn local_ip_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.local_ip_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by OS full name and version (supports multiple values).
    pub fn os_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by manufacturer (supports multiple values).
    pub fn manufacturer_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by mac address (supports multiple values).
    pub fn mac_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_address_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by hostname (supports multiple values).
    pub fn hostnames_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(join_csv(v));
        self
    }
    /// Query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
}

/// Body for `PUT /web/api/v2.1/rogues/settings` — Update Unprotected Endpoints
/// Discovery Settings (the `rogue_schemas_PutRoguesSchema` shape).
///
/// Both `data` and `filter` are required.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsBody {
    /// The settings values to apply. Required.
    pub data: UpdateSettingsData,
    /// The scope filter (which accounts/sites the settings apply to). Required.
    pub filter: UpdateSettingsFilter,
}

impl UpdateSettingsBody {
    /// Construct a body from its required `data` and `filter` parts.
    pub fn new(data: UpdateSettingsData, filter: UpdateSettingsFilter) -> Self {
        Self { data, filter }
    }
}

/// The `data` part of [`UpdateSettingsBody`]. All fields are optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsData {
    /// Is the Network Discovery collection enabled for the account. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Minimum agents required in a network to be listed as selectable for
    /// scan. Valid values are `2`, `10` and `100` if Unprotected Endpoints
    /// Discovery is enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_agents_in_network_to_scan: Option<i64>,
    /// Account id (e.g. `225494730938493804`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// A set of IP addresses that should not be scanned in the specific network
    /// (max 5000). Each entry: `type` (`ip`/`cidr`/`range`), `values`,
    /// optional `annotation`. Freeform JSON for full fidelity. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<serde_json::Value>,
    /// \[FUTURE\] Use only specific ports defined in specific ports as source
    /// ports of active scans. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_specific_ports: Option<bool>,
    /// \[FUTURE\] A set of specific ports allowed to be used as source ports for
    /// an active scan (max 5000). Each entry: `type` (`single`/`range`),
    /// `values`. Freeform JSON for full fidelity. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specific_ports: Option<serde_json::Value>,
}

/// The `filter` part of [`UpdateSettingsBody`]. All fields are optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsFilter {
    /// List of Account IDs to filter by (max 5000). Array param (comma-joined
    /// not applicable; serialized as a JSON array). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (max 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
}

impl UnprotectedEndpointsDiscoveryService<'_> {
    /// `GET /web/api/v2.1/rogues/report/csv` — Export Unprotected Endpoints
    /// Discovery Data.
    ///
    /// Export Unprotected Endpoints Discovery data to CSV. You can set filters
    /// to get only relevant data. The response sends the CSV data as text; it
    /// is surfaced here as freeform JSON.
    pub async fn export_report_csv(
        &self,
        query: &ReportCsvQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/rogues/report/csv", q)
            .await?)
    }

    /// `GET /web/api/v2.1/rogues/settings` — Get Unprotected Endpoints
    /// Discovery Settings.
    ///
    /// Unprotected Endpoints Discovery gives full visibility of all unsecured
    /// devices connected to your network, even those not protected by or
    /// supported by SentinelOne. It scans your corporate environment to
    /// identify and manage connected devices and classifies UnSecured devices
    /// (end-user computers, laptops, or servers without a SentinelOne Agent).
    /// When you install Windows Agents with Unprotected Endpoints Discovery,
    /// the Agents can become scanners. The `minAgentsInNetworkToScan` setting
    /// helps determine which networks are corporate: if there are not enough
    /// Agents in a network, Unprotected Endpoints Discovery considers it
    /// non-corporate and will not scan it.
    pub async fn get_settings(
        &self,
        query: &SettingsQuery,
    ) -> Result<Response<RoguesSettings>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/rogues/settings", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/rogues/settings` — Update Unprotected Endpoints
    /// Discovery Settings.
    ///
    /// Change the Unprotected Endpoints Discovery Settings. Best practice: get
    /// the current settings before you change them (see [`get_settings`]).
    ///
    /// [`get_settings`]: Self::get_settings
    pub async fn update_settings(
        &self,
        body: &UpdateSettingsBody,
    ) -> Result<Response<RoguesSettings>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<UpdateSettingsBody, Response<RoguesSettings>>(
                Method::PUT,
                "/web/api/v2.1/rogues/settings",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/rogues/table-view` — Get Unprotected Endpoints
    /// Discovery Table.
    ///
    /// Get the data for each row in the Unprotected Endpoints Discovery Device
    /// Inventory Table. Best practice: set filters. Each row is a set of
    /// parameters that quickly fills the pagination limits.
    pub async fn table_view(
        &self,
        query: &TableViewQuery,
    ) -> Result<Paginated<RoguesDevice>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/rogues/table-view", q)
            .await?)
    }
}
