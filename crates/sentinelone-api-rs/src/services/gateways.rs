use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::gateways::{Gateway, GatewayAffected};
use crate::pagination::{Paginated, Response};

/// `Gateways` tag.
///
/// Gateway views and misc. operations.
pub struct GatewaysService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/ranger/gateways`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListGatewaysQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: "150". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
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
    /// The column to sort the results by. Example: "id". Allowed values: `id`,
    /// `ip`, `macAddress`, `externalIp`, `allowScan`, `networkName`,
    /// `totalAgents`, `agentPercentage`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by (comma-joined). Example:
    /// "225494730938493804,225494730938493915". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by (comma-joined). Example:
    /// "225494730938493804,225494730938493915". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Free text query. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Search ip using a CIDR expression exact IP. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The gateway mac address. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Search external ip using a CIDR expression or exact IP. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ip: Option<String>,
    /// Do we allow scanning in this network. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_scan: Option<String>,
    /// List of gateway ids (comma-joined). Example:
    /// "225494730938493804,225494730938493915". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Allow remote tasks form this network. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_only_local_subnets: Option<bool>,
    /// True if this network was first seen some days ago, 3 by default.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new: Option<bool>,
    /// Archived network. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Agent count (less than). Optional.
    #[serde(rename = "numberOfAgents__lt", skip_serializing_if = "Option::is_none")]
    pub number_of_agents_lt: Option<i64>,
    /// Agent count (less than or equal). Optional.
    #[serde(rename = "numberOfAgents__lte", skip_serializing_if = "Option::is_none")]
    pub number_of_agents_lte: Option<i64>,
    /// Agent count (more than). Optional.
    #[serde(rename = "numberOfAgents__gt", skip_serializing_if = "Option::is_none")]
    pub number_of_agents_gt: Option<i64>,
    /// Agent count (more than or equal). Optional.
    #[serde(rename = "numberOfAgents__gte", skip_serializing_if = "Option::is_none")]
    pub number_of_agents_gte: Option<i64>,
    /// The number of non decommissioned agents in this network. Example: "2-8".
    /// Optional.
    #[serde(rename = "numberOfAgents__between", skip_serializing_if = "Option::is_none")]
    pub number_of_agents_between: Option<String>,
    /// Network Discovery count (less than). Optional.
    #[serde(rename = "numberOfRangers__lt", skip_serializing_if = "Option::is_none")]
    pub number_of_rangers_lt: Option<i64>,
    /// Network Discovery count (less than or equal). Optional.
    #[serde(rename = "numberOfRangers__lte", skip_serializing_if = "Option::is_none")]
    pub number_of_rangers_lte: Option<i64>,
    /// Network Discovery count (more than). Optional.
    #[serde(rename = "numberOfRangers__gt", skip_serializing_if = "Option::is_none")]
    pub number_of_rangers_gt: Option<i64>,
    /// Network Discovery count (more than or equal). Optional.
    #[serde(rename = "numberOfRangers__gte", skip_serializing_if = "Option::is_none")]
    pub number_of_rangers_gte: Option<i64>,
    /// The number of non decommissioned agents in this network. Example: "2-8".
    /// Optional.
    #[serde(rename = "numberOfRangers__between", skip_serializing_if = "Option::is_none")]
    pub number_of_rangers_between: Option<String>,
    /// Agent percentage (less than). Optional.
    #[serde(rename = "agentPercentage__lt", skip_serializing_if = "Option::is_none")]
    pub agent_percentage_lt: Option<f64>,
    /// Agent percentage (less than or equal). Optional.
    #[serde(rename = "agentPercentage__lte", skip_serializing_if = "Option::is_none")]
    pub agent_percentage_lte: Option<f64>,
    /// Agent percentage (more than). Optional.
    #[serde(rename = "agentPercentage__gt", skip_serializing_if = "Option::is_none")]
    pub agent_percentage_gt: Option<f64>,
    /// Agent percentage (more than or equal). Optional.
    #[serde(rename = "agentPercentage__gte", skip_serializing_if = "Option::is_none")]
    pub agent_percentage_gte: Option<f64>,
    /// Percentage of agents of the account in this network calculated as
    /// `numberOfAgents / totalAgents * 100`. Example: "70-80". Optional.
    #[serde(rename = "agentPercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_percentage_between: Option<String>,
    /// Total agents (less than). Optional.
    #[serde(rename = "totalAgents__lt", skip_serializing_if = "Option::is_none")]
    pub total_agents_lt: Option<i64>,
    /// Total agents (less than or equal). Optional.
    #[serde(rename = "totalAgents__lte", skip_serializing_if = "Option::is_none")]
    pub total_agents_lte: Option<i64>,
    /// Total agents (more than). Optional.
    #[serde(rename = "totalAgents__gt", skip_serializing_if = "Option::is_none")]
    pub total_agents_gt: Option<i64>,
    /// Total agents (more than or equal). Optional.
    #[serde(rename = "totalAgents__gte", skip_serializing_if = "Option::is_none")]
    pub total_agents_gte: Option<i64>,
    /// The total of non decommissioned agents in the account. Example: "2-8".
    /// Optional.
    #[serde(rename = "totalAgents__between", skip_serializing_if = "Option::is_none")]
    pub total_agents_between: Option<String>,
    /// Connected rangers (less than). Optional.
    #[serde(rename = "connectedRangers__lt", skip_serializing_if = "Option::is_none")]
    pub connected_rangers_lt: Option<i64>,
    /// Connected rangers (less than or equal). Optional.
    #[serde(rename = "connectedRangers__lte", skip_serializing_if = "Option::is_none")]
    pub connected_rangers_lte: Option<i64>,
    /// Connected rangers (more than). Optional.
    #[serde(rename = "connectedRangers__gt", skip_serializing_if = "Option::is_none")]
    pub connected_rangers_gt: Option<i64>,
    /// Connected rangers (more than or equal). Optional.
    #[serde(rename = "connectedRangers__gte", skip_serializing_if = "Option::is_none")]
    pub connected_rangers_gte: Option<i64>,
    /// The total of non decommissioned agents in the account. Example: "2-8".
    /// Optional.
    #[serde(rename = "connectedRangers__between", skip_serializing_if = "Option::is_none")]
    pub connected_rangers_between: Option<String>,
    /// Gateway created before this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Gateway created before or at this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Gateway created after this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Gateway created after or at this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Date range for creation time (format:
    /// `<from_timestamp>-<to_timestamp>`, inclusive). Example:
    /// "1514978890136-1514978650130". Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// Gateway updated before this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at_lt: Option<String>,
    /// Gateway updated before or at this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// Gateway updated after this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at_gt: Option<String>,
    /// Gateway updated after or at this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// Date range for update time (format: `<from_timestamp>-<to_timestamp>`,
    /// inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at_between: Option<String>,
    /// SMB scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_scan: Option<bool>,
    /// ICMP scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icmp_scan: Option<bool>,
    /// MDNS scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdns_scan: Option<bool>,
    /// SNMP scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmp_scan: Option<bool>,
    /// RDNS scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdns_scan: Option<bool>,
    /// The gateway manufacturer obtained from the mac address. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// Free-text filter by network name (supports multiple values, comma-
    /// joined). Example: "Network1". Optional.
    #[serde(rename = "networkName__contains", skip_serializing_if = "Option::is_none")]
    pub network_name_contains: Option<String>,
    /// Free-text filter by visible IP (supports multiple values, comma-joined).
    /// Example: "192.168.0.1/24,10.1". Optional.
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip_contains: Option<String>,
    /// Free-text filter by IP Address (supports multiple values, comma-joined).
    /// Example: "192.168.0.1/24,10.1". Optional.
    #[serde(rename = "ip__contains", skip_serializing_if = "Option::is_none")]
    pub ip_contains: Option<String>,
    /// Free-text filter by manufacturer (supports multiple values, comma-
    /// joined). Example: "Company". Optional.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// Free-text filter by mac address (supports multiple values, comma-
    /// joined). Example: "aa:ee:b1". Optional.
    #[serde(rename = "macAddress__contains", skip_serializing_if = "Option::is_none")]
    pub mac_address_contains: Option<String>,
    /// Free-text filter by tcp port (supports multiple values, comma-joined).
    /// Example: "80,24". Optional.
    #[serde(rename = "tcpPorts__contains", skip_serializing_if = "Option::is_none")]
    pub tcp_ports_contains: Option<String>,
    /// Free-text filter by udp port (supports multiple values, comma-joined).
    /// Example: "137,2002". Optional.
    #[serde(rename = "udpPorts__contains", skip_serializing_if = "Option::is_none")]
    pub udp_ports_contains: Option<String>,
}

impl ListGatewaysQuery {
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
    /// The column to sort the results by. Allowed values: `id`, `ip`,
    /// `macAddress`, `externalIp`, `allowScan`, `networkName`, `totalAgents`,
    /// `agentPercentage`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_comma(items));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_comma(items));
        self
    }
    /// Free text query.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Search ip using a CIDR expression exact IP.
    pub fn ip(mut self, v: impl Into<String>) -> Self {
        self.ip = Some(v.into());
        self
    }
    /// The gateway mac address.
    pub fn mac_address(mut self, v: impl Into<String>) -> Self {
        self.mac_address = Some(v.into());
        self
    }
    /// Search external ip using a CIDR expression or exact IP.
    pub fn external_ip(mut self, v: impl Into<String>) -> Self {
        self.external_ip = Some(v.into());
        self
    }
    /// Do we allow scanning in this network.
    pub fn allow_scan(mut self, v: impl Into<String>) -> Self {
        self.allow_scan = Some(v.into());
        self
    }
    /// List of gateway ids.
    pub fn ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_comma(items));
        self
    }
    /// Allow remote tasks form this network.
    pub fn scan_only_local_subnets(mut self, v: bool) -> Self {
        self.scan_only_local_subnets = Some(v);
        self
    }
    /// True if this network was first seen some days ago, 3 by default.
    pub fn new(mut self, v: bool) -> Self {
        self.new = Some(v);
        self
    }
    /// Archived network.
    pub fn archived(mut self, v: bool) -> Self {
        self.archived = Some(v);
        self
    }
    /// Agent count (less than).
    pub fn number_of_agents_lt(mut self, n: i64) -> Self {
        self.number_of_agents_lt = Some(n);
        self
    }
    /// Agent count (less than or equal).
    pub fn number_of_agents_lte(mut self, n: i64) -> Self {
        self.number_of_agents_lte = Some(n);
        self
    }
    /// Agent count (more than).
    pub fn number_of_agents_gt(mut self, n: i64) -> Self {
        self.number_of_agents_gt = Some(n);
        self
    }
    /// Agent count (more than or equal).
    pub fn number_of_agents_gte(mut self, n: i64) -> Self {
        self.number_of_agents_gte = Some(n);
        self
    }
    /// The number of non decommissioned agents in this network. Example: "2-8".
    pub fn number_of_agents_between(mut self, v: impl Into<String>) -> Self {
        self.number_of_agents_between = Some(v.into());
        self
    }
    /// Network Discovery count (less than).
    pub fn number_of_rangers_lt(mut self, n: i64) -> Self {
        self.number_of_rangers_lt = Some(n);
        self
    }
    /// Network Discovery count (less than or equal).
    pub fn number_of_rangers_lte(mut self, n: i64) -> Self {
        self.number_of_rangers_lte = Some(n);
        self
    }
    /// Network Discovery count (more than).
    pub fn number_of_rangers_gt(mut self, n: i64) -> Self {
        self.number_of_rangers_gt = Some(n);
        self
    }
    /// Network Discovery count (more than or equal).
    pub fn number_of_rangers_gte(mut self, n: i64) -> Self {
        self.number_of_rangers_gte = Some(n);
        self
    }
    /// The number of non decommissioned agents in this network. Example: "2-8".
    pub fn number_of_rangers_between(mut self, v: impl Into<String>) -> Self {
        self.number_of_rangers_between = Some(v.into());
        self
    }
    /// Agent percentage (less than).
    pub fn agent_percentage_lt(mut self, n: f64) -> Self {
        self.agent_percentage_lt = Some(n);
        self
    }
    /// Agent percentage (less than or equal).
    pub fn agent_percentage_lte(mut self, n: f64) -> Self {
        self.agent_percentage_lte = Some(n);
        self
    }
    /// Agent percentage (more than).
    pub fn agent_percentage_gt(mut self, n: f64) -> Self {
        self.agent_percentage_gt = Some(n);
        self
    }
    /// Agent percentage (more than or equal).
    pub fn agent_percentage_gte(mut self, n: f64) -> Self {
        self.agent_percentage_gte = Some(n);
        self
    }
    /// Percentage of agents of the account in this network. Example: "70-80".
    pub fn agent_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_percentage_between = Some(v.into());
        self
    }
    /// Total agents (less than).
    pub fn total_agents_lt(mut self, n: i64) -> Self {
        self.total_agents_lt = Some(n);
        self
    }
    /// Total agents (less than or equal).
    pub fn total_agents_lte(mut self, n: i64) -> Self {
        self.total_agents_lte = Some(n);
        self
    }
    /// Total agents (more than).
    pub fn total_agents_gt(mut self, n: i64) -> Self {
        self.total_agents_gt = Some(n);
        self
    }
    /// Total agents (more than or equal).
    pub fn total_agents_gte(mut self, n: i64) -> Self {
        self.total_agents_gte = Some(n);
        self
    }
    /// The total of non decommissioned agents in the account. Example: "2-8".
    pub fn total_agents_between(mut self, v: impl Into<String>) -> Self {
        self.total_agents_between = Some(v.into());
        self
    }
    /// Connected rangers (less than).
    pub fn connected_rangers_lt(mut self, n: i64) -> Self {
        self.connected_rangers_lt = Some(n);
        self
    }
    /// Connected rangers (less than or equal).
    pub fn connected_rangers_lte(mut self, n: i64) -> Self {
        self.connected_rangers_lte = Some(n);
        self
    }
    /// Connected rangers (more than).
    pub fn connected_rangers_gt(mut self, n: i64) -> Self {
        self.connected_rangers_gt = Some(n);
        self
    }
    /// Connected rangers (more than or equal).
    pub fn connected_rangers_gte(mut self, n: i64) -> Self {
        self.connected_rangers_gte = Some(n);
        self
    }
    /// The total of non decommissioned agents in the account. Example: "2-8".
    pub fn connected_rangers_between(mut self, v: impl Into<String>) -> Self {
        self.connected_rangers_between = Some(v.into());
        self
    }
    /// Gateway created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Gateway created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Gateway created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Gateway created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Date range for creation time (format:
    /// `<from_timestamp>-<to_timestamp>`, inclusive).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// Gateway updated before this timestamp.
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lt = Some(v.into());
        self
    }
    /// Gateway updated before or at this timestamp.
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// Gateway updated after this timestamp.
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gt = Some(v.into());
        self
    }
    /// Gateway updated after or at this timestamp.
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// Date range for update time (format: `<from_timestamp>-<to_timestamp>`,
    /// inclusive).
    pub fn updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.updated_at_between = Some(v.into());
        self
    }
    /// SMB scan enabled.
    pub fn smb_scan(mut self, v: bool) -> Self {
        self.smb_scan = Some(v);
        self
    }
    /// ICMP scan enabled.
    pub fn icmp_scan(mut self, v: bool) -> Self {
        self.icmp_scan = Some(v);
        self
    }
    /// MDNS scan enabled.
    pub fn mdns_scan(mut self, v: bool) -> Self {
        self.mdns_scan = Some(v);
        self
    }
    /// SNMP scan enabled.
    pub fn snmp_scan(mut self, v: bool) -> Self {
        self.snmp_scan = Some(v);
        self
    }
    /// RDNS scan enabled.
    pub fn rdns_scan(mut self, v: bool) -> Self {
        self.rdns_scan = Some(v);
        self
    }
    /// The gateway manufacturer obtained from the mac address.
    pub fn manufacturer(mut self, v: impl Into<String>) -> Self {
        self.manufacturer = Some(v.into());
        self
    }
    /// Free-text filter by network name (supports multiple values).
    pub fn network_name_contains<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name_contains = Some(join_comma(items));
        self
    }
    /// Free-text filter by visible IP (supports multiple values).
    pub fn external_ip_contains<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip_contains = Some(join_comma(items));
        self
    }
    /// Free-text filter by IP Address (supports multiple values).
    pub fn ip_contains<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_contains = Some(join_comma(items));
        self
    }
    /// Free-text filter by manufacturer (supports multiple values).
    pub fn manufacturer_contains<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_contains = Some(join_comma(items));
        self
    }
    /// Free-text filter by mac address (supports multiple values).
    pub fn mac_address_contains<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_address_contains = Some(join_comma(items));
        self
    }
    /// Free-text filter by tcp port (supports multiple values).
    pub fn tcp_ports_contains<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports_contains = Some(join_comma(items));
        self
    }
    /// Free-text filter by udp port (supports multiple values).
    pub fn udp_ports_contains<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports_contains = Some(join_comma(items));
        self
    }
}

fn join_comma<I, S>(items: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    items
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

/// `data` object for `POST /web/api/v2.1/ranger/gateways/update`.
///
/// The settings to apply to the filtered gateways. All fields optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGatewaysData {
    /// Do we allow scanning in this network. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_scan: Option<bool>,
    /// True if we should archive the network, valid for networks that are not
    /// allowed to scan only. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// Allow remote tasks form this network. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_only_local_subnets: Option<bool>,
    /// True if inherited values are taken from account settings. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_settings: Option<bool>,
}

/// Request body for `POST /web/api/v2.1/ranger/gateways/update`.
///
/// Both `data` and `filter` are required by the spec. The `filter` selects
/// which gateways to update; its shape mirrors the `GET` query params, so it is
/// modelled as a freeform [`serde_json::Value`] (e.g.
/// `serde_json::json!({ "ids": ["225494730938493804"] })`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGatewaysBody {
    /// The settings to apply. Required.
    pub data: UpdateGatewaysData,
    /// The filter selecting which gateways to update. Required. Freeform
    /// object; mirrors the `GET /web/api/v2.1/ranger/gateways` query params
    /// (e.g. `siteIds`, `accountIds`, `ids`, `query`, ...).
    pub filter: serde_json::Value,
}

/// `data` object for `PUT /web/api/v2.1/ranger/gateways/{gateway_id}`.
///
/// The Network Discovery scan configuration to set on the gateway. All fields
/// optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGatewayData {
    /// Do we allow scanning in this network. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_scan: Option<bool>,
    /// Allowed TCP ports. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<Vec<i64>>,
    /// Allowed UDP ports. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<Vec<i64>>,
    /// The Account Id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<i64>,
    /// The Site Id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<i64>,
    /// ICMP scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icmp_scan: Option<bool>,
    /// SMB scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb_scan: Option<bool>,
    /// MDNS scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdns_scan: Option<bool>,
    /// RDNS scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdns_scan: Option<bool>,
    /// SNMP scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snmp_scan: Option<bool>,
    /// TCP Port scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_port_scan: Option<bool>,
    /// UDP Port scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_port_scan: Option<bool>,
    /// Archived network. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    /// A set of IP addresses that should not be scanned in the specific
    /// network. Freeform objects (each entry has `type` one of `ip`/`cidr`/
    /// `range`, `values`, and an optional `annotation`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Vec<serde_json::Value>>,
    /// The network name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// Can we scan remote networks from this gateway. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_only_local_subnets: Option<bool>,
    /// True if inherited values are taken from account settings. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_settings: Option<bool>,
    /// Multicast SSDP scan enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_cast_ssdp_scan: Option<bool>,
}

/// Request body for `PUT /web/api/v2.1/ranger/gateways/{gateway_id}`.
///
/// `data` is required by the spec.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateGatewayBody {
    /// The scan configuration to set. Required.
    pub data: UpdateGatewayData,
}

impl GatewaysService<'_> {
    /// `GET /web/api/v2.1/ranger/gateways` — Get Gateways.
    ///
    /// Get the gateways in your deployment that match the filter from a Network
    /// Discovery scan. Network Discovery requires a Network Discovery license.
    pub async fn list(
        &self,
        query: &ListGatewaysQuery,
    ) -> Result<Paginated<Gateway>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/ranger/gateways", q)
            .await?)
    }

    /// `POST /web/api/v2.1/ranger/gateways/update` — Update Gateways.
    ///
    /// Change the status of filtered gateways discovered by Network Discovery.
    /// You can set the archived status, whether the network behind the gateway
    /// may be scanned by Network Discovery, and whether Network Discovery will
    /// scan only local networks.
    pub async fn update_gateways(
        &self,
        body: &UpdateGatewaysBody,
    ) -> Result<Response<GatewayAffected>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/ranger/gateways/update", body)
            .await?)
    }

    /// `PUT /web/api/v2.1/ranger/gateways/{gateway_id}` — Update Gateway.
    ///
    /// Change the Network Discovery scan configuration for a gateway that
    /// Network Discovery discovered.
    ///
    /// `gateway_id`: Gateway ID. Example: "225494730938493804".
    pub async fn update_gateway(
        &self,
        gateway_id: impl Into<String>,
        body: &UpdateGatewayBody,
    ) -> Result<Response<Gateway>, Error> {
        let path = format!(
            "/web/api/v2.1/ranger/gateways/{}",
            gateway_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<UpdateGatewayBody, Response<Gateway>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }
}
