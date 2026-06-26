use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::licenses::LicensesAffected;
use crate::pagination::Response;

/// `Licenses` tag.
pub struct LicensesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// `data` object of [`UpdateSitesModulesBody`]
/// (`licenses.schemas_SiteBulkModulesSchema` -> `.data`).
///
/// Required (in parent `required`). Describes which add-on modules to apply and
/// how.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSitesModulesData {
    /// Operation. Required (in `data.required`). Enum -> `String`; allowed
    /// values: `add`, `remove`.
    pub operation: String,
    /// Modules. Optional/nullable (`x-nullable`, not required) -> `Option`. Each
    /// item is an object with a required `name` field; modeled as
    /// [`UpdateSitesModule`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<UpdateSitesModule>>,
}

impl UpdateSitesModulesData {
    /// Construct a `data` object from the required `operation`.
    ///
    /// `operation`: allowed values `add`, `remove`.
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            modules: None,
        }
    }
    /// Set the optional list of modules.
    pub fn modules<I>(mut self, modules: I) -> Self
    where
        I: IntoIterator<Item = UpdateSitesModule>,
    {
        self.modules = Some(modules.into_iter().collect());
        self
    }
}

/// A single module entry inside [`UpdateSitesModulesData::modules`]
/// (`licenses.schemas_SiteBulkModulesSchema` -> `.data.modules.items`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSitesModule {
    /// Name. Required (in the item `required` array) -> bare `String`.
    pub name: String,
}

impl UpdateSitesModule {
    /// Construct a module entry from its required `name`.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Body for `PUT /web/api/v2.1/licenses/update-sites-modules`
/// (`licenses.schemas_SiteBulkModulesSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSitesModulesBody {
    /// Data. Required (in `required`). The add-on operation to apply; see
    /// [`UpdateSitesModulesData`].
    pub data: UpdateSitesModulesData,
    /// Filter. Required (in `required`). Freeform filter object -> use
    /// [`serde_json::Value`] for forward-compat with the many site filter fields
    /// (`siteIds`, `accountIds`, `query`, `name`, `isDefault`, `healthStatus`,
    /// `siteType` (`Trial`/`Paid`), `expiration`, `totalLicenses`,
    /// `activeLicenses`, `externalId`, `description`, `createdAt`, `updatedAt`,
    /// `state` (`active`/`expired`/`deleted`), `states`, `statesNin`, `suite`
    /// (`Core`/`Control`/`Complete`, deprecated), `features`
    /// (`firewall-control`/`device-control`/`ioc`), `sku`, `module`,
    /// `accountId`, `adminOnly` (deprecated), `availableMoveSites`,
    /// `registrationToken`, `accountName__contains`, `name__contains`,
    /// `description__contains`).
    pub filter: serde_json::Value,
}

impl UpdateSitesModulesBody {
    /// Construct an update body from the required `data` and freeform `filter`.
    pub fn new(data: UpdateSitesModulesData, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

impl LicensesService<'_> {
    /// `PUT /web/api/v2.1/licenses/update-sites-modules` — Update sites add-ons.
    ///
    /// Change the add-ons of the sites by a given filter.
    pub async fn update_sites_modules(
        &self,
        body: &UpdateSitesModulesBody,
    ) -> Result<Response<LicensesAffected>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<UpdateSitesModulesBody, Response<LicensesAffected>>(
                Method::PUT,
                "/web/api/v2.1/licenses/update-sites-modules",
                None,
                Some(body),
            )
            .await?)
    }
}
