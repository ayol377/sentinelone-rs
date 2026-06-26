//! Service for the `Application Control - Settings and Labels` tag.
//!
//! Native Application Control settings management and labels APIs.

use sentinelone_http::Method;
use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::application_control_settings_and_labels::{
    NacCommonResponse, NacLabel, NacSettings,
};

/// `Application Control - Settings and Labels` tag.
///
/// Native Application Control settings management and labels APIs.
pub struct ApplicationControlSettingsAndLabelsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/nac/config/api/v1/nac/settings`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNacSettingsQuery {
    /// Scope type. Allowed values: `ACCOUNT`, `SITE`, `GROUP`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl GetNacSettingsQuery {
    /// Scope type. Allowed values: `ACCOUNT`, `SITE`, `GROUP`.
    pub fn scope_type(mut self, v: impl Into<String>) -> Self {
        self.scope_type = Some(v.into());
        self
    }
    /// Scope ID.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

/// Scope selector as JSON (`#/definitions/ScopeSelectorInput`).
///
/// Nested object of [`UpdateNacSettingsBody`].
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeSelectorInput {
    /// Scope type. Allowed values: `ACCOUNT`, `SITE`, `GROUP`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope IDs (e.g. `["2429685771532281481"]`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<Vec<String>>,
}

impl ScopeSelectorInput {
    /// Scope type. Allowed values: `ACCOUNT`, `SITE`, `GROUP`.
    pub fn scope_type(mut self, v: impl Into<String>) -> Self {
        self.scope_type = Some(v.into());
        self
    }
    /// Scope IDs.
    pub fn scope_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.scope_ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body for `PUT /web/api/v2.1/nac/config/api/v1/nac/settings`
/// (`#/definitions/NACSettingsInput`).
///
/// NAC settings input. All fields are optional.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNacSettingsBody {
    /// Scope. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ScopeSelectorInput>,
    /// Whether settings are inherited. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_application_control: Option<bool>,
    /// Whether NAC is enabled. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_application_control: Option<bool>,
    /// Default behavior. Allowed values: `ALLOW`, `MONITOR`, `BLOCK`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_behavior: Option<String>,
}

impl UpdateNacSettingsBody {
    /// Scope.
    pub fn scope(mut self, v: ScopeSelectorInput) -> Self {
        self.scope = Some(v);
        self
    }
    /// Whether settings are inherited.
    pub fn inherit_application_control(mut self, v: bool) -> Self {
        self.inherit_application_control = Some(v);
        self
    }
    /// Whether NAC is enabled.
    pub fn enable_application_control(mut self, v: bool) -> Self {
        self.enable_application_control = Some(v);
        self
    }
    /// Default behavior. Allowed values: `ALLOW`, `MONITOR`, `BLOCK`.
    pub fn fallback_behavior(mut self, v: impl Into<String>) -> Self {
        self.fallback_behavior = Some(v.into());
        self
    }
}

impl ApplicationControlSettingsAndLabelsService<'_> {
    /// `GET /web/api/v2.1/nac/config/api/v1/nac/labels` — Get all NAC labels.
    ///
    /// Retrieves all available labels for NAC rules.
    ///
    /// The endpoint returns a bare JSON array of labels (no `{ data: ... }`
    /// envelope), so this returns `Vec<NacLabel>` directly.
    pub async fn get_nac_labels(&self) -> Result<Vec<NacLabel>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/nac/config/api/v1/nac/labels", None)
            .await?)
    }

    /// `GET /web/api/v2.1/nac/config/api/v1/nac/settings` — Get NAC settings.
    ///
    /// Retrieves NAC Application Control settings for the given scope or
    /// default settings.
    ///
    /// The endpoint returns the settings object directly (no `{ data: ... }`
    /// envelope), so this returns [`NacSettings`] directly.
    pub async fn get_nac_settings(
        &self,
        query: &GetNacSettingsQuery,
    ) -> Result<NacSettings, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/nac/config/api/v1/nac/settings", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/nac/config/api/v1/nac/settings` — Update NAC settings.
    ///
    /// Updates or creates NAC Application Control settings for the given scope.
    ///
    /// The endpoint returns the common response object directly (no
    /// `{ data: ... }` envelope), so this returns [`NacCommonResponse`]
    /// directly.
    pub async fn update_nac_settings(
        &self,
        body: &UpdateNacSettingsBody,
    ) -> Result<NacCommonResponse, Error> {
        Ok(self
            .client
            .http()
            .request_json::<UpdateNacSettingsBody, NacCommonResponse>(
                Method::PUT,
                "/web/api/v2.1/nac/config/api/v1/nac/settings",
                None,
                Some(body),
            )
            .await?)
    }
}
