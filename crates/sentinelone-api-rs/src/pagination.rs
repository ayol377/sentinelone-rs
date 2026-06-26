//! The SentinelOne response envelope.
//!
//! Every list endpoint wraps results as `{ data: [...], pagination, errors }`;
//! single-resource endpoints as `{ data: {...}, errors }`. Codegen strips these
//! and reuses [`Paginated`] / [`Response`] everywhere.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    /// Pass as `cursor` on the next request; `None` once the last page is reached.
    #[serde(rename = "nextCursor")]
    pub next_cursor: Option<String>,
    #[serde(rename = "totalItems")]
    pub total_items: i64,
}

/// List response envelope.
#[derive(Debug, Clone, Deserialize)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub pagination: Pagination,
    #[serde(default)]
    pub errors: Option<serde_json::Value>,
}

/// Single-resource response envelope.
#[derive(Debug, Clone, Deserialize)]
pub struct Response<T> {
    pub data: T,
    #[serde(default)]
    pub errors: Option<serde_json::Value>,
}
