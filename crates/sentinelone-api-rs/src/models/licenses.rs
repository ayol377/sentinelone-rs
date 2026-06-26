//! Response models for the `licenses` tag.

use serde::Deserialize;

/// `data` payload of `PUT /web/api/v2.1/licenses/update-sites-modules`
/// (`_AffectedResultsSchema_200`).
///
/// Reports how many entities were affected by the requested operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LicensesAffected {
    /// Number of entities affected by the requested operation.
    ///
    /// Optional/nullable in spec (not in any `required` array) -> `Option`.
    pub affected: Option<i64>,
}
