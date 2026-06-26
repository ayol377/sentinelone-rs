//! Request and response types for the SentinelOne XDR / Data Lake (DataSet) API.
//!
//! Request bodies are fully typed (we control them; the DataSet HTTP API is
//! well documented). Response envelopes are typed for the fields callers read
//! most, plus a `#[serde(flatten)] extra` catch-all so nothing the server
//! returns is ever silently dropped — mirroring [`PowerQueryResponse`].
//!
//! Time fields (`start_time` / `end_time`) accept DataSet time syntax: epoch
//! nanoseconds, RFC3339, or relative expressions like `"24 hours"` / `"now"`.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

// =============================================================================
// Standard log/event query — POST /api/query
// =============================================================================

/// Request body for `POST /api/query` (standard log/event query).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogQueryRequest {
    /// Query kind. Almost always `"log"` (the default). Other values:
    /// `"numeric"`, `"facet"` — but prefer the dedicated endpoints for those.
    pub query_type: String,
    /// DataSet filter expression selecting events (e.g.
    /// `serverHost = 'web-1' and severity >= 3`). Empty = all events.
    pub filter: String,
    /// Start of the time window.
    pub start_time: String,
    /// End of the time window.
    pub end_time: String,
    /// Maximum number of events to return (server caps at ~5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,
    /// `"head"` (oldest first) or `"tail"` (newest first). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_mode: Option<String>,
    /// Comma-separated list of columns/fields to return. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<String>,
    /// Continuation token from a previous response, to page further. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    /// Server scheduling priority: `"low"` or `"high"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}

impl LogQueryRequest {
    /// New log query over `filter` within `[start, end]`.
    pub fn new(
        filter: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Self {
        Self {
            query_type: "log".into(),
            filter: filter.into(),
            start_time: start.into(),
            end_time: end.into(),
            max_count: None,
            page_mode: None,
            columns: None,
            continuation_token: None,
            priority: None,
        }
    }

    /// Cap the number of returned events.
    pub fn max_count(mut self, n: i64) -> Self {
        self.max_count = Some(n);
        self
    }
    /// Set page direction (`"head"` / `"tail"`).
    pub fn page_mode(mut self, mode: impl Into<String>) -> Self {
        self.page_mode = Some(mode.into());
        self
    }
    /// Restrict to specific columns (comma-separated).
    pub fn columns(mut self, cols: impl Into<String>) -> Self {
        self.columns = Some(cols.into());
        self
    }
    /// Continue a previous page.
    pub fn continuation_token(mut self, token: impl Into<String>) -> Self {
        self.continuation_token = Some(token.into());
        self
    }
    /// Set scheduling priority.
    pub fn priority(mut self, p: impl Into<String>) -> Self {
        self.priority = Some(p.into());
        self
    }
}

/// Response from `POST /api/query`.
#[derive(Debug, Clone, Deserialize)]
pub struct LogQueryResponse {
    /// Server status string (e.g. `"success"`).
    #[serde(default)]
    pub status: Option<String>,
    /// Matched events. The DataSet API names this `matches`; accepted under
    /// `events` too for forward-compat.
    #[serde(default, alias = "events")]
    pub matches: Vec<Value>,
    /// Total number of events matching the filter (may exceed `matches.len()`).
    #[serde(default)]
    pub matching_events: Option<i64>,
    /// Session metadata for the matched events, keyed by session id.
    #[serde(default)]
    pub sessions: Option<Value>,
    /// Token to fetch the next page, if more events are available.
    #[serde(default)]
    pub continuation_token: Option<String>,
    /// Server-side execution time, milliseconds.
    #[serde(default)]
    pub execution_time: Option<f64>,
    /// Any other fields the API returns (warnings, cost, etc.).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

// =============================================================================
// PowerQuery (PQL) — POST /api/powerQuery
// =============================================================================

/// Request body for `POST /api/powerQuery`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PowerQueryRequest {
    /// The PowerQuery (PQL) text.
    pub query: String,
    /// Start of the time window.
    pub start_time: String,
    /// End of the time window.
    pub end_time: String,
    /// Server scheduling priority: `"low"` or `"high"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
}

impl PowerQueryRequest {
    /// New PowerQuery over `[start, end]`.
    pub fn new(
        query: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Self {
        Self {
            query: query.into(),
            start_time: start.into(),
            end_time: end.into(),
            priority: None,
        }
    }
    /// Set scheduling priority.
    pub fn priority(mut self, p: impl Into<String>) -> Self {
        self.priority = Some(p.into());
        self
    }
}

/// PowerQuery result table. `columns` describes fields; `values` are the rows.
#[derive(Debug, Clone, Deserialize)]
pub struct PowerQueryResponse {
    /// Column descriptors (name/type per output field).
    #[serde(default)]
    pub columns: Vec<Value>,
    /// Result rows, each a vector of cell values aligned to `columns`.
    #[serde(default)]
    pub values: Vec<Vec<Value>>,
    /// Number of matching events, when reported.
    #[serde(default)]
    pub matches: Option<i64>,
    /// Anything else the API returns (status, warnings, continuation, …).
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

// =============================================================================
// Facet query — POST /api/facetQuery
// =============================================================================

/// Request body for `POST /api/facetQuery` (top values of a field).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FacetQueryRequest {
    /// DataSet filter selecting the events to facet over.
    pub filter: String,
    /// The field whose distinct values + counts to return.
    pub field: String,
    /// Start of the time window.
    pub start_time: String,
    /// End of the time window.
    pub end_time: String,
    /// Maximum number of distinct values to return. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,
}

impl FacetQueryRequest {
    /// Facet `field` over events matching `filter` in `[start, end]`.
    pub fn new(
        filter: impl Into<String>,
        field: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Self {
        Self {
            filter: filter.into(),
            field: field.into(),
            start_time: start.into(),
            end_time: end.into(),
            max_count: None,
        }
    }
    /// Cap the number of returned facet values.
    pub fn max_count(mut self, n: i64) -> Self {
        self.max_count = Some(n);
        self
    }
}

/// One facet bucket: a field value and how many events had it.
#[derive(Debug, Clone, Deserialize)]
pub struct Facet {
    /// The field value.
    pub value: Value,
    /// Number of matching events with this value.
    #[serde(default)]
    pub count: i64,
}

/// Response from `POST /api/facetQuery`.
#[derive(Debug, Clone, Deserialize)]
pub struct FacetQueryResponse {
    /// Facet buckets, most frequent first. DataSet names this `values`; also
    /// accepted as `facets`.
    #[serde(default, alias = "facets")]
    pub values: Vec<Facet>,
    /// Total events matching the filter.
    #[serde(default)]
    pub matching_events: Option<i64>,
    /// Any other fields the API returns.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

// =============================================================================
// Numeric query — POST /api/numericQuery
// =============================================================================

/// Request body for `POST /api/numericQuery` (a numeric value over time).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NumericQueryRequest {
    /// DataSet filter selecting the events to aggregate.
    pub filter: String,
    /// Aggregation function, e.g. `"count"`, `"mean:responseTime"`,
    /// `"p90:latency"`, `"sum:bytes"`.
    pub function: String,
    /// Start of the time window.
    pub start_time: String,
    /// End of the time window.
    pub end_time: String,
    /// Number of equal-width time buckets to split the window into. Optional
    /// (omit for a single value over the whole window).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<i64>,
}

impl NumericQueryRequest {
    /// Aggregate `function` over events matching `filter` in `[start, end]`.
    pub fn new(
        filter: impl Into<String>,
        function: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Self {
        Self {
            filter: filter.into(),
            function: function.into(),
            start_time: start.into(),
            end_time: end.into(),
            buckets: None,
        }
    }
    /// Split the window into `n` time buckets.
    pub fn buckets(mut self, n: i64) -> Self {
        self.buckets = Some(n);
        self
    }
}

/// Response from `POST /api/numericQuery`. `values` holds one number per bucket
/// (or a single element when `buckets` was not set).
#[derive(Debug, Clone, Deserialize)]
pub struct NumericQueryResponse {
    /// One aggregated value per time bucket.
    #[serde(default)]
    pub values: Vec<f64>,
    /// Server warnings, if any.
    #[serde(default)]
    pub warnings: Vec<Value>,
    /// Any other fields the API returns.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

// =============================================================================
// Timeseries query — POST /api/timeseriesQuery (batch of numeric queries)
// =============================================================================

/// One numeric series within a batched timeseries query.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesQuerySpec {
    /// DataSet filter for this series.
    pub filter: String,
    /// Aggregation function (see [`NumericQueryRequest::function`]).
    pub function: String,
    /// Start of the time window.
    pub start_time: String,
    /// End of the time window.
    pub end_time: String,
    /// Number of time buckets. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<i64>,
}

impl TimeseriesQuerySpec {
    /// New series spec.
    pub fn new(
        filter: impl Into<String>,
        function: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Self {
        Self {
            filter: filter.into(),
            function: function.into(),
            start_time: start.into(),
            end_time: end.into(),
            buckets: None,
        }
    }
    /// Split into `n` buckets.
    pub fn buckets(mut self, n: i64) -> Self {
        self.buckets = Some(n);
        self
    }
}

/// Request body for `POST /api/timeseriesQuery`: many series in one round-trip.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesQueryRequest {
    /// The series to evaluate.
    pub queries: Vec<TimeseriesQuerySpec>,
}

impl TimeseriesQueryRequest {
    /// Wrap a set of series.
    pub fn new(queries: Vec<TimeseriesQuerySpec>) -> Self {
        Self { queries }
    }
}

/// Result for a single series in a timeseries response.
#[derive(Debug, Clone, Deserialize)]
pub struct TimeseriesResult {
    /// One value per time bucket for this series.
    #[serde(default)]
    pub values: Vec<f64>,
    /// Any other per-series fields.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Response from `POST /api/timeseriesQuery`, one [`TimeseriesResult`] per input
/// series (order-aligned with the request).
#[derive(Debug, Clone, Deserialize)]
pub struct TimeseriesQueryResponse {
    /// Per-series results, aligned with the request order.
    #[serde(default)]
    pub results: Vec<TimeseriesResult>,
    /// Any other fields the API returns.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

// =============================================================================
// Event ingestion — POST /api/addEvents
// =============================================================================

/// A single event to ingest via `POST /api/addEvents`.
#[derive(Debug, Clone, Serialize)]
pub struct Event {
    /// Timestamp in epoch nanoseconds (as a string to preserve precision).
    pub ts: String,
    /// Optional thread id this event belongs to (see
    /// [`AddEventsRequest::threads`]).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<String>,
    /// Severity 0–6 (DataSet finest→fatal). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sev: Option<i64>,
    /// Event attributes (the structured fields of the event).
    pub attrs: Value,
}

/// Request body for `POST /api/addEvents` (ingest events into the Data Lake).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddEventsRequest {
    /// Unique id for this upload session (a random string per process run).
    pub session: String,
    /// Stable attributes shared by all events in the session (host, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_info: Option<Value>,
    /// The events to ingest.
    pub events: Vec<Event>,
    /// Optional thread declarations referenced by [`Event::thread`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads: Option<Vec<Value>>,
}

impl AddEventsRequest {
    /// New ingestion request for `session` carrying `events`.
    pub fn new(session: impl Into<String>, events: Vec<Event>) -> Self {
        Self {
            session: session.into(),
            session_info: None,
            events,
            threads: None,
        }
    }
    /// Attach shared session attributes.
    pub fn session_info(mut self, info: Value) -> Self {
        self.session_info = Some(info);
        self
    }
    /// Declare threads.
    pub fn threads(mut self, threads: Vec<Value>) -> Self {
        self.threads = Some(threads);
        self
    }
}

/// Response from `POST /api/addEvents`.
#[derive(Debug, Clone, Deserialize)]
pub struct AddEventsResponse {
    /// Status string (`"success"` on accept).
    #[serde(default)]
    pub status: Option<String>,
    /// Bytes billed for this upload.
    #[serde(default)]
    pub bytes_charged: Option<i64>,
    /// Any other fields the API returns.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Response from `POST /api/uploadLogs`.
#[derive(Debug, Clone, Deserialize)]
pub struct UploadLogsResponse {
    /// Status string.
    #[serde(default)]
    pub status: Option<String>,
    /// Any other fields the API returns.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

// =============================================================================
// Long Running Query (LRQ) — async v2 API
// =============================================================================

/// Request body for `POST /v2/api/queries` (launch an async Long Running Query).
///
/// The accepted fields depend on `query_type`: a `"log"` query uses `filter`;
/// a `"powerQuery"` uses `power_query`. Anything not modeled here can be added
/// via [`LrqRequest::extra`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LrqRequest {
    /// Query kind: `"log"` or `"powerQuery"`.
    pub query_type: String,
    /// Start of the time window.
    pub start_time: String,
    /// End of the time window.
    pub end_time: String,
    /// DataSet filter (for `query_type = "log"`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// PowerQuery text (for `query_type = "powerQuery"`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_query: Option<String>,
    /// Max events/rows to return. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Comma-separated columns. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<String>,
    /// Escape hatch for any additional top-level body fields.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

impl LrqRequest {
    /// Launch a `"log"` LRQ over `filter`.
    pub fn log(
        filter: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Self {
        Self {
            query_type: "log".into(),
            start_time: start.into(),
            end_time: end.into(),
            filter: Some(filter.into()),
            power_query: None,
            limit: None,
            columns: None,
            extra: Map::new(),
        }
    }

    /// Launch a `"powerQuery"` LRQ over `pql`.
    pub fn power_query(
        pql: impl Into<String>,
        start: impl Into<String>,
        end: impl Into<String>,
    ) -> Self {
        Self {
            query_type: "powerQuery".into(),
            start_time: start.into(),
            end_time: end.into(),
            filter: None,
            power_query: Some(pql.into()),
            limit: None,
            columns: None,
            extra: Map::new(),
        }
    }

    /// Cap results.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Restrict columns (comma-separated).
    pub fn columns(mut self, cols: impl Into<String>) -> Self {
        self.columns = Some(cols.into());
        self
    }
    /// Set an arbitrary extra body field.
    pub fn with(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }
}

/// Response from launching an LRQ (`POST /v2/api/queries`).
#[derive(Debug, Clone, Deserialize)]
pub struct LrqLaunchResponse {
    /// The query id, used to poll and cancel.
    #[serde(default, alias = "queryId")]
    pub id: Option<String>,
    /// Initial status payload, if returned inline.
    #[serde(default)]
    pub status: Option<Value>,
    /// Any other fields the API returns.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Response from polling an LRQ (`GET /v2/api/queries/{id}`).
///
/// LRQ status/result payloads are rich and vary by query type; the commonly
/// read fields are typed and everything else is preserved in [`LrqStatusResponse::extra`].
#[derive(Debug, Clone, Deserialize)]
pub struct LrqStatusResponse {
    /// The query id.
    #[serde(default, alias = "queryId")]
    pub id: Option<String>,
    /// Lifecycle state, e.g. `"RUNNING"`, `"FINISHED"`, `"FAILED"`,
    /// `"CANCELLED"`. The exact field shape varies; also surfaced in `extra`.
    #[serde(default, alias = "state")]
    pub status: Option<Value>,
    /// Whether the server considers the query complete.
    #[serde(default)]
    pub is_complete: Option<bool>,
    /// Result payload once available (columns/values or events, by query type).
    #[serde(default)]
    pub data: Option<Value>,
    /// Continuation/next token for paging results.
    #[serde(default)]
    pub next: Option<String>,
    /// Any other fields the API returns.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}
