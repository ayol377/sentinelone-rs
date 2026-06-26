//! Models for the `Cloud Resources` tag.
//!
//! Field nullability mirrors the SentinelOne 2.1 spec: a field is a bare type
//! only when it is in the schema `required` array *and* not `x-nullable`;
//! otherwise it is `Option<T>` (the API's "default null" behaviour).

use serde::Deserialize;

/// Response envelope for `GET /web/api/v2.1/cloudnative/cloud-rogues`.
///
/// This endpoint uses a non-standard envelope: `data` is an *object*
/// (`{ resources: [...] }`) rather than a bare array, and `pagination` is a
/// sibling of `data`. It therefore does not fit the generic `Paginated<T>`
/// envelope and is modelled explicitly here.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudResourcesResponse {
    /// Response data wrapper holding the matched cloud resources.
    ///
    /// Optional/nullable per spec (only `pagination` is in the `required` set).
    pub data: Option<CloudResourcesData>,
    /// Pagination information. Always present (in the schema `required` set).
    pub pagination: CloudResourcesPagination,
    /// Errors array (freeform objects). Nullable per spec.
    pub errors: Option<serde_json::Value>,
}

/// `data` wrapper for [`CloudResourcesResponse`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudResourcesData {
    /// The matched cloud rogue resources.
    ///
    /// Optional/nullable per spec (not in any `required` set).
    pub resources: Option<Vec<CloudResource>>,
}

/// Pagination information for [`CloudResourcesResponse`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudResourcesPagination {
    /// Total number of items found matching your query. Example: `580`.
    ///
    /// Required and non-nullable per spec.
    pub total_items: i64,
    /// Pass this value as `cursor` on your next request to get the next page of
    /// results (will be `null` when the last page is reached).
    /// Example: `"YWdlbnRfaWQ6NTgwMjkzODE="`.
    ///
    /// Optional/nullable per spec.
    pub next_cursor: Option<String>,
}

/// A single cloud rogue resource.
///
/// Returned (inside `data.resources`) by
/// `GET /web/api/v2.1/cloudnative/cloud-rogues`.
///
/// No field is in the schema `required` array, so every field is `Option<T>`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloudResource {
    /// Cloud Resource ID. Example: `"225494730938493804"`.
    pub id: Option<String>,
    /// Resource name.
    pub name: Option<String>,
    /// Region.
    pub region: Option<String>,
    /// Cloud provider account ID.
    pub cloud_provider_account_id: Option<String>,
    /// Cloud provider account name.
    pub cloud_provider_account_name: Option<String>,
    /// Cloud provider name.
    pub cloud_provider_name: Option<String>,
    /// Cloud provider organization.
    pub cloud_provider_organization: Option<String>,
    /// OS type.
    pub os_type: Option<String>,
    /// OS type icon.
    pub os_type_icon: Option<String>,
    /// Resource type.
    pub resource_type: Option<String>,
    /// Internal resource type.
    pub internal_resource_type: Option<String>,
    /// Virtual network ID.
    pub virtual_network_id: Option<String>,
    /// Image ID.
    pub image_id: Option<String>,
    /// Concatenated tags (free-text representation of the resource tags).
    pub concatenated_tags: Option<String>,
    /// Resource tags (freeform; `readOnly` with no declared type in the spec).
    pub tags: Option<serde_json::Value>,
    /// Resource creation time (UTC string).
    pub created_time: Option<String>,
}
