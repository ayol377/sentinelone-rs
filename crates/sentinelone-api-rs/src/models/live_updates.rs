//! Models for the `Live Updates` tag.

use serde::Deserialize;

/// An Agent's merged content/live update entry.
///
/// Returned by `GET /web/api/v2.1/content-updates-inventory`.
/// No field is in the schema `required` array, so every field is `Option` (default null).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveUpdate {
    /// Live update category. Example: `"Security Update"`.
    pub asset_family_type: Option<String>,
    /// Live update type name. Example: `"Behavioral AI"`.
    pub display_name: Option<String>,
    /// Live update ID.
    pub live_update_id: Option<String>,
    /// Agent content update ID.
    pub agent_content_update_id: Option<String>,
    /// Timestamp of when the update was applied (date-time string).
    /// Example: `"2018-02-27T04:49:26.257525Z"`.
    pub applied_at: Option<String>,
}
