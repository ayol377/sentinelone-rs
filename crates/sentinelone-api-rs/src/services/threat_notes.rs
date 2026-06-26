use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::threat_notes::{
    SuccessResponse, ThreatNote, ThreatNoteResult, ThreatsNoteCreateBody, UpdateThreatNoteBody,
};
use crate::pagination::{Paginated, Response};

/// `Threat Notes` tag.
///
/// Threat Notes REST API.
pub struct ThreatNotesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/threats/{threat_id}/notes`
/// (Get Threat Notes).
///
/// Every field is optional. Builder methods set each one.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetThreatNotesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional. Example: `150`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional. Example: `10`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional. Example: `"YWdlbnRfaWQ6NTgwMjkzODE="`.
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
    /// The column to sort the results by. Optional.
    /// Allowed values: `createdAt`, `updatedAt`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Threat Note creator name (partial or full). Optional. Example: `"John"`.
    #[serde(rename = "creator__like", skip_serializing_if = "Option::is_none")]
    pub creator_like: Option<String>,
    /// Threat Note creator ID. Optional. Example: `"225494730938493804"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
}

impl GetThreatNotesQuery {
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
    /// The column to sort the results by. Allowed values: `createdAt`,
    /// `updatedAt`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Threat Note creator name (partial or full).
    pub fn creator_like(mut self, v: impl Into<String>) -> Self {
        self.creator_like = Some(v.into());
        self
    }
    /// Threat Note creator ID.
    pub fn creator_id(mut self, v: impl Into<String>) -> Self {
        self.creator_id = Some(v.into());
        self
    }
}

impl ThreatNotesService<'_> {
    /// `POST /web/api/v2.1/threats/notes` — Add Note to Multiple.
    ///
    /// Add a threat note to multiple threats.
    pub async fn add_note_to_multiple(
        &self,
        body: &ThreatsNoteCreateBody,
    ) -> Result<Response<ThreatNoteResult>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/threats/notes", body)
            .await?)
    }

    /// `GET /web/api/v2.1/threats/{threat_id}/notes` — Get Threat Notes.
    ///
    /// Get the threat notes that match the filter.
    pub async fn get_threat_notes(
        &self,
        threat_id: impl Into<String>,
        query: &GetThreatNotesQuery,
    ) -> Result<Paginated<ThreatNote>, Error> {
        let path = format!("/web/api/v2.1/threats/{}/notes", threat_id.into());
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `DELETE /web/api/v2.1/threats/{threat_id}/notes/{note_id}` — Delete
    /// Threat Note.
    ///
    /// Delete a threat note.
    pub async fn delete_threat_note(
        &self,
        threat_id: impl Into<String>,
        note_id: impl Into<String>,
    ) -> Result<Response<SuccessResponse>, Error> {
        let path = format!(
            "/web/api/v2.1/threats/{}/notes/{}",
            threat_id.into(),
            note_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<(), Response<SuccessResponse>>(Method::DELETE, &path, None, None)
            .await?)
    }

    /// `PUT /web/api/v2.1/threats/{threat_id}/notes/{note_id}` — Update Threat
    /// Note.
    ///
    /// Change the text of a threat note.
    pub async fn update_threat_note(
        &self,
        threat_id: impl Into<String>,
        note_id: impl Into<String>,
        body: &UpdateThreatNoteBody,
    ) -> Result<Response<ThreatNote>, Error> {
        let path = format!(
            "/web/api/v2.1/threats/{}/notes/{}",
            threat_id.into(),
            note_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<UpdateThreatNoteBody, Response<ThreatNote>>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }
}
