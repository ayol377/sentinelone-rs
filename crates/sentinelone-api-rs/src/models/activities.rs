//! Models for the `Activities` tag.
//!
//! Field nullability mirrors the SentinelOne 2.1 spec: a field is a bare type
//! only when it is in the schema `required` array *and* not `x-nullable`;
//! otherwise it is `Option<T>` (the API's "default null" behaviour).

use serde::Deserialize;

/// A single system activity.
///
/// Returned by `GET /web/api/v2.1/activities`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Activity {
    /// Activity ID. Example: `"225494730938493804"`.
    ///
    /// Optional/nullable per spec (not in the `required` set).
    pub id: Option<String>,
    /// Activity UUID.
    pub activity_uuid: Option<String>,
    /// Activity creation time (UTC). Example: `"2018-02-27T04:49:26.257525Z"`.
    pub created_at: Option<String>,
    /// Activity last updated time (UTC). Example: `"2018-02-27T04:49:26.257525Z"`.
    pub updated_at: Option<String>,
    /// Activity type code.
    pub activity_type: Option<i64>,
    /// Extra activity-specific data (freeform object).
    pub data: Option<serde_json::Value>,
    /// Primary description.
    pub primary_description: Option<String>,
    /// Secondary description.
    pub secondary_description: Option<String>,
    /// Comments.
    pub comments: Option<String>,
    /// Agent's OS type (if applicable). Example: `"linux"`.
    ///
    /// Enum (documented for forward-compat; stored as `String`):
    /// `linux`, `macos`, `windows_legacy`, `windows`. Nullable per spec.
    pub os_family: Option<String>,
    /// Extra activity information.
    pub description: Option<String>,
    /// Threat file hash (if applicable).
    pub hash: Option<String>,
    /// Agent's new version (if applicable). Example: `"2.5.1.1320"`.
    pub agent_updated_version: Option<String>,
    /// The user who invoked the activity (if applicable). Example: `"225494730938493804"`.
    pub user_id: Option<String>,
    /// Related threat (if applicable). Example: `"225494730938493804"`.
    pub threat_id: Option<String>,
    /// Related agent (if applicable). Example: `"225494730938493804"`.
    pub agent_id: Option<String>,
    /// Related account id (if applicable). Example: `"225494730938493804"`.
    pub account_id: Option<String>,
    /// Related site id (if applicable). Example: `"225494730938493804"`.
    pub site_id: Option<String>,
    /// Related group id (if applicable). Example: `"225494730938493804"`.
    pub group_id: Option<String>,
    /// Related account name (if applicable).
    pub account_name: Option<String>,
    /// Related site name (if applicable).
    pub site_name: Option<String>,
    /// Related group name (if applicable).
    pub group_name: Option<String>,
}

/// An activity type definition.
///
/// Returned by `GET /web/api/v2.1/activities/types`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityType {
    /// Action described in the activity.
    pub action: Option<String>,
    /// Activity description template as seen in the activity page.
    pub description_template: Option<String>,
    /// Activity type ID.
    pub id: Option<i64>,
}

/// The Syslog message corresponding to the last matching activity.
///
/// Returned by `GET /web/api/v2.1/last-activity-as-syslog`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityAsMessage {
    /// ID of the activity.
    pub activity_id: Option<String>,
    /// Syslog message corresponding to the activity.
    pub syslog_message: Option<String>,
}
