//! Models for the `Agent Support Actions` tag.

use serde::Deserialize;

/// Result of an Agent support action (the `data` envelope of
/// `_AffectedResultsSchema_200`).
///
/// Returned by mutating/action endpoints that report how many Agents were
/// affected by the requested operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentSupportActionResult {
    /// Number of entities affected by the requested operation.
    ///
    /// Optional / nullable in the spec (not in the schema `required` array)
    /// -> `Option<i64>`.
    pub affected: Option<i64>,
}
