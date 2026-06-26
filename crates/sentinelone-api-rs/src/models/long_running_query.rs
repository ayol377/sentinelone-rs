//! Models for the `Long Running Query` tag.
//!
//! The asynchronous query API launches a query and then lets the caller poll
//! for results. Both the launch (`POST`) and poll (`GET`) endpoints return a
//! [`QueryResult`]. While a query is still processing, the `data` field is
//! `null`; once it completes, `data` holds the full result set (shape depends
//! on the launched `queryType`, so it is modeled as a free-form value).

use serde::Deserialize;

/// Provides information about current query status and results.
///
/// Returned by both `POST /sdl/v2/api/queries` (launch) and
/// `GET /sdl/v2/api/queries/{id}` (poll).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResult {
    /// The unique query identifier. Used in Poll requests to fetch query
    /// status and results.
    ///
    /// Optional/nullable (not in the schema `required` array) -> `Option`.
    pub id: Option<String>,

    /// Number of steps completed in this long running query.
    ///
    /// Optional/nullable -> `Option`.
    pub steps_completed: Option<i64>,

    /// Total number of steps to execute.
    ///
    /// Optional/nullable -> `Option`.
    pub steps_total: Option<i64>,

    /// Start time and end time for the query in nanoseconds.
    ///
    /// Optional/nullable -> `Option`.
    pub resolved_time_range: Option<QueryResultTimeRange>,

    /// Data object containing results of the query.
    ///
    /// `null` while the query is still processing; populated once the query
    /// completes (up to configured limits). The concrete shape depends on the
    /// launched `queryType` (`LOG` -> matches list, `PQ` -> `PlotResultData`
    /// or `TableResultData` depending on `resultType`, etc.), so it is exposed
    /// as a free-form JSON value. Optional/nullable -> `Option`.
    pub data: Option<serde_json::Value>,

    /// Cost of the query execution measured in CPU usage in nanoseconds.
    ///
    /// Optional/nullable -> `Option`.
    pub cpu_usage: Option<i64>,

    /// Error if the query was not executed.
    ///
    /// If not null, will always have a `message` key with the basic error
    /// message and an optional `details` field with further information. When
    /// the query execution was successful, this field is `null`.
    /// Optional/nullable -> `Option`.
    pub error: Option<QueryResultError>,
}

/// Start time and end time for the query in nanoseconds.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResultTimeRange {
    /// End of the resolved time range, in nanoseconds since epoch.
    ///
    /// Optional/nullable -> `Option`.
    pub end: Option<i64>,

    /// Start of the resolved time range, in nanoseconds since epoch.
    ///
    /// Optional/nullable -> `Option`.
    pub start: Option<i64>,
}

/// Error if the query was not executed.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryResultError {
    /// Basic error message.
    ///
    /// Optional/nullable -> `Option`.
    pub message: Option<String>,

    /// Additional error details (free-form object).
    ///
    /// Optional/nullable -> `Option`.
    pub details: Option<serde_json::Value>,
}
