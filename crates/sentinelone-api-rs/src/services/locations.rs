//! Service for the `Locations` tag — define and manage locations for endpoints.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::locations::{Location, LocationAffected};
use crate::pagination::{Paginated, Response};

/// `Locations` tag — define and manage locations for endpoints.
pub struct LocationsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query types
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/locations` (Get Locations).
///
/// Array params (e.g. `ids`, `siteIds`, `*__contains`) are serialized
/// comma-joined, as the API expects. Every field is optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of
    /// the actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `id`, `scope`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Filter results by location IDs (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Filter results by scope (comma-joined). Allowed values: `global`,
    /// `group`, `account`, `site`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    /// Filter by locations with/without firewall rules associated to them.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_firewall_rules: Option<bool>,
    /// Free-text filter by location name (comma-joined, supports multiple
    /// values). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by description (comma-joined, supports multiple
    /// values). Optional.
    #[serde(
        rename = "description__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub description_contains: Option<String>,
    /// Free-text filter by creator of the location (comma-joined, supports
    /// multiple values). Optional.
    #[serde(rename = "creator__contains", skip_serializing_if = "Option::is_none")]
    pub creator_contains: Option<String>,
    /// Free-text filter by scope name (comma-joined, supports multiple
    /// values). Optional.
    #[serde(
        rename = "scopeName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub scope_name_contains: Option<String>,
    /// Free-text filter by hostname (comma-joined, supports multiple values).
    /// Optional.
    #[serde(rename = "hostname__contains", skip_serializing_if = "Option::is_none")]
    pub hostname_contains: Option<String>,
    /// Free-text filter by IP address (comma-joined, supports multiple
    /// values). Optional.
    #[serde(
        rename = "ipAddress__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by registry key (comma-joined, supports multiple
    /// values). Optional.
    #[serde(
        rename = "registryKey__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub registry_key_contains: Option<String>,
}

impl ListQuery {
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
    /// If true, only the total number of items is returned.
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }
    /// If true, the total number of items is not calculated.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `scope`.
    pub fn sort_by(mut self, s: impl Into<String>) -> Self {
        self.sort_by = Some(s.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, s: impl Into<String>) -> Self {
        self.sort_order = Some(s.into());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Filter results by location IDs.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// Filter results by scope. Allowed values: `global`, `group`, `account`,
    /// `site`.
    pub fn scopes<I, S>(mut self, scopes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scopes = Some(join_csv(scopes));
        self
    }
    /// Filter by locations with/without firewall rules associated to them.
    pub fn has_firewall_rules(mut self, b: bool) -> Self {
        self.has_firewall_rules = Some(b);
        self
    }
    /// Free-text filter by location name (supports multiple values).
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by description (supports multiple values).
    pub fn description_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by creator of the location (supports multiple values).
    pub fn creator_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.creator_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by scope name (supports multiple values).
    pub fn scope_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_name_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by hostname (supports multiple values).
    pub fn hostname_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostname_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by IP address (supports multiple values).
    pub fn ip_address_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by registry key (supports multiple values).
    pub fn registry_key_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.registry_key_contains = Some(join_csv(values));
        self
    }
}

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

// ---------------------------------------------------------------------------
// Body types
// ---------------------------------------------------------------------------

/// Request body for `POST /web/api/v2.1/locations` (Create Location).
#[derive(Debug, Clone, Serialize)]
pub struct CreateBody {
    /// Location scope (`siteIds` / `accountIds`). Required.
    pub filter: serde_json::Value,
    /// Location data (name, operator, and the location identifier objects:
    /// `ipAddresses`, `dnsServers`, `dnsLookup`, `networkInterfaces`,
    /// `serverConnectivity`, `registryKeys`). `name` and `operator` are
    /// required. Required.
    pub data: serde_json::Value,
}

/// Request body for `PUT /web/api/v2.1/locations/{location_id}`
/// (Update Location).
#[derive(Debug, Clone, Serialize)]
pub struct UpdateBody {
    /// Location data (name plus the location identifier objects). `name` is
    /// required within this object. Required.
    pub data: serde_json::Value,
}

/// Request body for `DELETE /web/api/v2.1/locations` (Delete Locations).
#[derive(Debug, Clone, Serialize)]
pub struct DeleteBody {
    /// Wrapper holding the list of location IDs to delete. Required.
    pub data: DeleteBodyData,
}

/// `data` object for [`DeleteBody`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct DeleteBodyData {
    /// List of location IDs to delete (max 5000). Required.
    pub ids: Vec<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

impl LocationsService<'_> {
    /// `GET /web/api/v2.1/locations` — Get Locations.
    ///
    /// Get the locations of Agents in a given scope that match the filter.
    /// Agent locations are based on endpoint network parameters (IP, DNS, NIC,
    /// Registry Key, or SentinelOne connection set for all true, at least one
    /// true, or none true and applied to a Site, Account, or Global). Agents
    /// detect their location settings and apply Firewall Control rules that
    /// have Location Aware parameters that match the Agent location. Agents
    /// can be in multiple locations at the same time. If an Agent that
    /// supports Locations does not detect that it is in a defined location, it
    /// uses the Firewall rules assigned to the Fallback location. Use this
    /// command with a filter for "hasFirewallRules" to find Locations that do
    /// not have matching Firewall Control rules. The response to this request
    /// includes the ID of the location, which you can use in other commands.
    /// Firewall Control and Location Awareness require Control SKU.
    pub async fn list(&self, query: &ListQuery) -> Result<Paginated<Location>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/locations", q).await?)
    }

    /// `POST /web/api/v2.1/locations` — Create Location.
    ///
    /// Create a location that defines parameters of Agents in a scope filter.
    /// Parameters include: `ipAddresses` (the Agent compares the endpoint
    /// active IPv4 or IPv6 addresses to the IP addresses, ranges, and CIDRs
    /// defined for the location); `dnsServers` (the Agent compares the
    /// configured DNS servers of the endpoint to the DNS servers defined for
    /// the location); `dnsLookup` (the Agent resolves the FQDN of the endpoint
    /// to IPv4 or IPv6 addresses and compares them to the addresses configured
    /// in the location setting); `networkInterfaces` (the Agent determines if
    /// the endpoint is connected to the network over a wireless connection; if
    /// one connected interface is wireless, the endpoint is considered
    /// wireless); `serverConnectivity` (the Agent reports if it is connected to
    /// its Management); `registryKeys` (the Agent compares the endpoint
    /// registry keys in HKEY_LOCAL_MACHINE\SOFTWARE with the registry key of
    /// the location definition). When you set a location parameter, also set
    /// the operator to ALL, NONE, or at least 1. The serverConnectivity
    /// parameter takes "enabled" (true or false) and "value" (connected or
    /// disconnected). The networkInterfaces parameter takes "enabled" (true or
    /// false) and "value" (wired or wireless).
    pub async fn create(&self, body: &CreateBody) -> Result<Response<Location>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/locations", body).await?)
    }

    /// `PUT /web/api/v2.1/locations/{location_id}` — Update Location.
    ///
    /// Change the parameter values of a location definition. See Create
    /// Location.
    ///
    /// `location_id`: Location ID. Example: "225494730938493804". Required.
    pub async fn update(
        &self,
        location_id: impl Into<String>,
        body: &UpdateBody,
    ) -> Result<Response<Location>, Error> {
        let path = format!("/web/api/v2.1/locations/{}", location_id.into());
        Ok(self
            .client
            .http()
            .request_json::<UpdateBody, Response<Location>>(Method::PUT, &path, None, Some(body))
            .await?)
    }

    /// `DELETE /web/api/v2.1/locations` — Delete Locations.
    ///
    /// Delete location definitions of a given location. To get location IDs,
    /// run "locations".
    pub async fn delete(&self, body: &DeleteBody) -> Result<Response<LocationAffected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteBody, Response<LocationAffected>>(
                Method::DELETE,
                "/web/api/v2.1/locations",
                None,
                Some(body),
            )
            .await?)
    }
}
