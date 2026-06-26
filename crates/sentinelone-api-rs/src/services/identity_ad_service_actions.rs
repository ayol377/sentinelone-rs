use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;

/// `Identity AD Service - Actions` tag.
///
/// APIs for performing AD service actions.
pub struct IdentityAdServiceActionsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `POST /web/api/v2.1/identity/adservice/api/actions/fetch`.
///
/// All three fields are `required` per the spec, but every field is modelled as
/// `Option<T>` (default-null convention) so the struct can be built
/// incrementally; populate all three before calling [`IdentityAdServiceActionsService::fetch`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchQuery {
    /// List of account IDs separated by comma.
    ///
    /// Required (string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma.
    ///
    /// Required (string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Type of action.
    ///
    /// Required (string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
}

impl FetchQuery {
    /// List of account IDs (joined by comma). Required.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            v.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of site IDs (joined by comma). Required.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
    /// Type of action. Required.
    pub fn action_type(mut self, v: impl Into<String>) -> Self {
        self.action_type = Some(v.into());
        self
    }
}

/// Query params for `POST /web/api/v2.1/identity/adservice/api/actions/perform`.
///
/// All three fields are `required` per the spec, but every field is modelled as
/// `Option<T>` (default-null convention) so the struct can be built
/// incrementally; populate all three before calling
/// [`IdentityAdServiceActionsService::perform`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformQuery {
    /// List of account IDs separated by comma.
    ///
    /// Required (string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of site IDs separated by comma.
    ///
    /// Required (string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Type of action.
    ///
    /// Required (string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
}

impl PerformQuery {
    /// List of account IDs (joined by comma). Required.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(
            v.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of site IDs (joined by comma). Required.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect::<Vec<_>>().join(","));
        self
    }
    /// Type of action. Required.
    pub fn action_type(mut self, v: impl Into<String>) -> Self {
        self.action_type = Some(v.into());
        self
    }
}

impl IdentityAdServiceActionsService<'_> {
    /// `POST /web/api/v2.1/identity/adservice/api/actions/fetch` — Fetch unified
    /// actions.
    ///
    /// Fetch available unified actions for the specified action type.
    ///
    /// The required query params (`accountIds`, `siteIds`, `actionType`) are
    /// supplied via [`FetchQuery`]. The request `body` is a raw `string` per the
    /// spec. The `200` response schema is also a bare `string`, so the result is
    /// returned as [`serde_json::Value`] (no `{ data }` envelope is used by this
    /// endpoint).
    pub async fn fetch(
        &self,
        query: &FetchQuery,
        body: impl Into<String>,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        let body: String = body.into();
        Ok(self
            .client
            .http()
            .request_json::<String, serde_json::Value>(
                Method::POST,
                "/web/api/v2.1/identity/adservice/api/actions/fetch",
                q,
                Some(&body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/identity/adservice/api/actions/perform` — Perform
    /// unified action.
    ///
    /// Perform a unified action for the specified action type.
    ///
    /// The required query params (`accountIds`, `siteIds`, `actionType`) are
    /// supplied via [`PerformQuery`]. The request `body` is a raw `string` per
    /// the spec. The `200` response schema is also a bare `string`, so the result
    /// is returned as [`serde_json::Value`] (no `{ data }` envelope is used by
    /// this endpoint).
    pub async fn perform(
        &self,
        query: &PerformQuery,
        body: impl Into<String>,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        let body: String = body.into();
        Ok(self
            .client
            .http()
            .request_json::<String, serde_json::Value>(
                Method::POST,
                "/web/api/v2.1/identity/adservice/api/actions/perform",
                q,
                Some(&body),
            )
            .await?)
    }
}
