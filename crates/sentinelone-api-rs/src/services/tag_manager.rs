//! `Tag Manager` tag — Tag Manager operations.
//!
//! Create, edit and delete endpoint tags. All three endpoints take a typed
//! request body and return the standard SentinelOne `{ data, errors }`
//! envelope ([`crate::pagination::Response`]).

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::tag_manager::*;
use crate::pagination::Response;

/// `Tag Manager` tag — Tag Manager operations.
pub struct TagManagerService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Filter for the delete (`DELETE /web/api/v2.1/tag-manager`) request body
/// (`TagsDeleteSchema.filter`). All fields are optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTagsFilter {
    /// List of Group IDs to filter by. (min 1, max 500 items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by. (min 1, max 500 items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by. (min 1, max 500 items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of tag IDs. (max 5000 items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    /// List of tag IDs to exclude. (max 5000 items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids_excluded: Option<Vec<String>>,
    /// Free text search on fields key, value, description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Return tags from children scope levels. (default: `false`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// Return tags from parent scope levels. (default: `false`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
}

impl DeleteTagsFilter {
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// List of tag IDs.
    pub fn tag_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tag_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// List of tag IDs to exclude.
    pub fn tag_ids_excluded<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tag_ids_excluded = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Free text search on fields key, value, description.
    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query = Some(q.into());
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Return tags from children scope levels.
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
    /// Return tags from parent scope levels.
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
}

/// Request body for the delete (`DELETE /web/api/v2.1/tag-manager`) endpoint
/// (`TagsDeleteSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTagsBody {
    /// Filter. Required.
    pub filter: DeleteTagsFilter,
}

impl DeleteTagsBody {
    /// Build a delete request body from a [`DeleteTagsFilter`].
    pub fn new(filter: DeleteTagsFilter) -> Self {
        Self { filter }
    }
    /// Set the filter.
    pub fn filter(mut self, filter: DeleteTagsFilter) -> Self {
        self.filter = filter;
        self
    }
}

/// Tag data for the create (`POST /web/api/v2.1/tag-manager`) request body
/// (`PostTagSchema.data`). `key`, `type` and `value` are required;
/// `description` is optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagData {
    /// Key. Required.
    pub key: String,
    /// Type. Required.
    #[serde(rename = "type")]
    pub tag_type: String,
    /// Value. Required.
    pub value: String,
    /// Description. Optional. (default: `""`)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateTagData {
    /// Construct tag data with the required `key`, `type` and `value`.
    pub fn new(
        key: impl Into<String>,
        tag_type: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            tag_type: tag_type.into(),
            value: value.into(),
            description: None,
        }
    }
    /// Description.
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
}

/// Filter for the create (`POST /web/api/v2.1/tag-manager`) request body
/// (`PostTagSchema.filter`). All fields are optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagFilter {
    /// List of Group IDs to filter by. (min 1, max 500 items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by. (min 1, max 500 items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by. (min 1, max 500 items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl CreateTagFilter {
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
}

/// Request body for the create (`POST /web/api/v2.1/tag-manager`) endpoint
/// (`PostTagSchema`). `filter` is required; `data` is optional in the schema
/// but carries the tag definition (`key`, `type`, `value`) when present.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagBody {
    /// Filter. Required.
    pub filter: CreateTagFilter,
    /// Data (the tag definition). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<CreateTagData>,
}

impl CreateTagBody {
    /// Build a create request body from a [`CreateTagFilter`].
    pub fn new(filter: CreateTagFilter) -> Self {
        Self { filter, data: None }
    }
    /// Set the filter.
    pub fn filter(mut self, filter: CreateTagFilter) -> Self {
        self.filter = filter;
        self
    }
    /// Set the tag data.
    pub fn data(mut self, data: CreateTagData) -> Self {
        self.data = Some(data);
        self
    }
}

/// Tag data for the edit (`PUT /web/api/v2.1/tag-manager/{tag_id}`) request
/// body (`PutTagSchema.data`). All fields are optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditTagData {
    /// Key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl EditTagData {
    /// Key.
    pub fn key(mut self, k: impl Into<String>) -> Self {
        self.key = Some(k.into());
        self
    }
    /// Value.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }
    /// Description.
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
}

/// Request body for the edit (`PUT /web/api/v2.1/tag-manager/{tag_id}`)
/// endpoint (`PutTagSchema`). `data` is required.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditTagBody {
    /// Data. Required.
    pub data: EditTagData,
}

impl EditTagBody {
    /// Build an edit request body from an [`EditTagData`].
    pub fn new(data: EditTagData) -> Self {
        Self { data }
    }
    /// Set the tag data.
    pub fn data(mut self, data: EditTagData) -> Self {
        self.data = data;
        self
    }
}

impl TagManagerService<'_> {
    /// `DELETE /web/api/v2.1/tag-manager` — Delete tags.
    ///
    /// Delete all tags that match the filters.
    ///
    /// Returns the number of affected entities via [`AffectedResult`].
    pub async fn delete(
        &self,
        body: &DeleteTagsBody,
    ) -> Result<Response<AffectedResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteTagsBody, Response<AffectedResult>>(
                Method::DELETE,
                "/web/api/v2.1/tag-manager",
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/tag-manager` — Create a new endpoint tag.
    ///
    /// Each tag must contain a type (endpoints) and key, Value is optional but
    /// recommended. A description is optional.
    pub async fn create(
        &self,
        body: &CreateTagBody,
    ) -> Result<Response<Tag>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/tag-manager", body)
            .await?)
    }

    /// `PUT /web/api/v2.1/tag-manager/{tag_id}` — Edit an existing tag.
    ///
    /// Change the key, value, or description of a tag.
    ///
    /// `tag_id` (path, required): Tag ID. You can get the ID from the Get
    /// Tag-Manager command. Example: `"225494730938493804"`.
    pub async fn edit(
        &self,
        tag_id: impl Into<String>,
        body: &EditTagBody,
    ) -> Result<Response<Tag>, Error> {
        let path = format!("/web/api/v2.1/tag-manager/{}", tag_id.into());
        Ok(self
            .client
            .http()
            .request_json::<EditTagBody, Response<Tag>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }
}
