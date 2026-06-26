//! `Identity AD Service - Configuration` tag.
//!
//! APIs for managing AD configuration and operations.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::identity_ad_service_configuration::*;

/// `Identity AD Service - Configuration` tag.
///
/// APIs for managing AD configuration and operations.
pub struct IdentityAdServiceConfigurationService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query structs
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/identity/adservice/api/adConfigurationFeatures`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdConfigurationFeaturesQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl AdConfigurationFeaturesQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/identity/adservice/api/adConfigurations`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdConfigurationsQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl AdConfigurationsQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `POST /web/api/v2.1/identity/adservice/api/addAdConfiguration`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAdConfigurationQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl AddAdConfigurationQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/identity/adservice/api/availableFeatures`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableFeaturesQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl AvailableFeaturesQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `POST /web/api/v2.1/identity/adservice/api/deleteAdConfiguration`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAdConfigurationQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl DeleteAdConfigurationQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/identity/adservice/api/domains`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DomainsQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl DomainsQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/identity/adservice/api/fetchGroupsSearch`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchGroupsSearchQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Domain name. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// Base DN. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_dn: Option<String>,
    /// Object classes (`array<integer(int32)>`, comma-joined). Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_classes: Option<String>,
    /// Object name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    /// Windows only computers. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_only_computers: Option<bool>,
    /// Search sub OUs. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_sub_ous: Option<bool>,
    /// Search sub domains. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_sub_domains: Option<bool>,
    /// S1 response format. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_response_format: Option<bool>,
}

impl FetchGroupsSearchQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// Domain name. Required.
    pub fn domain_name(mut self, v: impl Into<String>) -> Self {
        self.domain_name = Some(v.into());
        self
    }
    /// Base DN. Optional.
    pub fn base_dn(mut self, v: impl Into<String>) -> Self {
        self.base_dn = Some(v.into());
        self
    }
    /// Object classes (`array<integer(int32)>`, comma-joined). Required.
    pub fn object_classes<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_classes = Some(join_csv(ids));
        self
    }
    /// Object name. Optional.
    pub fn object_name(mut self, v: impl Into<String>) -> Self {
        self.object_name = Some(v.into());
        self
    }
    /// Windows only computers. Optional.
    pub fn windows_only_computers(mut self, v: bool) -> Self {
        self.windows_only_computers = Some(v);
        self
    }
    /// Search sub OUs. Required.
    pub fn search_sub_ous(mut self, v: bool) -> Self {
        self.search_sub_ous = Some(v);
        self
    }
    /// Search sub domains. Required.
    pub fn search_sub_domains(mut self, v: bool) -> Self {
        self.search_sub_domains = Some(v);
        self
    }
    /// S1 response format. Optional.
    pub fn s1_response_format(mut self, v: bool) -> Self {
        self.s1_response_format = Some(v);
        self
    }
}

/// Query params for `GET /web/api/v2.1/identity/adservice/api/fetchRootDomainsForScope`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchRootDomainsForScopeQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// S1 response format. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_response_format: Option<bool>,
}

impl FetchRootDomainsForScopeQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// S1 response format. Optional.
    pub fn s1_response_format(mut self, v: bool) -> Self {
        self.s1_response_format = Some(v);
        self
    }
}

/// Query params for `GET /web/api/v2.1/identity/adservice/api/getAdDomains`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdDomainsQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl GetAdDomainsQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/identity/adservice/api/getAdSearchResult`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdSearchResultQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Search ID. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_id: Option<String>,
    /// S1 response format. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_response_format: Option<bool>,
}

impl GetAdSearchResultQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// Search ID. Required.
    pub fn search_id(mut self, v: impl Into<String>) -> Self {
        self.search_id = Some(v.into());
        self
    }
    /// S1 response format. Optional.
    pub fn s1_response_format(mut self, v: bool) -> Self {
        self.s1_response_format = Some(v);
        self
    }
}

/// Query params for `POST /web/api/v2.1/identity/adservice/api/isPolicyInUse`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IsPolicyInUseQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl IsPolicyInUseQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/identity/adservice/api/netBios-data`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetBiosDataQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl NetBiosDataQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

/// Query params for `GET /web/api/v2.1/identity/adservice/api/timezones`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimezonesQuery {
    /// List of account IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma. Required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl TimezonesQuery {
    /// List of account IDs (comma-joined). Required.
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of site IDs (comma-joined). Required.
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
}

// ---------------------------------------------------------------------------
// Body structs
// ---------------------------------------------------------------------------

/// Scope reference for [`AdConfigurationInput`] (`Scope`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Scope type. Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Access-over-trust configuration for [`AdConfigurationInput`]
/// (`AccessOverTrustInfo`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessOverTrustInfo {
    /// Whether access over trust is enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_over_trust_enabled: Option<bool>,
    /// Trusting domain name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusting_domain_name: Option<String>,
    /// Trusting domain controller FQDN. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusting_domain_controller_fqdn: Option<String>,
}

/// Scope/info pair for [`AdConfigurationInput`] (`ScopeInfoPair`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeInfoPair {
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// Scope level. One of `ACCOUNT`, `SITE`, `GROUP`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_name: Option<String>,
    /// Scope path. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_path: Option<String>,
}

/// AD configuration input payload (`AdConfigurationInput`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdConfigurationInput {
    /// CloudLink ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudlink_id: Option<i64>,
    /// Opted features. Each value is one of `RANGER_AD`,
    /// `SINGULARITY_IDENTITY`, `RANGER_AD_PROTECT`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features_opted: Option<Vec<String>>,
    /// Allowed scopes. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_scopes: Option<Vec<Scope>>,
    /// Whether all scopes are allowed. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_scopes_allowed: Option<bool>,
    /// Domain name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// Whether to assess other domains in the forest. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assess_other_domains_in_forest: Option<bool>,
    /// Domain controller FQDN. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_controller_fqdn: Option<String>,
    /// Username. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// Password. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Encryption method. One of `LDAP`, `LDAPS`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    /// Access-over-trust info. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_over_trust_info: Option<AccessOverTrustInfo>,
    /// Whether threat detection is enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_threat_detection: Option<bool>,
    /// Whether LDAP referral is enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldap_referral: Option<bool>,
    /// Created-at scope info pair. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<ScopeInfoPair>,
    /// Whether this is an update. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<bool>,
    /// AD configuration ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_config_id: Option<i64>,
    /// Whether to use WinRM over SSL. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_win_rm_over_ssl: Option<bool>,
    /// Whether AD sync is enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_sync: Option<bool>,
}

/// Request body for `POST .../api/addAdConfiguration`
/// (`RequestBodyWrapperAdConfigurationInput`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddAdConfigurationBody {
    /// AD configuration input. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<AdConfigurationInput>,
}

/// Request body for `POST .../api/deleteAdConfiguration`
/// (`RequestBodyWrapperListLong`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteAdConfigurationBody {
    /// List of AD configuration IDs to delete. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<i64>>,
}

/// Request body for `POST .../api/isPolicyInUse`
/// (`PolicyUsageVerificationRequest`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IsPolicyInUseBody {
    /// Management ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management_id: Option<String>,
    /// AD domain. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_domain: Option<String>,
    /// Tenant ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    /// Subscriber ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_id: Option<i64>,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl IdentityAdServiceConfigurationService<'_> {
    /// `GET /web/api/v2.1/identity/adservice/api/adConfigurationFeatures` — Get
    /// AD configuration features.
    ///
    /// Retrieve list of AD configuration features.
    pub async fn ad_configuration_features(
        &self,
        query: &AdConfigurationFeaturesQuery,
    ) -> Result<GenericRestResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/identity/adservice/api/adConfigurationFeatures",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/adConfigurations` — Get AD
    /// configurations.
    ///
    /// Retrieve all AD configurations.
    pub async fn ad_configurations(
        &self,
        query: &AdConfigurationsQuery,
    ) -> Result<GenericRestResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/identity/adservice/api/adConfigurations", q)
            .await?)
    }

    /// `POST /web/api/v2.1/identity/adservice/api/addAdConfiguration` — Add AD
    /// configuration.
    ///
    /// Add a new AD configuration.
    pub async fn add_ad_configuration(
        &self,
        query: &AddAdConfigurationQuery,
        body: &AddAdConfigurationBody,
    ) -> Result<GenericRestResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<AddAdConfigurationBody, GenericRestResponse>(
                Method::POST,
                "/web/api/v2.1/identity/adservice/api/addAdConfiguration",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/availableFeatures` — Get
    /// available features.
    ///
    /// Retrieve list of available AD features.
    pub async fn available_features(
        &self,
        query: &AvailableFeaturesQuery,
    ) -> Result<GenericRestResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/identity/adservice/api/availableFeatures", q)
            .await?)
    }

    /// `POST /web/api/v2.1/identity/adservice/api/deleteAdConfiguration` —
    /// Delete AD configuration.
    ///
    /// Delete one or more AD configurations.
    pub async fn delete_ad_configuration(
        &self,
        query: &DeleteAdConfigurationQuery,
        body: &DeleteAdConfigurationBody,
    ) -> Result<GenericRestResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<DeleteAdConfigurationBody, GenericRestResponse>(
                Method::POST,
                "/web/api/v2.1/identity/adservice/api/deleteAdConfiguration",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/domains` — Get domains.
    ///
    /// Retrieve domain information for the tenant.
    pub async fn domains(&self, query: &DomainsQuery) -> Result<Vec<DomainInfo>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/identity/adservice/api/domains", q)
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/fetchGroupsSearch` — Fetch
    /// groups from AD domain.
    ///
    /// Fetch groups from Active Directory domain based on search criteria.
    ///
    /// The spec types the `200` response as a bare `string`.
    pub async fn fetch_groups_search(
        &self,
        query: &FetchGroupsSearchQuery,
    ) -> Result<String, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/identity/adservice/api/fetchGroupsSearch", q)
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/fetchRootDomainsForScope` —
    /// Fetch root domains for scope.
    ///
    /// Fetch root domains for the current scope.
    ///
    /// The spec types the `200` response as a bare `string`.
    pub async fn fetch_root_domains_for_scope(
        &self,
        query: &FetchRootDomainsForScopeQuery,
    ) -> Result<String, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/identity/adservice/api/fetchRootDomainsForScope",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/getAdDomains` — Get AD
    /// domains.
    ///
    /// Retrieve AD domain information.
    pub async fn get_ad_domains(
        &self,
        query: &GetAdDomainsQuery,
    ) -> Result<GenericRestResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/identity/adservice/api/getAdDomains", q)
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/getAdSearchResult` — Get AD
    /// search result.
    ///
    /// Retrieve AD search result by search ID.
    ///
    /// The spec types the `200` response as a bare `string`.
    pub async fn get_ad_search_result(
        &self,
        query: &GetAdSearchResultQuery,
    ) -> Result<String, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/identity/adservice/api/getAdSearchResult", q)
            .await?)
    }

    /// `POST /web/api/v2.1/identity/adservice/api/isPolicyInUse` — Check if
    /// policy is in use.
    ///
    /// Verify if a policy is currently in use.
    ///
    /// The spec types the `200` response as a bare `boolean`.
    pub async fn is_policy_in_use(
        &self,
        query: &IsPolicyInUseQuery,
        body: &IsPolicyInUseBody,
    ) -> Result<bool, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<IsPolicyInUseBody, bool>(
                Method::POST,
                "/web/api/v2.1/identity/adservice/api/isPolicyInUse",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/netBios-data` — Fetch NetBIOS
    /// data.
    ///
    /// Fetch NetBIOS data for specified AD IDs.
    ///
    /// Although the doc lists this as a `GET`, it declares an `array<string>`
    /// request body, so the AD IDs are sent as a JSON body. The spec types the
    /// `200` response as a bare `string`.
    pub async fn net_bios_data(
        &self,
        query: &NetBiosDataQuery,
        body: &[String],
    ) -> Result<String, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<[String], String>(
                Method::GET,
                "/web/api/v2.1/identity/adservice/api/netBios-data",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/identity/adservice/api/timezones` — Get timezones.
    ///
    /// Retrieve list of available timezone pairs.
    pub async fn timezones(
        &self,
        query: &TimezonesQuery,
    ) -> Result<GenericRestResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/identity/adservice/api/timezones", q)
            .await?)
    }
}
