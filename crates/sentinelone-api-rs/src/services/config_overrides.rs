use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::config_overrides::{ConfigOverride, ConfigOverrideAffected, ConfigOverrideSuccess};
use crate::pagination::{Paginated, Response};

/// `Config Overrides` tag.
///
/// Agents configuration override. There are different ways to override the
/// configuration of an Agent, and the priority of changes depends on the
/// endpoint OS and the version of the installed Agent.
pub struct ConfigOverridesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/config-override`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigOverridesQuery {
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
    /// If true, only total number of items will be returned, without any of
    /// the actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: "id". Optional.
    /// Allowed values: `id`, `createdAt`, `updatedAt`, `name`, `description`,
    /// `scope`, `osType`, `agentVersion`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Optional. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// List of ids to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of Agent IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<String>,
    /// Config Overrides created before this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Config Overrides created before or at this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Config Overrides created after this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Config Overrides created after or at this timestamp. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Date range for creation time (format: `<from_timestamp>-<to_timestamp>`,
    /// inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// Match name partially (substring). Optional.
    #[serde(rename = "name__like", skip_serializing_if = "Option::is_none")]
    pub name_like: Option<String>,
    /// Match description partially (substring). Optional.
    #[serde(rename = "description__like", skip_serializing_if = "Option::is_none")]
    pub description_like: Option<String>,
    /// Included OS types. Comma-joined. Example: "linux". Optional.
    /// Allowed item values: `linux`, `macos`, `windows_legacy`, `windows`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Included agent versions. Comma-joined. Example: "2.5.1.1320". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_versions: Option<String>,
    /// Version option. Example: "ALL". Optional. Allowed values: `ALL`,
    /// `SPECIFIC`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_option: Option<String>,
    /// Free text search on fields name, description, agent_version, os_type,
    /// config. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl GetConfigOverridesQuery {
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
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `createdAt`,
    /// `updatedAt`, `name`, `description`, `scope`, `osType`, `agentVersion`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
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
    pub fn tenant(mut self, b: bool) -> Self {
        self.tenant = Some(b);
        self
    }
    /// List of ids to filter by (comma-joined).
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// List of Agent IDs to filter by (comma-joined).
    pub fn agent_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ids = Some(join_csv(ids));
        self
    }
    /// Config Overrides created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Config Overrides created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Config Overrides created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Config Overrides created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Date range for creation time (`<from_timestamp>-<to_timestamp>`).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// Match name partially (substring).
    pub fn name_like(mut self, v: impl Into<String>) -> Self {
        self.name_like = Some(v.into());
        self
    }
    /// Match description partially (substring).
    pub fn description_like(mut self, v: impl Into<String>) -> Self {
        self.description_like = Some(v.into());
        self
    }
    /// Included OS types (comma-joined). Allowed item values: `linux`, `macos`,
    /// `windows_legacy`, `windows`.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// Included agent versions (comma-joined).
    pub fn agent_versions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions = Some(join_csv(v));
        self
    }
    /// Version option. Allowed values: `ALL`, `SPECIFIC`.
    pub fn version_option(mut self, v: impl Into<String>) -> Self {
        self.version_option = Some(v.into());
        self
    }
    /// Free text search on fields name, description, agent_version, os_type,
    /// config.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
}

/// Body for `DELETE /web/api/v2.1/config-override`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteConfigOverridesBody {
    /// Filter. Required (in `required`). Freeform filter object -> use
    /// [`serde_json::Value`] for forward-compat with the many filter fields
    /// (accountIds, siteIds, groupIds, tenant, ids, agentIds, createdAt__*,
    /// name__like, description__like, osTypes, agentVersions, versionOption,
    /// query).
    pub filter: serde_json::Value,
}

impl DeleteConfigOverridesBody {
    /// Construct a delete body from a freeform filter object.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { filter }
    }
}

/// Body for `POST /web/api/v2.1/config-override`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateConfigOverrideBody {
    /// Data. Required (in `required`). Freeform object -> use
    /// [`serde_json::Value`] for forward-compat. Required sub-fields: `config`,
    /// `name`, `osType`, `scope`. Optional sub-fields include `description`,
    /// `agentVersion`, `versionOption`, `account`, `site`, `group`.
    pub data: serde_json::Value,
    /// Filter. Optional/nullable -> `Option`. Freeform object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl CreateConfigOverrideBody {
    /// Construct a create body from a freeform `data` object.
    pub fn new(data: serde_json::Value) -> Self {
        Self { data, filter: None }
    }
    /// Set the optional freeform `filter` object.
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Body for `PUT /web/api/v2.1/config-override/{override_id}`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConfigOverrideBody {
    /// Data. Required (in `required`). Freeform object -> use
    /// [`serde_json::Value`] for forward-compat. Sub-fields include `name`,
    /// `description`, `osType`, `agentVersion`, `versionOption`, `config`,
    /// `scope`, `site`, `group`, `account`.
    pub data: serde_json::Value,
    /// Filter. Optional/nullable -> `Option`. Freeform object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
}

impl UpdateConfigOverrideBody {
    /// Construct an update body from a freeform `data` object.
    pub fn new(data: serde_json::Value) -> Self {
        Self { data, filter: None }
    }
    /// Set the optional freeform `filter` object.
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.filter = Some(filter);
        self
    }
}

/// Join an iterator of string-like values into a comma-separated string.
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

impl ConfigOverridesService<'_> {
    /// `GET /web/api/v2.1/config-override` — Get Config Overrides.
    ///
    /// There are different ways to override the configuration of an Agent, and
    /// the priority of changes depends on the endpoint OS and the version of
    /// the installed Agent. Use this command to see the configuration values
    /// that are changed for each Agent that matches the filter.
    pub async fn list(
        &self,
        query: &GetConfigOverridesQuery,
    ) -> Result<Paginated<ConfigOverride>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/config-override", q)
            .await?)
    }

    /// `POST /web/api/v2.1/config-override` — Create Config Override.
    ///
    /// Override the configuration of Agents that match the filter. Best
    /// practice: Run "support-actions/config" to get the complete syntax. This
    /// command requires a Global user or Support.
    pub async fn create(
        &self,
        body: &CreateConfigOverrideBody,
    ) -> Result<Response<ConfigOverride>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/config-override", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/config-override` — Delete Config Overrides.
    ///
    /// Delete overrides value. To get the required IDs, run "config-override".
    pub async fn delete(
        &self,
        body: &DeleteConfigOverridesBody,
    ) -> Result<Response<ConfigOverrideAffected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteConfigOverridesBody, Response<ConfigOverrideAffected>>(
                Method::DELETE,
                "/web/api/v2.1/config-override",
                None,
                Some(body),
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/config-override/{override_id}` — Delete Config
    /// Override.
    ///
    /// Delete an override value. To get the required ID, run "config-override".
    ///
    /// `override_id`: Config override object ID. Example: "225494730938493804".
    pub async fn delete_one(
        &self,
        override_id: impl Into<String>,
    ) -> Result<Response<ConfigOverrideSuccess>, Error> {
        let path = format!("/web/api/v2.1/config-override/{}", override_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<ConfigOverrideSuccess>>(
                Method::DELETE,
                &path,
                None,
                None,
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/config-override/{override_id}` — Update Config
    /// Override.
    ///
    /// Use this command to change the value of one configuration value. To get
    /// the required ID, run "config-override".
    ///
    /// `override_id`: Config override object ID. Example: "225494730938493804".
    pub async fn update(
        &self,
        override_id: impl Into<String>,
        body: &UpdateConfigOverrideBody,
    ) -> Result<Response<ConfigOverride>, Error> {
        let path = format!("/web/api/v2.1/config-override/{}", override_id.into());
        Ok(self
            .client
            .http()
            .request_json::<UpdateConfigOverrideBody, Response<ConfigOverride>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }
}
