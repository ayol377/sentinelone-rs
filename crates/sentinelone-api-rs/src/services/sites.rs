//! Service for the `Sites` tag — Sites related endpoints.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::sites::{
    SiteAffected, SiteApproval, SitePutApproval, SiteRegenerateKey, SiteSuccess, SiteToken,
    SitesListData,
};
use crate::pagination::Response;

/// `Sites` tag — Sites related endpoints.
pub struct SitesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/export/sites` — **Export Sites**.
///
/// Every field is optional. Array params are serialized as a single
/// comma-joined string, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSitesQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Full text search for fields: name, account_name, description. (Note: on
    /// single-account consoles account name will not be matched).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Name. Example: "My Site".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Is default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Health status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<bool>,
    /// Site type. Allowed values: `Trial`, `Paid`. Example: "Trial".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_type: Option<String>,
    /// Expiration. Example: "2018-02-27T04:49:26.257525Z".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Total licenses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_licenses: Option<i64>,
    /// Active licenses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_licenses: Option<i64>,
    /// Id in a CRM external system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The description for the Site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Timestamp of site creation. Example: "2018-02-27T04:49:26.257525Z".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Timestamp of last update. Example: "2018-02-27T04:49:26.257525Z".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Site state. Allowed values: `active`, `expired`, `deleted`. Example: "active".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// List of states to filter (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<String>,
    /// List of states to not filter (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_nin: Option<String>,
    /// [DEPRECATED] Use sku instead. Allowed values: `Core`, `Control`, `Complete`. Example: "Core".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite: Option<String>,
    /// If sent, return only sites that support these features. Allowed values:
    /// `firewall-control`, `device-control`, `ioc` (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    /// Sku. Example: "core".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Module. Example: "star,rso".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Account id. Example: "225494730938493804".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// [DEPRECATED] Show sites the user has Admin privileges to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_only: Option<bool>,
    /// Only return sites the user can move agents to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_move_sites: Option<bool>,
    /// Registration token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_token: Option<String>,
    /// Free-text filter by account name (supports multiple values, comma-joined).
    #[serde(rename = "accountName__contains", skip_serializing_if = "Option::is_none")]
    pub account_name_contains: Option<String>,
    /// Free-text filter by site name (supports multiple values, comma-joined).
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by site description (supports multiple values, comma-joined).
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
}

impl ExportSitesQuery {
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
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
    /// Full text search for fields: name, account_name, description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// Is default.
    pub fn is_default(mut self, v: bool) -> Self {
        self.is_default = Some(v);
        self
    }
    /// Health status.
    pub fn health_status(mut self, v: bool) -> Self {
        self.health_status = Some(v);
        self
    }
    /// Site type. Allowed values: `Trial`, `Paid`.
    pub fn site_type(mut self, v: impl Into<String>) -> Self {
        self.site_type = Some(v.into());
        self
    }
    /// Expiration.
    pub fn expiration(mut self, v: impl Into<String>) -> Self {
        self.expiration = Some(v.into());
        self
    }
    /// Total licenses.
    pub fn total_licenses(mut self, v: i64) -> Self {
        self.total_licenses = Some(v);
        self
    }
    /// Active licenses.
    pub fn active_licenses(mut self, v: i64) -> Self {
        self.active_licenses = Some(v);
        self
    }
    /// Id in a CRM external system.
    pub fn external_id(mut self, v: impl Into<String>) -> Self {
        self.external_id = Some(v.into());
        self
    }
    /// The description for the Site.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// Timestamp of site creation.
    pub fn created_at(mut self, v: impl Into<String>) -> Self {
        self.created_at = Some(v.into());
        self
    }
    /// Timestamp of last update.
    pub fn updated_at(mut self, v: impl Into<String>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
    /// Site state. Allowed values: `active`, `expired`, `deleted`.
    pub fn state(mut self, v: impl Into<String>) -> Self {
        self.state = Some(v.into());
        self
    }
    /// List of states to filter.
    pub fn states<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states = Some(join_csv(v));
        self
    }
    /// List of states to not filter.
    pub fn states_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states_nin = Some(join_csv(v));
        self
    }
    /// [DEPRECATED] Use sku instead. Allowed values: `Core`, `Control`, `Complete`.
    pub fn suite(mut self, v: impl Into<String>) -> Self {
        self.suite = Some(v.into());
        self
    }
    /// Return only sites supporting these features. Allowed values:
    /// `firewall-control`, `device-control`, `ioc`.
    pub fn features<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.features = Some(join_csv(v));
        self
    }
    /// Sku.
    pub fn sku(mut self, v: impl Into<String>) -> Self {
        self.sku = Some(v.into());
        self
    }
    /// Module.
    pub fn module(mut self, v: impl Into<String>) -> Self {
        self.module = Some(v.into());
        self
    }
    /// Account id.
    pub fn account_id(mut self, v: impl Into<String>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    /// [DEPRECATED] Show sites the user has Admin privileges to.
    pub fn admin_only(mut self, v: bool) -> Self {
        self.admin_only = Some(v);
        self
    }
    /// Only return sites the user can move agents to.
    pub fn available_move_sites(mut self, v: bool) -> Self {
        self.available_move_sites = Some(v);
        self
    }
    /// Registration token.
    pub fn registration_token(mut self, v: impl Into<String>) -> Self {
        self.registration_token = Some(v.into());
        self
    }
    /// Free-text filter by account name (supports multiple values).
    pub fn account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by site name (supports multiple values).
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by site description (supports multiple values).
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join_csv(v));
        self
    }
}

/// Query params for `GET /web/api/v2.1/sites` — **Get Sites**.
///
/// Every field is optional. Array params are serialized as a single
/// comma-joined string, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSitesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,
    /// use "cursor". Example: "150".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more than
    /// 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `totalLicenses`, `expiration`, `siteType`, `state`, `suite`, `createdAt`.
    /// Example: "id".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Example: "asc".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Full text search for fields: name, account_name, description. (Note: on
    /// single-account consoles account name will not be matched).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Name. Example: "My Site".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Is default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Health status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<bool>,
    /// Site type. Allowed values: `Trial`, `Paid`. Example: "Trial".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_type: Option<String>,
    /// Expiration. Example: "2018-02-27T04:49:26.257525Z".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    /// Total licenses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_licenses: Option<i64>,
    /// Active licenses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_licenses: Option<i64>,
    /// Id in a CRM external system.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The description for the Site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Timestamp of site creation. Example: "2018-02-27T04:49:26.257525Z".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Timestamp of last update. Example: "2018-02-27T04:49:26.257525Z".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Site state. Allowed values: `active`, `expired`, `deleted`. Example: "active".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// List of states to filter (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<String>,
    /// List of states to not filter (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states_nin: Option<String>,
    /// [DEPRECATED] Use sku instead. Allowed values: `Core`, `Control`, `Complete`. Example: "Core".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite: Option<String>,
    /// If sent, return only sites that support these features. Allowed values:
    /// `firewall-control`, `device-control`, `ioc` (comma-joined).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<String>,
    /// Sku. Example: "core".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// Module. Example: "star,rso".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    /// Account id. Example: "225494730938493804".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// [DEPRECATED] Show sites the user has Admin privileges to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_only: Option<bool>,
    /// Only return sites the user can move agents to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_move_sites: Option<bool>,
    /// Registration token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_token: Option<String>,
    /// Free-text filter by account name (supports multiple values, comma-joined).
    #[serde(rename = "accountName__contains", skip_serializing_if = "Option::is_none")]
    pub account_name_contains: Option<String>,
    /// Free-text filter by site name (supports multiple values, comma-joined).
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by site description (supports multiple values, comma-joined).
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
}

impl ListSitesQuery {
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// If true, only the total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, the total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `totalLicenses`, `expiration`, `siteType`, `state`, `suite`, `createdAt`.
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
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
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
    /// Full text search for fields: name, account_name, description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// Is default.
    pub fn is_default(mut self, v: bool) -> Self {
        self.is_default = Some(v);
        self
    }
    /// Health status.
    pub fn health_status(mut self, v: bool) -> Self {
        self.health_status = Some(v);
        self
    }
    /// Site type. Allowed values: `Trial`, `Paid`.
    pub fn site_type(mut self, v: impl Into<String>) -> Self {
        self.site_type = Some(v.into());
        self
    }
    /// Expiration.
    pub fn expiration(mut self, v: impl Into<String>) -> Self {
        self.expiration = Some(v.into());
        self
    }
    /// Total licenses.
    pub fn total_licenses(mut self, v: i64) -> Self {
        self.total_licenses = Some(v);
        self
    }
    /// Active licenses.
    pub fn active_licenses(mut self, v: i64) -> Self {
        self.active_licenses = Some(v);
        self
    }
    /// Id in a CRM external system.
    pub fn external_id(mut self, v: impl Into<String>) -> Self {
        self.external_id = Some(v.into());
        self
    }
    /// The description for the Site.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// Timestamp of site creation.
    pub fn created_at(mut self, v: impl Into<String>) -> Self {
        self.created_at = Some(v.into());
        self
    }
    /// Timestamp of last update.
    pub fn updated_at(mut self, v: impl Into<String>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
    /// Site state. Allowed values: `active`, `expired`, `deleted`.
    pub fn state(mut self, v: impl Into<String>) -> Self {
        self.state = Some(v.into());
        self
    }
    /// List of states to filter.
    pub fn states<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states = Some(join_csv(v));
        self
    }
    /// List of states to not filter.
    pub fn states_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.states_nin = Some(join_csv(v));
        self
    }
    /// [DEPRECATED] Use sku instead. Allowed values: `Core`, `Control`, `Complete`.
    pub fn suite(mut self, v: impl Into<String>) -> Self {
        self.suite = Some(v.into());
        self
    }
    /// Return only sites supporting these features. Allowed values:
    /// `firewall-control`, `device-control`, `ioc`.
    pub fn features<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.features = Some(join_csv(v));
        self
    }
    /// Sku.
    pub fn sku(mut self, v: impl Into<String>) -> Self {
        self.sku = Some(v.into());
        self
    }
    /// Module.
    pub fn module(mut self, v: impl Into<String>) -> Self {
        self.module = Some(v.into());
        self
    }
    /// Account id.
    pub fn account_id(mut self, v: impl Into<String>) -> Self {
        self.account_id = Some(v.into());
        self
    }
    /// [DEPRECATED] Show sites the user has Admin privileges to.
    pub fn admin_only(mut self, v: bool) -> Self {
        self.admin_only = Some(v);
        self
    }
    /// Only return sites the user can move agents to.
    pub fn available_move_sites(mut self, v: bool) -> Self {
        self.available_move_sites = Some(v);
        self
    }
    /// Registration token.
    pub fn registration_token(mut self, v: impl Into<String>) -> Self {
        self.registration_token = Some(v.into());
        self
    }
    /// Free-text filter by account name (supports multiple values).
    pub fn account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by site name (supports multiple values).
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by site description (supports multiple values).
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join_csv(v));
        self
    }
}

/// Body for `POST /web/api/v2.1/site-with-admin` — **Create Site and User**.
///
/// Wraps the freeform `data` payload (schema `sites_SiteDataWithUserSchema`),
/// which holds the Site properties, nested `policy`, `licenses`, `irFields`,
/// `promptFields`, and a `user` block. Required.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSiteWithUserBody {
    /// Data — the Site, policy and admin user properties. Required.
    pub data: serde_json::Value,
}

/// Body for `POST /web/api/v2.1/sites` — **Create Site**.
///
/// Wraps the freeform `data` payload (schema `sites_PostSiteSchema`), which
/// holds the Site properties and nested `policy`/`licenses`/`irFields`/
/// `promptFields`. Required.
#[derive(Debug, Clone, Serialize)]
pub struct CreateSiteBody {
    /// Data — the Site and policy properties. Required.
    pub data: serde_json::Value,
}

/// Body for `POST /web/api/v2.1/sites/duplicate-site` — **Create duplicate
/// site** ([DEPRECATED]).
///
/// Wraps the freeform `data` payload (schema `sites_DuplicateSiteSchema`).
/// Required.
#[derive(Debug, Clone, Serialize)]
pub struct DuplicateSiteBody {
    /// Data — the duplicate-site properties (`sourceSiteId`, `name`, `policy`,
    /// `copyUsers`, `totalLicenses`, `unlimitedLicenses`, `policySource`).
    /// Required.
    pub data: serde_json::Value,
}

/// Body for `PUT /web/api/v2.1/sites/update-bulk` — **Update Sites**.
///
/// Wraps the freeform `data` payload (schema `sites_SiteBulkPutSchema`) which
/// holds the properties to apply and the target `siteIds`. Required.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSitesBody {
    /// Data — the bulk update properties. Required.
    pub data: serde_json::Value,
}

/// Body for `PUT /web/api/v2.1/sites/{site_id}` — **Update Site**.
///
/// Wraps the freeform `data` payload (schema `sites_SitePutSchema`), which holds
/// the Site properties and nested `policy`/`licenses`/`irFields`/`promptFields`.
/// Required.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSiteBody {
    /// Data — the Site and policy properties. Required.
    pub data: serde_json::Value,
}

/// Body for `PUT /web/api/v2.1/sites/{site_id}/local-authorization` — **Edit
/// local upgrade/downgrade Site authorization** (schema
/// `sites_PutSiteApprovalJsonSchema`).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditLocalAuthorizationBody {
    /// New expiration date for Site local upgrade/downgrade authorization.
    /// Example: "2018-02-27T04:49:26.257525Z". Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_authorization: Option<String>,
}

impl EditLocalAuthorizationBody {
    /// New expiration date for Site local upgrade/downgrade authorization.
    pub fn site_authorization(mut self, v: impl Into<String>) -> Self {
        self.site_authorization = Some(v.into());
        self
    }
}

/// Inner `data` of [`ReactivateSiteBody`] (schema `sites_ReactivateSiteSchema`).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactivateSiteData {
    /// If false an expiration should be supplied. Defaults to false server-side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited: Option<bool>,
    /// New expiration date for the site. Example: "2018-02-27T04:49:26.257525Z".
    /// Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
}

/// Body for `PUT /web/api/v2.1/sites/{site_id}/reactivate` — **Reactivate Site**
/// (schema `sites_ReactivateSiteSchema`).
#[derive(Debug, Clone, Serialize)]
pub struct ReactivateSiteBody {
    /// Data — the reactivation properties. Required.
    pub data: ReactivateSiteData,
}

/// Inner `data` of [`RevertPolicyBody`] (schema
/// `policies_schemas_RevertPolicySchema`).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevertPolicyData {
    /// Id. Example: "225494730938493804".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Body for `PUT /web/api/v2.1/sites/{site_id}/revert-policy` — **Revert Policy**
/// (schema `policies_schemas_RevertPolicySchema`).
#[derive(Debug, Clone, Default, Serialize)]
pub struct RevertPolicyBody {
    /// Data — the revert-policy properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<RevertPolicyData>,
}

/// Join an iterator of string-like values into a single comma-separated string,
/// matching the SentinelOne array-query / multi-value convention.
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

impl SitesService<'_> {
    /// **Export Sites** — Export Sites data to a CSV, for Sites that match the
    /// filter.
    ///
    /// The endpoint returns a CSV payload; this method deserializes the response
    /// as a generic JSON value (the spec does not define a typed schema).
    ///
    /// `GET /web/api/v2.1/export/sites`
    pub async fn export(
        &self,
        query: &ExportSitesQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/export/sites", q).await?)
    }

    /// **Create Site and User** — Create a Site and an Admin role user. This
    /// requires an Admin role with a Global scope or Account scope that has
    /// permissions over the Account to which the Site will belong. You must have
    /// a license for a new Site. In the body of this request, include the policy
    /// and user properties.
    ///
    /// `POST /web/api/v2.1/site-with-admin`
    pub async fn create_with_user(
        &self,
        body: &CreateSiteWithUserBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/site-with-admin", body)
            .await?)
    }

    /// **Get Sites** — Get the Sites that match the filters. The response
    /// includes the IDs of Sites, which you can use in other commands.
    ///
    /// Note: this endpoint nests its results under `data.sites` (alongside an
    /// `allSites` aggregate) rather than the standard paginated array envelope,
    /// so it returns [`Response<SitesListData>`].
    ///
    /// `GET /web/api/v2.1/sites`
    pub async fn list(
        &self,
        query: &ListSitesQuery,
    ) -> Result<Response<SitesListData>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/sites", q).await?)
    }

    /// **Create Site** — Create a Site. This requires an Admin role with a
    /// Global scope or Account scope that has permissions over the Account to
    /// which the Site will belong. You must have a license for a new Site. In
    /// the body of this request, include the policy.
    ///
    /// `POST /web/api/v2.1/sites`
    pub async fn create(
        &self,
        body: &CreateSiteBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/sites", body).await?)
    }

    /// **Create duplicate site** — [DEPRECATED] Create duplicate site.
    ///
    /// `POST /web/api/v2.1/sites/duplicate-site`
    pub async fn duplicate(
        &self,
        body: &DuplicateSiteBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/sites/duplicate-site", body)
            .await?)
    }

    /// **Update Sites** — Change the properties of the Sites given by IDs. To
    /// get the IDs, run 'sites'.
    ///
    /// `PUT /web/api/v2.1/sites/update-bulk`
    pub async fn update_bulk(
        &self,
        body: &UpdateSitesBody,
    ) -> Result<Response<SiteAffected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<UpdateSitesBody, Response<SiteAffected>>(
                Method::PUT,
                "/web/api/v2.1/sites/update-bulk",
                None,
                Some(body),
            )
            .await?)
    }

    /// **Delete Site** — Delete the Site of the given ID. To get the ID, run
    /// "sites". You must have an Admin role with scope access that includes the
    /// Site.
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `DELETE /web/api/v2.1/sites/{site_id}`
    pub async fn delete(
        &self,
        site_id: impl Into<String>,
    ) -> Result<Response<SiteSuccess>, Error> {
        let path = format!("/web/api/v2.1/sites/{}", site_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<SiteSuccess>>(Method::DELETE, &path, None, None)
            .await?)
    }

    /// **Get Site by ID** — Get the data of the Site of the ID. To get the ID,
    /// run "sites". The response shows the Site expiration date, SKU, licenses
    /// (total and active), token, Account name and ID, who and when it was
    /// created and changed, and its status.
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `GET /web/api/v2.1/sites/{site_id}`
    pub async fn get(
        &self,
        site_id: impl Into<String>,
    ) -> Result<Response<crate::models::sites::Site>, Error> {
        let path = format!("/web/api/v2.1/sites/{}", site_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// **Update Site** — Change the policy and properties of the Site given by
    /// ID. To get the ID, run 'sites'.
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `PUT /web/api/v2.1/sites/{site_id}`
    pub async fn update(
        &self,
        site_id: impl Into<String>,
        body: &UpdateSiteBody,
    ) -> Result<Response<crate::models::sites::Site>, Error> {
        let path = format!("/web/api/v2.1/sites/{}", site_id.into());
        Ok(self
            .client
            .http()
            .request_json::<UpdateSiteBody, Response<crate::models::sites::Site>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// **Expire Site** — Expire the Site of the given ID (run "sites" to get the
    /// ID). You must have an Admin role with scope access that includes this
    /// Site.
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `POST /web/api/v2.1/sites/{site_id}/expire-now`
    pub async fn expire_now(
        &self,
        site_id: impl Into<String>,
    ) -> Result<Response<crate::models::sites::Site>, Error> {
        let path = format!("/web/api/v2.1/sites/{}/expire-now", site_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<crate::models::sites::Site>>(
                Method::POST,
                &path,
                None,
                None,
            )
            .await?)
    }

    /// **Get local upgrade/downgrade Site authorization** — Get the time when
    /// authorization of local upgrades/downgrades expires, and the number of
    /// Agents authorized for local upgrades/downgrades, in this Site.
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `GET /web/api/v2.1/sites/{site_id}/local-authorization`
    pub async fn get_local_authorization(
        &self,
        site_id: impl Into<String>,
    ) -> Result<Response<SiteApproval>, Error> {
        let path = format!(
            "/web/api/v2.1/sites/{}/local-authorization",
            site_id.into()
        );
        Ok(self.client.http().get(&path, None).await?)
    }

    /// **Edit local upgrade/downgrade Site authorization** — Edit when
    /// authorization of local upgrades/downgrades expires. Returns the number of
    /// Agents authorized for local upgrades/downgrades, in this Site.
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `PUT /web/api/v2.1/sites/{site_id}/local-authorization`
    pub async fn edit_local_authorization(
        &self,
        site_id: impl Into<String>,
        body: &EditLocalAuthorizationBody,
    ) -> Result<Response<SitePutApproval>, Error> {
        let path = format!(
            "/web/api/v2.1/sites/{}/local-authorization",
            site_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<EditLocalAuthorizationBody, Response<SitePutApproval>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// **Get a CSV file of local upgrade/downgrade Site authorization data** —
    /// Get a CSV file containing the Agents authorized for local
    /// upgrades/downgrades, in this Site.
    ///
    /// The endpoint returns a CSV payload; this method deserializes the response
    /// as a generic JSON value.
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `GET /web/api/v2.1/sites/{site_id}/local-upgrade-approved-agents-csv`
    pub async fn local_upgrade_approved_agents_csv(
        &self,
        site_id: impl Into<String>,
    ) -> Result<serde_json::Value, Error> {
        let path = format!(
            "/web/api/v2.1/sites/{}/local-upgrade-approved-agents-csv",
            site_id.into()
        );
        Ok(self.client.http().get(&path, None).await?)
    }

    /// **Reactivate Site** — Reactivate an expired Site. You must have an Admin
    /// role with scope access that includes this Site, and you must have a
    /// license for the Site. To get the site_id, run "sites".
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `PUT /web/api/v2.1/sites/{site_id}/reactivate`
    pub async fn reactivate(
        &self,
        site_id: impl Into<String>,
        body: &ReactivateSiteBody,
    ) -> Result<Response<SiteSuccess>, Error> {
        let path = format!("/web/api/v2.1/sites/{}/reactivate", site_id.into());
        Ok(self
            .client
            .http()
            .request_json::<ReactivateSiteBody, Response<SiteSuccess>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// **Regenerate Site Key** — Regenerate the key for the given Site. To get
    /// the site_id, use "sites".
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `PUT /web/api/v2.1/sites/{site_id}/regenerate-key`
    pub async fn regenerate_key(
        &self,
        site_id: impl Into<String>,
    ) -> Result<Response<SiteRegenerateKey>, Error> {
        let path = format!("/web/api/v2.1/sites/{}/regenerate-key", site_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<SiteRegenerateKey>>(Method::PUT, &path, None, None)
            .await?)
    }

    /// **Revert Policy** — When a Site is created through the Console, it gets
    /// the Global policy. If you change the policy and later want it set to the
    /// Global policy, use this command. The site_id is required. You can get it
    /// from "sites".
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `PUT /web/api/v2.1/sites/{site_id}/revert-policy`
    pub async fn revert_policy(
        &self,
        site_id: impl Into<String>,
        body: &RevertPolicyBody,
    ) -> Result<Response<SiteSuccess>, Error> {
        let path = format!("/web/api/v2.1/sites/{}/revert-policy", site_id.into());
        Ok(self
            .client
            .http()
            .request_json::<RevertPolicyBody, Response<SiteSuccess>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// **Get Site registration token by ID** — Get the registration token of the
    /// Site of the ID.
    ///
    /// `site_id`: Site ID. Example: "225494730938493804".
    ///
    /// `GET /web/api/v2.1/sites/{site_id}/token`
    pub async fn token(
        &self,
        site_id: impl Into<String>,
    ) -> Result<Response<SiteToken>, Error> {
        let path = format!("/web/api/v2.1/sites/{}/token", site_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }
}
