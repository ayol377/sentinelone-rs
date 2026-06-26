//! Models for the `VCS Integration` tag.
//!
//! VCS integration management and CICD scanner policy configuration. Response
//! envelopes (`{ data, pagination }`) are stripped by [`crate::pagination`];
//! the structs here describe the inner entities only.
//!
//! Field nullability follows the spec: a field is a bare `T` only when it is in
//! the definition's `required` array and not `x-nullable`; otherwise it is
//! `Option<T>` (default-null behaviour). Enum-typed fields are kept as `String`
//! for forward compatibility, with the allowed values documented inline.

use serde::Deserialize;

/// A SentinelOne tag (`S1Tag`) attached to a VCS integration or repository.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S1Tag {
    /// Tag key. Optional/nullable -> `Option`.
    pub key: Option<String>,
    /// SentinelOne tag id. Optional/nullable -> `Option`.
    pub s1_tag_id: Option<String>,
    /// Tag value. Optional/nullable -> `Option`.
    pub value: Option<String>,
}

/// Scope information (`ScopeInfo`) for a VCS integration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeInfo {
    /// Management id. Optional/nullable -> `Option`.
    pub mgmt_id: Option<String>,
    /// Scope id. Optional/nullable -> `Option`.
    pub scope_id: Option<String>,
    /// Scope type. Optional/nullable -> `Option`.
    pub scope_type: Option<String>,
}

/// VCS integration settings (`VCSSettings`).
///
/// All fields are listed in the definition's `required` array and are not
/// nullable, so each is a bare `T`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VcsSettings {
    /// Alert severity for new developer repositories. Required.
    /// Allowed values: `CRITICAL`, `HIGH`, `MEDIUM`, `LOW`.
    pub dev_new_repository_alert_severity: String,
    /// Monitor new public developer repositories. Required.
    pub monitor_dev_new_public_repositories: bool,
    /// Monitor new public organization repositories. Required.
    pub monitor_org_new_public_repositories: bool,
    /// Alert severity for new organization repositories. Required.
    /// Allowed values: `CRITICAL`, `HIGH`, `MEDIUM`, `LOW`.
    pub org_new_repository_alert_severity: String,
    /// Scan public developer repositories. Required.
    pub scan_dev_public_repositories: bool,
    /// Scan private organization repositories. Required.
    pub scan_org_private_repositories: bool,
    /// Scan public organization repositories. Required.
    pub scan_org_public_repositories: bool,
}

/// A configured VCS integration (`VCSIntegration`).
///
/// The definition declares no `required` fields, so every field is `Option`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VcsIntegration {
    /// Tags attached to the integration. Optional/nullable -> `Option`.
    pub tags: Option<Vec<S1Tag>>,
    /// Creation timestamp (ISO-8601 string). Optional/nullable -> `Option`.
    pub created_at: Option<String>,
    /// Identity that created the integration. Optional/nullable -> `Option`.
    pub created_by: Option<String>,
    /// Free-text description. Optional/nullable -> `Option`.
    pub description: Option<String>,
    /// Integration id. Optional/nullable -> `Option`.
    pub id: Option<String>,
    /// VCS provider. Optional/nullable -> `Option`.
    /// Allowed values: `GITHUB`, `GITLAB`, `BITBUCKET`, `GITHUB_ENTERPRISE`,
    /// `GITLAB_ENTERPRISE`, `AZURE_REPOS`.
    pub provider: Option<String>,
    /// Last repositories sync timestamp. Optional/nullable -> `Option`.
    pub repos_last_sync_at: Option<String>,
    /// SentinelOne tenancy id. Optional/nullable -> `Option`.
    pub s1_tenancy_id: Option<String>,
    /// Scope information. Optional/nullable -> `Option`.
    pub scope_info: Option<ScopeInfo>,
    /// Scope path. Optional/nullable -> `Option`.
    pub scope_path: Option<String>,
    /// Integration settings. Optional/nullable -> `Option`.
    pub settings: Option<VcsSettings>,
    /// Integration status. Optional/nullable -> `Option`.
    pub status: Option<String>,
    /// Integration title. Optional/nullable -> `Option`.
    pub title: Option<String>,
    /// VCS account title. Optional/nullable -> `Option`.
    pub vcs_account_title: Option<String>,
}

/// A repository under a VCS integration (`VCSRepository`).
///
/// The definition declares no `required` fields, so every field is `Option`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VcsRepository {
    /// Tags attached to the repository. Optional/nullable -> `Option`.
    pub tags: Option<Vec<S1Tag>>,
    /// Fully-qualified repository name. Optional/nullable -> `Option`.
    pub full_name: Option<String>,
    /// Whether the GitHub app is installed. Optional/nullable -> `Option`.
    pub github_installed: Option<bool>,
    /// Repository id. Optional/nullable -> `Option`.
    pub id: Option<String>,
    /// Repository name. Optional/nullable -> `Option`.
    pub name: Option<String>,
    /// Repository type. Optional/nullable -> `Option`.
    pub repo_type: Option<String>,
    /// Provider-side repository id. Optional/nullable -> `Option`.
    pub repository_id: Option<String>,
    /// Whether scanning is enabled. Optional/nullable -> `Option`.
    pub scan_enabled: Option<bool>,
    /// Parent VCS integration id. Optional/nullable -> `Option`.
    pub vcs_integration_id: Option<String>,
    /// Repository visibility (e.g. `PUBLIC`, `PRIVATE`). Optional/nullable.
    pub visibility: Option<String>,
}

/// Repositories container (`Repos`) returned by the list-repositories endpoint.
///
/// The list-repositories response wraps the repository array inside a `data`
/// object together with a top-level `pagination` block; the pagination is not
/// surfaced through this struct.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repos {
    /// Repositories in this page. Optional/nullable -> `Option`.
    pub repos: Option<Vec<VcsRepository>>,
}

/// Scanner-policy scope (`Scope`).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    /// Scope id. Optional/nullable -> `Option`.
    pub id: Option<String>,
    /// Scope type. Optional/nullable -> `Option`.
    #[serde(rename = "type")]
    pub r#type: Option<String>,
}

/// A VCS / CICD scanner policy (`ScannerPolicy`).
///
/// The definition declares no `required` fields, so every field is `Option`.
/// The freeform configuration blocks (`iacConfig`, `secretConfig`,
/// `vulnerabilityConfig`, `scopeInfo`) are typed as [`serde_json::Value`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScannerPolicy {
    /// Tags attached to the policy. Optional/nullable -> `Option`.
    pub tags: Option<Vec<S1Tag>>,
    /// Branch the policy applies to. Optional/nullable -> `Option`.
    pub branch: Option<String>,
    /// Creation timestamp. Optional/nullable -> `Option`.
    pub created_at: Option<String>,
    /// Identity that created the policy. Optional/nullable -> `Option`.
    pub created_by: Option<String>,
    /// Free-text description. Optional/nullable -> `Option`.
    pub description: Option<String>,
    /// IaC scanning configuration (freeform object). Optional/nullable.
    pub iac_config: Option<serde_json::Value>,
    /// Policy id. Optional/nullable -> `Option`.
    pub id: Option<String>,
    /// Policy name. Optional/nullable -> `Option`.
    pub name: Option<String>,
    /// Policy priority. Optional/nullable -> `Option`.
    pub priority: Option<i64>,
    /// SentinelOne tenancy id. Optional/nullable -> `Option`.
    pub s1_tenancy_id: Option<String>,
    /// Policy scope. Optional/nullable -> `Option`.
    pub scope: Option<Scope>,
    /// Scope information (freeform object). Optional/nullable -> `Option`.
    pub scope_info: Option<serde_json::Value>,
    /// Scope path. Optional/nullable -> `Option`.
    pub scope_path: Option<String>,
    /// Secret-scanning configuration (freeform object). Optional/nullable.
    pub secret_config: Option<serde_json::Value>,
    /// Last update timestamp. Optional/nullable -> `Option`.
    pub updated_at: Option<String>,
    /// Vulnerability-scanning configuration (freeform object). Optional/nullable.
    pub vulnerability_config: Option<serde_json::Value>,
}

/// Result data (`DeleteIntegrationData`) of deleting a VCS integration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteIntegrationData {
    /// Whether the integration app was uninstalled. Optional/nullable.
    pub uninstall: Option<bool>,
}

/// Result data (`AddScannerPolicyResponseData`) of creating a scanner policy.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddScannerPolicyResponseData {
    /// Id of the newly created policy. Optional/nullable -> `Option`.
    pub policy_id: Option<String>,
}

/// Result data (`MaxPriorityData`) of the max-allowed-priority endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxPriorityData {
    /// Maximum allowed priority value. Optional/nullable -> `Option`.
    pub max_allowed_priority: Option<i64>,
}

/// Result data (`TunnelUserResponseData`) of registering a tunnel user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TunnelUserResponseData {
    /// API server address. Optional/nullable -> `Option`.
    pub api_server_address: Option<String>,
    /// API server port. Optional/nullable -> `Option`.
    pub api_server_port: Option<i64>,
    /// Tunnel user password. Optional/nullable -> `Option`.
    pub password: Option<String>,
    /// Tunnel server address. Optional/nullable -> `Option`.
    pub server_address: Option<String>,
    /// Tunnel server port. Optional/nullable -> `Option`.
    pub server_port: Option<i64>,
    /// Tunnel port. Optional/nullable -> `Option`.
    pub tunnel_port: Option<i64>,
    /// Tunnel user name. Optional/nullable -> `Option`.
    pub user: Option<String>,
}

/// Onboarding/offboarding result (`VCSOnboardingResponse`).
///
/// Unlike most endpoints this response is a flat object (no `data` wrapper); it
/// is deserialized directly and carries the generated app URL.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VcsOnboardingResult {
    /// App URL to complete (off)boarding. Optional/nullable -> `Option`.
    pub app_url: Option<String>,
}
