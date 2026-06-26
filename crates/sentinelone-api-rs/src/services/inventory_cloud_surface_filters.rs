//! Service for the `Inventory Cloud Surface Filters` tag (Inventory Cloud
//! Surface Resource Filters).
//!
//! Hand-written for 1:1 parity with the SentinelOne Management API spec
//! (`swagger_2_1.json` / `docs/inventory-cloud-surface-filters.md`).

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_cloud_surface_filters::{
    AutoCompleteResponse, CountFiltersResponse, FreeTextFilterResponse,
};
use crate::pagination::Response;

/// `Inventory Cloud Surface Filters` tag — Inventory Cloud Surface Resource Filters.
pub struct InventoryCloudSurfaceFiltersService<'a> {
    pub(crate) client: &'a ManagementClient,
}


/// Query params for `GET /web/api/v2.1/xdr/assets/surface/cloud/filters/autocomplete`.
///
/// Array params are serialized comma-joined, as the API expects. Every field is `Option<T>`;
/// `key` and `text` are required by the spec but are kept optional here so the struct can be
/// built incrementally — populate them before calling.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutocompleteQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Enum values: `critical`, `high`,
    /// `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in). Enum values: `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The state of the instance (not in). Optional.
    #[serde(rename = "state__nin", skip_serializing_if = "Option::is_none")]
    pub state_nin: Option<String>,
    /// The missing coverage for the asset. Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`,
    /// `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// Running on nodes. Optional.
    #[serde(rename = "k8sRunningOnNodes__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes_contains: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CDS malware scan status (not in). Optional.
    #[serde(rename = "scanStatus__nin", skip_serializing_if = "Option::is_none")]
    pub scan_status_nin: Option<String>,
    /// The region. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The memory of the device in human readable format. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The encryption type (not in). Optional.
    #[serde(rename = "encryptionType__nin", skip_serializing_if = "Option::is_none")]
    pub encryption_type_nin: Option<String>,
    /// The IP addresses. Optional.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Running on Nodes. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes: Option<String>,
    /// Tag Keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The state of the instance. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Policy Type. Optional.
    #[serde(rename = "k8sPolicyType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_policy_type_contains: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The risk factors associated with the asset (not in). Enum values: `Unresolved Alerts`, `High
    /// Value`. Optional.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// The operating system of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// The cloud provider account ID. Optional.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The subnets. Optional.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// Service Type. Optional.
    #[serde(rename = "k8sServiceType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_service_type_contains: Option<String>,
    /// The instance role. Optional.
    #[serde(rename = "instanceRole__contains", skip_serializing_if = "Option::is_none")]
    pub instance_role_contains: Option<String>,
    /// Threat Detection Status (not in). Optional.
    #[serde(rename = "threatDetectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub threat_detection_status_nin: Option<String>,
    /// The active coverage for the asset (not in). Enum values: `CWS`, `CDS`, `EPP`, `Ranger
    /// Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. Optional.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// Search field key. Enum values: `resourceType__contains`, `id__contains`, `name__contains`,
    /// `tagsKey__contains`, `tagsKeyValue__contains`, `agentCustomerIdentifier__contains`,
    /// `agentLocationAwareness__contains`, `agentS1AgentLiveUpdatesVersion__contains`. Required by
    /// the spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
    /// The region (not in). Optional.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs. Optional.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to. Enum values: `Cloud`, `Identity`, `Network`,
    /// `Endpoint`, `Network Discovery`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The number of objects in the bucket (not in). Optional.
    #[serde(rename = "objectCount__nin", skip_serializing_if = "Option::is_none")]
    pub object_count_nin: Option<String>,
    /// The operating system name and version of the device (not in). Optional.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// Kubernetes Resource ID. Optional.
    #[serde(rename = "k8sResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id_contains: Option<String>,
    /// The missing coverage for the asset (not in). Enum values: `CWS`, `CDS`, `EPP`, `Ranger
    /// Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. Optional.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). Enum values: `Active`, `Inactive`. Optional.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The state. Optional.
    #[serde(rename = "state__contains", skip_serializing_if = "Option::is_none")]
    pub state_contains: Option<String>,
    /// Search term text. Required by the spec.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The asset review (not in). Enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty). Optional.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Threat Detection Policy. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_policy_status: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The Kubernetes node. Optional.
    #[serde(rename = "k8sNode__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_contains: Option<String>,
    /// The severity of the alert. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The Kubernetes Node (not in). Optional.
    #[serde(rename = "k8sNode__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_node_nin: Option<String>,
    /// The OS names and versions. Optional.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The domain. Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). Enum values: `Access Control and
    /// Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD
    /// Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain` (abbreviated; see API
    /// docs for the full list). Optional.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs. Optional.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The operating system of the device (not in). Optional.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// The Kubernetes Cluster (not in). Optional.
    #[serde(rename = "k8sCluster__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_nin: Option<String>,
    /// Namespace Name. Optional.
    #[serde(rename = "k8sNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace_contains: Option<String>,
    /// The Kubernetes Resource ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Enum values: `Unresolved Alerts`, `High Value`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// Free-text filter by Kubernetes Labels key (supports multiple values). Optional.
    #[serde(rename = "k8sLabelsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_contains: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The number of objects in the bucket. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<String>,
    /// The operating system family of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The Surface that each asset belongs to (not in). Enum values: `Cloud`, `Identity`,
    /// `Network`, `Endpoint`, `Network Discovery`. Optional.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The encryption type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values). Optional.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in). Optional.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// Free-text filter by Kubernetes Labels key value (supports multiple values). Optional.
    #[serde(rename = "k8sLabelsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_value_contains: Option<String>,
    /// Tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The Kubernetes cluster. Optional.
    #[serde(rename = "k8sCluster__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_contains: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// Update Strategy. Optional.
    #[serde(rename = "k8sUpdateStrategy__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_update_strategy_contains: Option<String>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The Kubernetes Cluster. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_cluster: Option<String>,
    /// The MAC addresses. Optional.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The instance type. Optional.
    #[serde(rename = "instanceType__contains", skip_serializing_if = "Option::is_none")]
    pub instance_type_contains: Option<String>,
    /// The name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by Kubernetes Annotations key (supports multiple values). Optional.
    #[serde(rename = "k8sAnnotationsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_contains: Option<String>,
    /// The operating system version of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The threat detection status. Optional.
    #[serde(rename = "threatDetectionStatus__contains", skip_serializing_if = "Option::is_none")]
    pub threat_detection_status_contains: Option<String>,
    /// The criticality that each asset belongs to. Enum values: `critical`, `high`, `medium`,
    /// `low`, `--`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// The Kubernetes Node. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_node: Option<String>,
    /// The Kubernetes Cluster ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id: Option<String>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The virtual network ID. Optional.
    #[serde(rename = "virtualNetworkId__contains", skip_serializing_if = "Option::is_none")]
    pub virtual_network_id_contains: Option<String>,
    /// The instance ID. Optional.
    #[serde(rename = "instanceId__contains", skip_serializing_if = "Option::is_none")]
    pub instance_id_contains: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Enum values: `Access Control and Surveillance
    /// System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate
    /// Template`, `AD Containers`, `AD DNS Zone`, `AD Domain` (abbreviated; see API docs for the
    /// full list). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in).
    /// Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// The Kubernetes Version (not in). Optional.
    #[serde(rename = "k8sVersion__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_version_nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Deployment Strategy. Optional.
    #[serde(rename = "k8sDeploymentStrategy__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_deployment_strategy_contains: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The number of cores. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in). Optional.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in). Enum values: `All`, `Access Key and
    /// Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative
    /// Unit`, `Admission Controller` (abbreviated; see API docs for the full list). Optional.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The Kubernetes Version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_version: Option<String>,
    /// The status alerts of the asset. Enum values: `Infected`, `Healthy`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset. Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`,
    /// `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in). Optional.
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The network security group. Optional.
    #[serde(rename = "networkSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub network_security_groups_contains: Option<String>,
    /// Threat Detection Status. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_status: Option<String>,
    /// The architecture of the device (not in). Optional.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// The Kubernetes Namespace Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_namespace: Option<String>,
    /// The Kubernetes Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_type: Option<String>,
    /// The cloud provider account id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// Whether the instance is a rogue or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rogues: Option<String>,
    /// The sub-category that each resource belongs to. Enum values: `All`, `Access Key and Secret`,
    /// `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller` (abbreviated; see API docs for the full list). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The image ID. Optional.
    #[serde(rename = "imageId__contains", skip_serializing_if = "Option::is_none")]
    pub image_id_contains: Option<String>,
    /// The OS versions. Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review. Enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`,
    /// `` (empty). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The hostnames. Optional.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// Limit number of returned items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The Kubernetes Type (not in). Optional.
    #[serde(rename = "k8sType__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_type_nin: Option<String>,
    /// Service Name. Optional.
    #[serde(rename = "k8sServiceName__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_service_name_contains: Option<String>,
    /// Free-text filter by Kubernetes Annotations key value (supports multiple values). Optional.
    #[serde(rename = "k8sAnnotationsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_value_contains: Option<String>,
    /// The CDS malware scan status. Optional.
    #[serde(rename = "scanStatus__contains", skip_serializing_if = "Option::is_none")]
    pub scan_status_contains: Option<String>,
    /// The architecture of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The internal IPs. Optional.
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The subnet ID. Optional.
    #[serde(rename = "subnetId__contains", skip_serializing_if = "Option::is_none")]
    pub subnet_id_contains: Option<String>,
    /// The status of the asset. Enum values: `Active`, `Inactive`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The domain of the device (not in). Optional.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// Kubernetes Cluster ID. Optional.
    #[serde(rename = "k8sClusterId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id_contains: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Optional.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The domain of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The operating system family of the device (not in). Optional.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The CDS malware scan status. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// The operating system name and version of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl AutocompleteQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to (not in).
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(join_csv(v));
        self
    }
    /// The status alerts of the asset (not in).
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(join_csv(v));
        self
    }
    /// The state of the instance (not in).
    pub fn state_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state_nin = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset.
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(join_csv(v));
        self
    }
    /// User and cloud tags.
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(join_csv(v));
        self
    }
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(join_csv(v));
        self
    }
    /// Running on nodes.
    pub fn k8s_running_on_nodes_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_running_on_nodes_contains = Some(join_csv(v));
        self
    }
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(join_csv(v));
        self
    }
    /// The cloud resource ID.
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id_contains = Some(join_csv(v));
        self
    }
    /// Tag Keys exists.
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(join_csv(v));
        self
    }
    /// The CDS malware scan status (not in).
    pub fn scan_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_nin = Some(join_csv(v));
        self
    }
    /// The region.
    pub fn region<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(v));
        self
    }
    /// The memory of the device in human readable format.
    pub fn memory_readable<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(join_csv(v));
        self
    }
    /// The encryption type (not in).
    pub fn encryption_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type_nin = Some(join_csv(v));
        self
    }
    /// The IP addresses.
    pub fn ip_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by tag key value (supports multiple values).
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_nin = Some(join_csv(v));
        self
    }
    /// Running on Nodes.
    pub fn k8s_running_on_nodes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_running_on_nodes = Some(join_csv(v));
        self
    }
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(join_csv(v));
        self
    }
    /// The state of the instance.
    pub fn state<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state = Some(join_csv(v));
        self
    }
    /// Policy Type.
    pub fn k8s_policy_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_policy_type_contains = Some(join_csv(v));
        self
    }
    /// Tag Keys not exists.
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(join_csv(v));
        self
    }
    /// The risk factors associated with the asset (not in).
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(join_csv(v));
        self
    }
    /// The operating system of the device.
    pub fn os<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(join_csv(v));
        self
    }
    /// The cloud provider account ID.
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(join_csv(v));
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
    /// The subnets.
    pub fn subnets_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets_contains = Some(join_csv(v));
        self
    }
    /// Service Type.
    pub fn k8s_service_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_service_type_contains = Some(join_csv(v));
        self
    }
    /// The instance role.
    pub fn instance_role_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.instance_role_contains = Some(join_csv(v));
        self
    }
    /// Threat Detection Status (not in).
    pub fn threat_detection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_nin = Some(join_csv(v));
        self
    }
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(join_csv(v));
        self
    }
    /// Search field key.
    pub fn key(mut self, v: impl Into<String>) -> Self {
        self.key = Some(v.into());
        self
    }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(join_csv(v));
        self
    }
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(join_csv(v));
        self
    }
    /// The gateway IPs.
    pub fn gateway_ips_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips_contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to.
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(join_csv(v));
        self
    }
    /// The number of objects in the bucket (not in).
    pub fn object_count_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count_nin = Some(join_csv(v));
        self
    }
    /// The operating system name and version of the device (not in).
    pub fn os_name_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_nin = Some(join_csv(v));
        self
    }
    /// Kubernetes Resource ID.
    pub fn k8s_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_resource_id_contains = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset (not in).
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(join_csv(v));
        self
    }
    /// The status of the asset (not in).
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(join_csv(v));
        self
    }
    /// The state.
    pub fn state_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state_contains = Some(join_csv(v));
        self
    }
    /// Search term text.
    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.text = Some(v.into());
        self
    }
    /// The geographical area where cloud resources are hosted.
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(join_csv(v));
        self
    }
    /// The asset review (not in).
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(join_csv(v));
        self
    }
    /// Threat Detection Policy.
    pub fn threat_detection_policy_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_policy_status = Some(join_csv(v));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(join_csv(v));
        self
    }
    /// Asset Contact Email (not in).
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes node.
    pub fn k8s_node_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_contains = Some(join_csv(v));
        self
    }
    /// The severity of the alert.
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(join_csv(v));
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
    /// The cloud tags key value.
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value = Some(join_csv(v));
        self
    }
    /// The Kubernetes Node (not in).
    pub fn k8s_node_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_nin = Some(join_csv(v));
        self
    }
    /// The OS names and versions.
    pub fn os_name_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_contains = Some(join_csv(v));
        self
    }
    /// The domain.
    pub fn domain_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_contains = Some(join_csv(v));
        self
    }
    /// The canonical name for the resource type (not in).
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(join_csv(v));
        self
    }
    /// The gateway MACs.
    pub fn gateway_macs_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs_contains = Some(join_csv(v));
        self
    }
    /// The operating system of the device (not in).
    pub fn os_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes Cluster (not in).
    pub fn k8s_cluster_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_nin = Some(join_csv(v));
        self
    }
    /// Namespace Name.
    pub fn k8s_namespace_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace_contains = Some(join_csv(v));
        self
    }
    /// The Kubernetes Resource ID.
    pub fn k8s_resource_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_resource_id = Some(join_csv(v));
        self
    }
    /// Asset Contact Email.
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(join_csv(v));
        self
    }
    /// The risk factors associated with the asset.
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(v));
        self
    }
    /// Free-text filter by Kubernetes Labels key (supports multiple values).
    pub fn k8s_labels_unified_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_labels_unified_key_contains = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(join_csv(v));
        self
    }
    /// The number of objects in the bucket.
    pub fn object_count<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count = Some(join_csv(v));
        self
    }
    /// The operating system family of the device.
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(join_csv(v));
        self
    }
    /// The encryption type.
    pub fn encryption_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type = Some(join_csv(v));
        self
    }
    /// The ID.
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_contains = Some(join_csv(v));
        self
    }
    /// The Asset Type.
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(join_csv(v));
        self
    }
    /// The memory of the device in human readable format (not in).
    pub fn memory_readable_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable_nin = Some(join_csv(v));
        self
    }
    /// Free-text filter by Kubernetes Labels key value (supports multiple values).
    pub fn k8s_labels_unified_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_labels_unified_key_value_contains = Some(join_csv(v));
        self
    }
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(join_csv(v));
        self
    }
    /// The Kubernetes cluster.
    pub fn k8s_cluster_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider organization unit.
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider organization.
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_contains = Some(join_csv(v));
        self
    }
    /// Update Strategy.
    pub fn k8s_update_strategy_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_update_strategy_contains = Some(join_csv(v));
        self
    }
    /// Name.
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join_csv(v));
        self
    }
    /// The Kubernetes Cluster.
    pub fn k8s_cluster<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster = Some(join_csv(v));
        self
    }
    /// The MAC addresses.
    pub fn mac_addresses_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses_contains = Some(join_csv(v));
        self
    }
    /// The instance type.
    pub fn instance_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.instance_type_contains = Some(join_csv(v));
        self
    }
    /// The name.
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by Kubernetes Annotations key (supports multiple values).
    pub fn k8s_annotations_unified_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_annotations_unified_key_contains = Some(join_csv(v));
        self
    }
    /// The operating system version of the device.
    pub fn os_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(join_csv(v));
        self
    }
    /// The threat detection status.
    pub fn threat_detection_status_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to.
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(v));
        self
    }
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes Node.
    pub fn k8s_node<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node = Some(join_csv(v));
        self
    }
    /// The Kubernetes Cluster ID.
    pub fn k8s_cluster_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_id = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(join_csv(v));
        self
    }
    /// The virtual network ID.
    pub fn virtual_network_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.virtual_network_id_contains = Some(join_csv(v));
        self
    }
    /// The instance ID.
    pub fn instance_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.instance_id_contains = Some(join_csv(v));
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
    /// The Last Seen date and time for the asset.
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    /// The canonical name for the resource type.
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in).
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes Version (not in).
    pub fn k8s_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version_nin = Some(join_csv(v));
        self
    }
    /// Name (not in).
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(join_csv(v));
        self
    }
    /// Deployment Strategy.
    pub fn k8s_deployment_strategy_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_deployment_strategy_contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key.
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key = Some(join_csv(v));
        self
    }
    /// The number of cores.
    pub fn core_count<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(join_csv(v));
        self
    }
    /// The operating system version of the device (not in).
    pub fn os_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_nin = Some(join_csv(v));
        self
    }
    /// The sub-category that each resource belongs to (not in).
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes Version.
    pub fn k8s_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version = Some(join_csv(v));
        self
    }
    /// The status alerts of the asset.
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(join_csv(v));
        self
    }
    /// The active coverage for the asset.
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(join_csv(v));
        self
    }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(join_csv(v));
        self
    }
    /// The number of cores (not in).
    pub fn core_count_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count_nin = Some(join_csv(v));
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self {
        self.csv_filter_id = Some(n);
        self
    }
    /// The network security group.
    pub fn network_security_groups_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_security_groups_contains = Some(join_csv(v));
        self
    }
    /// Threat Detection Status.
    pub fn threat_detection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status = Some(join_csv(v));
        self
    }
    /// The architecture of the device (not in).
    pub fn architecture_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes Namespace Name.
    pub fn k8s_namespace<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace = Some(join_csv(v));
        self
    }
    /// The Kubernetes Type.
    pub fn k8s_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type = Some(join_csv(v));
        self
    }
    /// The cloud provider account id.
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id = Some(join_csv(v));
        self
    }
    /// Whether the instance is a rogue or not.
    pub fn is_rogues<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_rogues = Some(join_csv(v));
        self
    }
    /// The sub-category that each resource belongs to.
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(join_csv(v));
        self
    }
    /// The image ID.
    pub fn image_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_id_contains = Some(join_csv(v));
        self
    }
    /// The OS versions.
    pub fn os_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(join_csv(v));
        self
    }
    /// The asset review.
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys.
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(join_csv(v));
        self
    }
    /// The hostnames.
    pub fn hostnames_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(join_csv(v));
        self
    }
    /// Limit number of returned items.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// The Kubernetes Type (not in).
    pub fn k8s_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type_nin = Some(join_csv(v));
        self
    }
    /// Service Name.
    pub fn k8s_service_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_service_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by Kubernetes Annotations key value (supports multiple values).
    pub fn k8s_annotations_unified_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_annotations_unified_key_value_contains = Some(join_csv(v));
        self
    }
    /// The CDS malware scan status.
    pub fn scan_status_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_contains = Some(join_csv(v));
        self
    }
    /// The architecture of the device.
    pub fn architecture<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(join_csv(v));
        self
    }
    /// The ID.
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(join_csv(v));
        self
    }
    /// The internal IPs.
    pub fn internal_ips_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips_contains = Some(join_csv(v));
        self
    }
    /// The subnet ID.
    pub fn subnet_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnet_id_contains = Some(join_csv(v));
        self
    }
    /// The status of the asset.
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(join_csv(v));
        self
    }
    /// The domain of the device (not in).
    pub fn domain_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(join_csv(v));
        self
    }
    /// Kubernetes Cluster ID.
    pub fn k8s_cluster_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_id_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key (supports multiple values).
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_contains = Some(join_csv(v));
        self
    }
    /// Tags (not in).
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys exists.
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(join_csv(v));
        self
    }
    /// The domain of the device.
    pub fn domain<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(join_csv(v));
        self
    }
    /// The operating system family of the device (not in).
    pub fn os_family_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family_nin = Some(join_csv(v));
        self
    }
    /// The CDS malware scan status.
    pub fn scan_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status = Some(join_csv(v));
        self
    }
    /// The operating system name and version of the device.
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(join_csv(v));
        self
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/surface/cloud/filters/count`.
///
/// Array params are serialized comma-joined, as the API expects. Every field is `Option<T>`
/// (all params are optional for this endpoint).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Enum values: `critical`, `high`,
    /// `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in). Enum values: `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The state of the instance (not in). Optional.
    #[serde(rename = "state__nin", skip_serializing_if = "Option::is_none")]
    pub state_nin: Option<String>,
    /// The missing coverage for the asset. Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`,
    /// `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// Running on nodes. Optional.
    #[serde(rename = "k8sRunningOnNodes__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes_contains: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CDS malware scan status (not in). Optional.
    #[serde(rename = "scanStatus__nin", skip_serializing_if = "Option::is_none")]
    pub scan_status_nin: Option<String>,
    /// The region. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The memory of the device in human readable format. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The encryption type (not in). Optional.
    #[serde(rename = "encryptionType__nin", skip_serializing_if = "Option::is_none")]
    pub encryption_type_nin: Option<String>,
    /// The IP addresses. Optional.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Running on Nodes. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes: Option<String>,
    /// Tag Keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The state of the instance. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Policy Type. Optional.
    #[serde(rename = "k8sPolicyType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_policy_type_contains: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The risk factors associated with the asset (not in). Enum values: `Unresolved Alerts`, `High
    /// Value`. Optional.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// The operating system of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// The cloud provider account ID. Optional.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The subnets. Optional.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// Service Type. Optional.
    #[serde(rename = "k8sServiceType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_service_type_contains: Option<String>,
    /// The instance role. Optional.
    #[serde(rename = "instanceRole__contains", skip_serializing_if = "Option::is_none")]
    pub instance_role_contains: Option<String>,
    /// Threat Detection Status (not in). Optional.
    #[serde(rename = "threatDetectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub threat_detection_status_nin: Option<String>,
    /// The active coverage for the asset (not in). Enum values: `CWS`, `CDS`, `EPP`, `Ranger
    /// Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. Optional.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
    /// The region (not in). Optional.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs. Optional.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to. Enum values: `Cloud`, `Identity`, `Network`,
    /// `Endpoint`, `Network Discovery`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The number of objects in the bucket (not in). Optional.
    #[serde(rename = "objectCount__nin", skip_serializing_if = "Option::is_none")]
    pub object_count_nin: Option<String>,
    /// The operating system name and version of the device (not in). Optional.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// Kubernetes Resource ID. Optional.
    #[serde(rename = "k8sResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id_contains: Option<String>,
    /// The missing coverage for the asset (not in). Enum values: `CWS`, `CDS`, `EPP`, `Ranger
    /// Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. Optional.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). Enum values: `Active`, `Inactive`. Optional.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The state. Optional.
    #[serde(rename = "state__contains", skip_serializing_if = "Option::is_none")]
    pub state_contains: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The asset review (not in). Enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty). Optional.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Threat Detection Policy. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_policy_status: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The Kubernetes node. Optional.
    #[serde(rename = "k8sNode__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_contains: Option<String>,
    /// The severity of the alert. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The Kubernetes Node (not in). Optional.
    #[serde(rename = "k8sNode__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_node_nin: Option<String>,
    /// The OS names and versions. Optional.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The domain. Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). Enum values: `Access Control and
    /// Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD
    /// Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain` (abbreviated; see API
    /// docs for the full list). Optional.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs. Optional.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The operating system of the device (not in). Optional.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// The Kubernetes Cluster (not in). Optional.
    #[serde(rename = "k8sCluster__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_nin: Option<String>,
    /// Namespace Name. Optional.
    #[serde(rename = "k8sNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace_contains: Option<String>,
    /// The Kubernetes Resource ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Enum values: `Unresolved Alerts`, `High Value`.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// Free-text filter by Kubernetes Labels key (supports multiple values). Optional.
    #[serde(rename = "k8sLabelsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_contains: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The number of objects in the bucket. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<String>,
    /// The operating system family of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The Surface that each asset belongs to (not in). Enum values: `Cloud`, `Identity`,
    /// `Network`, `Endpoint`, `Network Discovery`. Optional.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The encryption type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values). Optional.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in). Optional.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// Free-text filter by Kubernetes Labels key value (supports multiple values). Optional.
    #[serde(rename = "k8sLabelsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_value_contains: Option<String>,
    /// Tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The Kubernetes cluster. Optional.
    #[serde(rename = "k8sCluster__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_contains: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// Update Strategy. Optional.
    #[serde(rename = "k8sUpdateStrategy__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_update_strategy_contains: Option<String>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The Kubernetes Cluster. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_cluster: Option<String>,
    /// The MAC addresses. Optional.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The instance type. Optional.
    #[serde(rename = "instanceType__contains", skip_serializing_if = "Option::is_none")]
    pub instance_type_contains: Option<String>,
    /// The name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Free-text filter by Kubernetes Annotations key (supports multiple values). Optional.
    #[serde(rename = "k8sAnnotationsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_contains: Option<String>,
    /// The operating system version of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The threat detection status. Optional.
    #[serde(rename = "threatDetectionStatus__contains", skip_serializing_if = "Option::is_none")]
    pub threat_detection_status_contains: Option<String>,
    /// The criticality that each asset belongs to. Enum values: `critical`, `high`, `medium`,
    /// `low`, `--`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// The Kubernetes Node. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_node: Option<String>,
    /// The Kubernetes Cluster ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id: Option<String>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The virtual network ID. Optional.
    #[serde(rename = "virtualNetworkId__contains", skip_serializing_if = "Option::is_none")]
    pub virtual_network_id_contains: Option<String>,
    /// The instance ID. Optional.
    #[serde(rename = "instanceId__contains", skip_serializing_if = "Option::is_none")]
    pub instance_id_contains: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Enum values: `Access Control and Surveillance
    /// System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate
    /// Template`, `AD Containers`, `AD DNS Zone`, `AD Domain` (abbreviated; see API docs for the
    /// full list). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in).
    /// Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// The Kubernetes Version (not in). Optional.
    #[serde(rename = "k8sVersion__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_version_nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Deployment Strategy. Optional.
    #[serde(rename = "k8sDeploymentStrategy__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_deployment_strategy_contains: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The number of cores. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in). Optional.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in). Enum values: `All`, `Access Key and
    /// Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative
    /// Unit`, `Admission Controller` (abbreviated; see API docs for the full list). Optional.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The Kubernetes Version. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_version: Option<String>,
    /// The status alerts of the asset. Enum values: `Infected`, `Healthy`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset. Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`,
    /// `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in). Optional.
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The network security group. Optional.
    #[serde(rename = "networkSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub network_security_groups_contains: Option<String>,
    /// Threat Detection Status. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_status: Option<String>,
    /// The architecture of the device (not in). Optional.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// The Kubernetes Namespace Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_namespace: Option<String>,
    /// The Kubernetes Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_type: Option<String>,
    /// The cloud provider account id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// Whether the instance is a rogue or not. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rogues: Option<String>,
    /// The sub-category that each resource belongs to. Enum values: `All`, `Access Key and Secret`,
    /// `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller` (abbreviated; see API docs for the full list). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The image ID. Optional.
    #[serde(rename = "imageId__contains", skip_serializing_if = "Option::is_none")]
    pub image_id_contains: Option<String>,
    /// The OS versions. Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review. Enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`,
    /// `` (empty). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The hostnames. Optional.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// The Kubernetes Type (not in). Optional.
    #[serde(rename = "k8sType__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_type_nin: Option<String>,
    /// Service Name. Optional.
    #[serde(rename = "k8sServiceName__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_service_name_contains: Option<String>,
    /// Free-text filter by Kubernetes Annotations key value (supports multiple values). Optional.
    #[serde(rename = "k8sAnnotationsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_value_contains: Option<String>,
    /// The CDS malware scan status. Optional.
    #[serde(rename = "scanStatus__contains", skip_serializing_if = "Option::is_none")]
    pub scan_status_contains: Option<String>,
    /// The architecture of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The internal IPs. Optional.
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The subnet ID. Optional.
    #[serde(rename = "subnetId__contains", skip_serializing_if = "Option::is_none")]
    pub subnet_id_contains: Option<String>,
    /// The status of the asset. Enum values: `Active`, `Inactive`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The domain of the device (not in). Optional.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// Kubernetes Cluster ID. Optional.
    #[serde(rename = "k8sClusterId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id_contains: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Optional.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The domain of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The operating system family of the device (not in). Optional.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The CDS malware scan status. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// The operating system name and version of the device. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl CountQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to (not in).
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(join_csv(v));
        self
    }
    /// The status alerts of the asset (not in).
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(join_csv(v));
        self
    }
    /// The state of the instance (not in).
    pub fn state_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state_nin = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset.
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(join_csv(v));
        self
    }
    /// User and cloud tags.
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(join_csv(v));
        self
    }
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(join_csv(v));
        self
    }
    /// Running on nodes.
    pub fn k8s_running_on_nodes_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_running_on_nodes_contains = Some(join_csv(v));
        self
    }
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(join_csv(v));
        self
    }
    /// The cloud resource ID.
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id_contains = Some(join_csv(v));
        self
    }
    /// Tag Keys exists.
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(join_csv(v));
        self
    }
    /// The CDS malware scan status (not in).
    pub fn scan_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_nin = Some(join_csv(v));
        self
    }
    /// The region.
    pub fn region<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(v));
        self
    }
    /// The memory of the device in human readable format.
    pub fn memory_readable<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(join_csv(v));
        self
    }
    /// The encryption type (not in).
    pub fn encryption_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type_nin = Some(join_csv(v));
        self
    }
    /// The IP addresses.
    pub fn ip_address_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by tag key value (supports multiple values).
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_nin = Some(join_csv(v));
        self
    }
    /// Running on Nodes.
    pub fn k8s_running_on_nodes<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_running_on_nodes = Some(join_csv(v));
        self
    }
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(join_csv(v));
        self
    }
    /// The state of the instance.
    pub fn state<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state = Some(join_csv(v));
        self
    }
    /// Policy Type.
    pub fn k8s_policy_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_policy_type_contains = Some(join_csv(v));
        self
    }
    /// Tag Keys not exists.
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(join_csv(v));
        self
    }
    /// The risk factors associated with the asset (not in).
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(join_csv(v));
        self
    }
    /// The operating system of the device.
    pub fn os<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(join_csv(v));
        self
    }
    /// The cloud provider account ID.
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(join_csv(v));
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
    /// The subnets.
    pub fn subnets_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets_contains = Some(join_csv(v));
        self
    }
    /// Service Type.
    pub fn k8s_service_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_service_type_contains = Some(join_csv(v));
        self
    }
    /// The instance role.
    pub fn instance_role_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.instance_role_contains = Some(join_csv(v));
        self
    }
    /// Threat Detection Status (not in).
    pub fn threat_detection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_nin = Some(join_csv(v));
        self
    }
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(join_csv(v));
        self
    }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(join_csv(v));
        self
    }
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(join_csv(v));
        self
    }
    /// The gateway IPs.
    pub fn gateway_ips_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips_contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to.
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(join_csv(v));
        self
    }
    /// The number of objects in the bucket (not in).
    pub fn object_count_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count_nin = Some(join_csv(v));
        self
    }
    /// The operating system name and version of the device (not in).
    pub fn os_name_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_nin = Some(join_csv(v));
        self
    }
    /// Kubernetes Resource ID.
    pub fn k8s_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_resource_id_contains = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset (not in).
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(join_csv(v));
        self
    }
    /// The status of the asset (not in).
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(join_csv(v));
        self
    }
    /// The state.
    pub fn state_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state_contains = Some(join_csv(v));
        self
    }
    /// The geographical area where cloud resources are hosted.
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(join_csv(v));
        self
    }
    /// The asset review (not in).
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(join_csv(v));
        self
    }
    /// Threat Detection Policy.
    pub fn threat_detection_policy_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_policy_status = Some(join_csv(v));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(join_csv(v));
        self
    }
    /// Asset Contact Email (not in).
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes node.
    pub fn k8s_node_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_contains = Some(join_csv(v));
        self
    }
    /// The severity of the alert.
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(join_csv(v));
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
    /// The cloud tags key value.
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value = Some(join_csv(v));
        self
    }
    /// The Kubernetes Node (not in).
    pub fn k8s_node_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_nin = Some(join_csv(v));
        self
    }
    /// The OS names and versions.
    pub fn os_name_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_contains = Some(join_csv(v));
        self
    }
    /// The domain.
    pub fn domain_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_contains = Some(join_csv(v));
        self
    }
    /// The canonical name for the resource type (not in).
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(join_csv(v));
        self
    }
    /// The gateway MACs.
    pub fn gateway_macs_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs_contains = Some(join_csv(v));
        self
    }
    /// The operating system of the device (not in).
    pub fn os_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes Cluster (not in).
    pub fn k8s_cluster_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_nin = Some(join_csv(v));
        self
    }
    /// Namespace Name.
    pub fn k8s_namespace_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace_contains = Some(join_csv(v));
        self
    }
    /// The Kubernetes Resource ID.
    pub fn k8s_resource_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_resource_id = Some(join_csv(v));
        self
    }
    /// Asset Contact Email.
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(join_csv(v));
        self
    }
    /// The risk factors associated with the asset.
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(v));
        self
    }
    /// Free-text filter by Kubernetes Labels key (supports multiple values).
    pub fn k8s_labels_unified_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_labels_unified_key_contains = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(join_csv(v));
        self
    }
    /// The number of objects in the bucket.
    pub fn object_count<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count = Some(join_csv(v));
        self
    }
    /// The operating system family of the device.
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(join_csv(v));
        self
    }
    /// The encryption type.
    pub fn encryption_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type = Some(join_csv(v));
        self
    }
    /// The ID.
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_contains = Some(join_csv(v));
        self
    }
    /// The Asset Type.
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(join_csv(v));
        self
    }
    /// The memory of the device in human readable format (not in).
    pub fn memory_readable_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable_nin = Some(join_csv(v));
        self
    }
    /// Free-text filter by Kubernetes Labels key value (supports multiple values).
    pub fn k8s_labels_unified_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_labels_unified_key_value_contains = Some(join_csv(v));
        self
    }
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(join_csv(v));
        self
    }
    /// The Kubernetes cluster.
    pub fn k8s_cluster_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider organization unit.
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider organization.
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_contains = Some(join_csv(v));
        self
    }
    /// Update Strategy.
    pub fn k8s_update_strategy_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_update_strategy_contains = Some(join_csv(v));
        self
    }
    /// Name.
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join_csv(v));
        self
    }
    /// The Kubernetes Cluster.
    pub fn k8s_cluster<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster = Some(join_csv(v));
        self
    }
    /// The MAC addresses.
    pub fn mac_addresses_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses_contains = Some(join_csv(v));
        self
    }
    /// The instance type.
    pub fn instance_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.instance_type_contains = Some(join_csv(v));
        self
    }
    /// The name.
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by Kubernetes Annotations key (supports multiple values).
    pub fn k8s_annotations_unified_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_annotations_unified_key_contains = Some(join_csv(v));
        self
    }
    /// The operating system version of the device.
    pub fn os_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(join_csv(v));
        self
    }
    /// The threat detection status.
    pub fn threat_detection_status_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to.
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(v));
        self
    }
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes Node.
    pub fn k8s_node<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node = Some(join_csv(v));
        self
    }
    /// The Kubernetes Cluster ID.
    pub fn k8s_cluster_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_id = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(join_csv(v));
        self
    }
    /// The virtual network ID.
    pub fn virtual_network_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.virtual_network_id_contains = Some(join_csv(v));
        self
    }
    /// The instance ID.
    pub fn instance_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.instance_id_contains = Some(join_csv(v));
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
    /// The Last Seen date and time for the asset.
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    /// The canonical name for the resource type.
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in).
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes Version (not in).
    pub fn k8s_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version_nin = Some(join_csv(v));
        self
    }
    /// Name (not in).
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(join_csv(v));
        self
    }
    /// Deployment Strategy.
    pub fn k8s_deployment_strategy_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_deployment_strategy_contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key.
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key = Some(join_csv(v));
        self
    }
    /// The number of cores.
    pub fn core_count<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(join_csv(v));
        self
    }
    /// The operating system version of the device (not in).
    pub fn os_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_nin = Some(join_csv(v));
        self
    }
    /// The sub-category that each resource belongs to (not in).
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes Version.
    pub fn k8s_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version = Some(join_csv(v));
        self
    }
    /// The status alerts of the asset.
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(join_csv(v));
        self
    }
    /// The active coverage for the asset.
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(join_csv(v));
        self
    }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(join_csv(v));
        self
    }
    /// The number of cores (not in).
    pub fn core_count_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count_nin = Some(join_csv(v));
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self {
        self.csv_filter_id = Some(n);
        self
    }
    /// The network security group.
    pub fn network_security_groups_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_security_groups_contains = Some(join_csv(v));
        self
    }
    /// Threat Detection Status.
    pub fn threat_detection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status = Some(join_csv(v));
        self
    }
    /// The architecture of the device (not in).
    pub fn architecture_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture_nin = Some(join_csv(v));
        self
    }
    /// The Kubernetes Namespace Name.
    pub fn k8s_namespace<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace = Some(join_csv(v));
        self
    }
    /// The Kubernetes Type.
    pub fn k8s_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type = Some(join_csv(v));
        self
    }
    /// The cloud provider account id.
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id = Some(join_csv(v));
        self
    }
    /// Whether the instance is a rogue or not.
    pub fn is_rogues<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_rogues = Some(join_csv(v));
        self
    }
    /// The sub-category that each resource belongs to.
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(join_csv(v));
        self
    }
    /// The image ID.
    pub fn image_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_id_contains = Some(join_csv(v));
        self
    }
    /// The OS versions.
    pub fn os_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version_contains = Some(join_csv(v));
        self
    }
    /// The asset review.
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys.
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(join_csv(v));
        self
    }
    /// The hostnames.
    pub fn hostnames_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames_contains = Some(join_csv(v));
        self
    }
    /// The Kubernetes Type (not in).
    pub fn k8s_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type_nin = Some(join_csv(v));
        self
    }
    /// Service Name.
    pub fn k8s_service_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_service_name_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by Kubernetes Annotations key value (supports multiple values).
    pub fn k8s_annotations_unified_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_annotations_unified_key_value_contains = Some(join_csv(v));
        self
    }
    /// The CDS malware scan status.
    pub fn scan_status_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_contains = Some(join_csv(v));
        self
    }
    /// The architecture of the device.
    pub fn architecture<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(join_csv(v));
        self
    }
    /// The ID.
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(v));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(join_csv(v));
        self
    }
    /// The internal IPs.
    pub fn internal_ips_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips_contains = Some(join_csv(v));
        self
    }
    /// The subnet ID.
    pub fn subnet_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnet_id_contains = Some(join_csv(v));
        self
    }
    /// The status of the asset.
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(join_csv(v));
        self
    }
    /// The domain of the device (not in).
    pub fn domain_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(join_csv(v));
        self
    }
    /// Kubernetes Cluster ID.
    pub fn k8s_cluster_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_id_contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key (supports multiple values).
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_contains = Some(join_csv(v));
        self
    }
    /// Tags (not in).
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys exists.
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(join_csv(v));
        self
    }
    /// The domain of the device.
    pub fn domain<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(join_csv(v));
        self
    }
    /// The operating system family of the device (not in).
    pub fn os_family_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family_nin = Some(join_csv(v));
        self
    }
    /// The CDS malware scan status.
    pub fn scan_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status = Some(join_csv(v));
        self
    }
    /// The operating system name and version of the device.
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(join_csv(v));
        self
    }
}

impl InventoryCloudSurfaceFiltersService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/surface/cloud/filters/autocomplete` — Auto Complete.
    ///
    /// Use this command to get values for other fields. When you send this
    /// command with input text and a field name, it returns auto-complete
    /// suggestions for the field.
    pub async fn autocomplete(
        &self,
        query: &AutocompleteQuery,
    ) -> Result<Response<AutoCompleteResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/surface/cloud/filters/autocomplete", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/surface/cloud/filters/count` — Filter counts.
    ///
    /// Get filter counts.
    pub async fn count(
        &self,
        query: &CountQuery,
    ) -> Result<Response<Vec<CountFiltersResponse>>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/surface/cloud/filters/count", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/surface/cloud/filters/free-text` — Free text filters.
    ///
    /// Get free text filters. This endpoint takes no parameters.
    pub async fn free_text(&self) -> Result<Response<Vec<FreeTextFilterResponse>>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/surface/cloud/filters/free-text", None)
            .await?)
    }
}

/// Joins an iterator of string-like values into a comma-separated string, as the
/// SentinelOne API expects for array-typed query params.
fn join_csv<I, S>(v: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    v.into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}
