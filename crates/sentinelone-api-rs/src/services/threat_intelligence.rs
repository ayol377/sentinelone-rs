//! Service for the `Threat Intelligence` tag — managing Threat Intelligence.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::threat_intelligence::{
    ThreatIntelligenceAffected, ThreatIntelligenceIndicator, ThreatIntelligenceUserConfig,
};
use crate::pagination::{Paginated, Response};

/// `Threat Intelligence` tag — managing Threat Intelligence.
pub struct ThreatIntelligenceService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query param structs
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/threat-intelligence/iocs`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetIocsQuery {
    /// Free-text filter by the user uploaded the Threat Intelligence indicator
    /// (supports multiple values). Example: `"admin@sentinelone.com"`.
    /// Optional. Array param (comma-joined).
    #[serde(rename = "creator__contains", skip_serializing_if = "Option::is_none")]
    pub creator_contains: Option<String>,
    /// Creation Time as set by the user lesser than (date-time).
    /// Example: `"2021-07-13T20:33:29.007906Z"`. Optional.
    #[serde(rename = "creationTime__lt", skip_serializing_if = "Option::is_none")]
    pub creation_time_lt: Option<String>,
    /// Creation Time as set by the user lesser or equal than (date-time).
    /// Example: `"2021-07-11T20:33:29.007906Z"`. Optional.
    #[serde(rename = "creationTime__lte", skip_serializing_if = "Option::is_none")]
    pub creation_time_lte: Option<String>,
    /// A list of severities to filter by (0-7). Optional. Array param
    /// (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// The time at which the Threat Intelligence indicator was uploaded to
    /// SentinelOne DB lesser or equal than (date-time).
    /// Example: `"2022-07-13T20:33:29.007906Z"`. Optional.
    #[serde(rename = "uploadTime__lte", skip_serializing_if = "Option::is_none")]
    pub upload_time_lte: Option<String>,
    /// A list of unique Ids of the parent process of the indicator of
    /// compromise. Optional. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuids: Option<String>,
    /// List of Account IDs to filter by. Optional. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Free-text filter by the Indicator name (supports multiple values).
    /// Example: `"foo.dll"`. Optional. Array param (comma-joined).
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The time at which the indicator was last updated in SentinelOne DB
    /// greater than (date-time). Example: `"2021-07-13T20:33:29.007906Z"`.
    /// Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at_gt: Option<String>,
    /// A list of malware names to filter by. Optional. Array param
    /// (comma-joined).
    #[serde(rename = "malwareNames__in", skip_serializing_if = "Option::is_none")]
    pub malware_names_in: Option<String>,
    /// The column to sort the results by. Example: `"id"`. Optional.
    /// Allowed values: `id`, `creationTime`, `uploadTime`, `updatedAt`,
    /// `source`, `type`. Note: sorting by `creationTime` is deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Creation Time as set by the user greater than (date-time).
    /// Example: `"2021-07-12T20:33:29.007906Z"`. Optional.
    #[serde(rename = "creationTime__gt", skip_serializing_if = "Option::is_none")]
    pub creation_time_gt: Option<String>,
    /// List of threat actor types associated with the indicator. Optional.
    /// Array param (comma-joined).
    #[serde(
        rename = "threatActorTypes__in",
        skip_serializing_if = "Option::is_none"
    )]
    pub threat_actor_types_in: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: `"YWdlbnRfaWQ6NTgwMjkzODE="`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// List of labels associated with the indicator. Optional. Array param
    /// (comma-joined).
    #[serde(rename = "labels__in", skip_serializing_if = "Option::is_none")]
    pub labels_in: Option<String>,
    /// The type of the Threat Intelligence indicator. Example: `"IPv4"`.
    /// Optional. Allowed values: `DNS`, `IPV4`, `IPV6`, `MD5`, `SHA1`,
    /// `SHA256`, `URL`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// A list of threat actors to filter by. Optional. Array param
    /// (comma-joined).
    #[serde(rename = "threatActors__in", skip_serializing_if = "Option::is_none")]
    pub threat_actors_in: Option<String>,
    /// The time at which the indicator was last updated in SentinelOne DB
    /// lesser than (date-time). Example: `"2021-07-13T20:33:29.007906Z"`.
    /// Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at_lt: Option<String>,
    /// The time at which the Threat Intelligence indicator was uploaded to
    /// SentinelOne DB greater or equal than (date-time).
    /// Example: `"2022-07-13T20:33:29.007906Z"`. Optional.
    #[serde(rename = "uploadTime__gte", skip_serializing_if = "Option::is_none")]
    pub upload_time_gte: Option<String>,
    /// The time at which the indicator was last updated in SentinelOne DB
    /// greater or equal than (date-time).
    /// Example: `"2021-07-13T20:33:29.007906Z"`. Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// Unique ID of the uploaded indicators batch.
    /// Example: `"atmtn000000028a881bcf939dc6d92ab55443"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    /// List of campaign names associated with the indicator. Optional. Array
    /// param (comma-joined).
    #[serde(rename = "campaignNames__in", skip_serializing_if = "Option::is_none")]
    pub campaign_names_in: Option<String>,
    /// The time at which the Threat Intelligence indicator was uploaded to
    /// SentinelOne DB lesser than (date-time).
    /// Example: `"2021-07-13T20:33:29.007906Z"`. Optional.
    #[serde(rename = "uploadTime__lt", skip_serializing_if = "Option::is_none")]
    pub upload_time_lt: Option<String>,
    /// Creation Time as set by the user greater or equal than (date-time).
    /// Example: `"2021-07-13T20:33:29.007906Z"`. Optional.
    #[serde(rename = "creationTime__gte", skip_serializing_if = "Option::is_none")]
    pub creation_time_gte: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The time at which the indicator was last updated in SentinelOne DB
    /// lesser or equal than (date-time).
    /// Example: `"2021-07-13T20:33:29.007906Z"`. Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// Limit number of returned items (1-1000). Example: `"10"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// List of the sources of the identified Threat Intelligence indicator.
    /// Example: `"AlienVault"`. Optional. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Free-text filter by the description of the indicator (supports multiple
    /// values). Example: `"Malicious-activity"`. Optional. Array param
    /// (comma-joined).
    #[serde(
        rename = "description__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub description_contains: Option<String>,
    /// The time at which the Threat Intelligence indicator was uploaded to
    /// SentinelOne DB greater than (date-time).
    /// Example: `"2022-07-13T20:33:29.007906Z"`. Optional.
    #[serde(rename = "uploadTime__gt", skip_serializing_if = "Option::is_none")]
    pub upload_time_gt: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `"150"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of Site IDs to filter by. Optional. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Sort direction. Example: `"asc"`. Optional. Allowed values: `asc`,
    /// `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The categories of the Threat Intelligence indicator, e.g. the malware
    /// type associated with the IOC. Optional.
    #[serde(rename = "category__in", skip_serializing_if = "Option::is_none")]
    pub category_in: Option<String>,
    /// The unique identifier of the indicator as provided by the Threat
    /// Intelligence source. Example: `"e277603e-1060-5ad4-9937-c26c97f1ca68"`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The value of the Threat Intelligence indicator.
    /// Example: `"175.45.176.1"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl GetIocsQuery {
    /// Free-text filter by the user uploaded the Threat Intelligence indicator
    /// (supports multiple values). Array param (comma-joined).
    pub fn creator_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.creator_contains = Some(join(v));
        self
    }
    /// Creation Time as set by the user lesser than (date-time).
    pub fn creation_time_lt(mut self, v: impl Into<String>) -> Self {
        self.creation_time_lt = Some(v.into());
        self
    }
    /// Creation Time as set by the user lesser or equal than (date-time).
    pub fn creation_time_lte(mut self, v: impl Into<String>) -> Self {
        self.creation_time_lte = Some(v.into());
        self
    }
    /// A list of severities to filter by (0-7). Array param (comma-joined).
    pub fn severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.severity = Some(join(v));
        self
    }
    /// The time at which the indicator was uploaded to SentinelOne DB lesser or
    /// equal than (date-time).
    pub fn upload_time_lte(mut self, v: impl Into<String>) -> Self {
        self.upload_time_lte = Some(v.into());
        self
    }
    /// A list of unique Ids of the parent process of the IOC. Array param
    /// (comma-joined).
    pub fn uuids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuids = Some(join(v));
        self
    }
    /// List of Account IDs to filter by. Array param (comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// Free-text filter by the Indicator name (supports multiple values). Array
    /// param (comma-joined).
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join(v));
        self
    }
    /// The time at which the indicator was last updated in SentinelOne DB
    /// greater than (date-time).
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gt = Some(v.into());
        self
    }
    /// A list of malware names to filter by. Array param (comma-joined).
    pub fn malware_names_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.malware_names_in = Some(join(v));
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `creationTime`,
    /// `uploadTime`, `updatedAt`, `source`, `type`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Creation Time as set by the user greater than (date-time).
    pub fn creation_time_gt(mut self, v: impl Into<String>) -> Self {
        self.creation_time_gt = Some(v.into());
        self
    }
    /// List of threat actor types associated with the indicator. Array param
    /// (comma-joined).
    pub fn threat_actor_types_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_actor_types_in = Some(join(v));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// List of labels associated with the indicator. Array param (comma-joined).
    pub fn labels_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.labels_in = Some(join(v));
        self
    }
    /// The type of the Threat Intelligence indicator. Allowed values: `DNS`,
    /// `IPV4`, `IPV6`, `MD5`, `SHA1`, `SHA256`, `URL`.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    /// A list of threat actors to filter by. Array param (comma-joined).
    pub fn threat_actors_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_actors_in = Some(join(v));
        self
    }
    /// The time at which the indicator was last updated in SentinelOne DB
    /// lesser than (date-time).
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lt = Some(v.into());
        self
    }
    /// The time at which the indicator was uploaded to SentinelOne DB greater
    /// or equal than (date-time).
    pub fn upload_time_gte(mut self, v: impl Into<String>) -> Self {
        self.upload_time_gte = Some(v.into());
        self
    }
    /// The time at which the indicator was last updated in SentinelOne DB
    /// greater or equal than (date-time).
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// Unique ID of the uploaded indicators batch.
    pub fn batch_id(mut self, v: impl Into<String>) -> Self {
        self.batch_id = Some(v.into());
        self
    }
    /// List of campaign names associated with the indicator. Array param
    /// (comma-joined).
    pub fn campaign_names_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.campaign_names_in = Some(join(v));
        self
    }
    /// The time at which the indicator was uploaded to SentinelOne DB lesser
    /// than (date-time).
    pub fn upload_time_lt(mut self, v: impl Into<String>) -> Self {
        self.upload_time_lt = Some(v.into());
        self
    }
    /// Creation Time as set by the user greater or equal than (date-time).
    pub fn creation_time_gte(mut self, v: impl Into<String>) -> Self {
        self.creation_time_gte = Some(v.into());
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The time at which the indicator was last updated in SentinelOne DB
    /// lesser or equal than (date-time).
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// List of the sources of the identified Threat Intelligence indicator.
    /// Array param (comma-joined).
    pub fn source<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source = Some(join(v));
        self
    }
    /// Free-text filter by the description of the indicator (supports multiple
    /// values). Array param (comma-joined).
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join(v));
        self
    }
    /// The time at which the indicator was uploaded to SentinelOne DB greater
    /// than (date-time).
    pub fn upload_time_gt(mut self, v: impl Into<String>) -> Self {
        self.upload_time_gt = Some(v.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// List of Site IDs to filter by. Array param (comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// The categories of the Threat Intelligence indicator.
    pub fn category_in(mut self, v: impl Into<String>) -> Self {
        self.category_in = Some(v.into());
        self
    }
    /// The unique identifier of the indicator as provided by the Threat
    /// Intelligence source.
    pub fn external_id(mut self, v: impl Into<String>) -> Self {
        self.external_id = Some(v.into());
        self
    }
    /// The value of the Threat Intelligence indicator.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/threat-intelligence/user-config`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserConfigQuery {
    /// List of Site IDs to filter by. Optional. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Optional. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl GetUserConfigQuery {
    /// List of Site IDs to filter by. Array param (comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join(v));
        self
    }
    /// List of Account IDs to filter by. Array param (comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
}

// ---------------------------------------------------------------------------
// Body structs
// ---------------------------------------------------------------------------

/// Request body for `DELETE /web/api/v2.1/threat-intelligence/iocs`.
///
/// Delete an IoC from the Threat Intelligence database that matches a filter
/// using the `accountID` and one other field.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteIocsBody {
    /// Filter selecting the IOC(s) to delete. Required. The filter accepts the
    /// same fields as the GET IOCs query (e.g. `accountIds`, `value`, `type`,
    /// `uuids`, time-range filters, etc.); represented as freeform JSON for
    /// full fidelity.
    pub filter: serde_json::Value,
}

impl DeleteIocsBody {
    /// Build a delete body from a freeform `filter` object.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { filter }
    }
}

/// Request body for `POST /web/api/v2.1/threat-intelligence/iocs`.
///
/// Add IoCs to the Threat Intelligence database. Each entry under `data`
/// requires `source`, `type`, `value`, and `method`; `type` and `method` must
/// be upper case. See the endpoint docs for `validUntil` expiration behaviour.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIocsBody {
    /// The list of indicators to create. Required. Each item is a freeform JSON
    /// object; required keys per item: `source`, `type`, `value` (and `method`
    /// per the endpoint description).
    pub data: Vec<serde_json::Value>,
    /// Scope filter (`groupIds` / `siteIds` / `accountIds` / `tenant`).
    /// Optional. Represented as freeform JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl CreateIocsBody {
    /// Build a create body from the list of indicator objects.
    pub fn new(data: Vec<serde_json::Value>) -> Self {
        Self { data, filter: None }
    }
    /// Set the scope filter (`groupIds` / `siteIds` / `accountIds` / `tenant`).
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Request body for `POST /web/api/v2.1/threat-intelligence/iocs/stix`.
///
/// Add IOCs to the Threat Intelligence database from a STIX 2.1 bundle.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIocsStixBody {
    /// A valid STIX 2.1 bundle containing indicator objects. Required. Must
    /// contain an `objects` array; represented as freeform JSON.
    pub bundle: serde_json::Value,
    /// Scope filter (`groupIds` / `siteIds` / `accountIds` / `tenant`).
    /// Optional. Represented as freeform JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl CreateIocsStixBody {
    /// Build a STIX create body from the bundle object.
    pub fn new(bundle: serde_json::Value) -> Self {
        Self {
            bundle,
            filter: None,
        }
    }
    /// Set the scope filter (`groupIds` / `siteIds` / `accountIds` / `tenant`).
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Request body for `DELETE /web/api/v2.1/threat-intelligence/user-config`.
///
/// Delete Threat Intelligence user config that matches the filter.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUserConfigBody {
    /// List of Site IDs to filter by. Optional. Array param (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl DeleteUserConfigBody {
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
}

/// Request body for `POST /web/api/v2.1/threat-intelligence/user-config`.
///
/// Create Threat Intelligence user config.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserConfigBody {
    /// The user config payload (`disableRh`, `threatExcludeFields`,
    /// `description`, `threatMinScore`, `excludeTii`, `enableXdrMatching`,
    /// `disableThreat`). Required. Represented as freeform JSON.
    pub data: serde_json::Value,
    /// Scope filter (`groupIds` / `siteIds` / `accountIds` / `tenant`).
    /// Required. Represented as freeform JSON.
    pub filter: serde_json::Value,
}

impl CreateUserConfigBody {
    /// Build a user-config create body from the `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl ThreatIntelligenceService<'_> {
    /// `GET /web/api/v2.1/threat-intelligence/iocs` — Get IOCs.
    ///
    /// Get the IOCs of a specified Account that match the filter.
    ///
    /// Note: Using `creationTime` to sort results has been deprecated and
    /// should not be used. In the future, the ability to sort by
    /// `creationTime` will be removed. Please sort by `uploadTime` or
    /// `updatedAt` as an alternative.
    pub async fn get_iocs(
        &self,
        query: &GetIocsQuery,
    ) -> Result<Paginated<ThreatIntelligenceIndicator>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/threat-intelligence/iocs", q)
            .await?)
    }

    /// `POST /web/api/v2.1/threat-intelligence/iocs` — Create IOCs.
    ///
    /// Add an IoC to the Threat Intelligence database. These values under
    /// `data` are required fields: `source`, `type`, `value`, and `method`.
    /// `type` and `method` must be in upper case. The `validUntil` field is
    /// mandatory and must contain a date (e.g. `2021-03-20 09:14:47.779000`);
    /// it determines when the IOC expires. If left blank it defaults to the
    /// upload date plus a default offset (14 days for IPs, 90 days for URLs and
    /// domains, 180 days for file hashes), capped at the maximum offset
    /// (30 days for IPs, 180 days for URLs/domains/hashes).
    pub async fn create_iocs(
        &self,
        body: &CreateIocsBody,
    ) -> Result<Response<Vec<ThreatIntelligenceIndicator>>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/threat-intelligence/iocs", body)
            .await?)
    }

    /// `POST /web/api/v2.1/threat-intelligence/iocs/stix` — Create IOCs from
    /// STIX bundle.
    ///
    /// Add IOCs to the Threat Intelligence database from a STIX 2.1 bundle.
    /// The API transforms STIX indicators into Threat Intelligence IOCs. The
    /// bundle must be a valid STIX 2.1 bundle containing indicator objects;
    /// each indicator object should have a valid STIX pattern for one of: file
    /// hashes (MD5/SHA-1/SHA-256), IPv4 addresses, domains, or URLs. Optional
    /// STIX relationships (threat actors and types, malware names, campaign
    /// names, intrusion sets) are processed if present. Unsupported objects or
    /// patterns are ignored. `validUntil` expiration behaviour matches the
    /// Create IOCs endpoint.
    pub async fn create_iocs_stix(
        &self,
        body: &CreateIocsStixBody,
    ) -> Result<Response<Vec<ThreatIntelligenceIndicator>>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/threat-intelligence/iocs/stix", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/threat-intelligence/iocs` — Delete IOCs.
    ///
    /// Delete an IoC from the Threat Intelligence database that matches a
    /// filter using the `accountID` and one other field.
    pub async fn delete_iocs(
        &self,
        body: &DeleteIocsBody,
    ) -> Result<Response<ThreatIntelligenceAffected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteIocsBody, Response<ThreatIntelligenceAffected>>(
                Method::DELETE,
                "/web/api/v2.1/threat-intelligence/iocs",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/threat-intelligence/user-config` — Get Threat
    /// Intelligence user config.
    ///
    /// Get the Threat Intelligence user config that matches the filter.
    ///
    /// Note: this endpoint returns a non-paginated `{ data: [...], errors }`
    /// envelope (no `pagination` block), hence `Response<Vec<_>>`.
    pub async fn get_user_config(
        &self,
        query: &GetUserConfigQuery,
    ) -> Result<Response<Vec<ThreatIntelligenceUserConfig>>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/threat-intelligence/user-config", q)
            .await?)
    }

    /// `POST /web/api/v2.1/threat-intelligence/user-config` — Create Threat
    /// Intelligence user config.
    ///
    /// Create Threat Intelligence user config.
    pub async fn create_user_config(
        &self,
        body: &CreateUserConfigBody,
    ) -> Result<Response<Vec<ThreatIntelligenceUserConfig>>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/threat-intelligence/user-config", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/threat-intelligence/user-config` — Delete Threat
    /// Intelligence user config.
    ///
    /// Delete Threat Intelligence user config that matches the filter.
    pub async fn delete_user_config(
        &self,
        body: &DeleteUserConfigBody,
    ) -> Result<Response<ThreatIntelligenceAffected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteUserConfigBody, Response<ThreatIntelligenceAffected>>(
                Method::DELETE,
                "/web/api/v2.1/threat-intelligence/user-config",
                None,
                Some(body),
            )
            .await?)
    }
}

/// Join an iterator of stringy values by comma for array query params.
fn join<I, S>(values: I) -> String
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
