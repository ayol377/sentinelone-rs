//! Service for the `Deep Visibility` tag.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::deep_visibility::{
    DeepVisibilityEvent, DeepVisibilityFileDownloadLink, DeepVisibilityPowerQuery,
    DeepVisibilityQueryId, DeepVisibilityQueryStatus, DeepVisibilitySuccess,
};
use crate::pagination::{Paginated, Response};

/// `Deep Visibility` tag — Deep Visibility features.
pub struct DeepVisibilityService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Body types
// ---------------------------------------------------------------------------

/// Request body for `POST /web/api/v2.1/dv/cancel-query`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelQueryBody {
    /// QueryId obtained when creating a query under Create Query (e.g. `q1xx2xx3`).
    /// Required.
    pub query_id: String,
}

impl CancelQueryBody {
    /// Build a cancel-query body for the given query id.
    pub fn new(query_id: impl Into<String>) -> Self {
        Self { query_id: query_id.into() }
    }
}

/// Request body for `POST /web/api/v2.1/dv/init-query`
/// (`DeepVisibilityApiRequestSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitQueryBody {
    /// Events matching the query search term will be returned. Required.
    /// Only S1QL 1.0 syntax is supported (e.g. `AgentName IS NOT EMPTY`).
    pub query: String,
    /// Events created after this timestamp (date-time string,
    /// e.g. `2018-02-27T04:49:26.257525Z`). Required.
    pub from_date: String,
    /// Events created before or at this timestamp (date-time string). Required.
    pub to_date: String,
    /// List of Account IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// Query Search Type — only one is allowed (1 item). Optional/nullable.
    /// Allowed values: `events`, `processState`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_type: Option<Vec<String>>,
    /// Time frame that the query was performed on; when omitted defaults to
    /// `Last 48 Hours`. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_frame: Option<String>,
    /// Limit number of returned items (1-100000). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Show all fields or just priority fields. Optional. From Management
    /// version Rio (Feb 2022) the default is `false` instead of `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_verbose: Option<bool>,
}

impl InitQueryBody {
    /// Build an init-query body from the three required fields.
    pub fn new(
        query: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
    ) -> Self {
        Self {
            query: query.into(),
            from_date: from_date.into(),
            to_date: to_date.into(),
            account_ids: None,
            site_ids: None,
            query_type: None,
            time_frame: None,
            limit: None,
            is_verbose: None,
        }
    }
    /// Set the list of Account IDs to filter by (1-500 items).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set the list of Site IDs to filter by (1-500 items).
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set the Query Search Type (only one is allowed). Allowed values:
    /// `events`, `processState`.
    pub fn query_type<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.query_type = Some(types.into_iter().map(Into::into).collect());
        self
    }
    /// Set the time frame (defaults to `Last 48 Hours` when omitted).
    pub fn time_frame(mut self, v: impl Into<String>) -> Self {
        self.time_frame = Some(v.into());
        self
    }
    /// Set the limit of returned items (1-100000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Set whether to show all fields (`true`) or just priority fields (`false`).
    pub fn is_verbose(mut self, v: bool) -> Self {
        self.is_verbose = Some(v);
        self
    }
}

/// Request body for `POST /web/api/v2.1/dv/events/pq`
/// (`DeepVisibilityPQRequestSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePowerQueryBody {
    /// Events matching the query search term will be returned. Required.
    /// Example: `event.time = * | columns eventTime = event.time, ...`.
    pub query: String,
    /// Events created after this timestamp (date-time string,
    /// e.g. `2018-02-27T04:49:26.257525Z`). Required.
    pub from_date: String,
    /// Events created before or at this timestamp (date-time string). Required.
    pub to_date: String,
    /// List of Account IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// Limit number of returned items (1-100000). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl CreatePowerQueryBody {
    /// Build a Power Query body from the three required fields.
    pub fn new(
        query: impl Into<String>,
        from_date: impl Into<String>,
        to_date: impl Into<String>,
    ) -> Self {
        Self {
            query: query.into(),
            from_date: from_date.into(),
            to_date: to_date.into(),
            account_ids: None,
            site_ids: None,
            limit: None,
        }
    }
    /// Set the list of Account IDs to filter by (1-500 items).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set the list of Site IDs to filter by (1-500 items).
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set the limit of returned items (1-100000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
}

// ---------------------------------------------------------------------------
// Query types
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/dv/events` and
/// `GET /web/api/v2.1/dv/events/{event_type}`.
///
/// `queryId` is required by the API and is set via [`EventsQuery::new`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventsQuery {
    /// QueryId obtained when creating a query under Create Query
    /// (e.g. `q1xx2xx3`). Required.
    pub query_id: String,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Should be used instead of
    /// `skip`. Supports sort by `createdAt`, `pid`, `processStartTime`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Create a sub query to run on the data that was already pulled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_query: Option<String>,
    /// Events sorted by field (e.g. `createdAt`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Event sorting order. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl EventsQuery {
    /// Build an events query for the given (required) query id.
    pub fn new(query_id: impl Into<String>) -> Self {
        Self {
            query_id: query_id.into(),
            skip: None,
            limit: None,
            cursor: None,
            sub_query: None,
            sort_by: None,
            sort_order: None,
        }
    }
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
    /// Create a sub query to run on the data that was already pulled.
    pub fn sub_query(mut self, q: impl Into<String>) -> Self {
        self.sub_query = Some(q.into());
        self
    }
    /// Events sorted by field (e.g. `createdAt`).
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Event sorting order. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/dv/events/pq-ping`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PingPowerQuery {
    /// QueryId query param. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
}

impl PingPowerQuery {
    /// Set the query id to ping.
    pub fn query_id(mut self, v: impl Into<String>) -> Self {
        self.query_id = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/dv/fetch-file`.
///
/// `downloadToken` is required by the API and is set via [`FetchFileQuery::new`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchFileQuery {
    /// Download token. Required.
    pub download_token: String,
}

impl FetchFileQuery {
    /// Build a fetch-file query for the given (required) download token.
    pub fn new(download_token: impl Into<String>) -> Self {
        Self { download_token: download_token.into() }
    }
}

/// Query params for `GET /web/api/v2.1/dv/process-state`.
///
/// `queryId` is required by the API and is set via [`ProcessStateQuery::new`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessStateQuery {
    /// QueryId obtained when creating a query under Create Query
    /// (e.g. `q1xx2xx3`). Required.
    pub query_id: String,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Events sorted by field (e.g. `SrcProcStartTime`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Event sorting order. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl ProcessStateQuery {
    /// Build a process-state query for the given (required) query id.
    pub fn new(query_id: impl Into<String>) -> Self {
        Self {
            query_id: query_id.into(),
            skip: None,
            limit: None,
            cursor: None,
            sort_by: None,
            sort_order: None,
        }
    }
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
    /// Events sorted by field (e.g. `SrcProcStartTime`).
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Event sorting order. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/dv/query-status`.
///
/// `queryId` is required by the API and is set via [`QueryStatusQuery::new`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryStatusQuery {
    /// QueryId obtained when creating a query under Create Query
    /// (e.g. `q1xx2xx3`). Required.
    pub query_id: String,
}

impl QueryStatusQuery {
    /// Build a query-status query for the given (required) query id.
    pub fn new(query_id: impl Into<String>) -> Self {
        Self { query_id: query_id.into() }
    }
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl DeepVisibilityService<'_> {
    /// `POST /web/api/v2.1/dv/cancel-query` — [DEPRECATED] Cancel Running Query.
    ///
    /// Stop a Deep Visibility Query by queryId. The body is
    /// `{"queryID":"string_ID"}`. Get the ID of the Deep Visibility query or
    /// Power Query from "init-query". See "Create Query and get QueryId".
    /// Deep Visibility requires Complete SKU.
    pub async fn cancel_query(
        &self,
        body: &CancelQueryBody,
    ) -> Result<Response<DeepVisibilitySuccess>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/dv/cancel-query", body)
            .await?)
    }

    /// `GET /web/api/v2.1/dv/events` — [DEPRECATED] Get Events.
    ///
    /// Get all Deep Visibility events from a queryId. You can use this command
    /// to send a sub-query, a new query to run on these events. Get the ID from
    /// "init-query". See "Create Query and get QueryId". For complete
    /// documentation, see Query Syntax in the Knowledge Base
    /// (support.sentinelone.com) or the Console Help.
    pub async fn events(
        &self,
        query: &EventsQuery,
    ) -> Result<Paginated<DeepVisibilityEvent>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/dv/events", q).await?)
    }

    /// `POST /web/api/v2.1/dv/events/pq` — [DEPRECATED] Create a Power Query and Get QueryId.
    ///
    /// Start a Deep Visibility Power Query, get back status and potential
    /// results (ping afterwards using the queryId if query has not finished).
    pub async fn create_power_query(
        &self,
        body: &CreatePowerQueryBody,
    ) -> Result<Response<DeepVisibilityPowerQuery>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/dv/events/pq", body)
            .await?)
    }

    /// `GET /web/api/v2.1/dv/events/pq-ping` — [DEPRECATED] Ping a Power Query if results haven't been retrieved.
    ///
    /// Ping a Deep Visibility Power Query using the queryId if results have not
    /// returned from an initial Power Query or a previous ping. Use pq-ping soon
    /// after initiating a Power Query via the `/web/api/v2.1/dv/events/pq`
    /// endpoint. It is recommended to create a script that initiates the Power
    /// Query and then pings the query every few seconds to check the status and
    /// retrieve results when they are ready. If the ping is run after a
    /// significant delay, it may result in a 503 Service Unavailable response.
    /// This indicates that the results are no longer available due to cache
    /// eviction.
    pub async fn ping_power_query(
        &self,
        query: &PingPowerQuery,
    ) -> Result<Response<DeepVisibilityPowerQuery>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/dv/events/pq-ping", q)
            .await?)
    }

    /// `GET /web/api/v2.1/dv/events/{event_type}` — [DEPRECATED] Get Events By Type.
    ///
    /// Get Deep Visibility results from the query that matches the given event
    /// type. Valid values for Event Type include: Process Exit, Process
    /// Modification, Process Creation, Duplicate Process Handle, Duplicate
    /// Thread Handle, Open Remote Process Handle, Remote Thread Creation, Remote
    /// Process Termination, Command Script, IP Connect, IP Listen, File
    /// Modification, File Creation, File Scan, File Deletion, File Rename, Pre
    /// Execution Detection, Login, Logout, GET, OPTIONS, POST, PUT, DELETE,
    /// CONNECT, HEAD, DNS Resolved, DNS Unresolved, Task Register, Task Update,
    /// Task Start, Task Trigger, Task Delete, Registry Key Create, Registry Key
    /// Rename, Registry Key Delete, Registry Key Export, Registry Key Security
    /// Changed, Registry Key Import, Registry Value Modified, Registry Value
    /// Create, Registry Value Delete, Behavioral Indicators, Module Load.
    pub async fn events_by_type(
        &self,
        event_type: impl Into<String>,
        query: &EventsQuery,
    ) -> Result<Paginated<DeepVisibilityEvent>, Error> {
        let path = format!("/web/api/v2.1/dv/events/{}", event_type.into());
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `GET /web/api/v2.1/dv/fetch-file` — Download source process file.
    ///
    /// Download the source process file associated with a Deep Visibility event.
    pub async fn fetch_file(
        &self,
        query: &FetchFileQuery,
    ) -> Result<Response<DeepVisibilityFileDownloadLink>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/dv/fetch-file", q)
            .await?)
    }

    /// `POST /web/api/v2.1/dv/init-query` — [DEPRECATED] Create Query and Get QueryId.
    ///
    /// Start a Deep Visibility Query and get the queryId. You can use the
    /// queryId for other commands, such as Get Events and Get Query Status. For
    /// complete query syntax, see Query Syntax in the Knowledge Base
    /// (community.sentinelone.com) or the Console Help. Only SentinelOne Deep
    /// Visibility Query Language (S1QL 1.0) syntax is supported. SentinelOne
    /// Deep Visibility extends the ActiveEDR capabilities, with full visibility
    /// into endpoint data and threat hunting. Its kernel-based monitoring
    /// searches across endpoints for all indicators of compromise (IOC).
    /// Note: From Management version Rio (February 2022) the default of
    /// "isVerbose" is "false" instead of "true". Deep Visibility requires
    /// Complete SKU.
    pub async fn init_query(
        &self,
        body: &InitQueryBody,
    ) -> Result<Response<DeepVisibilityQueryId>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/dv/init-query", body)
            .await?)
    }

    /// `GET /web/api/v2.1/dv/process-state` — [DEPRECATED] Get Process State.
    ///
    /// Get details of all Deep Visibility processes from a queryId. To get the
    /// ID from "init-query". See "Create Query and get QueryId".
    ///
    /// The spec defines no typed `200` schema for this endpoint, so the rows are
    /// returned as freeform JSON.
    pub async fn process_state(
        &self,
        query: &ProcessStateQuery,
    ) -> Result<Paginated<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/dv/process-state", q)
            .await?)
    }

    /// `GET /web/api/v2.1/dv/query-status` — [DEPRECATED] Get Query Status.
    ///
    /// Get the status of a Deep Visibility Query. When the status is FINISHED,
    /// you can get the results with the queryId in "Get Events". Deep Visibility
    /// requires Complete SKU. Rate limit: 1 call per second for each different
    /// user token. responseState can return these values: EMPTY_RESULTS,
    /// EVENTS_RUNNING, FAILED, FAILED_CLIENT, FINISHED, PLANNING,
    /// PROCESS_RUNNING, QUERY_CANCEL, QUERY_EXPIRED, QUERY_NOT_FOUND,
    /// QUERY_RUNNING, RUNNING, TIMED_OUT.
    pub async fn query_status(
        &self,
        query: &QueryStatusQuery,
    ) -> Result<Response<DeepVisibilityQueryStatus>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/dv/query-status", q)
            .await?)
    }
}
