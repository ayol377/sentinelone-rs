//! Models for the `Saved Searches` tag.
//!
//! Saved Searches API enables persistence and retrieval of search URLs for use
//! in Event Search.

use serde::Deserialize;

/// Represents a saved search with its position, name, URL, and visibility type.
///
/// Returned by `GET /sdl/v2/api/saved-searches`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearch {
    /// Zero-based position of the saved search in the list. This determines the
    /// display order.
    ///
    /// Required, non-nullable.
    pub index: i64,
    /// Display name of the saved search. Must be unique within the same type and
    /// scope.
    ///
    /// Required, non-nullable. Max length 1000.
    pub name: String,
    /// Event Search URL path with query parameters, typically starting with
    /// `/events`. This URL is used to navigate to the saved search.
    ///
    /// Required, non-nullable. Max length 999999.
    pub url: String,
    /// Visibility type. `PRIVATE` searches are visible only to you, while
    /// `SHARED` searches are visible to all users in the same scope. Use the
    /// `S1-Scope` header to manage searches in different scopes.
    ///
    /// Required, non-nullable. Enum (kept as `String` for forward-compat):
    /// `PRIVATE`, `SHARED`. Defaults to `PRIVATE`.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Operation result for a single saved search.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchOperationResult {
    /// Name of the search from the request.
    ///
    /// Required, non-nullable.
    pub search_name: String,
    /// Explanation for why the operation failed or was skipped. Only present when
    /// status is `FAILED` or `SKIPPED`.
    ///
    /// Optional / nullable.
    pub reason: Option<String>,
    /// Operation outcome. `SUCCESS` indicates the search was saved/deleted.
    /// `SKIPPED` indicates the search was not processed (e.g., duplicate with
    /// `NEW_ONLY` strategy). `FAILED` indicates an error occurred (see `reason`
    /// field for details).
    ///
    /// Required, non-nullable. Enum (kept as `String` for forward-compat):
    /// `SUCCESS`, `SKIPPED`, `FAILED`.
    pub status: String,
}

/// Response containing the result status for each saved search in the batch
/// operation.
///
/// Returned by `PUT /sdl/v2/api/saved-searches` and
/// `POST /sdl/v2/api/saved-searches/batch-delete`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOperationResponse {
    /// Individual result for each search in the request, in the same order as
    /// submitted.
    ///
    /// Required, non-nullable.
    pub results: Vec<SearchOperationResult>,
}
