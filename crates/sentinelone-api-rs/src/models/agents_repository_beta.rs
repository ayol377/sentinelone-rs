//! Models for the `Agents Repository (Beta)` tag.
//!
//! Source of truth: `swagger_2_1.json` definitions
//! `handlers.ListTokensResponse`, `handlers.TokenResponse` and
//! `handlers.Pagination`.
//!
//! Fidelity note: none of the fields on these definitions are listed in a
//! schema `required` array, so every field is modelled as `Option<T>`
//! (the spec's "default null" behaviour).

use serde::Deserialize;

/// An access token for the S1 Agent Artifacts Repository.
///
/// Spec definition: `handlers.TokenResponse`. Returned both as the element
/// type of [`ListTokensResponse::data`] and as the body of the create
/// (`POST`) endpoint. No field is `required`, so all are `Option`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    /// Created At timestamp of the token. Date/time string.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Token description.
    ///
    /// Optional/nullable -> `Option`.
    pub description: Option<String>,
    /// Access token ID. Integer.
    ///
    /// Optional/nullable -> `Option`.
    pub id: Option<i64>,
    /// Scope ID.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(rename = "scope_id")]
    pub scope_id: Option<String>,
    /// Scope level of the token.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(rename = "scope_level")]
    pub scope_level: Option<String>,
    /// Token name.
    ///
    /// Optional/nullable -> `Option`.
    pub title: Option<String>,
    /// Access token - seen only once.
    ///
    /// Optional/nullable -> `Option`.
    pub token: Option<String>,
    /// Username of the token.
    ///
    /// Optional/nullable -> `Option`.
    pub username: Option<String>,
}

/// Offset-based pagination metadata for the token list.
///
/// Spec definition: `handlers.Pagination`. Note this is a tag-local pagination
/// shape (`next_offset` / `total`) and is distinct from the cursor-based
/// [`crate::pagination::Pagination`] envelope used elsewhere. No field is
/// `required`, so all are `Option`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokensPagination {
    /// Offset to pass as `offset` on the next request. Integer.
    ///
    /// Optional/nullable -> `Option`.
    #[serde(rename = "next_offset")]
    pub next_offset: Option<i64>,
    /// Total number of tokens. Integer.
    ///
    /// Optional/nullable -> `Option`.
    pub total: Option<i64>,
}

/// Response body of `GET /web/api/v2.1/agent-artifacts/token`.
///
/// Spec definition: `handlers.ListTokensResponse`. This endpoint uses a
/// bespoke offset-based envelope (`{ data, pagination }`) rather than the
/// standard cursor-based [`crate::pagination::Paginated`], so it is modelled
/// verbatim here. No field is `required`, so all are `Option`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTokensResponse {
    /// List of existing tokens.
    ///
    /// Optional/nullable -> `Option`.
    pub data: Option<Vec<TokenResponse>>,
    /// Pagination information.
    ///
    /// Optional/nullable -> `Option`.
    pub pagination: Option<TokensPagination>,
}
