use serde::{Deserialize, Serialize};

/// A SentinelOne Agent.
///
/// The most-used fields are typed; every other field the API returns is kept in
/// [`Agent::extra`] (via `#[serde(flatten)]`) so nothing is lost — e.g. network
/// interfaces / IPs, OS details, scope, tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub id: String,
    pub uuid: Option<String>,
    pub computer_name: Option<String>,
    pub network_status: Option<String>,
    pub is_active: Option<bool>,
    pub last_active_date: Option<String>,
    pub agent_version: Option<String>,
    /// All other fields returned by the API (IPs, OS, scope, tags, …).
    #[serde(flatten, default)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
