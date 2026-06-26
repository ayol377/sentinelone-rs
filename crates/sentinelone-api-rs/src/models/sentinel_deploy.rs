//! Models for the `Sentinel Deploy` tag.
//!
//! Sentinel deploy views and operations (Ranger auto-deploy cred groups).

use serde::Deserialize;

/// A Cred Group row (Cred Groups table).
///
/// Returned by `GET`/`POST /web/api/v2.1/ranger/cred-groups`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredGroup {
    /// The cred group name. Required, non-nullable.
    pub group_name: String,
    /// Encrypted passphrase with key unknown by the management. Required,
    /// non-nullable.
    pub group_passphrase: String,
    /// Scope id. Required, non-nullable.
    pub scope_id: String,
    /// The domain associated to this cred group. Optional.
    pub domain: Option<String>,
    /// The os type for this cred group. Optional.
    ///
    /// Allowed values: `windows`, `osx_linux`. Default `windows`.
    pub target_os: Option<String>,
    /// The cred group id. Optional.
    pub id: Option<String>,
    /// The number of cred details in the group. Optional.
    pub total_details: Option<i64>,
}

/// A Cred Group Detail row (Cred Groups details table).
///
/// Returned by `GET /web/api/v2.1/ranger/cred-groups/details` and
/// `PUT /web/api/v2.1/ranger/cred-groups/details/{detail_id}`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredGroupDetail {
    /// The title for the cred. Required, non-nullable.
    pub title: String,
    /// The type of the cred group. Required, non-nullable.
    pub cred_type: String,
    /// The detail id. Optional.
    pub id: Option<String>,
    /// The cred group id. Optional.
    pub cred_group_id: Option<String>,
    /// The last update time (date-time string). Optional.
    pub updated_at: Option<String>,
    /// The creation time (date-time string). Optional.
    pub created_at: Option<String>,
    /// The user that created the details. Optional.
    pub created_by: Option<String>,
    /// The user that updated the details. Optional.
    pub updated_by: Option<String>,
}

/// Generic success indicator returned by delete/add operations.
///
/// Returned by `DELETE`/`POST` cred-group operations that wrap
/// `_SuccessResponseSchema`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredGroupSuccess {
    /// Indicates a successful operation. Optional.
    pub success: Option<bool>,
}
