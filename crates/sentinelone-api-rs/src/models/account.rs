use serde::Deserialize;

/// A SentinelOne Account (partial — full fields come from codegen).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub name: Option<String>,
    pub account_type: Option<String>,
    pub state: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub number_of_sites: Option<i64>,
    pub active_agents: Option<i64>,
}
