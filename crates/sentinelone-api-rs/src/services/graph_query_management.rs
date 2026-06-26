use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::graph_query_management::{
    QueryManagementSaveResponse, QueryManagementUpdateResponse, QueryManagerResponse,
    QueryTypeCountsResponse, RecentQueriesResponse,
};
use crate::pagination::{Paginated, Response};

/// `Graph Query Management` tag.
///
/// Manage saved, shared and suggested graph-explorer queries.
pub struct GraphQueryManagementService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/xdr/graph-explorer/query/management/query`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQueriesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// List of Group IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Query type. Allowed values: `saved`, `shared`, `suggested`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<String>,
    /// Free-text search filter by query name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
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
    /// The column to sort the results by. Allowed values: `name`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListQueriesQuery {
    /// Skip first number of items (0-1000). Optional.
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// List of Group IDs to filter by. Comma-joined. Optional.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Account IDs to filter by. Comma-joined. Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Limit number of returned items (1-1000). Optional.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Version. Optional.
    pub fn version(mut self, v: impl Into<String>) -> Self {
        self.version = Some(v.into());
        self
    }
    /// If true, only the total number of items will be returned. Optional.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Query type. Allowed values: `saved`, `shared`, `suggested`. Optional.
    pub fn query_type(mut self, v: impl Into<String>) -> Self {
        self.query_type = Some(v.into());
        self
    }
    /// Free-text search filter by query name. Optional.
    pub fn name_contains(mut self, v: impl Into<String>) -> Self {
        self.name_contains = Some(v.into());
        self
    }
    /// List of Site IDs to filter by. Comma-joined. Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Cursor position returned by the last request. Optional.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// If true, the total number of items will not be calculated. Optional.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `name`. Optional.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/graph-explorer/query/management/query`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveQueryQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl SaveQueryQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Site IDs to filter by. Comma-joined. Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Group IDs to filter by. Comma-joined. Optional.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query params for `GET /web/api/v2.1/xdr/graph-explorer/query/management/query/type-counts`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeCountsQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl TypeCountsQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Site IDs to filter by. Comma-joined. Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Group IDs to filter by. Comma-joined. Optional.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query params for `DELETE /web/api/v2.1/xdr/graph-explorer/query/management/query/{query_id}`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteQueryQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl DeleteQueryQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Site IDs to filter by. Comma-joined. Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Group IDs to filter by. Comma-joined. Optional.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query params for `PUT /web/api/v2.1/xdr/graph-explorer/query/management/query/{query_id}`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQueryQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl UpdateQueryQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Site IDs to filter by. Comma-joined. Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Group IDs to filter by. Comma-joined. Optional.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query params for `GET /web/api/v2.1/xdr/graph-explorer/query/management/recent-queries`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentQueriesQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Limit number of returned items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// List of Group IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl RecentQueriesQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Site IDs to filter by. Comma-joined. Optional.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Limit number of returned items. Optional.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// List of Group IDs to filter by. Comma-joined. Optional.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Request body for saving/updating a graph query
/// (`v2_1.graph.query.schemas_QueryManagementUpdateSchema`).
///
/// All four fields are required by the spec.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryManagementBody {
    /// Name. Required.
    pub name: String,
    /// Query. Required.
    pub query: String,
    /// Query description. Required.
    pub query_description: String,
    /// Shared. Required.
    pub shared: bool,
}

impl GraphQueryManagementService<'_> {
    /// `GET /web/api/v2.1/xdr/graph-explorer/query/management/query` — Get graph
    /// query list.
    ///
    /// Get graph query list.
    pub async fn list(
        &self,
        query: &ListQueriesQuery,
    ) -> Result<Paginated<QueryManagerResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/graph-explorer/query/management/query",
                q,
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/graph-explorer/query/management/query` — Save
    /// graph query.
    ///
    /// Save graph query.
    pub async fn save(
        &self,
        query: &SaveQueryQuery,
        body: &QueryManagementBody,
    ) -> Result<Response<QueryManagementSaveResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<QueryManagementBody, Response<QueryManagementSaveResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/graph-explorer/query/management/query",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/graph-explorer/query/management/query/type-counts`
    /// — Get graph query counts by type.
    ///
    /// Get graph query counts by type.
    pub async fn type_counts(
        &self,
        query: &TypeCountsQuery,
    ) -> Result<Response<QueryTypeCountsResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/graph-explorer/query/management/query/type-counts",
                q,
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/xdr/graph-explorer/query/management/query/{query_id}`
    /// — Delete graph query.
    ///
    /// Delete graph query.
    ///
    /// `query_id`: Query ID. Required (path).
    pub async fn delete(
        &self,
        query_id: impl Into<String>,
        query: &DeleteQueryQuery,
    ) -> Result<Response<QueryManagementUpdateResponse>, Error> {
        let path = format!(
            "/web/api/v2.1/xdr/graph-explorer/query/management/query/{}",
            query_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<QueryManagementUpdateResponse>>(
                Method::DELETE,
                &path,
                q,
                None,
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/xdr/graph-explorer/query/management/query/{query_id}` —
    /// Update graph query.
    ///
    /// Update graph query.
    ///
    /// `query_id`: Query ID. Required (path).
    pub async fn update(
        &self,
        query_id: impl Into<String>,
        query: &UpdateQueryQuery,
        body: &QueryManagementBody,
    ) -> Result<Response<QueryManagementUpdateResponse>, Error> {
        let path = format!(
            "/web/api/v2.1/xdr/graph-explorer/query/management/query/{}",
            query_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<QueryManagementBody, Response<QueryManagementUpdateResponse>>(
                Method::PUT,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/graph-explorer/query/management/recent-queries` —
    /// Get graph recent query list.
    ///
    /// Get graph recent query list.
    pub async fn recent_queries(
        &self,
        query: &RecentQueriesQuery,
    ) -> Result<Response<Vec<RecentQueriesResponse>>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/graph-explorer/query/management/recent-queries",
                q,
            )
            .await?)
    }
}
