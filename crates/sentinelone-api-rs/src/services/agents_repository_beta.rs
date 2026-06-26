use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::agents_repository_beta::{ListTokensResponse, TokenResponse};

/// `Agents Repository (Beta)` tag.
///
/// Manage access tokens for the S1 Agent Artifacts Repository: list valid
/// tokens (optionally scoped), create a token (required for pulling
/// artifacts) and delete a token.
///
/// Fidelity note: these endpoints do not use the standard cursor-based
/// SentinelOne envelopes. The list endpoint returns a bespoke offset-based
/// envelope ([`ListTokensResponse`]); create returns a bare [`TokenResponse`];
/// delete returns an opaque body. Return types are modelled verbatim rather
/// than coerced into [`crate::pagination::Paginated`] / [`crate::pagination::Response`].
pub struct AgentsRepositoryBetaService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/agent-artifacts/token` — List Access Tokens.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTokensQuery {
    /// Scope level to list the tokens for.
    ///
    /// Allowed values: `site`, `account`, `tenant`. Enum modelled as `String`
    /// for forward-compat. Required: no -> `Option`.
    #[serde(rename = "scope_level", skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope id to list the tokens for, example: `983604236220743370`. Integer.
    ///
    /// Required: no -> `Option`.
    #[serde(rename = "scope_id", skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i64>,
    /// The number of tokens to return, for example: `10`. Integer.
    ///
    /// Required: no -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The number of tokens to skip before starting to collect the result,
    /// for example: `2`. Integer.
    ///
    /// Required: no -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl ListTokensQuery {
    /// Scope level to list the tokens for. Allowed values: `site`, `account`,
    /// `tenant`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Scope id to list the tokens for, example: `983604236220743370`.
    pub fn scope_id(mut self, v: i64) -> Self {
        self.scope_id = Some(v);
        self
    }
    /// The number of tokens to return, for example: `10`.
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// The number of tokens to skip before starting to collect the result,
    /// for example: `2`.
    pub fn offset(mut self, v: i64) -> Self {
        self.offset = Some(v);
        self
    }
}

/// Query params for `DELETE /web/api/v2.1/agent-artifacts/token` — Delete Access Token.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTokenQuery {
    /// Scope level to list the tokens for.
    ///
    /// Allowed values: `site`, `account`. Enum modelled as `String` for
    /// forward-compat. Required: no -> `Option`.
    #[serde(rename = "scope_level", skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope id to list the tokens for, example: `983604236220743370`. Integer.
    ///
    /// Required: no -> `Option`.
    #[serde(rename = "scope_id", skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<i64>,
    /// Token id of the token to be deleted, example: `42`. Integer.
    ///
    /// Required: no -> `Option`.
    #[serde(rename = "token_id", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<i64>,
}

impl DeleteTokenQuery {
    /// Scope level to list the tokens for. Allowed values: `site`, `account`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Scope id to list the tokens for, example: `983604236220743370`.
    pub fn scope_id(mut self, v: i64) -> Self {
        self.scope_id = Some(v);
        self
    }
    /// Token id of the token to be deleted, example: `42`.
    pub fn token_id(mut self, v: i64) -> Self {
        self.token_id = Some(v);
        self
    }
}

/// Request body for `POST /web/api/v2.1/agent-artifacts/token` — Create Access Token.
///
/// Spec definition: `handlers.TokenRequest`. No field is `required`, so all
/// are `Option`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTokenBody {
    /// Token description.
    ///
    /// Optional -> `Option`.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Scope ID of the specified account or site.
    ///
    /// Optional -> `Option`.
    #[serde(rename = "scope_id", skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// Scope Level of the token e.g. `account`, `site`.
    ///
    /// Allowed values: `account`, `site`. Enum modelled as `String` for
    /// forward-compat. Optional -> `Option`.
    #[serde(rename = "scope_level", skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Token title.
    ///
    /// Optional -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreateTokenBody {
    /// Token description.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// Scope ID of the specified account or site.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// Scope Level of the token e.g. `account`, `site`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Token title.
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.title = Some(v.into());
        self
    }
}

impl AgentsRepositoryBetaService<'_> {
    /// `GET /web/api/v2.1/agent-artifacts/token` — List Access Tokens.
    ///
    /// Lists valid access tokens for the S1 Agent Artifacts Repository, with
    /// the option to filter by scope.
    ///
    /// Returns the bespoke offset-based envelope [`ListTokensResponse`]
    /// (`{ data, pagination }`) verbatim — this endpoint does not use the
    /// standard cursor-based [`crate::pagination::Paginated`].
    pub async fn list_tokens(
        &self,
        query: &ListTokensQuery,
    ) -> Result<ListTokensResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/agent-artifacts/token", q)
            .await?)
    }

    /// `POST /web/api/v2.1/agent-artifacts/token` — Create Access Token.
    ///
    /// Creates an access token for the S1 Agent Artifacts Repository, which is
    /// needed for pulling artifacts.
    ///
    /// Returns the created [`TokenResponse`]; note `token` is seen only once.
    pub async fn create_token(
        &self,
        body: &CreateTokenBody,
    ) -> Result<TokenResponse, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/agent-artifacts/token", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/agent-artifacts/token` — Delete Access Token.
    ///
    /// Deletes an access token for the S1 Agent Artifacts Repository.
    ///
    /// The spec declares no `200` response schema for this endpoint, so the
    /// (possibly empty) body is returned as an opaque [`serde_json::Value`].
    pub async fn delete_token(
        &self,
        query: &DeleteTokenQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<(), serde_json::Value>(
                Method::DELETE,
                "/web/api/v2.1/agent-artifacts/token",
                q,
                None,
            )
            .await?)
    }
}
