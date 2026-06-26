use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::alerts::*;
use crate::pagination::{Paginated, Response};

/// `alerts` tag — Cloud Detection / STAR alerts.
///
/// Provides listing of alerts for a scope and mutation of an alert's analyst
/// verdict and incident status.
pub struct AlertsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/cloud-detection/alerts`.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects (builders take an iterator and join by `,`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAlertsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional. Example: `150`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional. Example: `10`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional. Example: `YWdlbnRfaWQ6NTgwMjkzODE=`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items is returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, the total number of items is not calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Optional. Allowed values: `id`,
    /// `machineType`, `osName`, `incidentStatus`, `analystVerdict`, `severity`,
    /// `agentDetectionInfoMachineType`, `agentDetectionInfoName`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// A list of Alert IDs (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Filter results by scope (comma-joined). Optional. Allowed values per
    /// item: `group`, `global`, `site`, `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
    /// Created at lesser than. Optional. Date/time string.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at_lt: Option<String>,
    /// Created at greater than. Optional. Date/time string.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at_gt: Option<String>,
    /// Created at lesser or equal than. Optional. Date/time string.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at_lte: Option<String>,
    /// Created at greater or equal than. Optional. Date/time string.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at_gte: Option<String>,
    /// Reported at lesser than. Optional. Date/time string.
    #[serde(rename = "reportedAt__lt", skip_serializing_if = "Option::is_none")]
    pub reported_at_lt: Option<String>,
    /// Reported at greater than. Optional. Date/time string.
    #[serde(rename = "reportedAt__gt", skip_serializing_if = "Option::is_none")]
    pub reported_at_gt: Option<String>,
    /// Reported at lesser or equal than. Optional. Date/time string.
    #[serde(rename = "reportedAt__lte", skip_serializing_if = "Option::is_none")]
    pub reported_at_lte: Option<String>,
    /// Reported at greater or equal than. Optional. Date/time string.
    #[serde(rename = "reportedAt__gte", skip_serializing_if = "Option::is_none")]
    pub reported_at_gte: Option<String>,
    /// Filter threats by an incident status (comma-joined). Optional. Allowed
    /// values per item: `UNRESOLVED`, `IN_PROGRESS`, `RESOLVED`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_status: Option<String>,
    /// Severity (comma-joined). Optional. Allowed values per item: `Info`,
    /// `Low`, `Medium`, `High`, `Critical`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// Free-text filter by agent OS revision (comma-joined). Optional.
    #[serde(
        rename = "origAgentOsRevision__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_agent_os_revision_contains: Option<String>,
    /// Free-text filter by agent OS version (comma-joined). Optional.
    #[serde(
        rename = "origAgentVersion__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_agent_version_contains: Option<String>,
    /// Free-text filter by agent UUID (comma-joined). Optional.
    #[serde(
        rename = "origAgentUuid__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_agent_uuid_contains: Option<String>,
    /// Free-text filter by agent name (comma-joined). Optional.
    #[serde(
        rename = "origAgentName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub orig_agent_name_contains: Option<String>,
    /// Free-text filter by source process name (comma-joined). Optional.
    #[serde(
        rename = "sourceProcessName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_process_name_contains: Option<String>,
    /// Free-text filter by rule name (comma-joined). Optional.
    #[serde(rename = "ruleName__contains", skip_serializing_if = "Option::is_none")]
    pub rule_name_contains: Option<String>,
    /// Free-text filter by source storyline (comma-joined). Optional.
    #[serde(
        rename = "sourceProcessStoryline__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_process_storyline_contains: Option<String>,
    /// Free-text filter by source commandline (comma-joined). Optional.
    #[serde(
        rename = "sourceProcessCommandline__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_process_commandline_contains: Option<String>,
    /// Free-text filter by source SHA1 (comma-joined). Optional.
    #[serde(
        rename = "sourceProcessFileHashSha1__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_process_file_hash_sha1_contains: Option<String>,
    /// Free-text filter by source SHA256 (comma-joined). Optional.
    #[serde(
        rename = "sourceProcessFileHashSha256__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_process_file_hash_sha256_contains: Option<String>,
    /// Free-text filter by source MD5 (comma-joined). Optional.
    #[serde(
        rename = "sourceProcessFileHashMd5__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_process_file_hash_md5_contains: Option<String>,
    /// Free-text filter by source file path (comma-joined). Optional.
    #[serde(
        rename = "sourceProcessFilePath__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub source_process_file_path_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes cluster name (comma-joined,
    /// supports multiple values). Optional.
    #[serde(
        rename = "k8sCluster__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub k8s_cluster_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes node name (comma-joined,
    /// supports multiple values). Optional.
    #[serde(rename = "k8sNode__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes namespace name (comma-joined,
    /// supports multiple values). Optional.
    #[serde(
        rename = "k8sNamespaceName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub k8s_namespace_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes namespace labels
    /// (comma-joined, supports multiple values). Optional.
    #[serde(
        rename = "k8sNamespaceLabels__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub k8s_namespace_labels_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes controller name
    /// (comma-joined, supports multiple values). Optional.
    #[serde(
        rename = "k8sControllerName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub k8s_controller_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes controller labels
    /// (comma-joined, supports multiple values). Optional.
    #[serde(
        rename = "k8sControllerLabels__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub k8s_controller_labels_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes pod name (comma-joined,
    /// supports multiple values). Optional.
    #[serde(rename = "k8sPod__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_pod_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes pod labels (comma-joined,
    /// supports multiple values). Optional.
    #[serde(
        rename = "k8sPodLabels__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub k8s_pod_labels_contains: Option<String>,
    /// Free-text filter by the endpoint container name (comma-joined, supports
    /// multiple values). Optional.
    #[serde(
        rename = "containerName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_name_contains: Option<String>,
    /// Free-text filter by the endpoint container image name (comma-joined,
    /// supports multiple values). Optional.
    #[serde(
        rename = "containerImageName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_image_name_contains: Option<String>,
    /// Free-text filter by the endpoint container labels (comma-joined,
    /// supports multiple values). Optional.
    #[serde(
        rename = "containerLabels__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub container_labels_contains: Option<String>,
    /// Full text search for all fields. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Filter threats by an analyst verdict (comma-joined). Optional. Allowed
    /// values per item: `UNDEFINED`, `TRUE_POSITIVE`, `FALSE_POSITIVE`,
    /// `SUSPICIOUS`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyst_verdict: Option<String>,
    /// Agent machine type (comma-joined). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_type: Option<String>,
    /// Included OS types (comma-joined). Optional. Allowed values per item:
    /// `macos`, `windows_legacy`, `windows`, `linux`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// If true, all rules for the requested scope are returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_pagination: Option<bool>,
}

fn join_csv<I, S>(values: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    values
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

impl ListAlertsQuery {
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// If true, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, the total number of items is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The column to sort the results by. Allowed values: `id`, `machineType`,
    /// `osName`, `incidentStatus`, `analystVerdict`, `severity`,
    /// `agentDetectionInfoMachineType`, `agentDetectionInfoName`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Indicates a tenant scope request.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// A list of Alert IDs.
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(v));
        self
    }
    /// Filter results by scope. Allowed values per item: `group`, `global`,
    /// `site`, `account`.
    pub fn scopes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scopes = Some(join_csv(v));
        self
    }
    /// Created at lesser than.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Created at greater than.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Created at lesser or equal than.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Created at greater or equal than.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Reported at lesser than.
    pub fn reported_at_lt(mut self, v: impl Into<String>) -> Self {
        self.reported_at_lt = Some(v.into());
        self
    }
    /// Reported at greater than.
    pub fn reported_at_gt(mut self, v: impl Into<String>) -> Self {
        self.reported_at_gt = Some(v.into());
        self
    }
    /// Reported at lesser or equal than.
    pub fn reported_at_lte(mut self, v: impl Into<String>) -> Self {
        self.reported_at_lte = Some(v.into());
        self
    }
    /// Reported at greater or equal than.
    pub fn reported_at_gte(mut self, v: impl Into<String>) -> Self {
        self.reported_at_gte = Some(v.into());
        self
    }
    /// Filter threats by an incident status. Allowed values per item:
    /// `UNRESOLVED`, `IN_PROGRESS`, `RESOLVED`.
    pub fn incident_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.incident_status = Some(join_csv(v));
        self
    }
    /// Severity. Allowed values per item: `Info`, `Low`, `Medium`, `High`,
    /// `Critical`.
    pub fn severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.severity = Some(join_csv(v));
        self
    }
    /// Free-text filter by agent OS revision.
    pub fn orig_agent_os_revision_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.orig_agent_os_revision_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by agent OS version.
    pub fn orig_agent_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.orig_agent_version_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by agent UUID.
    pub fn orig_agent_uuid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.orig_agent_uuid_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by agent name.
    pub fn orig_agent_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.orig_agent_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by source process name.
    pub fn source_process_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source_process_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by rule name.
    pub fn rule_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.rule_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by source storyline.
    pub fn source_process_storyline_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source_process_storyline_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by source commandline.
    pub fn source_process_commandline_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source_process_commandline_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by source SHA1.
    pub fn source_process_file_hash_sha1_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source_process_file_hash_sha1_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by source SHA256.
    pub fn source_process_file_hash_sha256_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source_process_file_hash_sha256_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by source MD5.
    pub fn source_process_file_hash_md5_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source_process_file_hash_md5_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by source file path.
    pub fn source_process_file_path_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.source_process_file_path_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint Kubernetes cluster name.
    pub fn k8s_cluster_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint Kubernetes node name.
    pub fn k8s_node_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint Kubernetes namespace name.
    pub fn k8s_namespace_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint Kubernetes namespace labels.
    pub fn k8s_namespace_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace_labels_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint Kubernetes controller name.
    pub fn k8s_controller_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_controller_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint Kubernetes controller labels.
    pub fn k8s_controller_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_controller_labels_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint Kubernetes pod name.
    pub fn k8s_pod_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_pod_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint Kubernetes pod labels.
    pub fn k8s_pod_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_pod_labels_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint container name.
    pub fn container_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.container_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint container image name.
    pub fn container_image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.container_image_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the endpoint container labels.
    pub fn container_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.container_labels_contains = Some(join_csv(v));
        self
    }
    /// Full text search for all fields.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Filter threats by an analyst verdict. Allowed values per item:
    /// `UNDEFINED`, `TRUE_POSITIVE`, `FALSE_POSITIVE`, `SUSPICIOUS`.
    pub fn analyst_verdict<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.analyst_verdict = Some(join_csv(v));
        self
    }
    /// Agent machine type.
    pub fn machine_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_type = Some(join_csv(v));
        self
    }
    /// Included OS types. Allowed values per item: `macos`, `windows_legacy`,
    /// `windows`, `linux`.
    pub fn os_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_type = Some(join_csv(v));
        self
    }
    /// If true, all rules for the requested scope are returned.
    pub fn disable_pagination(mut self, v: bool) -> Self {
        self.disable_pagination = Some(v);
        self
    }
}

/// `data` payload for `POST /web/api/v2.1/cloud-detection/alerts/analyst-verdict`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAnalystVerdictData {
    /// Analyst verdict. Required. Allowed values: `undefined`, `UNDEFINED`,
    /// `true_positive`, `TRUE_POSITIVE`, `false_positive`, `FALSE_POSITIVE`,
    /// `suspicious`, `SUSPICIOUS`.
    pub analyst_verdict: String,
}

/// Request body for
/// `POST /web/api/v2.1/cloud-detection/alerts/analyst-verdict`.
///
/// Both `data` and `filter` are required by the spec. `filter` is a freeform
/// object (the alert filter shared with the list endpoint) and is therefore
/// modelled as [`serde_json::Value`]; pass an empty object if no scoping is
/// required.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateAnalystVerdictBody {
    /// Verdict data. Required.
    pub data: UpdateAnalystVerdictData,
    /// Alert filter (freeform object: `accountIds`, `siteIds`, `groupIds`,
    /// `ids`, `scopes`, date/severity/free-text filters, etc.). Required.
    pub filter: serde_json::Value,
}

/// `data` payload for `POST /web/api/v2.1/cloud-detection/alerts/incident`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIncidentData {
    /// Incident status. Required. Allowed values: `unresolved`, `UNRESOLVED`,
    /// `in_progress`, `IN_PROGRESS`, `resolved`, `RESOLVED`.
    pub incident_status: String,
}

/// Request body for `POST /web/api/v2.1/cloud-detection/alerts/incident`.
///
/// Both `data` and `filter` are required by the spec. `filter` is a freeform
/// object (the alert filter shared with the list endpoint) and is therefore
/// modelled as [`serde_json::Value`]; pass an empty object if no scoping is
/// required.
#[derive(Debug, Clone, Serialize)]
pub struct UpdateIncidentBody {
    /// Incident data. Required.
    pub data: UpdateIncidentData,
    /// Alert filter (freeform object: `accountIds`, `siteIds`, `groupIds`,
    /// `ids`, `scopes`, date/severity/free-text filters, etc.). Required.
    pub filter: serde_json::Value,
}

impl AlertsService<'_> {
    /// `GET /web/api/v2.1/cloud-detection/alerts` — Get alerts.
    ///
    /// Get a list of alerts for a given scope.
    pub async fn list(&self, query: &ListAlertsQuery) -> Result<Paginated<Alert>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/cloud-detection/alerts", q)
            .await?)
    }

    /// `POST /web/api/v2.1/cloud-detection/alerts/analyst-verdict` — Update
    /// Alert Analyst Verdict.
    ///
    /// Change the verdict of an alert. Returns the number of affected entities.
    pub async fn update_analyst_verdict(
        &self,
        body: &UpdateAnalystVerdictBody,
    ) -> Result<Response<AlertsAffected>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/cloud-detection/alerts/analyst-verdict", body)
            .await?)
    }

    /// `POST /web/api/v2.1/cloud-detection/alerts/incident` — Update Threat
    /// Incident.
    ///
    /// Update the incident details of an alert. Returns the number of affected
    /// entities.
    pub async fn update_incident(
        &self,
        body: &UpdateIncidentBody,
    ) -> Result<Response<AlertsAffected>, Error> {
        Ok(self
            .client
            .http()
            .post("/web/api/v2.1/cloud-detection/alerts/incident", body)
            .await?)
    }
}
