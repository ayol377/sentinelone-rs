//! Low-level async client for the SentinelOne **XDR / Singularity Data Lake**
//! API (DataSet lineage).
//!
//! Host: e.g. `https://xdr.ap1.sentinelone.net`. Auth: `Bearer`.
//!
//! Surface (all hand-written, clean-room from the DataSet HTTP API):
//!
//! Synchronous queries:
//! - [`XdrClient::query`] / `POST /api/query` — standard log/event query
//! - [`XdrClient::power_query`] / `POST /api/powerQuery` — PQL
//! - [`XdrClient::facet_query`] / `POST /api/facetQuery` — top values of a field
//! - [`XdrClient::numeric_query`] / `POST /api/numericQuery` — a number over time
//! - [`XdrClient::timeseries_query`] / `POST /api/timeseriesQuery` — batched series
//!
//! Ingestion:
//! - [`XdrClient::add_events`] / `POST /api/addEvents`
//! - [`XdrClient::upload_logs`] / `POST /api/uploadLogs`
//!
//! Long Running Query (async v2):
//! - [`XdrClient::lrq_launch`] / `POST /v2/api/queries`
//! - [`XdrClient::lrq_poll`] / `GET /v2/api/queries/{id}`
//! - [`XdrClient::lrq_cancel`] / `DELETE /v2/api/queries/{id}`
//!
//! Each typed method has a `*_with` form taking the full request builder for
//! parameters beyond the common case.

pub mod error;
pub mod models;

use sentinelone_http::{Auth, HttpClient, Method};

pub use error::Error;
pub use models::{
    AddEventsRequest, AddEventsResponse, Event, Facet, FacetQueryRequest, FacetQueryResponse,
    LogQueryRequest, LogQueryResponse, LrqLaunchResponse, LrqRequest, LrqStatusResponse,
    NumericQueryRequest, NumericQueryResponse, PowerQueryRequest, PowerQueryResponse,
    TimeseriesQueryRequest, TimeseriesQueryResponse, TimeseriesQuerySpec, TimeseriesResult,
    UploadLogsResponse,
};

/// Low-level XDR / Data Lake client. Cheap to clone (`Arc`-backed inside).
#[derive(Clone)]
pub struct XdrClient {
    http: HttpClient,
}

impl XdrClient {
    /// `host` e.g. `https://xdr.ap1.sentinelone.net`; `bearer` = DataSet API key.
    pub fn new(host: &str, bearer: impl Into<String>) -> Result<Self, Error> {
        let http = HttpClient::new(host, Auth::Bearer(bearer.into()))?;
        Ok(Self { http })
    }

    pub(crate) fn http(&self) -> &HttpClient {
        &self.http
    }

    // ---- standard log/event query ----------------------------------------

    /// `POST /api/query` — standard log/event query over a DataSet `filter`.
    pub async fn query(
        &self,
        filter: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Result<LogQueryResponse, Error> {
        self.query_with(&LogQueryRequest::new(filter, start, end)).await
    }

    /// `POST /api/query` with a fully-specified [`LogQueryRequest`]
    /// (max count, paging, columns, continuation, priority).
    pub async fn query_with(&self, req: &LogQueryRequest) -> Result<LogQueryResponse, Error> {
        Ok(self.http().post("/api/query", req).await?)
    }

    // ---- PowerQuery (PQL) -------------------------------------------------

    /// `POST /api/powerQuery` — synchronous PowerQuery (PQL).
    ///
    /// `start`/`end` accept DataSet time syntax (epoch ns, RFC3339, or relative
    /// like `"24 hours"` / `"now"`).
    pub async fn power_query(
        &self,
        pql: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Result<PowerQueryResponse, Error> {
        self.power_query_with(&PowerQueryRequest::new(pql, start, end)).await
    }

    /// `POST /api/powerQuery` with a fully-specified [`PowerQueryRequest`].
    pub async fn power_query_with(
        &self,
        req: &PowerQueryRequest,
    ) -> Result<PowerQueryResponse, Error> {
        Ok(self.http().post("/api/powerQuery", req).await?)
    }

    // ---- facet query ------------------------------------------------------

    /// `POST /api/facetQuery` — distinct values of `field` and their counts.
    pub async fn facet_query(
        &self,
        filter: impl Into<String>,
        field: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Result<FacetQueryResponse, Error> {
        self.facet_query_with(&FacetQueryRequest::new(filter, field, start, end)).await
    }

    /// `POST /api/facetQuery` with a fully-specified [`FacetQueryRequest`].
    pub async fn facet_query_with(
        &self,
        req: &FacetQueryRequest,
    ) -> Result<FacetQueryResponse, Error> {
        Ok(self.http().post("/api/facetQuery", req).await?)
    }

    // ---- numeric query ----------------------------------------------------

    /// `POST /api/numericQuery` — a numeric aggregation (optionally bucketed).
    pub async fn numeric_query(
        &self,
        filter: impl Into<String>,
        function: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Result<NumericQueryResponse, Error> {
        self.numeric_query_with(&NumericQueryRequest::new(filter, function, start, end)).await
    }

    /// `POST /api/numericQuery` with a fully-specified [`NumericQueryRequest`].
    pub async fn numeric_query_with(
        &self,
        req: &NumericQueryRequest,
    ) -> Result<NumericQueryResponse, Error> {
        Ok(self.http().post("/api/numericQuery", req).await?)
    }

    // ---- timeseries query -------------------------------------------------

    /// `POST /api/timeseriesQuery` — evaluate a batch of numeric series.
    pub async fn timeseries_query(
        &self,
        specs: Vec<TimeseriesQuerySpec>,
    ) -> Result<TimeseriesQueryResponse, Error> {
        self.timeseries_query_with(&TimeseriesQueryRequest::new(specs)).await
    }

    /// `POST /api/timeseriesQuery` with a fully-specified request.
    pub async fn timeseries_query_with(
        &self,
        req: &TimeseriesQueryRequest,
    ) -> Result<TimeseriesQueryResponse, Error> {
        Ok(self.http().post("/api/timeseriesQuery", req).await?)
    }

    // ---- ingestion --------------------------------------------------------

    /// `POST /api/addEvents` — ingest structured events into the Data Lake.
    pub async fn add_events(&self, req: &AddEventsRequest) -> Result<AddEventsResponse, Error> {
        Ok(self.http().post("/api/addEvents", req).await?)
    }

    /// `POST /api/uploadLogs` — upload raw log text.
    ///
    /// `query` is the pre-serialized querystring of upload parameters (parser,
    /// host, logfile, server fields, …); `body` is the raw log payload. Sent as
    /// `text/plain`.
    pub async fn upload_logs(
        &self,
        query: Option<&str>,
        body: String,
    ) -> Result<UploadLogsResponse, Error> {
        Ok(self
            .http()
            .post_raw("/api/uploadLogs", query, "text/plain", body)
            .await?)
    }

    // ---- Long Running Query (async v2) ------------------------------------

    /// `POST /v2/api/queries` — launch an async Long Running Query. Returns the
    /// query id (in the response) to poll with [`XdrClient::lrq_poll`].
    pub async fn lrq_launch(&self, req: &LrqRequest) -> Result<LrqLaunchResponse, Error> {
        Ok(self.http().post("/v2/api/queries", req).await?)
    }

    /// `GET /v2/api/queries/{id}` — poll an LRQ for status and (when ready)
    /// results. Pass `next` to page through results returned by a prior poll.
    pub async fn lrq_poll(
        &self,
        id: &str,
        next: Option<&str>,
    ) -> Result<LrqStatusResponse, Error> {
        let path = format!("/v2/api/queries/{id}");
        let query = next.map(|n| format!("next={n}"));
        Ok(self.http().get(&path, query.as_deref()).await?)
    }

    /// `DELETE /v2/api/queries/{id}` — cancel a running LRQ.
    pub async fn lrq_cancel(&self, id: &str) -> Result<serde_json::Value, Error> {
        let path = format!("/v2/api/queries/{id}");
        Ok(self
            .http()
            .request_json::<(), serde_json::Value>(Method::DELETE, &path, None, None)
            .await?)
    }
}
