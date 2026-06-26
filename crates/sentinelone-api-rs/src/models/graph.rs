//! Models for the `Graph` tag (XDR Graph Explorer).
//!
//! Hand-written for strict 1:1 parity with `swagger_2_1.json`. Field optionality
//! follows the spec: a field is a bare `T` only when it is in the schema
//! `required` array and not `x-nullable`; otherwise it is `Option<T>`
//! (the "default null" behaviour).

use std::collections::HashMap;

use serde::Deserialize;

/// A single property attached to a graph node or link (`PropertyResponse`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphProperty {
    /// Property label. Required.
    pub label: String,
    /// Property name. Required.
    pub name: String,
    /// Property value. Freeform / untyped in the spec. Required.
    pub value: serde_json::Value,
}

/// A node in the queried graph (`Node`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNode {
    /// Node label. Required.
    pub label: String,
    /// Node type name. Required.
    pub type_name: String,
    /// Properties attached to the node. Optional/nullable.
    pub properties: Option<Vec<GraphProperty>>,
    /// Node type id. Optional/nullable.
    pub type_id: Option<String>,
}

/// A link (edge) between two nodes in the queried graph (`Link`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphLink {
    /// Id of the first node of the link. Required.
    pub id1: String,
    /// Id of the second node of the link. Required.
    pub id2: String,
    /// Link name. Required.
    pub name: String,
    /// Link label. Optional/nullable.
    pub label: Option<String>,
    /// Properties attached to the link. Optional/nullable.
    pub properties: Option<Vec<GraphProperty>>,
}

/// The graph query result entity (`QueryGraphResponse`), i.e. the `data`
/// member of the response envelope returned by every Graph endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryGraphResponse {
    /// Total available nodes. Required.
    pub total_available_nodes: i64,
    /// Whether the query timed out. Required.
    pub time_out: bool,
    /// Whether the result was capped by the max limit. Required.
    pub max_limit: bool,
    /// Continuation token for fetching the next page. Optional/nullable.
    pub continuation_token: Option<String>,
    /// Links (edges), keyed by link id. Optional/nullable.
    pub links: Option<HashMap<String, GraphLink>>,
    /// Nodes, keyed by node id. Optional/nullable.
    pub nodes: Option<HashMap<String, GraphNode>>,
}
