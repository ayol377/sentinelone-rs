use sentinelone_http::Method;
use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::rbac::*;
use crate::pagination::{Paginated, Response};

/// `RBAC` tag.
///
/// RBAC APIs — manage Role-Based Access Control roles: list roles, fetch a new
/// role template, create / read / update / delete a specific role.
pub struct RbacService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/rbac/role` — Get template for new role.
///
/// Array params (`accountIds`, `siteIds`, `groupIds`) are serialized
/// comma-joined, as the API expects. Every field is optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleTemplateQuery {
    /// List of Account IDs to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

impl RoleTemplateQuery {
    /// List of Account IDs to filter by (comma-joined).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Site IDs to filter by (comma-joined).
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by (comma-joined).
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
}

/// Query params for `GET /web/api/v2.1/rbac/role/{role_id}` —
/// Get Specific Role Definition. Every field is optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleQuery {
    /// List of Account IDs to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Return RBAC role matching the name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Return RBAC roles created before this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Return RBAC roles created after this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Return RBAC roles created before or at this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Return RBAC roles created after or at this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Return RBAC roles created within this range (inclusive).
    /// Example: `1514978764288-1514978999999`. Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// Return RBAC roles updated before this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at_lt: Option<String>,
    /// Return RBAC roles updated after this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at_gt: Option<String>,
    /// Return RBAC roles updated before or at this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// Return RBAC roles updated after or at this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// Return RBAC roles updated within this range (inclusive).
    /// Example: `1514978764288-1514978999999`. Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at_between: Option<String>,
    /// Free text search on role name, and description. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

impl RoleQuery {
    /// List of Account IDs to filter by (comma-joined).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Site IDs to filter by (comma-joined).
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by (comma-joined).
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Return RBAC role matching the name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// Return RBAC roles created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Return RBAC roles created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Return RBAC roles created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Return RBAC roles created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Return RBAC roles created within this range (inclusive).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// Return RBAC roles updated before this timestamp.
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lt = Some(v.into());
        self
    }
    /// Return RBAC roles updated after this timestamp.
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gt = Some(v.into());
        self
    }
    /// Return RBAC roles updated before or at this timestamp.
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// Return RBAC roles updated after or at this timestamp.
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// Return RBAC roles updated within this range (inclusive).
    pub fn updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.updated_at_between = Some(v.into());
        self
    }
    /// Free text search on role name, and description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/rbac/roles` — Get All Roles.
/// Every field is optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RolesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: `150`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: `10`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: `YWdlbnRfaWQ6NTgwMjkzODE=`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If `true`, only the total number of items will be returned, without any
    /// of the actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If `true`, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Allowed values: `id`, `description`,
    /// `usersInRoles`, `creator`, `siteName`, `createdAt`, `updatedAt`,
    /// `updatedBy`. Example: `id`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Example: `asc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Return RBAC role matching the name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Return RBAC roles created before this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Return RBAC roles created after this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Return RBAC roles created before or at this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Return RBAC roles created after or at this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Return RBAC roles created within this range (inclusive).
    /// Example: `1514978764288-1514978999999`. Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// Return RBAC roles updated before this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at_lt: Option<String>,
    /// Return RBAC roles updated after this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at_gt: Option<String>,
    /// Return RBAC roles updated before or at this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// Return RBAC roles updated after or at this timestamp.
    /// Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// Return RBAC roles updated within this range (inclusive).
    /// Example: `1514978764288-1514978999999`. Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at_between: Option<String>,
    /// Free text search on role name, and description. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// List of ids to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of Tenancies IDs to filter by.
    /// Example: `225494730938493804,225494730938493915`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenancy_ids: Option<String>,
    /// Email of the creating user. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// Id of the creating user. Example: `225494730938493804`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// Email of the updating user. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    /// Id of the updating user. Example: `225494730938493804`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_by_id: Option<String>,
    /// Created at. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Updated at. Example: `2018-02-27T04:49:26.257525Z`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Description. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Name of the account that contains the role. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// Name of the site that contains the role. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    /// Include parent scopes roles. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Include child scopes roles. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// Filter only system/custom roles. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_role: Option<bool>,
}

impl RolesQuery {
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// If `true`, only the total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If `true`, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `description`,
    /// `usersInRoles`, `creator`, `siteName`, `createdAt`, `updatedAt`,
    /// `updatedBy`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Account IDs to filter by (comma-joined).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// List of Site IDs to filter by (comma-joined).
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// List of Group IDs to filter by (comma-joined).
    pub fn group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(ids));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Return RBAC role matching the name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// Return RBAC roles created before this timestamp.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Return RBAC roles created after this timestamp.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Return RBAC roles created before or at this timestamp.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Return RBAC roles created after or at this timestamp.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Return RBAC roles created within this range (inclusive).
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// Return RBAC roles updated before this timestamp.
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lt = Some(v.into());
        self
    }
    /// Return RBAC roles updated after this timestamp.
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gt = Some(v.into());
        self
    }
    /// Return RBAC roles updated before or at this timestamp.
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// Return RBAC roles updated after or at this timestamp.
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// Return RBAC roles updated within this range (inclusive).
    pub fn updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.updated_at_between = Some(v.into());
        self
    }
    /// Free text search on role name, and description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// List of ids to filter by (comma-joined).
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// List of Tenancies IDs to filter by (comma-joined).
    pub fn tenancy_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tenancy_ids = Some(join_csv(ids));
        self
    }
    /// Email of the creating user.
    pub fn creator(mut self, v: impl Into<String>) -> Self {
        self.creator = Some(v.into());
        self
    }
    /// Id of the creating user.
    pub fn creator_id(mut self, v: impl Into<String>) -> Self {
        self.creator_id = Some(v.into());
        self
    }
    /// Email of the updating user.
    pub fn updated_by(mut self, v: impl Into<String>) -> Self {
        self.updated_by = Some(v.into());
        self
    }
    /// Id of the updating user.
    pub fn updated_by_id(mut self, v: impl Into<String>) -> Self {
        self.updated_by_id = Some(v.into());
        self
    }
    /// Created at.
    pub fn created_at(mut self, v: impl Into<String>) -> Self {
        self.created_at = Some(v.into());
        self
    }
    /// Updated at.
    pub fn updated_at(mut self, v: impl Into<String>) -> Self {
        self.updated_at = Some(v.into());
        self
    }
    /// Description.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// Name of the account that contains the role.
    pub fn account_name(mut self, v: impl Into<String>) -> Self {
        self.account_name = Some(v.into());
        self
    }
    /// Name of the site that contains the role.
    pub fn site_name(mut self, v: impl Into<String>) -> Self {
        self.site_name = Some(v.into());
        self
    }
    /// Include parent scopes roles.
    pub fn include_parents(mut self, v: bool) -> Self {
        self.include_parents = Some(v);
        self
    }
    /// Include child scopes roles.
    pub fn include_children(mut self, v: bool) -> Self {
        self.include_children = Some(v);
        self
    }
    /// Filter only system/custom roles.
    pub fn predefined_role(mut self, v: bool) -> Self {
        self.predefined_role = Some(v);
        self
    }
}

/// The `data` object of a create/update role request body.
///
/// `name` and `description` are **required** by the spec.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleData {
    /// Permission ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_ids: Option<Vec<String>>,
    /// Role name (1-255 chars). Recommendation: Use a prefix or suffix for each
    /// role that identifies it as related to a specific Account or Site.
    /// **Required.**
    pub name: String,
    /// Description (max 255 chars). **Required.**
    pub description: String,
}

/// The `filter` object of a create/update role request body. Scope selector.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleFilter {
    /// List of Account IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Group IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
}

/// Request body for `POST /web/api/v2.1/rbac/role` — Create new role
/// (`rbac.schemas_RbacCreateRoleSchema`). Both `data` and `filter` are
/// **required**.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoleBody {
    /// Role data (permissions, name, description). **Required.**
    pub data: RoleData,
    /// Scope filter. **Required.**
    pub filter: RoleFilter,
}

/// Request body for `PUT /web/api/v2.1/rbac/role/{role_id}` — Update role
/// (`rbac.schemas_RbacUpdateRoleSchema`). `data` is **required**; `filter`
/// is optional.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoleBody {
    /// Role data (permissions, name, description). **Required.**
    pub data: RoleData,
    /// Scope filter. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<RoleFilter>,
}

/// The `data` object of a delete role request body.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRoleData {
    /// Role ID of new role to assign to users with the deleted role.
    /// Example: `225494730938493804`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

/// Request body for `DELETE /web/api/v2.1/rbac/role/{role_id}` — Delete role
/// (`rbac.schemas_RbacDeleteRoleSchema`). `data` is **required**.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRoleBody {
    /// Delete data (optional reassignment target). **Required.**
    pub data: DeleteRoleData,
}

/// Joins an iterator of string-like items by comma, as the API expects for
/// array query params.
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

impl RbacService<'_> {
    /// `GET /web/api/v2.1/rbac/role` — Get template for new role.
    ///
    /// Get the template for a new role.
    pub async fn role_template(
        &self,
        query: &RoleTemplateQuery,
    ) -> Result<Response<NewRoleTemplate>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/rbac/role", q).await?)
    }

    /// `POST /web/api/v2.1/rbac/role` — Create new role.
    ///
    /// Create a new role for Role-Based Access Control (RBAC).
    pub async fn create_role(
        &self,
        body: &CreateRoleBody,
    ) -> Result<Response<RolePermissions>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/rbac/role", body).await?)
    }

    /// `DELETE /web/api/v2.1/rbac/role/{role_id}` — Delete role.
    ///
    /// With the ID of a role (see Get All Roles), you can delete a role. If
    /// there are users assigned to the role, specify the ID of their new role.
    ///
    /// `role_id` — Role ID. Example: `225494730938493804`. **Required.**
    pub async fn delete_role(
        &self,
        role_id: impl Into<String>,
        body: &DeleteRoleBody,
    ) -> Result<Response<SuccessResponse>, Error> {
        let path = format!("/web/api/v2.1/rbac/role/{}", role_id.into());
        Ok(self
            .client
            .http()
            .request_json::<DeleteRoleBody, Response<SuccessResponse>>(
                Method::DELETE,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/rbac/role/{role_id}` — Get Specific Role Definition.
    ///
    /// With the ID of a role (see Get All Roles) you can see the permissions of
    /// that role. The definition of a role can change in different scopes and
    /// SKUs. For example, an Admin role with the scope access of a Site does
    /// not have Network Discovery permissions, but an IT role with the scope
    /// access of an Account with a Network Discovery license does have
    /// permissions on Network Discovery. The Response shows role permissions to
    /// see views in the WebUI and to use Console features.
    ///
    /// `role_id` — Role ID. Example: `225494730938493804`. **Required.**
    pub async fn get_role(
        &self,
        role_id: impl Into<String>,
        query: &RoleQuery,
    ) -> Result<Response<RolePermissions>, Error> {
        let path = format!("/web/api/v2.1/rbac/role/{}", role_id.into());
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `PUT /web/api/v2.1/rbac/role/{role_id}` — Update role.
    ///
    /// With the ID of a role (see Get All Roles), you can update the
    /// permissions of users with this role.
    ///
    /// `role_id` — Role ID. Example: `225494730938493804`. **Required.**
    pub async fn update_role(
        &self,
        role_id: impl Into<String>,
        body: &UpdateRoleBody,
    ) -> Result<Response<RolePermissions>, Error> {
        let path = format!("/web/api/v2.1/rbac/role/{}", role_id.into());
        Ok(self
            .client
            .http()
            .request_json::<UpdateRoleBody, Response<RolePermissions>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/rbac/roles` — Get All Roles.
    ///
    /// See roles assigned to users that match the filter, a basic description
    /// of the roles, and the number of users for each role. Role-Based Access
    /// Control (RBAC) has predefined roles. (Currently, customized roles are
    /// not supported.) This command gives the ID of the role, which you can use
    /// in other commands.
    pub async fn list(&self, query: &RolesQuery) -> Result<Paginated<Role>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/rbac/roles", q).await?)
    }
}
