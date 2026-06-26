//! `Remote Ops MMS` tag — service, query and body types.
//!
//! 1:1 with `docs/remote-ops-mms.md` / `swagger_2_1.json`. Every endpoint is an
//! async method. Query params live in per-method `*Query` structs (all fields
//! optional, camelCase, comma-joined arrays). Request bodies live in per-method
//! `*Body` structs (deeply nested/freeform shapes use `serde_json::Value`).
//! Enum-valued fields are `String` for forward-compatibility; allowed values
//! are documented inline.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::remote_ops_mms::{
    DeletedDestinationProfilesResult, DeletedScheduledTasksResult, DestinationProfile,
    ProfileIdResult, ScheduleForensicsResult, ScheduleRemoteScriptResult, ScheduledTask,
    ScheduledTaskIdResult, SkylightUploadResults,
};
use crate::pagination::{Paginated, Response};

/// `Remote Ops MMS` tag. Remote operations: data-exporter Destination
/// profiles, exported results and scheduled tasks (remote scripts and
/// forensics collections).
pub struct RemoteOpsMmsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// GET /web/api/v2.1/remote-ops/data-exporter/destination-profiles
// ---------------------------------------------------------------------------

/// Query params for
/// `GET /web/api/v2.1/remote-ops/data-exporter/destination-profiles`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDestinationProfilesQuery {
    /// Scope level to get Destination profile configuration. Optional.
    /// Allowed values: `tenant`, `account`, `site`, `group`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope ID to get Destination profiles configuration. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl GetDestinationProfilesQuery {
    /// Scope level to get Destination profile configuration.
    /// Allowed values: `tenant`, `account`, `site`, `group`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Scope ID to get Destination profiles configuration.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

// ---------------------------------------------------------------------------
// GET /web/api/v2.1/remote-ops/data-exporter/destination-profiles/{profile_id}
// ---------------------------------------------------------------------------

/// Query params for
/// `GET /web/api/v2.1/remote-ops/data-exporter/destination-profiles/{profile_id}`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDestinationProfileQuery {
    /// Scope level to get Destination profile configuration. Optional.
    /// Allowed values: `tenant`, `account`, `site`, `group`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope ID to get Destination profiles configuration. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl GetDestinationProfileQuery {
    /// Scope level to get Destination profile configuration.
    /// Allowed values: `tenant`, `account`, `site`, `group`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Scope ID to get Destination profiles configuration.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

// ---------------------------------------------------------------------------
// GET /web/api/v2.1/remote-ops/data-exporter/results
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/remote-ops/data-exporter/results`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDataExporterResultsQuery {
    /// Id of the agent the data came from. Required.
    pub agent_id: String,
    /// Task id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// Threat malicious group id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malicious_group_id: Option<String>,
}

impl GetDataExporterResultsQuery {
    /// Construct the query with the required `agentId`.
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
            task_id: None,
            malicious_group_id: None,
        }
    }
    /// Task id.
    pub fn task_id(mut self, v: impl Into<String>) -> Self {
        self.task_id = Some(v.into());
        self
    }
    /// Threat malicious group id.
    pub fn malicious_group_id(mut self, v: impl Into<String>) -> Self {
        self.malicious_group_id = Some(v.into());
        self
    }
}

// ---------------------------------------------------------------------------
// GET /web/api/v2.1/remote-ops/scheduled-tasks
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/remote-ops/scheduled-tasks`.
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScheduledTasksQuery {
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Keyword to search in scope name (array). Optional.
    #[serde(rename = "scopeName__contains", skip_serializing_if = "Option::is_none")]
    pub scope_name_contains: Option<String>,
    /// List of output destinations (array). Optional.
    /// Allowed values: `SentinelCloud`, `Local`, `None`, `SingularityXDR`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_destination: Option<String>,
    /// IDs of creating user of scheduled task (array). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    /// Date range for creation time. Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at_between: Option<String>,
    /// List of Account IDs to filter by (array). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of the task types (array). Optional.
    /// Allowed values: `action`, `dataCollection`, `artifactCollection`,
    /// `forensicsProfile`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// List of the OS types (array). Optional.
    /// Allowed values: `linux`, `macos`, `windows`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// A list of scheduled task ids (array). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Keyword to search in description (array). Optional.
    #[serde(rename = "description__contains", skip_serializing_if = "Option::is_none")]
    pub description_contains: Option<String>,
    /// List of the task statuses (array). Optional.
    /// Allowed values: `scheduled`, `waiting_approval`, `declined`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// List of Group IDs to filter by (array). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Keyword to search in scheduled tasks profile name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Date range for scheduled execution time. Optional.
    #[serde(rename = "scheduledTime__between", skip_serializing_if = "Option::is_none")]
    pub scheduled_time_between: Option<String>,
    /// IDs of target scopes of scheduled task (array). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_scope: Option<String>,
    /// Keyword to search in task ID (array). Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by (array). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The column to sort the results by. Optional.
    /// Allowed values: `id`, `name`, `description`, `scheduledTime`, `type`,
    /// `osTypes`, `outputDestination`, `createdAt`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// If true, only the total number of items is returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, the total number of items is not calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Keyword to search in task name (array). Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
}

impl GetScheduledTasksQuery {
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Keyword to search in scope name (comma-joined).
    pub fn scope_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scope_name_contains = Some(join_csv(v));
        self
    }
    /// List of output destinations (comma-joined).
    /// Allowed values: `SentinelCloud`, `Local`, `None`, `SingularityXDR`.
    pub fn output_destination<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.output_destination = Some(join_csv(v));
        self
    }
    /// IDs of creating user of scheduled task (comma-joined).
    pub fn creator_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.creator_id = Some(join_csv(v));
        self
    }
    /// Date range for creation time.
    pub fn created_at_between(mut self, v: impl Into<String>) -> Self {
        self.created_at_between = Some(v.into());
        self
    }
    /// List of Account IDs to filter by (comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// List of the task types (comma-joined).
    /// Allowed values: `action`, `dataCollection`, `artifactCollection`,
    /// `forensicsProfile`.
    pub fn r#type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.r#type = Some(join_csv(v));
        self
    }
    /// List of the OS types (comma-joined).
    /// Allowed values: `linux`, `macos`, `windows`.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// A list of scheduled task ids (comma-joined).
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// Keyword to search in description (comma-joined).
    pub fn description_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.description_contains = Some(join_csv(v));
        self
    }
    /// List of the task statuses (comma-joined).
    /// Allowed values: `scheduled`, `waiting_approval`, `declined`.
    pub fn status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.status = Some(join_csv(v));
        self
    }
    /// List of Group IDs to filter by (comma-joined).
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Keyword to search in scheduled tasks profile name.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Date range for scheduled execution time.
    pub fn scheduled_time_between(mut self, v: impl Into<String>) -> Self {
        self.scheduled_time_between = Some(v.into());
        self
    }
    /// IDs of target scopes of scheduled task (comma-joined).
    pub fn target_scope<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.target_scope = Some(join_csv(v));
        self
    }
    /// Keyword to search in task ID (comma-joined).
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(v));
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Site IDs to filter by (comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// The column to sort the results by.
    /// Allowed values: `id`, `name`, `description`, `scheduledTime`, `type`,
    /// `osTypes`, `outputDestination`, `createdAt`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// If true, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, the total number of items is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Keyword to search in task name (comma-joined).
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
}

// ---------------------------------------------------------------------------
// Request bodies
// ---------------------------------------------------------------------------

/// Body for
/// `DELETE /web/api/v2.1/remote-ops/data-exporter/destination-profiles`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDestinationProfilesBody {
    /// Filter selecting which Destination profiles to delete. Required.
    pub filter: DeleteDestinationProfilesFilter,
    /// Free-form data object. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Filter for [`DeleteDestinationProfilesBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDestinationProfilesFilter {
    /// List of Destination profile IDs to delete (max 5000). Required.
    pub ids: Vec<String>,
}

/// Body for
/// `POST /web/api/v2.1/remote-ops/data-exporter/destination-profiles`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostDestinationProfileBody {
    /// Destination profile name. Required.
    pub name: String,
    /// Write key of api account to upload data. Required.
    pub api_key: String,
    /// URL of api instance to upload the events. Required.
    pub api_url: String,
    /// Scope level to store the Destination profile. Required.
    /// Allowed values: `tenant`, `account`, `site`, `group`.
    pub scope_level: String,
    /// Scope ID to store the Destination profile. Required.
    pub scope_id: String,
    /// Destination profile type. Required. Allowed values: `skylight`.
    pub destination: String,
    /// Flag if the profile should be marked as default in its scope. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

/// Body for
/// `POST /web/api/v2.1/remote-ops/data-exporter/destination-profiles/set-default`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultDestinationProfileBody {
    /// Data payload. Required.
    pub data: SetDefaultDestinationProfileData,
}

/// Data payload for [`SetDefaultDestinationProfileBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDefaultDestinationProfileData {
    /// Profile Id to set as default profile. Required.
    pub profile_id: String,
    /// Scope level to get Destination profile configuration. Required.
    /// Allowed values: `tenant`, `account`, `site`, `group`.
    pub scope_level: String,
    /// Scope ID to get Destination profiles configuration. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

/// Body for
/// `PUT /web/api/v2.1/remote-ops/data-exporter/destination-profiles/{profile_id}`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutDestinationProfileBody {
    /// Data payload. Required.
    pub data: PutDestinationProfileData,
}

/// Data payload for [`PutDestinationProfileBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutDestinationProfileData {
    /// Destination profile name. Required.
    pub name: String,
    /// Write key of api account to upload data. Required.
    pub api_key: String,
    /// URL of api instance to upload the events. Required.
    pub api_url: String,
    /// Flag if the profile should be marked as default in its scope. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

/// Body for `POST /web/api/v2.1/remote-ops/schedule/forensics`.
///
/// The `data` payload is deeply nested (scheduling + action with
/// destination/password); it is modeled as [`serde_json::Value`] for fidelity
/// and flexibility.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleForensicsBody {
    /// Endpoints filter (scopes or IDs). Required.
    pub filter: ScheduleEndpointsFilter,
    /// Data payload: `scheduling` and `action`. Required. Freeform; see the
    /// SentinelOne API docs for `ScheduleForensicsCollectionRequestSchema`.
    pub data: serde_json::Value,
}

/// Body for `POST /web/api/v2.1/remote-ops/schedule/remote-script`.
///
/// The `data` payload is deeply nested (scheduling + action with many optional
/// script/destination fields); it is modeled as [`serde_json::Value`] for
/// fidelity and flexibility.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleRemoteScriptBody {
    /// Endpoints filter (scopes or IDs). Required.
    pub filter: ScheduleEndpointsFilter,
    /// Data payload: `scheduling` and `action`. Required. Freeform; see the
    /// SentinelOne API docs for `ScheduleRemoteScriptRequestSchema`.
    pub data: serde_json::Value,
}

/// Endpoints filter (scopes or IDs) used by the schedule endpoints.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleEndpointsFilter {
    /// Group ids (max 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// Account ids (max 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// Ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// Tenant. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Site ids (max 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
}

/// Body for `DELETE /web/api/v2.1/remote-ops/scheduled-tasks`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteScheduledTasksBody {
    /// Filter selecting which Scheduled tasks to delete. Required.
    pub filter: DeleteScheduledTasksFilter,
    /// Free-form data object. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Filter for [`DeleteScheduledTasksBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteScheduledTasksFilter {
    /// List of Scheduled Tasks IDs to delete (max 5000). Required.
    pub ids: Vec<String>,
}

/// Body for
/// `PUT /web/api/v2.1/remote-ops/scheduled-tasks/{scheduled_task_id}`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutScheduledTaskBody {
    /// Data payload. Required.
    pub data: PutScheduledTaskData,
}

/// Data payload for [`PutScheduledTaskBody`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PutScheduledTaskData {
    /// Scheduling payload (`scheduledTime`, `expireTime`, `recurrence`).
    /// Required. Freeform; see the SentinelOne API docs for
    /// `PutScheduledTaskRequestSchema`.
    pub scheduling: serde_json::Value,
}

/// Comma-join an iterator of string-likes for array query params.
fn join_csv<I, S>(values: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    values
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl RemoteOpsMmsService<'_> {
    /// **Delete multiple Destination profiles by ID** — Delete multiple
    /// Destination profiles. The profiles that are not possible to delete
    /// (e.g. non-existing or user does not have proper permissions) are
    /// skipped. IDs of successfully deleted profiles are returned in the
    /// response.
    ///
    /// `DELETE /web/api/v2.1/remote-ops/data-exporter/destination-profiles`
    pub async fn delete_destination_profiles(
        &self,
        body: &DeleteDestinationProfilesBody,
    ) -> Result<Response<DeletedDestinationProfilesResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json(
                Method::DELETE,
                "/web/api/v2.1/remote-ops/data-exporter/destination-profiles",
                None,
                Some(body),
            )
            .await?)
    }

    /// **Get available Destination profiles** — Get Destination profiles
    /// available for the specified scope. The profiles are inherited
    /// downwards, e.g. the profiles from parent Account and Tenant scopes are
    /// available for a Site. At most one of the returned destination profiles
    /// will be marked as default for the scope. If the scope does not have a
    /// default profile defined, it is inherited from the higher scope, unless
    /// inheritance was broken.
    ///
    /// `GET /web/api/v2.1/remote-ops/data-exporter/destination-profiles`
    pub async fn get_destination_profiles(
        &self,
        query: &GetDestinationProfilesQuery,
    ) -> Result<Response<Vec<DestinationProfile>>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-ops/data-exporter/destination-profiles", q)
            .await?)
    }

    /// **Create new Destination profile** — Create a Destination profile
    /// inside the specified scope. If the created profile is requested to be
    /// default, the default profile of the specified scope is overridden.
    ///
    /// `POST /web/api/v2.1/remote-ops/data-exporter/destination-profiles`
    pub async fn create_destination_profile(
        &self,
        body: &PostDestinationProfileBody,
    ) -> Result<Response<ProfileIdResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/remote-ops/data-exporter/destination-profiles", body)
            .await?)
    }

    /// **Set profile as default profile of the scope** — Set a profile as the
    /// default profile of the scope.
    ///
    /// `POST /web/api/v2.1/remote-ops/data-exporter/destination-profiles/set-default`
    pub async fn set_default_destination_profile(
        &self,
        body: &SetDefaultDestinationProfileBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        Ok(self
            .client
            .http()
            .post(
                "/web/api/v2.1/remote-ops/data-exporter/destination-profiles/set-default",
                body,
            )
            .await?)
    }

    /// **Delete Destination profile by ID** — Delete the Destination profile
    /// with the specified ID. If the profile was used as default for a scope,
    /// the last created profile will be marked as default for that scope.
    ///
    /// `DELETE /web/api/v2.1/remote-ops/data-exporter/destination-profiles/{profile_id}`
    pub async fn delete_destination_profile(
        &self,
        profile_id: impl Into<String>,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/remote-ops/data-exporter/destination-profiles/{}",
            profile_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<(), Response<serde_json::Value>>(Method::DELETE, &path, None, None)
            .await?)
    }

    /// **Get Destination profile by ID** — Get the Destination profile with
    /// the specified ID.
    ///
    /// `GET /web/api/v2.1/remote-ops/data-exporter/destination-profiles/{profile_id}`
    pub async fn get_destination_profile(
        &self,
        profile_id: impl Into<String>,
        query: &GetDestinationProfileQuery,
    ) -> Result<Response<DestinationProfile>, Error> {
        let path = format!(
            "/web/api/v2.1/remote-ops/data-exporter/destination-profiles/{}",
            profile_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// **Update existing Destination profile** — Update the contents of the
    /// existing Destination profile with the specified ID. All the profile
    /// data should be specified, even if the values are not changed. If the
    /// updated profile is requested to be default, the default profile of its
    /// scope is modified.
    ///
    /// `PUT /web/api/v2.1/remote-ops/data-exporter/destination-profiles/{profile_id}`
    pub async fn update_destination_profile(
        &self,
        profile_id: impl Into<String>,
        body: &PutDestinationProfileBody,
    ) -> Result<Response<ProfileIdResult>, Error> {
        let path = format!(
            "/web/api/v2.1/remote-ops/data-exporter/destination-profiles/{}",
            profile_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json(Method::PUT, &path, None, Some(body))
            .await?)
    }

    /// **Get results sent to data exporter** — Get the results sent to the
    /// data exporter.
    ///
    /// `GET /web/api/v2.1/remote-ops/data-exporter/results`
    pub async fn get_data_exporter_results(
        &self,
        query: &GetDataExporterResultsQuery,
    ) -> Result<Response<SkylightUploadResults>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-ops/data-exporter/results", q)
            .await?)
    }

    /// **Schedule forensics for future run** — Schedule forensics for a future
    /// run. The profile will be scheduled for execution on all endpoints
    /// matching the filter.
    ///
    /// `POST /web/api/v2.1/remote-ops/schedule/forensics`
    pub async fn schedule_forensics(
        &self,
        body: &ScheduleForensicsBody,
    ) -> Result<Response<ScheduleForensicsResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/remote-ops/schedule/forensics", body)
            .await?)
    }

    /// **Schedule remote script for future run** — Schedule a remote script
    /// for a future run. The script will be scheduled for execution on all
    /// endpoints matching the filter. If appropriate, a pending approval will
    /// be created for the execution.
    ///
    /// `POST /web/api/v2.1/remote-ops/schedule/remote-script`
    pub async fn schedule_remote_script(
        &self,
        body: &ScheduleRemoteScriptBody,
    ) -> Result<Response<ScheduleRemoteScriptResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/remote-ops/schedule/remote-script", body)
            .await?)
    }

    /// **Delete multiple scheduled tasks by ID** — Delete multiple Scheduled
    /// tasks. The tasks that are not possible to delete (e.g. non-existing or
    /// user does not have proper permissions) are skipped. IDs of successfully
    /// deleted tasks are returned in the response.
    ///
    /// `DELETE /web/api/v2.1/remote-ops/scheduled-tasks`
    pub async fn delete_scheduled_tasks(
        &self,
        body: &DeleteScheduledTasksBody,
    ) -> Result<Response<DeletedScheduledTasksResult>, Error> {
        Ok(self
            .client
            .http()
            .request_json(
                Method::DELETE,
                "/web/api/v2.1/remote-ops/scheduled-tasks",
                None,
                Some(body),
            )
            .await?)
    }

    /// **Get available Scheduled Tasks** — Get available Scheduled Tasks.
    ///
    /// `GET /web/api/v2.1/remote-ops/scheduled-tasks`
    pub async fn get_scheduled_tasks(
        &self,
        query: &GetScheduledTasksQuery,
    ) -> Result<Paginated<ScheduledTask>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-ops/scheduled-tasks", q)
            .await?)
    }

    /// **Update existing Scheduled task** — Update an existing Scheduled task.
    ///
    /// `PUT /web/api/v2.1/remote-ops/scheduled-tasks/{scheduled_task_id}`
    pub async fn update_scheduled_task(
        &self,
        scheduled_task_id: impl Into<String>,
        body: &PutScheduledTaskBody,
    ) -> Result<Response<ScheduledTaskIdResult>, Error> {
        let path = format!(
            "/web/api/v2.1/remote-ops/scheduled-tasks/{}",
            scheduled_task_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json(Method::PUT, &path, None, Some(body))
            .await?)
    }
}
