use serde::Deserialize;

/// Reputation verdict for a hash known to the management console.
///
/// Returned by `GET /web/api/v2.1/hashes/{hash}/verdict`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HashVerdict {
    /// The hash verdict.
    ///
    /// Allowed values: `malicious`, `non-malicious`, `unknown`.
    ///
    /// Optional / nullable in the spec, so represented as `Option`.
    pub verdict: Option<String>,
}
