//! `overview` tag — overview of resources.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::overview::InventoryOverview;
use crate::pagination::Response;

/// `overview` tag — Cloud Inventory resource overview operations.
pub struct OverviewService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Joins an iterator of string-likes by comma (the form array query params use).
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

/// Query params for `POST /web/api/v2.1/xdr/assets/overview`.
///
/// Every field is optional. Array filters are serialized as a single
/// comma-joined string (the form the API expects); use the builder methods,
/// which accept an iterator and join by comma.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceOverviewQuery {
    /// List of Account IDs to filter by (`accountIds`). Array param. Optional.
    /// (Spec: 1-500 items.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (`siteIds`). Array param. Optional.
    /// (Spec: 1-500 items.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by (`groupIds`). Array param. Optional.
    /// (Spec: 1-500 items.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl ResourceOverviewQuery {
    /// List of Account IDs to filter by (`accountIds`). Array param. Optional.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// List of Site IDs to filter by (`siteIds`). Array param. Optional.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// List of Group IDs to filter by (`groupIds`). Array param. Optional.
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(values));
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/overview`
/// (`InventoryOverviewJsonSchema` in the spec).
///
/// The spec marks no field as required, so every field is optional and is
/// omitted from the serialized JSON when `None`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceOverviewBody {
    /// Agent uuid (`agentUuid`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// Resource id (`resourceId`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl ResourceOverviewBody {
    /// Agent uuid (`agentUuid`). Optional.
    pub fn agent_uuid(mut self, v: impl Into<String>) -> Self {
        self.agent_uuid = Some(v.into());
        self
    }
    /// Resource id (`resourceId`). Optional.
    pub fn resource_id(mut self, v: impl Into<String>) -> Self {
        self.resource_id = Some(v.into());
        self
    }
}

impl OverviewService<'_> {
    /// **Cloud Inventory resource overview** — Get overview of a resource
    /// belonging to a category.
    ///
    /// Get overview of a resource belonging to a category.
    ///
    /// `POST /web/api/v2.1/xdr/assets/overview`
    pub async fn resource_overview(
        &self,
        query: &ResourceOverviewQuery,
        body: &ResourceOverviewBody,
    ) -> Result<Response<InventoryOverview>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let path = if qs.is_empty() {
            "/web/api/v2.1/xdr/assets/overview".to_owned()
        } else {
            format!("/web/api/v2.1/xdr/assets/overview?{qs}")
        };
        Ok(self.client.http().post(&path, body).await?)
    }
}
