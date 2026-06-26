use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::cloud_resources::CloudResourcesResponse;
use crate::pagination::Response;

/// `Cloud Resources` tag.
///
/// Cloud Resources Data.
pub struct CloudResourcesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/cloudnative/cloud-rogues`.
///
/// Every field is optional. Array params are serialized comma-joined (set them
/// with the iterator-taking builder methods). `sortBy`/`sortOrder` are enums;
/// allowed values are documented on each field and stored as `String` for
/// forward-compat.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudRoguesQuery {
    /// Filter by cloud account (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// The column to sort the results by. Example: `"id"`. Optional.
    ///
    /// Enum (documented for forward-compat; stored as `String`):
    /// `id`, `createdTime`, `resourceType`, `name`, `region`,
    /// `virtualNetworkId`, `imageId`, `osType`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Free-text filter by resource name (supports multiple values).
    /// Comma-joined array. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// List of Account IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Free-text filter by network id (supports multiple values).
    /// Comma-joined array. Optional.
    #[serde(
        rename = "virtualNetworkId__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_network_id_contains: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: `"YWdlbnRfaWQ6NTgwMjkzODE="`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Filter by cloud provider name (supports multiple values).
    /// Comma-joined array. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_name: Option<String>,
    /// Filter by region (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Sort direction. Example: `"asc"`. Optional.
    ///
    /// Enum (documented for forward-compat; stored as `String`): `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Free-text filter by image (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(rename = "imageId__contains", skip_serializing_if = "Option::is_none")]
    pub image_id_contains: Option<String>,
    /// Included OS types. Comma-joined array. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Free-text filter by region (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// Free-text filter by tags (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(
        rename = "concatenatedTags__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub concatenated_tags_contains: Option<String>,
    /// Free-text filter by id (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// List of Site IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// If `true`, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Free-text filter by cloud account id (supports multiple values).
    /// Comma-joined array. Optional.
    #[serde(
        rename = "cloudProviderAccountId__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Limit number of returned items (1-1000). Example: `"10"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// If `true`, only total number of items will be returned, without any of
    /// the actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Free-text filter by cloud account (supports multiple values).
    /// Comma-joined array. Optional.
    #[serde(
        rename = "cloudProviderAccountName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_account_name_contains: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `"150"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
}

impl CloudRoguesQuery {
    /// Filter by cloud account (supports multiple values).
    pub fn cloud_provider_account_name<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(join_csv(values));
        self
    }
    /// The column to sort the results by. Allowed: `id`, `createdTime`,
    /// `resourceType`, `name`, `region`, `virtualNetworkId`, `imageId`,
    /// `osType`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Free-text filter by resource name (supports multiple values).
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(values));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// Free-text filter by network id (supports multiple values).
    pub fn virtual_network_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.virtual_network_id_contains = Some(join_csv(values));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// Filter by cloud provider name (supports multiple values).
    pub fn cloud_provider_name<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_name = Some(join_csv(values));
        self
    }
    /// Filter by region (supports multiple values).
    pub fn region<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(values));
        self
    }
    /// Sort direction. Allowed: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Free-text filter by image (supports multiple values).
    pub fn image_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_id_contains = Some(join_csv(values));
        self
    }
    /// Included OS types.
    pub fn os_types<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(values));
        self
    }
    /// Free-text filter by region (supports multiple values).
    pub fn region_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by tags (supports multiple values).
    pub fn concatenated_tags_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.concatenated_tags_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by id (supports multiple values).
    pub fn id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(values));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// If `true`, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Free-text filter by cloud account id (supports multiple values).
    pub fn cloud_provider_account_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(join_csv(values));
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// If `true`, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Free-text filter by cloud account (supports multiple values).
    pub fn cloud_provider_account_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(join_csv(values));
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
}

/// Query params for `GET /web/api/v2.1/cloudnative/cloud-rogues/export`.
///
/// Mirrors [`CloudRoguesQuery`] but the `sortBy` enum is restricted to
/// `id`/`createdTime` and an `exportFormat` param is added. Every field is
/// optional; array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudRoguesExportQuery {
    /// Filter by cloud account (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// The column to sort the results by. Example: `"id"`. Optional.
    ///
    /// Enum (documented for forward-compat; stored as `String`):
    /// `id`, `createdTime`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Free-text filter by resource name (supports multiple values).
    /// Comma-joined array. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// List of Account IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Free-text filter by network id (supports multiple values).
    /// Comma-joined array. Optional.
    #[serde(
        rename = "virtualNetworkId__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_network_id_contains: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: `"YWdlbnRfaWQ6NTgwMjkzODE="`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Filter by cloud provider name (supports multiple values).
    /// Comma-joined array. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_name: Option<String>,
    /// Filter by region (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Sort direction. Example: `"asc"`. Optional.
    ///
    /// Enum (documented for forward-compat; stored as `String`): `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Free-text filter by image (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(rename = "imageId__contains", skip_serializing_if = "Option::is_none")]
    pub image_id_contains: Option<String>,
    /// Included OS types. Comma-joined array. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Free-text filter by region (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// Free-text filter by tags (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(
        rename = "concatenatedTags__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub concatenated_tags_contains: Option<String>,
    /// Free-text filter by id (supports multiple values). Comma-joined array.
    /// Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// List of Site IDs to filter by.
    /// Example: `"225494730938493804,225494730938493915"`. Comma-joined array.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// If `true`, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Free-text filter by cloud account id (supports multiple values).
    /// Comma-joined array. Optional.
    #[serde(
        rename = "cloudProviderAccountId__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Limit number of returned items (1-1000). Example: `"10"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// If `true`, only total number of items will be returned, without any of
    /// the actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Export format. Example: `"csv"`. Optional.
    ///
    /// Enum (documented for forward-compat; stored as `String`): `csv`, `json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<String>,
    /// Free-text filter by cloud account (supports multiple values).
    /// Comma-joined array. Optional.
    #[serde(
        rename = "cloudProviderAccountName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_account_name_contains: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `"150"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
}

impl CloudRoguesExportQuery {
    /// Filter by cloud account (supports multiple values).
    pub fn cloud_provider_account_name<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(join_csv(values));
        self
    }
    /// The column to sort the results by. Allowed: `id`, `createdTime`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Free-text filter by resource name (supports multiple values).
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(values));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// Free-text filter by network id (supports multiple values).
    pub fn virtual_network_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.virtual_network_id_contains = Some(join_csv(values));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// Filter by cloud provider name (supports multiple values).
    pub fn cloud_provider_name<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_name = Some(join_csv(values));
        self
    }
    /// Filter by region (supports multiple values).
    pub fn region<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(values));
        self
    }
    /// Sort direction. Allowed: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Free-text filter by image (supports multiple values).
    pub fn image_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_id_contains = Some(join_csv(values));
        self
    }
    /// Included OS types.
    pub fn os_types<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(values));
        self
    }
    /// Free-text filter by region (supports multiple values).
    pub fn region_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by tags (supports multiple values).
    pub fn concatenated_tags_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.concatenated_tags_contains = Some(join_csv(values));
        self
    }
    /// Free-text filter by id (supports multiple values).
    pub fn id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(values));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// If `true`, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Free-text filter by cloud account id (supports multiple values).
    pub fn cloud_provider_account_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(join_csv(values));
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// If `true`, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Export format. Allowed: `csv`, `json`.
    pub fn export_format(mut self, v: impl Into<String>) -> Self {
        self.export_format = Some(v.into());
        self
    }
    /// Free-text filter by cloud account (supports multiple values).
    pub fn cloud_provider_account_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(join_csv(values));
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
}

/// Join an iterator of stringy values into a comma-separated query value.
fn join_csv<I, S>(values: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    values
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

impl CloudResourcesService<'_> {
    /// `GET /web/api/v2.1/cloudnative/cloud-rogues` — Get cloud rogue resources.
    ///
    /// Returns the cloud rogue resources for the given filter.
    ///
    /// Note: this endpoint uses a non-standard envelope (`data` is an object
    /// containing a `resources` array, with `pagination` as a sibling of
    /// `data`), so it returns a [`Response<CloudResourcesResponse>`] rather than
    /// the generic `Paginated<T>`.
    pub async fn list(
        &self,
        query: &CloudRoguesQuery,
    ) -> Result<Response<CloudResourcesResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cloudnative/cloud-rogues", q)
            .await?)
    }

    /// `GET /web/api/v2.1/cloudnative/cloud-rogues/export` — Export cloud rogue
    /// resources to csv (default) or json.
    ///
    /// Returns the results for the given cloud rogues filter in a csv (default)
    /// or json format.
    ///
    /// Note: the spec declares no response schema for this endpoint (it streams
    /// a file). The raw body is returned as `serde_json::Value` for
    /// forward-compat.
    pub async fn export(
        &self,
        query: &CloudRoguesExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cloudnative/cloud-rogues/export", q)
            .await?)
    }
}
