//! Models for the `Graph Query Management` tag.

use serde::Deserialize;

/// A saved/shared/suggested graph query as returned by the query list endpoint.
///
/// All fields are optional: the spec marks none of them as required on
/// `QueryManagerResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryManagerResponse {
    /// Query ID. Example: `"225494730938493804"`. Optional/nullable.
    pub id: Option<String>,
    /// Name. Optional/nullable.
    pub name: Option<String>,
    /// Scope path. Optional/nullable.
    pub scope_path: Option<String>,
    /// Stored query. Optional/nullable.
    pub stored_query: Option<String>,
    /// Asset type ids. Optional/nullable.
    pub asset_type_ids: Option<Vec<String>>,
    /// Shared. Optional/nullable.
    pub shared: Option<bool>,
    /// Query description. Optional/nullable.
    pub query_description: Option<String>,
}

/// A recently executed graph query as returned by the recent-queries endpoint.
///
/// All fields are optional: the spec marks none of them as required on
/// `RecentQueriesResponse`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentQueriesResponse {
    /// Query ID. Example: `"225494730938493804"`. Optional/nullable.
    pub id: Option<String>,
    /// Name. Optional/nullable.
    pub name: Option<String>,
    /// Scope path. Optional/nullable.
    pub scope_path: Option<String>,
    /// Last execution time (ISO-8601 date-time string). Optional/nullable.
    pub last_execution_time: Option<String>,
    /// Stored query. Optional/nullable.
    pub stored_query: Option<String>,
}

/// Response data for saving a graph query.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryManagementSaveResponse {
    /// Query id of the newly saved query. Optional/nullable.
    pub query_id: Option<String>,
}

/// Counts of graph queries grouped by type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryTypeCountsResponse {
    /// Number of shared queries. Optional/nullable.
    pub shared: Option<i64>,
    /// Number of suggested queries. Optional/nullable.
    pub suggested: Option<i64>,
    /// Number of saved queries. Optional/nullable.
    pub saved: Option<i64>,
}

/// Generic update/delete acknowledgement for a graph query.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryManagementUpdateResponse {
    /// Simple response message denoting action success. Optional/nullable.
    pub message: Option<String>,
}
