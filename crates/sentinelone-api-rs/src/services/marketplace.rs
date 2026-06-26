//! `marketplace` tag — Singularity Marketplace application integrations
//! (install, configure, enable/disable, delete, browse the catalog, and read
//! invocation logs).

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::marketplace::*;
use crate::pagination::{Paginated, Response};

/// `marketplace` tag — manage Singularity Marketplace application
/// integrations and browse the Application Catalog.
pub struct MarketplaceService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ===========================================================================
// Body params
// ===========================================================================

/// Filter for [`DeleteApplicationRequest`].
///
/// Selects which installed applications to delete.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAppFilter {
    /// Filter by account IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Filter by application catalog IDs. Optional.
    #[serde(rename = "application_catalog_id", skip_serializing_if = "Option::is_none")]
    pub application_catalog_id: Option<Vec<String>>,
    /// Free-text filter by application creator. Optional.
    #[serde(rename = "creator__contains", skip_serializing_if = "Option::is_none")]
    pub creator_contains: Option<String>,
    /// Filter by group IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Filter by application IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Vec<String>>,
    /// Free-text filter by application name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter to match name or creator. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Filter by site IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// Apply at the tenant (global) level. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl DeleteAppFilter {
    /// Filter by account IDs.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Filter by application catalog IDs.
    pub fn application_catalog_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.application_catalog_id = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Free-text filter by application creator.
    pub fn creator_contains(mut self, v: impl Into<String>) -> Self {
        self.creator_contains = Some(v.into());
        self
    }
    /// Filter by group IDs.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Filter by application IDs.
    pub fn id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Free-text filter by application name.
    pub fn name_contains(mut self, v: impl Into<String>) -> Self {
        self.name_contains = Some(v.into());
        self
    }
    /// Free-text filter to match name or creator.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Filter by site IDs.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Apply at the tenant (global) level.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
}

/// Request body for `DELETE .../singularity-marketplace/applications`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteApplicationBody {
    /// Filter selecting which installed applications to delete. Required.
    pub filter: DeleteAppFilter,
}

impl DeleteApplicationBody {
    /// Construct from a [`DeleteAppFilter`].
    pub fn new(filter: DeleteAppFilter) -> Self {
        Self { filter }
    }
}

/// A single configuration key/value pair supplied when installing or updating
/// an application.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    /// Configuration field ID. Required.
    pub id: String,
    /// Configuration field value. Required.
    pub value: String,
}

impl Configuration {
    /// Construct a configuration pair.
    pub fn new(id: impl Into<String>, value: impl Into<String>) -> Self {
        Self { id: id.into(), value: value.into() }
    }
}

/// Installation data for [`InstallationBody`].
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallAppData {
    /// Name for the new application instance. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_instance_name: Option<String>,
    /// Configuration values for the application. Required.
    pub configurations: Vec<Configuration>,
}

impl InstallAppData {
    /// Construct with the required configuration values.
    pub fn new(configurations: Vec<Configuration>) -> Self {
        Self { application_instance_name: None, configurations }
    }
    /// Set the application instance name.
    pub fn application_instance_name(mut self, v: impl Into<String>) -> Self {
        self.application_instance_name = Some(v.into());
        self
    }
}

/// Installation filter for [`InstallationBody`].
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallAppFilter {
    /// Account IDs to install into. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Catalog application ID to install. Required.
    pub application_catalog_id: String,
    /// Group IDs to install into. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Site IDs to install into. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// Install at the tenant (global) level. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl InstallAppFilter {
    /// Construct with the required catalog application ID.
    pub fn new(application_catalog_id: impl Into<String>) -> Self {
        Self {
            account_ids: None,
            application_catalog_id: application_catalog_id.into(),
            group_ids: None,
            site_ids: None,
            tenant: None,
        }
    }
    /// Set account IDs to install into.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Set group IDs to install into.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Set site IDs to install into.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Install at the tenant (global) level.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
}

/// Request body for `POST .../singularity-marketplace/applications`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallationBody {
    /// Installation data (configuration + optional instance name). Required.
    pub data: InstallAppData,
    /// Filter selecting where to install. Required.
    pub filter: InstallAppFilter,
}

impl InstallationBody {
    /// Construct from the required data and filter.
    pub fn new(data: InstallAppData, filter: InstallAppFilter) -> Self {
        Self { data, filter }
    }
}

/// Update data for [`UpdateConfigurationBody`].
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppConfigData {
    /// Map of application ID to a new instance name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id_to_name_map: Option<std::collections::HashMap<String, String>>,
    /// Updated configuration values. Required.
    pub configurations: Vec<Configuration>,
}

impl UpdateAppConfigData {
    /// Construct with the required configuration values.
    pub fn new(configurations: Vec<Configuration>) -> Self {
        Self { application_id_to_name_map: None, configurations }
    }
    /// Set the application-ID-to-name map.
    pub fn application_id_to_name_map(
        mut self,
        v: std::collections::HashMap<String, String>,
    ) -> Self {
        self.application_id_to_name_map = Some(v);
        self
    }
}

/// Update filter for [`UpdateConfigurationBody`].
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppConfigFilter {
    /// Account IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Group IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Application IDs to update. Required.
    pub ids: Vec<String>,
    /// Site IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// Apply at the tenant (global) level. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl UpdateAppConfigFilter {
    /// Construct with the required application IDs.
    pub fn new<I, S>(ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            account_ids: None,
            group_ids: None,
            ids: ids.into_iter().map(Into::into).collect(),
            site_ids: None,
            tenant: None,
        }
    }
    /// Set account IDs.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Set group IDs.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Set site IDs.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Apply at the tenant (global) level.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
}

/// Request body for `PUT .../singularity-marketplace/applications`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConfigurationBody {
    /// Updated configuration data. Required.
    pub data: UpdateAppConfigData,
    /// Filter selecting which installations to update. Required.
    pub filter: UpdateAppConfigFilter,
}

impl UpdateConfigurationBody {
    /// Construct from the required data and filter.
    pub fn new(data: UpdateAppConfigData, filter: UpdateAppConfigFilter) -> Self {
        Self { data, filter }
    }
}

/// Filter for [`SwitchApplicationModeBody`].
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchApplicationModeFilter {
    /// Account IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Application ID to enable/disable. Required.
    pub application_id: String,
    /// Group IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Site IDs. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// Apply at the tenant (global) level. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl SwitchApplicationModeFilter {
    /// Construct with the required application ID.
    pub fn new(application_id: impl Into<String>) -> Self {
        Self {
            account_ids: None,
            application_id: application_id.into(),
            group_ids: None,
            site_ids: None,
            tenant: None,
        }
    }
    /// Set account IDs.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Set group IDs.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Set site IDs.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Apply at the tenant (global) level.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
}

/// Request body for
/// `POST .../singularity-marketplace/applications/{applicationMode}`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchApplicationModeBody {
    /// Filter selecting which installation to enable/disable. Required.
    pub filter: SwitchApplicationModeFilter,
}

impl SwitchApplicationModeBody {
    /// Construct from a [`SwitchApplicationModeFilter`].
    pub fn new(filter: SwitchApplicationModeFilter) -> Self {
        Self { filter }
    }
}

// ===========================================================================
// Query params
// ===========================================================================

/// Query params for
/// `GET /web/api/v2.1/singularity-marketplace/applications`.
///
/// Array-style params (`applicationCatalogId`, `id`, `accountIds`, `siteIds`)
/// are serialized as comma-joined strings, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstalledApplicationsQuery {
    /// Filter results by application catalog id (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_catalog_id: Option<String>,
    /// A list of application IDs (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Free-text filter by application name (supports multiple values). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by application creator (supports multiple values).
    /// Optional.
    #[serde(rename = "creator__contains", skip_serializing_if = "Option::is_none")]
    pub creator_contains: Option<String>,
    /// Free-text filter to match name or creator. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Filter results by account id (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Filter results by site id (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// If true, only the number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<String>,
    /// If true, pagination will be disabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_pagination: Option<String>,
    /// The column to sort the results by (e.g. `id`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl GetInstalledApplicationsQuery {
    /// Filter results by application catalog id (joined by comma).
    pub fn application_catalog_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_catalog_id =
            Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// A list of application IDs (joined by comma).
    pub fn id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Free-text filter by application name.
    pub fn name_contains(mut self, v: impl Into<String>) -> Self {
        self.name_contains = Some(v.into());
        self
    }
    /// Free-text filter by application creator.
    pub fn creator_contains(mut self, v: impl Into<String>) -> Self {
        self.creator_contains = Some(v.into());
        self
    }
    /// Free-text filter to match name or creator.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Filter results by account id (joined by comma).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids =
            Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Filter results by site id (joined by comma).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids =
            Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: impl Into<String>) -> Self {
        self.limit = Some(v.into());
        self
    }
    /// If true, only the number of items will be returned.
    pub fn count_only(mut self, v: impl Into<String>) -> Self {
        self.count_only = Some(v.into());
        self
    }
    /// If true, pagination will be disabled.
    pub fn disable_pagination(mut self, v: impl Into<String>) -> Self {
        self.disable_pagination = Some(v.into());
        self
    }
    /// The column to sort the results by (e.g. `id`).
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/singularity-marketplace/applications-catalog`.
///
/// Array-style params (`id`, `categoryIds`) are serialized as comma-joined
/// strings, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApplicationsCatalogQuery {
    /// Filter results by application catalog id (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Free-text filter by catalog application category (supports multiple
    /// values). Optional.
    #[serde(rename = "category__contains", skip_serializing_if = "Option::is_none")]
    pub category_contains: Option<String>,
    /// Free-text filter by catalog application name (supports multiple values).
    /// Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by catalog application description (supports multiple
    /// values). Optional.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// Free-text filter to match name, description or category. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Filter results by application catalog category id (comma-joined).
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_ids: Option<String>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// The column to sort the results by (e.g. `id`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl GetApplicationsCatalogQuery {
    /// Filter results by application catalog id (joined by comma).
    pub fn id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Free-text filter by catalog application category.
    pub fn category_contains(mut self, v: impl Into<String>) -> Self {
        self.category_contains = Some(v.into());
        self
    }
    /// Free-text filter by catalog application name.
    pub fn name_contains(mut self, v: impl Into<String>) -> Self {
        self.name_contains = Some(v.into());
        self
    }
    /// Free-text filter by catalog application description.
    pub fn description_contains(mut self, v: impl Into<String>) -> Self {
        self.description_contains = Some(v.into());
        self
    }
    /// Free-text filter to match name, description or category.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Filter results by application catalog category id (joined by comma).
    pub fn category_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.category_ids =
            Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: impl Into<String>) -> Self {
        self.limit = Some(v.into());
        self
    }
    /// The column to sort the results by (e.g. `id`).
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/singularity-marketplace/applications/{id}/log`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApplicationLogQuery {
    /// If true, only logs with error status (`Failure` or `Retry`) will be
    /// returned. Optional.
    #[serde(rename = "only_errors", skip_serializing_if = "Option::is_none")]
    pub only_errors: Option<String>,
}

impl GetApplicationLogQuery {
    /// If true, only logs with error status (`Failure` or `Retry`) will be
    /// returned.
    pub fn only_errors(mut self, v: impl Into<String>) -> Self {
        self.only_errors = Some(v.into());
        self
    }
}

// ===========================================================================
// Service methods
// ===========================================================================

impl MarketplaceService<'_> {
    /// `DELETE /web/api/v2.1/singularity-marketplace/applications` — Delete
    /// Application.
    ///
    /// Delete application integration from your Marketplace.
    pub async fn delete_application(
        &self,
        body: &DeleteApplicationBody,
    ) -> Result<Response<Affected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteApplicationBody, Response<Affected>>(
                Method::DELETE,
                "/web/api/v2.1/singularity-marketplace/applications",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/singularity-marketplace/applications` — Get Installed
    /// Applications.
    ///
    /// Get the installed Marketplace applications for a scope specified.
    pub async fn list_installed_applications(
        &self,
        query: &GetInstalledApplicationsQuery,
    ) -> Result<Paginated<ApplicationView>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/singularity-marketplace/applications", q)
            .await?)
    }

    /// `POST /web/api/v2.1/singularity-marketplace/applications` — Install
    /// Applications.
    ///
    /// Install application from the Application Catalog.
    pub async fn install_applications(
        &self,
        body: &InstallationBody,
    ) -> Result<Response<Affected>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/singularity-marketplace/applications", body)
            .await?)
    }

    /// `PUT /web/api/v2.1/singularity-marketplace/applications` — Update
    /// Application Configuration.
    ///
    /// Update installed application configuration.
    pub async fn update_application_configuration(
        &self,
        body: &UpdateConfigurationBody,
    ) -> Result<Response<Affected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<UpdateConfigurationBody, Response<Affected>>(
                Method::PUT,
                "/web/api/v2.1/singularity-marketplace/applications",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/singularity-marketplace/applications-catalog` — Get
    /// Applications Catalog.
    ///
    /// Get the Marketplace Application Catalog.
    pub async fn list_applications_catalog(
        &self,
        query: &GetApplicationsCatalogQuery,
    ) -> Result<Paginated<ApplicationCatalogView>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/singularity-marketplace/applications-catalog", q)
            .await?)
    }

    /// `GET /web/api/v2.1/singularity-marketplace/applications-catalog/{applicationCatalogId}/config`
    /// — Get Configuration Fields.
    ///
    /// Get the Configuration Fields of the Catalog Application.
    pub async fn get_catalog_configuration_fields(
        &self,
        application_catalog_id: impl Into<String>,
    ) -> Result<Response<ConfigurationSchemaFields>, Error> {
        let path = format!(
            "/web/api/v2.1/singularity-marketplace/applications-catalog/{}/config",
            application_catalog_id.into()
        );
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `GET /web/api/v2.1/singularity-marketplace/applications/{applicationId}/config`
    /// — Get Configuration Fields For Installed Application.
    ///
    /// Get the Catalog Application Configuration Fields.
    pub async fn get_application_configuration_fields(
        &self,
        application_id: impl Into<String>,
    ) -> Result<Response<ApplicationWithConfigurationFields>, Error> {
        let path = format!(
            "/web/api/v2.1/singularity-marketplace/applications/{}/config",
            application_id.into()
        );
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `POST /web/api/v2.1/singularity-marketplace/applications/{applicationMode}`
    /// — Enable Or Disable Application.
    ///
    /// Use this command to enable or disable application integrations that
    /// match the filter.
    ///
    /// `application_mode` (path) is required. Allowed values: `enable`,
    /// `disable`.
    pub async fn switch_application_mode(
        &self,
        application_mode: impl Into<String>,
        body: &SwitchApplicationModeBody,
    ) -> Result<Response<Affected>, Error> {
        let path = format!(
            "/web/api/v2.1/singularity-marketplace/applications/{}",
            application_mode.into()
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `GET /web/api/v2.1/singularity-marketplace/applications/{id}/log` — Get
    /// application log.
    ///
    /// Returns application invocation log.
    ///
    /// `id` (path) is the Application ID and is required.
    pub async fn get_application_log(
        &self,
        id: impl Into<String>,
        query: &GetApplicationLogQuery,
    ) -> Result<Response<Vec<ApplicationLogResponse>>, Error> {
        let path = format!(
            "/web/api/v2.1/singularity-marketplace/applications/{}/log",
            id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }
}
