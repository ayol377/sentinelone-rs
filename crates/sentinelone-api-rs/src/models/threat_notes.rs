//! Models for the `Threat Notes` tag.
//!
//! Covers the typed response and request shapes for the four Threat Notes
//! endpoints. Threat note entity fields are all optional in the spec (the
//! `data` items carry no `required` array), so every field is modelled as
//! `Option<T>` for "default null" fidelity.

use serde::{Deserialize, Serialize};

/// A single threat note.
///
/// Entity for `threats.schemas_ThreatNoteSchema` (`data` of the single-resource
/// and list responses). None of these fields are listed as `required` in the
/// spec, so all are `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatNote {
    /// Threat Note ID. Optional/nullable -> `Option`.
    /// Example: `"225494730938493804"`.
    #[serde(default)]
    pub id: Option<String>,
    /// Threat Note text. Optional/nullable -> `Option`.
    /// Length 2..=10000. Example: `"Discovered using analysis"`.
    #[serde(default)]
    pub text: Option<String>,
    /// Threat Note creator name. Optional/nullable -> `Option`.
    /// Example: `"John Doe"`.
    #[serde(default)]
    pub creator: Option<String>,
    /// Threat Note creator id. Optional/nullable -> `Option`.
    /// Example: `"225494730938493804"`.
    #[serde(default)]
    pub creator_id: Option<String>,
    /// Identifies if the note changed. Optional/nullable -> `Option`.
    #[serde(default)]
    pub edited: Option<bool>,
    /// Timestamp of date creation (ISO-8601 string). Optional/nullable ->
    /// `Option`. Example: `"2018-02-27T04:49:26.257525Z"`.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Timestamp of last update (ISO-8601 string). Optional/nullable ->
    /// `Option`. Example: `"2018-02-27T04:49:26.257525Z"`.
    #[serde(default)]
    pub updated_at: Option<String>,
}

/// Result of adding a note to multiple threats.
///
/// Entity for `threats.schemas_ThreatNoteResultSchema` (`data` of the
/// `POST /web/api/v2.1/threats/notes` response).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatNoteResult {
    /// Number of entities affected by the requested operation.
    /// Optional/nullable -> `Option`.
    #[serde(default)]
    pub affected: Option<i64>,
    /// Result details for each threat. Optional/nullable -> `Option`.
    #[serde(default)]
    pub details: Option<Vec<ThreatNoteResultDetail>>,
}

/// Per-threat detail entry within [`ThreatNoteResult`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatNoteResultDetail {
    /// Threat id. Optional/nullable -> `Option`.
    /// Example: `"225494730938493804"`.
    #[serde(default)]
    pub threat_id: Option<String>,
}

/// Generic success indicator returned by the delete endpoint.
///
/// Entity for `_SuccessResponseSchema` (`data` of the
/// `DELETE /web/api/v2.1/threats/{threat_id}/notes/{note_id}` response).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse {
    /// Indicates a successful operation. Optional/nullable -> `Option`.
    #[serde(default)]
    pub success: Option<bool>,
}

/// Request body for `POST /web/api/v2.1/threats/notes` (Add Note to Multiple).
///
/// Maps `threats.schemas_ThreatsNoteCreateSchema`. Both `data` and `filter` are
/// in the schema `required` array, so both are non-optional. The `filter`
/// object is large and freeform (it mirrors the full Threats filter set), so it
/// is modelled as [`serde_json::Value`] per the freeform-body convention.
#[derive(Debug, Clone, Serialize)]
pub struct ThreatsNoteCreateBody {
    /// Data. Required -> non-`Option`. Carries the note `text`.
    pub data: ThreatNoteTextData,
    /// Filter. Required -> non-`Option`. Selects which threats receive the
    /// note (account/site/group ids, threat ids, time ranges, verdicts, etc.).
    /// Freeform object -> [`serde_json::Value`].
    pub filter: serde_json::Value,
}

impl ThreatsNoteCreateBody {
    /// Construct the body from the note `text` and a freeform `filter` object.
    pub fn new(text: impl Into<String>, filter: serde_json::Value) -> Self {
        Self {
            data: ThreatNoteTextData { text: text.into() },
            filter,
        }
    }
}

/// Request body for `PUT /web/api/v2.1/threats/{threat_id}/notes/{note_id}`
/// (Update Threat Note).
///
/// Maps `threats.schemas_PostThreatNoteDataSchema`. `data` is in the schema
/// `required` array, so it is non-optional.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateThreatNoteBody {
    /// Data. Required -> non-`Option`. Carries the new note `text`.
    pub data: ThreatNoteTextData,
}

impl UpdateThreatNoteBody {
    /// Construct the body from the new note `text`.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            data: ThreatNoteTextData { text: text.into() },
        }
    }
}

/// The `data` object shared by the create and update bodies.
#[derive(Debug, Clone, Serialize)]
pub struct ThreatNoteTextData {
    /// Threat Note text. Required -> non-`Option`. Length 2..=10000.
    /// Example: `"Discovered using analysis"`.
    pub text: String,
}
