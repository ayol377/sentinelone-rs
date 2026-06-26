//! `Threats` tag — threat-related operations.
//!
//! Hand-written for strict 1:1 parity with the SentinelOne Management API spec
//! (`swagger_2_1.json`) and the endpoint documentation (`docs/threats.md`).
//!
//! Query-parameter structs expose every documented parameter as an
//! `Option<T>` builder; array parameters are joined with commas, as the API
//! expects. Request bodies follow the `{ data, filter }` envelope; their deeply
//! nested / freeform contents are kept as [`serde_json::Value`] for fidelity.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::threats::*;
use crate::pagination::{Paginated, Response};

/// `Threats` tag.
pub struct ThreatsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/threats` (Get Threats).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreatsQuery {
    /// Skip first number of items (0-1000). Use `cursor` to iterate beyond 1000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total item count is not calculated (faster).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Column to sort by. Allowed: id, createdAt, updatedAt, mitigationStatus, fileDisplayName, agentVersion, agentMachineType, siteId.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed: asc, desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Created at lesser than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "createdAt__lt")]
    pub created_at_lt: Option<String>,
    /// Created at greater than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "createdAt__gt")]
    pub created_at_gt: Option<String>,
    /// Created at lesser or equal than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "createdAt__lte")]
    pub created_at_lte: Option<String>,
    /// Created at greater or equal than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "createdAt__gte")]
    pub created_at_gte: Option<String>,
    /// Updated at lesser than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updatedAt__lt")]
    pub updated_at_lt: Option<String>,
    /// Updated at greater than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updatedAt__gt")]
    pub updated_at_gt: Option<String>,
    /// Updated at lesser or equal than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updatedAt__lte")]
    pub updated_at_lte: Option<String>,
    /// Updated at greater or equal than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updatedAt__gte")]
    pub updated_at_gte: Option<String>,
    /// List of sha1 hashes to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hashes: Option<String>,
    /// Display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Filter threats by a specific status. Allowed: not_mitigated, mitigated, marked_as_benign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_statuses: Option<String>,
    /// Filter threats not by a specific status. Allowed: not_mitigated, mitigated, marked_as_benign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_statuses_nin: Option<String>,
    /// List of Agent IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<String>,
    /// List of Agent context to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storylines: Option<String>,
    /// List of threat IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of collection IDs to search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<String>,
    /// Included engines. Allowed: reputation, sentinelone_cloud, cloud_detection, user_blacklist, pre_execution, pre_execution_suspicious, executables, data_files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<String>,
    /// Excluded engines. Allowed: reputation, sentinelone_cloud, cloud_detection, user_blacklist, pre_execution, pre_execution_suspicious, executables, data_files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines_nin: Option<String>,
    /// Included engines. Allowed: reputation, sentinelone_cloud, cloud_detection, user_blacklist, pre_execution, pre_execution_suspicious, executables, data_files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_engines: Option<String>,
    /// Excluded engines. Allowed: reputation, sentinelone_cloud, cloud_detection, user_blacklist, pre_execution, pre_execution_suspicious, executables, data_files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_engines_nin: Option<String>,
    /// List of threat classifications to search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications: Option<String>,
    /// List of threat classifications not to search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications_nin: Option<String>,
    /// Classification sources list. Allowed: Cloud, Behavioral, Static, Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_sources: Option<String>,
    /// Classification sources list to exclude. Allowed: Cloud, Behavioral, Static, Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_sources_nin: Option<String>,
    /// Include Agent versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_versions: Option<String>,
    /// Excluded Agent versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_versions_nin: Option<String>,
    /// Include Agent machine types. Allowed: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_machine_types: Option<String>,
    /// Excluded Agent machine types. Allowed: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_machine_types_nin: Option<String>,
    /// Included OS types. Allowed: linux, macos, windows_legacy, windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Excluded OS types. Allowed: linux, macos, windows_legacy, windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types_nin: Option<String>,
    /// Included OS Architectures. Allowed: 32 bit, 64 bit, ARM64.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_archs: Option<String>,
    /// Included OS names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_names: Option<String>,
    /// Excluded OS names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_names_nin: Option<String>,
    /// Include Agents currently connected to the Management Console.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_is_active: Option<bool>,
    /// Only include threats from specific initiating sources. Allowed: agent_policy, full_disk_scan, sentinelctl, dv_command, console_api, on_demand_scan, star_active, star_manual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<String>,
    /// Exclude threats with specific initiating sources. Allowed: agent_policy, full_disk_scan, sentinelctl, dv_command, console_api, on_demand_scan, star_active, star_manual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_by_nin: Option<String>,
    /// Filter threats by a specific confidence level. Allowed: malicious, suspicious, n/a.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_levels: Option<String>,
    /// Exclude threats with specific confidence level. Allowed: malicious, suspicious, n/a.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_levels_nin: Option<String>,
    /// Filter threats by a specific analyst verdict. Allowed: undefined, true_positive, false_positive, suspicious.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyst_verdicts: Option<String>,
    /// Exclude threats with specific analyst verdicts. Allowed: undefined, true_positive, false_positive, suspicious.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyst_verdicts_nin: Option<String>,
    /// Filter threats by a specific incident status. Allowed: unresolved, in_progress, resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_statuses: Option<String>,
    /// Exclude threats with specific incident statuses. Allowed: unresolved, in_progress, resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_statuses_nin: Option<String>,
    /// The threat contains at least one note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_exists: Option<bool>,
    /// At least one action failed on the threat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_actions: Option<bool>,
    /// A reboot is required on any endpoint for at least one action on the threat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_required: Option<bool>,
    /// At least one action is pending for the Agent for the threat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_actions: Option<bool>,
    /// The threat contains ticket number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ticket_exists: Option<bool>,
    /// External ticket ID for the threat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ticket_ids: Option<String>,
    /// If the threat was detected pre-execution or post-execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigated_preemptively: Option<bool>,
    /// Filter threats by assigned tags to the related agent (JSON object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_tags_data: Option<String>,
    /// Include only Threats whose Agent is assigned any tags if True, or none if False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_agent_tags: Option<bool>,
    /// Full text search across multiple threat fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Free-text filter by file content hash (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentHash__contains")]
    pub content_hash_contains: Option<String>,
    /// Free-text filter by threat details (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threatDetails__contains")]
    pub threat_details_contains: Option<String>,
    /// Free-text filter by file path (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filePath__contains")]
    pub file_path_contains: Option<String>,
    /// Free-text filter by computer name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "computerName__contains")]
    pub computer_name_contains: Option<String>,
    /// Free-text filter by Agent UUID (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "uuid__contains")]
    pub uuid_contains: Option<String>,
    /// Free-text filter by Agent version at detection time (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detectionAgentVersion__contains")]
    pub detection_agent_version_contains: Option<String>,
    /// Free-text filter by Agent version at current time (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "realtimeAgentVersion__contains")]
    pub realtime_agent_version_contains: Option<String>,
    /// Free-text filter by Agent domain at detection time (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detectionAgentDomain__contains")]
    pub detection_agent_domain_contains: Option<String>,
    /// Free-text filter by threat command line arguments (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "commandLineArguments__contains")]
    pub command_line_arguments_contains: Option<String>,
    /// Free-text filter by the username that initiated that threat (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "initiatedByUsername__contains")]
    pub initiated_by_username_contains: Option<String>,
    /// Free-text filter by threat storyline (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storyline__contains")]
    pub storyline_contains: Option<String>,
    /// Free-text filter by the originated process name of the threat (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "originatedProcess__contains")]
    pub originated_process_contains: Option<String>,
    /// Free-text filter by threat's publisher name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publisherName__contains")]
    pub publisher_name_contains: Option<String>,
    /// Free-text filter by threat's signer identity (certificate ID) (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "signerIdentity__contains")]
    pub signer_identity_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes cluster name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sClusterName__contains")]
    pub k8s_cluster_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes node name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sNodeName__contains")]
    pub k8s_node_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes node labels (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sNodeLabels__contains")]
    pub k8s_node_labels_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes namespace name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sNamespaceName__contains")]
    pub k8s_namespace_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes namespace labels (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sNamespaceLabels__contains")]
    pub k8s_namespace_labels_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes controller name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sControllerName__contains")]
    pub k8s_controller_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes controller labels (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sControllerLabels__contains")]
    pub k8s_controller_labels_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes pod name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sPodName__contains")]
    pub k8s_pod_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes pod labels (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sPodLabels__contains")]
    pub k8s_pod_labels_contains: Option<String>,
    /// Free-text filter by the endpoint container name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerName__contains")]
    pub container_name_contains: Option<String>,
    /// Free-text filter by the endpoint container image name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerImageName__contains")]
    pub container_image_name_contains: Option<String>,
    /// Free-text filter by the endpoint container labels (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerLabels__contains")]
    pub container_labels_contains: Option<String>,
    /// Free-text filter by the threat external ticket ID (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "externalTicketId__contains")]
    pub external_ticket_id_contains: Option<String>,
    /// Comma-separated list of fields to be shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// Used for backward-compatibility with API 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
    /// Agents from which cloud provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// Exclude Agents from these cloud provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_nin: Option<String>,
    /// Free-text filter by aws securityGroups (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "awsSecurityGroups__contains")]
    pub aws_security_groups_contains: Option<String>,
    /// Free-text filter by cloud account (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudAccount__contains")]
    pub cloud_account_contains: Option<String>,
    /// Free-text filter by cloud image (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudImage__contains")]
    pub cloud_image_contains: Option<String>,
    /// Free-text filter by cloud instance id (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudInstanceId__contains")]
    pub cloud_instance_id_contains: Option<String>,
    /// Free-text filter by cloud instance size (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudInstanceSize__contains")]
    pub cloud_instance_size_contains: Option<String>,
    /// Free-text filter by cloud location (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudLocation__contains")]
    pub cloud_location_contains: Option<String>,
    /// Free-text filter by cloud network (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudNetwork__contains")]
    pub cloud_network_contains: Option<String>,
    /// Free-text filter by aws role (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "awsRole__contains")]
    pub aws_role_contains: Option<String>,
    /// Free-text filter by aws subnet ids (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "awsSubnetIds__contains")]
    pub aws_subnet_ids_contains: Option<String>,
    /// Free-text filter by azure resource group (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "azureResourceGroup__contains")]
    pub azure_resource_group_contains: Option<String>,
    /// Free-text filter by gcp service account (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gcpServiceAccount__contains")]
    pub gcp_service_account_contains: Option<String>,
}

impl ThreatsQuery {
    /// Set `skip`.
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Set `limit`.
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Set `cursor`.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Set `countOnly`.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Set `skipCount`.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Set `sortBy`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Set `sortOrder`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Set `accountIds` (comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `siteIds` (comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `groupIds` (comma-joined).
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `tenant`.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Set `createdAt__lt`.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Set `createdAt__gt`.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Set `createdAt__lte`.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Set `createdAt__gte`.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Set `updatedAt__lt`.
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lt = Some(v.into());
        self
    }
    /// Set `updatedAt__gt`.
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gt = Some(v.into());
        self
    }
    /// Set `updatedAt__lte`.
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// Set `updatedAt__gte`.
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// Set `contentHashes` (comma-joined).
    pub fn content_hashes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.content_hashes = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `displayName`.
    pub fn display_name(mut self, v: impl Into<String>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    /// Set `mitigationStatuses` (comma-joined).
    pub fn mitigation_statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mitigation_statuses = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `mitigationStatusesNin` (comma-joined).
    pub fn mitigation_statuses_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mitigation_statuses_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentIds` (comma-joined).
    pub fn agent_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `storylines` (comma-joined).
    pub fn storylines<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storylines = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `ids` (comma-joined).
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `collectionIds` (comma-joined).
    pub fn collection_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `engines` (comma-joined).
    pub fn engines<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.engines = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `enginesNin` (comma-joined).
    pub fn engines_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.engines_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `detectionEngines` (comma-joined).
    pub fn detection_engines<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detection_engines = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `detectionEnginesNin` (comma-joined).
    pub fn detection_engines_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detection_engines_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `classifications` (comma-joined).
    pub fn classifications<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.classifications = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `classificationsNin` (comma-joined).
    pub fn classifications_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.classifications_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `classificationSources` (comma-joined).
    pub fn classification_sources<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.classification_sources = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `classificationSourcesNin` (comma-joined).
    pub fn classification_sources_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.classification_sources_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentVersions` (comma-joined).
    pub fn agent_versions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentVersionsNin` (comma-joined).
    pub fn agent_versions_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentMachineTypes` (comma-joined).
    pub fn agent_machine_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_machine_types = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentMachineTypesNin` (comma-joined).
    pub fn agent_machine_types_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_machine_types_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `osTypes` (comma-joined).
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `osTypesNin` (comma-joined).
    pub fn os_types_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `osArchs` (comma-joined).
    pub fn os_archs<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_archs = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `osNames` (comma-joined).
    pub fn os_names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_names = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `osNamesNin` (comma-joined).
    pub fn os_names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_names_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentIsActive`.
    pub fn agent_is_active(mut self, v: bool) -> Self {
        self.agent_is_active = Some(v);
        self
    }
    /// Set `initiatedBy` (comma-joined).
    pub fn initiated_by<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.initiated_by = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `initiatedByNin` (comma-joined).
    pub fn initiated_by_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.initiated_by_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `confidenceLevels` (comma-joined).
    pub fn confidence_levels<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.confidence_levels = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `confidenceLevelsNin` (comma-joined).
    pub fn confidence_levels_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.confidence_levels_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `analystVerdicts` (comma-joined).
    pub fn analyst_verdicts<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.analyst_verdicts = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `analystVerdictsNin` (comma-joined).
    pub fn analyst_verdicts_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.analyst_verdicts_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `incidentStatuses` (comma-joined).
    pub fn incident_statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.incident_statuses = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `incidentStatusesNin` (comma-joined).
    pub fn incident_statuses_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.incident_statuses_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `noteExists`.
    pub fn note_exists(mut self, v: bool) -> Self {
        self.note_exists = Some(v);
        self
    }
    /// Set `failedActions`.
    pub fn failed_actions(mut self, v: bool) -> Self {
        self.failed_actions = Some(v);
        self
    }
    /// Set `rebootRequired`.
    pub fn reboot_required(mut self, v: bool) -> Self {
        self.reboot_required = Some(v);
        self
    }
    /// Set `pendingActions`.
    pub fn pending_actions(mut self, v: bool) -> Self {
        self.pending_actions = Some(v);
        self
    }
    /// Set `externalTicketExists`.
    pub fn external_ticket_exists(mut self, v: bool) -> Self {
        self.external_ticket_exists = Some(v);
        self
    }
    /// Set `externalTicketIds` (comma-joined).
    pub fn external_ticket_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ticket_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `mitigatedPreemptively`.
    pub fn mitigated_preemptively(mut self, v: bool) -> Self {
        self.mitigated_preemptively = Some(v);
        self
    }
    /// Set `agentTagsData`.
    pub fn agent_tags_data(mut self, v: impl Into<String>) -> Self {
        self.agent_tags_data = Some(v.into());
        self
    }
    /// Set `hasAgentTags`.
    pub fn has_agent_tags(mut self, v: bool) -> Self {
        self.has_agent_tags = Some(v);
        self
    }
    /// Set `query`.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Set `contentHash__contains` (comma-joined).
    pub fn content_hash_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.content_hash_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `threatDetails__contains` (comma-joined).
    pub fn threat_details_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_details_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `filePath__contains` (comma-joined).
    pub fn file_path_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.file_path_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `computerName__contains` (comma-joined).
    pub fn computer_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `uuid__contains` (comma-joined).
    pub fn uuid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuid_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `detectionAgentVersion__contains` (comma-joined).
    pub fn detection_agent_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detection_agent_version_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `realtimeAgentVersion__contains` (comma-joined).
    pub fn realtime_agent_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.realtime_agent_version_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `detectionAgentDomain__contains` (comma-joined).
    pub fn detection_agent_domain_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detection_agent_domain_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `commandLineArguments__contains` (comma-joined).
    pub fn command_line_arguments_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.command_line_arguments_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `initiatedByUsername__contains` (comma-joined).
    pub fn initiated_by_username_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.initiated_by_username_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `storyline__contains` (comma-joined).
    pub fn storyline_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storyline_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `originatedProcess__contains` (comma-joined).
    pub fn originated_process_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.originated_process_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `publisherName__contains` (comma-joined).
    pub fn publisher_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.publisher_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `signerIdentity__contains` (comma-joined).
    pub fn signer_identity_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.signer_identity_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sClusterName__contains` (comma-joined).
    pub fn k8s_cluster_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sNodeName__contains` (comma-joined).
    pub fn k8s_node_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sNodeLabels__contains` (comma-joined).
    pub fn k8s_node_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_labels_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sNamespaceName__contains` (comma-joined).
    pub fn k8s_namespace_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sNamespaceLabels__contains` (comma-joined).
    pub fn k8s_namespace_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace_labels_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sControllerName__contains` (comma-joined).
    pub fn k8s_controller_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_controller_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sControllerLabels__contains` (comma-joined).
    pub fn k8s_controller_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_controller_labels_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sPodName__contains` (comma-joined).
    pub fn k8s_pod_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_pod_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sPodLabels__contains` (comma-joined).
    pub fn k8s_pod_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_pod_labels_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `containerName__contains` (comma-joined).
    pub fn container_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.container_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `containerImageName__contains` (comma-joined).
    pub fn container_image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.container_image_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `containerLabels__contains` (comma-joined).
    pub fn container_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.container_labels_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `externalTicketId__contains` (comma-joined).
    pub fn external_ticket_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ticket_id_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `countsFor`.
    pub fn counts_for(mut self, v: impl Into<String>) -> Self {
        self.counts_for = Some(v.into());
        self
    }
    /// Set `resolved`.
    pub fn resolved(mut self, v: bool) -> Self {
        self.resolved = Some(v);
        self
    }
    /// Set `cloudProvider` (comma-joined).
    pub fn cloud_provider<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudProviderNin` (comma-joined).
    pub fn cloud_provider_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `awsSecurityGroups__contains` (comma-joined).
    pub fn aws_security_groups_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_security_groups_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudAccount__contains` (comma-joined).
    pub fn cloud_account_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_account_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudImage__contains` (comma-joined).
    pub fn cloud_image_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_image_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudInstanceId__contains` (comma-joined).
    pub fn cloud_instance_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_id_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudInstanceSize__contains` (comma-joined).
    pub fn cloud_instance_size_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_size_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudLocation__contains` (comma-joined).
    pub fn cloud_location_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_location_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudNetwork__contains` (comma-joined).
    pub fn cloud_network_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_network_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `awsRole__contains` (comma-joined).
    pub fn aws_role_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_role_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `awsSubnetIds__contains` (comma-joined).
    pub fn aws_subnet_ids_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_subnet_ids_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `azureResourceGroup__contains` (comma-joined).
    pub fn azure_resource_group_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.azure_resource_group_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `gcpServiceAccount__contains` (comma-joined).
    pub fn gcp_service_account_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gcp_service_account_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
}

/// Query params for `GET /web/api/v2.1/threats/export` (Export Threats).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportThreatsQuery {
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Indicates a tenant scope request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<bool>,
    /// Created at lesser than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "createdAt__lt")]
    pub created_at_lt: Option<String>,
    /// Created at greater than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "createdAt__gt")]
    pub created_at_gt: Option<String>,
    /// Created at lesser or equal than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "createdAt__lte")]
    pub created_at_lte: Option<String>,
    /// Created at greater or equal than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "createdAt__gte")]
    pub created_at_gte: Option<String>,
    /// Updated at lesser than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updatedAt__lt")]
    pub updated_at_lt: Option<String>,
    /// Updated at greater than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updatedAt__gt")]
    pub updated_at_gt: Option<String>,
    /// Updated at lesser or equal than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updatedAt__lte")]
    pub updated_at_lte: Option<String>,
    /// Updated at greater or equal than (ISO-8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "updatedAt__gte")]
    pub updated_at_gte: Option<String>,
    /// List of sha1 hashes to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_hashes: Option<String>,
    /// Display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Filter threats by a specific status. Allowed: not_mitigated, mitigated, marked_as_benign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_statuses: Option<String>,
    /// Filter threats not by a specific status. Allowed: not_mitigated, mitigated, marked_as_benign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_statuses_nin: Option<String>,
    /// List of Agent IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<String>,
    /// List of Agent context to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storylines: Option<String>,
    /// List of threat IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// List of collection IDs to search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<String>,
    /// Included engines. Allowed: reputation, sentinelone_cloud, cloud_detection, user_blacklist, pre_execution, pre_execution_suspicious, executables, data_files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines: Option<String>,
    /// Excluded engines. Allowed: reputation, sentinelone_cloud, cloud_detection, user_blacklist, pre_execution, pre_execution_suspicious, executables, data_files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engines_nin: Option<String>,
    /// Included engines. Allowed: reputation, sentinelone_cloud, cloud_detection, user_blacklist, pre_execution, pre_execution_suspicious, executables, data_files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_engines: Option<String>,
    /// Excluded engines. Allowed: reputation, sentinelone_cloud, cloud_detection, user_blacklist, pre_execution, pre_execution_suspicious, executables, data_files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_engines_nin: Option<String>,
    /// List of threat classifications to search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications: Option<String>,
    /// List of threat classifications not to search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications_nin: Option<String>,
    /// Classification sources list. Allowed: Cloud, Behavioral, Static, Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_sources: Option<String>,
    /// Classification sources list to exclude. Allowed: Cloud, Behavioral, Static, Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_sources_nin: Option<String>,
    /// Include Agent versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_versions: Option<String>,
    /// Excluded Agent versions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_versions_nin: Option<String>,
    /// Include Agent machine types. Allowed: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_machine_types: Option<String>,
    /// Excluded Agent machine types. Allowed: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_machine_types_nin: Option<String>,
    /// Included OS types. Allowed: linux, macos, windows_legacy, windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Excluded OS types. Allowed: linux, macos, windows_legacy, windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_types_nin: Option<String>,
    /// Included OS Architectures. Allowed: 32 bit, 64 bit, ARM64.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_archs: Option<String>,
    /// Included OS names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_names: Option<String>,
    /// Excluded OS names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_names_nin: Option<String>,
    /// Include Agents currently connected to the Management Console.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_is_active: Option<bool>,
    /// Only include threats from specific initiating sources. Allowed: agent_policy, full_disk_scan, sentinelctl, dv_command, console_api, on_demand_scan, star_active, star_manual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<String>,
    /// Exclude threats with specific initiating sources. Allowed: agent_policy, full_disk_scan, sentinelctl, dv_command, console_api, on_demand_scan, star_active, star_manual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_by_nin: Option<String>,
    /// Filter threats by a specific confidence level. Allowed: malicious, suspicious, n/a.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_levels: Option<String>,
    /// Exclude threats with specific confidence level. Allowed: malicious, suspicious, n/a.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_levels_nin: Option<String>,
    /// Filter threats by a specific analyst verdict. Allowed: undefined, true_positive, false_positive, suspicious.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyst_verdicts: Option<String>,
    /// Exclude threats with specific analyst verdicts. Allowed: undefined, true_positive, false_positive, suspicious.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyst_verdicts_nin: Option<String>,
    /// Filter threats by a specific incident status. Allowed: unresolved, in_progress, resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_statuses: Option<String>,
    /// Exclude threats with specific incident statuses. Allowed: unresolved, in_progress, resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_statuses_nin: Option<String>,
    /// The threat contains at least one note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_exists: Option<bool>,
    /// At least one action failed on the threat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_actions: Option<bool>,
    /// A reboot is required on any endpoint for at least one action on the threat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot_required: Option<bool>,
    /// At least one action is pending for the Agent for the threat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_actions: Option<bool>,
    /// The threat contains ticket number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ticket_exists: Option<bool>,
    /// External ticket ID for the threat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ticket_ids: Option<String>,
    /// If the threat was detected pre-execution or post-execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigated_preemptively: Option<bool>,
    /// Filter threats by assigned tags to the related agent (JSON object).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_tags_data: Option<String>,
    /// Include only Threats whose Agent is assigned any tags if True, or none if False.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_agent_tags: Option<bool>,
    /// Full text search across multiple threat fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Free-text filter by file content hash (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "contentHash__contains")]
    pub content_hash_contains: Option<String>,
    /// Free-text filter by threat details (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threatDetails__contains")]
    pub threat_details_contains: Option<String>,
    /// Free-text filter by file path (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filePath__contains")]
    pub file_path_contains: Option<String>,
    /// Free-text filter by computer name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "computerName__contains")]
    pub computer_name_contains: Option<String>,
    /// Free-text filter by Agent UUID (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "uuid__contains")]
    pub uuid_contains: Option<String>,
    /// Free-text filter by Agent version at detection time (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detectionAgentVersion__contains")]
    pub detection_agent_version_contains: Option<String>,
    /// Free-text filter by Agent version at current time (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "realtimeAgentVersion__contains")]
    pub realtime_agent_version_contains: Option<String>,
    /// Free-text filter by Agent domain at detection time (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detectionAgentDomain__contains")]
    pub detection_agent_domain_contains: Option<String>,
    /// Free-text filter by threat command line arguments (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "commandLineArguments__contains")]
    pub command_line_arguments_contains: Option<String>,
    /// Free-text filter by the username that initiated that threat (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "initiatedByUsername__contains")]
    pub initiated_by_username_contains: Option<String>,
    /// Free-text filter by threat storyline (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storyline__contains")]
    pub storyline_contains: Option<String>,
    /// Free-text filter by the originated process name of the threat (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "originatedProcess__contains")]
    pub originated_process_contains: Option<String>,
    /// Free-text filter by threat's publisher name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "publisherName__contains")]
    pub publisher_name_contains: Option<String>,
    /// Free-text filter by threat's signer identity (certificate ID) (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "signerIdentity__contains")]
    pub signer_identity_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes cluster name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sClusterName__contains")]
    pub k8s_cluster_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes node name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sNodeName__contains")]
    pub k8s_node_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes node labels (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sNodeLabels__contains")]
    pub k8s_node_labels_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes namespace name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sNamespaceName__contains")]
    pub k8s_namespace_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes namespace labels (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sNamespaceLabels__contains")]
    pub k8s_namespace_labels_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes controller name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sControllerName__contains")]
    pub k8s_controller_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes controller labels (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sControllerLabels__contains")]
    pub k8s_controller_labels_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes pod name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sPodName__contains")]
    pub k8s_pod_name_contains: Option<String>,
    /// Free-text filter by the endpoint Kubernetes pod labels (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "k8sPodLabels__contains")]
    pub k8s_pod_labels_contains: Option<String>,
    /// Free-text filter by the endpoint container name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerName__contains")]
    pub container_name_contains: Option<String>,
    /// Free-text filter by the endpoint container image name (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerImageName__contains")]
    pub container_image_name_contains: Option<String>,
    /// Free-text filter by the endpoint container labels (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "containerLabels__contains")]
    pub container_labels_contains: Option<String>,
    /// Free-text filter by the threat external ticket ID (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "externalTicketId__contains")]
    pub external_ticket_id_contains: Option<String>,
    /// Comma-separated list of fields to be shown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// Used for backward-compatibility with API 2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
    /// Agents from which cloud provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// Exclude Agents from these cloud provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_nin: Option<String>,
    /// Free-text filter by aws securityGroups (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "awsSecurityGroups__contains")]
    pub aws_security_groups_contains: Option<String>,
    /// Free-text filter by cloud account (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudAccount__contains")]
    pub cloud_account_contains: Option<String>,
    /// Free-text filter by cloud image (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudImage__contains")]
    pub cloud_image_contains: Option<String>,
    /// Free-text filter by cloud instance id (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudInstanceId__contains")]
    pub cloud_instance_id_contains: Option<String>,
    /// Free-text filter by cloud instance size (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudInstanceSize__contains")]
    pub cloud_instance_size_contains: Option<String>,
    /// Free-text filter by cloud location (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudLocation__contains")]
    pub cloud_location_contains: Option<String>,
    /// Free-text filter by cloud network (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudNetwork__contains")]
    pub cloud_network_contains: Option<String>,
    /// Free-text filter by aws role (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "awsRole__contains")]
    pub aws_role_contains: Option<String>,
    /// Free-text filter by aws subnet ids (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "awsSubnetIds__contains")]
    pub aws_subnet_ids_contains: Option<String>,
    /// Free-text filter by azure resource group (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "azureResourceGroup__contains")]
    pub azure_resource_group_contains: Option<String>,
    /// Free-text filter by gcp service account (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gcpServiceAccount__contains")]
    pub gcp_service_account_contains: Option<String>,
}

impl ExportThreatsQuery {
    /// Set `accountIds` (comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `siteIds` (comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `groupIds` (comma-joined).
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `tenant`.
    pub fn tenant(mut self, v: bool) -> Self {
        self.tenant = Some(v);
        self
    }
    /// Set `createdAt__lt`.
    pub fn created_at_lt(mut self, v: impl Into<String>) -> Self {
        self.created_at_lt = Some(v.into());
        self
    }
    /// Set `createdAt__gt`.
    pub fn created_at_gt(mut self, v: impl Into<String>) -> Self {
        self.created_at_gt = Some(v.into());
        self
    }
    /// Set `createdAt__lte`.
    pub fn created_at_lte(mut self, v: impl Into<String>) -> Self {
        self.created_at_lte = Some(v.into());
        self
    }
    /// Set `createdAt__gte`.
    pub fn created_at_gte(mut self, v: impl Into<String>) -> Self {
        self.created_at_gte = Some(v.into());
        self
    }
    /// Set `updatedAt__lt`.
    pub fn updated_at_lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lt = Some(v.into());
        self
    }
    /// Set `updatedAt__gt`.
    pub fn updated_at_gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gt = Some(v.into());
        self
    }
    /// Set `updatedAt__lte`.
    pub fn updated_at_lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_lte = Some(v.into());
        self
    }
    /// Set `updatedAt__gte`.
    pub fn updated_at_gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at_gte = Some(v.into());
        self
    }
    /// Set `contentHashes` (comma-joined).
    pub fn content_hashes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.content_hashes = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `displayName`.
    pub fn display_name(mut self, v: impl Into<String>) -> Self {
        self.display_name = Some(v.into());
        self
    }
    /// Set `mitigationStatuses` (comma-joined).
    pub fn mitigation_statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mitigation_statuses = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `mitigationStatusesNin` (comma-joined).
    pub fn mitigation_statuses_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mitigation_statuses_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentIds` (comma-joined).
    pub fn agent_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `storylines` (comma-joined).
    pub fn storylines<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storylines = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `ids` (comma-joined).
    pub fn ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `collectionIds` (comma-joined).
    pub fn collection_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.collection_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `engines` (comma-joined).
    pub fn engines<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.engines = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `enginesNin` (comma-joined).
    pub fn engines_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.engines_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `detectionEngines` (comma-joined).
    pub fn detection_engines<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detection_engines = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `detectionEnginesNin` (comma-joined).
    pub fn detection_engines_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detection_engines_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `classifications` (comma-joined).
    pub fn classifications<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.classifications = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `classificationsNin` (comma-joined).
    pub fn classifications_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.classifications_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `classificationSources` (comma-joined).
    pub fn classification_sources<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.classification_sources = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `classificationSourcesNin` (comma-joined).
    pub fn classification_sources_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.classification_sources_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentVersions` (comma-joined).
    pub fn agent_versions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentVersionsNin` (comma-joined).
    pub fn agent_versions_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentMachineTypes` (comma-joined).
    pub fn agent_machine_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_machine_types = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentMachineTypesNin` (comma-joined).
    pub fn agent_machine_types_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_machine_types_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `osTypes` (comma-joined).
    pub fn os_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `osTypesNin` (comma-joined).
    pub fn os_types_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `osArchs` (comma-joined).
    pub fn os_archs<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_archs = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `osNames` (comma-joined).
    pub fn os_names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_names = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `osNamesNin` (comma-joined).
    pub fn os_names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_names_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `agentIsActive`.
    pub fn agent_is_active(mut self, v: bool) -> Self {
        self.agent_is_active = Some(v);
        self
    }
    /// Set `initiatedBy` (comma-joined).
    pub fn initiated_by<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.initiated_by = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `initiatedByNin` (comma-joined).
    pub fn initiated_by_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.initiated_by_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `confidenceLevels` (comma-joined).
    pub fn confidence_levels<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.confidence_levels = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `confidenceLevelsNin` (comma-joined).
    pub fn confidence_levels_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.confidence_levels_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `analystVerdicts` (comma-joined).
    pub fn analyst_verdicts<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.analyst_verdicts = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `analystVerdictsNin` (comma-joined).
    pub fn analyst_verdicts_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.analyst_verdicts_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `incidentStatuses` (comma-joined).
    pub fn incident_statuses<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.incident_statuses = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `incidentStatusesNin` (comma-joined).
    pub fn incident_statuses_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.incident_statuses_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `noteExists`.
    pub fn note_exists(mut self, v: bool) -> Self {
        self.note_exists = Some(v);
        self
    }
    /// Set `failedActions`.
    pub fn failed_actions(mut self, v: bool) -> Self {
        self.failed_actions = Some(v);
        self
    }
    /// Set `rebootRequired`.
    pub fn reboot_required(mut self, v: bool) -> Self {
        self.reboot_required = Some(v);
        self
    }
    /// Set `pendingActions`.
    pub fn pending_actions(mut self, v: bool) -> Self {
        self.pending_actions = Some(v);
        self
    }
    /// Set `externalTicketExists`.
    pub fn external_ticket_exists(mut self, v: bool) -> Self {
        self.external_ticket_exists = Some(v);
        self
    }
    /// Set `externalTicketIds` (comma-joined).
    pub fn external_ticket_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ticket_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `mitigatedPreemptively`.
    pub fn mitigated_preemptively(mut self, v: bool) -> Self {
        self.mitigated_preemptively = Some(v);
        self
    }
    /// Set `agentTagsData`.
    pub fn agent_tags_data(mut self, v: impl Into<String>) -> Self {
        self.agent_tags_data = Some(v.into());
        self
    }
    /// Set `hasAgentTags`.
    pub fn has_agent_tags(mut self, v: bool) -> Self {
        self.has_agent_tags = Some(v);
        self
    }
    /// Set `query`.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Set `contentHash__contains` (comma-joined).
    pub fn content_hash_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.content_hash_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `threatDetails__contains` (comma-joined).
    pub fn threat_details_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_details_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `filePath__contains` (comma-joined).
    pub fn file_path_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.file_path_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `computerName__contains` (comma-joined).
    pub fn computer_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `uuid__contains` (comma-joined).
    pub fn uuid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuid_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `detectionAgentVersion__contains` (comma-joined).
    pub fn detection_agent_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detection_agent_version_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `realtimeAgentVersion__contains` (comma-joined).
    pub fn realtime_agent_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.realtime_agent_version_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `detectionAgentDomain__contains` (comma-joined).
    pub fn detection_agent_domain_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detection_agent_domain_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `commandLineArguments__contains` (comma-joined).
    pub fn command_line_arguments_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.command_line_arguments_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `initiatedByUsername__contains` (comma-joined).
    pub fn initiated_by_username_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.initiated_by_username_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `storyline__contains` (comma-joined).
    pub fn storyline_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storyline_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `originatedProcess__contains` (comma-joined).
    pub fn originated_process_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.originated_process_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `publisherName__contains` (comma-joined).
    pub fn publisher_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.publisher_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `signerIdentity__contains` (comma-joined).
    pub fn signer_identity_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.signer_identity_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sClusterName__contains` (comma-joined).
    pub fn k8s_cluster_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sNodeName__contains` (comma-joined).
    pub fn k8s_node_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sNodeLabels__contains` (comma-joined).
    pub fn k8s_node_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_labels_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sNamespaceName__contains` (comma-joined).
    pub fn k8s_namespace_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sNamespaceLabels__contains` (comma-joined).
    pub fn k8s_namespace_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace_labels_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sControllerName__contains` (comma-joined).
    pub fn k8s_controller_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_controller_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sControllerLabels__contains` (comma-joined).
    pub fn k8s_controller_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_controller_labels_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sPodName__contains` (comma-joined).
    pub fn k8s_pod_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_pod_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `k8sPodLabels__contains` (comma-joined).
    pub fn k8s_pod_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_pod_labels_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `containerName__contains` (comma-joined).
    pub fn container_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.container_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `containerImageName__contains` (comma-joined).
    pub fn container_image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.container_image_name_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `containerLabels__contains` (comma-joined).
    pub fn container_labels_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.container_labels_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `externalTicketId__contains` (comma-joined).
    pub fn external_ticket_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ticket_id_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `countsFor`.
    pub fn counts_for(mut self, v: impl Into<String>) -> Self {
        self.counts_for = Some(v.into());
        self
    }
    /// Set `resolved`.
    pub fn resolved(mut self, v: bool) -> Self {
        self.resolved = Some(v);
        self
    }
    /// Set `cloudProvider` (comma-joined).
    pub fn cloud_provider<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudProviderNin` (comma-joined).
    pub fn cloud_provider_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_nin = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `awsSecurityGroups__contains` (comma-joined).
    pub fn aws_security_groups_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_security_groups_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudAccount__contains` (comma-joined).
    pub fn cloud_account_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_account_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudImage__contains` (comma-joined).
    pub fn cloud_image_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_image_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudInstanceId__contains` (comma-joined).
    pub fn cloud_instance_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_id_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudInstanceSize__contains` (comma-joined).
    pub fn cloud_instance_size_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_size_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudLocation__contains` (comma-joined).
    pub fn cloud_location_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_location_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `cloudNetwork__contains` (comma-joined).
    pub fn cloud_network_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_network_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `awsRole__contains` (comma-joined).
    pub fn aws_role_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_role_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `awsSubnetIds__contains` (comma-joined).
    pub fn aws_subnet_ids_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_subnet_ids_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `azureResourceGroup__contains` (comma-joined).
    pub fn azure_resource_group_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.azure_resource_group_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `gcpServiceAccount__contains` (comma-joined).
    pub fn gcp_service_account_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gcp_service_account_contains = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
}

/// Query params for `GET /web/api/v2.1/threats/{threat_id}/explore/events` (Get Events).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEventsQuery {
    /// Skip first number of items (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total item count is not calculated (faster).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Column to sort by. Allowed: id, createdAt, eventType, fileSize, fileType, registryPath, registryId, registryClassification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed: asc, desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Filter events by type. Allowed: events, file, ip, url, dns, process, registry, scheduled_task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<String>,
    /// Filter events by sub-type. Allowed: PROCESSCREATION, PROCESSTERMINATION, PROCESSMODIFICATION, TCPV4, TCPV6, TCPV4LISTEN, TCPV6LISTEN, FILEMODIFICATION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_sub_types: Option<String>,
    /// Filter by a specific process key and its children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Filter by process name (substring).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "processName__like")]
    pub process_name_like: Option<String>,
}

impl GetEventsQuery {
    /// Set `skip`.
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Set `limit`.
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Set `cursor`.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Set `countOnly`.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Set `skipCount`.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Set `sortBy`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Set `sortOrder`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Set `eventTypes` (comma-joined).
    pub fn event_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.event_types = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `eventSubTypes` (comma-joined).
    pub fn event_sub_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.event_sub_types = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `eventId`.
    pub fn event_id(mut self, v: impl Into<String>) -> Self {
        self.event_id = Some(v.into());
        self
    }
    /// Set `processName__like`.
    pub fn process_name_like(mut self, v: impl Into<String>) -> Self {
        self.process_name_like = Some(v.into());
        self
    }
}

/// Query params for `GET /web/api/v2.1/threats/{threat_id}/quarantined-files` (Get Quarantined Files).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuarantinedFilesQuery {
    /// Skip first number of items (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total item count is not calculated (faster).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
}

impl QuarantinedFilesQuery {
    /// Set `skip`.
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Set `limit`.
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Set `cursor`.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Set `countOnly`.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Set `skipCount`.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
}

/// Query params for `GET /web/api/v2.1/threats/{threat_id}/timeline` (Get Threat Timeline).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineQuery {
    /// Skip first number of items (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only the total number of items is returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total item count is not calculated (faster).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Column to sort by. Allowed: hash, activityType, primaryDescription, secondaryDescription, createdAt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Allowed: asc, desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Full text search for fields: hash, primary_description, secondary_description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return only these activity codes. Allowed activity codes: 6, 7, 8, 17, 43, 160, 161, 162.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_types: Option<String>,
}

impl TimelineQuery {
    /// Set `skip`.
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
        self
    }
    /// Set `limit`.
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    /// Set `cursor`.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Set `countOnly`.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Set `skipCount`.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Set `sortBy`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Set `sortOrder`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// Set `siteIds` (comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `accountIds` (comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `groupIds` (comma-joined).
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `query`.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Set `activityTypes` (comma-joined).
    pub fn activity_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.activity_types = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
}

/// Query params for `GET /web/api/v2.1/export/threats/{threat_id}/timeline` (Export Threat Timeline).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportTimelineQuery {
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Full text search for fields: hash, primary_description, secondary_description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Return only these activity codes. Allowed activity codes: 6, 7, 8, 17, 43, 160, 161, 162.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_types: Option<String>,
}

impl ExportTimelineQuery {
    /// Set `siteIds` (comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `accountIds` (comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `groupIds` (comma-joined).
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `query`.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }
    /// Set `activityTypes` (comma-joined).
    pub fn activity_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.activity_types = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/export/threats/{threat_id}/explore/events` (Export Events).
///
/// The `format` field is **required** by the API.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportEventsQuery {
    /// Exported file format. **Required.** Allowed: json, csv.
    pub format: String,
    /// Filter events by type. Allowed: events, file, ip, url, dns, process, registry, scheduled_task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<String>,
    /// Filter events by sub-type. Allowed: PROCESSCREATION, PROCESSTERMINATION, PROCESSMODIFICATION, TCPV4, TCPV6, TCPV4LISTEN, TCPV6LISTEN, FILEMODIFICATION.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_sub_types: Option<String>,
    /// Filter by a specific process key and its children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Filter by process name (substring).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "processName__like")]
    pub process_name_like: Option<String>,
}

impl ExportEventsQuery {
    /// New query with the required `format` (`json` or `csv`).
    pub fn new(format: impl Into<String>) -> Self {
        Self {
            format: format.into(),
            event_types: None,
            event_sub_types: None,
            event_id: None,
            process_name_like: None,
        }
    }
    /// Set `eventTypes` (comma-joined).
    pub fn event_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.event_types = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `eventSubTypes` (comma-joined).
    pub fn event_sub_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.event_sub_types = Some(v.into_iter().map(|s| s.as_ref().to_owned()).collect::<Vec<_>>().join(","));
        self
    }
    /// Set `eventId`.
    pub fn event_id(mut self, v: impl Into<String>) -> Self {
        self.event_id = Some(v.into());
        self
    }
    /// Set `processName__like`.
    pub fn process_name_like(mut self, v: impl Into<String>) -> Self {
        self.process_name_like = Some(v.into());
        self
    }
}

/// Request body for the container network connect/disconnect actions.
#[derive(Debug, Clone, Serialize)]
pub struct ContainerNetworkBody {
    /// Operation data. **Required.** Freeform object (see API docs).
    pub data: serde_json::Value,
    /// Threat filter selecting the affected threats. **Required.** Freeform object.
    pub filter: serde_json::Value,
}

impl ContainerNetworkBody {
    /// New body from `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for `POST /web/api/v2.1/threats/add-to-blacklist`.
#[derive(Debug, Clone, Serialize)]
pub struct AddToBlocklistBody {
    /// Operation data. **Required.** Freeform object (see API docs).
    pub data: serde_json::Value,
    /// Threat filter selecting the affected threats. **Required.** Freeform object.
    pub filter: serde_json::Value,
}

impl AddToBlocklistBody {
    /// New body from `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for `POST /web/api/v2.1/threats/add-to-exclusions`.
#[derive(Debug, Clone, Serialize)]
pub struct AddToExclusionsBody {
    /// Operation data. **Required.** Freeform object (see API docs).
    pub data: serde_json::Value,
    /// Threat filter selecting the affected threats. **Required.** Freeform object.
    pub filter: serde_json::Value,
}

impl AddToExclusionsBody {
    /// New body from `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for `POST /web/api/v2.1/threats/analyst-verdict`.
#[derive(Debug, Clone, Serialize)]
pub struct AnalystVerdictBody {
    /// Operation data. **Required.** Freeform object (see API docs).
    pub data: serde_json::Value,
    /// Threat filter selecting the affected threats. **Required.** Freeform object.
    pub filter: serde_json::Value,
}

impl AnalystVerdictBody {
    /// New body from `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for `POST /web/api/v2.1/threats/external-ticket-id`.
#[derive(Debug, Clone, Serialize)]
pub struct ExternalTicketBody {
    /// Operation data. **Required.** Freeform object (see API docs).
    pub data: serde_json::Value,
    /// Threat filter selecting the affected threats. **Required.** Freeform object.
    pub filter: serde_json::Value,
}

impl ExternalTicketBody {
    /// New body from `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for `POST /web/api/v2.1/threats/fetch-file`.
#[derive(Debug, Clone, Serialize)]
pub struct FetchFileBody {
    /// Operation data. **Required.** Freeform object (see API docs).
    pub data: serde_json::Value,
    /// Threat filter selecting the affected threats. **Required.** Freeform object.
    pub filter: serde_json::Value,
}

impl FetchFileBody {
    /// New body from `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for `POST /web/api/v2.1/threats/incident`.
#[derive(Debug, Clone, Serialize)]
pub struct IncidentBody {
    /// Operation data. **Required.** Freeform object (see API docs).
    pub data: serde_json::Value,
    /// Threat filter selecting the affected threats. **Required.** Freeform object.
    pub filter: serde_json::Value,
}

impl IncidentBody {
    /// New body from `data` and `filter` objects.
    pub fn new(data: serde_json::Value, filter: serde_json::Value) -> Self {
        Self { data, filter }
    }
}

/// Request body for `POST /web/api/v2.1/threats/dv-add-to-blacklist`.
#[derive(Debug, Clone, Serialize)]
pub struct DvAddToBlacklistBody {
    /// Operation data. **Required.** Freeform object (see API docs).
    pub data: serde_json::Value,
}

impl DvAddToBlacklistBody {
    /// New body from a `data` object.
    pub fn new(data: serde_json::Value) -> Self {
        Self { data }
    }
}

/// Request body for `POST /web/api/v2.1/threats/dv-mark-as-threat`.
#[derive(Debug, Clone, Serialize)]
pub struct DvMarkAsThreatBody {
    /// Operation data. **Required.** Freeform object (see API docs).
    pub data: serde_json::Value,
}

impl DvMarkAsThreatBody {
    /// New body from a `data` object.
    pub fn new(data: serde_json::Value) -> Self {
        Self { data }
    }
}

/// Request body for `POST /web/api/v2.1/threats/mitigate-alerts`.
#[derive(Debug, Clone, Serialize)]
pub struct MitigateAlertsBody {
    /// Operation data. **Required.** Freeform object (see API docs).
    pub data: serde_json::Value,
}

impl MitigateAlertsBody {
    /// New body from a `data` object.
    pub fn new(data: serde_json::Value) -> Self {
        Self { data }
    }
}

/// Request body for `POST /web/api/v2.1/threats/engines/disable`.
#[derive(Debug, Clone, Default, Serialize)]
pub struct EngineDisableBody {
    /// Engines to disable. Optional freeform object.
    /// Valid engine values: penetration, dataFiles, exploits, reputation,
    /// executables, preExecutionSuspicious, preExecution, lateralMovement, pup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl EngineDisableBody {
    /// Set the `data` object.
    pub fn data(mut self, v: serde_json::Value) -> Self {
        self.data = Some(v);
        self
    }
}

/// Request body for `POST /web/api/v2.1/threats/mitigate/{action}`.
#[derive(Debug, Clone, Serialize)]
pub struct MitigateBody {
    /// Optional operation data. Freeform object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Threat filter selecting the affected threats. **Required.** Freeform object.
    pub filter: serde_json::Value,
}

impl MitigateBody {
    /// New body from the required `filter` object.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { data: None, filter }
    }
    /// Set the optional `data` object.
    pub fn data(mut self, v: serde_json::Value) -> Self {
        self.data = Some(v);
        self
    }
}

impl ThreatsService<'_> {
    /// Export Events.
    ///
    /// Export threat events in CSV or JSON format.
    ///
    /// `GET /web/api/v2.1/export/threats/{threat_id}/explore/events`
    pub async fn export_events(&self, threat_id: impl Into<String>, query: &ExportEventsQuery) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/export/threats/{}/explore/events", threat_id.into());
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// Export Threat Timeline.
    ///
    /// Export a threat's timeline.
    ///
    /// `GET /web/api/v2.1/export/threats/{threat_id}/timeline`
    pub async fn export_timeline(&self, threat_id: impl Into<String>, query: &ExportTimelineQuery) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/export/threats/{}/timeline", threat_id.into());
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// Get Threats.
    ///
    /// Get data of threats that match the filter.
    /// Best Practice: Use the filters. Each threat gives a number of data lines
    /// that will quickly fill the page limit.
    ///
    /// `GET /web/api/v2.1/threats`
    pub async fn list(&self, query: &ThreatsQuery) -> Result<Paginated<Threat>, Error> {
        let path = "/web/api/v2.1/threats";
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(path, q).await?)
    }

    /// Reconnect Container.
    ///
    /// Restore network to a container that was disconnected.
    ///
    /// `POST /web/api/v2.1/threats/actions/container-network-connect`
    pub async fn container_network_connect(&self, body: &ContainerNetworkBody) -> Result<Response<SuccessResult>, Error> {
        let path = "/web/api/v2.1/threats/actions/container-network-connect";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Disconnect Container.
    ///
    /// Network quarantine a specific container.
    ///
    /// `POST /web/api/v2.1/threats/actions/container-network-disconnect`
    pub async fn container_network_disconnect(&self, body: &ContainerNetworkBody) -> Result<Response<SuccessResult>, Error> {
        let path = "/web/api/v2.1/threats/actions/container-network-disconnect";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Add to Blocklist.
    ///
    /// Add threats that have a SHA1 hash and match the filter to the Blocklist of
    /// the target scope: Global, Account, Site, or Group.
    ///
    /// `POST /web/api/v2.1/threats/add-to-blacklist`
    pub async fn add_to_blocklist(&self, body: &AddToBlocklistBody) -> Result<Response<ThreatRestrictionResult>, Error> {
        let path = "/web/api/v2.1/threats/add-to-blacklist";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Add to Exclusions.
    ///
    /// Add a threat to exclusions. The whitening option is required.
    /// Use with caution: an exclusion overrides the malicious verdict.
    ///
    /// `POST /web/api/v2.1/threats/add-to-exclusions`
    pub async fn add_to_exclusions(&self, body: &AddToExclusionsBody) -> Result<Response<ThreatRestrictionResult>, Error> {
        let path = "/web/api/v2.1/threats/add-to-exclusions";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Update Threat Analyst Verdict.
    ///
    /// Change the verdict of a threat, as determined by a Console user.
    ///
    /// `POST /web/api/v2.1/threats/analyst-verdict`
    pub async fn analyst_verdict(&self, body: &AnalystVerdictBody) -> Result<Response<ThreatAnalystVerdictResult>, Error> {
        let path = "/web/api/v2.1/threats/analyst-verdict";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Add to Blocklist (Deep Visibility).
    ///
    /// From Deep Visibility results, add a SHA1 hash to the Blocklist.
    /// The SHA1 and the Agent ID are required. Requires Complete SKU.
    ///
    /// `POST /web/api/v2.1/threats/dv-add-to-blacklist`
    pub async fn dv_add_to_blocklist(&self, body: &DvAddToBlacklistBody) -> Result<Response<AffectedResult>, Error> {
        let path = "/web/api/v2.1/threats/dv-add-to-blacklist";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Mark as Threat (Deep Visibility).
    ///
    /// Mark an event from Deep Visibility data as a threat.
    /// The item is marked as a threat and added to the blocklist.
    ///
    /// `POST /web/api/v2.1/threats/dv-mark-as-threat`
    pub async fn dv_mark_as_threat(&self, body: &DvMarkAsThreatBody) -> Result<Response<AffectedResult>, Error> {
        let path = "/web/api/v2.1/threats/dv-mark-as-threat";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Disable Engines.
    ///
    /// Troubleshoot Agent Engines that return unexpected results.
    /// Valid values: penetration, dataFiles, exploits, reputation, executables,
    /// preExecutionSuspicious, preExecution, lateralMovement, pup.
    ///
    /// `POST /web/api/v2.1/threats/engines/disable`
    pub async fn disable_engines(&self, body: &EngineDisableBody) -> Result<Response<SuccessResult>, Error> {
        let path = "/web/api/v2.1/threats/engines/disable";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Export Threats.
    ///
    /// Export data of threats (as seen in Console > Incidents) that match the
    /// filter. Exports only 20,000 items (each datum is an item).
    ///
    /// `GET /web/api/v2.1/threats/export`
    pub async fn export(&self, query: &ExportThreatsQuery) -> Result<Response<serde_json::Value>, Error> {
        let path = "/web/api/v2.1/threats/export";
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(path, q).await?)
    }

    /// Update Threat External Ticket ID.
    ///
    /// Change the external ticket ID of a threat.
    ///
    /// `POST /web/api/v2.1/threats/external-ticket-id`
    pub async fn external_ticket_id(&self, body: &ExternalTicketBody) -> Result<Response<AffectedResult>, Error> {
        let path = "/web/api/v2.1/threats/external-ticket-id";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Fetch Threat File.
    ///
    /// Fetch a file associated with the threat that matches the filter.
    /// Returns the number of affected agents.
    ///
    /// `POST /web/api/v2.1/threats/fetch-file`
    pub async fn fetch_file(&self, body: &FetchFileBody) -> Result<Response<AffectedResult>, Error> {
        let path = "/web/api/v2.1/threats/fetch-file";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Update Threat Incident.
    ///
    /// Update the incident details of a threat.
    ///
    /// `POST /web/api/v2.1/threats/incident`
    pub async fn incident(&self, body: &IncidentBody) -> Result<Response<ThreatIncidentResult>, Error> {
        let path = "/web/api/v2.1/threats/incident";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Mitigate Alerts.
    ///
    /// Mark alerts as a threat and run a mitigation action from the Management UI.
    ///
    /// `POST /web/api/v2.1/threats/mitigate-alerts`
    pub async fn mitigate_alerts(&self, body: &MitigateAlertsBody) -> Result<Response<AffectedResult>, Error> {
        let path = "/web/api/v2.1/threats/mitigate-alerts";
        Ok(self.client.http().post(path, body).await?)
    }

    /// Mitigate Threats.
    ///
    /// Apply a mitigation action to a group of threats that match the filter.
    /// `action` allowed values: kill, remediate, rollback-remediation, quarantine,
    /// un-quarantine, None, remove_macros, restore_macros.
    /// Rollback is Windows only; remediate is macOS and Windows only.
    ///
    /// `POST /web/api/v2.1/threats/mitigate/{action}`
    pub async fn mitigate(&self, action: impl Into<String>, body: &MitigateBody) -> Result<Response<ThreatMitigationActionResult>, Error> {
        let path = format!("/web/api/v2.1/threats/mitigate/{}", action.into());
        Ok(self.client.http().post(&path, body).await?)
    }

    /// Export Mitigation Report.
    ///
    /// Export the mitigation report as a CSV file.
    ///
    /// `GET /web/api/v2.1/threats/mitigation-report/{report_id}`
    pub async fn mitigation_report(&self, report_id: impl Into<String>) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/threats/mitigation-report/{}", report_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// Download from Cloud.
    ///
    /// Download threat file from cloud.
    ///
    /// `GET /web/api/v2.1/threats/{threat_id}/download-from-cloud`
    pub async fn download_from_cloud(&self, threat_id: impl Into<String>) -> Result<Response<ThreatFileDownload>, Error> {
        let path = format!("/web/api/v2.1/threats/{}/download-from-cloud", threat_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }

    /// Get Events.
    ///
    /// Get all threat events.
    ///
    /// `GET /web/api/v2.1/threats/{threat_id}/explore/events`
    pub async fn get_events(&self, threat_id: impl Into<String>, query: &GetEventsQuery) -> Result<Paginated<ThreatEvent>, Error> {
        let path = format!("/web/api/v2.1/threats/{}/explore/events", threat_id.into());
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// Get Quarantined Files.
    ///
    /// Get quarantined files for a single threat by alert ID with pagination.
    ///
    /// `GET /web/api/v2.1/threats/{threat_id}/quarantined-files`
    pub async fn quarantined_files(&self, threat_id: impl Into<String>, query: &QuarantinedFilesQuery) -> Result<Paginated<QuarantinedFile>, Error> {
        let path = format!("/web/api/v2.1/threats/{}/quarantined-files", threat_id.into());
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// Get Threat Timeline.
    ///
    /// Get a threat's timeline.
    ///
    /// `GET /web/api/v2.1/threats/{threat_id}/timeline`
    pub async fn timeline(&self, threat_id: impl Into<String>, query: &TimelineQuery) -> Result<Paginated<ThreatTimelineEntry>, Error> {
        let path = format!("/web/api/v2.1/threats/{}/timeline", threat_id.into());
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get(&path, q).await?)
    }

    /// Exclusion Options.
    ///
    /// Get the Exclusion types that can be created from the detection data.
    ///
    /// `GET /web/api/v2.1/threats/{threat_id}/whitening-options`
    pub async fn whitening_options(&self, threat_id: impl Into<String>) -> Result<Response<WhiteningOptions>, Error> {
        let path = format!("/web/api/v2.1/threats/{}/whitening-options", threat_id.into());
        Ok(self.client.http().get(&path, None).await?)
    }
}
