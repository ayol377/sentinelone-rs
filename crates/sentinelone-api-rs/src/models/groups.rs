//! Response models for the `Groups` tag.

use serde::Deserialize;

/// A SentinelOne Group as returned by the list endpoint
/// (`groups.groups_SummarizedGroupSchema`).
///
/// Returned by `GET /web/api/v2.1/groups`. Note: the list (summarized) schema
/// carries a few extra fields (`totalAgents`, `filterName`, `inherits`) that the
/// single-resource schema omits. The spec marks none of the data fields as
/// required, so every field is optional/nullable.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    /// Id. Optional/nullable.
    #[serde(default)]
    pub id: Option<String>,
    /// Timestamp of group creation (date-time string, e.g.
    /// `2018-02-27T04:49:26.257525Z`). Optional/nullable.
    #[serde(default)]
    pub created_at: Option<String>,
    /// Timestamp of last update (date-time string). Optional/nullable.
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Name. Optional/nullable.
    #[serde(default)]
    pub name: Option<String>,
    /// The user-defined description for the Group. Optional/nullable.
    #[serde(default)]
    pub description: Option<String>,
    /// The id of the site this group is part of. Optional/nullable.
    #[serde(default)]
    pub site_id: Option<String>,
    /// True only for the default group of the Site. Optional/nullable.
    #[serde(default)]
    pub is_default: Option<bool>,
    /// The user that created the group. Optional/nullable.
    #[serde(default)]
    pub creator: Option<String>,
    /// The ID of the user that created the group. Optional/nullable.
    #[serde(default)]
    pub creator_id: Option<String>,
    /// Group type. Optional/nullable. Allowed values: `static`, `dynamic`,
    /// `pinned`.
    #[serde(default, rename = "type")]
    pub type_: Option<String>,
    /// If the group is dynamic, id of the filter which is used to associate
    /// agents. Optional/nullable.
    #[serde(default)]
    pub filter_id: Option<String>,
    /// The rank sets the priority of a dynamic group over others.
    /// Optional/nullable.
    #[serde(default)]
    pub rank: Option<i64>,
    /// [DEPRECATED] token generation moved to dedicated endpoint
    /// `/groups/<group_id>/token`. Read-only. Optional/nullable.
    #[serde(default)]
    pub registration_token: Option<String>,
    /// Count of agents in the group. Present only on the list (summarized)
    /// schema. Optional/nullable.
    #[serde(default)]
    pub total_agents: Option<i64>,
    /// If the group is dynamic, the name of the filter which is used to
    /// associate agents. Present only on the list (summarized) schema.
    /// Optional/nullable.
    #[serde(default)]
    pub filter_name: Option<String>,
    /// True if the policy is inherited from Site, False if the group has its own
    /// edited policy. Present only on the list (summarized) schema.
    /// Optional/nullable.
    #[serde(default)]
    pub inherits: Option<bool>,
}

/// Registration token returned when regenerating a Group token
/// (`groups.groups_RegenerateKeySchema`).
///
/// Returned by `PUT /web/api/v2.1/groups/{group_id}/regenerate-key`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegenerateKey {
    /// Registration token. Optional/nullable.
    #[serde(default)]
    pub registration_token: Option<String>,
}

/// Group registration token (`groups.groups_GroupTokenGenerationSchema`).
///
/// Returned by `GET /web/api/v2.1/groups/{group_id}/token`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupToken {
    /// Token. Optional/nullable.
    #[serde(default)]
    pub token: Option<String>,
}

/// Generic success indicator (`_SuccessResponseSchema`).
///
/// Returned by `DELETE /web/api/v2.1/groups/{group_id}` and
/// `PUT /web/api/v2.1/groups/{group_id}/revert-policy`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupSuccess {
    /// Indicates a successful operation. Optional/nullable.
    #[serde(default)]
    pub success: Option<bool>,
}
