//! Models for the `RBAC` tag.
//!
//! Hand-written for 1:1 spec parity with `swagger_2_1.json`. None of the
//! response `data` entities in this tag declare a `required` array on their
//! item-level properties, so every field defaults to `Option<T>` (the
//! "default null" behaviour). Fields explicitly marked `x-nullable` are also
//! `Option<T>`.

use serde::Deserialize;

/// A flattened RBAC role with account/site name, as returned by
/// `GET /web/api/v2.1/rbac/roles` (`rbac.schemas_FlatRoleWithAccountOrSiteName`).
///
/// Each entry describes a role assigned to users matching the filter, a basic
/// description of the role, and the number of users for each role.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    /// Role ID.
    pub id: Option<String>,
    /// Role name.
    pub name: Option<String>,
    /// Role description.
    pub description: Option<String>,
    /// Created at (ISO-8601 date/time string). Example: `2018-02-27T04:49:26.257525Z`.
    pub created_at: Option<String>,
    /// Updated at (ISO-8601 date/time string). Example: `2018-02-27T04:49:26.257525Z`.
    pub updated_at: Option<String>,
    /// Email of the creating user.
    pub creator: Option<String>,
    /// Id of the creating user.
    pub creator_id: Option<String>,
    /// Email of the updating user.
    pub updated_by: Option<String>,
    /// Id of the updating user.
    pub updated_by_id: Option<String>,
    /// How many users use this role.
    pub users_in_roles: Option<i64>,
    /// Id of the containing scope.
    pub scope_id: Option<String>,
    /// Scope of the role. Allowed values: `Group`, `Site`, `Account`, `Tenant`.
    pub scope: Option<String>,
    /// Whether this role is a system role.
    pub predefined_role: Option<bool>,
    /// Account name.
    pub account_name: Option<String>,
    /// Site name.
    pub site_name: Option<String>,
}

/// Full role definition including page-level permissions, as returned by
/// `GET`/`PUT` `/web/api/v2.1/rbac/role/{role_id}` and `POST /web/api/v2.1/rbac/role`
/// (`rbac.schemas_RolePermissions`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolePermissions {
    /// Role ID.
    pub id: Option<String>,
    /// Role name.
    pub name: Option<String>,
    /// Role description.
    pub description: Option<String>,
    /// Created at (ISO-8601 date/time string). Example: `2018-02-27T04:49:26.257525Z`.
    pub created_at: Option<String>,
    /// Updated at (ISO-8601 date/time string). Example: `2018-02-27T04:49:26.257525Z`.
    pub updated_at: Option<String>,
    /// Email of the creating user.
    pub creator: Option<String>,
    /// Id of the creating user.
    pub creator_id: Option<String>,
    /// Email of the updating user.
    pub updated_by: Option<String>,
    /// Id of the updating user.
    pub updated_by_id: Option<String>,
    /// How many users use this role.
    pub users_in_roles: Option<i64>,
    /// Id of the containing scope.
    pub scope_id: Option<String>,
    /// Scope of the role. Allowed values: `Group`, `Site`, `Account`, `Tenant`.
    pub scope: Option<String>,
    /// Whether this role is a system role.
    pub predefined_role: Option<bool>,
    /// Account name.
    pub account_name: Option<String>,
    /// Site name.
    pub site_name: Option<String>,
    /// Pages, each grouping a set of permissions visible in the WebUI / Console.
    pub pages: Option<Vec<RolePage>>,
}

/// A new-role template, as returned by `GET /web/api/v2.1/rbac/role`
/// (`rbac.schemas_NewRoleTemplate`).
///
/// Describes the default permission pages used to seed a brand-new role.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoleTemplate {
    /// Description. (Marked `required` in the spec, but defaults to `Option`
    /// here only when present; the spec lists it as required and not nullable.)
    pub description: String,
    /// Pages, each grouping a set of permissions visible in the WebUI / Console.
    pub pages: Option<Vec<RolePage>>,
}

/// A single permission page within a role definition or template.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolePage {
    /// Page name.
    pub name: Option<String>,
    /// Page identifier.
    pub identifier: Option<String>,
    /// Permissions belonging to this page.
    pub permissions: Option<Vec<RolePermission>>,
}

/// A single permission entry within a [`RolePage`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolePermission {
    /// Permission identifier.
    pub identifier: Option<String>,
    /// Permission title.
    pub title: Option<String>,
    /// Whether the permission is granted.
    pub value: Option<bool>,
    /// Permission description.
    pub description: Option<String>,
    /// Additional description (nullable).
    pub additional_description: Option<String>,
    /// Permission type.
    #[serde(rename = "type")]
    pub permission_type: Option<String>,
    /// Identifiers of permissions this one depends on.
    pub depends_on: Option<Vec<String>>,
    /// Group name (nullable).
    pub group_name: Option<String>,
    /// Disabled reason (nullable).
    pub disabled_reason: Option<String>,
    /// Disabled reason code (nullable).
    pub disabled_reason_code: Option<String>,
}

/// Acknowledgement returned by `DELETE /web/api/v2.1/rbac/role/{role_id}`
/// (`_SuccessResponseSchema`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Indicates a successful operation.
    pub success: Option<bool>,
}
