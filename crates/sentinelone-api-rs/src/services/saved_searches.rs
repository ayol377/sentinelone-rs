use sentinelone_http::Method;
use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::saved_searches::{BatchOperationResponse, SavedSearch};

/// `Saved Searches` tag.
///
/// Saved Searches API enables persistence and retrieval of search URLs for use
/// in Event Search.
pub struct SavedSearchesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /sdl/v2/api/saved-searches`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
    /// Filter by search type. `PRIVATE` searches are visible only to you, while
    /// `SHARED` searches are visible to all users in your current scope.
    /// Defaults to `PRIVATE` if not specified.
    ///
    /// Optional. Enum (kept as `String` for forward-compat): `PRIVATE`,
    /// `SHARED`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ListQuery {
    /// Filter by search type. `PRIVATE` searches are visible only to you, while
    /// `SHARED` searches are visible to all users in your current scope.
    ///
    /// Allowed values: `PRIVATE`, `SHARED`.
    pub fn r#type(mut self, t: impl Into<String>) -> Self {
        self.r#type = Some(t.into());
        self
    }
}

/// Specification for creating or updating a saved search.
///
/// Item of [`UpsertBody::searches`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearchInput {
    /// Display name for the saved search. Must be unique within the same type
    /// and scope.
    ///
    /// Required. Max length 1000.
    pub name: String,
    /// Event Search URL path with query parameters. Should start with `/events`
    /// and include any query parameters such as `filter`, `startTime`,
    /// `endTime`, etc. The `filter` parameter should be URL-encoded. Example:
    /// `/events?filter=event.category%3D%27registry%27&startTime=1+hour`.
    ///
    /// Required. Max length 999999.
    pub url: String,
    /// Visibility type. `PRIVATE` searches are visible only to you, while
    /// `SHARED` searches are visible to all users in the same scope. Defaults to
    /// `PRIVATE` if not specified. Use the `S1-Scope` header to manage searches
    /// in different scopes.
    ///
    /// Optional. Enum (kept as `String` for forward-compat): `PRIVATE`,
    /// `SHARED`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl SavedSearchInput {
    /// Construct a new saved-search input from its required fields (`name`,
    /// `url`).
    pub fn new(name: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            r#type: None,
        }
    }

    /// Set the visibility type. Allowed values: `PRIVATE`, `SHARED`.
    pub fn r#type(mut self, t: impl Into<String>) -> Self {
        self.r#type = Some(t.into());
        self
    }
}

/// Request body for `PUT /sdl/v2/api/saved-searches`.
///
/// Request body for creating or updating multiple saved searches.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertBody {
    /// List of saved searches to create or update. Maximum 100 searches per
    /// request (minimum 1).
    ///
    /// Required.
    pub searches: Vec<SavedSearchInput>,
    /// How to handle searches with names that already exist in the target scope.
    /// `NEW_ONLY` (default) skips duplicates and marks them as `SKIPPED` in the
    /// response. `REPLACE` overwrites existing searches with the same name.
    /// `APPEND_SUFFIX` creates new searches by adding numeric suffixes like
    /// ` (2)`, ` (3)` to duplicate names.
    ///
    /// Optional. Enum (kept as `String` for forward-compat): `NEW_ONLY`,
    /// `REPLACE`, `APPEND_SUFFIX`. Defaults to `NEW_ONLY`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_strategy: Option<String>,
}

impl UpsertBody {
    /// Construct a new upsert body from its required field (`searches`).
    pub fn new(searches: impl IntoIterator<Item = SavedSearchInput>) -> Self {
        Self {
            searches: searches.into_iter().collect(),
            duplicate_strategy: None,
        }
    }

    /// Set the duplicate handling strategy. Allowed values: `NEW_ONLY`,
    /// `REPLACE`, `APPEND_SUFFIX`.
    pub fn duplicate_strategy(mut self, strategy: impl Into<String>) -> Self {
        self.duplicate_strategy = Some(strategy.into());
        self
    }
}

/// Identifies a saved search to delete by name and type.
///
/// Item of [`BatchDeleteBody::searches`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedSearchDeleteInput {
    /// Name of the saved search to delete. Must match exactly.
    ///
    /// Required.
    pub name: String,
    /// Visibility type of the search to delete. `PRIVATE` searches are visible
    /// only to you, while `SHARED` searches are visible to all users in the same
    /// scope. Defaults to `PRIVATE` if not specified.
    ///
    /// Optional. Enum (kept as `String` for forward-compat): `PRIVATE`,
    /// `SHARED`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl SavedSearchDeleteInput {
    /// Construct a new delete input from its required field (`name`).
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            r#type: None,
        }
    }

    /// Set the visibility type. Allowed values: `PRIVATE`, `SHARED`.
    pub fn r#type(mut self, t: impl Into<String>) -> Self {
        self.r#type = Some(t.into());
        self
    }
}

/// Request body for `POST /sdl/v2/api/saved-searches/batch-delete`.
///
/// Request body for deleting multiple saved searches.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchDeleteBody {
    /// List of saved searches to delete, identified by name and type. Maximum
    /// 100 searches per request (minimum 1).
    ///
    /// Required.
    pub searches: Vec<SavedSearchDeleteInput>,
}

impl BatchDeleteBody {
    /// Construct a new batch-delete body from its required field (`searches`).
    pub fn new(searches: impl IntoIterator<Item = SavedSearchDeleteInput>) -> Self {
        Self {
            searches: searches.into_iter().collect(),
        }
    }
}

impl SavedSearchesService<'_> {
    /// `GET /sdl/v2/api/saved-searches` — List saved searches.
    ///
    /// Retrieves all saved searches visible to the current user. Saved searches
    /// allow you to persist and quickly access frequently used search queries in
    /// Event Search. Use the `type` parameter to filter by `PRIVATE`
    /// (user-specific) or `SHARED` (visible to all users in the scope) searches.
    ///
    /// The response is a bare JSON array of saved searches (this endpoint does
    /// not use the standard `{ data, pagination }` envelope), so it is returned
    /// as a `Vec<SavedSearch>`.
    pub async fn list(&self, query: &ListQuery) -> Result<Vec<SavedSearch>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/sdl/v2/api/saved-searches", q).await?)
    }

    /// `PUT /sdl/v2/api/saved-searches` — Create or update saved searches.
    ///
    /// Create or update multiple saved searches in a single request. This
    /// endpoint allows you to save up to 100 searches at once. When a search
    /// with the same name already exists in the target scope, the
    /// `duplicateStrategy` parameter controls the behavior: `NEW_ONLY` (default)
    /// skips duplicates, `REPLACE` overwrites existing searches, or
    /// `APPEND_SUFFIX` creates new searches with numeric suffixes (e.g.,
    /// 'My Search (2)'). You can change the active scope using the `S1-Scope`
    /// header to manage `SHARED` searches in different scopes.
    ///
    /// Note: A `teamToken` query parameter will be automatically added to each
    /// search URL if it is not already present. This token ensures the searches
    /// are scoped correctly.
    ///
    /// The response is the `BatchOperationResponse` object directly (no
    /// `{ data }` envelope).
    pub async fn upsert(&self, body: &UpsertBody) -> Result<BatchOperationResponse, Error> {
        Ok(self
            .client
            .http()
            .request_json::<UpsertBody, BatchOperationResponse>(
                Method::PUT,
                "/sdl/v2/api/saved-searches",
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /sdl/v2/api/saved-searches/batch-delete` — Delete saved searches.
    ///
    /// Delete multiple saved searches in a single request. You can delete up to
    /// 100 searches at once by providing their names and types. Each search name
    /// must be unique within its type (`PRIVATE` or `SHARED`) and scope. Use the
    /// `S1-Scope` header to target `SHARED` searches in specific scopes.
    ///
    /// The response is the `BatchOperationResponse` object directly (no
    /// `{ data }` envelope).
    pub async fn batch_delete(
        &self,
        body: &BatchDeleteBody,
    ) -> Result<BatchOperationResponse, Error> {
        Ok(self
            .client
            .http()
            .post("/sdl/v2/api/saved-searches/batch-delete", body)
            .await?)
    }
}
