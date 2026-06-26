//! `Long Running Query` tag — asynchronous query API.
//!
//! Launch a query, then poll it by id until results are available, and finally
//! delete it. See [`LongRunningQueryService`].

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::long_running_query::QueryResult;
use crate::pagination::Response;

/// `Long Running Query` tag.
///
/// Long Running Query API enables asynchronous querying. Launch a query with
/// [`launch`](LongRunningQueryService::launch), poll it with
/// [`poll`](LongRunningQueryService::poll) until the `data` field is populated,
/// then remove it with [`delete`](LongRunningQueryService::delete).
///
/// # Routing header
///
/// A successful launch response includes an `X-Dataset-Query-Forward-Tag`
/// header that must be applied to the subsequent poll and delete requests for
/// routing. The shared `HttpClient` used by this SDK does not currently expose
/// a way to attach arbitrary per-request headers, so the `forward_tag`
/// arguments on [`poll`](LongRunningQueryService::poll) and
/// [`delete`](LongRunningQueryService::delete) are accepted for API parity but
/// are not transmitted yet. See the crate notes for this limitation.
pub struct LongRunningQueryService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ===========================================================================
// Request body for `POST /sdl/v2/api/queries`
// ===========================================================================

/// Request body for `POST /sdl/v2/api/queries` (`LaunchQueryRequestBody`).
///
/// Only one set of query-type-specific attributes (`log`, `pq`, `top_facets`,
/// `facet_values`, `plot`, `distribution`) should be supplied, matching the
/// specified `query_type`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchQueryBody {
    /// Specifies the type of query to launch. **Required.**
    ///
    /// Allowed values (enum -> `String` for forward-compat):
    /// * `LOG`: Fetch a time-ordered list of matching events, with support for
    ///   pagination.
    /// * `TOP_FACETS`: Find the top facets & facet values for all matching
    ///   events.
    /// * `FACET_VALUES`: Find the top values for a particular facet across all
    ///   matching events.
    /// * `PLOT`: Generate a plot of a numeric function applied to all matching
    ///   events. The plot can be broken down by values of a certain facet.
    /// * `PQ`: Fetch data using Power Query language. Results can be returned
    ///   as a table or a plot.
    /// * `DISTRIBUTION`: Map, as a histogram, the distribution of values for a
    ///   facet.
    pub query_type: String,

    /// List of account ids to query.
    ///
    /// There is no corresponding `siteIds` parameter; supply site ids as part
    /// of the query filter instead, e.g.
    /// `site.id in ('1383518925981773650', '479559923400588751')`. You can
    /// also restrict the query space with an appropriately scoped service
    /// token. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,

    /// Specifies start of the time range to query, using the same syntax as the
    /// Start and End fields in the query UI, or a simple timestamp (seconds,
    /// milliseconds, or nanoseconds since 1/1/1970).
    ///
    /// Defaults to the last 24 hours. If only `start_time` is set, the query
    /// covers 24 hours beginning at it. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,

    /// Specifies end of the time range to query, using the same syntax as the
    /// Start and End fields in the query UI, or a simple timestamp (seconds,
    /// milliseconds, or nanoseconds since 1/1/1970).
    ///
    /// Defaults to the last 24 hours. If only `end_time` is set, the query
    /// covers 24 hours ending at it. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,

    /// Specifies the execution priority for this query; defaults to `LOW`.
    ///
    /// Allowed values (enum -> `String`):
    /// * `HIGH`: Standard query priority for API requests, with tighter rate
    ///   limits.
    /// * `LOW`: Use for scripted queries where a slight delay is acceptable;
    ///   low priority queries are subject to less rate limiting.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_priority: Option<String>,

    /// Controls the scope of the query across accessible data.
    ///
    /// When `true`, global admins query global scope and all accounts;
    /// account/site-level users query within their authorized scope; multi
    /// users query across all authorized accounts/sites. Must be `false` when
    /// specifying scope via `account_ids`. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,

    /// Attributes specific to the `LOG` query type. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<LogAttributes>,

    /// Attributes specific to the `PQ` query type. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pq: Option<PqAttributes>,

    /// Attributes specific to the `TOP_FACETS` query type. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_facets: Option<TopFacetsAttributes>,

    /// Attributes specific to the `FACET_VALUES` query type.
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_values: Option<FacetValuesAttributes>,

    /// Attributes specific to the `PLOT` query type. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plot: Option<PlotAttributes>,

    /// Attributes specific to the `DISTRIBUTION` query type.
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<DistributionAttributes>,
}

impl LaunchQueryBody {
    /// Construct a body with the (required) `query_type`.
    ///
    /// Allowed values: `LOG`, `TOP_FACETS`, `FACET_VALUES`, `PLOT`, `PQ`,
    /// `DISTRIBUTION`.
    pub fn new(query_type: impl Into<String>) -> Self {
        Self {
            query_type: query_type.into(),
            ..Default::default()
        }
    }

    /// Set the list of account ids to query.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Set the start of the time range to query.
    pub fn start_time(mut self, v: impl Into<String>) -> Self {
        self.start_time = Some(v.into());
        self
    }

    /// Set the end of the time range to query.
    pub fn end_time(mut self, v: impl Into<String>) -> Self {
        self.end_time = Some(v.into());
        self
    }

    /// Set the execution priority (`HIGH` or `LOW`).
    pub fn query_priority(mut self, v: impl Into<String>) -> Self {
        self.query_priority = Some(v.into());
        self
    }

    /// Set the `tenant` scope flag.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }

    /// Set the `LOG` query-type attributes.
    pub fn log(mut self, v: LogAttributes) -> Self {
        self.log = Some(v);
        self
    }

    /// Set the `PQ` query-type attributes.
    pub fn pq(mut self, v: PqAttributes) -> Self {
        self.pq = Some(v);
        self
    }

    /// Set the `TOP_FACETS` query-type attributes.
    pub fn top_facets(mut self, v: TopFacetsAttributes) -> Self {
        self.top_facets = Some(v);
        self
    }

    /// Set the `FACET_VALUES` query-type attributes.
    pub fn facet_values(mut self, v: FacetValuesAttributes) -> Self {
        self.facet_values = Some(v);
        self
    }

    /// Set the `PLOT` query-type attributes.
    pub fn plot(mut self, v: PlotAttributes) -> Self {
        self.plot = Some(v);
        self
    }

    /// Set the `DISTRIBUTION` query-type attributes.
    pub fn distribution(mut self, v: DistributionAttributes) -> Self {
        self.distribution = Some(v);
        self
    }
}

/// Attributes specific to the `LOG` query type.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogAttributes {
    /// If `true`, results are returned oldest to newest; if `false`, newest to
    /// oldest (default `false`). Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ascending: Option<bool>,

    /// Cursor returned by a previous `LOG` query, used for pagination.
    ///
    /// When provided, the query returns events starting with this cursor
    /// (inclusive) up to `limit`. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,

    /// Specifies which events to match, using the same syntax as the
    /// Expression field in the query UI. Omit or pass an empty string to match
    /// all events (default `""`). Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// Limits the number of returned log events (default `1000`, minimum `1`).
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    /// When `true` and a `cursor` is provided, pagination continues from the
    /// cursor event exclusively (the cursor event is NOT included), giving
    /// duplicate-free pagination. Defaults to `false`. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
}

/// Attributes specific to the `PQ` (Power Query) query type.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PqAttributes {
    /// The power query to execute. **Required** for `PQ` queries.
    pub query: String,

    /// Specifies the result type of a power query. **Required** for `PQ`
    /// queries.
    ///
    /// Allowed values (enum -> `String`): `TABLE` (default), `PLOT`.
    pub result_type: String,

    /// Clients should set frequency to `HIGH` if the query with the same
    /// expression will be repeated; defaults to `LOW`.
    ///
    /// Allowed values (enum -> `String`): `LOW`, `HIGH`. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
}

impl PqAttributes {
    /// Construct `PQ` attributes with the (required) `query` and `result_type`.
    ///
    /// `result_type` allowed values: `TABLE`, `PLOT`.
    pub fn new(query: impl Into<String>, result_type: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            result_type: result_type.into(),
            frequency: None,
        }
    }

    /// Set the query `frequency` (`LOW` or `HIGH`).
    pub fn frequency(mut self, v: impl Into<String>) -> Self {
        self.frequency = Some(v.into());
        self
    }
}

/// Attributes specific to the `TOP_FACETS` query type.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopFacetsAttributes {
    /// Whether to determine if a value is numeric (default `false`).
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub determine_numeric: Option<bool>,

    /// Specifies which events to match, using the same syntax as the
    /// Expression field in the query UI. Omit or pass an empty string to match
    /// all events. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// Number of facets to return. Facets are sorted by event count, ties
    /// broken alphabetically. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_facets_to_return: Option<i64>,

    /// The maximum number of values to return per facet for the request.
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_values_to_return_per_facet: Option<i64>,
}

/// Attributes specific to the `FACET_VALUES` query type.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FacetValuesAttributes {
    /// Whether to determine if a value is numeric (default `false`).
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub determine_numeric: Option<bool>,

    /// Specifies which events to match, using the same syntax as the
    /// Expression field in the query UI. Omit or pass an empty string to match
    /// all events. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// The maximum number of unique values to return per facet for the request
    /// (default `100`). Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<i64>,

    /// Specifies the facet name to get values for. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Attributes specific to the `PLOT` query type.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlotAttributes {
    /// When `true`, regardless of the function specified in the expression, the
    /// result includes a list of plots with count, mean, min, max, sum,
    /// sumPerSecond, countPerSecond, p10, p50, p90, p95, p99 and p999
    /// aggregations applied (default `false`). Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_aggregate: Option<bool>,

    /// Based on the time range and a target number of slices, auto aligns the
    /// slices to a meaningful boundary (default `false`). Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_align: Option<bool>,

    /// Additional facet to breakdown by. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown_facet: Option<String>,

    /// Expression to query and filter numeric data. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /// Specifies which events to match, using the same syntax as the
    /// Expression field in the query UI. Omit or pass an empty string to match
    /// all events. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,

    /// Clients should set frequency to `HIGH` if the query with the same
    /// expression will be repeated; defaults to `LOW`.
    ///
    /// Allowed values (enum -> `String`): `LOW`, `HIGH`. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,

    /// Width in duration the time range will be divided into. Cannot be used
    /// together with `slices`. Supported units: `seconds`/`s`, `minutes`/`m`,
    /// `hours`/`h`, `days`/`d`, `weeks`/`w`. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slice_width: Option<String>,

    /// Number of parts the time range will be divided into. Cannot be used
    /// together with `slice_width`. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
}

/// Attributes specific to the `DISTRIBUTION` query type.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DistributionAttributes {
    /// The facet to query and aggregate on. Currently all facet queries are
    /// aggregated using the `count` function. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet: Option<String>,

    /// Specifies which events to match, using the same syntax as the
    /// Expression field in the query UI. Omit or pass an empty string to match
    /// all events. Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}

// ===========================================================================
// Query params for `GET /sdl/v2/api/queries/{id}`
// ===========================================================================

/// Query params for `GET /sdl/v2/api/queries/{id}` (poll).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PollQueryQuery {
    /// The step to start return result from. **Required** by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_step_seen: Option<i64>,
}

impl PollQueryQuery {
    /// Set `lastStepSeen` — the step to start returning results from.
    pub fn last_step_seen(mut self, n: i64) -> Self {
        self.last_step_seen = Some(n);
        self
    }
}

impl LongRunningQueryService<'_> {
    /// `POST /sdl/v2/api/queries` — Launch a query.
    ///
    /// Returns a [`QueryResult`] containing the query identifier, status, and,
    /// if the query completes immediately, the full result set in the `data`
    /// field. If the query is still processing, `data` is `null`; use the
    /// returned query id to poll via [`poll`](Self::poll) until results are
    /// available. Log queries default to a 1000-event limit but can paginate
    /// to return essentially unlimited results; power query results are subject
    /// to the standard power query limits on row count and memory consumption.
    ///
    /// A successful response also includes an `X-Dataset-Query-Forward-Tag`
    /// header that must be applied to the subsequent poll and delete requests
    /// for routing. Only one set of attributes (`log`, `pq`, ...) should be
    /// supplied, depending on the specified `query_type`.
    pub async fn launch(&self, body: &LaunchQueryBody) -> Result<Response<QueryResult>, Error> {
        Ok(self.client.http().post("/sdl/v2/api/queries", body).await?)
    }

    /// `GET /sdl/v2/api/queries/{id}` — Poll query.
    ///
    /// Poll a previously launched query by its unique identifier. Returns a
    /// [`QueryResult`]. If the query has not yet completed, the `data` field is
    /// `null`; once it completes, `data` contains the full result set (up to
    /// configured limits). It is recommended to poll every second; queries
    /// expire after the configured TTL (default 30 seconds).
    ///
    /// # Arguments
    /// * `id` - The unique query identifier (path param, required).
    /// * `forward_tag` - The `X-Dataset-Query-Forward-Tag` header value from
    ///   the launch response (required by the API for routing). Accepted for
    ///   parity; see the [service docs](LongRunningQueryService) — the shared
    ///   HTTP client cannot attach this header yet, so it is not transmitted.
    /// * `query` - Carries the required `lastStepSeen` query param.
    pub async fn poll(
        &self,
        id: impl Into<String>,
        forward_tag: impl Into<String>,
        query: &PollQueryQuery,
    ) -> Result<Response<QueryResult>, Error> {
        let _forward_tag = forward_tag.into();
        let path = format!("/sdl/v2/api/queries/{}", id.into());
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `DELETE /sdl/v2/api/queries/{id}` — Delete query.
    ///
    /// Remove the query from the list of launched queries. Clients are required
    /// to call this once their query is complete; subsequent polls using the
    /// same token return a not-found response. Responds `204 No Content` on
    /// success.
    ///
    /// # Arguments
    /// * `id` - The unique query identifier (path param, required).
    /// * `forward_tag` - The `X-Dataset-Query-Forward-Tag` header value from
    ///   the launch response (required by the API for routing). Accepted for
    ///   parity; see the [service docs](LongRunningQueryService) — the shared
    ///   HTTP client cannot attach this header yet, so it is not transmitted.
    pub async fn delete(
        &self,
        id: impl Into<String>,
        forward_tag: impl Into<String>,
    ) -> Result<Response<serde_json::Value>, Error> {
        let _forward_tag = forward_tag.into();
        let path = format!("/sdl/v2/api/queries/{}", id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<serde_json::Value>>(
                sentinelone_http::Method::DELETE,
                &path,
                None,
                None,
            )
            .await?)
    }
}
