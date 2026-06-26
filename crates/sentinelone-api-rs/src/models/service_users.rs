//! Models for the `Service Users` tag.

use serde::Deserialize;

/// A service user.
///
/// Returned by the list (`GET /web/api/v2.1/service-users`), get
/// (`GET /web/api/v2.1/service-users/{service_user_id}`), update
/// (`PUT /web/api/v2.1/service-users/{service_user_id}`) and create
/// (`POST /web/api/v2.1/service-users`) endpoints.
///
/// Note: the create response additionally exposes the newly minted token value
/// at `api_token.value` (see [`ServiceUserApiTokenInfo::value`]); other
/// endpoints leave it `None`. The single-get response additionally populates
/// [`ServiceUser::account`]; list/update/create responses leave it `None`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUser {
    /// Id.
    ///
    /// Optional/nullable per spec (not in `required`) -> defaults to `None`.
    pub id: Option<String>,
    /// Name.
    ///
    /// Nullable per spec (`x-nullable`) -> defaults to `None`.
    pub name: Option<String>,
    /// Description.
    ///
    /// Optional per spec (not in `required`) -> defaults to `None`.
    pub description: Option<String>,
    /// Created at (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub created_at: Option<String>,
    /// Updated at (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub updated_at: Option<String>,
    /// Last activation (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub last_activation: Option<String>,
    /// User scope.
    ///
    /// Required + not nullable per spec -> bare `String`.
    /// Allowed values: `tenant`, `account`, `site`.
    pub scope: String,
    /// Roles of the scope user.
    ///
    /// Optional per spec -> defaults to `None`.
    pub scope_roles: Option<Vec<ServiceUserScopeRole>>,
    /// Created by.
    ///
    /// Optional per spec -> defaults to `None`.
    pub created_by: Option<ServiceUserActor>,
    /// Updated by.
    ///
    /// Optional per spec -> defaults to `None`.
    pub updated_by: Option<ServiceUserActor>,
    /// Api token metadata.
    ///
    /// Optional per spec -> defaults to `None`.
    pub api_token: Option<ServiceUserApiTokenInfo>,
    /// Account, relevant if the user is a site level user or single account.
    ///
    /// Only populated by the single-get endpoint; nullable per spec
    /// (`x-nullable`) -> defaults to `None`.
    pub account: Option<ServiceUserAccount>,
}

/// A scope/role association for a service user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUserScopeRole {
    /// Scope ID.
    ///
    /// Optional per spec -> defaults to `None`.
    pub id: Option<String>,
    /// \[DEPRECATED\] List containing the desired role name in this scope. Use
    /// `role_id` or `role_name` instead.
    ///
    /// Optional per spec -> defaults to `None`.
    pub roles: Option<Vec<String>>,
    /// \[DEPRECATED\] Name of the role, will work only for predefined roles.
    ///
    /// Optional per spec -> defaults to `None`.
    pub role_name: Option<String>,
    /// ID of the wanted role.
    ///
    /// Optional per spec -> defaults to `None`.
    pub role_id: Option<String>,
    /// Scope name.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub name: String,
    /// Scope (account) name.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub account_name: String,
}

/// A short actor reference (created-by / updated-by) for a service user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUserActor {
    /// Id.
    ///
    /// Optional per spec -> defaults to `None`.
    pub id: Option<String>,
    /// Name.
    ///
    /// Optional per spec -> defaults to `None`.
    pub name: Option<String>,
}

/// API token metadata attached to a service user.
///
/// `value` is only present on the create response; it carries the actual token
/// secret and is `None` everywhere else.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUserApiTokenInfo {
    /// Created at (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub created_at: Option<String>,
    /// Expires at (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub expires_at: Option<String>,
    /// Token value (only present on the create response).
    ///
    /// Optional per spec -> defaults to `None`.
    pub value: Option<String>,
}

/// Account a (site/single-account scoped) service user belongs to.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUserAccount {
    /// The id of the account.
    ///
    /// Optional per spec -> defaults to `None`.
    pub id: Option<String>,
    /// The name of the account.
    ///
    /// Optional per spec -> defaults to `None`.
    pub name: Option<String>,
}

/// A freshly generated API token for a service user.
///
/// Returned by `POST /web/api/v2.1/service-users/{service_user_id}/generate-api-token`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUserApiToken {
    /// Created at (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub created_at: Option<String>,
    /// Expires at (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub expires_at: Option<String>,
    /// Service User's API token.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub token: String,
}

/// Number of entities affected by a bulk operation.
///
/// Returned by `POST /web/api/v2.1/service-users/delete-service-users`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResults {
    /// Number of entities affected by the requested operation.
    ///
    /// Optional per spec -> defaults to `None`.
    pub affected: Option<i64>,
}

/// Indicates a successful operation.
///
/// Returned by `DELETE /web/api/v2.1/service-users/{service_user_id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Indicates a successful operation.
    ///
    /// Optional per spec -> defaults to `None`.
    pub success: Option<bool>,
}
