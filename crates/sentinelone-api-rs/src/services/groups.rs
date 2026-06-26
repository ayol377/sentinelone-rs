//! Service for the `Groups` tag.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::groups::{Group, GroupSuccess, GroupToken, RegenerateKey};
use crate::pagination::{Paginated, Response};

/// `Groups` tag — Groups related endpoints.
pub struct GroupsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query types
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/groups` (Get Groups).
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
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
    /// The column to sort the results by. Example: "id". Optional. Allowed
    /// values: `id`, `name`, `type`, `rank`, `siteId`, `createdAt`,
    /// `updatedAt`, `description`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Optional. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by. Example:
    /// "225494730938493804,225494730938493915". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example:
    /// "225494730938493804,225494730938493915". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by. Example:
    /// "225494730938493804,225494730938493915". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Group type. Example: "static". Optional. Allowed values: `static`,
    /// `dynamic`, `pinned`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// A list of Group types. Example: "static". Optional. Allowed item values:
    /// `static`, `dynamic`, `pinned`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<String>,
    /// Free text search on fields name, description. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Id. Example: "225494730938493804". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description for the Group. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The rank sets the priority of a dynamic group over others. Example: "1".
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// Is this the default group? Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Updated at lesser than. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at_lt: Option<String>,
    /// Updated at greater than. Example: "2018-02-27T04:49:26.257525Z".
    /// Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at_gt: Option<String>,
    /// Updated at lesser or equal than. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
    /// Updated at greater or equal than. Example:
    /// "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// Registration token (base64-encoded `{"url", "site_key"}`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_token: Option<String>,
}

impl ListQuery {
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
    /// If true, only the total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, the total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort by. Allowed: `id`, `name`, `type`, `rank`, `siteId`,
    /// `createdAt`, `updatedAt`, `description`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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
    /// List of Account IDs to filter by (comma-joined).
    pub fn account_ids<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(ids));
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
    /// Group type. Allowed: `static`, `dynamic`, `pinned`.
    pub fn type_(mut self, v: impl Into<String>) -> Self {
        self.type_ = Some(v.into());
        self
    }
    /// A list of Group types (comma-joined). Allowed items: `static`,
    /// `dynamic`, `pinned`.
    pub fn types<I, S>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.types = Some(join_csv(types));
        self
    }
    /// Free text search on fields name, description.
    pub fn query(mut self, q: impl Into<String>) -> Self {
        self.query = Some(q.into());
        self
    }
    /// Id.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    /// Name.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// The description for the Group.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// The rank sets the priority of a dynamic group over others.
    pub fn rank(mut self, n: i64) -> Self {
        self.rank = Some(n);
        self
    }
    /// Is this the default group?
    pub fn is_default(mut self, v: bool) -> Self {
        self.is_default = Some(v);
        self
    }
    /// Updated at lesser than.
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lt = Some(v.into());
        self
    }
    /// Updated at greater than.
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gt = Some(v.into());
        self
    }
    /// Updated at lesser or equal than.
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// Updated at greater or equal than.
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// Registration token.
    pub fn registration_token(mut self, v: impl Into<String>) -> Self {
        self.registration_token = Some(v.into());
        self
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

// ---------------------------------------------------------------------------
// Body types
// ---------------------------------------------------------------------------

/// Body for `POST /web/api/v2.1/groups` (Create Group)
/// (`groups.groups_PostGroupSchema`).
#[derive(Debug, Clone, Serialize)]
pub struct CreateBody {
    /// Data. Required.
    pub data: CreateBodyData,
}

/// `data` object for [`CreateBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBodyData {
    /// Name. Required (min length 1, pattern `^[^<>]+$`).
    pub name: String,
    /// The id of the site this group is created in. Required.
    pub site_id: String,
    /// True to inherit from site policy. Required.
    pub inherits: bool,
    /// The user-defined description for the Group. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If supplied this group will be dynamic using the filter to associate
    /// agents. Example: "225494730938493804". Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<String>,
    /// Obsolete - Always MGMT. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Obsolete for post - The rank of the group. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// Is this the default group? Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// Group type. Optional. Allowed values: `static`, `dynamic`, `pinned`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// The group policy; required only if `inherits` is False, ignored
    /// otherwise. Freeform nested object. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,
}

impl CreateBody {
    /// Build a [`CreateBody`] with the required fields.
    pub fn new(
        name: impl Into<String>,
        site_id: impl Into<String>,
        inherits: bool,
    ) -> Self {
        Self {
            data: CreateBodyData {
                name: name.into(),
                site_id: site_id.into(),
                inherits,
                ..Default::default()
            },
        }
    }
}

/// Body for `PUT /web/api/v2.1/groups/{group_id}` (Update Group)
/// (`groups.groups_PutGroupSchema`). The spec marks no field required.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateBody {
    /// Data. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<UpdateBodyData>,
}

/// `data` object for [`UpdateBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBodyData {
    /// Id. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The id of the site this group is part of. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    /// The user-defined description for the Group. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// If supplied this group will be dynamic using the filter to associate
    /// agents. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<String>,
    /// Obsolete - Always MGMT. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The rank of the group. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    /// True to inherit from site policy. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherits: Option<bool>,
    /// Is this the default group? Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// The group policy. Freeform nested object. Note: `iocAttributes` refers to
    /// Deep Visibility; remove it if you do not have a Complete SKU.
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,
}

/// Body for `PUT /web/api/v2.1/groups/ranks` (Update Ranks)
/// (`groups.groups_PutRanksSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRanksBody {
    /// Data. Required.
    pub data: UpdateRanksData,
    /// Filter. Required.
    pub filter: UpdateRanksFilter,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

/// `data` object for [`UpdateRanksBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRanksData {
    /// List of ranks to update. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranks: Option<Vec<GroupRank>>,
}

/// A single group rank entry for [`UpdateRanksData`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupRank {
    /// Id. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The new rank for the group. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
}

/// `filter` object for [`UpdateRanksBody`].
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRanksFilter {
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

/// Body for `PUT /web/api/v2.1/groups/{group_id}/move-agents` (Move Agents)
/// (`groups.groups_PutAddAgentsSchema`).
#[derive(Debug, Clone, Serialize)]
pub struct MoveAgentsBody {
    /// Specification of which agents should be moved. Required. This is the
    /// large Agents filter object; pass it as freeform JSON.
    pub filter: serde_json::Value,
}

/// Body for `PUT /web/api/v2.1/groups/{group_id}/revert-policy` (Revert Policy)
/// (`policies_schemas.policies_schemas_RevertPolicySchema`).
#[derive(Debug, Clone, Default, Serialize)]
pub struct RevertPolicyBody {
    /// Data. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<RevertPolicyData>,
}

/// `data` object for [`RevertPolicyBody`].
#[derive(Debug, Clone, Default, Serialize)]
pub struct RevertPolicyData {
    /// Id. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ---------------------------------------------------------------------------
// Service
// ---------------------------------------------------------------------------

impl GroupsService<'_> {
    /// `GET /web/api/v2.1/groups` — Get Groups.
    ///
    /// Get data of groups that match the filter. Best practice: use as narrow a
    /// filter as you can. The data can be quite long for many groups. The
    /// response returns the ID of each group, which you can use in other
    /// commands.
    pub async fn list(&self, query: &ListQuery) -> Result<Paginated<Group>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/groups", q).await?)
    }

    /// `POST /web/api/v2.1/groups` — Create Group.
    ///
    /// Create a new group. You must create the Group in a Site (run "sites" to
    /// get the Site ID) for which you have permissions. If you create a dynamic
    /// Group, you must have the ID of a filter saved in the Site (run
    /// "filters?siteIds=<id from sites>").
    pub async fn create(&self, body: &CreateBody) -> Result<Response<Group>, Error> {
        Ok(self.client.http().post("/web/api/v2.1/groups", body).await?)
    }

    /// `PUT /web/api/v2.1/groups/ranks` — Update Ranks.
    ///
    /// An Agent can belong to only one Group. If the Agent matches multiple
    /// Dynamic Groups, it goes to the Group with the highest rank. The "rank"
    /// parameter has a minimum of "1". The lower the integer, the higher
    /// priority it has to collect Agents. Make sure the IDs of the groups in
    /// this command are for Dynamic groups.
    ///
    /// Returns `204 No Content` on success.
    pub async fn update_ranks(
        &self,
        body: &UpdateRanksBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<UpdateRanksBody, Response<serde_json::Value>>(
                Method::PUT,
                "/web/api/v2.1/groups/ranks",
                None,
                Some(body),
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/groups/{group_id}` — Delete Group.
    ///
    /// Delete a Group given by the required Group ID (run "groups"). If there
    /// are Agents in the Group, and the Group is dynamic, the next dynamic
    /// Groups will collect matching Agents, and unmatched Agents will go to the
    /// Default Group. If this is a static Group with Agents, all the Agents will
    /// go to the Default Group. (Agents always go to matching dynamic Groups. If
    /// a static Group holds Agents, there are no matching dynamic Groups.)
    ///
    /// `group_id`: Group ID. Example: "225494730938493804".
    pub async fn delete(
        &self,
        group_id: impl Into<String>,
    ) -> Result<Response<GroupSuccess>, Error> {
        let path = format!("/web/api/v2.1/groups/{}", group_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<GroupSuccess>>(Method::DELETE, &path, None, None)
            .await?)
    }

    /// `GET /web/api/v2.1/groups/{group_id}` — Get Group by ID.
    ///
    /// Get data of a given Group. To get a Group ID, run "groups". This command
    /// responds with the ID of the Site of the Group, Group name, type (dynamic
    /// or static), and similar data. Your username must have permissions for the
    /// Site.
    ///
    /// `group_id`: Group ID. Example: "225494730938493804".
    pub async fn get(
        &self,
        group_id: impl Into<String>,
    ) -> Result<Response<Group>, Error> {
        let path = format!("/web/api/v2.1/groups/{}", group_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `PUT /web/api/v2.1/groups/{group_id}` — Update Group.
    ///
    /// Change properties of a Group specified by its ID (run "groups"). The body
    /// of the request holds all the properties of a Group. You must have access
    /// permissions on the Site. Note: iocAttributes refers to Deep Visibility.
    /// If you do not have a Complete SKU, you can remove this set.
    ///
    /// `group_id`: Group ID. Example: "225494730938493804".
    pub async fn update(
        &self,
        group_id: impl Into<String>,
        body: &UpdateBody,
    ) -> Result<Response<Group>, Error> {
        let path = format!("/web/api/v2.1/groups/{}", group_id.into());
        Ok(self
            .client
            .http()
            .request_json::<UpdateBody, Response<Group>>(Method::PUT, &path, None, Some(body))
            .await?)
    }

    /// `PUT /web/api/v2.1/groups/{group_id}/move-agents` — Move Agents.
    ///
    /// Move Agents that match the filter to a Group. The Group ID (run "groups")
    /// is required and there can be only one. This will move the matched Agents
    /// that are in the same Site as the given Group.
    ///
    /// Returns `204 No Content` on success.
    ///
    /// `group_id`: Group ID. Example: "225494730938493804".
    pub async fn move_agents(
        &self,
        group_id: impl Into<String>,
        body: &MoveAgentsBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/groups/{}/move-agents", group_id.into());
        Ok(self
            .client
            .http()
            .request_json::<MoveAgentsBody, Response<serde_json::Value>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/groups/{group_id}/regenerate-key` — Regenerate Group
    /// Token.
    ///
    /// Get a new Group Token for a static Group. This command requires the Group
    /// ID ("groups") and you must have permissions for the Group. If you run
    /// this command on a dynamic Group, it ends in an error. If you use the API
    /// in scripts to add new endpoints with a Group Token rather than a Site
    /// Token, be aware that you must update the token value in your scripts.
    ///
    /// `group_id`: Group ID. Example: "225494730938493804".
    pub async fn regenerate_key(
        &self,
        group_id: impl Into<String>,
    ) -> Result<Response<RegenerateKey>, Error> {
        let path = format!("/web/api/v2.1/groups/{}/regenerate-key", group_id.into());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<RegenerateKey>>(Method::PUT, &path, None, None)
            .await?)
    }

    /// `PUT /web/api/v2.1/groups/{group_id}/revert-policy` — Revert Policy.
    ///
    /// A Group can have a policy that is different from its Site policy. Use this
    /// command to revert the changes on the Group policy to inherit the Site
    /// policy. Your user must have permissions on the Site.
    ///
    /// `group_id`: Group ID. Example: "225494730938493804".
    pub async fn revert_policy(
        &self,
        group_id: impl Into<String>,
        body: &RevertPolicyBody,
    ) -> Result<Response<GroupSuccess>, Error> {
        let path = format!("/web/api/v2.1/groups/{}/revert-policy", group_id.into());
        Ok(self
            .client
            .http()
            .request_json::<RevertPolicyBody, Response<GroupSuccess>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/groups/{group_id}/token` — Get Site registration token
    /// by ID.
    ///
    /// Get the registration token of the Group of the ID.
    ///
    /// `group_id`: Group ID. Example: "225494730938493804".
    pub async fn token(
        &self,
        group_id: impl Into<String>,
    ) -> Result<Response<GroupToken>, Error> {
        let path = format!("/web/api/v2.1/groups/{}/token", group_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }
}
