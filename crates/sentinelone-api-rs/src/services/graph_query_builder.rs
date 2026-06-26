use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::graph_query_builder::{
    AutoCompleteResponse, GetAvailableOptionMetadataSchemaV2, GraphFeatureToggleResponse,
    GraphQueryBuilderMetadata, OntologyRelations, TagAutoCompleteResponse,
};
use crate::pagination::Response;

/// `Graph Query Builder` tag.
///
/// Graph Query Builder metadata: fetch the initial query builder metadata
/// (categories / sub-categories / asset types), retrieve metadata for specific
/// asset types, resolve available relations, request field and tag
/// auto-complete suggestions, and read graph service feature toggles.
pub struct GraphQueryBuilderService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for the scope-only graph query builder metadata endpoints.
///
/// Used by both `GET /web/api/v2.1/xdr/graph-explorer/query/builder/metadata`
/// and `GET /web/api/v2.1/xdr/assets/query/builder/metadata`, as well as the
/// available-relations and graph-services-features endpoints.
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphScopeQuery {
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

impl GraphScopeQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for the field auto-complete endpoints.
///
/// Used by both
/// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/autocomplete` (deprecated)
/// and `GET /web/api/v2.1/xdr/graph-explorer/query/builder/autocomplete/v2`.
///
/// Array params are serialized comma-joined, as the API expects. The `key`
/// param is required by the API but kept `Option` here so the builder is
/// uniform; set it via [`AutoCompleteQuery::key`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteQuery {
    /// List of asset type ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Search term text. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Limit number of returned items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// List of subCategory ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_categories_ids: Option<String>,
    /// Search field key. Required by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// List of category ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_ids: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl AutoCompleteQuery {
    /// List of asset type ids.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// Search term text.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    /// Version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
    /// Limit number of returned items.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// List of subCategory ids.
    pub fn sub_categories_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_categories_ids = Some(join_csv(ids));
        self
    }
    /// Search field key (required by the API).
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    /// List of category ids.
    pub fn category_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.category_ids = Some(join_csv(ids));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/metadata/available-options/v2`.
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableOptionsQuery {
    /// List of asset type ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// List of subCategory ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_categories_ids: Option<String>,
    /// List of category ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_ids: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl AvailableOptionsQuery {
    /// List of asset type ids.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// Version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
    /// List of subCategory ids.
    pub fn sub_categories_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_categories_ids = Some(join_csv(ids));
        self
    }
    /// List of category ids.
    pub fn category_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.category_ids = Some(join_csv(ids));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for
/// `POST /web/api/v2.1/xdr/graph-explorer/query/builder/tag/autocomplete`.
///
/// Array params are serialized comma-joined, as the API expects. The `field`
/// param is required by the API but kept `Option` here so the builder is
/// uniform; set it via [`TagAutoCompleteQuery::field`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TagAutoCompleteQuery {
    /// List of asset type ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Search in keys or values. Required by the API.
    /// Allowed values: `key`, `value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Search term value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Limit number of returned items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// List of subCategory ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_categories_ids: Option<String>,
    /// List of category ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_ids: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl TagAutoCompleteQuery {
    /// List of asset type ids.
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// Search in keys or values (required by the API).
    /// Allowed values: `key`, `value`.
    pub fn field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }
    /// Version.
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
    /// Search term value.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    /// Limit number of returned items.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// List of subCategory ids.
    pub fn sub_categories_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_categories_ids = Some(join_csv(ids));
        self
    }
    /// List of category ids.
    pub fn category_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.category_ids = Some(join_csv(ids));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

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

impl GraphQueryBuilderService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/query/builder/metadata` — Get Graph Query
    /// Builder Initial Metadata.
    ///
    /// Get graph query builder initial metadata.
    pub async fn get_assets_metadata(
        &self,
        query: &GraphScopeQuery,
    ) -> Result<Response<GraphQueryBuilderMetadata>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/query/builder/metadata", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/metadata` — Get Graph
    /// Query Builder Initial Metadata.
    ///
    /// Get graph query builder initial metadata.
    pub async fn get_metadata(
        &self,
        query: &GraphScopeQuery,
    ) -> Result<Response<GraphQueryBuilderMetadata>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/graph-explorer/query/builder/metadata", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/autocomplete` — Auto
    /// Complete.
    ///
    /// This api is now deprecated use
    /// `/xdr/graph-explorer/query/builder/autocomplete/v2` instead.
    pub async fn autocomplete(
        &self,
        query: &AutoCompleteQuery,
    ) -> Result<Response<AutoCompleteResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/graph-explorer/query/builder/autocomplete",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/autocomplete/v2` —
    /// Auto Complete.
    ///
    /// Use this command to get values for other fields. When you send this
    /// command with input text and a field name, it returns auto-complete
    /// suggestions for the field.
    pub async fn autocomplete_v2(
        &self,
        query: &AutoCompleteQuery,
    ) -> Result<Response<AutoCompleteResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/graph-explorer/query/builder/autocomplete/v2",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/metadata/available-options/v2`
    /// — Get Query Builder metadata For Requested Resource Types.
    ///
    /// Get query builder metadata for requested cloud asset types.
    pub async fn available_options_v2(
        &self,
        query: &AvailableOptionsQuery,
    ) -> Result<Response<GetAvailableOptionMetadataSchemaV2>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/graph-explorer/query/builder/metadata/available-options/v2",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/graph-explorer/query/builder/metadata/available-relations`
    /// — Get the available relations.
    ///
    /// Get the available relations.
    pub async fn available_relations(
        &self,
        query: &GraphScopeQuery,
    ) -> Result<Response<Vec<OntologyRelations>>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/graph-explorer/query/builder/metadata/available-relations",
                q,
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/graph-explorer/query/builder/tag/autocomplete` —
    /// Tag Auto Complete.
    ///
    /// Use this command to get tag keys or values. When you send this command
    /// with input value and field name either `key` or `value`, it returns
    /// auto-complete suggestions for the field.
    ///
    /// All parameters are query parameters; the request carries no body.
    pub async fn tag_autocomplete(
        &self,
        query: &TagAutoCompleteQuery,
    ) -> Result<Response<TagAutoCompleteResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<TagAutoCompleteResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/graph-explorer/query/builder/tag/autocomplete",
                q,
                None,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/private/graph-services-features` — Get all of the
    /// feature toggles for graph services.
    ///
    /// Get all of the feature toggles for graph services.
    pub async fn graph_services_features(
        &self,
        query: &GraphScopeQuery,
    ) -> Result<Response<GraphFeatureToggleResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/private/graph-services-features", q)
            .await?)
    }
}
