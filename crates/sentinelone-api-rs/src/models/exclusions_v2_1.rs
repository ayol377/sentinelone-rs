//! Models for the `Exclusions v2.1` tag.
//!
//! The unified-exclusion entity itself is returned by the API as an open
//! object (the spec defines `data.items` with empty `properties: {}`), so it
//! is modelled as a freeform [`serde_json::Value`] alias ([`UnifiedExclusion`])
//! for forward-compatibility. The action-metadata and import-report shapes are
//! fully specified, so those are given named structs.

use serde::Deserialize;

/// A single unified exclusion.
///
/// The SentinelOne spec leaves the per-exclusion object open (its `properties`
/// map is empty in `swagger_2_1.json`) because the concrete shape depends on
/// the exclusion `type` (white_hash, path, certificate, file_type, browser,
/// commandline, container_native, idr, ...). It is therefore kept as a
/// freeform JSON value so no fields are lost across API versions.
pub type UnifiedExclusion = serde_json::Value;

/// Result of a delete operation (`DELETE /web/api/v2.1/unified-exclusions`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedExclusionDeleteResult {
    /// Number of entities affected by the requested operation.
    ///
    /// Optional/nullable in the spec (not in the parent `required` array).
    pub affected: Option<i64>,
}

/// Result of an import operation (`POST /web/api/v2.1/unified-exclusions/import`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportUnifiedExclusionsResult {
    /// The number of rows in the file. Optional/nullable.
    pub total: Option<i64>,
    /// The number of entries that imported successfully. Optional/nullable.
    pub succeeded: Option<i64>,
    /// The ID of the Validation Report generated for the import. It can help
    /// you fix entries that did not import successfully. Optional/nullable.
    pub report_id: Option<String>,
}

/// A child action within a [`UnifiedExclusionAction`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedExclusionActionChild {
    /// Action programmatic name. Required (in parent `required` array).
    pub name: String,
    /// Human-readable action title. Required.
    pub title: String,
    /// Display/order index. Required.
    pub order: i64,
    /// Whether the action is currently disabled. Required.
    pub is_disabled: bool,
    /// Reason the action is disabled, if any. Optional/nullable.
    pub disabled_reason: Option<String>,
}

/// An available action for the matched exclusions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedExclusionAction {
    /// Action programmatic name. Required (in parent `required` array).
    pub name: String,
    /// Human-readable action title. Required.
    pub title: String,
    /// Display/order index. Required.
    pub order: i64,
    /// Whether the action is currently disabled. Required.
    pub is_disabled: bool,
    /// Reason the action is disabled, if any. Optional/nullable.
    pub disabled_reason: Option<String>,
    /// Nested child actions. Optional/nullable.
    pub children: Option<Vec<UnifiedExclusionActionChild>>,
}

/// Response data for
/// `GET /web/api/v2.1/unified-exclusions/available-actions`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnifiedExclusionsActions {
    /// Available actions. Required (in parent `required` array).
    pub actions: Vec<UnifiedExclusionAction>,
    /// The exclusions the actions apply to (freeform objects). Required.
    pub exclusions: Vec<UnifiedExclusion>,
}
