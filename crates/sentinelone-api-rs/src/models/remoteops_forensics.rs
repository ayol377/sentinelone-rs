//! Models for the `RemoteOps Forensics` tag.
//!
//! RemoteOps Forensics lets you collect forensic artifacts from Agents using
//! reusable Collection profiles, track the resulting collection tasks, and
//! download the produced collection files.
//!
//! Field nullability follows the OpenAPI spec exactly: a field is a bare type
//! only when it is in the schema `required` array and not `x-nullable`;
//! otherwise it is `Option<T>` (the default-null behaviour). Enum fields are
//! represented as `String` for forward compatibility, with the allowed values
//! documented inline.

use serde::Deserialize;

// ===========================================================================
// Collection profile (full content) — GET/POST/PUT/DELETE collection-profiles
// ===========================================================================

/// A Forensics Collection profile with full artifact contents.
///
/// Returned (as a single resource) by `GET/POST/PUT
/// /web/api/v2.1/remote-ops/forensics/collection-profiles[/{profile_id}]`, and
/// (as a list) by `DELETE
/// /web/api/v2.1/remote-ops/forensics/collection-profiles`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProfile {
    /// Collection profile ID. Required.
    pub id: String,
    /// Name of collection profile in db. Required.
    pub name: String,
    /// Collection profile description. Required (present in `required`) but
    /// `x-nullable`, so `Option`.
    #[serde(default)]
    pub description: Option<String>,
    /// Type of RemoteOps Action. Allowed values: `forensicsProfile`. Required.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Flag indicating if the Collection profile is bundled (provided by S1).
    /// Required.
    pub is_bundled: bool,
    /// Collection profile version. Required.
    pub version: String,
    /// Email of user who created the profile. Required.
    pub creator: String,
    /// Email of user who update the profile. Required.
    pub updater: String,
    /// Possible target OS types of the collection profile. Allowed values per
    /// item: `linux`, `macos`, `windows`. Required.
    pub os_types: Vec<String>,
    /// Scope level where the Collection profile is stored. Allowed values:
    /// `tenant`, `account`, `site`, `group`, `sentinel`. Required.
    pub scope_level: String,
    /// Scope ID where the Collection profile is stored. Required.
    pub scope_id: String,
    /// Scope name where the Collection profile is stored. Required (present in
    /// `required`) but `x-nullable`, so `Option`.
    #[serde(default)]
    pub scope_name: Option<String>,
    /// Full path of Scope where the Collection profile is stored. Required.
    pub scope_path: String,
    /// Timestamp (date-time) when the profile was created. Required.
    pub created_at: String,
    /// Timestamp (date-time) when the profile was updated. Optional/nullable.
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Artifacts to collect. Optional/nullable.
    #[serde(default)]
    pub artifacts: Option<Vec<CollectionProfileArtifact>>,
    /// Profile name in camel case representation used for localization, only
    /// relevant for default profiles. Optional/nullable.
    #[serde(default)]
    pub name_key: Option<String>,
}

/// A single artifact specification inside a [`CollectionProfile`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProfileArtifact {
    /// Type of artifact to collect (e.g. `users`). Required.
    pub artifact_type: String,
    /// OS type where the artifact will be collected. Allowed values: `linux`,
    /// `macos`, `windows`. Required.
    pub os_type: String,
    /// Input parameters for the artifact. Optional/nullable.
    #[serde(default)]
    pub parameters: Option<serde_json::Value>,
}

// ===========================================================================
// Collection profile (list summary) — GET collection-profiles
// ===========================================================================

/// A Forensics Collection profile summary, as returned in the list endpoint.
///
/// Returned (as a list) by `GET
/// /web/api/v2.1/remote-ops/forensics/collection-profiles`. Unlike
/// [`CollectionProfile`] this summary omits the `artifacts` content and adds
/// the `creatorId`/`updaterId` fields.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionProfileSummary {
    /// Collection profile ID. Required.
    pub id: String,
    /// Name of collection profile in db. Required.
    pub name: String,
    /// Collection profile description. Required (present in `required`) but
    /// `x-nullable`, so `Option`.
    #[serde(default)]
    pub description: Option<String>,
    /// Type of RemoteOps Action. Allowed values: `forensicsProfile`. Required.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Flag indicating if the Collection profile is bundled (provided by S1).
    /// Required.
    pub is_bundled: bool,
    /// Collection profile version. Required.
    pub version: String,
    /// Email of user who created the profile. Required.
    pub creator: String,
    /// ID of user who created the profile. Required.
    pub creator_id: String,
    /// Email of user who update the profile. Required.
    pub updater: String,
    /// ID of user who updated the profile. Required.
    pub updater_id: String,
    /// Possible target OS types of the collection profile. Allowed values per
    /// item: `linux`, `macos`, `windows`. Required.
    pub os_types: Vec<String>,
    /// Scope level where the Collection profile is stored. Allowed values:
    /// `tenant`, `account`, `site`, `group`, `sentinel`. Required.
    pub scope_level: String,
    /// Scope ID where the Collection profile is stored. Required.
    pub scope_id: String,
    /// Scope name where the Collection profile is stored. Required (present in
    /// `required`) but `x-nullable`, so `Option`.
    #[serde(default)]
    pub scope_name: Option<String>,
    /// Full path of Scope where the Collection profile is stored. Required.
    pub scope_path: String,
    /// Timestamp (date-time) when the profile was created. Required.
    pub created_at: String,
    /// Timestamp (date-time) when the profile was updated. Optional/nullable.
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Profile name in camel case representation used for localization, only
    /// relevant for default profiles. Optional/nullable.
    #[serde(default)]
    pub name_key: Option<String>,
}

// ===========================================================================
// Artifact types — GET artifact-types
// ===========================================================================

/// A supported artifact type.
///
/// Returned (as a list) by `GET
/// /web/api/v2.1/remote-ops/forensics/artifact-types`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactType {
    /// Artifact type code name, used as identifier of the artifact. Required.
    pub artifact_type: String,
    /// User-readable name of the artifact. Required.
    pub name: String,
    /// Category of the artifact type. Required.
    pub category: String,
    /// OS types where the artifact can be collected. Allowed values per item:
    /// `linux`, `macos`, `windows`. Required.
    pub os_types: Vec<String>,
    /// Category key for translation. Optional/nullable.
    #[serde(default)]
    pub category_key: Option<String>,
    /// Parameters of the artifact. Optional/nullable.
    #[serde(default)]
    pub parameters: Option<Vec<ArtifactTypeParameter>>,
    /// Internal version of the artifact type. Optional/nullable.
    #[serde(default)]
    pub version: Option<i64>,
}

/// A parameter accepted by an [`ArtifactType`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactTypeParameter {
    /// Unique key of artifact parameter. Required.
    pub key: String,
    /// Type of artifact parameter. Allowed values: `str`, `pathlist`, `int`,
    /// `dict`. Required.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Default value of artifact parameter, null if the parameter does not
    /// allow default. Required (present in `required`) — kept `Option` because
    /// it may be JSON `null`.
    #[serde(default)]
    pub default: Option<String>,
    /// Artifact parameter description. Optional/nullable.
    #[serde(default)]
    pub description: Option<String>,
    /// Example of the parameter value. Optional/nullable.
    #[serde(default)]
    pub example: Option<String>,
}

// ===========================================================================
// Task result — GET task-result
// ===========================================================================

/// Result of a collection task.
///
/// Returned (as a single resource) by `GET
/// /web/api/v2.1/remote-ops/forensics/task-result`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsTaskResult {
    /// Details of the collection profile used for collection. Optional.
    #[serde(default)]
    pub collection_profile: Option<ForensicsTaskCollectionProfile>,
    /// Details of the collection. Optional.
    #[serde(default)]
    pub collection: Option<ForensicsTaskCollection>,
    /// Details of the destination. Optional.
    #[serde(default)]
    pub destination: Option<ForensicsTaskDestination>,
    /// Details of the collection file, if exists. Optional/nullable.
    #[serde(default)]
    pub collection_file: Option<ForensicsTaskCollectionFile>,
    /// Link to Skylight view with results for the single task.
    /// Optional/nullable.
    #[serde(default)]
    pub skylight_results_url: Option<String>,
    /// Status of Skylight results. Optional/nullable.
    #[serde(default)]
    pub skylight_results_status: Option<ForensicsTaskSkylightStatus>,
    /// Link to Skylight view with results for parent task. Optional/nullable.
    #[serde(default)]
    pub skylight_parent_task_results_url: Option<String>,
    /// Collection file error. Optional.
    #[serde(default)]
    pub collection_file_error: Option<String>,
}

/// Collection profile reference inside a [`ForensicsTaskResult`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsTaskCollectionProfile {
    /// ID of used collection profile. Required.
    pub id: String,
    /// Name of used collection profile. Required.
    pub name: String,
}

/// Collection details inside a [`ForensicsTaskResult`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsTaskCollection {
    /// Tag of the collection. Required.
    pub tag: String,
    /// Description of the collection. Optional.
    #[serde(default)]
    pub description: Option<String>,
    /// Artifacts included in the collection. Optional.
    #[serde(default)]
    pub artifacts: Option<Vec<ForensicsTaskArtifact>>,
}

/// A single artifact entry inside a [`ForensicsTaskCollection`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsTaskArtifact {
    /// Type of the artifact. Required.
    pub artifact_type: String,
    /// Detailed status of the artifact collection. Required.
    pub detailed_status: String,
    /// Target Os. Required (present in `required`) but `x-nullable`, so
    /// `Option`.
    #[serde(default)]
    pub os_type: Option<String>,
    /// Status of the artifact collection. Required.
    pub status: String,
    /// Parameters passed to the artifact collector. Optional/nullable.
    #[serde(default)]
    pub parameters: Option<serde_json::Value>,
}

/// Destination details inside a [`ForensicsTaskResult`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsTaskDestination {
    /// ID of destination profile used for collection. Optional/nullable.
    #[serde(default)]
    pub profile_id: Option<String>,
}

/// Collection file details inside a [`ForensicsTaskResult`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsTaskCollectionFile {
    /// File Signature. Required.
    pub signature: String,
    /// Signature type. Optional.
    #[serde(default)]
    pub signature_type: Option<String>,
    /// Site id. Required.
    pub site_id: String,
    /// Agent id. Required.
    pub agent_id: String,
    /// Uploaded timestamp. Required.
    pub uploaded_timestamp: String,
}

/// Skylight results status inside a [`ForensicsTaskResult`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForensicsTaskSkylightStatus {
    /// Indicates if the collection contains no data store in Skylight.
    /// Required.
    pub is_empty: bool,
    /// Indicates if there were failures during uploading data to Skylight.
    /// Required.
    pub has_failures: bool,
    /// Last error message if there were failures during upload.
    /// Optional/nullable.
    #[serde(default)]
    pub error_message: Option<String>,
}

// ===========================================================================
// Collection file URL — GET collection-file-url
// ===========================================================================

/// A pre-signed download URL for a collection file.
///
/// Returned (as a single resource) by `GET
/// /web/api/v2.1/remote-ops/forensics/collection-file-url`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionFileDownload {
    /// Download link for the file. Optional.
    #[serde(default)]
    pub download_url: Option<String>,
    /// The name of the file. Optional.
    #[serde(default)]
    pub file_name: Option<String>,
}

// ===========================================================================
// Is-collection-file — GET is-collection-file
// ===========================================================================

/// Collection-file existence/metadata for a given storyline.
///
/// Returned (as a single resource) by `GET
/// /web/api/v2.1/remote-ops/forensics/is-collection-file`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionFileInfo {
    /// Agent's ID. Required.
    pub agent_id: String,
    /// File's Signature. Required.
    pub signature: String,
    /// Site's ID. Required.
    pub site_id: String,
    /// Signature type. Optional.
    #[serde(default)]
    pub signature_type: Option<String>,
    /// Collection file uploaded DateTime iso-formatted. Optional.
    #[serde(default)]
    pub uploaded_timestamp: Option<String>,
}

// ===========================================================================
// Start collection — POST start-collection (202)
// ===========================================================================

/// Result of starting a Forensics collection.
///
/// Returned (as a single resource) by `POST
/// /web/api/v2.1/remote-ops/forensics/start-collection`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartCollectionResult {
    /// Number of entities affected by the requested operation. Optional.
    #[serde(default)]
    pub affected: Option<i64>,
    /// The parent task ID of the started collection. Optional/nullable.
    #[serde(default)]
    pub parent_task_id: Option<String>,
    /// Counts of agents not included in collection task. Optional.
    #[serde(default)]
    pub skipped: Option<StartCollectionSkipped>,
}

/// Breakdown of agents skipped when starting a collection.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartCollectionSkipped {
    /// Feature is not enabled for the agent. Optional.
    #[serde(default)]
    pub not_enabled_feature: Option<i64>,
    /// Agent is currently not available for execution. Optional.
    #[serde(default)]
    pub not_available: Option<i64>,
    /// Agent is outside of collection profile scope. Optional.
    #[serde(default)]
    pub not_applicable_scope: Option<i64>,
    /// Count of agents matching filter with not applicable OS. Optional.
    #[serde(default)]
    pub not_applicable_os: Option<i64>,
    /// Count of agents matching filter not supporting the collection. Optional.
    #[serde(default)]
    pub not_supported: Option<i64>,
    /// Count of agents matching filter skipped because of other reason.
    /// Optional.
    #[serde(default)]
    pub other: Option<i64>,
}
