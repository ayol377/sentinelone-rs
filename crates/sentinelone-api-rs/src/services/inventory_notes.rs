use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_notes::InventoryNotesPayload;
use crate::pagination::Response;

/// `Inventory Notes` tag.
///
/// Inventory Notes Resources.
pub struct InventoryNotesService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `POST /web/api/v2.1/xdr/assets/notes`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInventoryNoteQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl CreateInventoryNoteQuery {
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
}

/// Query params for `DELETE /web/api/v2.1/xdr/assets/notes`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteInventoryNoteQuery {
    /// List of Account IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Comma-joined. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl DeleteInventoryNoteQuery {
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
}

/// Join an iterator of string-like values into a comma-separated string.
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

impl InventoryNotesService<'_> {
    /// `POST /web/api/v2.1/xdr/assets/notes` — Create or update note against
    /// asset.
    ///
    /// create or update note.
    ///
    /// The spec documents no `200` response schema for this endpoint (only
    /// `400`/`401`), so the success envelope is returned as a freeform
    /// [`serde_json::Value`].
    pub async fn create_or_update(
        &self,
        query: &CreateInventoryNoteQuery,
        body: &InventoryNotesPayload,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<InventoryNotesPayload, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/notes",
                q,
                Some(body),
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/xdr/assets/notes` — Delete note.
    ///
    /// Delete note.
    ///
    /// The spec documents no `200` response schema for this endpoint (only
    /// `400`/`401`), so the success envelope is returned as a freeform
    /// [`serde_json::Value`].
    pub async fn delete(
        &self,
        query: &DeleteInventoryNoteQuery,
        body: &InventoryNotesPayload,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<InventoryNotesPayload, Response<serde_json::Value>>(
                Method::DELETE,
                "/web/api/v2.1/xdr/assets/notes",
                q,
                Some(body),
            )
            .await?)
    }
}
