//! Service for the `Tags` tag.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::tags::{Tag, TagAffected};
use crate::pagination::{Paginated, Response};

/// `Tags` tag — Tags related operations.
pub struct TagsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn join_csv<I, S>(items: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    items
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

// ---------------------------------------------------------------------------
// Query types
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/tags` (Get Tags).
///
/// `type` is the only required parameter. Array params are serialized
/// comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
    /// Type in. Required. Example: "firewall". Allowed item values: `firewall`,
    /// `network-quarantine`, `device-inventory`. Serialized comma-joined.
    #[serde(rename = "type")]
    pub type_: String,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: "150". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: "id". Optional. Allowed
    /// values: `id`, `type`, `scope`, `query`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Optional. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by. Example:
    /// "225494730938493804,225494730938493915". Optional. Serialized
    /// comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Example:
    /// "225494730938493804,225494730938493915". Optional. Serialized
    /// comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Example:
    /// "225494730938493804,225494730938493915". Optional. Serialized
    /// comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of IDs to filter by. Example:
    /// "225494730938493804,225494730938493915". Optional. Serialized
    /// comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Free text search on tag name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return tags from given scope level. Example: "global". Optional. Allowed
    /// values: `global`, `group`, `account`, `site`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Free-text filter by tag name. Example: "tag_name,tag_na". Optional.
    /// Serialized comma-joined.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// If true returns all tags possible to inherit from parent scopes,
    /// otherwise returns all tags already inherited and tags from this scope.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_parents: Option<bool>,
    /// Returns tags of this specific kind. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// If true, all tags for requested filters will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_pagination: Option<bool>,
}

impl ListQuery {
    /// Create a [`ListQuery`] with the required `type` filter (comma-joined).
    ///
    /// Allowed item values: `firewall`, `network-quarantine`,
    /// `device-inventory`.
    pub fn new<I, S>(type_: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        Self {
            type_: join_csv(type_),
            ..Default::default()
        }
    }

    /// Type in. Allowed item values: `firewall`, `network-quarantine`,
    /// `device-inventory` (comma-joined).
    pub fn type_<I, S>(mut self, type_: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.type_ = join_csv(type_);
        self
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
    /// If true, only the total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, the total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort by. Allowed: `id`, `type`, `scope`, `query`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Account IDs to filter by (comma-joined).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Site IDs to filter by (comma-joined).
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by (comma-joined).
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// List of IDs to filter by (comma-joined).
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// Free text search on tag name.
    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query = Some(q.into());
        self
    }
    /// Return tags from given scope level. Allowed: `global`, `group`,
    /// `account`, `site`.
    pub fn scope(mut self, v: impl Into<String>) -> Self {
        self.scope = Some(v.into());
        self
    }
    /// Free-text filter by tag name (comma-joined).
    pub fn name_contains<I, S>(mut self, names: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(names));
        self
    }
    /// If true returns all tags possible to inherit from parent scopes.
    pub fn only_parents(mut self, v: bool) -> Self {
        self.only_parents = Some(v);
        self
    }
    /// Returns tags of this specific kind.
    pub fn kind(mut self, v: impl Into<String>) -> Self {
        self.kind = Some(v.into());
        self
    }
    /// If true, all tags for requested filters will be returned.
    pub fn disable_pagination(mut self, v: bool) -> Self {
        self.disable_pagination = Some(v);
        self
    }
}

// ---------------------------------------------------------------------------
// Body types
// ---------------------------------------------------------------------------

/// Body for `DELETE /web/api/v2.1/tags` (Delete Tags)
/// (`tags.schemas_TagDeleteSchema`).
#[derive(Debug, Clone, Serialize)]
pub struct DeleteBody {
    /// Filter. Required.
    pub filter: DeleteFilter,
}

/// `filter` object for [`DeleteBody`]. Only `type` is required.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFilter {
    /// Type in. Required. Allowed item values: `firewall`,
    /// `network-quarantine`, `device-inventory`.
    #[serde(rename = "type")]
    pub type_: Vec<String>,
    /// List of Account IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of IDs to filter by (max 5000 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Free text search on tag name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return tags from given scope level. Optional. Allowed values: `global`,
    /// `group`, `account`, `site`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Free-text filter by tag name (max 10 items, each min length 2). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<Vec<String>>,
    /// If true returns all tags possible to inherit from parent scopes,
    /// otherwise returns all tags already inherited and tags from this scope.
    /// Optional (defaults to false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_parents: Option<bool>,
    /// Returns tags of this specific kind. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl DeleteBody {
    /// Build a [`DeleteBody`] with the required `type` filter.
    ///
    /// Allowed item values: `firewall`, `network-quarantine`,
    /// `device-inventory`.
    pub fn new<I, S>(type_: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            filter: DeleteFilter {
                type_: type_.into_iter().map(Into::into).collect(),
                ..Default::default()
            },
        }
    }
}

/// Body for `POST /web/api/v2.1/tags` (Create Tags)
/// (`tags.schemas_PostTagSchema`). Both `data` and `filter` are required.
#[derive(Debug, Clone, Serialize)]
pub struct CreateBody {
    /// Data. Required.
    pub data: CreateBodyData,
    /// Filter. Required.
    pub filter: CreateBodyFilter,
}

/// `data` object for [`CreateBody`]. `name` and `type` are required.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBodyData {
    /// Name. Required (min length 2). Example: "My Tag".
    pub name: String,
    /// Type. Required. Allowed values: `firewall`, `network-quarantine`,
    /// `device-inventory`. Example: "firewall".
    #[serde(rename = "type")]
    pub type_: String,
    /// Id. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Description (max length 256). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Kind (max length 64). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

/// `filter` object for [`CreateBody`]. The spec marks no field required.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBodyFilter {
    /// List of Account IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl CreateBody {
    /// Build a [`CreateBody`] with the required `name`, `type` and `filter`.
    ///
    /// Allowed `type` values: `firewall`, `network-quarantine`,
    /// `device-inventory`.
    pub fn new(
        name: impl Into<String>,
        type_: impl Into<String>,
        filter: CreateBodyFilter,
    ) -> Self {
        Self {
            data: CreateBodyData {
                name: name.into(),
                type_: type_.into(),
                ..Default::default()
            },
            filter,
        }
    }
}

/// Body for `PUT /web/api/v2.1/tags/{tag_id}` (Edit Tag)
/// (`tags.schemas_PutTagSchema`). Only `data` is required.
#[derive(Debug, Clone, Serialize)]
pub struct EditBody {
    /// Data. Required.
    pub data: EditBodyData,
}

/// `data` object for [`EditBody`]. The spec marks no field required.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditBodyData {
    /// Name. Optional/nullable. Example: "My Tag".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Id. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Description (max length 256). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Kind (max length 64). Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
}

impl EditBody {
    /// Build an [`EditBody`] from the (optional) `data` fields.
    pub fn new(data: EditBodyData) -> Self {
        Self { data }
    }
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

impl TagsService<'_> {
    /// `DELETE /web/api/v2.1/tags` — Delete Tags.
    ///
    /// Delete tags by given filter.
    pub async fn delete_tags(
        &self,
        body: &DeleteBody,
    ) -> Result<Response<TagAffected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteBody, Response<TagAffected>>(
                Method::DELETE,
                "/web/api/v2.1/tags",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/tags` — Get Tags.
    ///
    /// Get tags.
    pub async fn list(&self, query: &ListQuery) -> Result<Paginated<Tag>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/tags", q).await?)
    }

    /// `POST /web/api/v2.1/tags` — Create Tags.
    ///
    /// Add tags to create user-defined logical groups.
    pub async fn create(&self, body: &CreateBody) -> Result<Response<Tag>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/tags", body).await?)
    }

    /// `DELETE /web/api/v2.1/tags/{tag_id}` — Delete Tag by ID.
    ///
    /// Delete tag by ID.
    ///
    /// `tag_id`: Rule ID. Example: "225494730938493804".
    pub async fn delete(
        &self,
        tag_id: impl Into<String>,
    ) -> Result<Response<TagAffected>, Error> {
        let path = format!("/web/api/v2.1/tags/{}", tag_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<TagAffected>>(Method::DELETE, &path, None, None)
            .await?)
    }

    /// `PUT /web/api/v2.1/tags/{tag_id}` — Edit Tag.
    ///
    /// Edit tag.
    ///
    /// `tag_id`: Rule ID. Example: "225494730938493804".
    pub async fn edit(
        &self,
        tag_id: impl Into<String>,
        body: &EditBody,
    ) -> Result<Response<Tag>, Error> {
        let path = format!("/web/api/v2.1/tags/{}", tag_id.into());
        Ok(self
            .client
            .http()
            .request_json::<EditBody, Response<Tag>>(Method::PUT, &path, None, Some(body))
            .await?)
    }
}
