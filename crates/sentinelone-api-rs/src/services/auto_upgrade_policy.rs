use sentinelone_http::Method;
use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::auto_upgrade_policy::*;

/// `Auto Upgrade Policy` tag.
///
/// Auto Upgrade Policy API related APIs.
///
/// Every endpoint in this tag returns its response object directly (the
/// SentinelOne `{ data, pagination, errors }` envelope is not used here), so
/// these methods return the typed model structs rather than
/// [`crate::pagination::Paginated`] / [`crate::pagination::Response`].
pub struct AutoUpgradePolicyService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/upgrade-policy/all-policies-count`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AllPoliciesCountQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl AllPoliciesCountQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Scope ID. Optional.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/upgrade-policy/available-packages`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailablePackagesQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// OS type. **Required.** One of `linux`, `macos`, `windows`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Partially match the name of the package, e.g. `22.1 GA`. Optional.
    #[serde(rename = "displayName__contains", skip_serializing_if = "Option::is_none")]
    pub display_name_contains: Option<String>,
}

impl AvailablePackagesQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Scope ID. Optional.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// OS type. **Required.** One of `linux`, `macos`, `windows`.
    pub fn os_type(mut self, v: impl Into<String>) -> Self {
        self.os_type = Some(v.into());
        self
    }
    /// Partially match the name of the package, e.g. `22.1 GA`. Optional.
    pub fn display_name_contains(mut self, v: impl Into<String>) -> Self {
        self.display_name_contains = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/upgrade-policy/parent-policies`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentPoliciesQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// OS type. **Required.** One of `linux`, `macos`, `windows`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Limit number of returned items. Should be more than 1, e.g. `10`. **Required.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Skip first number of items, e.g. `0`. **Required.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The column to sort the results by, e.g. `priority`. **Required.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. **Required.** One of `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl ParentPoliciesQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Scope ID. Optional.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// OS type. **Required.** One of `linux`, `macos`, `windows`.
    pub fn os_type(mut self, v: impl Into<String>) -> Self {
        self.os_type = Some(v.into());
        self
    }
    /// Limit number of returned items. Should be more than 1, e.g. `10`. **Required.**
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Skip first number of items, e.g. `0`. **Required.**
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// The column to sort the results by, e.g. `priority`. **Required.**
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. **Required.** One of `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/upgrade-policy/policies`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PoliciesQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// OS type. **Required.** One of `linux`, `macos`, `windows`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Limit number of returned items. Should be more than 1, e.g. `10`. **Required.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Skip first number of items, e.g. `0`. **Required.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The column to sort the results by, e.g. `priority`. **Required.**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. **Required.** One of `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl PoliciesQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Scope ID. Optional.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// OS type. **Required.** One of `linux`, `macos`, `windows`.
    pub fn os_type(mut self, v: impl Into<String>) -> Self {
        self.os_type = Some(v.into());
        self
    }
    /// Limit number of returned items. Should be more than 1, e.g. `10`. **Required.**
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Skip first number of items, e.g. `0`. **Required.**
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// The column to sort the results by, e.g. `priority`. **Required.**
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. **Required.** One of `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
}

/// Query params for `POST /web/api/v2.1/upgrade-policy/policies` (Deactivate Policies).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeactivatePoliciesQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// OS type. **Required.** One of `linux`, `macos`, `windows`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
}

impl DeactivatePoliciesQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Scope ID. Optional.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// OS type. **Required.** One of `linux`, `macos`, `windows`.
    pub fn os_type(mut self, v: impl Into<String>) -> Self {
        self.os_type = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/upgrade-policy/policies-count`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PoliciesCountQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Scope ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
}

impl PoliciesCountQuery {
    /// Scope level. **Required.** One of `account`, `group`, `site`, `tenant`.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Scope ID. Optional.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
}

/// Body for `POST /web/api/v2.1/upgrade-policy/has-policy`
/// (`v2_1.models.HasPoliciesRequest`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HasPolicyBody {
    /// List of Group IDs to filter by, e.g.
    /// `225494730938493804,225494730938493915`. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    /// OS type. One of `linux`, `macos`, `windows`. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
}

impl HasPolicyBody {
    /// List of Group IDs to filter by. Optional/nullable.
    pub fn groups<I, S>(mut self, groups: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.groups = Some(groups.into_iter().map(Into::into).collect());
        self
    }
    /// OS type. One of `linux`, `macos`, `windows`. Optional/nullable.
    pub fn os_type(mut self, v: impl Into<String>) -> Self {
        self.os_type = Some(v.into());
        self
    }
}

/// The package to send to an Agent during the upgrade (`v2_1.models.Package`).
///
/// Used as a nested field of [`CreatePolicyBody`].
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageBody {
    /// Package build identifier. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    /// File ID of the package. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// Major version component. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<String>,
    /// Minor version component. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<String>,
}

impl PackageBody {
    /// Package build identifier. Optional/nullable.
    pub fn build(mut self, v: impl Into<String>) -> Self {
        self.build = Some(v.into());
        self
    }
    /// File ID of the package. Optional/nullable.
    pub fn file_id(mut self, v: impl Into<String>) -> Self {
        self.file_id = Some(v.into());
        self
    }
    /// Major version component. Optional/nullable.
    pub fn major(mut self, v: impl Into<String>) -> Self {
        self.major = Some(v.into());
        self
    }
    /// Minor version component. Optional/nullable.
    pub fn minor(mut self, v: impl Into<String>) -> Self {
        self.minor = Some(v.into());
        self
    }
}

/// Body for `POST /web/api/v2.1/upgrade-policy/policy` (Create Policy,
/// `v2_1.models.Policy`) and `PUT /web/api/v2.1/upgrade-policy/policy/{policyid}`
/// (Update Policy, `v2_1.models.CreatePolicyRequest`).
///
/// The create and update endpoints share the same fields.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePolicyBody {
    /// Affected endpoints. `true` if the policy is applied to all endpoints.
    /// If `false`, `tags` must be provided. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_endpoints: Option<bool>,
    /// Policy description. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// `true` if policy is active, `false` if disabled. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// If a maintenance window is selected, schedule an upgrade for it.
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_scheduled: Option<bool>,
    /// Maximum number of upgrade retries for an endpoint on retriable failure.
    /// If provided, should be a positive integer. Defaults to 5.
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i64>,
    /// Policy name. Used for creating tasks. Should be unique. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// OS type. One of `linux`, `macos`, `windows`. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// The package to be sent to an Agent during the upgrade. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<PackageBody>,
    /// Scope ID. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// Scope level. One of `account`, `group`, `site`, `tenant`. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
    /// Tags for policy application. If provided, `all_endpoints` should be
    /// `false`. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl CreatePolicyBody {
    /// Affected endpoints. `true` if applied to all endpoints. Optional/nullable.
    pub fn all_endpoints(mut self, v: bool) -> Self {
        self.all_endpoints = Some(v);
        self
    }
    /// Policy description. Optional/nullable.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }
    /// `true` if policy is active, `false` if disabled. Optional/nullable.
    pub fn is_active(mut self, v: bool) -> Self {
        self.is_active = Some(v);
        self
    }
    /// Schedule an upgrade for a maintenance window. Optional/nullable.
    pub fn is_scheduled(mut self, v: bool) -> Self {
        self.is_scheduled = Some(v);
        self
    }
    /// Maximum number of upgrade retries. Optional/nullable.
    pub fn max_retries(mut self, v: i64) -> Self {
        self.max_retries = Some(v);
        self
    }
    /// Policy name. Optional/nullable.
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    /// OS type. One of `linux`, `macos`, `windows`. Optional/nullable.
    pub fn os_type(mut self, v: impl Into<String>) -> Self {
        self.os_type = Some(v.into());
        self
    }
    /// The package to be sent to an Agent during the upgrade. Optional/nullable.
    pub fn package(mut self, v: PackageBody) -> Self {
        self.package = Some(v);
        self
    }
    /// Scope ID. Optional/nullable.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// Scope level. One of `account`, `group`, `site`, `tenant`. Optional/nullable.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
    /// Tags for policy application. Optional/nullable.
    pub fn tags<I, S>(mut self, tags: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags = Some(tags.into_iter().map(Into::into).collect());
        self
    }
}

/// Body for `POST /web/api/v2.1/upgrade-policy/policy/{policyid}` (Policy Action,
/// `v2_1.models.EndpointActionRequest`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PolicyActionBody {
    /// Policy action. One of `delete`, `activate`, `deactivate`.
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

impl PolicyActionBody {
    /// Policy action. One of `delete`, `activate`, `deactivate`. Optional/nullable.
    pub fn action(mut self, v: impl Into<String>) -> Self {
        self.action = Some(v.into());
        self
    }
}

/// A policy ID and its new order (`v2_1.models.OrderedPolicy`).
///
/// Used as a nested element of [`ReorderBody`].
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedPolicy {
    /// Policy ID. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// New order/position for the policy. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

impl OrderedPolicy {
    /// Policy ID. Optional/nullable.
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.id = Some(v.into());
        self
    }
    /// New order/position for the policy. Optional/nullable.
    pub fn order(mut self, v: i64) -> Self {
        self.order = Some(v);
        self
    }
}

/// Body for `PUT /web/api/v2.1/upgrade-policy/reorder` (Reorder Policies,
/// `v2_1.models.ReorderPolicyRequest`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReorderBody {
    /// List of policy IDs and their new order, e.g.
    /// `[{"id":"...","order":0},{"id":"...","order":1}]`. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<OrderedPolicy>>,
}

impl ReorderBody {
    /// List of policy IDs and their new order. Optional/nullable.
    pub fn policies<I>(mut self, policies: I) -> Self
    where
        I: IntoIterator<Item = OrderedPolicy>,
    {
        self.policies = Some(policies.into_iter().collect());
        self
    }
}

/// Body for `PUT /web/api/v2.1/upgrade-policy/set-inheriting`
/// (Set Scope Inheriting, `v2_1.models.ScopeInheritanceRequest`).
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInheritingBody {
    /// `true` if policies are inherited from higher scopes, `false` otherwise.
    /// Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inheriting: Option<bool>,
    /// Scope ID. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_id: Option<String>,
    /// Scope level. One of `account`, `group`, `site`, `tenant`. Optional/nullable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_level: Option<String>,
}

impl SetInheritingBody {
    /// `true` if policies are inherited from higher scopes. Optional/nullable.
    pub fn is_inheriting(mut self, v: bool) -> Self {
        self.is_inheriting = Some(v);
        self
    }
    /// Scope ID. Optional/nullable.
    pub fn scope_id(mut self, v: impl Into<String>) -> Self {
        self.scope_id = Some(v.into());
        self
    }
    /// Scope level. One of `account`, `group`, `site`, `tenant`. Optional/nullable.
    pub fn scope_level(mut self, v: impl Into<String>) -> Self {
        self.scope_level = Some(v.into());
        self
    }
}

impl AutoUpgradePolicyService<'_> {
    /// `GET /web/api/v2.1/upgrade-policy/all-policies-count` — All Policies OS Count.
    ///
    /// Get the number of all policies for each OS from the given scope and the
    /// inherited scopes.
    pub async fn all_policies_count(
        &self,
        query: &AllPoliciesCountQuery,
    ) -> Result<OsCountResult, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/upgrade-policy/all-policies-count", q)
            .await?)
    }

    /// `GET /web/api/v2.1/upgrade-policy/available-packages` — Get Available Packages.
    ///
    /// Get Available Packages.
    pub async fn available_packages(
        &self,
        query: &AvailablePackagesQuery,
    ) -> Result<GetAvailablePackagesResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/upgrade-policy/available-packages", q)
            .await?)
    }

    /// `POST /web/api/v2.1/upgrade-policy/has-policy` — Has Policy.
    ///
    /// Has policy.
    pub async fn has_policy(
        &self,
        body: &HasPolicyBody,
    ) -> Result<HasPoliciesResponse, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/upgrade-policy/has-policy", body)
            .await?)
    }

    /// `GET /web/api/v2.1/upgrade-policy/parent-policies` — Get Parent Policies.
    ///
    /// Get paginated and ordered parent policies by a given scope.
    pub async fn parent_policies(
        &self,
        query: &ParentPoliciesQuery,
    ) -> Result<GetPoliciesResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/upgrade-policy/parent-policies", q)
            .await?)
    }

    /// `GET /web/api/v2.1/upgrade-policy/policies` — Get Policies.
    ///
    /// Get paginated and ordered policies by a given scope.
    pub async fn policies(
        &self,
        query: &PoliciesQuery,
    ) -> Result<GetPoliciesResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/upgrade-policy/policies", q)
            .await?)
    }

    /// `POST /web/api/v2.1/upgrade-policy/policies` — Deactivate Policies.
    ///
    /// Deactivate all policies.
    ///
    /// This endpoint takes only query params (no request body); the spec
    /// returns the `EmptyResponse` acknowledgement object.
    pub async fn deactivate_policies(
        &self,
        query: &DeactivatePoliciesQuery,
    ) -> Result<EmptyResponse, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<(), EmptyResponse>(
                Method::POST,
                "/web/api/v2.1/upgrade-policy/policies",
                q,
                None,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/upgrade-policy/policies-count` — Policies OS Count.
    ///
    /// Get the number of policies for each OS, for a given scope level and id.
    pub async fn policies_count(
        &self,
        query: &PoliciesCountQuery,
    ) -> Result<OsCountResult, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/upgrade-policy/policies-count", q)
            .await?)
    }

    /// `POST /web/api/v2.1/upgrade-policy/policy` — Create Policy.
    ///
    /// Add policy.
    pub async fn create_policy(
        &self,
        body: &CreatePolicyBody,
    ) -> Result<EmptyResponse, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/upgrade-policy/policy", body)
            .await?)
    }

    /// `POST /web/api/v2.1/upgrade-policy/policy/{policyid}` — Policy Action.
    ///
    /// Perform action on a certain policy.
    ///
    /// `policy_id` — Policy id. **Required.**
    pub async fn policy_action(
        &self,
        policy_id: impl Into<String>,
        body: &PolicyActionBody,
    ) -> Result<EmptyResponse, Error> {
        let path = format!(
            "/web/api/v2.1/upgrade-policy/policy/{}",
            policy_id.into()
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `PUT /web/api/v2.1/upgrade-policy/policy/{policyid}` — Update Policy.
    ///
    /// Update existing policy.
    ///
    /// `policy_id` — Policy id. **Required.**
    pub async fn update_policy(
        &self,
        policy_id: impl Into<String>,
        body: &CreatePolicyBody,
    ) -> Result<EmptyResponse, Error> {
        let path = format!(
            "/web/api/v2.1/upgrade-policy/policy/{}",
            policy_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<CreatePolicyBody, EmptyResponse>(
                Method::PUT,
                &path,
                None,
                Some(body),
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/upgrade-policy/policy/{policyid}/reset-retry-counter`
    /// — Reset Policy Retry Counter.
    ///
    /// Reset the number of times an Agent upgrade will be retried if the
    /// original upgrade attempt fails.
    ///
    /// `policy_id` — Policy ID. **Required.**
    pub async fn reset_retry_counter(
        &self,
        policy_id: impl Into<String>,
    ) -> Result<EmptyResponse, Error> {
        let path = format!(
            "/web/api/v2.1/upgrade-policy/policy/{}/reset-retry-counter",
            policy_id.into()
        );
        Ok(self
            .client
            .http()
            .request_json::<(), EmptyResponse>(Method::PUT, &path, None, None)
            .await?)
    }

    /// `PUT /web/api/v2.1/upgrade-policy/reorder` — Reorder Policies.
    ///
    /// Reorder policies.
    pub async fn reorder(&self, body: &ReorderBody) -> Result<EmptyResponse, Error> {
        Ok(self
            .client
            .http()
            .request_json::<ReorderBody, EmptyResponse>(
                Method::PUT,
                "/web/api/v2.1/upgrade-policy/reorder",
                None,
                Some(body),
            )
            .await?)
    }

    /// `PUT /web/api/v2.1/upgrade-policy/set-inheriting` — Set Scope Inheriting.
    ///
    /// Set Scope Inheriting.
    pub async fn set_inheriting(
        &self,
        body: &SetInheritingBody,
    ) -> Result<EmptyResponse, Error> {
        Ok(self
            .client
            .http()
            .request_json::<SetInheritingBody, EmptyResponse>(
                Method::PUT,
                "/web/api/v2.1/upgrade-policy/set-inheriting",
                None,
                Some(body),
            )
            .await?)
    }
}
