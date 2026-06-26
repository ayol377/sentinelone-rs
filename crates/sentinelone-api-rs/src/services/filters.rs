//! Service for the `Filters` tag (saved network filters).

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::filters::{
    CsvFilter, DeepVisibilityFilter, Filter, FilterSuccess, XdrFilter, XdrFilterEnriched,
};
use crate::pagination::{Paginated, Response};

/// `Filters` tag — saved network filters.
pub struct FiltersService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query types
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/filters`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Clone, Serialize)]
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
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `createdAt`, `updatedAt`. Optional.
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
    /// Text query for filter's name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// [DEPRECATED] Return global filters even when specific sites are selected.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global: Option<bool>,
    /// Return filters from parent scope levels (Default: false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels (Default: false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// A list of Filter IDs (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
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
    /// `createdAt`, `updatedAt`.
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
    /// Text query for filter's name.
    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query = Some(q.into());
        self
    }
    /// [DEPRECATED] Return global filters even when specific sites are selected.
    pub fn include_global(mut self, v: bool) -> Self {
        self.include_global = Some(v);
        self
    }
    /// Return filters from parent scope levels (Default: false).
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Return filters from children scope levels (Default: false).
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
    /// A list of Filter IDs.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/filters/dv`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDeepVisibilityQuery {
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
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `createdAt`, `updatedAt`. Optional.
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
    /// Text query for filter's name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// [DEPRECATED] Return global filters even when specific sites are selected.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global: Option<bool>,
    /// Return filters from parent scope levels (Default: false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return filters from children scope levels (Default: false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// A list of Filter IDs (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
}

impl ListDeepVisibilityQuery {
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
    /// `createdAt`, `updatedAt`.
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
    /// Text query for filter's name.
    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query = Some(q.into());
        self
    }
    /// [DEPRECATED] Return global filters even when specific sites are selected.
    pub fn include_global(mut self, v: bool) -> Self {
        self.include_global = Some(v);
        self
    }
    /// Return filters from parent scope levels (Default: false).
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Return filters from children scope levels (Default: false).
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
    /// A list of Filter IDs.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
}

/// Query params for the XDR filter list endpoints
/// (`GET /web/api/v2.1/xdr/filters` and
/// `GET /web/api/v2.1/xdr/private/filters/enriched`).
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListXdrQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Text query for filter's name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Return filters from children scope levels (Default: false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// A list of Filter IDs (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Return filters from parent scope levels (Default: false). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// List of Site IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `createdAt`, `updatedAt`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListXdrQuery {
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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
    /// Text query for filter's name.
    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query = Some(q.into());
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Return filters from children scope levels (Default: false).
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
    /// A list of Filter IDs.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// Return filters from parent scope levels (Default: false).
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// If true, only the total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
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
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// If true, the total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `createdAt`, `updatedAt`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
}

// ---------------------------------------------------------------------------
// Body types
// ---------------------------------------------------------------------------

/// Request body for `POST /web/api/v2.1/filters`
/// (`filters.filters_NewFilterSchema`).
///
/// The freeform body is wrapped as `{ "data": { ... }, "filter": { ... } }`;
/// because the schema is deeply nested/freeform, use [`serde_json::Value`].
#[derive(Debug, Clone, Serialize)]
pub struct SaveFilterBody {
    /// Filter data: contains `filterFields` (required), `name` (required),
    /// `scopeLevel` (`site`, `account`, `global`) and `siteId`. Required.
    pub data: serde_json::Value,
    /// Filter scope (optional, freeform object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl SaveFilterBody {
    /// Build a save-filter body from the (required) `data` object.
    pub fn new(data: serde_json::Value) -> Self {
        Self { data, filter: None }
    }
    /// Set the (optional) `filter` scope object.
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Request body for `PUT /web/api/v2.1/filters/{filter_id}`
/// (`filters.filters_UpdateFilterSchema`).
///
/// The body is wrapped as `{ "data": { "name": ..., "filterFields": ... } }`;
/// because the schema is freeform, use [`serde_json::Value`].
#[derive(Debug, Clone, Serialize)]
pub struct UpdateFilterBody {
    /// Filter data: may contain `name` and `filterFields`. Required.
    pub data: serde_json::Value,
}

impl UpdateFilterBody {
    /// Build an update-filter body from the (required) `data` object.
    pub fn new(data: serde_json::Value) -> Self {
        Self { data }
    }
}

/// Request body for `POST /web/api/v2.1/filters/dv` and
/// `PUT /web/api/v2.1/filters/dv/{filter_id}`
/// (`filters.filters_NewDeepVisibilityFilterSchema`).
///
/// The body is wrapped as `{ "data": { ... }, "filter": { ... } }`; because the
/// schema is freeform, use [`serde_json::Value`].
#[derive(Debug, Clone, Serialize)]
pub struct DeepVisibilityFilterBody {
    /// Filter data: contains `filterFields` (required), `name` (required),
    /// `recipients`, `frequency` and `notifications`. Required.
    pub data: serde_json::Value,
    /// Filter scope (`siteIds`, `accountIds`, `groupIds`, `scope_level`).
    /// Required.
    pub filter: serde_json::Value,
}

impl DeepVisibilityFilterBody {
    /// Build a Deep Visibility filter body from the (required) `data` and
    /// `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for `POST /web/api/v2.1/xdr/filters`
/// (`v2_1.config.schemas_NewFilterSchema`).
///
/// The body is wrapped as `{ "data": { ... }, "filter": { ... } }`; because the
/// schema is deeply nested/freeform, use [`serde_json::Value`].
#[derive(Debug, Clone, Serialize)]
pub struct SaveXdrFilterBody {
    /// Filter data: contains `filterFields` (required), `name` (required),
    /// `filter`, `scopeId`, `scopeLevel` and `visibility` (`public`,
    /// `private`). Required.
    pub data: serde_json::Value,
    /// Filter scope (optional, freeform object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl SaveXdrFilterBody {
    /// Build a save XDR-filter body from the (required) `data` object.
    pub fn new(data: serde_json::Value) -> Self {
        Self { data, filter: None }
    }
    /// Set the (optional) `filter` scope object.
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Request body for `PUT /web/api/v2.1/xdr/filters/{filter_id}`
/// (`v2_1.config.schemas_UpdateFilterSchema`).
///
/// The body is wrapped as `{ "data": { "name": ..., "filterFields": ... } }`;
/// because the schema is freeform, use [`serde_json::Value`].
#[derive(Debug, Clone, Serialize)]
pub struct UpdateXdrFilterBody {
    /// Filter data: may contain `name` and `filterFields`. Required.
    pub data: serde_json::Value,
    /// Filter scope (optional, freeform object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl UpdateXdrFilterBody {
    /// Build an update XDR-filter body from the (required) `data` object.
    pub fn new(data: serde_json::Value) -> Self {
        Self { data, filter: None }
    }
    /// Set the (optional) `filter` scope object.
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Form body for `POST /web/api/v2.1/filters/csv-filter`.
///
/// This endpoint expects a `multipart/form-data` upload with three fields. The
/// raw file bytes are carried freeform; the actual multipart encoding is the
/// caller's responsibility.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadCsvFilterBody {
    /// The property of the endpoint to filter by. Required.
    pub agent_filter_field: String,
    /// Set to True to exclude the column header. Required.
    pub exclude_header: bool,
    /// File (freeform). Required.
    pub file: serde_json::Value,
}

impl UploadCsvFilterBody {
    /// Build a CSV upload body from the three required fields.
    pub fn new(
        agent_filter_field: impl Into<String>,
        exclude_header: bool,
        file: serde_json::Value,
    ) -> Self {
        Self {
            agent_filter_field: agent_filter_field.into(),
            exclude_header,
            file,
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn join_csv<I, S>(items: I) -> String
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

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl FiltersService<'_> {
    /// `GET /web/api/v2.1/filters` — Get Filters.
    ///
    /// Get the list of saved filters. See Save Filter. The response includes
    /// the ID of the filter, which you can use in other commands.
    pub async fn list(&self, query: &ListQuery) -> Result<Paginated<Filter>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/filters", q).await?)
    }

    /// `POST /web/api/v2.1/filters` — Save Filter.
    ///
    /// Save a new filter to get a list of matching endpoints. When you save a
    /// filter, you can run actions on the Agents as a set of objects or create a
    /// dynamic group (automatically adds new Agents that match the filter and
    /// drops Agents if they change to not match). For example, you can save a
    /// filter with `{"data":{"filterFields":{"infected":true}}}` to run kill and
    /// quarantine commands on all the Agents at once, or to create a group that
    /// holds currently infected endpoints. Best Practice: Set a scope for the
    /// new Saved Filter. Run "accounts", "sites", or "groups" to get the IDs for
    /// the scope.
    pub async fn save(&self, body: &SaveFilterBody) -> Result<Response<Filter>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/filters", body).await?)
    }

    /// `POST /web/api/v2.1/filters/csv-filter` — Upload CSV file.
    ///
    /// Upload CSV file.
    ///
    /// Note: this endpoint expects a `multipart/form-data` upload; the JSON body
    /// here carries the field metadata and freeform file bytes.
    pub async fn upload_csv_filter(
        &self,
        body: &UploadCsvFilterBody,
    ) -> Result<Response<CsvFilter>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/filters/csv-filter", body)
            .await?)
    }

    /// `GET /web/api/v2.1/filters/dv` — [DEPRECATED] Get Deep Visibility Filters.
    ///
    /// Get saved Deep Visibility queries with full data. See Save Deep
    /// Visibility Filters. The response includes the ID of the filter, which you
    /// can use in other commands.
    ///
    /// The spec defines no typed `200` schema for this endpoint, so the rows are
    /// returned as freeform JSON.
    pub async fn list_deep_visibility(
        &self,
        query: &ListDeepVisibilityQuery,
    ) -> Result<Paginated<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/filters/dv", q).await?)
    }

    /// `POST /web/api/v2.1/filters/dv` — [DEPRECATED] Save Deep Visibility Filter.
    ///
    /// Save a Deep Visibility query with data as a filter, to get notifications
    /// of specific events sent to named recipients on a given frequency. The
    /// recipients must be Console users with permissions on the scope of the
    /// query. Notifications are sent through email: you must have an SMTP server
    /// configured in the SentinelOne solution (/settings/smtp see Set SMTP
    /// Settings). Deep Visibility requires a Complete SKU.
    ///
    /// The spec defines no typed `200` schema for this endpoint, so the response
    /// is returned as freeform JSON.
    pub async fn save_deep_visibility(
        &self,
        body: &DeepVisibilityFilterBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/filters/dv", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/filters/dv/{filter_id}` — [DEPRECATED] Delete Deep
    /// Visibility Filter.
    ///
    /// Delete a saved Deep Visibility query.
    ///
    /// `filter_id`: Filter ID. Example: "225494730938493804".
    pub async fn delete_deep_visibility(
        &self,
        filter_id: impl Into<String>,
    ) -> Result<Response<FilterSuccess>, Error> {
        let path = format!("/web/api/v2.1/filters/dv/{}", filter_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<FilterSuccess>>(Method::DELETE, &path, None, None)
            .await?)
    }

    /// `PUT /web/api/v2.1/filters/dv/{filter_id}` — [DEPRECATED] Update Deep
    /// Visibility Filter.
    ///
    /// Change a saved Deep Visibility filter. To get the ID and fields to
    /// change, run Get Deep Visibility Filters.
    ///
    /// `filter_id`: Filter ID. Example: "225494730938493804".
    pub async fn update_deep_visibility(
        &self,
        filter_id: impl Into<String>,
        body: &DeepVisibilityFilterBody,
    ) -> Result<Response<DeepVisibilityFilter>, Error> {
        let path = format!("/web/api/v2.1/filters/dv/{}", filter_id.into());
        Ok(self
            .client
            .http()
            .request_json::<DeepVisibilityFilterBody, Response<DeepVisibilityFilter>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/filters/{filter_id}` — Delete Filter.
    ///
    /// Delete a saved filter.
    ///
    /// `filter_id`: Filter ID. Example: "225494730938493804".
    pub async fn delete(
        &self,
        filter_id: impl Into<String>,
    ) -> Result<Response<FilterSuccess>, Error> {
        let path = format!("/web/api/v2.1/filters/{}", filter_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<FilterSuccess>>(Method::DELETE, &path, None, None)
            .await?)
    }

    /// `PUT /web/api/v2.1/filters/{filter_id}` — Update Filter.
    ///
    /// Update an existing filter.
    ///
    /// `filter_id`: Filter ID. Example: "225494730938493804".
    pub async fn update(
        &self,
        filter_id: impl Into<String>,
        body: &UpdateFilterBody,
    ) -> Result<Response<Filter>, Error> {
        let path = format!("/web/api/v2.1/filters/{}", filter_id.into());
        Ok(self
            .client
            .http()
            .request_json::<UpdateFilterBody, Response<Filter>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/filters` — Get Filters.
    ///
    /// Get the list of saved filters. See Save Filter. The response includes the
    /// ID of the filter, which you can use in other commands.
    pub async fn list_xdr(
        &self,
        query: &ListXdrQuery,
    ) -> Result<Paginated<XdrFilterEnriched>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/xdr/filters", q).await?)
    }

    /// `POST /web/api/v2.1/xdr/filters` — Save Filter.
    ///
    /// Save a new filter to get a list of matching endpoints. When you save a
    /// filter, you can run actions on the Agents as a set of objects or create a
    /// dynamic group (automatically adds new Agents that match the filter and
    /// drops Agents if they change to not match). For example, you can save a
    /// filter with `{"data":{"filterFields":{"infected":true}}}` to run kill and
    /// quarantine commands on all the Agents at once, or to create a group that
    /// holds currently infected endpoints. Best Practice: Set a scope for the
    /// new Saved Filter. Run "accounts", "sites", or "groups" to get the IDs for
    /// the scope.
    pub async fn save_xdr(&self, body: &SaveXdrFilterBody) -> Result<Response<XdrFilter>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/xdr/filters", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/xdr/filters/{filter_id}` — Delete Filter.
    ///
    /// Delete a saved filter.
    ///
    /// `filter_id`: Filter ID.
    pub async fn delete_xdr(
        &self,
        filter_id: impl Into<String>,
    ) -> Result<Response<FilterSuccess>, Error> {
        let path = format!("/web/api/v2.1/xdr/filters/{}", filter_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<FilterSuccess>>(Method::DELETE, &path, None, None)
            .await?)
    }

    /// `PUT /web/api/v2.1/xdr/filters/{filter_id}` — Update Filter.
    ///
    /// Update an existing filter.
    ///
    /// `filter_id`: Filter ID.
    pub async fn update_xdr(
        &self,
        filter_id: impl Into<String>,
        body: &UpdateXdrFilterBody,
    ) -> Result<Response<XdrFilter>, Error> {
        let path = format!("/web/api/v2.1/xdr/filters/{}", filter_id.into());
        Ok(self
            .client
            .http()
            .request_json::<UpdateXdrFilterBody, Response<XdrFilter>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/private/filters/enriched` — Filters with Metadata.
    ///
    /// Get a list of saved endpoint filters, with enriched data. One of the
    /// fields in the response is the ID of each filter, which you will need in
    /// other commands.
    pub async fn list_xdr_enriched(
        &self,
        query: &ListXdrQuery,
    ) -> Result<Paginated<XdrFilterEnriched>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/private/filters/enriched", q)
            .await?)
    }
}
