//! `Network Discovery Self Enablement` tag.
//!
//! [DEPRECATED] Network Discovery self enablement views and operations.
//!
//! The SentinelOne spec does not define `200` response schemas for any of these
//! endpoints (only `400`/`401`/`404` are documented), so every method returns a
//! `serde_json::Value` payload. The `GET /ranger/enablement` endpoint exposes the
//! standard cursor pagination params, so it is modelled as a list endpoint and
//! returns [`Paginated`]; the remaining endpoints return [`Response`].

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::pagination::{Paginated, Response};

/// `Network Discovery Self Enablement` tag.
///
/// [DEPRECATED] Network Discovery self enablement views and operations.
pub struct NetworkDiscoverySelfEnablementService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/ranger/enablement` (Get Self Enablement).
///
/// Every field is optional. Array params are serialized comma-joined, as the API
/// expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSelfEnablementQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,
    /// use `cursor`. Example: `"150"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `"10"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more than
    /// 1000 items. Example: `"YWdlbnRfaWQ6NTgwMjkzODE="`. Optional.
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
    /// The column to sort the results by. Example: `"id"`. Optional.
    ///
    /// Allowed values: `siteName`, `activeAgents`, `rangerProEnabled`,
    /// `roguesEnabled`, `rangerEnabled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: `"asc"`. Optional.
    ///
    /// Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by. Example:
    /// `"225494730938493804,225494730938493915"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Example:
    /// `"225494730938493804,225494730938493915"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Example:
    /// `"225494730938493804,225494730938493915"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// The enablement id. Example: `"225494730938493804"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of non-decommissioned agents in the site. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_agents: Option<i64>,
    /// [DEPRECATED]. Use `rangerEnabled` instead. Network Discovery Pro Enabled
    /// true/false. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_pro_enabled: Option<bool>,
    /// Network Discovery Enabled true/false. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_enabled: Option<bool>,
    /// Unprotected Endpoints Discovery Enabled true/false. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rogues_enabled: Option<bool>,
    /// Agent count (less than). Optional.
    #[serde(rename = "activeAgents__lt", skip_serializing_if = "Option::is_none")]
    pub active_agents_lt: Option<i64>,
    /// Agent count (less than or equal). Optional.
    #[serde(rename = "activeAgents__lte", skip_serializing_if = "Option::is_none")]
    pub active_agents_lte: Option<i64>,
    /// Agent count (more than). Optional.
    #[serde(rename = "activeAgents__gt", skip_serializing_if = "Option::is_none")]
    pub active_agents_gt: Option<i64>,
    /// Agent count (more than or equal). Optional.
    #[serde(rename = "activeAgents__gte", skip_serializing_if = "Option::is_none")]
    pub active_agents_gte: Option<i64>,
    /// Agent count (between). Example: `"2-8"`. Optional.
    #[serde(
        rename = "activeAgents__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_agents_between: Option<String>,
    /// The site name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// A list of ids to get. Example:
    /// `"225494730938493804,225494730938493915"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Free-text filter by site name (supports multiple values). Optional.
    #[serde(rename = "siteName__contains", skip_serializing_if = "Option::is_none")]
    pub site_name_contains: Option<String>,
}

impl GetSelfEnablementQuery {
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
    /// The column to sort the results by.
    ///
    /// Allowed values: `siteName`, `activeAgents`, `rangerProEnabled`,
    /// `roguesEnabled`, `rangerEnabled`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Account IDs to filter by (comma-joined).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Site IDs to filter by (comma-joined).
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by (comma-joined).
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// The enablement id.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    /// The number of non-decommissioned agents in the site.
    pub fn active_agents(mut self, n: i64) -> Self {
        self.active_agents = Some(n);
        self
    }
    /// [DEPRECATED]. Use `ranger_enabled` instead.
    pub fn ranger_pro_enabled(mut self, v: bool) -> Self {
        self.ranger_pro_enabled = Some(v);
        self
    }
    /// Network Discovery Enabled true/false.
    pub fn ranger_enabled(mut self, v: bool) -> Self {
        self.ranger_enabled = Some(v);
        self
    }
    /// Unprotected Endpoints Discovery Enabled true/false.
    pub fn rogues_enabled(mut self, v: bool) -> Self {
        self.rogues_enabled = Some(v);
        self
    }
    /// Agent count (less than).
    pub fn active_agents_lt(mut self, n: i64) -> Self {
        self.active_agents_lt = Some(n);
        self
    }
    /// Agent count (less than or equal).
    pub fn active_agents_lte(mut self, n: i64) -> Self {
        self.active_agents_lte = Some(n);
        self
    }
    /// Agent count (more than).
    pub fn active_agents_gt(mut self, n: i64) -> Self {
        self.active_agents_gt = Some(n);
        self
    }
    /// Agent count (more than or equal).
    pub fn active_agents_gte(mut self, n: i64) -> Self {
        self.active_agents_gte = Some(n);
        self
    }
    /// Agent count (between). Example: `"2-8"`.
    pub fn active_agents_between(mut self, v: impl Into<String>) -> Self {
        self.active_agents_between = Some(v.into());
        self
    }
    /// The site name.
    pub fn site_name(mut self, v: impl Into<String>) -> Self {
        self.site_name = Some(v.into());
        self
    }
    /// A list of ids to get (comma-joined).
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// Free-text filter by site name (comma-joined; supports multiple values).
    pub fn site_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_name_contains = Some(join_csv(values));
        self
    }
}

/// Query params for `GET /web/api/v2.1/ranger/enablement/defaults`
/// (Features Configuration for New Sites).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEnablementDefaultsQuery {
    /// List of Account IDs to filter by. Example:
    /// `"225494730938493804,225494730938493915"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl GetEnablementDefaultsQuery {
    /// List of Account IDs to filter by (comma-joined).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
}

// ---------------------------------------------------------------------------
// Body types
// ---------------------------------------------------------------------------

/// Body for `POST /web/api/v2.1/ranger/enable-self-management`
/// (Change the Self-Enablement for Accounts).
///
/// Maps to the spec `ranger.enablement.schemas_UpdateEnablementPostSchema`.
/// Both `data` and `filter` are required by the schema.
#[derive(Debug, Clone, Serialize)]
pub struct EnableSelfManagementBody {
    /// Data. Required.
    pub data: EnableSelfManagementBodyData,
    /// Filter. Required.
    pub filter: EnableSelfManagementBodyFilter,
}

/// `data` object for [`EnableSelfManagementBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableSelfManagementBodyData {
    /// enable: true/false. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

/// `filter` object for [`EnableSelfManagementBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableSelfManagementBodyFilter {
    /// List of Account IDs to filter by. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

/// Body shared by `POST /web/api/v2.1/ranger/enablement`
/// (Change Network Discovery or Unprotected Endpoints Discovery Features) and
/// `POST /web/api/v2.1/ranger/enablement/defaults`
/// (Change Feature Defaults for New Sites).
///
/// Maps to the spec
/// `ranger.enablement.schemas_UpdateSelfEnablementFeaturesSchema`.
/// Both `data` and `filter` are required by the schema.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateSelfEnablementFeaturesBody {
    /// Data. Required.
    pub data: UpdateSelfEnablementFeaturesBodyData,
    /// Filter. Required.
    pub filter: UpdateSelfEnablementFeaturesBodyFilter,
}

/// `data` object for [`UpdateSelfEnablementFeaturesBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSelfEnablementFeaturesBodyData {
    /// [DEPRECATED] Use `rangerEnabled` parameter instead. Network Discovery Pro
    /// Enabled true/false. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_pro_enabled: Option<bool>,
    /// Network Discovery Enabled true/false. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_enabled: Option<bool>,
    /// Unprotected Endpoints Discovery Enabled true/false. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rogues_enabled: Option<bool>,
}

/// `filter` object for [`UpdateSelfEnablementFeaturesBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSelfEnablementFeaturesBodyFilter {
    /// List of Account IDs to filter by (1-500 items). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (1-500 items). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by (1-500 items). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// The enablement id. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The number of non-decommissioned agents in the site. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_agents: Option<i64>,
    /// [DEPRECATED]. Use `rangerEnabled` instead. Network Discovery Pro Enabled
    /// true/false. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_pro_enabled: Option<bool>,
    /// Network Discovery Enabled true/false. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_enabled: Option<bool>,
    /// Unprotected Endpoints Discovery Enabled true/false. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rogues_enabled: Option<bool>,
    /// Agent count (less than). Optional/nullable.
    #[serde(rename = "activeAgents__lt", skip_serializing_if = "Option::is_none")]
    pub active_agents_lt: Option<i64>,
    /// Agent count (less than or equal). Optional/nullable.
    #[serde(rename = "activeAgents__lte", skip_serializing_if = "Option::is_none")]
    pub active_agents_lte: Option<i64>,
    /// Agent count (more than). Optional/nullable.
    #[serde(rename = "activeAgents__gt", skip_serializing_if = "Option::is_none")]
    pub active_agents_gt: Option<i64>,
    /// Agent count (more than or equal). Optional/nullable.
    #[serde(rename = "activeAgents__gte", skip_serializing_if = "Option::is_none")]
    pub active_agents_gte: Option<i64>,
    /// Agent count (between). Example: `"2-8"`. Optional/nullable.
    #[serde(
        rename = "activeAgents__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_agents_between: Option<String>,
    /// The site name. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// A list of ids to get (up to 5000). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Free-text filter by site name (supports multiple values, up to 20).
    /// Optional/nullable.
    #[serde(rename = "siteName__contains", skip_serializing_if = "Option::is_none")]
    pub site_name_contains: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

impl NetworkDiscoverySelfEnablementService<'_> {
    /// `POST /web/api/v2.1/ranger/enable-self-management` — Change the
    /// Self-Enablement for Accounts.
    ///
    /// [DEPRECATED] Use the Update Account, Get Account, Get Sites, or the Update
    /// Site Add-ons APIs instead.
    pub async fn enable_self_management(
        &self,
        body: &EnableSelfManagementBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/ranger/enable-self-management", body)
            .await?)
    }

    /// `GET /web/api/v2.1/ranger/enablement` — Get Self Enablement.
    ///
    /// [DEPRECATED] Use the Update Account, Get Account, Get Sites, or the Update
    /// Site Add-ons APIs instead.
    pub async fn get_self_enablement(
        &self,
        query: &GetSelfEnablementQuery,
    ) -> Result<Paginated<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/ranger/enablement", q)
            .await?)
    }

    /// `POST /web/api/v2.1/ranger/enablement` — Change Network Discovery or
    /// Unprotected Endpoints Discovery Features.
    ///
    /// [DEPRECATED] Use the Update Account, Get Account, Get Sites, or the Update
    /// Site Add-ons APIs instead.
    pub async fn change_enablement_features(
        &self,
        body: &UpdateSelfEnablementFeaturesBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/ranger/enablement", body)
            .await?)
    }

    /// `GET /web/api/v2.1/ranger/enablement/defaults` — Features Configuration for
    /// New Sites.
    ///
    /// [DEPRECATED] Use the Update Account, Get Account, Get Sites, or the Update
    /// Site Add-ons APIs instead.
    pub async fn get_enablement_defaults(
        &self,
        query: &GetEnablementDefaultsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/ranger/enablement/defaults", q)
            .await?)
    }

    /// `POST /web/api/v2.1/ranger/enablement/defaults` — Change Feature Defaults
    /// for New Sites.
    ///
    /// [DEPRECATED] Use the Update Account, Get Account, Get Sites, or the Update
    /// Site Add-ons APIs instead.
    pub async fn change_enablement_defaults(
        &self,
        body: &UpdateSelfEnablementFeaturesBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/ranger/enablement/defaults", body)
            .await?)
    }
}

/// Join an iterator of string-like values into a comma-separated string, as the
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
