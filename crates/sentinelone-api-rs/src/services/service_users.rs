use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::service_users::{
    AffectedResults, ServiceUser, ServiceUserApiToken, SuccessResponse,
};
use crate::pagination::{Paginated, Response};

/// `Service Users` tag.
///
/// Service Users related APIs.
pub struct ServiceUsersService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/export/service-users` (Export Service
/// Users).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUsersExportQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    ///
    /// Optional. Array param -> comma-joined string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    ///
    /// Optional. Array param -> comma-joined string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of service user IDs to filter by. Example: "225494730938493804,225494730938493915".
    ///
    /// Optional. Array param -> comma-joined string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Full text search for fields: full_name, email, description.
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// List of rbac roles to filter by. Example: "225494730938493804,225494730938493915".
    ///
    /// Optional. Array param -> comma-joined string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<String>,
}

impl ServiceUsersExportQuery {
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of service user IDs to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Full text search for fields: full_name, email, description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// List of rbac roles to filter by.
    pub fn role_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.role_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query params for `GET /web/api/v2.1/service-users` (Get Service Users).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUsersQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use "cursor". Example: "150".
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10".
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=".
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects.
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time.
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: "id".
    ///
    /// Optional. Allowed values: `id`, `createdAt`, `updatedAt`, `name`,
    /// `fullName`, `description`, `firstLogin`, `lastLogin`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc".
    ///
    /// Optional. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    ///
    /// Optional. Array param -> comma-joined string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    ///
    /// Optional. Array param -> comma-joined string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of service user IDs to filter by. Example: "225494730938493804,225494730938493915".
    ///
    /// Optional. Array param -> comma-joined string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Full text search for fields: full_name, email, description.
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// List of rbac roles to filter by. Example: "225494730938493804,225494730938493915".
    ///
    /// Optional. Array param -> comma-joined string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<String>,
}

impl ServiceUsersQuery {
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
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `createdAt`,
    /// `updatedAt`, `name`, `fullName`, `description`, `firstLogin`,
    /// `lastLogin`.
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
        self.site_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// List of service user IDs to filter by.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Full text search for fields: full_name, email, description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// List of rbac roles to filter by.
    pub fn role_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.role_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// A scope/role association used in create/update request bodies.
///
/// `id` is mandatory for users in scope `account`/`site`; users in the `tenant`
/// (global) scope do not need to provide an `id`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceUserScopeRoleInput {
    /// Scope ID. Example: "225494730938493804".
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// \[DEPRECATED\] List containing the desired role name in this scope. Use
    /// `role_id` or `role_name` instead.
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// \[DEPRECATED\] Name of the role, will work only for predefined roles.
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// ID of the wanted role. Example: "225494730938493804".
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
}

/// `data` payload for `POST /web/api/v2.1/service-users` (Create Service User).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceUserData {
    /// Name of the service user. Example: "My app integration".
    ///
    /// Required (1-255 chars, pattern `^[^<>=]+$`).
    pub name: String,
    /// Description.
    ///
    /// Optional/nullable (max 255 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Date when the generated token expires (date-time string).
    ///
    /// Required.
    pub expiration_date: String,
    /// User scope. Example: "tenant".
    ///
    /// Required. Allowed values: `tenant`, `account`, `site`.
    pub scope: String,
    /// List of id and role id, id is mandatory for user in scope account/site.
    /// User in tenant (global) role does not need to provide an id.
    ///
    /// Optional (defaults to an empty list server-side).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_roles: Option<Vec<ServiceUserScopeRoleInput>>,
    /// Temporary attribute for WA: If the flag is set to True the legacy token
    /// will be generated even if the auth_tokens global switch is turned on.
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_legacy: Option<bool>,
}

/// Request body for `POST /web/api/v2.1/service-users` (Create Service User).
#[derive(Debug, Clone, Serialize)]
pub struct CreateServiceUserBody {
    /// Data.
    ///
    /// Required.
    pub data: CreateServiceUserData,
}

/// `filter` payload for `POST /web/api/v2.1/service-users/delete-service-users`
/// (Bulk Delete Service Users).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkDeleteServiceUsersFilter {
    /// List of Site IDs to filter by (1-500 items).
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by (1-500 items).
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of service user IDs to filter by (max 5000 items).
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Full text search for fields: full_name, email, description.
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// List of rbac roles to filter by (max 5000 items).
    ///
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
}

/// Request body for `POST /web/api/v2.1/service-users/delete-service-users`
/// (Bulk Delete Service Users).
#[derive(Debug, Default, Clone, Serialize)]
pub struct BulkDeleteServiceUsersBody {
    /// Filter.
    ///
    /// Required.
    pub filter: BulkDeleteServiceUsersFilter,
    /// Data.
    ///
    /// Optional/nullable; freeform object per spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// `data` payload for `PUT /web/api/v2.1/service-users/{service_user_id}`
/// (Update Service User).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceUserData {
    /// Description.
    ///
    /// Optional/nullable (max 255 chars).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// User scope. Example: "tenant".
    ///
    /// Optional/nullable. Allowed values: `tenant`, `account`, `site`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// List of id and role id, id is mandatory for user in scope account/site.
    /// User in tenant (global) role does not need to provide an id.
    ///
    /// Optional (defaults to an empty list server-side).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_roles: Option<Vec<ServiceUserScopeRoleInput>>,
}

/// Request body for `PUT /web/api/v2.1/service-users/{service_user_id}` (Update
/// Service User).
#[derive(Debug, Default, Clone, Serialize)]
pub struct UpdateServiceUserBody {
    /// Data.
    ///
    /// Required.
    pub data: UpdateServiceUserData,
}

/// `data` payload for
/// `POST /web/api/v2.1/service-users/{service_user_id}/generate-api-token`
/// (Generate API Token for Service User).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateServiceUserApiTokenData {
    /// Date when the generated token expires (date-time string).
    ///
    /// Required.
    pub expiration_date: String,
}

/// Request body for
/// `POST /web/api/v2.1/service-users/{service_user_id}/generate-api-token`
/// (Generate API Token for Service User).
#[derive(Debug, Clone, Serialize)]
pub struct GenerateServiceUserApiTokenBody {
    /// Data.
    ///
    /// Required.
    pub data: GenerateServiceUserApiTokenData,
}

impl ServiceUsersService<'_> {
    /// `GET /web/api/v2.1/export/service-users` — Export Service Users.
    ///
    /// Export Service User data to a CSV, for Service Users that match the
    /// filter. The response is an export payload rather than a typed envelope,
    /// so it is returned as a freeform JSON value.
    pub async fn export(
        &self,
        query: &ServiceUsersExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/export/service-users", q)
            .await?)
    }

    /// `GET /web/api/v2.1/service-users` — Get Service Users.
    ///
    /// Get a list of service users.
    pub async fn list(
        &self,
        query: &ServiceUsersQuery,
    ) -> Result<Paginated<ServiceUser>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/service-users", q)
            .await?)
    }

    /// `POST /web/api/v2.1/service-users` — Create Service User.
    ///
    /// Create a new service user. The response carries the new token secret at
    /// `api_token.value`.
    pub async fn create(
        &self,
        body: &CreateServiceUserBody,
    ) -> Result<Response<ServiceUser>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/service-users", body)
            .await?)
    }

    /// `POST /web/api/v2.1/service-users/delete-service-users` — Bulk Delete
    /// Service Users.
    ///
    /// Delete all service users that match the filter.
    pub async fn bulk_delete(
        &self,
        body: &BulkDeleteServiceUsersBody,
    ) -> Result<Response<AffectedResults>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/service-users/delete-service-users", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/service-users/{service_user_id}` — Delete Service
    /// User.
    ///
    /// Delete a service user by ID.
    ///
    /// `service_user_id`: Service User ID. Example: "225494730938493804".
    pub async fn delete(
        &self,
        service_user_id: impl Into<String>,
    ) -> Result<Response<SuccessResponse>, Error> {
        let path = format!(
            "/web/api/v2.1/service-users/{}",
            service_user_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<(), Response<SuccessResponse>>(
                Method::DELETE,
                &path,
                None,
                None,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/service-users/{service_user_id}` — Get Service User.
    ///
    /// Get a specific service user by ID.
    ///
    /// `service_user_id`: Service User ID. Example: "225494730938493804".
    pub async fn get(
        &self,
        service_user_id: impl Into<String>,
    ) -> Result<Response<ServiceUser>, Error> {
        let path = format!(
            "/web/api/v2.1/service-users/{}",
            service_user_id.into()
        );
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `PUT /web/api/v2.1/service-users/{service_user_id}` — Update Service
    /// User.
    ///
    /// Change properties of the service user with the given ID.
    ///
    /// `service_user_id`: Service User ID. Example: "225494730938493804".
    pub async fn update(
        &self,
        service_user_id: impl Into<String>,
        body: &UpdateServiceUserBody,
    ) -> Result<Response<ServiceUser>, Error> {
        let path = format!(
            "/web/api/v2.1/service-users/{}",
            service_user_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<UpdateServiceUserBody, Response<ServiceUser>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/service-users/{service_user_id}/generate-api-token` —
    /// Generate API Token for Service User.
    ///
    /// Generate a new API token for a service user and revoke the existing API
    /// token.
    ///
    /// `service_user_id`: Service User ID. Example: "225494730938493804".
    pub async fn generate_api_token(
        &self,
        service_user_id: impl Into<String>,
        body: &GenerateServiceUserApiTokenBody,
    ) -> Result<Response<ServiceUserApiToken>, Error> {
        let path = format!(
            "/web/api/v2.1/service-users/{}/generate-api-token",
            service_user_id.into()
        );
        Ok(self
            .client
            .http()
            .post(&path, body)
            .await?)
    }
}
