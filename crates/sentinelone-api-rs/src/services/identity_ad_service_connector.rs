//! `Identity AD Service - Connector` tag.
//!
//! APIs for managing AD connector information and operations.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::identity_ad_service_connector::RequestBodyWrapperAgentIdrStatusInput;
use crate::pagination::Response;

/// `Identity AD Service - Connector` tag.
///
/// APIs for managing AD connector information and operations.
pub struct IdentityAdServiceConnectorService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for
/// `GET /web/api/v2.1/identity/adservice/api/getCloudlinkConfiguration`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCloudlinkConfigurationQuery {
    /// List of account IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub account_ids: String,
    /// List of site IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub site_ids: String,
}

impl GetCloudlinkConfigurationQuery {
    /// Construct the required query params.
    ///
    /// `account_ids` / `site_ids` are comma-separated lists of IDs.
    pub fn new(account_ids: impl Into<String>, site_ids: impl Into<String>) -> Self {
        Self { account_ids: account_ids.into(), site_ids: site_ids.into() }
    }
    /// Set the comma-separated list of account IDs.
    pub fn account_ids(mut self, v: impl Into<String>) -> Self {
        self.account_ids = v.into();
        self
    }
    /// Set the comma-separated list of site IDs.
    pub fn site_ids(mut self, v: impl Into<String>) -> Self {
        self.site_ids = v.into();
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/identity/adservice/api/getCloudlinkConfigurationUuid`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCloudlinkConfigurationUuidQuery {
    /// List of account IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub account_ids: String,
    /// List of site IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub site_ids: String,
}

impl GetCloudlinkConfigurationUuidQuery {
    /// Construct the required query params.
    ///
    /// `account_ids` / `site_ids` are comma-separated lists of IDs.
    pub fn new(account_ids: impl Into<String>, site_ids: impl Into<String>) -> Self {
        Self { account_ids: account_ids.into(), site_ids: site_ids.into() }
    }
    /// Set the comma-separated list of account IDs.
    pub fn account_ids(mut self, v: impl Into<String>) -> Self {
        self.account_ids = v.into();
        self
    }
    /// Set the comma-separated list of site IDs.
    pub fn site_ids(mut self, v: impl Into<String>) -> Self {
        self.site_ids = v.into();
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/identity/adservice/api/getCloudlinkConfigurations`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCloudlinkConfigurationsQuery {
    /// List of account IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub account_ids: String,
    /// List of site IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub site_ids: String,
}

impl GetCloudlinkConfigurationsQuery {
    /// Construct the required query params.
    ///
    /// `account_ids` / `site_ids` are comma-separated lists of IDs.
    pub fn new(account_ids: impl Into<String>, site_ids: impl Into<String>) -> Self {
        Self { account_ids: account_ids.into(), site_ids: site_ids.into() }
    }
    /// Set the comma-separated list of account IDs.
    pub fn account_ids(mut self, v: impl Into<String>) -> Self {
        self.account_ids = v.into();
        self
    }
    /// Set the comma-separated list of site IDs.
    pub fn site_ids(mut self, v: impl Into<String>) -> Self {
        self.site_ids = v.into();
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/identity/adservice/api/getWindowsUnifiedAgents`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWindowsUnifiedAgentsQuery {
    /// List of account IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub account_ids: String,
    /// List of site IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub site_ids: String,
    /// Filter input for agents.
    ///
    /// Type: string. **Required.**
    pub filter_input: String,
    /// Request ID.
    ///
    /// Type: string. **Required.**
    pub request_id: String,
}

impl GetWindowsUnifiedAgentsQuery {
    /// Construct the required query params.
    ///
    /// `account_ids` / `site_ids` are comma-separated lists of IDs.
    pub fn new(
        account_ids: impl Into<String>,
        site_ids: impl Into<String>,
        filter_input: impl Into<String>,
        request_id: impl Into<String>,
    ) -> Self {
        Self {
            account_ids: account_ids.into(),
            site_ids: site_ids.into(),
            filter_input: filter_input.into(),
            request_id: request_id.into(),
        }
    }
    /// Set the comma-separated list of account IDs.
    pub fn account_ids(mut self, v: impl Into<String>) -> Self {
        self.account_ids = v.into();
        self
    }
    /// Set the comma-separated list of site IDs.
    pub fn site_ids(mut self, v: impl Into<String>) -> Self {
        self.site_ids = v.into();
        self
    }
    /// Set the filter input for agents.
    pub fn filter_input(mut self, v: impl Into<String>) -> Self {
        self.filter_input = v.into();
        self
    }
    /// Set the request ID.
    pub fn request_id(mut self, v: impl Into<String>) -> Self {
        self.request_id = v.into();
        self
    }
}

/// Query params for
/// `POST /web/api/v2.1/identity/adservice/api/isIDREnabledOnEndpoint`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IsIdrEnabledOnEndpointQuery {
    /// List of account IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub account_ids: String,
    /// List of site IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub site_ids: String,
}

impl IsIdrEnabledOnEndpointQuery {
    /// Construct the required query params.
    ///
    /// `account_ids` / `site_ids` are comma-separated lists of IDs.
    pub fn new(account_ids: impl Into<String>, site_ids: impl Into<String>) -> Self {
        Self { account_ids: account_ids.into(), site_ids: site_ids.into() }
    }
    /// Set the comma-separated list of account IDs.
    pub fn account_ids(mut self, v: impl Into<String>) -> Self {
        self.account_ids = v.into();
        self
    }
    /// Set the comma-separated list of site IDs.
    pub fn site_ids(mut self, v: impl Into<String>) -> Self {
        self.site_ids = v.into();
        self
    }
}

/// Query params for
/// `POST /web/api/v2.1/identity/adservice/api/publishADConnectorDetails`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishAdConnectorDetailsQuery {
    /// List of account IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub account_ids: String,
    /// List of site IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub site_ids: String,
}

impl PublishAdConnectorDetailsQuery {
    /// Construct the required query params.
    ///
    /// `account_ids` / `site_ids` are comma-separated lists of IDs.
    pub fn new(account_ids: impl Into<String>, site_ids: impl Into<String>) -> Self {
        Self { account_ids: account_ids.into(), site_ids: site_ids.into() }
    }
    /// Set the comma-separated list of account IDs.
    pub fn account_ids(mut self, v: impl Into<String>) -> Self {
        self.account_ids = v.into();
        self
    }
    /// Set the comma-separated list of site IDs.
    pub fn site_ids(mut self, v: impl Into<String>) -> Self {
        self.site_ids = v.into();
        self
    }
}

/// Query params for
/// `POST /web/api/v2.1/identity/adservice/api/replaceAdConnector`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceAdConnectorQuery {
    /// List of account IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub account_ids: String,
    /// List of site IDs separated by comma.
    ///
    /// Type: string. **Required.**
    pub site_ids: String,
    /// Agent UUID.
    ///
    /// Type: string. **Required.**
    pub agent_uuid: String,
}

impl ReplaceAdConnectorQuery {
    /// Construct the required query params.
    ///
    /// `account_ids` / `site_ids` are comma-separated lists of IDs.
    pub fn new(
        account_ids: impl Into<String>,
        site_ids: impl Into<String>,
        agent_uuid: impl Into<String>,
    ) -> Self {
        Self {
            account_ids: account_ids.into(),
            site_ids: site_ids.into(),
            agent_uuid: agent_uuid.into(),
        }
    }
    /// Set the comma-separated list of account IDs.
    pub fn account_ids(mut self, v: impl Into<String>) -> Self {
        self.account_ids = v.into();
        self
    }
    /// Set the comma-separated list of site IDs.
    pub fn site_ids(mut self, v: impl Into<String>) -> Self {
        self.site_ids = v.into();
        self
    }
    /// Set the agent UUID.
    pub fn agent_uuid(mut self, v: impl Into<String>) -> Self {
        self.agent_uuid = v.into();
        self
    }
}

impl IdentityAdServiceConnectorService<'_> {
    /// `GET /web/api/v2.1/identity/adservice/api/getCloudlinkConfiguration` —
    /// Get Cloudlink Configuration.
    ///
    /// Retrieve the Cloudlink configuration details.
    ///
    /// The spec returns the `GenericRestResponse` envelope whose `data` member
    /// is freeform, so the payload is surfaced as [`serde_json::Value`].
    pub async fn get_cloudlink_configuration(
        &self,
        query: &GetCloudlinkConfigurationQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/identity/adservice/api/getCloudlinkConfiguration",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/getCloudlinkConfigurationUuid`
    /// — Get Cloudlink Configuration UUID.
    ///
    /// Retrieve the UUID of the Cloudlink configuration.
    ///
    /// The spec returns the `GenericRestResponse` envelope whose `data` member
    /// is freeform, so the payload is surfaced as [`serde_json::Value`].
    pub async fn get_cloudlink_configuration_uuid(
        &self,
        query: &GetCloudlinkConfigurationUuidQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/identity/adservice/api/getCloudlinkConfigurationUuid",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/getCloudlinkConfigurations` —
    /// Get Cloudlink Configurations.
    ///
    /// Retrieve all Cloudlink configurations.
    ///
    /// The spec returns the `GenericRestResponse` envelope whose `data` member
    /// is freeform, so the payload is surfaced as [`serde_json::Value`].
    pub async fn get_cloudlink_configurations(
        &self,
        query: &GetCloudlinkConfigurationsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/identity/adservice/api/getCloudlinkConfigurations",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/getWindowsUnifiedAgents` —
    /// Get Windows Unified Agents.
    ///
    /// Retrieve Windows unified agents based on filter criteria.
    ///
    /// The spec returns the `GenericRestResponse` envelope whose `data` member
    /// is freeform, so the payload is surfaced as [`serde_json::Value`].
    pub async fn get_windows_unified_agents(
        &self,
        query: &GetWindowsUnifiedAgentsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/identity/adservice/api/getWindowsUnifiedAgents",
                q,
            )
            .await?)
    }

    /// `POST /web/api/v2.1/identity/adservice/api/isIDREnabledOnEndpoint` —
    /// Check if IDR is enabled on endpoint.
    ///
    /// Check if Incident Detection and Response is enabled on the specified
    /// endpoint.
    ///
    /// The spec returns the `GenericRestResponse` envelope whose `data` member
    /// is freeform, so the payload is surfaced as [`serde_json::Value`].
    pub async fn is_idr_enabled_on_endpoint(
        &self,
        query: &IsIdrEnabledOnEndpointQuery,
        body: &RequestBodyWrapperAgentIdrStatusInput,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<RequestBodyWrapperAgentIdrStatusInput, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/identity/adservice/api/isIDREnabledOnEndpoint",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/identity/adservice/api/publishADConnectorDetails` —
    /// Publish AD Connector details.
    ///
    /// Publish AD connector details for console.
    ///
    /// The spec returns the `GenericRestResponse` envelope whose `data` member
    /// is freeform, so the payload is surfaced as [`serde_json::Value`].
    pub async fn publish_ad_connector_details(
        &self,
        query: &PublishAdConnectorDetailsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/identity/adservice/api/publishADConnectorDetails",
                q,
                None,
            )
            .await?)
    }

    /// `POST /web/api/v2.1/identity/adservice/api/replaceAdConnector` —
    /// Replace AD Connector.
    ///
    /// Replace the AD connector with a new agent.
    ///
    /// The spec returns the `GenericRestResponse` envelope whose `data` member
    /// is freeform, so the payload is surfaced as [`serde_json::Value`].
    pub async fn replace_ad_connector(
        &self,
        query: &ReplaceAdConnectorQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/identity/adservice/api/replaceAdConnector",
                q,
                None,
            )
            .await?)
    }
}
