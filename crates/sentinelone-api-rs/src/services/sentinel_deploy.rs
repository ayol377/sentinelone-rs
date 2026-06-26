use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::sentinel_deploy::{CredGroup, CredGroupDetail, CredGroupSuccess};
use crate::pagination::{Paginated, Response};

/// `Sentinel Deploy` tag.
///
/// Sentinel deploy views and operations (Ranger auto-deploy cred groups).
pub struct SentinelDeployService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/ranger/cred-groups`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCredGroupsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: "150". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: "id". Optional.
    ///
    /// Allowed values: `groupName`, `updatedAt`, `createdAt`, `domain`,
    /// `targetOs`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Optional.
    ///
    /// Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Single Account ID to filter by. Example: "225494730938493804".
    /// Array, comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Single Site ID to filter by. Example: "225494730938493804". Array,
    /// comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Group name being searched. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// Group name being searched (like filter). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name_like: Option<String>,
    /// A list of ids to get. Array, comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Get creds with total details greater than the supplied number. Optional.
    #[serde(rename = "totalDetails__gt", skip_serializing_if = "Option::is_none")]
    pub total_details__gt: Option<i64>,
    /// The os type for this cred group. Example: "windows". Optional.
    ///
    /// Allowed values: `windows`, `osx_linux`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_os: Option<String>,
}

impl GetCredGroupsQuery {
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
    /// The column to sort the results by.
    ///
    /// Allowed values: `groupName`, `updatedAt`, `createdAt`, `domain`,
    /// `targetOs`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Single Account ID to filter by (comma-joined).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// Single Site ID to filter by (comma-joined).
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// Group name being searched (exact).
    pub fn group_name(mut self, v: impl Into<String>) -> Self {
        self.group_name = Some(v.into());
        self
    }
    /// Group name being searched (like filter).
    pub fn group_name_like(mut self, v: impl Into<String>) -> Self {
        self.group_name_like = Some(v.into());
        self
    }
    /// A list of ids to get (comma-joined).
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// Get creds with total details greater than the supplied number.
    pub fn total_details_gt(mut self, n: i64) -> Self {
        self.total_details__gt = Some(n);
        self
    }
    /// The os type for this cred group. Allowed values: `windows`, `osx_linux`.
    pub fn target_os(mut self, v: impl Into<String>) -> Self {
        self.target_os = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/ranger/cred-groups/details`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCredGroupDetailsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: "150". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: "id". Optional.
    ///
    /// Allowed values: `title`, `type`, `updatedAt`, `createdAt`, `credType`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Optional.
    ///
    /// Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Single Account ID to filter by. Example: "225494730938493804". Array,
    /// comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Single Site ID to filter by. Example: "225494730938493804". Array,
    /// comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// Exact filter by title. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Like filter by title. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_like: Option<String>,
    /// The type of the cred group (like filter). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cred_type_like: Option<String>,
    /// A list of ids to get. Array, comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// A list of cred group ids to get. Array, comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cred_group_ids: Option<String>,
}

impl GetCredGroupDetailsQuery {
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
    /// The column to sort the results by.
    ///
    /// Allowed values: `title`, `type`, `updatedAt`, `createdAt`, `credType`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Single Account ID to filter by (comma-joined).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
        self
    }
    /// Single Site ID to filter by (comma-joined).
    pub fn site_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(ids));
        self
    }
    /// Exact filter by title.
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.title = Some(v.into());
        self
    }
    /// Like filter by title.
    pub fn title_like(mut self, v: impl Into<String>) -> Self {
        self.title_like = Some(v.into());
        self
    }
    /// The type of the cred group (like filter).
    pub fn cred_type_like(mut self, v: impl Into<String>) -> Self {
        self.cred_type_like = Some(v.into());
        self
    }
    /// A list of ids to get (comma-joined).
    pub fn ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(ids));
        self
    }
    /// A list of cred group ids to get (comma-joined).
    pub fn cred_group_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cred_group_ids = Some(join_csv(ids));
        self
    }
}

/// Inner `data` object for [`CreateCredGroupBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCredGroupData {
    /// The cred group name (min length 2). Required.
    pub group_name: String,
    /// Encrypted passphrase with key unknown by the management. Required.
    pub group_passphrase: String,
    /// Scope id. Example: "225494730938493804". Required.
    pub scope_id: String,
    /// The domain associated to this cred group. Example: "OFFICE". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The os type for this cred group. Example: "windows". Optional.
    ///
    /// Allowed values: `windows`, `osx_linux`. Default `windows`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_os: Option<String>,
}

/// Body for `POST /web/api/v2.1/ranger/cred-groups` (Create Cred Group).
#[derive(Debug, Clone, Serialize)]
pub struct CreateCredGroupBody {
    /// The cred group data. Required.
    pub data: CreateCredGroupData,
}

impl CreateCredGroupBody {
    /// Build a body from the three required fields.
    pub fn new(
        group_name: impl Into<String>,
        group_passphrase: impl Into<String>,
        scope_id: impl Into<String>,
    ) -> Self {
        Self {
            data: CreateCredGroupData {
                group_name: group_name.into(),
                group_passphrase: group_passphrase.into(),
                scope_id: scope_id.into(),
                domain: None,
                target_os: None,
            },
        }
    }
    /// Set the optional `domain`.
    pub fn domain(mut self, v: impl Into<String>) -> Self {
        self.data.domain = Some(v.into());
        self
    }
    /// Set the optional `targetOs`. Allowed values: `windows`, `osx_linux`.
    pub fn target_os(mut self, v: impl Into<String>) -> Self {
        self.data.target_os = Some(v.into());
        self
    }
}

/// A single cred detail item used in [`AddCredDetailsData`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CredDetailItem {
    /// A encrypted key for the creds. Required.
    pub encrypted_key: String,
    /// The encrypted creds. Required.
    pub encrypted_cred: String,
    /// The title for the cred. Example: "AD admin". Required.
    pub title: String,
    /// The type of the cred. Example: "User/Password". Required.
    pub cred_type: String,
}

/// Inner `data` object for [`AddCredDetailsBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCredDetailsData {
    /// Cred group id. Example: "225494730938493804". Required.
    pub cred_group_id: String,
    /// The cred group details (max 5000 items). Required.
    pub details: Vec<CredDetailItem>,
}

/// Body for `POST /web/api/v2.1/ranger/cred-groups/details` (Add cred details).
#[derive(Debug, Clone, Serialize)]
pub struct AddCredDetailsBody {
    /// The cred details payload.
    pub data: AddCredDetailsData,
}

impl AddCredDetailsBody {
    /// Build a body for the given cred group id and detail items.
    pub fn new(cred_group_id: impl Into<String>, details: Vec<CredDetailItem>) -> Self {
        Self {
            data: AddCredDetailsData {
                cred_group_id: cred_group_id.into(),
                details,
            },
        }
    }
}

/// Inner `data` object for [`UpdateCredDetailBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCredDetailData {
    /// A encrypted key for the creds. Required.
    pub encrypted_key: String,
    /// The encrypted creds. Required.
    pub encrypted_cred: String,
    /// The title for the cred. Example: "AD admin". Required.
    pub title: String,
    /// The type of the cred. Example: "User/Password". Required.
    pub cred_type: String,
}

/// Body for `PUT /web/api/v2.1/ranger/cred-groups/details/{detail_id}`
/// (Update Cred Group Details).
#[derive(Debug, Clone, Serialize)]
pub struct UpdateCredDetailBody {
    /// The cred detail data. Required.
    pub data: UpdateCredDetailData,
}

impl UpdateCredDetailBody {
    /// Build a body from the four required fields.
    pub fn new(
        encrypted_key: impl Into<String>,
        encrypted_cred: impl Into<String>,
        title: impl Into<String>,
        cred_type: impl Into<String>,
    ) -> Self {
        Self {
            data: UpdateCredDetailData {
                encrypted_key: encrypted_key.into(),
                encrypted_cred: encrypted_cred.into(),
                title: title.into(),
                cred_type: cred_type.into(),
            },
        }
    }
}

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

impl SentinelDeployService<'_> {
    /// `GET /web/api/v2.1/ranger/cred-groups` — Get Cred groups.
    ///
    /// Get the data for each row in the Cred Groups table.
    pub async fn list_cred_groups(
        &self,
        query: &GetCredGroupsQuery,
    ) -> Result<Paginated<CredGroup>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/ranger/cred-groups", q)
            .await?)
    }

    /// `POST /web/api/v2.1/ranger/cred-groups` — Create Cred Group.
    ///
    /// Create a new Cred Group.
    pub async fn create_cred_group(
        &self,
        body: &CreateCredGroupBody,
    ) -> Result<Response<CredGroup>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/ranger/cred-groups", body)
            .await?)
    }

    /// `GET /web/api/v2.1/ranger/cred-groups/details` — Get Cred group details.
    ///
    /// Get the data for each row in the Cred Groups details table.
    pub async fn list_cred_group_details(
        &self,
        query: &GetCredGroupDetailsQuery,
    ) -> Result<Paginated<CredGroupDetail>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/ranger/cred-groups/details", q)
            .await?)
    }

    /// `POST /web/api/v2.1/ranger/cred-groups/details` — Add cred details.
    ///
    /// Add cred details to a cred group.
    pub async fn add_cred_details(
        &self,
        body: &AddCredDetailsBody,
    ) -> Result<Response<CredGroupSuccess>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/ranger/cred-groups/details", body)
            .await?)
    }

    /// `DELETE /web/api/v2.1/ranger/cred-groups/details/{detail_id}` — Delete
    /// Cred Group Detail.
    ///
    /// Delete cred group detail value.
    ///
    /// `detail_id`: Cred group detail ID. Example: "225494730938493804".
    pub async fn delete_cred_group_detail(
        &self,
        detail_id: impl Into<String>,
    ) -> Result<Response<CredGroupSuccess>, Error> {
        let path = format!(
            "/web/api/v2.1/ranger/cred-groups/details/{}",
            detail_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<(), Response<CredGroupSuccess>>(Method::DELETE, &path, None, None)
            .await?)
    }

    /// `PUT /web/api/v2.1/ranger/cred-groups/details/{detail_id}` — Update Cred
    /// Group Details.
    ///
    /// Update cred group values.
    ///
    /// `detail_id`: Cred group detail ID. Example: "225494730938493804".
    pub async fn update_cred_group_detail(
        &self,
        detail_id: impl Into<String>,
        body: &UpdateCredDetailBody,
    ) -> Result<Response<CredGroupDetail>, Error> {
        let path = format!(
            "/web/api/v2.1/ranger/cred-groups/details/{}",
            detail_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<UpdateCredDetailBody, Response<CredGroupDetail>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/ranger/cred-groups/{cred_group_id}` — Delete Cred
    /// Group.
    ///
    /// Delete cred group value.
    ///
    /// `cred_group_id`: Cred group ID. Example: "225494730938493804".
    pub async fn delete_cred_group(
        &self,
        cred_group_id: impl Into<String>,
    ) -> Result<Response<CredGroupSuccess>, Error> {
        let path = format!("/web/api/v2.1/ranger/cred-groups/{}", cred_group_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<CredGroupSuccess>>(Method::DELETE, &path, None, None)
            .await?)
    }
}
