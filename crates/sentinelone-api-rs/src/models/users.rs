//! Models for the `Users` tag.

use serde::Deserialize;

/// A user.
///
/// Returned by the list (`GET /web/api/v2.1/users`), user-by-token
/// (`GET /web/api/v2.1/user`), create (`POST /web/api/v2.1/users`), update
/// (`PUT /web/api/v2.1/users/{user_id}`) and get
/// (`GET /web/api/v2.1/users/{user_id}`) endpoints.
///
/// The single-get response additionally populates [`User::pages`] and
/// [`User::account`]; other responses leave them `None`. [`User::scope`] is the
/// only field marked `required` in the spec; everything else defaults to `None`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// Id.
    ///
    /// Optional per spec (not in `required`) -> defaults to `None`.
    pub id: Option<String>,
    /// Source.
    ///
    /// Optional per spec -> defaults to `None`.
    pub source: Option<String>,
    /// Global user id.
    ///
    /// Nullable per spec (`x-nullable`) -> defaults to `None`.
    pub global_user_id: Option<String>,
    /// Global organization id.
    ///
    /// Nullable per spec (`x-nullable`) -> defaults to `None`.
    pub global_organization_id: Option<String>,
    /// Email.
    ///
    /// Nullable per spec (`x-nullable`) -> defaults to `None`.
    pub email: Option<String>,
    /// True if email cannot be modified.
    ///
    /// Optional per spec -> defaults to `None`.
    pub email_read_only: Option<bool>,
    /// Full name.
    ///
    /// Nullable per spec (`x-nullable`) -> defaults to `None`.
    pub full_name: Option<String>,
    /// True if full name cannot be modified.
    ///
    /// Optional per spec -> defaults to `None`.
    pub full_name_read_only: Option<bool>,
    /// First login (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub first_login: Option<String>,
    /// Last login (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub last_login: Option<String>,
    /// Date joined (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub date_joined: Option<String>,
    /// \[DEPRECATED\] In RBAC there's no 'lowest' role. Returns `Admin` if the
    /// user has admin permission on all sites, otherwise a different role.
    ///
    /// Optional per spec -> defaults to `None`.
    pub lowest_role: Option<String>,
    /// \[Deprecated\].
    ///
    /// Optional per spec -> defaults to `None`.
    pub groups_read_only: Option<bool>,
    /// Two fa enabled.
    ///
    /// Optional per spec -> defaults to `None`.
    pub two_fa_enabled: Option<bool>,
    /// Primary two fa method.
    ///
    /// Optional per spec -> defaults to `None`.
    pub primary_two_fa_method: Option<String>,
    /// True if user verification completed successfully.
    ///
    /// Optional per spec -> defaults to `None`.
    pub email_verified: Option<bool>,
    /// User Scope.
    ///
    /// Required + not nullable per spec -> bare `String`.
    /// Allowed values: `tenant`, `account`, `site`.
    pub scope: String,
    /// Api token metadata.
    ///
    /// Optional per spec -> defaults to `None`.
    pub api_token: Option<UserApiTokenInfo>,
    /// True if two fa option cannot be modified.
    ///
    /// Optional per spec -> defaults to `None`.
    pub two_fa_enabled_read_only: Option<bool>,
    /// True if EULA was agreed for user's sites.
    ///
    /// Optional per spec -> defaults to `None`.
    pub agreed_eula: Option<bool>,
    /// Link to EULA agreement if it was not agreed yet.
    ///
    /// Optional per spec -> defaults to `None`.
    pub agreement_url: Option<String>,
    /// \[DEPRECATED\] Unused field. The user's role will determine if it is
    /// allowed to use remote_shell.
    ///
    /// Optional per spec -> defaults to `None`.
    pub allow_remote_shell: Option<bool>,
    /// User 2FA Auth is configured.
    ///
    /// Optional per spec -> defaults to `None`.
    pub two_fa_configured: Option<bool>,
    /// Roles of the scope user.
    ///
    /// Optional per spec -> defaults to `None`.
    pub scope_roles: Option<Vec<UserScopeRole>>,
    /// Whether the user is a system user.
    ///
    /// Optional per spec -> defaults to `None`.
    pub is_system: Option<bool>,
    /// Is external login user.
    ///
    /// Optional per spec (only on single/by-token responses) -> defaults to
    /// `None`.
    pub is_external_login_user: Option<bool>,
    /// Can generate api token.
    ///
    /// Optional per spec -> defaults to `None`.
    pub can_generate_api_token: Option<bool>,
    /// Defines for how many minutes can the user call protected actions once
    /// their session is elevated.
    ///
    /// Optional per spec (only on single/by-token responses) -> defaults to
    /// `None`.
    pub elevated_session_duration_minutes: Option<i64>,
    /// State of 2FA setup.
    ///
    /// Optional per spec -> defaults to `None`.
    /// Allowed values: `configured`, `not_configured`, `enrolled`,
    /// `enrollment_expired`.
    pub two_fa_status: Option<String>,
    /// \[DEPRECATED\] Role ids for the tenant user. Using `scope_roles` is more
    /// consistent.
    ///
    /// Optional per spec -> defaults to `None`. Freeform items.
    pub tenant_roles: Option<Vec<serde_json::Value>>,
    /// \[DEPRECATED\] Role and site ids for the user. Using `scope_roles` is more
    /// consistent.
    ///
    /// Optional per spec -> defaults to `None`.
    pub site_roles: Option<Vec<UserSiteRole>>,
    /// Pages (only populated by the single-get endpoint).
    ///
    /// Optional per spec -> defaults to `None`.
    pub pages: Option<Vec<UserPage>>,
    /// Account, relevant if the user is a site level user or single account.
    ///
    /// Only populated by the single-get endpoint; nullable per spec
    /// (`x-nullable`) -> defaults to `None`.
    pub account: Option<UserAccount>,
}

/// API token metadata attached to a user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserApiTokenInfo {
    /// Created at (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub created_at: Option<String>,
    /// Expires at (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub expires_at: Option<String>,
}

/// A scope/role association for a user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserScopeRole {
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

/// \[DEPRECATED\] A site/role association for a user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSiteRole {
    /// Site ID.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub id: String,
    /// Site name.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub name: String,
    /// \[DEPRECATED\] List containing the desired role name in this scope. Use
    /// `role_id` instead.
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
}

/// A console page the user has access to (single-get endpoint only).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPage {
    /// Name.
    ///
    /// Optional per spec -> defaults to `None`.
    pub name: Option<String>,
    /// Identifier.
    ///
    /// Optional per spec -> defaults to `None`.
    pub identifier: Option<String>,
    /// Permissions.
    ///
    /// Optional per spec -> defaults to `None`.
    pub permissions: Option<Vec<UserPagePermission>>,
}

/// A permission entry for a [`UserPage`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPagePermission {
    /// Identifier.
    ///
    /// Optional per spec -> defaults to `None`.
    pub identifier: Option<String>,
    /// Title.
    ///
    /// Optional per spec -> defaults to `None`.
    pub title: Option<String>,
    /// Value.
    ///
    /// Optional per spec -> defaults to `None`.
    pub value: Option<bool>,
    /// Description.
    ///
    /// Optional per spec -> defaults to `None`.
    pub description: Option<String>,
    /// Additional description.
    ///
    /// Nullable per spec (`x-nullable`) -> defaults to `None`.
    pub additional_description: Option<String>,
    /// Type.
    ///
    /// Optional per spec -> defaults to `None`.
    #[serde(rename = "type")]
    pub permission_type: Option<String>,
    /// Depends on.
    ///
    /// Optional per spec -> defaults to `None`.
    pub depends_on: Option<Vec<String>>,
    /// Group name.
    ///
    /// Nullable per spec (`x-nullable`) -> defaults to `None`.
    pub group_name: Option<String>,
    /// Disabled reason.
    ///
    /// Nullable per spec (`x-nullable`) -> defaults to `None`.
    pub disabled_reason: Option<String>,
    /// Disabled reason code.
    ///
    /// Nullable per spec (`x-nullable`) -> defaults to `None`.
    pub disabled_reason_code: Option<String>,
}

/// Account a (site/single-account scoped) user belongs to.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAccount {
    /// The id of the account.
    ///
    /// Optional per spec -> defaults to `None`.
    pub id: Option<String>,
    /// The name of the account.
    ///
    /// Optional per spec -> defaults to `None`.
    pub name: Option<String>,
}

/// Indicates a successful operation.
///
/// Returned by the 2FA enable/disable, revoke-api-token, change-password,
/// auth/eula, the *-auth-check, onboarding verify/validate-token endpoints.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Indicates a successful operation.
    ///
    /// Optional per spec -> defaults to `None`.
    pub success: Option<bool>,
}

/// Number of entities affected by a bulk operation.
///
/// Returned by reset-2fa, delete-2fa, onboarding/send-verification-email,
/// login/send-reset-password-email and login/force-reset-password-on-login.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResults {
    /// Number of entities affected by the requested operation.
    ///
    /// Optional per spec -> defaults to `None`.
    pub affected: Option<i64>,
}

/// An iFrame token.
///
/// Returned by `POST /web/api/v2.1/users/generate-iframe-token`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIFrameToken {
    /// User's iframe token.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub iframe_token: String,
}

/// A freshly generated API token for the authenticated user.
///
/// Returned by `POST /web/api/v2.1/users/generate-api-token`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedApiToken {
    /// User's API token.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub token: String,
}

/// Details of an API token (created/expiry timestamps).
///
/// Returned by `POST /web/api/v2.1/users/api-token-details` and
/// `GET /web/api/v2.1/users/{user_id}/api-token-details`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTokenDetail {
    /// Created at (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub created_at: Option<String>,
    /// Expires at (date-time string).
    ///
    /// Optional per spec -> defaults to `None`.
    pub expires_at: Option<String>,
}

/// Response to a 2FA app request.
///
/// Returned by `POST /web/api/v2.1/users/request-app`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestAppResponse {
    /// Qr code.
    ///
    /// Optional per spec -> defaults to `None`.
    pub qr_code: Option<String>,
    /// Code.
    ///
    /// Optional per spec -> defaults to `None`.
    pub code: Option<String>,
}

/// Response to a session elevation request.
///
/// Returned by `POST /web/api/v2.1/users/auth/elevate`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevateSessionResponse {
    /// Duration.
    ///
    /// Optional per spec -> defaults to `None`.
    pub duration: Option<i64>,
}

/// Response to a 2FA enrollment request.
///
/// Returned by `POST /web/api/v2.1/users/enroll-2fa`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollTfaResponse {
    /// Number of entities affected by the requested operation.
    ///
    /// Optional per spec -> defaults to `None`.
    pub affected: Option<i64>,
    /// The number of hours until 2FA enrollment expires.
    ///
    /// Optional per spec -> defaults to `None`.
    pub expiration: Option<i64>,
}

/// Result of a login or app-auth attempt.
///
/// Returned by `POST /web/api/v2.1/users/login` and
/// `POST /web/api/v2.1/users/auth/app`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginOutput {
    /// Generated authentication token.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub token: String,
    /// User verification status.
    ///
    /// Optional per spec -> defaults to `None`.
    /// Allowed values: `verified`, `temporary`, `pending`, `incorrect`,
    /// `expired`, `no_permissions`, `temporary_token_is_older_than_password`,
    /// `too_many_sessions`, `session_hijacking`, `elevated_session_required`,
    /// `elevated_session_setup_required`, `two_fa_enrollment_expired`,
    /// `two_fa_not_enrolled`.
    pub status: Option<String>,
    /// Two-factor authentication method (if enabled).
    ///
    /// Optional per spec -> defaults to `None`. Allowed values: `sms`,
    /// `application`.
    pub two_fa_method: Option<String>,
    /// Generated csrf token.
    ///
    /// Optional per spec -> defaults to `None`.
    pub csrf: Option<String>,
}

/// Result of a logout.
///
/// Returned by `POST /web/api/v2.1/users/logout`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogoutResponse {
    /// Where the browser should be redirected after the logout.
    ///
    /// Optional per spec -> defaults to `None`.
    pub redirect_to: Option<String>,
}

/// A user token (login by API token).
///
/// Returned by `POST /web/api/v2.1/users/login/by-api-token`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenResponse {
    /// User token.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub token: String,
    /// When logging in from Atlas, specifies the actual user who logged in.
    ///
    /// Optional per spec -> defaults to `None`.
    pub real_user: Option<String>,
    /// Removed saved scope.
    ///
    /// Optional per spec -> defaults to `None`.
    pub removed_saved_scope: Option<String>,
    /// Relative url to redirect to.
    ///
    /// Optional per spec -> defaults to `None`.
    pub redirect_to: Option<String>,
    /// Query params for the redirect to, without '?' prefix.
    ///
    /// Optional per spec -> defaults to `None`.
    pub redirect_to_params: Option<String>,
}

/// Result of continuing a login flow.
///
/// Returned by `POST /web/api/v2.1/users/login-continue`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginContinueResponse {
    /// Generated authentication token.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub token: String,
    /// Generated csrf token.
    ///
    /// Optional per spec -> defaults to `None`.
    pub csrf: Option<String>,
}

/// Result of setting a new password.
///
/// Returned by `POST /web/api/v2.1/users/login/set-password`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPasswordResponse {
    /// Generated authentication token.
    ///
    /// Required + not nullable per spec -> bare `String`.
    pub token: String,
    /// Generated csrf token.
    ///
    /// Optional per spec -> defaults to `None`.
    pub csrf: Option<String>,
}
