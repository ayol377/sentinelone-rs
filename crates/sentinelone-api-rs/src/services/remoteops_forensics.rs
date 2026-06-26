//! Service for the `RemoteOps Forensics` tag — RemoteOps Forensics operations.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::remoteops_forensics::{
    ArtifactType, CollectionFileDownload, CollectionFileInfo, CollectionProfile,
    CollectionProfileSummary, ForensicsTaskResult, StartCollectionResult,
};
use crate::pagination::{Paginated, Response};

/// `RemoteOps Forensics` tag — RemoteOps Forensics operations.
///
/// Collect forensic artifacts from Agents using reusable Collection profiles,
/// manage those profiles, track collection tasks, and download collection
/// files.
pub struct RemoteopsForensicsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ===========================================================================
// Query params: GET collection-file-url
// ===========================================================================

/// Query params for `GET
/// /web/api/v2.1/remote-ops/forensics/collection-file-url`.
///
/// All five fields are required by the spec; they default to empty strings
/// (the API will reject blank values). Use the builder methods to populate.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionFileUrlQuery {
    /// Site id. Example: "225494730938493804". Required.
    pub site_id: String,
    /// Agent id. Example: "225494730938493804". Required.
    pub agent_id: String,
    /// Signature. Required.
    pub signature: String,
    /// Signature type. Required.
    pub signature_type: String,
    /// Uploaded timestamp. Required.
    pub uploaded_timestamp: String,
}

impl CollectionFileUrlQuery {
    /// Site id. Example: "225494730938493804". Required.
    pub fn site_id(mut self, v: impl Into<String>) -> Self {
        self.site_id = v.into();
        self
    }
    /// Agent id. Example: "225494730938493804". Required.
    pub fn agent_id(mut self, v: impl Into<String>) -> Self {
        self.agent_id = v.into();
        self
    }
    /// Signature. Required.
    pub fn signature(mut self, v: impl Into<String>) -> Self {
        self.signature = v.into();
        self
    }
    /// Signature type. Required.
    pub fn signature_type(mut self, v: impl Into<String>) -> Self {
        self.signature_type = v.into();
        self
    }
    /// Uploaded timestamp. Required.
    pub fn uploaded_timestamp(mut self, v: impl Into<String>) -> Self {
        self.uploaded_timestamp = v.into();
        self
    }
}

// ===========================================================================
// Query params: GET collection-profiles (list)
// ===========================================================================

/// Query params for `GET
/// /web/api/v2.1/remote-ops/forensics/collection-profiles`.
///
/// Array params (`accountIds`, `ids`, `osTypes`, `siteIds`) are serialized
/// comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProfilesQuery {
    /// List of Account IDs to filter by. Example:
    /// "225494730938493804,225494730938493915". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`. Example: "asc". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Fetch auto triggering compatible profiles. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_triggering_compatible: Option<bool>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Example: "150". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A list of collection profiles IDs. Example:
    /// "225494730938493804,225494730938493915". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Os types. Allowed values per item: `linux`, `macos`, `windows`.
    /// Example: "linux". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Keyword to search in Collection profile name / description. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The column to sort the results by. Allowed values: `id`, `createdAt`,
    /// `mgmtId`, `scopeId`, `name`, `osTypes`, `version`, `scopeLevel`.
    /// Example: "id". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// List of Site IDs to filter by. Example:
    /// "225494730938493804,225494730938493915". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
}

impl CollectionProfilesQuery {
    /// List of Account IDs to filter by. Comma-joined.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Fetch auto triggering compatible profiles.
    pub fn auto_triggering_compatible(mut self, v: bool) -> Self {
        self.auto_triggering_compatible = Some(v);
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// A list of collection profiles IDs. Comma-joined.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// Os types. Allowed values per item: `linux`, `macos`, `windows`.
    /// Comma-joined.
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(v));
        self
    }
    /// Keyword to search in Collection profile name / description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `createdAt`,
    /// `mgmtId`, `scopeId`, `name`, `osTypes`, `version`, `scopeLevel`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// List of Site IDs to filter by. Comma-joined.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
}

// ===========================================================================
// Query params: GET is-collection-file
// ===========================================================================

/// Query params for `GET
/// /web/api/v2.1/remote-ops/forensics/is-collection-file`.
///
/// Both fields are required by the spec.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IsCollectionFileQuery {
    /// Storyline ID. Required.
    pub storyline: String,
    /// Agent's ID. Example: "225494730938493804". Required.
    pub agent_id: String,
}

impl IsCollectionFileQuery {
    /// Storyline ID. Required.
    pub fn storyline(mut self, v: impl Into<String>) -> Self {
        self.storyline = v.into();
        self
    }
    /// Agent's ID. Example: "225494730938493804". Required.
    pub fn agent_id(mut self, v: impl Into<String>) -> Self {
        self.agent_id = v.into();
        self
    }
}

// ===========================================================================
// Query params: GET task-result
// ===========================================================================

/// Query params for `GET /web/api/v2.1/remote-ops/forensics/task-result`.
///
/// `taskId` is required by the spec.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskResultQuery {
    /// Task id. Example: "225494730938493804". Required.
    pub task_id: String,
}

impl TaskResultQuery {
    /// Task id. Example: "225494730938493804". Required.
    pub fn task_id(mut self, v: impl Into<String>) -> Self {
        self.task_id = v.into();
        self
    }
}

// ===========================================================================
// Body: DELETE collection-profiles
// ===========================================================================

/// Body for `DELETE
/// /web/api/v2.1/remote-ops/forensics/collection-profiles`
/// (`v2_1.forensics.schema_DeleteProfilesRequestSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProfilesBody {
    /// Data. Optional freeform object (empty in the spec).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Filter. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DeleteProfilesFilter>,
}

impl DeleteProfilesBody {
    /// Set the filter selecting which profiles to delete.
    pub fn filter(mut self, filter: DeleteProfilesFilter) -> Self {
        self.filter = Some(filter);
        self
    }
    /// Set the freeform `data` object.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Filter for [`DeleteProfilesBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteProfilesFilter {
    /// List of Collection profile IDs to delete (max 5000). Required.
    pub ids: Vec<String>,
    /// List of Account IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    /// List of Site IDs to filter by (1-500 items). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<String>>,
}

impl DeleteProfilesFilter {
    /// Create a filter from the required list of profile IDs to delete.
    pub fn new<I, S>(ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            ids: ids.into_iter().map(Into::into).collect(),
            account_ids: None,
            site_ids: None,
        }
    }
    /// List of Account IDs to filter by (1-500 items).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// List of Site IDs to filter by (1-500 items).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

// ===========================================================================
// Body: POST collection-profiles (create)
// ===========================================================================

/// A single artifact specification used in create/update profile bodies.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileArtifactInput {
    /// Type of artifact to collect (e.g. "users"). Required.
    pub artifact_type: String,
    /// OS type where the artifact will be collected. Allowed values: `linux`,
    /// `macos`, `windows`. Required.
    pub os_type: String,
    /// Input parameters for the artifact. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

impl ProfileArtifactInput {
    /// Create an artifact input from the required `artifactType` and `osType`.
    pub fn new(artifact_type: impl Into<String>, os_type: impl Into<String>) -> Self {
        Self {
            artifact_type: artifact_type.into(),
            os_type: os_type.into(),
            parameters: None,
        }
    }
    /// Input parameters for the artifact.
    pub fn parameters(mut self, v: serde_json::Value) -> Self {
        self.parameters = Some(v);
        self
    }
}

/// Body for `POST
/// /web/api/v2.1/remote-ops/forensics/collection-profiles`
/// (`v2_1.forensics.schema_CollectionProfileRequestSchema`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionProfileBody {
    /// Data. Required.
    pub data: CreateCollectionProfileData,
}

/// `data` payload for [`CreateCollectionProfileBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCollectionProfileData {
    /// Collection profile name. Required.
    pub name: String,
    /// Scope level of the collection profile. Allowed values: `tenant`,
    /// `account`, `site`, `group`. Required.
    pub scope_level: String,
    /// Artifacts (at least 1). Required.
    pub artifacts: Vec<ProfileArtifactInput>,
    /// Collection profile description. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Scope ID of the collection profile. Example: "225494730938493804".
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl CreateCollectionProfileData {
    /// Create from the required `name`, `scopeLevel` and `artifacts`.
    pub fn new(
        name: impl Into<String>,
        scope_level: impl Into<String>,
        artifacts: Vec<ProfileArtifactInput>,
    ) -> Self {
        Self {
            name: name.into(),
            scope_level: scope_level.into(),
            artifacts,
            description: None,
            scope_id: None,
        }
    }
    /// Collection profile description.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// Scope ID of the collection profile.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

// ===========================================================================
// Body: PUT collection-profiles/{profile_id} (update)
// ===========================================================================

/// Body for `PUT
/// /web/api/v2.1/remote-ops/forensics/collection-profiles/{profile_id}`
/// (`v2_1.forensics.schema_PutCollectionProfileRequestSchema`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCollectionProfileBody {
    /// Data. Required.
    pub data: UpdateCollectionProfileData,
}

/// `data` payload for [`UpdateCollectionProfileBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCollectionProfileData {
    /// Collection profile name. Required.
    pub name: String,
    /// Artifacts (at least 1). Required.
    pub artifacts: Vec<ProfileArtifactInput>,
    /// Collection profile description. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateCollectionProfileData {
    /// Create from the required `name` and `artifacts`.
    pub fn new(name: impl Into<String>, artifacts: Vec<ProfileArtifactInput>) -> Self {
        Self {
            name: name.into(),
            artifacts,
            description: None,
        }
    }
    /// Collection profile description.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
}

// ===========================================================================
// Body: POST start-collection
// ===========================================================================

/// Body for `POST /web/api/v2.1/remote-ops/forensics/start-collection`
/// (`remote_ops.schemas_StartCollectionSchema`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartCollectionBody {
    /// Data. Required.
    pub data: StartCollectionData,
    /// Filter specification of targeted agents. Required.
    ///
    /// This is the large, deeply nested agent filter; it is kept as freeform
    /// JSON for forward compatibility (it supports the full Agents filter set,
    /// e.g. `siteIds`, `accountIds`, `groupIds`, `ids`, and many more).
    pub filter: serde_json::Value,
}

impl StartCollectionBody {
    /// Create from the required `data` and `filter`.
    pub fn new(data: StartCollectionData, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// `data` payload for [`StartCollectionBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartCollectionData {
    /// ID of the Collection Profile that will be used. Example:
    /// "225494730938493804". Required.
    pub collection_profile_id: String,
    /// Destination. Required.
    pub destination: StartCollectionDestination,
    /// Description of the collection (min length 2). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Tag identifier of the collection. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl StartCollectionData {
    /// Create from the required `collectionProfileId` and `destination`.
    pub fn new(
        collection_profile_id: impl Into<String>,
        destination: StartCollectionDestination,
    ) -> Self {
        Self {
            collection_profile_id: collection_profile_id.into(),
            destination,
            description: None,
            tag: None,
        }
    }
    /// Description of the collection (min length 2).
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// Tag identifier of the collection.
    pub fn tag(mut self, v: impl Into<String>) -> Self {
        self.tag = Some(v.into());
        self
    }
}

/// `destination` payload for [`StartCollectionData`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartCollectionDestination {
    /// ID of profile for destination of exported collection data. Example:
    /// "225494730938493804". Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
    /// Password for encrypting uploaded binary artifacts. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Used to specify execution where a generic password is used.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_from_scope: Option<StartCollectionPasswordFromScope>,
}

impl StartCollectionDestination {
    /// ID of profile for destination of exported collection data.
    pub fn profile_id(mut self, v: impl Into<String>) -> Self {
        self.profile_id = Some(v.into());
        self
    }
    /// Password for encrypting uploaded binary artifacts.
    pub fn password(mut self, v: impl Into<String>) -> Self {
        self.password = Some(v.into());
        self
    }
    /// Used to specify execution where a generic password is used.
    pub fn password_from_scope(mut self, v: StartCollectionPasswordFromScope) -> Self {
        self.password_from_scope = Some(v);
        self
    }
}

/// `passwordFromScope` payload for [`StartCollectionDestination`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartCollectionPasswordFromScope {
    /// User scope. Allowed values: `tenant`, `account`, `site`. Required.
    pub scope_level: String,
    /// String repr. of scope id. Example: "225494730938493804". Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl StartCollectionPasswordFromScope {
    /// Create from the required `scopeLevel`.
    pub fn new(scope_level: impl Into<String>) -> Self {
        Self {
            scope_level: scope_level.into(),
            scope_id: None,
        }
    }
    /// String repr. of scope id.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

// ===========================================================================
// Helpers
// ===========================================================================

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
// Service methods
// ===========================================================================

impl RemoteopsForensicsService<'_> {
    /// `GET /web/api/v2.1/remote-ops/forensics/artifact-types` — Get list of
    /// supported artifact types.
    ///
    /// Return a complete list of supported artifact types.
    ///
    /// The response is a plain `{ data: [...] }` list with no pagination
    /// envelope, so this returns `Response<Vec<ArtifactType>>`.
    pub async fn artifact_types(&self) -> Result<Response<Vec<ArtifactType>>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-ops/forensics/artifact-types", None)
            .await?)
    }

    /// `GET /web/api/v2.1/remote-ops/forensics/collection-file-url` — Returns
    /// collection file download pre-signed url.
    ///
    /// Returns collection file download pre-signed url.
    pub async fn collection_file_url(
        &self,
        query: &CollectionFileUrlQuery,
    ) -> Result<Response<CollectionFileDownload>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-ops/forensics/collection-file-url", q)
            .await?)
    }

    /// `DELETE /web/api/v2.1/remote-ops/forensics/collection-profiles` —
    /// Delete Collection profiles.
    ///
    /// Delete multiple Forensics Collection profiles. The profiles that are not
    /// possible to delete (e.g. bundled profiles by S1, non-existing or user
    /// does not have proper permissions) are skipped. Contents of successfully
    /// deleted profiles are returned in response.
    ///
    /// The response is a plain `{ data: [...] }` list with no pagination
    /// envelope, so this returns `Response<Vec<CollectionProfile>>`.
    pub async fn delete_collection_profiles(
        &self,
        body: &DeleteProfilesBody,
    ) -> Result<Response<Vec<CollectionProfile>>, Error> {
        Ok(self
            .client
            .http()
            .request_json::<DeleteProfilesBody, Response<Vec<CollectionProfile>>>(
                Method::DELETE,
                "/web/api/v2.1/remote-ops/forensics/collection-profiles",
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/remote-ops/forensics/collection-profiles` — Get list
    /// of available Collection profiles.
    ///
    /// Get list of available Forensics collection profiles. The list may be
    /// narrowed by specifying filter parameter. Profiles are inherited between
    /// scopes in both upward and downward directions, e.g. profiles on parent
    /// Account and Tenant scopes are returned when querying for a Site scope,
    /// and profiles on a Site scopes are returned when querying its parent
    /// Account. Bundled profiles are available regardless of requested scope.
    /// If scope is not specified in filter, the scopes of the requesting user
    /// are considered.
    pub async fn list_collection_profiles(
        &self,
        query: &CollectionProfilesQuery,
    ) -> Result<Paginated<CollectionProfileSummary>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-ops/forensics/collection-profiles", q)
            .await?)
    }

    /// `POST /web/api/v2.1/remote-ops/forensics/collection-profiles` — Create
    /// new Collection profile.
    ///
    /// Create a Forensics Collection profile with provided artifacts on the
    /// specified scope. The profile name must be unique inside the scope, if
    /// the name already exists, Bad request error is returned.
    pub async fn create_collection_profile(
        &self,
        body: &CreateCollectionProfileBody,
    ) -> Result<Response<CollectionProfile>, Error> {
        Ok(self
            .client
            .http()
            .post(
                "/web/api/v2.1/remote-ops/forensics/collection-profiles",
                body,
            )
            .await?)
    }

    /// `GET
    /// /web/api/v2.1/remote-ops/forensics/collection-profiles/{profile_id}` —
    /// Get Collection profile by ID.
    ///
    /// Get contents of an existing Forensics Collection profile, including
    /// specification of artifacts to be collected and profile metadata.
    ///
    /// `profile_id`: Profile ID. Example: "225494730938493804". Required.
    pub async fn get_collection_profile(
        &self,
        profile_id: impl Into<String>,
    ) -> Result<Response<CollectionProfile>, Error> {
        let path = format!(
            "/web/api/v2.1/remote-ops/forensics/collection-profiles/{}",
            profile_id.into()
        );
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `PUT
    /// /web/api/v2.1/remote-ops/forensics/collection-profiles/{profile_id}` —
    /// Update Collection profile by ID.
    ///
    /// Update contents of an existing Forensics Collection profile. All the
    /// profile data should be specified, even if the values are not changed.
    /// It's not allowed to change scope of profile. The name must be unique
    /// inside the scope, if different profile with specified name already
    /// exists, Bad request error is returned and no profile data is changed.
    ///
    /// `profile_id`: Profile ID. Example: "225494730938493804". Required.
    pub async fn update_collection_profile(
        &self,
        profile_id: impl Into<String>,
        body: &UpdateCollectionProfileBody,
    ) -> Result<Response<CollectionProfile>, Error> {
        let path = format!(
            "/web/api/v2.1/remote-ops/forensics/collection-profiles/{}",
            profile_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<UpdateCollectionProfileBody, Response<CollectionProfile>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/remote-ops/forensics/is-collection-file` — Check if
    /// collection file exists for given storyline.
    ///
    /// Check if collection file exists for given storyline.
    pub async fn is_collection_file(
        &self,
        query: &IsCollectionFileQuery,
    ) -> Result<Response<CollectionFileInfo>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-ops/forensics/is-collection-file", q)
            .await?)
    }

    /// `POST /web/api/v2.1/remote-ops/forensics/start-collection` — Start
    /// collection of Forensics artifacts according to specified profile.
    ///
    /// Start collection of Forensics artifacts according to specified profile.
    /// Returns HTTP 202 (Accepted) once the collection has been started.
    pub async fn start_collection(
        &self,
        body: &StartCollectionBody,
    ) -> Result<Response<StartCollectionResult>, Error> {
        Ok(self
            .client
            .http()
            .post(
                "/web/api/v2.1/remote-ops/forensics/start-collection",
                body,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/remote-ops/forensics/task-result` — Return result of
    /// collection task.
    ///
    /// Return result of collection task.
    pub async fn task_result(
        &self,
        query: &TaskResultQuery,
    ) -> Result<Response<ForensicsTaskResult>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/remote-ops/forensics/task-result", q)
            .await?)
    }
}
