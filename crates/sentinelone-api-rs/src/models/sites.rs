//! Entity models for the `Sites` tag.
//!
//! Field nullability mirrors `swagger_2_1.json`: a field is a bare `T` only when
//! it is in the schema `required` array and not `x-nullable`; otherwise it is
//! `Option<T>` (the SentinelOne "default null" behaviour). Enum-valued fields are
//! kept as `String` for forward compatibility; allowed values are documented on
//! each field.

use serde::Deserialize;

/// A SentinelOne Site.
///
/// Returned by the Sites list, get-by-id, create, update, expire, and
/// create-with-user endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    /// Site ID. Example: "182483948009279264".
    pub id: Option<String>,
    /// Name. Example: "My Site".
    pub name: Option<String>,
    /// [DEPRECATED] Registration token; use the dedicated
    /// `/sites/{site_id}/token` endpoint instead.
    pub registration_token: Option<String>,
    /// Is default.
    pub is_default: Option<bool>,
    /// Obsolete. Always true.
    pub health_status: Option<bool>,
    /// Total licenses.
    pub total_licenses: Option<i64>,
    /// Site type.
    pub site_type: Option<String>,
    /// Expiration. Example: "2018-02-27T04:49:26.257525Z".
    pub expiration: Option<String>,
    /// Id of CRM external system.
    pub external_id: Option<String>,
    /// The user-defined description for the Site.
    pub description: Option<String>,
    /// Site state. Allowed values: `active`, `expired`, `deleted`.
    pub state: Option<String>,
    /// [DEPRECATED] Use `sku` instead. Allowed values: `Core`, `Control`,
    /// `Complete`. Nullable.
    pub suite: Option<String>,
    /// [DEPRECATED] The sku of product features active for this site. Allowed
    /// values: `Core`, `Control`, `Complete`. Nullable.
    pub sku: Option<String>,
    /// Timestamp of site creation. Example: "2018-02-27T04:49:26.257525Z".
    pub created_at: Option<String>,
    /// Timestamp of last update. Example: "2018-02-27T04:49:26.257525Z".
    pub updated_at: Option<String>,
    /// Account id. Example: "225494730938493804".
    pub account_id: Option<String>,
    /// Account name. Example: "SentinelOne".
    pub account_name: Option<String>,
    /// Number of active licenses for the site.
    pub active_licenses: Option<i64>,
    /// The site licenses configuration.
    pub licenses: Option<SiteLicenses>,
    /// Usage type.
    pub usage_type: Option<String>,
    /// IR (incident response) fields.
    pub ir_fields: Option<SiteIrFields>,
    /// Prompt fields.
    pub prompt_fields: Option<SitePromptFields>,
    /// Inherit account expiration.
    pub inherit_account_expiration: Option<bool>,
    /// Full name of the creating user.
    pub creator: Option<String>,
    /// Id of the creating user. Example: "225494730938493804".
    pub creator_id: Option<String>,
    /// True if the Site has no expiration date.
    pub unlimited_expiration: Option<bool>,
    /// True if the Site has unlimited licenses.
    pub unlimited_licenses: Option<bool>,
    /// The data of the newly created site admin (only present on the
    /// create-site-and-user response).
    pub user: Option<SiteUser>,
}

/// The licenses configuration for a Site.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteLicenses {
    /// The licenses Bundles.
    pub bundles: Option<Vec<SiteLicenseBundle>>,
    /// The licenses Add-ons.
    pub modules: Option<Vec<SiteLicenseModule>>,
    /// The licenses Settings.
    pub settings: Option<Vec<SiteLicenseSetting>>,
}

/// A licenses Bundle of a Site.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteLicenseBundle {
    /// The Bundle display name.
    pub display_name: Option<String>,
    /// The Bundle major version.
    pub major_version: Option<i64>,
    /// The Bundle minor version.
    pub minor_version: Option<i64>,
    /// The Bundle internal api name.
    pub name: Option<String>,
    /// The Surfaces in the Bundle.
    pub surfaces: Option<Vec<SiteLicenseSurface>>,
    /// The total number of Surfaces in this Bundle. -1 indicates unlimited count.
    pub total_surfaces: Option<i64>,
}

/// A Surface within a licenses Bundle.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteLicenseSurface {
    /// The Surface count. -1 indicates unlimited count.
    pub count: Option<i64>,
    /// The Surface name.
    pub name: Option<String>,
}

/// A licenses Add-on (module) of a Site.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteLicenseModule {
    /// The Add-on display name.
    pub display_name: Option<String>,
    /// The Add-on major version.
    pub major_version: Option<i64>,
    /// The Add-on internal api name.
    pub name: Option<String>,
}

/// A licenses Setting of a Site.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteLicenseSetting {
    /// [DEPRECATED] The Setting display name.
    pub display_name: Option<String>,
    /// [DEPRECATED] The Setting group name.
    pub setting_group: Option<String>,
    /// The Setting group display name.
    pub setting_group_display_name: Option<String>,
    /// The Setting group name.
    pub group_name: Option<serde_json::Value>,
    /// The Setting display name.
    pub setting: Option<serde_json::Value>,
}

/// IR (incident response) fields of a Site.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteIrFields {
    /// The customer's company name. Example: "Acme Corp".
    pub company_name: Option<String>,
    /// First name of the customer contact. Example: "Jane".
    pub contact_first_name: Option<String>,
    /// Last name of the customer contact. Example: "Doe".
    pub contact_last_name: Option<String>,
    /// Business email of the customer contact.
    pub contact_email: Option<String>,
    /// State or region within the country. Example: "California".
    pub region: Option<String>,
    /// Country of the customer. Example: "United States".
    pub country: Option<String>,
    /// City of the customer. Example: "San Francisco". Nullable.
    pub city: Option<String>,
    /// Postal / ZIP code of the customer. Example: "94105". Nullable.
    pub postal: Option<String>,
    /// Estimated number of employees or endpoints at the customer. Example: 500.
    pub number_of_employees_endpoints: Option<i64>,
    /// Industry vertical of the customer. Allowed values: `Education`,
    /// `Government (Central/Federal)`, `Government (Local)`,
    /// `Healthcare (Provider)`, `Other`.
    pub industry: Option<String>,
}

/// Prompt fields of a Site.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SitePromptFields {
    /// Region associated with the prompt. Allowed values: `useast`, `eunorth`,
    /// `apsouth`. Nullable.
    pub prompt_region: Option<String>,
    /// Tenant name associated with the prompt. Example: "Acme Corp". Nullable.
    pub prompt_tenant_name: Option<String>,
    /// Email of the user associated with the prompt. Nullable.
    pub prompt_user_email: Option<String>,
    /// Name of the user associated with the prompt. Example: "John Doe". Nullable.
    pub prompt_user_name: Option<String>,
}

/// The data of a newly created site admin user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteUser {
    /// Id.
    pub id: Option<String>,
    /// Email. Example: "admin@sentinelone.com".
    pub email: Option<String>,
    /// Full name.
    pub full_name: Option<String>,
    /// Two fa enabled.
    pub two_fa_enabled: Option<bool>,
    /// Primary two fa method.
    pub primary_two_fa_method: Option<String>,
}

/// `data` payload of the Sites list response
/// (`GET /web/api/v2.1/sites`).
///
/// Unlike most list endpoints, the Sites list nests its array under
/// `data.sites` alongside an `allSites` aggregate, so it does not use the
/// generic `Paginated<T>` envelope.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SitesListData {
    /// Aggregate license counts across all matching sites.
    pub all_sites: Option<SitesListAllSites>,
    /// The matching sites.
    pub sites: Option<Vec<Site>>,
}

/// Aggregate license counts in the Sites list response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SitesListAllSites {
    /// Active licenses.
    pub active_licenses: Option<i64>,
    /// Total licenses.
    pub total_licenses: Option<i64>,
}

/// `data` payload of `GET /web/api/v2.1/sites/{site_id}/token`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteToken {
    /// Token.
    pub token: Option<String>,
}

/// `data` payload of `PUT /web/api/v2.1/sites/{site_id}/regenerate-key`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteRegenerateKey {
    /// Registration token.
    pub registration_token: Option<String>,
}

/// `data` payload of a generic success response (e.g. delete / revert-policy /
/// reactivate).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteSuccess {
    /// Indicates a successful operation.
    pub success: Option<bool>,
}

/// `data` payload of an affected-results response
/// (`PUT /web/api/v2.1/sites/update-bulk`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteAffected {
    /// Number of entities affected by the requested operation.
    pub affected: Option<i64>,
}

/// `data` payload of `GET /web/api/v2.1/sites/{site_id}/local-authorization`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SiteApproval {
    /// Authorized agents.
    pub authorized_agents: Option<i64>,
    /// Site authorization (expiration timestamp).
    pub site_authorization: Option<String>,
}

/// `data` payload of `PUT /web/api/v2.1/sites/{site_id}/local-authorization`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SitePutApproval {
    /// Authorized agents.
    pub authorized_agents: Option<i64>,
}
