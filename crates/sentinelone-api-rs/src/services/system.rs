//! `System` tag — general system information.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::system::{SystemConfiguration, SystemEnv, SystemInfo, SystemStatus};
use crate::pagination::Response;

/// `System` tag — general system information.
///
/// Covers system configuration, environment, build info, and health status.
pub struct SystemService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/system/configuration`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConfigurationQuery {
    /// List of Site IDs to filter by. Example:
    /// `"225494730938493804,225494730938493915"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example:
    /// `"225494730938493804,225494730938493915"`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
}

impl GetConfigurationQuery {
    /// List of Site IDs to filter by. The iterator items are joined by comma.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Account IDs to filter by. The iterator items are joined by comma.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            ids.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// The `filter` section of [`SetConfigurationBody`].
///
/// Determines the configuration scope. Provide a site ID or leave empty for
/// global configuration.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetConfigurationFilter {
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

impl SetConfigurationFilter {
    /// Set the list of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set the list of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body for `PUT /web/api/v2.1/system/configuration`
/// (`system_PutSystemConfiguration`).
///
/// `data` carries the configuration values to change; `filter` determines the
/// scope. Both are marked `required` in the spec, but every nested field is
/// optional. The `data` payload is freeform (`serde_json::Value`) — supply only
/// the keys you want to change, matching the field names from
/// [`SystemConfiguration`].
#[derive(Debug, Default, Serialize)]
pub struct SetConfigurationBody {
    /// Configuration values to change (e.g. `globalTwoFaEnabled`,
    /// `rememberMeLength`, `accessibleUrl`, `advancedMode`, `earlyAccess`,
    /// `earlyAccessPlatforms`, `tfaEnrollmentExpiration`, `passwordExpiration`,
    /// `uiInactivityTimeoutSeconds`, `allowedDomains`). Freeform JSON object.
    pub data: serde_json::Value,
    /// Scope of the change. Provide site/account IDs or leave empty for global
    /// configuration.
    pub filter: SetConfigurationFilter,
}

impl SetConfigurationBody {
    /// Construct a body with the given freeform `data` object and a default
    /// (global-scope) filter.
    pub fn new(data: serde_json::Value) -> Self {
        Self {
            data,
            filter: SetConfigurationFilter::default(),
        }
    }
    /// Set the freeform configuration `data` object.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = data;
        self
    }
    /// Set the `filter` (scope) section.
    pub fn filter(mut self, filter: SetConfigurationFilter) -> Self {
        self.filter = filter;
        self
    }
}

impl SystemService<'_> {
    /// `GET /web/api/v2.1/system/configuration` — Get System Config.
    ///
    /// Get the configuration of your SentinelOne system. The response shows
    /// basic information of the deployed SKUs and licenses, 2FA, and the
    /// Management URL.
    pub async fn get_configuration(
        &self,
        query: &GetConfigurationQuery,
    ) -> Result<Response<SystemConfiguration>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/system/configuration", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/system/configuration` — Set System Config.
    ///
    /// Change the system configuration. Before you run this, see Get System
    /// Config. This command requires a Global Admin user or Support.
    pub async fn set_configuration(
        &self,
        body: &SetConfigurationBody,
    ) -> Result<Response<SystemConfiguration>, Error> {
        Ok(self
            .client
            .http()
            .request_json(
                Method::PUT,
                "/web/api/v2.1/system/configuration",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/system/env` — System Environment.
    ///
    /// Get environment details of the system.
    pub async fn env(&self) -> Result<Response<SystemEnv>, Error> {
        Ok(self.client.http().get("/web/api/v2.1/system/env", None).await?)
    }

    /// `GET /web/api/v2.1/system/info` — System Info.
    ///
    /// Get the Console build, version, patch, and release information.
    pub async fn info(&self) -> Result<Response<SystemInfo>, Error> {
        Ok(self.client.http().get("/web/api/v2.1/system/info", None).await?)
    }

    /// `GET /web/api/v2.1/system/status` — System Status.
    ///
    /// Get an indication of the system's health status. This command always
    /// returns a positive response when the Management Console and API server
    /// are up and running. In these cases, some features or capabilities might
    /// be impaired. This command does not require authentication. Rate limit:
    /// 1 call per second for each IP address that communicates with the
    /// Console.
    pub async fn status(&self) -> Result<Response<SystemStatus>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/system/status", None)
            .await?)
    }

    /// `GET /web/api/v2.1/system/status/cache` — Cache Status.
    ///
    /// \[DEPRECATED\] Works the same way as the "System Status" endpoint. This
    /// command does not require authentication. Rate limit: 1 call per second
    /// for each IP address that communicates with the Console.
    pub async fn status_cache(&self) -> Result<Response<SystemStatus>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/system/status/cache", None)
            .await?)
    }

    /// `GET /web/api/v2.1/system/status/db` — Database Status.
    ///
    /// \[DEPRECATED\] Works the same way as the "System Status" endpoint. This
    /// command does not require authentication. Rate limit: 1 call per second
    /// for each IP address that communicates with the Console.
    pub async fn status_db(&self) -> Result<Response<SystemStatus>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/system/status/db", None)
            .await?)
    }
}
