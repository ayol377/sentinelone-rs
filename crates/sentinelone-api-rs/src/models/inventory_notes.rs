//! Models for the `Inventory Notes` tag.
//!
//! The two Inventory Notes endpoints (`POST`/`DELETE`
//! `/web/api/v2.1/xdr/assets/notes`) only document `400`/`401` responses in the
//! spec (no `200` schema), so successful responses are returned as a freeform
//! [`serde_json::Value`] envelope. The only typed shape here is the request
//! payload, [`InventoryNotesPayload`].

use serde::{Deserialize, Serialize};

/// Request payload for the Inventory Notes endpoints
/// (`v2_1.inventory.notes.schemas_InventoryNotesPayloadSchema`).
///
/// The spec defines no `required` fields, so every field is optional and
/// serialized only when set.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryNotesPayload {
    /// Id. Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Note. Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Resource id. Optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

impl InventoryNotesPayload {
    /// Construct an empty payload.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the note `id`.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    /// Set the `note` text.
    pub fn note(mut self, v: impl Into<String>) -> Self {
        self.note = Some(v.into());
        self
    }
    /// Set the `resourceId` (asset/resource the note is attached to).
    pub fn resource_id(mut self, v: impl Into<String>) -> Self {
        self.resource_id = Some(v.into());
        self
    }
}
