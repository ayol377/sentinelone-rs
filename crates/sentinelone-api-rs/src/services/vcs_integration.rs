//! `VCS Integration` tag — VCS integration management and CICD scanner policy
//! configuration.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::vcs_integration::{
    AddScannerPolicyResponseData, DeleteIntegrationData, MaxPriorityData, Repos, S1Tag,
    ScannerPolicy, TunnelUserResponseData, VcsIntegration, VcsOnboardingResult,
};
use crate::pagination::{Paginated, Response};

/// `VCS Integration` tag.
///
/// APIs for VCS integration management and CICD scanner policy configuration:
/// onboard / update / delete integrations, list integrations and their
/// repositories, enable/disable scanning, manage repository and integration
/// tags, create/update/delete scanner policies, and register tunnel users for
/// self-hosted (enterprise) providers.
pub struct VcsIntegrationService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Query structs
//
// `scopeType` / `scopeIds` are required on most endpoints and are therefore
// non-`Option` fields populated via a `new(..)` constructor; optional params are
// `Option` with `skip_serializing_if` and a builder each.
// ---------------------------------------------------------------------------

/// Query params for `GET /web/api/v2.1/cnapp/vcs/filters/count`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FiltersCountQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl FiltersCountQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for
/// `GET /web/api/v2.1/cnapp/vcs/integration/{integrationId}/offboarding`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationOffboardingQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl IntegrationOffboardingQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for
/// `PUT /web/api/v2.1/cnapp/vcs/integration/{integrationId}/repos/disable-scan`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisableReposScanQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl DisableReposScanQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for
/// `PUT /web/api/v2.1/cnapp/vcs/integration/{integrationId}/repos/edit-tags`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditReposTagsQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl EditReposTagsQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for
/// `PUT /web/api/v2.1/cnapp/vcs/integration/{integrationId}/repos/enable-scan`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableReposScanQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl EnableReposScanQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for
/// `POST /web/api/v2.1/cnapp/vcs/integration/{integrationId}/repos/get-tags`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetReposTagsQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl GetReposTagsQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for
/// `DELETE /web/api/v2.1/cnapp/vcs/integration/{vcsIntegrationId}`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteIntegrationQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl DeleteIntegrationQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for
/// `PUT /web/api/v2.1/cnapp/vcs/integration/{vcsIntegrationId}`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIntegrationQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl UpdateIntegrationQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for
/// `GET /web/api/v2.1/cnapp/vcs/integration/{vcsIntegrationId}/repos`.
///
/// Every param is optional on this endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListIntegrationReposQuery {
    /// Scope type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<String>,
    /// Page size (the spec types this param as `string`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Pagination cursor. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Number of records to skip. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<String>,
}

impl ListIntegrationReposQuery {
    /// Scope type.
    pub fn scope_type(mut self, v: impl Into<String>) -> Self {
        self.scope_type = Some(v.into());
        self
    }
    /// Scope ids.
    pub fn scope_ids(mut self, v: impl Into<String>) -> Self {
        self.scope_ids = Some(v.into());
        self
    }
    /// Page size.
    pub fn limit(mut self, v: impl Into<String>) -> Self {
        self.limit = Some(v.into());
        self
    }
    /// Pagination cursor.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Number of records to skip.
    pub fn skip(mut self, v: impl Into<String>) -> Self {
        self.skip = Some(v.into());
        self
    }
}

/// Query params for
/// `PUT /web/api/v2.1/cnapp/vcs/integration/{vcsIntegrationId}/repos/resync`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResyncIntegrationReposQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl ResyncIntegrationReposQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for `GET /web/api/v2.1/cnapp/vcs/integrations`.
///
/// Every param is optional on this endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListIntegrationsQuery {
    /// Scope type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<String>,
    /// Page size (the spec types this param as `string`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Number of records to skip. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<String>,
}

impl ListIntegrationsQuery {
    /// Scope type.
    pub fn scope_type(mut self, v: impl Into<String>) -> Self {
        self.scope_type = Some(v.into());
        self
    }
    /// Scope ids.
    pub fn scope_ids(mut self, v: impl Into<String>) -> Self {
        self.scope_ids = Some(v.into());
        self
    }
    /// Page size.
    pub fn limit(mut self, v: impl Into<String>) -> Self {
        self.limit = Some(v.into());
        self
    }
    /// Number of records to skip.
    pub fn skip(mut self, v: impl Into<String>) -> Self {
        self.skip = Some(v.into());
        self
    }
}

/// Query params for `PUT /web/api/v2.1/cnapp/vcs/integrations/edit-tags`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditIntegrationsTagsQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl EditIntegrationsTagsQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for `POST /web/api/v2.1/cnapp/vcs/onboarding`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OnboardingQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl OnboardingQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for `GET /web/api/v2.1/cnapp/vcs/scanner-policies`.
///
/// Every param is optional on this endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListScannerPoliciesQuery {
    /// Scope type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// Scope ids. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_ids: Option<String>,
    /// Page size (the spec types this param as `string`). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    /// Number of records to skip. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<String>,
}

impl ListScannerPoliciesQuery {
    /// Scope type.
    pub fn scope_type(mut self, v: impl Into<String>) -> Self {
        self.scope_type = Some(v.into());
        self
    }
    /// Scope ids.
    pub fn scope_ids(mut self, v: impl Into<String>) -> Self {
        self.scope_ids = Some(v.into());
        self
    }
    /// Page size.
    pub fn limit(mut self, v: impl Into<String>) -> Self {
        self.limit = Some(v.into());
        self
    }
    /// Number of records to skip.
    pub fn skip(mut self, v: impl Into<String>) -> Self {
        self.skip = Some(v.into());
        self
    }
}

/// Query params for `POST /web/api/v2.1/cnapp/vcs/scanner-policy`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateScannerPolicyQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl CreateScannerPolicyQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for
/// `GET /web/api/v2.1/cnapp/vcs/scanner-policy/max-allowed-priority`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxAllowedPriorityQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
    /// Target scope type. Required.
    pub target_scope_type: String,
    /// Target scope id. Required.
    pub target_scope_id: String,
}

impl MaxAllowedPriorityQuery {
    /// Build with all four required params.
    pub fn new(
        scope_type: impl Into<String>,
        scope_ids: impl Into<String>,
        target_scope_type: impl Into<String>,
        target_scope_id: impl Into<String>,
    ) -> Self {
        Self {
            scope_type: scope_type.into(),
            scope_ids: scope_ids.into(),
            target_scope_type: target_scope_type.into(),
            target_scope_id: target_scope_id.into(),
        }
    }
}

/// Query params for
/// `DELETE /web/api/v2.1/cnapp/vcs/scanner-policy/{policyId}`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteScannerPolicyQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl DeleteScannerPolicyQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for `GET /web/api/v2.1/cnapp/vcs/scanner-policy/{policyId}`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScannerPolicyQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl GetScannerPolicyQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for `PUT /web/api/v2.1/cnapp/vcs/scanner-policy/{policyId}`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateScannerPolicyQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl UpdateScannerPolicyQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

/// Query params for `POST /web/api/v2.1/cnapp/vcs/tunnel/user`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TunnelUserQuery {
    /// Scope type. Required.
    pub scope_type: String,
    /// Scope ids. Required.
    pub scope_ids: String,
}

impl TunnelUserQuery {
    /// Build with the required `scopeType` and `scopeIds`.
    pub fn new(scope_type: impl Into<String>, scope_ids: impl Into<String>) -> Self {
        Self { scope_type: scope_type.into(), scope_ids: scope_ids.into() }
    }
}

// ---------------------------------------------------------------------------
// Shared body sub-objects
// ---------------------------------------------------------------------------

/// Settings block (`VCSSettings`) used by onboarding and update-integration
/// bodies.
///
/// All fields are required by the spec; once a settings block is supplied each
/// field is serialized.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VcsSettingsBody {
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

impl VcsSettingsBody {
    /// Alert severity for new developer repositories
    /// (`CRITICAL` | `HIGH` | `MEDIUM` | `LOW`).
    pub fn dev_new_repository_alert_severity(mut self, v: impl Into<String>) -> Self {
        self.dev_new_repository_alert_severity = v.into();
        self
    }
    /// Monitor new public developer repositories.
    pub fn monitor_dev_new_public_repositories(mut self, v: bool) -> Self {
        self.monitor_dev_new_public_repositories = v;
        self
    }
    /// Monitor new public organization repositories.
    pub fn monitor_org_new_public_repositories(mut self, v: bool) -> Self {
        self.monitor_org_new_public_repositories = v;
        self
    }
    /// Alert severity for new organization repositories
    /// (`CRITICAL` | `HIGH` | `MEDIUM` | `LOW`).
    pub fn org_new_repository_alert_severity(mut self, v: impl Into<String>) -> Self {
        self.org_new_repository_alert_severity = v.into();
        self
    }
    /// Scan public developer repositories.
    pub fn scan_dev_public_repositories(mut self, v: bool) -> Self {
        self.scan_dev_public_repositories = v;
        self
    }
    /// Scan private organization repositories.
    pub fn scan_org_private_repositories(mut self, v: bool) -> Self {
        self.scan_org_private_repositories = v;
        self
    }
    /// Scan public organization repositories.
    pub fn scan_org_public_repositories(mut self, v: bool) -> Self {
        self.scan_org_public_repositories = v;
        self
    }
}

/// Scope block (`Scope`) used by the create-scanner-policy body.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeBody {
    /// Scope id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Scope type. Optional.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ScopeBody {
    /// Scope id.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    /// Scope type.
    pub fn r#type(mut self, v: impl Into<String>) -> Self {
        self.r#type = Some(v.into());
        self
    }
}

// ---------------------------------------------------------------------------
// Body structs
//
// Every body is wrapped in a top-level required `data` object; each `*Body`
// therefore carries a single `data: *Data` field whose inner nullability
// follows the nested object's own `required` array.
// ---------------------------------------------------------------------------

/// Request body (`EnableDisableRepos`) for enable-scan / disable-scan.
#[derive(Debug, Default, Serialize)]
pub struct EnableDisableReposBody {
    /// Data. Required top-level wrapper.
    pub data: EnableDisableReposData,
}

/// Inner `data` object (`EnableDisableData`) for [`EnableDisableReposBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnableDisableReposData {
    /// Repository ids to enable/disable. Required.
    pub repository_ids: Vec<String>,
}

impl EnableDisableReposBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: EnableDisableReposData) -> Self {
        Self { data }
    }
}

impl EnableDisableReposData {
    /// Repository ids to enable/disable. Required.
    pub fn repository_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.repository_ids = v.into_iter().map(Into::into).collect();
        self
    }
}

/// Request body (`EditReposTags`) for editing repository tags.
#[derive(Debug, Default, Serialize)]
pub struct EditReposTagsBody {
    /// Data. Required top-level wrapper.
    pub data: EditReposTagsData,
}

/// Inner `data` object (`EditReposTagsData`) for [`EditReposTagsBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditReposTagsData {
    /// Tag action. Required.
    /// Allowed values: `ADD`, `REMOVE`, `REPLACE_ALL`, `CLEAR_ALL`.
    pub action: String,
    /// Repository ids to modify. Required.
    pub repository_ids: Vec<String>,
    /// Tag ids to apply. Required.
    pub tag_ids: Vec<String>,
}

impl EditReposTagsBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: EditReposTagsData) -> Self {
        Self { data }
    }
}

impl EditReposTagsData {
    /// Tag action (`ADD` | `REMOVE` | `REPLACE_ALL` | `CLEAR_ALL`). Required.
    pub fn action(mut self, v: impl Into<String>) -> Self {
        self.action = v.into();
        self
    }
    /// Repository ids to modify. Required.
    pub fn repository_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.repository_ids = v.into_iter().map(Into::into).collect();
        self
    }
    /// Tag ids to apply. Required.
    pub fn tag_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tag_ids = v.into_iter().map(Into::into).collect();
        self
    }
}

/// Request body (`FetchReposTags`) for fetching repository tags.
#[derive(Debug, Default, Serialize)]
pub struct FetchReposTagsBody {
    /// Data. Required top-level wrapper.
    pub data: FetchReposTagsData,
}

/// Inner `data` object (`FetchReposTagsData`) for [`FetchReposTagsBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchReposTagsData {
    /// Repository ids whose tags to fetch. Required.
    pub repository_ids: Vec<String>,
}

impl FetchReposTagsBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: FetchReposTagsData) -> Self {
        Self { data }
    }
}

impl FetchReposTagsData {
    /// Repository ids whose tags to fetch. Required.
    pub fn repository_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.repository_ids = v.into_iter().map(Into::into).collect();
        self
    }
}

/// Request body (`UpdateVCSIntegration`) for updating an integration.
#[derive(Debug, Default, Serialize)]
pub struct UpdateVcsIntegrationBody {
    /// Data. Required top-level wrapper.
    pub data: UpdateVcsIntegrationData,
}

/// Inner `data` object (`UpdateIntegrationData`) for
/// [`UpdateVcsIntegrationBody`]. Declares no required fields.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVcsIntegrationData {
    /// Tag ids to set (min 1 item). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// Free-text description (max length 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Integration settings. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<VcsSettingsBody>,
    /// Integration title (max length 200). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UpdateVcsIntegrationBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: UpdateVcsIntegrationData) -> Self {
        Self { data }
    }
}

impl UpdateVcsIntegrationData {
    /// Tag ids to set (min 1 item).
    pub fn tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Free-text description (max length 5000).
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// Integration settings.
    pub fn settings(mut self, v: VcsSettingsBody) -> Self {
        self.settings = Some(v);
        self
    }
    /// Integration title (max length 200).
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.title = Some(v.into());
        self
    }
}

/// Request body (`EditIntegrationsTags`) for editing integration tags.
#[derive(Debug, Default, Serialize)]
pub struct EditIntegrationsTagsBody {
    /// Data. Required top-level wrapper.
    pub data: EditIntegrationsTagsData,
}

/// Inner `data` object (`EditIntegrationsTagsData`) for
/// [`EditIntegrationsTagsBody`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditIntegrationsTagsData {
    /// Tag action. Required.
    /// Allowed values: `ADD`, `REMOVE`, `REPLACE_ALL`, `CLEAR_ALL`.
    pub action: String,
    /// Integration ids to modify (min 1 item). Required.
    pub integration_ids: Vec<String>,
    /// Tag ids to apply. Required.
    pub tag_ids: Vec<String>,
}

impl EditIntegrationsTagsBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: EditIntegrationsTagsData) -> Self {
        Self { data }
    }
}

impl EditIntegrationsTagsData {
    /// Tag action (`ADD` | `REMOVE` | `REPLACE_ALL` | `CLEAR_ALL`). Required.
    pub fn action(mut self, v: impl Into<String>) -> Self {
        self.action = v.into();
        self
    }
    /// Integration ids to modify (min 1 item). Required.
    pub fn integration_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.integration_ids = v.into_iter().map(Into::into).collect();
        self
    }
    /// Tag ids to apply. Required.
    pub fn tag_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tag_ids = v.into_iter().map(Into::into).collect();
        self
    }
}

/// Request body (`VCSOnboarding`) for onboarding a new integration.
#[derive(Debug, Default, Serialize)]
pub struct VcsOnboardingBody {
    /// Data. Required top-level wrapper.
    pub data: VcsOnboardingData,
}

/// Inner `data` object (`VCSOnboardingData`) for [`VcsOnboardingBody`].
///
/// Required fields: `provider` and `tags`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VcsOnboardingData {
    /// VCS provider. Required.
    /// Allowed values: `GITHUB`, `GITLAB`, `BITBUCKET`, `GITHUB_ENTERPRISE`,
    /// `GITLAB_ENTERPRISE`, `AZURE_REPOS`.
    pub provider: String,
    /// Tag ids (min 1 item). Required.
    pub tags: Vec<String>,
    /// Azure organization URL. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_organization_url: Option<String>,
    /// Azure personal access token. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_personal_access_token: Option<String>,
    /// Free-text description (max length 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// GitHub app id (max length 200). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_app_id: Option<String>,
    /// GitHub app private key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_app_private_key: Option<String>,
    /// GitHub installation id (max length 200). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_installation_id: Option<String>,
    /// GitLab personal access token (max length 200). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitlab_personal_access_token: Option<String>,
    /// Server host (max length 1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_host: Option<String>,
    /// Server port (0..=65535). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_port: Option<i64>,
    /// Integration settings. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<VcsSettingsBody>,
    /// Integration title (max length 200). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl VcsOnboardingBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: VcsOnboardingData) -> Self {
        Self { data }
    }
}

impl VcsOnboardingData {
    /// VCS provider (`GITHUB` | `GITLAB` | `BITBUCKET` | `GITHUB_ENTERPRISE`
    /// | `GITLAB_ENTERPRISE` | `AZURE_REPOS`). Required.
    pub fn provider(mut self, v: impl Into<String>) -> Self {
        self.provider = v.into();
        self
    }
    /// Tag ids (min 1 item). Required.
    pub fn tags<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags = v.into_iter().map(Into::into).collect();
        self
    }
    /// Azure organization URL.
    pub fn azure_organization_url(mut self, v: impl Into<String>) -> Self {
        self.azure_organization_url = Some(v.into());
        self
    }
    /// Azure personal access token.
    pub fn azure_personal_access_token(mut self, v: impl Into<String>) -> Self {
        self.azure_personal_access_token = Some(v.into());
        self
    }
    /// Free-text description (max length 5000).
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// GitHub app id (max length 200).
    pub fn github_app_id(mut self, v: impl Into<String>) -> Self {
        self.github_app_id = Some(v.into());
        self
    }
    /// GitHub app private key.
    pub fn github_app_private_key(mut self, v: impl Into<String>) -> Self {
        self.github_app_private_key = Some(v.into());
        self
    }
    /// GitHub installation id (max length 200).
    pub fn github_installation_id(mut self, v: impl Into<String>) -> Self {
        self.github_installation_id = Some(v.into());
        self
    }
    /// GitLab personal access token (max length 200).
    pub fn gitlab_personal_access_token(mut self, v: impl Into<String>) -> Self {
        self.gitlab_personal_access_token = Some(v.into());
        self
    }
    /// Server host (max length 1000).
    pub fn server_host(mut self, v: impl Into<String>) -> Self {
        self.server_host = Some(v.into());
        self
    }
    /// Server port (0..=65535).
    pub fn server_port(mut self, v: i64) -> Self {
        self.server_port = Some(v);
        self
    }
    /// Integration settings.
    pub fn settings(mut self, v: VcsSettingsBody) -> Self {
        self.settings = Some(v);
        self
    }
    /// Integration title (max length 200).
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.title = Some(v.into());
        self
    }
}

/// Request body (`AddScannerPolicy`) for creating a scanner policy.
#[derive(Debug, Default, Serialize)]
pub struct AddScannerPolicyBody {
    /// Data. Required top-level wrapper.
    pub data: AddScannerPolicyData,
}

/// Inner `data` object (`ScannerPolicyData`) for [`AddScannerPolicyBody`].
///
/// Required fields: `priority` and `scope`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddScannerPolicyData {
    /// Policy priority. Required.
    pub priority: i64,
    /// Policy scope. Required.
    pub scope: ScopeBody,
    /// Branch the policy applies to (max length 255). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Free-text description (max length 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Policy name (max length 255). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AddScannerPolicyBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: AddScannerPolicyData) -> Self {
        Self { data }
    }
}

impl AddScannerPolicyData {
    /// Policy priority. Required.
    pub fn priority(mut self, v: i64) -> Self {
        self.priority = v;
        self
    }
    /// Policy scope. Required.
    pub fn scope(mut self, v: ScopeBody) -> Self {
        self.scope = v;
        self
    }
    /// Branch the policy applies to (max length 255).
    pub fn branch(mut self, v: impl Into<String>) -> Self {
        self.branch = Some(v.into());
        self
    }
    /// Free-text description (max length 5000).
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// Policy name (max length 255).
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
}

/// Request body (`UpdateVCSScannerPolicy`) for updating a scanner policy.
#[derive(Debug, Default, Serialize)]
pub struct UpdateVcsScannerPolicyBody {
    /// Data. Required top-level wrapper.
    pub data: UpdateVcsScannerPolicyData,
}

/// Inner `data` object (`UpdateVCSScannerPolicyData`) for
/// [`UpdateVcsScannerPolicyBody`].
///
/// Required fields: `name`, `priority` and `tagIds`. The configuration blocks
/// (`iacConfig`, `secretConfig`, `vulnerabilityConfig`) are deeply nested and
/// are typed as [`serde_json::Value`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateVcsScannerPolicyData {
    /// Policy name (max length 255). Required.
    pub name: String,
    /// Policy priority. Required.
    pub priority: i64,
    /// Tag ids to set. Required.
    pub tag_ids: Vec<String>,
    /// Branch the policy applies to (max length 255). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    /// Free-text description (max length 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// IaC scanning configuration (freeform object). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iac_config: Option<serde_json::Value>,
    /// Secret-scanning configuration (freeform object). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_config: Option<serde_json::Value>,
    /// Vulnerability-scanning configuration (freeform object). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_config: Option<serde_json::Value>,
}

impl UpdateVcsScannerPolicyBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: UpdateVcsScannerPolicyData) -> Self {
        Self { data }
    }
}

impl UpdateVcsScannerPolicyData {
    /// Policy name (max length 255). Required.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = v.into();
        self
    }
    /// Policy priority. Required.
    pub fn priority(mut self, v: i64) -> Self {
        self.priority = v;
        self
    }
    /// Tag ids to set. Required.
    pub fn tag_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tag_ids = v.into_iter().map(Into::into).collect();
        self
    }
    /// Branch the policy applies to (max length 255).
    pub fn branch(mut self, v: impl Into<String>) -> Self {
        self.branch = Some(v.into());
        self
    }
    /// Free-text description (max length 5000).
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// IaC scanning configuration (freeform object).
    pub fn iac_config(mut self, v: serde_json::Value) -> Self {
        self.iac_config = Some(v);
        self
    }
    /// Secret-scanning configuration (freeform object).
    pub fn secret_config(mut self, v: serde_json::Value) -> Self {
        self.secret_config = Some(v);
        self
    }
    /// Vulnerability-scanning configuration (freeform object).
    pub fn vulnerability_config(mut self, v: serde_json::Value) -> Self {
        self.vulnerability_config = Some(v);
        self
    }
}

/// Request body (`TunnelUser`) for registering a tunnel user.
#[derive(Debug, Default, Serialize)]
pub struct TunnelUserBody {
    /// Data. Required top-level wrapper.
    pub data: TunnelUserData,
}

/// Inner `data` object (`TunnelData`) for [`TunnelUserBody`].
///
/// Required fields: `provider` and `serverPort`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TunnelUserData {
    /// VCS provider. Required.
    /// Allowed values: `GITHUB_ENTERPRISE`, `GITLAB_ENTERPRISE`.
    pub provider: String,
    /// Server port (0..=65535). Required.
    pub server_port: i64,
    /// GitLab personal access token (max length 200). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gitlab_personal_access_token: Option<String>,
    /// Server host (max length 1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_host: Option<String>,
}

impl TunnelUserBody {
    /// Construct from an already-built inner `data` object.
    pub fn new(data: TunnelUserData) -> Self {
        Self { data }
    }
}

impl TunnelUserData {
    /// VCS provider (`GITHUB_ENTERPRISE` | `GITLAB_ENTERPRISE`). Required.
    pub fn provider(mut self, v: impl Into<String>) -> Self {
        self.provider = v.into();
        self
    }
    /// Server port (0..=65535). Required.
    pub fn server_port(mut self, v: i64) -> Self {
        self.server_port = v;
        self
    }
    /// GitLab personal access token (max length 200).
    pub fn gitlab_personal_access_token(mut self, v: impl Into<String>) -> Self {
        self.gitlab_personal_access_token = Some(v.into());
        self
    }
    /// Server host (max length 1000).
    pub fn server_host(mut self, v: impl Into<String>) -> Self {
        self.server_host = Some(v.into());
        self
    }
}

// ---------------------------------------------------------------------------
// Service methods
// ---------------------------------------------------------------------------

impl VcsIntegrationService<'_> {
    /// `GET /web/api/v2.1/cnapp/vcs/filters/count` — Fetch filter count.
    ///
    /// Fetch filter count.
    pub async fn get_filters_count(
        &self,
        query: &FiltersCountQuery,
    ) -> Result<Response<Vec<String>>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cnapp/vcs/filters/count", q)
            .await?)
    }

    /// `GET /web/api/v2.1/cnapp/vcs/integration/{integrationId}/offboarding` —
    /// Delete a VCS Integration.
    ///
    /// This API is used to off-board a VCS integration. The response is a flat
    /// object carrying the generated app URL (no `data` wrapper).
    ///
    /// `integration_id`: VCS integration id (path).
    pub async fn get_integration_offboarding(
        &self,
        integration_id: impl Into<String>,
        query: &IntegrationOffboardingQuery,
    ) -> Result<VcsOnboardingResult, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/integration/{}/offboarding",
            integration_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `PUT /web/api/v2.1/cnapp/vcs/integration/{integrationId}/repos/disable-scan`
    /// — Disable scanning for repositories in a VCS integration.
    ///
    /// Deactivate scanning for repositories associated with the given VCS
    /// integration.
    ///
    /// `integration_id`: VCS integration id (path).
    pub async fn disable_repos_scan(
        &self,
        integration_id: impl Into<String>,
        query: &DisableReposScanQuery,
        body: &EnableDisableReposBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/integration/{}/repos/disable-scan",
            integration_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<EnableDisableReposBody, Response<serde_json::Value>>(
                Method::PUT,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/cnapp/vcs/integration/{integrationId}/repos/edit-tags`
    /// — Edit tags for repositories in a VCS integration.
    ///
    /// Allows modification of tags associated with one or more repositories
    /// under a specified VCS integration. This operation supports adding,
    /// removing, or updating tags to help organize and categorize repositories
    /// effectively.
    ///
    /// `integration_id`: VCS integration id (path).
    pub async fn edit_repos_tags(
        &self,
        integration_id: impl Into<String>,
        query: &EditReposTagsQuery,
        body: &EditReposTagsBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/integration/{}/repos/edit-tags",
            integration_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<EditReposTagsBody, Response<serde_json::Value>>(
                Method::PUT,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/cnapp/vcs/integration/{integrationId}/repos/enable-scan`
    /// — Enable scanning for repositories in a VCS integration.
    ///
    /// Activates scanning for repositories associated with the given VCS
    /// integration.
    ///
    /// `integration_id`: VCS integration id (path).
    pub async fn enable_repos_scan(
        &self,
        integration_id: impl Into<String>,
        query: &EnableReposScanQuery,
        body: &EnableDisableReposBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/integration/{}/repos/enable-scan",
            integration_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<EnableDisableReposBody, Response<serde_json::Value>>(
                Method::PUT,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/cnapp/vcs/integration/{integrationId}/repos/get-tags`
    /// — Fetch repository tags.
    ///
    /// This endpoint retrieves tags associated with repositories under a VCS
    /// integration.
    ///
    /// `integration_id`: VCS integration id (path).
    pub async fn get_repos_tags(
        &self,
        integration_id: impl Into<String>,
        query: &GetReposTagsQuery,
        body: &FetchReposTagsBody,
    ) -> Result<Response<Vec<S1Tag>>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/integration/{}/repos/get-tags",
            integration_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<FetchReposTagsBody, Response<Vec<S1Tag>>>(
                Method::POST,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `DELETE /web/api/v2.1/cnapp/vcs/integration/{vcsIntegrationId}` —
    /// Delete a VCS integration.
    ///
    /// This endpoint permanently deletes a VCS integration.
    ///
    /// `vcs_integration_id`: VCS integration id (path).
    pub async fn delete_integration(
        &self,
        vcs_integration_id: impl Into<String>,
        query: &DeleteIntegrationQuery,
    ) -> Result<Response<DeleteIntegrationData>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/integration/{}",
            vcs_integration_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<DeleteIntegrationData>>(Method::DELETE, &path, q, None)
            .await?)
    }

    /// `PUT /web/api/v2.1/cnapp/vcs/integration/{vcsIntegrationId}` —
    /// Update a VCS integration.
    ///
    /// This endpoint allows users to update configuration details for a VCS
    /// integration.
    ///
    /// `vcs_integration_id`: VCS integration id (path).
    pub async fn update_integration(
        &self,
        vcs_integration_id: impl Into<String>,
        query: &UpdateIntegrationQuery,
        body: &UpdateVcsIntegrationBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/integration/{}",
            vcs_integration_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<UpdateVcsIntegrationBody, Response<serde_json::Value>>(
                Method::PUT,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/cnapp/vcs/integration/{vcsIntegrationId}/repos` —
    /// List VCS integration repositories.
    ///
    /// Fetches a list of repositories associated with the VCS integration. The
    /// repositories are wrapped in a `data` object; the top-level `pagination`
    /// block is not surfaced through the returned [`Repos`].
    ///
    /// `vcs_integration_id`: VCS integration id (path).
    pub async fn list_integration_repos(
        &self,
        vcs_integration_id: impl Into<String>,
        query: &ListIntegrationReposQuery,
    ) -> Result<Response<Repos>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/integration/{}/repos",
            vcs_integration_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `PUT /web/api/v2.1/cnapp/vcs/integration/{vcsIntegrationId}/repos/resync`
    /// — Resync VCS Integration Repositories.
    ///
    /// Initiates a process to resynchronize repositories associated with the
    /// specified VCS integration.
    ///
    /// `vcs_integration_id`: VCS integration id (path).
    pub async fn resync_integration_repos(
        &self,
        vcs_integration_id: impl Into<String>,
        query: &ResyncIntegrationReposQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/integration/{}/repos/resync",
            vcs_integration_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<serde_json::Value>>(Method::PUT, &path, q, None)
            .await?)
    }

    /// `GET /web/api/v2.1/cnapp/vcs/integrations` — List VCS integrations.
    ///
    /// Fetches a list of all configured VCS integrations.
    pub async fn list_integrations(
        &self,
        query: &ListIntegrationsQuery,
    ) -> Result<Paginated<VcsIntegration>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cnapp/vcs/integrations", q)
            .await?)
    }

    /// `PUT /web/api/v2.1/cnapp/vcs/integrations/edit-tags` —
    /// Edit tags for a VCS integration.
    ///
    /// Allows modification of tags in a VCS integration. This operation
    /// supports adding, removing, or updating tags to help organize and
    /// categorize integration repositories effectively.
    pub async fn edit_integrations_tags(
        &self,
        query: &EditIntegrationsTagsQuery,
        body: &EditIntegrationsTagsBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<EditIntegrationsTagsBody, Response<serde_json::Value>>(
                Method::PUT,
                "/web/api/v2.1/cnapp/vcs/integrations/edit-tags",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/cnapp/vcs/onboarding` — Onboard a new VCS
    /// integration.
    ///
    /// This endpoint allows users to onboard a new VCS integration, enabling
    /// automated scanning for secrets and IaC misconfigurations. The response
    /// is a flat object carrying the generated app URL (no `data` wrapper).
    pub async fn onboard_integration(
        &self,
        query: &OnboardingQuery,
        body: &VcsOnboardingBody,
    ) -> Result<VcsOnboardingResult, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<VcsOnboardingBody, VcsOnboardingResult>(
                Method::POST,
                "/web/api/v2.1/cnapp/vcs/onboarding",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/cnapp/vcs/scanner-policies` —
    /// List VCS and CICD scanner policies.
    ///
    /// This endpoint retrieves a list of all configured scanner policies used
    /// for VCS and CICD integrations.
    pub async fn list_scanner_policies(
        &self,
        query: &ListScannerPoliciesQuery,
    ) -> Result<Paginated<ScannerPolicy>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cnapp/vcs/scanner-policies", q)
            .await?)
    }

    /// `POST /web/api/v2.1/cnapp/vcs/scanner-policy` —
    /// Create a VCS and CICD scanner policy.
    ///
    /// Defines a scanning policy for a VCS integration, configuring parameters
    /// for detecting secrets, IaC misconfigurations, and vulnerabilities within
    /// repositories.
    pub async fn create_scanner_policy(
        &self,
        query: &CreateScannerPolicyQuery,
        body: &AddScannerPolicyBody,
    ) -> Result<Response<AddScannerPolicyResponseData>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<AddScannerPolicyBody, Response<AddScannerPolicyResponseData>>(
                Method::POST,
                "/web/api/v2.1/cnapp/vcs/scanner-policy",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/cnapp/vcs/scanner-policy/max-allowed-priority` —
    /// Get max allowed priority.
    ///
    /// This endpoint returns the maximum allowed value for the priority in
    /// scanner policies.
    pub async fn get_max_allowed_priority(
        &self,
        query: &MaxAllowedPriorityQuery,
    ) -> Result<Response<MaxPriorityData>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cnapp/vcs/scanner-policy/max-allowed-priority", q)
            .await?)
    }

    /// `DELETE /web/api/v2.1/cnapp/vcs/scanner-policy/{policyId}` —
    /// Delete a VCS and CICD scanner policy.
    ///
    /// This endpoint deletes a scanner policy.
    ///
    /// `policy_id`: Scanner policy id (path).
    pub async fn delete_scanner_policy(
        &self,
        policy_id: impl Into<String>,
        query: &DeleteScannerPolicyQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/scanner-policy/{}",
            policy_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<(), Response<serde_json::Value>>(Method::DELETE, &path, q, None)
            .await?)
    }

    /// `GET /web/api/v2.1/cnapp/vcs/scanner-policy/{policyId}` —
    /// Get a VCS and CICD scanner policy.
    ///
    /// Get a VCS and CICD scanner policy.
    ///
    /// `policy_id`: Scanner policy id (path).
    pub async fn get_scanner_policy(
        &self,
        policy_id: impl Into<String>,
        query: &GetScannerPolicyQuery,
    ) -> Result<Response<ScannerPolicy>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/scanner-policy/{}",
            policy_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// `PUT /web/api/v2.1/cnapp/vcs/scanner-policy/{policyId}` —
    /// Update a VCS and CICD scanner policy.
    ///
    /// This endpoint updates an existing scanner policy. A scanner policy
    /// defines how secrets, IaC misconfigurations, and vulnerabilities should
    /// be detected and handled within a VCS integration/CICD.
    ///
    /// `policy_id`: Scanner policy id (path).
    pub async fn update_scanner_policy(
        &self,
        policy_id: impl Into<String>,
        query: &UpdateScannerPolicyQuery,
        body: &UpdateVcsScannerPolicyBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/cnapp/vcs/scanner-policy/{}",
            policy_id.into()
        );
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<UpdateVcsScannerPolicyBody, Response<serde_json::Value>>(
                Method::PUT,
                &path,
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/cnapp/vcs/tunnel/user` — Register Tunnel User.
    ///
    /// This API is used to register a new tunnel user. It sets up the necessary
    /// tunnel configuration and credentials for secure access.
    pub async fn register_tunnel_user(
        &self,
        query: &TunnelUserQuery,
        body: &TunnelUserBody,
    ) -> Result<Response<TunnelUserResponseData>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<TunnelUserBody, Response<TunnelUserResponseData>>(
                Method::POST,
                "/web/api/v2.1/cnapp/vcs/tunnel/user",
                q,
                Some(body),
            )
            .await?)
    }
}
