use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::identity_ad_service_onboarding::*;
use crate::pagination::Response;

/// `Identity AD Service - Onboarding` tag.
///
/// APIs for managing AD service onboarding status.
pub struct IdentityAdServiceOnboardingService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/identity/adservice/api/getOnboardingStatus`.
///
/// Array params (`accountIds`, `siteIds`) are serialized comma-joined, as the
/// API expects. Both params are **required**.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOnboardingStatusQuery {
    /// List of account IDs separated by comma.
    ///
    /// **Required.** Serialized as a comma-joined string.
    pub account_ids: String,
    /// List of site IDs separated by comma.
    ///
    /// **Required.** Serialized as a comma-joined string.
    pub site_ids: String,
}

impl GetOnboardingStatusQuery {
    /// Construct the query from already comma-joined strings for both required params.
    pub fn new(account_ids: impl Into<String>, site_ids: impl Into<String>) -> Self {
        Self {
            account_ids: account_ids.into(),
            site_ids: site_ids.into(),
        }
    }

    /// List of account IDs (joined by comma). **Required.**
    pub fn account_ids<I, S>(mut self, account_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = account_ids
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.account_ids = joined;
        self
    }

    /// List of site IDs (joined by comma). **Required.**
    pub fn site_ids<I, S>(mut self, site_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let joined = site_ids
            .into_iter()
            .map(|s| s.as_ref().to_owned())
            .collect::<Vec<_>>()
            .join(",");
        self.site_ids = joined;
        self
    }
}

impl IdentityAdServiceOnboardingService<'_> {
    /// `GET /web/api/v2.1/identity/adservice/api/getOnboardingStatus` — Get onboarding status.
    ///
    /// Retrieve the current onboarding status for the AD service.
    ///
    /// The success body is a single-resource `{ data }` envelope. The spec's
    /// `200` schema is the generic untyped wrapper, but the onboarding payload
    /// shape is defined by `OnboardingStatusResponse` (see [`OnboardingStatus`]),
    /// which is surfaced here as the typed `data`.
    pub async fn get_onboarding_status(
        &self,
        query: &GetOnboardingStatusQuery,
    ) -> Result<Response<OnboardingStatus>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/identity/adservice/api/getOnboardingStatus",
                q,
            )
            .await?)
    }
}
