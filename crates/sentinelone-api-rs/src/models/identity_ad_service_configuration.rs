//! Models for the `Identity AD Service - Configuration` tag.
//!
//! APIs for managing AD configuration and operations.
//!
//! Field nullability follows the swagger `2.1` spec: a field is a bare `T`
//! only when it is listed in the schema `required` array *and* not
//! `x-nullable`; otherwise it is `Option<T>` (default-null behaviour). Enum
//! fields are kept as `String` for forward-compatibility, with the allowed
//! values documented in the field doc comment.

use serde::Deserialize;

/// Generic SentinelOne REST response wrapper (`GenericRestResponse`).
///
/// The `data` payload is freeform/untyped in the spec, so it is exposed as a
/// raw [`serde_json::Value`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericRestResponse {
    /// Freeform response payload. `None` when absent.
    pub data: Option<serde_json::Value>,
}

/// Domain information returned by `GET .../api/domains` (`DomainInfo`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainInfo {
    /// Domain name. Optional/nullable.
    pub domain: Option<String>,
    /// Parent domain name. Optional/nullable.
    pub parent_domain: Option<String>,
    /// Whether this domain is a root domain. Optional/nullable.
    pub root: Option<bool>,
}
