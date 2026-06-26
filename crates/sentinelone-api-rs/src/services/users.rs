//! `Users` tag — User related APIs.
//!
//! Hand-written for 1:1 parity with the SentinelOne Management API 2.1 spec.
//! Every endpoint listed under the `Users` tag is implemented as an async method
//! on [`UsersService`].

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::users::*;
use crate::pagination::{Paginated, Response};

/// `Users` tag — user management, authentication, 2FA and API-token endpoints.
pub struct UsersService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Join an iterator of strings into a comma-separated value for array query
/// params (the encoding the SentinelOne API expects, e.g. `id1,id2`).
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

// ===========================================================================
// Query params
// ===========================================================================

/// Query params for `GET /web/api/v2.1/export/users` (Export Users).
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportUsersQuery {
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// User Source. Allowed values: `mgmt`, `sso_saml`, `active_directory`, `global`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Source in. Allowed values: `mgmt`, `sso_saml`, `active_directory`, `global`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<String>,
    /// Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Match email partially (substring).
    #[serde(rename = "email__contains", skip_serializing_if = "Option::is_none")]
    pub email_contains: Option<String>,
    /// True if email cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_read_only: Option<bool>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Match full name partially (substring).
    #[serde(rename = "fullName__contains", skip_serializing_if = "Option::is_none")]
    pub full_name_contains: Option<String>,
    /// True if full name cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_read_only: Option<bool>,
    /// First login (date-time string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_login: Option<String>,
    /// Last login (date-time string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    /// Date joined (date-time string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<String>,
    /// \[DEPRECATED\] True if permissions cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_read_only: Option<bool>,
    /// Two fa enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_enabled: Option<bool>,
    /// Primary two fa method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_two_fa_method: Option<String>,
    /// Return only verified/unverified users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    /// Full text search for fields: full_name, email, description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// List of user IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of rbac roles to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<String>,
    /// Two fa status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_status: Option<String>,
    /// Two fa status in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_statuses: Option<String>,
    /// Can generate api token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_generate_api_token: Option<bool>,
    /// Has valid api token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_valid_api_token: Option<bool>,
    /// User was last active before this timestamp.
    #[serde(rename = "lastActivation__lt", skip_serializing_if = "Option::is_none")]
    pub last_activation_lt: Option<String>,
    /// User was last active before or at this timestamp.
    #[serde(rename = "lastActivation__lte", skip_serializing_if = "Option::is_none")]
    pub last_activation_lte: Option<String>,
    /// User was last active after this timestamp.
    #[serde(rename = "lastActivation__gt", skip_serializing_if = "Option::is_none")]
    pub last_activation_gt: Option<String>,
    /// User was last active after or at this timestamp.
    #[serde(rename = "lastActivation__gte", skip_serializing_if = "Option::is_none")]
    pub last_activation_gte: Option<String>,
    /// Date range for when the user was last active (`<from>-<to>`, inclusive).
    #[serde(rename = "lastActivation__between", skip_serializing_if = "Option::is_none")]
    pub last_activation_between: Option<String>,
    /// API token expires before this timestamp.
    #[serde(rename = "apiTokenExpiresAt__lt", skip_serializing_if = "Option::is_none")]
    pub api_token_expires_at_lt: Option<String>,
    /// API token expires before or at this timestamp.
    #[serde(rename = "apiTokenExpiresAt__lte", skip_serializing_if = "Option::is_none")]
    pub api_token_expires_at_lte: Option<String>,
    /// API token expires after this timestamp.
    #[serde(rename = "apiTokenExpiresAt__gt", skip_serializing_if = "Option::is_none")]
    pub api_token_expires_at_gt: Option<String>,
    /// API token expires after or at this timestamp.
    #[serde(rename = "apiTokenExpiresAt__gte", skip_serializing_if = "Option::is_none")]
    pub api_token_expires_at_gte: Option<String>,
    /// Date range for when the API token expires (`<from>-<to>`, inclusive).
    #[serde(rename = "apiTokenExpiresAt__between", skip_serializing_if = "Option::is_none")]
    pub api_token_expires_at_between: Option<String>,
    /// User was created before this timestamp.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// User was created before or at this timestamp.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// User was created after this timestamp.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// User was created after or at this timestamp.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Date range for when the user was created (`<from>-<to>`, inclusive).
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
}

impl ExportUsersQuery {
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// User Source. Allowed values: `mgmt`, `sso_saml`, `active_directory`, `global`.
    pub fn source(mut self, v: impl Into<String>) -> Self {
        self.source = Some(v.into());
        self
    }
    /// Source in (allowed values: `mgmt`, `sso_saml`, `active_directory`, `global`).
    pub fn sources<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sources = Some(join_csv(v));
        self
    }
    /// Email.
    pub fn email(mut self, v: impl Into<String>) -> Self {
        self.email = Some(v.into());
        self
    }
    /// Match email partially (substring).
    pub fn email_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.email_contains = Some(join_csv(v));
        self
    }
    /// True if email cannot be changed.
    pub fn email_read_only(mut self, v: bool) -> Self {
        self.email_read_only = Some(v);
        self
    }
    /// Full name.
    pub fn full_name(mut self, v: impl Into<String>) -> Self {
        self.full_name = Some(v.into());
        self
    }
    /// Match full name partially (substring).
    pub fn full_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.full_name_contains = Some(join_csv(v));
        self
    }
    /// True if full name cannot be changed.
    pub fn full_name_read_only(mut self, v: bool) -> Self {
        self.full_name_read_only = Some(v);
        self
    }
    /// First login (date-time string).
    pub fn first_login(mut self, v: impl Into<String>) -> Self {
        self.first_login = Some(v.into());
        self
    }
    /// Last login (date-time string).
    pub fn last_login(mut self, v: impl Into<String>) -> Self {
        self.last_login = Some(v.into());
        self
    }
    /// Date joined (date-time string).
    pub fn date_joined(mut self, v: impl Into<String>) -> Self {
        self.date_joined = Some(v.into());
        self
    }
    /// \[DEPRECATED\] True if permissions cannot be changed.
    pub fn groups_read_only(mut self, v: bool) -> Self {
        self.groups_read_only = Some(v);
        self
    }
    /// Two fa enabled.
    pub fn two_fa_enabled(mut self, v: bool) -> Self {
        self.two_fa_enabled = Some(v);
        self
    }
    /// Primary two fa method.
    pub fn primary_two_fa_method(mut self, v: impl Into<String>) -> Self {
        self.primary_two_fa_method = Some(v.into());
        self
    }
    /// Return only verified/unverified users.
    pub fn email_verified(mut self, v: bool) -> Self {
        self.email_verified = Some(v);
        self
    }
    /// Full text search for fields: full_name, email, description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// List of user IDs to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// List of rbac roles to filter by.
    pub fn role_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.role_ids = Some(join_csv(v));
        self
    }
    /// Two fa status.
    pub fn two_fa_status(mut self, v: impl Into<String>) -> Self {
        self.two_fa_status = Some(v.into());
        self
    }
    /// Two fa status in.
    pub fn two_fa_statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.two_fa_statuses = Some(join_csv(v));
        self
    }
    /// Can generate api token.
    pub fn can_generate_api_token(mut self, v: bool) -> Self {
        self.can_generate_api_token = Some(v);
        self
    }
    /// Has valid api token.
    pub fn has_valid_api_token(mut self, v: bool) -> Self {
        self.has_valid_api_token = Some(v);
        self
    }
    /// User was last active before this timestamp.
    pub fn last_activation_lt(mut self, v: impl Into<String>) -> Self {
        self.last_activation_lt = Some(v.into());
        self
    }
    /// User was last active before or at this timestamp.
    pub fn last_activation_lte(mut self, v: impl Into<String>) -> Self {
        self.last_activation_lte = Some(v.into());
        self
    }
    /// User was last active after this timestamp.
    pub fn last_activation_gt(mut self, v: impl Into<String>) -> Self {
        self.last_activation_gt = Some(v.into());
        self
    }
    /// User was last active after or at this timestamp.
    pub fn last_activation_gte(mut self, v: impl Into<String>) -> Self {
        self.last_activation_gte = Some(v.into());
        self
    }
    /// Date range for when the user was last active (`<from>-<to>`, inclusive).
    pub fn last_activation_between(mut self, v: impl Into<String>) -> Self {
        self.last_activation_between = Some(v.into());
        self
    }
    /// API token expires before this timestamp.
    pub fn api_token_expires_at_lt(mut self, v: impl Into<String>) -> Self {
        self.api_token_expires_at_lt = Some(v.into());
        self
    }
    /// API token expires before or at this timestamp.
    pub fn api_token_expires_at_lte(mut self, v: impl Into<String>) -> Self {
        self.api_token_expires_at_lte = Some(v.into());
        self
    }
    /// API token expires after this timestamp.
    pub fn api_token_expires_at_gt(mut self, v: impl Into<String>) -> Self {
        self.api_token_expires_at_gt = Some(v.into());
        self
    }
    /// API token expires after or at this timestamp.
    pub fn api_token_expires_at_gte(mut self, v: impl Into<String>) -> Self {
        self.api_token_expires_at_gte = Some(v.into());
        self
    }
    /// Date range for when the API token expires (`<from>-<to>`, inclusive).
    pub fn api_token_expires_at_between(mut self, v: impl Into<String>) -> Self {
        self.api_token_expires_at_between = Some(v.into());
        self
    }
    /// User was created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// User was created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// User was created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// User was created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Date range for when the user was created (`<from>-<to>`, inclusive).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/users` (List users).
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000 items, use `cursor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<u32>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor position returned by the last request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items is returned, without the objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, the total item count is not calculated (faster).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort by. Allowed values: `id`, `createdAt`, `fullName`,
    /// `firstLogin`, `lastLogin`, `source`, `email`, `emailVerified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// User Source. Allowed values: `mgmt`, `sso_saml`, `active_directory`, `global`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Source in. Allowed values: `mgmt`, `sso_saml`, `active_directory`, `global`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<String>,
    /// Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Match email partially (substring).
    #[serde(rename = "email__contains", skip_serializing_if = "Option::is_none")]
    pub email_contains: Option<String>,
    /// True if email cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_read_only: Option<bool>,
    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Match full name partially (substring).
    #[serde(rename = "fullName__contains", skip_serializing_if = "Option::is_none")]
    pub full_name_contains: Option<String>,
    /// True if full name cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_read_only: Option<bool>,
    /// First login (date-time string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_login: Option<String>,
    /// Last login (date-time string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    /// Date joined (date-time string).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_joined: Option<String>,
    /// \[DEPRECATED\] True if permissions cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups_read_only: Option<bool>,
    /// Two fa enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_enabled: Option<bool>,
    /// Primary two fa method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_two_fa_method: Option<String>,
    /// Return only verified/unverified users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    /// Full text search for fields: full_name, email, description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// List of user IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of rbac roles to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<String>,
    /// Two fa status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_status: Option<String>,
    /// Two fa status in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_statuses: Option<String>,
    /// Can generate api token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_generate_api_token: Option<bool>,
    /// Has valid api token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_valid_api_token: Option<bool>,
    /// User was last active before this timestamp.
    #[serde(rename = "lastActivation__lt", skip_serializing_if = "Option::is_none")]
    pub last_activation_lt: Option<String>,
    /// User was last active before or at this timestamp.
    #[serde(rename = "lastActivation__lte", skip_serializing_if = "Option::is_none")]
    pub last_activation_lte: Option<String>,
    /// User was last active after this timestamp.
    #[serde(rename = "lastActivation__gt", skip_serializing_if = "Option::is_none")]
    pub last_activation_gt: Option<String>,
    /// User was last active after or at this timestamp.
    #[serde(rename = "lastActivation__gte", skip_serializing_if = "Option::is_none")]
    pub last_activation_gte: Option<String>,
    /// Date range for when the user was last active (`<from>-<to>`, inclusive).
    #[serde(rename = "lastActivation__between", skip_serializing_if = "Option::is_none")]
    pub last_activation_between: Option<String>,
    /// API token expires before this timestamp.
    #[serde(rename = "apiTokenExpiresAt__lt", skip_serializing_if = "Option::is_none")]
    pub api_token_expires_at_lt: Option<String>,
    /// API token expires before or at this timestamp.
    #[serde(rename = "apiTokenExpiresAt__lte", skip_serializing_if = "Option::is_none")]
    pub api_token_expires_at_lte: Option<String>,
    /// API token expires after this timestamp.
    #[serde(rename = "apiTokenExpiresAt__gt", skip_serializing_if = "Option::is_none")]
    pub api_token_expires_at_gt: Option<String>,
    /// API token expires after or at this timestamp.
    #[serde(rename = "apiTokenExpiresAt__gte", skip_serializing_if = "Option::is_none")]
    pub api_token_expires_at_gte: Option<String>,
    /// Date range for when the API token expires (`<from>-<to>`, inclusive).
    #[serde(rename = "apiTokenExpiresAt__between", skip_serializing_if = "Option::is_none")]
    pub api_token_expires_at_between: Option<String>,
    /// User was created before this timestamp.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// User was created before or at this timestamp.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// User was created after this timestamp.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// User was created after or at this timestamp.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Date range for when the user was created (`<from>-<to>`, inclusive).
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
}

impl ListUsersQuery {
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: u32) -> Self {
        self.skip = Some(v);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: u32) -> Self {
        self.limit = Some(v);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// If true, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, the total item count is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort by. Allowed values: `id`, `createdAt`, `fullName`,
    /// `firstLogin`, `lastLogin`, `source`, `email`, `emailVerified`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// User Source. Allowed values: `mgmt`, `sso_saml`, `active_directory`, `global`.
    pub fn source(mut self, v: impl Into<String>) -> Self {
        self.source = Some(v.into());
        self
    }
    /// Source in (allowed values: `mgmt`, `sso_saml`, `active_directory`, `global`).
    pub fn sources<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sources = Some(join_csv(v));
        self
    }
    /// Email.
    pub fn email(mut self, v: impl Into<String>) -> Self {
        self.email = Some(v.into());
        self
    }
    /// Match email partially (substring).
    pub fn email_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.email_contains = Some(join_csv(v));
        self
    }
    /// True if email cannot be changed.
    pub fn email_read_only(mut self, v: bool) -> Self {
        self.email_read_only = Some(v);
        self
    }
    /// Full name.
    pub fn full_name(mut self, v: impl Into<String>) -> Self {
        self.full_name = Some(v.into());
        self
    }
    /// Match full name partially (substring).
    pub fn full_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.full_name_contains = Some(join_csv(v));
        self
    }
    /// True if full name cannot be changed.
    pub fn full_name_read_only(mut self, v: bool) -> Self {
        self.full_name_read_only = Some(v);
        self
    }
    /// First login (date-time string).
    pub fn first_login(mut self, v: impl Into<String>) -> Self {
        self.first_login = Some(v.into());
        self
    }
    /// Last login (date-time string).
    pub fn last_login(mut self, v: impl Into<String>) -> Self {
        self.last_login = Some(v.into());
        self
    }
    /// Date joined (date-time string).
    pub fn date_joined(mut self, v: impl Into<String>) -> Self {
        self.date_joined = Some(v.into());
        self
    }
    /// \[DEPRECATED\] True if permissions cannot be changed.
    pub fn groups_read_only(mut self, v: bool) -> Self {
        self.groups_read_only = Some(v);
        self
    }
    /// Two fa enabled.
    pub fn two_fa_enabled(mut self, v: bool) -> Self {
        self.two_fa_enabled = Some(v);
        self
    }
    /// Primary two fa method.
    pub fn primary_two_fa_method(mut self, v: impl Into<String>) -> Self {
        self.primary_two_fa_method = Some(v.into());
        self
    }
    /// Return only verified/unverified users.
    pub fn email_verified(mut self, v: bool) -> Self {
        self.email_verified = Some(v);
        self
    }
    /// Full text search for fields: full_name, email, description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// List of user IDs to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// List of rbac roles to filter by.
    pub fn role_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.role_ids = Some(join_csv(v));
        self
    }
    /// Two fa status.
    pub fn two_fa_status(mut self, v: impl Into<String>) -> Self {
        self.two_fa_status = Some(v.into());
        self
    }
    /// Two fa status in.
    pub fn two_fa_statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.two_fa_statuses = Some(join_csv(v));
        self
    }
    /// Can generate api token.
    pub fn can_generate_api_token(mut self, v: bool) -> Self {
        self.can_generate_api_token = Some(v);
        self
    }
    /// Has valid api token.
    pub fn has_valid_api_token(mut self, v: bool) -> Self {
        self.has_valid_api_token = Some(v);
        self
    }
    /// User was last active before this timestamp.
    pub fn last_activation_lt(mut self, v: impl Into<String>) -> Self {
        self.last_activation_lt = Some(v.into());
        self
    }
    /// User was last active before or at this timestamp.
    pub fn last_activation_lte(mut self, v: impl Into<String>) -> Self {
        self.last_activation_lte = Some(v.into());
        self
    }
    /// User was last active after this timestamp.
    pub fn last_activation_gt(mut self, v: impl Into<String>) -> Self {
        self.last_activation_gt = Some(v.into());
        self
    }
    /// User was last active after or at this timestamp.
    pub fn last_activation_gte(mut self, v: impl Into<String>) -> Self {
        self.last_activation_gte = Some(v.into());
        self
    }
    /// Date range for when the user was last active (`<from>-<to>`, inclusive).
    pub fn last_activation_between(mut self, v: impl Into<String>) -> Self {
        self.last_activation_between = Some(v.into());
        self
    }
    /// API token expires before this timestamp.
    pub fn api_token_expires_at_lt(mut self, v: impl Into<String>) -> Self {
        self.api_token_expires_at_lt = Some(v.into());
        self
    }
    /// API token expires before or at this timestamp.
    pub fn api_token_expires_at_lte(mut self, v: impl Into<String>) -> Self {
        self.api_token_expires_at_lte = Some(v.into());
        self
    }
    /// API token expires after this timestamp.
    pub fn api_token_expires_at_gt(mut self, v: impl Into<String>) -> Self {
        self.api_token_expires_at_gt = Some(v.into());
        self
    }
    /// API token expires after or at this timestamp.
    pub fn api_token_expires_at_gte(mut self, v: impl Into<String>) -> Self {
        self.api_token_expires_at_gte = Some(v.into());
        self
    }
    /// Date range for when the API token expires (`<from>-<to>`, inclusive).
    pub fn api_token_expires_at_between(mut self, v: impl Into<String>) -> Self {
        self.api_token_expires_at_between = Some(v.into());
        self
    }
    /// User was created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// User was created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// User was created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// User was created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Date range for when the user was created (`<from>-<to>`, inclusive).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/user` (User by token).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserByTokenQuery {
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl UserByTokenQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
}

/// Query params for `GET /web/api/v2.1/users/login/by-token` (Login by Token).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginByTokenQuery {
    /// User token. Required.
    pub token: String,
    /// Removed saved scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_saved_scope: Option<String>,
    /// Relative url to redirect to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to: Option<String>,
    /// Query params for the redirect to, without '?' prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to_params: Option<String>,
}

impl LoginByTokenQuery {
    /// Create the query with the required `token`.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            removed_saved_scope: None,
            redirect_to: None,
            redirect_to_params: None,
        }
    }
    /// Removed saved scope.
    pub fn removed_saved_scope(mut self, v: impl Into<String>) -> Self {
        self.removed_saved_scope = Some(v.into());
        self
    }
    /// Relative url to redirect to.
    pub fn redirect_to(mut self, v: impl Into<String>) -> Self {
        self.redirect_to = Some(v.into());
        self
    }
    /// Query params for the redirect to, without '?' prefix.
    pub fn redirect_to_params(mut self, v: impl Into<String>) -> Self {
        self.redirect_to_params = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/users/login/sso-saml2` (Redirect to SSO).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SsoSaml2Query {
    /// Email address of the user trying to log in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The scope the desired SSO IdP is configured on. `email` is ignored when set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl SsoSaml2Query {
    /// Email address of the user trying to log in.
    pub fn email(mut self, v: impl Into<String>) -> Self {
        self.email = Some(v.into());
        self
    }
    /// The scope the desired SSO IdP is configured on.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/users/onboarding/validate-token`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateTokenQuery {
    /// Verification token. Required.
    pub token: String,
    /// Reset password flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_password_flow: Option<bool>,
}

impl ValidateTokenQuery {
    /// Create the query with the required `token`.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            reset_password_flow: None,
        }
    }
    /// Reset password flow.
    pub fn reset_password_flow(mut self, v: bool) -> Self {
        self.reset_password_flow = Some(v);
        self
    }
}

// ===========================================================================
// Request bodies
// ===========================================================================

/// Body for `POST /web/api/v2.1/users` (Create User).
#[derive(Debug, Clone, Serialize)]
pub struct CreateUserBody {
    /// Request payload.
    pub data: CreateUserData,
}

/// Payload for [`CreateUserBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserData {
    /// The email of the user. Required.
    pub email: String,
    /// Full name of the user. Required.
    pub full_name: String,
    /// User scope. Required. Allowed values: `tenant`, `account`, `site`.
    pub scope: String,
    /// \[DEPRECATED\] Use roles instead. List of tenant roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_roles: Option<Vec<String>>,
    /// \[DEPRECATED\] Please use `scopeRoles` instead. Freeform objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_roles: Option<Vec<serde_json::Value>>,
    /// List of id and role id. Freeform objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_roles: Option<Vec<serde_json::Value>>,
    /// Two fa enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_enabled: Option<bool>,
    /// \[DEPRECATED\] Unused field. The user's role determines remote-shell access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_shell: Option<bool>,
    /// User password. Nullable. Not allowed if automatic onboarding is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// If true, credential auth type is forced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_credential_auth: Option<bool>,
}

impl CreateUserBody {
    /// Create with the required `email`, `full_name` and `scope`
    /// (`tenant`, `account` or `site`).
    pub fn new(
        email: impl Into<String>,
        full_name: impl Into<String>,
        scope: impl Into<String>,
    ) -> Self {
        Self {
            data: CreateUserData {
                email: email.into(),
                full_name: full_name.into(),
                scope: scope.into(),
                tenant_roles: None,
                site_roles: None,
                scope_roles: None,
                two_fa_enabled: None,
                allow_remote_shell: None,
                password: None,
                force_credential_auth: None,
            },
        }
    }
    /// \[DEPRECATED\] List of tenant roles.
    pub fn tenant_roles<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.data.tenant_roles = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// \[DEPRECATED\] Site roles (freeform objects).
    pub fn site_roles(mut self, v: Vec<serde_json::Value>) -> Self {
        self.data.site_roles = Some(v);
        self
    }
    /// Scope roles (freeform objects).
    pub fn scope_roles(mut self, v: Vec<serde_json::Value>) -> Self {
        self.data.scope_roles = Some(v);
        self
    }
    /// Two fa enabled.
    pub fn two_fa_enabled(mut self, v: bool) -> Self {
        self.data.two_fa_enabled = Some(v);
        self
    }
    /// \[DEPRECATED\] Unused field.
    pub fn allow_remote_shell(mut self, v: bool) -> Self {
        self.data.allow_remote_shell = Some(v);
        self
    }
    /// User password.
    pub fn password(mut self, v: impl Into<String>) -> Self {
        self.data.password = Some(v.into());
        self
    }
    /// Force credential auth.
    pub fn force_credential_auth(mut self, v: bool) -> Self {
        self.data.force_credential_auth = Some(v);
        self
    }
}

/// Body for `PUT /web/api/v2.1/users/{user_id}` (Update User).
#[derive(Debug, Clone, Serialize)]
pub struct UpdateUserBody {
    /// Request payload.
    pub data: UpdateUserData,
}

/// Payload for [`UpdateUserBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserData {
    /// User scope. Required. Allowed values: `tenant`, `account`, `site`.
    pub scope: String,
    /// Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// User password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Full name of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// \[DEPRECATED\] The email of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User password, new name for backward compatibility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    /// Two-Factor Authorization code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_code: Option<String>,
    /// Can generate api token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_generate_api_token: Option<bool>,
    /// \[DEPRECATED\] Use roles instead. List of tenant roles.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_roles: Option<Vec<String>>,
    /// \[DEPRECATED\] Please use `scopeRoles` instead. Freeform objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_roles: Option<Vec<serde_json::Value>>,
    /// List of id and role id. Freeform objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_roles: Option<Vec<serde_json::Value>>,
    /// Two fa enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_enabled: Option<bool>,
    /// \[DEPRECATED\] Unused field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remote_shell: Option<bool>,
}

impl UpdateUserBody {
    /// Create with the required `scope` (`tenant`, `account` or `site`).
    pub fn new(scope: impl Into<String>) -> Self {
        Self {
            data: UpdateUserData {
                scope: scope.into(),
                id: None,
                password: None,
                full_name: None,
                email: None,
                current_password: None,
                two_fa_code: None,
                can_generate_api_token: None,
                tenant_roles: None,
                site_roles: None,
                scope_roles: None,
                two_fa_enabled: None,
                allow_remote_shell: None,
            },
        }
    }
    /// Id.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.data.id = Some(v.into());
        self
    }
    /// User password.
    pub fn password(mut self, v: impl Into<String>) -> Self {
        self.data.password = Some(v.into());
        self
    }
    /// Full name of the user.
    pub fn full_name(mut self, v: impl Into<String>) -> Self {
        self.data.full_name = Some(v.into());
        self
    }
    /// \[DEPRECATED\] The email of the user.
    pub fn email(mut self, v: impl Into<String>) -> Self {
        self.data.email = Some(v.into());
        self
    }
    /// User password (current).
    pub fn current_password(mut self, v: impl Into<String>) -> Self {
        self.data.current_password = Some(v.into());
        self
    }
    /// Two-Factor Authorization code.
    pub fn two_fa_code(mut self, v: impl Into<String>) -> Self {
        self.data.two_fa_code = Some(v.into());
        self
    }
    /// Can generate api token.
    pub fn can_generate_api_token(mut self, v: bool) -> Self {
        self.data.can_generate_api_token = Some(v);
        self
    }
    /// \[DEPRECATED\] List of tenant roles.
    pub fn tenant_roles<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.data.tenant_roles = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// \[DEPRECATED\] Site roles (freeform objects).
    pub fn site_roles(mut self, v: Vec<serde_json::Value>) -> Self {
        self.data.site_roles = Some(v);
        self
    }
    /// Scope roles (freeform objects).
    pub fn scope_roles(mut self, v: Vec<serde_json::Value>) -> Self {
        self.data.scope_roles = Some(v);
        self
    }
    /// Two fa enabled.
    pub fn two_fa_enabled(mut self, v: bool) -> Self {
        self.data.two_fa_enabled = Some(v);
        self
    }
    /// \[DEPRECATED\] Unused field.
    pub fn allow_remote_shell(mut self, v: bool) -> Self {
        self.data.allow_remote_shell = Some(v);
        self
    }
}

/// Body wrapping a single user id (used by `2fa/enable`, `2fa/disable` and
/// `revoke-api-token`).
#[derive(Debug, Clone, Serialize)]
pub struct UserIdBody {
    /// Request payload.
    pub data: UserIdData,
}

/// Payload for [`UserIdBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdData {
    /// User ID. Required.
    pub id: String,
    /// Current password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    /// Two-Factor Authorization code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_code: Option<String>,
}

impl UserIdBody {
    /// Create with the required user `id`.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            data: UserIdData {
                id: id.into(),
                current_password: None,
                two_fa_code: None,
            },
        }
    }
    /// Current password.
    pub fn current_password(mut self, v: impl Into<String>) -> Self {
        self.data.current_password = Some(v.into());
        self
    }
    /// Two-Factor Authorization code.
    pub fn two_fa_code(mut self, v: impl Into<String>) -> Self {
        self.data.two_fa_code = Some(v.into());
        self
    }
}

/// Body wrapping a list of user ids (used by `enroll-2fa`).
#[derive(Debug, Clone, Serialize)]
pub struct UserIdsBody {
    /// Request payload.
    pub data: UserIdsData,
}

/// Payload for [`UserIdsBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdsData {
    /// A list of user ids. Required.
    pub ids: Vec<String>,
}

impl UserIdsBody {
    /// Create with the required list of user `ids`.
    pub fn new<I, S>(ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            data: UserIdsData {
                ids: ids.into_iter().map(Into::into).collect(),
            },
        }
    }
}

/// Body for `POST /web/api/v2.1/users/api-token-details` (API Token Details).
#[derive(Debug, Clone, Default, Serialize)]
pub struct ApiTokenBody {
    /// Request payload.
    pub data: ApiTokenData,
}

/// Payload for [`ApiTokenBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiTokenData {
    /// Api token. Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_token: Option<String>,
}

impl ApiTokenBody {
    /// New empty body.
    pub fn new() -> Self {
        Self::default()
    }
    /// Api token.
    pub fn api_token(mut self, v: impl Into<String>) -> Self {
        self.data.api_token = Some(v.into());
        self
    }
}

/// Body for `POST /web/api/v2.1/users/auth/app` (Auth App).
#[derive(Debug, Clone, Default, Serialize)]
pub struct AuthCodeBody {
    /// Request payload.
    pub data: AuthCodeData,
}

/// Payload for [`AuthCodeBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthCodeData {
    /// Code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// User should be remembered across sessions. Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember_me: Option<bool>,
}

impl AuthCodeBody {
    /// New empty body.
    pub fn new() -> Self {
        Self::default()
    }
    /// Code.
    pub fn code(mut self, v: impl Into<String>) -> Self {
        self.data.code = Some(v.into());
        self
    }
    /// User should be remembered across sessions.
    pub fn remember_me(mut self, v: bool) -> Self {
        self.data.remember_me = Some(v);
        self
    }
}

/// Body for `POST /web/api/v2.1/users/auth/elevate` (Auth Elevate).
#[derive(Debug, Clone, Serialize)]
pub struct ElevateSessionBody {
    /// Request payload.
    pub data: ElevateSessionData,
}

/// Payload for [`ElevateSessionBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ElevateSessionData {
    /// Code. Required.
    pub code: String,
}

impl ElevateSessionBody {
    /// Create with the required `code`.
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            data: ElevateSessionData { code: code.into() },
        }
    }
}

/// Body for `POST /web/api/v2.1/users/change-password` (Change Password).
#[derive(Debug, Clone, Serialize)]
pub struct ChangePasswordBody {
    /// Request payload.
    pub data: ChangePasswordData,
}

/// Payload for [`ChangePasswordBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordData {
    /// User ID. Required.
    pub id: String,
    /// Current password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    /// New password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    /// Confirm new password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_new_password: Option<String>,
    /// Two-Factor Authorization code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub two_fa_code: Option<String>,
}

impl ChangePasswordBody {
    /// Create with the required user `id`.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            data: ChangePasswordData {
                id: id.into(),
                current_password: None,
                new_password: None,
                confirm_new_password: None,
                two_fa_code: None,
            },
        }
    }
    /// Current password.
    pub fn current_password(mut self, v: impl Into<String>) -> Self {
        self.data.current_password = Some(v.into());
        self
    }
    /// New password.
    pub fn new_password(mut self, v: impl Into<String>) -> Self {
        self.data.new_password = Some(v.into());
        self
    }
    /// Confirm new password.
    pub fn confirm_new_password(mut self, v: impl Into<String>) -> Self {
        self.data.confirm_new_password = Some(v.into());
        self
    }
    /// Two-Factor Authorization code.
    pub fn two_fa_code(mut self, v: impl Into<String>) -> Self {
        self.data.two_fa_code = Some(v.into());
        self
    }
}

/// Body for `POST /web/api/v2.1/users/delete-2fa` (Delete 2FA).
#[derive(Debug, Clone, Serialize)]
pub struct DeleteTfaBody {
    /// Request payload.
    pub data: DeleteTfaData,
}

/// Payload for [`DeleteTfaBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteTfaData {
    /// A list of user ids. Required.
    pub ids: Vec<String>,
}

impl DeleteTfaBody {
    /// Create with the required list of user `ids`.
    pub fn new<I, S>(ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            data: DeleteTfaData {
                ids: ids.into_iter().map(Into::into).collect(),
            },
        }
    }
}

/// Body for `POST /web/api/v2.1/users/reset-2fa` (Reset 2FA).
#[derive(Debug, Clone, Serialize)]
pub struct ResetTfaBody {
    /// Request payload.
    pub data: ResetTfaData,
}

/// Payload for [`ResetTfaBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetTfaData {
    /// A list of user ids. Required.
    pub ids: Vec<String>,
    /// \[DEPRECATED\] Not used, deprecated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enroll: Option<bool>,
}

impl ResetTfaBody {
    /// Create with the required list of user `ids`.
    pub fn new<I, S>(ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            data: ResetTfaData {
                ids: ids.into_iter().map(Into::into).collect(),
                enroll: None,
            },
        }
    }
    /// \[DEPRECATED\] Not used, deprecated.
    pub fn enroll(mut self, v: bool) -> Self {
        self.data.enroll = Some(v);
        self
    }
}

/// Body for bulk user actions: `delete-users`,
/// `login/force-reset-password-on-login`, `login/send-reset-password-email` and
/// `onboarding/send-verification-email`.
#[derive(Debug, Clone, Serialize)]
pub struct BulkUsersActionBody {
    /// Filter selecting the users the action applies to. Required.
    ///
    /// Freeform object; accepts the same fields as [`ListUsersQuery`]
    /// (e.g. `{ "ids": ["1","2"] }`).
    pub filter: serde_json::Value,
    /// Optional action data. Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl BulkUsersActionBody {
    /// Create with the required `filter`.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { filter, data: None }
    }
    /// Optional action data.
    pub fn data(mut self, v: serde_json::Value) -> Self {
        self.data = Some(v);
        self
    }
}

/// Body for `POST /web/api/v2.1/users/enable-app` (Enable 2FA App).
#[derive(Debug, Clone, Default, Serialize)]
pub struct EnableAppBody {
    /// Request payload.
    pub data: EnableAppData,
}

/// Payload for [`EnableAppBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableAppData {
    /// Id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl EnableAppBody {
    /// New empty body.
    pub fn new() -> Self {
        Self::default()
    }
    /// Id.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.data.id = Some(v.into());
        self
    }
    /// Code.
    pub fn code(mut self, v: impl Into<String>) -> Self {
        self.data.code = Some(v.into());
        self
    }
}

/// Body for `POST /web/api/v2.1/users/generate-api-token` (Generate API Token).
#[derive(Debug, Clone, Default, Serialize)]
pub struct GenerateApiTokenBody {
    /// Request payload.
    pub data: GenerateApiTokenData,
}

/// Payload for [`GenerateApiTokenBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateApiTokenData {
    /// Temporary WA flag: if true the legacy token is generated even if the
    /// auth_tokens global switch is on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_legacy: Option<bool>,
}

impl GenerateApiTokenBody {
    /// New empty body.
    pub fn new() -> Self {
        Self::default()
    }
    /// Force legacy token generation.
    pub fn force_legacy(mut self, v: bool) -> Self {
        self.data.force_legacy = Some(v);
        self
    }
}

/// Body for `POST /web/api/v2.1/users/generate-iframe-token` (Generate iFrame Token).
#[derive(Debug, Clone, Serialize)]
pub struct CreateIFrameUserBody {
    /// Request payload.
    pub data: CreateIFrameUserData,
}

/// Payload for [`CreateIFrameUserBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIFrameUserData {
    /// Account id. Required.
    pub account_id: String,
    /// \[DEPRECATED\] Name of the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// RBAC role name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// The username that will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// A list of included UUIDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuids: Option<Vec<String>>,
}

impl CreateIFrameUserBody {
    /// Create with the required `account_id`.
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            data: CreateIFrameUserData {
                account_id: account_id.into(),
                role: None,
                role_name: None,
                user_name: None,
                agent_uuids: None,
            },
        }
    }
    /// \[DEPRECATED\] Name of the role.
    pub fn role(mut self, v: impl Into<String>) -> Self {
        self.data.role = Some(v.into());
        self
    }
    /// RBAC role name.
    pub fn role_name(mut self, v: impl Into<String>) -> Self {
        self.data.role_name = Some(v.into());
        self
    }
    /// The username that will be displayed.
    pub fn user_name(mut self, v: impl Into<String>) -> Self {
        self.data.user_name = Some(v.into());
        self
    }
    /// A list of included UUIDs.
    pub fn agent_uuids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.data.agent_uuids = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

/// Body for `POST /web/api/v2.1/users/login` (Login). Sent flat (no `data` wrapper).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginInputBody {
    /// Username. Required.
    pub username: String,
    /// Password. Required.
    pub password: String,
    /// Remember me. Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remember_me: Option<bool>,
}

impl LoginInputBody {
    /// Create with the required `username` and `password`.
    pub fn new(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
            remember_me: None,
        }
    }
    /// Remember me.
    pub fn remember_me(mut self, v: bool) -> Self {
        self.remember_me = Some(v);
        self
    }
}

/// Body for `POST /web/api/v2.1/users/login-continue` (Continue login).
#[derive(Debug, Clone, Serialize)]
pub struct LoginContinueBody {
    /// Request payload.
    pub data: LoginContinueData,
}

/// Payload for [`LoginContinueBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginContinueData {
    /// Temporary JWT. Required.
    pub token: String,
    /// Indicates if the user wants to change the password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_password: Option<bool>,
    /// Indicates if the user wants to skip the continue next time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dont_show_again: Option<bool>,
}

impl LoginContinueBody {
    /// Create with the required `token`.
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            data: LoginContinueData {
                token: token.into(),
                reset_password: None,
                dont_show_again: None,
            },
        }
    }
    /// Indicates if the user wants to change the password.
    pub fn reset_password(mut self, v: bool) -> Self {
        self.data.reset_password = Some(v);
        self
    }
    /// Indicates if the user wants to skip the continue next time.
    pub fn dont_show_again(mut self, v: bool) -> Self {
        self.data.dont_show_again = Some(v);
        self
    }
}

/// Body for `POST /web/api/v2.1/users/login/by-api-token` (Login by API Token).
#[derive(Debug, Clone, Default, Serialize)]
pub struct LoginByApiTokenBody {
    /// Request payload.
    pub data: LoginByApiTokenData,
}

/// Payload for [`LoginByApiTokenBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginByApiTokenData {
    /// Api token. Nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_token: Option<String>,
    /// When logging in from Atlas, specifies the login reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl LoginByApiTokenBody {
    /// New empty body.
    pub fn new() -> Self {
        Self::default()
    }
    /// Api token.
    pub fn api_token(mut self, v: impl Into<String>) -> Self {
        self.data.api_token = Some(v.into());
        self
    }
    /// Login reason.
    pub fn reason(mut self, v: impl Into<String>) -> Self {
        self.data.reason = Some(v.into());
        self
    }
}

/// Body for `POST /web/api/v2.1/users/login/set-password` (Set a New Password).
#[derive(Debug, Clone, Serialize)]
pub struct SetPasswordBody {
    /// Request payload.
    pub data: SetPasswordData,
}

/// Payload for [`SetPasswordBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetPasswordData {
    /// The new password. Required.
    pub password: String,
    /// Verification token. Required.
    pub token: String,
}

impl SetPasswordBody {
    /// Create with the required `password` and `token`.
    pub fn new(password: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            data: SetPasswordData {
                password: password.into(),
                token: token.into(),
            },
        }
    }
}

/// Body for `POST /web/api/v2.1/users/onboarding/verify` (Email Verification).
#[derive(Debug, Clone, Serialize)]
pub struct OnboardingVerificationBody {
    /// Request payload.
    pub data: OnboardingVerificationData,
}

/// Payload for [`OnboardingVerificationBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingVerificationData {
    /// User selected password. Required.
    pub password: String,
    /// Verification token. Required.
    pub token: String,
    /// Reset password flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_password_flow: Option<bool>,
}

impl OnboardingVerificationBody {
    /// Create with the required `password` and `token`.
    pub fn new(password: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            data: OnboardingVerificationData {
                password: password.into(),
                token: token.into(),
                reset_password_flow: None,
            },
        }
    }
    /// Reset password flow.
    pub fn reset_password_flow(mut self, v: bool) -> Self {
        self.data.reset_password_flow = Some(v);
        self
    }
}

/// Body for `POST /web/api/v2.1/users/request-app` (Request 2FA App). Sent flat.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestAppBody {
    /// Current password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
}

impl RequestAppBody {
    /// New empty body.
    pub fn new() -> Self {
        Self::default()
    }
    /// Current password.
    pub fn current_password(mut self, v: impl Into<String>) -> Self {
        self.current_password = Some(v.into());
        self
    }
}

// ===========================================================================
// Service
// ===========================================================================

impl UsersService<'_> {
    /// `GET /web/api/v2.1/export/users` — Export Users.
    ///
    /// Export User data to a CSV, for Users that match the filter. The endpoint
    /// returns CSV content rather than the JSON envelope.
    pub async fn export_users(
        &self,
        query: &ExportUsersQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/export/users", q).await?)
    }

    /// `GET /web/api/v2.1/user` — User by token.
    ///
    /// Get a user by token.
    pub async fn user_by_token(&self, query: &UserByTokenQuery) -> Result<Response<User>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/user", q).await?)
    }

    /// `GET /web/api/v2.1/users` — List users.
    ///
    /// Get a list of users.
    pub async fn list(&self, query: &ListUsersQuery) -> Result<Paginated<User>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/users", q).await?)
    }

    /// `POST /web/api/v2.1/users` — Create User.
    ///
    /// Create a new user.
    pub async fn create(&self, body: &CreateUserBody) -> Result<Response<User>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users", body).await?)
    }

    /// `POST /web/api/v2.1/users/2fa/disable` — Disable 2FA.
    ///
    /// Disable Two-Factor Authentication for one user.
    pub async fn disable_2fa(&self, body: &UserIdBody) -> Result<Response<SuccessResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/2fa/disable", body).await?)
    }

    /// `POST /web/api/v2.1/users/2fa/enable` — Enable 2FA.
    ///
    /// Enable two-factor authentication for a given user.
    pub async fn enable_2fa(&self, body: &UserIdBody) -> Result<Response<SuccessResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/2fa/enable", body).await?)
    }

    /// `POST /web/api/v2.1/users/api-token-details` — API Token Details.
    ///
    /// Get details of the API token that matches the filter.
    pub async fn api_token_details(
        &self,
        body: &ApiTokenBody,
    ) -> Result<Response<ApiTokenDetail>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/api-token-details", body).await?)
    }

    /// `POST /web/api/v2.1/users/auth/app` — Auth App.
    ///
    /// Authenticate a user with a third-party app (e.g. DUO or Google
    /// Authenticator) for deployments that require Two Factor Authentication.
    pub async fn auth_app(&self, body: &AuthCodeBody) -> Result<Response<LoginOutput>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/auth/app", body).await?)
    }

    /// `POST /web/api/v2.1/users/auth/elevate` — Auth Elevate.
    ///
    /// Elevate a session with a third-party app, such as DUO or Google
    /// Authenticator.
    pub async fn auth_elevate(
        &self,
        body: &ElevateSessionBody,
    ) -> Result<Response<ElevateSessionResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/auth/elevate", body).await?)
    }

    /// `POST /web/api/v2.1/users/auth/eula` — Sign EULA.
    ///
    /// Mark the End User License Agreement (EULA) as signed for user scopes.
    pub async fn auth_eula(&self) -> Result<Response<SuccessResponse>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<(), _>(Method::POST, "/web/api/v2.1/users/auth/eula", None, None)
            .await?)
    }

    /// `POST /web/api/v2.1/users/change-password` — Change Password.
    ///
    /// Change the user password.
    pub async fn change_password(
        &self,
        body: &ChangePasswordBody,
    ) -> Result<Response<SuccessResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/change-password", body).await?)
    }

    /// `POST /web/api/v2.1/users/delete-2fa` — Delete 2FA.
    ///
    /// Delete 2FA for users.
    pub async fn delete_2fa(&self, body: &DeleteTfaBody) -> Result<Response<AffectedResults>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/delete-2fa", body).await?)
    }

    /// `POST /web/api/v2.1/users/delete-users` — Bulk Delete Users.
    ///
    /// Delete all users that match the filter.
    pub async fn delete_users(
        &self,
        body: &BulkUsersActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/delete-users", body).await?)
    }

    /// `POST /web/api/v2.1/users/enable-app` — Enable 2FA App.
    ///
    /// Enable support for the 2FA app (such as Duo or Google Authenticator) that
    /// your Console users will use to log in.
    pub async fn enable_app(&self, body: &EnableAppBody) -> Result<Response<SuccessResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/enable-app", body).await?)
    }

    /// `POST /web/api/v2.1/users/enroll-2fa` — Enroll 2FA.
    ///
    /// Enroll users for 2FA setup.
    pub async fn enroll_2fa(&self, body: &UserIdsBody) -> Result<Response<EnrollTfaResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/enroll-2fa", body).await?)
    }

    /// `POST /web/api/v2.1/users/generate-api-token` — Generate API Token.
    ///
    /// Get the API token for the authenticated user.
    pub async fn generate_api_token(
        &self,
        body: &GenerateApiTokenBody,
    ) -> Result<Response<GeneratedApiToken>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/generate-api-token", body).await?)
    }

    /// `POST /web/api/v2.1/users/generate-iframe-token` — Generate iFrame Token.
    ///
    /// Get a new iFrame token with the provided limitations.
    pub async fn generate_iframe_token(
        &self,
        body: &CreateIFrameUserBody,
    ) -> Result<Response<CreateIFrameToken>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/generate-iframe-token", body).await?)
    }

    /// `POST /web/api/v2.1/users/login` — Login.
    ///
    /// Authenticate a user by username and password and return an authentication
    /// token. Rate limit: 1 call per second per IP address.
    pub async fn login(&self, body: &LoginInputBody) -> Result<Response<LoginOutput>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/login", body).await?)
    }

    /// `POST /web/api/v2.1/users/login-continue` — Continue login.
    ///
    /// Continue a login flow due to an upcoming password expiration or SSO 2FA
    /// setup.
    pub async fn login_continue(
        &self,
        body: &LoginContinueBody,
    ) -> Result<Response<LoginContinueResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/login-continue", body).await?)
    }

    /// `POST /web/api/v2.1/users/login/by-api-token` — Login by API Token.
    ///
    /// Log in to the API with a token.
    pub async fn login_by_api_token(
        &self,
        body: &LoginByApiTokenBody,
    ) -> Result<Response<TokenResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/login/by-api-token", body).await?)
    }

    /// `GET /web/api/v2.1/users/login/by-token` — Login by Token.
    ///
    /// Log in with a user token. Returns a redirect rather than the JSON
    /// envelope.
    pub async fn login_by_token(
        &self,
        query: &LoginByTokenQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/users/login/by-token", q).await?)
    }

    /// `POST /web/api/v2.1/users/login/force-reset-password-on-login` — Reset
    /// password on next login.
    ///
    /// Force users to reset their password on next login.
    pub async fn force_reset_password_on_login(
        &self,
        body: &BulkUsersActionBody,
    ) -> Result<Response<AffectedResults>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/users/login/force-reset-password-on-login", body)
            .await?)
    }

    /// `POST /web/api/v2.1/users/login/send-reset-password-email` — Prompt reset
    /// password.
    ///
    /// Prompt reset password for users.
    pub async fn send_reset_password_email(
        &self,
        body: &BulkUsersActionBody,
    ) -> Result<Response<AffectedResults>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/users/login/send-reset-password-email", body)
            .await?)
    }

    /// `POST /web/api/v2.1/users/login/set-password` — Set a New Password.
    ///
    /// Sets a new password for the user.
    pub async fn set_password(
        &self,
        body: &SetPasswordBody,
    ) -> Result<Response<SetPasswordResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/login/set-password", body).await?)
    }

    /// `GET /web/api/v2.1/users/login/sso-saml2` — Redirect to SSO.
    ///
    /// If SSO is enabled for a deployment or scope, redirects the login to SSO.
    /// Returns a redirect rather than the JSON envelope.
    pub async fn login_sso_saml2(
        &self,
        query: &SsoSaml2Query,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/users/login/sso-saml2", q).await?)
    }

    /// `POST /web/api/v2.1/users/login/sso-saml2/{scope_id}` — Auth by SSO.
    ///
    /// Authenticate a Single Sign-On response over SAML v2 protocol. Returns a
    /// redirect rather than the JSON envelope.
    pub async fn auth_by_sso(
        &self,
        scope_id: impl Into<String>,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/users/login/sso-saml2/{}", scope_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), _>(Method::POST, &path, None, None)
            .await?)
    }

    /// `POST /web/api/v2.1/users/logout` — Logout.
    ///
    /// Log out the authenticated user.
    pub async fn logout(&self) -> Result<Response<LogoutResponse>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<(), _>(Method::POST, "/web/api/v2.1/users/logout", None, None)
            .await?)
    }

    /// `POST /web/api/v2.1/users/onboarding/send-verification-email` — Send
    /// Verification Email.
    ///
    /// Send a verification email to users that match the filter.
    pub async fn send_verification_email(
        &self,
        body: &BulkUsersActionBody,
    ) -> Result<Response<AffectedResults>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/users/onboarding/send-verification-email", body)
            .await?)
    }

    /// `GET /web/api/v2.1/users/onboarding/validate-token` — Validate
    /// Verification Token.
    ///
    /// Validate a verification token received when a new user verifies their
    /// email.
    pub async fn validate_onboarding_token(
        &self,
        query: &ValidateTokenQuery,
    ) -> Result<Response<SuccessResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/users/onboarding/validate-token", q).await?)
    }

    /// `POST /web/api/v2.1/users/onboarding/verify` — Email Verification.
    ///
    /// Verify the onboarding token and set a new password.
    pub async fn verify_onboarding(
        &self,
        body: &OnboardingVerificationBody,
    ) -> Result<Response<SuccessResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/onboarding/verify", body).await?)
    }

    /// `POST /web/api/v2.1/users/request-app` — Request 2FA App.
    ///
    /// Request 2FA App response.
    pub async fn request_app(
        &self,
        body: &RequestAppBody,
    ) -> Result<Response<RequestAppResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/request-app", body).await?)
    }

    /// `POST /web/api/v2.1/users/reset-2fa` — Reset 2FA.
    ///
    /// Reset 2FA for users.
    pub async fn reset_2fa(&self, body: &ResetTfaBody) -> Result<Response<AffectedResults>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/reset-2fa", body).await?)
    }

    /// `POST /web/api/v2.1/users/revoke-api-token` — Revoke API Token.
    ///
    /// Revoke an API token.
    pub async fn revoke_api_token(
        &self,
        body: &UserIdBody,
    ) -> Result<Response<SuccessResponse>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/users/revoke-api-token", body).await?)
    }

    /// `GET /web/api/v2.1/users/rs-auth-check` — Check Remote Shell Permissions.
    ///
    /// See if the logged in user is allowed to use Remote Shell.
    pub async fn rs_auth_check(&self) -> Result<Response<SuccessResponse>, Error> {
        Ok(self.client.http().get("/web/api/v2.1/users/rs-auth-check", None).await?)
    }

    /// `GET /web/api/v2.1/users/sso-saml2/re-auth` — Redirect to SSO for
    /// re-authentication.
    ///
    /// Initiates re-authentication with the user's identity provider. Returns a
    /// redirect rather than the JSON envelope.
    pub async fn sso_saml2_re_auth(&self) -> Result<Response<serde_json::Value>, Error> {
        Ok(self.client.http().get("/web/api/v2.1/users/sso-saml2/re-auth", None).await?)
    }

    /// `GET /web/api/v2.1/users/tenant-admin-auth-check` — Check Global User.
    ///
    /// See if the logged in user is a user with the Global scope of access.
    pub async fn tenant_admin_auth_check(&self) -> Result<Response<SuccessResponse>, Error> {
        Ok(self.client.http().get("/web/api/v2.1/users/tenant-admin-auth-check", None).await?)
    }

    /// `GET /web/api/v2.1/users/viewer-auth-check` — Check Viewer.
    ///
    /// See if the logged in user has only viewer permissions.
    pub async fn viewer_auth_check(&self) -> Result<Response<SuccessResponse>, Error> {
        Ok(self.client.http().get("/web/api/v2.1/users/viewer-auth-check", None).await?)
    }

    /// `DELETE /web/api/v2.1/users/{user_id}` — Delete User.
    ///
    /// Delete a user by ID.
    pub async fn delete(
        &self,
        user_id: impl Into<String>,
    ) -> Result<Response<SuccessResponse>, Error> {
        let path = format!("/web/api/v2.1/users/{}", user_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), _>(Method::DELETE, &path, None, None)
            .await?)
    }

    /// `GET /web/api/v2.1/users/{user_id}` — Get User.
    ///
    /// Get a user by ID.
    pub async fn get(&self, user_id: impl Into<String>) -> Result<Response<User>, Error> {
        let path = format!("/web/api/v2.1/users/{}", user_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `PUT /web/api/v2.1/users/{user_id}` — Update User.
    ///
    /// Change properties of the user of the given ID.
    pub async fn update(
        &self,
        user_id: impl Into<String>,
        body: &UpdateUserBody,
    ) -> Result<Response<User>, Error> {
        let path = format!("/web/api/v2.1/users/{}", user_id.into());
        Ok(self
            .client
            .http()
            .request_json::<UpdateUserBody, _>(Method::PUT, &path, None, Some(body))
            .await?)
    }

    /// `GET /web/api/v2.1/users/{user_id}/api-token-details` — API Token by User ID.
    ///
    /// Get the details of the API token generated for a given user.
    pub async fn api_token_details_by_user(
        &self,
        user_id: impl Into<String>,
    ) -> Result<Response<ApiTokenDetail>, Error> {
        let path = format!("/web/api/v2.1/users/{}/api-token-details", user_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }
}
