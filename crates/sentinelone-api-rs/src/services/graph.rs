//! Service for the `Graph` tag — query the XDR security graph.

use sentinelone_http::Method;
use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::graph::QueryGraphResponse;
use crate::pagination::Response;

/// `Graph` tag — query graph.
///
/// Endpoints for querying the XDR Graph Explorer: run query-builder filters
/// against the full graph (v1 and v2) and fetch the sub-graph of a given asset.
pub struct GraphService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query the graph (v1)
// ---------------------------------------------------------------------------

/// Query params for `POST /web/api/v2.1/xdr/graph-explorer/query/explorer`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryExplorerQuery {
    /// Mock. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mock: Option<bool>,
    /// List of Group IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Account IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Limit. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Continuation token. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// List of Site IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl QueryExplorerQuery {
    /// Mock. Optional.
    pub fn mock(mut self, v: bool) -> Self {
        self.mock = Some(v);
        self
    }
    /// List of Group IDs to filter by. Optional.
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
    /// List of Account IDs to filter by. Optional.
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
    /// Limit. Optional.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Continuation token. Optional.
    pub fn continuation_token(mut self, t: impl Into<String>) -> Self {
        self.continuation_token = Some(t.into());
        self
    }
    /// List of Site IDs to filter by. Optional.
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
}

/// Request body for `POST /web/api/v2.1/xdr/graph-explorer/query/explorer`
/// (`QueryGraphInputSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryExplorerBody {
    /// Search. The query-builder filter tree (`SearchEntity`). Freeform /
    /// recursively nested in the spec. Required.
    pub search: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Query the graph (v2)
// ---------------------------------------------------------------------------

/// Query params for `POST /web/api/v2.1/xdr/graph-explorer/query/explorer/v2`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryExplorerV2Query {
    /// Mock. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mock: Option<bool>,
    /// List of Group IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Account IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Limit. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Continuation token. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// List of Site IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl QueryExplorerV2Query {
    /// Mock. Optional.
    pub fn mock(mut self, v: bool) -> Self {
        self.mock = Some(v);
        self
    }
    /// List of Group IDs to filter by. Optional.
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
    /// List of Account IDs to filter by. Optional.
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
    /// Limit. Optional.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Continuation token. Optional.
    pub fn continuation_token(mut self, t: impl Into<String>) -> Self {
        self.continuation_token = Some(t.into());
        self
    }
    /// List of Site IDs to filter by. Optional.
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
}

/// Request body for `POST /web/api/v2.1/xdr/graph-explorer/query/explorer/v2`
/// (`QueryGraphInputSchemaV2`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryExplorerV2Body {
    /// Name. Required.
    pub name: String,
    /// Search. The query-builder filter tree (`SearchEntity`). Freeform /
    /// recursively nested in the spec. Required.
    pub search: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Query the sub-graph
// ---------------------------------------------------------------------------

/// Query params for `POST /web/api/v2.1/xdr/graph-explorer/query/subgraph`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySubgraphQuery {
    /// List of Account IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by (comma-separated). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl QuerySubgraphQuery {
    /// List of Account IDs to filter by. Optional.
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
    /// List of Site IDs to filter by. Optional.
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
    /// List of Group IDs to filter by. Optional.
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

/// Request body for `POST /web/api/v2.1/xdr/graph-explorer/query/subgraph`
/// (`QuerySubGrapInputSchema`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySubgraphBody {
    /// List of Account IDs to filter by. Optional (1..=500 items in spec).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by. Optional (1..=500 items in spec).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// Ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// List of Group IDs to filter by. Optional (1..=500 items in spec).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

impl GraphService<'_> {
    /// `POST /web/api/v2.1/xdr/graph-explorer/query/explorer` — Query the graph
    /// based on query builder filters.
    ///
    /// Query the graph.
    pub async fn query_explorer(
        &self,
        query: &QueryExplorerQuery,
        body: &QueryExplorerBody,
    ) -> Result<Response<QueryGraphResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<QueryExplorerBody, Response<QueryGraphResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/graph-explorer/query/explorer",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/graph-explorer/query/explorer/v2` — Query the
    /// graph based on query builder filters.
    ///
    /// Query the graph.
    pub async fn query_explorer_v2(
        &self,
        query: &QueryExplorerV2Query,
        body: &QueryExplorerV2Body,
    ) -> Result<Response<QueryGraphResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<QueryExplorerV2Body, Response<QueryGraphResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/graph-explorer/query/explorer/v2",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/graph-explorer/query/subgraph` — Query the sub
    /// graph of an asset type and id.
    ///
    /// Query the sub graph of an asset type and id.
    pub async fn query_subgraph(
        &self,
        query: &QuerySubgraphQuery,
        body: &QuerySubgraphBody,
    ) -> Result<Response<QueryGraphResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<QuerySubgraphBody, Response<QueryGraphResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/graph-explorer/query/subgraph",
                q,
                Some(body),
            )
            .await?)
    }
}
