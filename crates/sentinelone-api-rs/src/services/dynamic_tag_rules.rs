use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::dynamic_tag_rules::{DeleteTagRuleResponse, InventoryResponse, TagRule};
use crate::pagination::{Paginated, Response};

/// `Dynamic tag rules` tag.
///
/// API for managing dynamic tag rules.
pub struct DynamicTagRulesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `DELETE /web/api/v2.1/xdr/assets/tags/rules`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTagRulesQuery {
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The list of tag rule ID identifiers to be removed. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl DeleteTagRulesQuery {
    /// List of Account IDs to filter by. Optional.
    pub fn account_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(items));
        self
    }
    /// List of Site IDs to filter by. Optional.
    pub fn site_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(items));
        self
    }
    /// The list of tag rule ID identifiers to be removed. Optional.
    pub fn ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(items));
        self
    }
    /// List of Group IDs to filter by. Optional.
    pub fn group_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(items));
        self
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/tags/rules`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTagRulesQuery {
    /// Filter by created by emails of the tag rules. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_email: Option<String>,
    /// The list of tag identifiers that the rule is associated with. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Negative filter by created by emails of the tag rules. Optional.
    #[serde(rename = "createdByEmail__nin", skip_serializing_if = "Option::is_none")]
    pub created_by_email_nin: Option<String>,
    /// Status. Allowed values: `enabled`, `disabled`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by names of the tag rules. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Negative filter by updated by emails of the tag rules. Optional.
    #[serde(rename = "updatedByEmail__nin", skip_serializing_if = "Option::is_none")]
    pub updated_by_email_nin: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Filter by description of the tag rules. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Negative filter by names of the tag rules. Optional.
    #[serde(rename = "name__nin", skip_serializing_if = "Option::is_none")]
    pub name_nin: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Negative filter by descriptions of the tag rules. Optional.
    #[serde(rename = "description__nin", skip_serializing_if = "Option::is_none")]
    pub description_nin: Option<String>,
    /// Filter by updated by emails of the tag rules. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by_email: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The list of tag rule ID identifiers. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `status`, `createdBy`, `createdByEmail`, `createdAt`, `updatedBy`,
    /// `updatedByEmail`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListTagRulesQuery {
    /// Filter by created by emails of the tag rules. Optional.
    pub fn created_by_email<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.created_by_email = Some(join_csv(items));
        self
    }
    /// The list of tag identifiers that the rule is associated with. Optional.
    pub fn tag_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tag_ids = Some(join_csv(items));
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Negative filter by created by emails of the tag rules. Optional.
    pub fn created_by_email_nin<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.created_by_email_nin = Some(join_csv(items));
        self
    }
    /// Status. Allowed values: `enabled`, `disabled`. Optional.
    pub fn status(mut self, v: impl Into<String>) -> Self {
        self.status = Some(v.into());
        self
    }
    /// Filter by names of the tag rules. Optional.
    pub fn name<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name = Some(join_csv(items));
        self
    }
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Negative filter by updated by emails of the tag rules. Optional.
    pub fn updated_by_email_nin<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.updated_by_email_nin = Some(join_csv(items));
        self
    }
    /// List of Account IDs to filter by. Optional.
    pub fn account_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(items));
        self
    }
    /// Limit number of returned items (1-1000). Optional.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Filter by description of the tag rules. Optional.
    pub fn description<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description = Some(join_csv(items));
        self
    }
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Skip first number of items (0-1000). Optional.
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Negative filter by names of the tag rules. Optional.
    pub fn name_nin<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_nin = Some(join_csv(items));
        self
    }
    /// List of Group IDs to filter by. Optional.
    pub fn group_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(items));
        self
    }
    /// Negative filter by descriptions of the tag rules. Optional.
    pub fn description_nin<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_nin = Some(join_csv(items));
        self
    }
    /// Filter by updated by emails of the tag rules. Optional.
    pub fn updated_by_email<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.updated_by_email = Some(join_csv(items));
        self
    }
    /// List of Site IDs to filter by. Optional.
    pub fn site_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(items));
        self
    }
    /// The list of tag rule ID identifiers. Optional.
    pub fn ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(items));
        self
    }
    /// Cursor position returned by the last request. Optional.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `name`,
    /// `status`, `createdBy`, `createdByEmail`, `createdAt`, `updatedBy`,
    /// `updatedByEmail`. Optional.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/tags/rules`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagRuleQuery {
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl CreateTagRuleQuery {
    /// List of Account IDs to filter by. Optional.
    pub fn account_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(items));
        self
    }
    /// List of Site IDs to filter by. Optional.
    pub fn site_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(items));
        self
    }
    /// List of Group IDs to filter by. Optional.
    pub fn group_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(items));
        self
    }
}

/// Query params for `PUT /web/api/v2.1/xdr/assets/tags/rules`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagRuleQuery {
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl UpdateTagRuleQuery {
    /// List of Account IDs to filter by. Optional.
    pub fn account_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(items));
        self
    }
    /// List of Site IDs to filter by. Optional.
    pub fn site_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(items));
        self
    }
    /// List of Group IDs to filter by. Optional.
    pub fn group_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(items));
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/tags/rules/test`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TestTagRuleQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `s1UpdatedAt`,
    /// `name`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl TestTagRuleQuery {
    /// Skip first number of items (0-1000). Optional.
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// List of Group IDs to filter by. Optional.
    pub fn group_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(items));
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Account IDs to filter by. Optional.
    pub fn account_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(items));
        self
    }
    /// Limit number of returned items (1-1000). Optional.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// List of Site IDs to filter by. Optional.
    pub fn site_ids<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(items));
        self
    }
    /// Cursor position returned by the last request. Optional.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `s1UpdatedAt`,
    /// `name`. Optional.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
}

impl DynamicTagRulesService<'_> {
    /// `DELETE /web/api/v2.1/xdr/assets/tags/rules` — Delete tag rules.
    ///
    /// Delete tag rules.
    ///
    /// Request body is the freeform `v2_1.inventory.tags.rules.schemas_TagRuleSchema`
    /// payload; pass [`serde_json::Value`] to match the spec exactly.
    pub async fn delete(
        &self,
        query: &DeleteTagRulesQuery,
        body: &serde_json::Value,
    ) -> Result<Response<DeleteTagRuleResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Response<DeleteTagRuleResponse>>(
                Method::DELETE,
                "/web/api/v2.1/xdr/assets/tags/rules",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/tags/rules` — Get all tags rules.
    ///
    /// Get all tags rules.
    pub async fn list(&self, query: &ListTagRulesQuery) -> Result<Paginated<TagRule>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/tags/rules", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/tags/rules` — Create new tag rule.
    ///
    /// Create new tag rule.
    ///
    /// Request body is the `v2_1.inventory.tags.rules.schemas_TagRuleSchema`
    /// payload (requires `name` and `conditions`); pass [`serde_json::Value`]
    /// to match the spec exactly. The response is the created [`TagRule`]
    /// returned directly (it is not wrapped in a `data` envelope).
    pub async fn create(
        &self,
        query: &CreateTagRuleQuery,
        body: &serde_json::Value,
    ) -> Result<TagRule, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, TagRule>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/tags/rules",
                q,
                Some(body),
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/xdr/assets/tags/rules` — Update tag rule.
    ///
    /// Update tag rule.
    ///
    /// Request body is the `v2_1.inventory.tags.rules.schemas_TagRuleSchema`
    /// payload (requires `name` and `conditions`); pass [`serde_json::Value`]
    /// to match the spec exactly. The response is the updated [`TagRule`]
    /// returned directly (it is not wrapped in a `data` envelope).
    pub async fn update(
        &self,
        query: &UpdateTagRuleQuery,
        body: &serde_json::Value,
    ) -> Result<TagRule, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, TagRule>(
                Method::PUT,
                "/web/api/v2.1/xdr/assets/tags/rules",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/tags/rules/test` — Check how many assets
    /// this tag rule matches.
    ///
    /// Check how many assets this tag rule matches.
    ///
    /// Request body is the `v2_1.inventory.tags.rules.schemas_TagRuleSchema`
    /// payload (requires `name` and `conditions`); pass [`serde_json::Value`]
    /// to match the spec exactly.
    pub async fn test(
        &self,
        query: &TestTagRuleQuery,
        body: &serde_json::Value,
    ) -> Result<Paginated<InventoryResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<serde_json::Value, Paginated<InventoryResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/tags/rules/test",
                q,
                Some(body),
            )
            .await?)
    }
}

/// Join an iterator of string-like items into a comma-separated value, as the
/// SentinelOne API expects for array query params.
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
