use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::live_updates::LiveUpdate;
use crate::pagination::Paginated;

/// `Live Updates` tag.
///
/// Live updates related APIs.
pub struct LiveUpdatesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/content-updates-inventory`.
///
/// `agentId` is required by the API; construct via [`ContentUpdatesInventoryQuery::new`].
/// All other params are optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentUpdatesInventoryQuery {
    /// The ID of the Agent. Required. Example: `"225494730938493804"`.
    pub agent_id: String,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,
    /// use `cursor`. Optional. Example: `"150"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional. Example: `"10"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more than
    /// 1000 items. Optional. Example: `"YWdlbnRfaWQ6NTgwMjkzODE="`.
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
    /// The column to sort the results by. Optional.
    /// Allowed values: `appliedAt`, `agentId`, `assetFamilyType`, `version`, `displayName`.
    /// Example: `"id"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`. Example: `"asc"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl ContentUpdatesInventoryQuery {
    /// Construct a query with the required `agentId`.
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
            ..Default::default()
        }
    }
    /// Set `skip` — skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Set `limit` — limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Set `cursor` — cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// Set `countOnly` — return only the total number of items.
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }
    /// Set `skipCount` — skip calculating the total number of items.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }
    /// Set `sortBy` — column to sort by.
    /// Allowed values: `appliedAt`, `agentId`, `assetFamilyType`, `version`, `displayName`.
    pub fn sort_by(mut self, s: impl Into<String>) -> Self {
        self.sort_by = Some(s.into());
        self
    }
    /// Set `sortOrder` — sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, s: impl Into<String>) -> Self {
        self.sort_order = Some(s.into());
        self
    }
}

impl LiveUpdatesService<'_> {
    /// `GET /web/api/v2.1/content-updates-inventory` — Get Agent Merged Updates.
    ///
    /// Get Agent's merged updates.
    pub async fn list_content_updates_inventory(
        &self,
        query: &ContentUpdatesInventoryQuery,
    ) -> Result<Paginated<LiveUpdate>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/content-updates-inventory", q)
            .await?)
    }
}
