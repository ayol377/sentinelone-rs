//! `Inventory Server Filters` tag — Inventory Server Resource Filters.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_server_filters::{
    AutoCompleteResponse, CountFiltersResponse, FreeTextFilterResponse,
};
use crate::pagination::Response;

/// `Inventory Server Filters` tag.
///
/// Inventory Server Resource Filters: auto-complete suggestions, filter
/// counts, and free-text filter descriptors for server-level XDR assets.
pub struct InventoryServerFiltersService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for
/// `GET /web/api/v2.1/xdr/assets/server/filters/autocomplete`.
///
/// All array params are serialized comma-joined, as the API expects. Every
/// field is optional except `text` and `key`, which are required by the spec;
/// set those via [`AutoCompleteQuery::text`] and [`AutoCompleteQuery::key`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteQuery {
    /// Free-text filter by tag key (supports multiple values).
    /// Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The agent operational state (not in). Optional.
    #[serde(rename = "agentOperationalState__nin", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state_nin: Option<String>,
    /// The criticality that each asset belongs to (not in). Allowed
    /// values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// Legacy Identity Policy Name. Optional.
    #[serde(rename = "legacyIdentityPolicyName", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The state of the instance (not in). Optional.
    #[serde(rename = "state__nin", skip_serializing_if = "Option::is_none")]
    pub state_nin: Option<String>,
    /// The missing coverage for the asset. Allowed values: `CWS`,
    /// `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data
    /// Classification`, `CNS KSPM`. Optional.
    #[serde(rename = "missingCoverage", skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(rename = "allTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The agent console migration status (not in). Optional.
    #[serde(rename = "agentConsoleMigrationStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status_nin: Option<String>,
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
    /// The agent free disk percentage on any of the disks. Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CPU. Optional.
    #[serde(rename = "cpu__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The region. Optional.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The memory of the device in human readable format. Optional.
    #[serde(rename = "memoryReadable", skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses. Optional.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values).
    /// Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Running on Nodes. Optional.
    #[serde(rename = "k8sRunningOnNodes", skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes: Option<String>,
    /// Tag Keys. Optional.
    #[serde(rename = "tagsKey", skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in). Optional.
    #[serde(rename = "agentVssProtectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The state of the instance. Optional.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The risk factors associated with the asset (not in). Allowed
    /// values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// AD machine or its groups. Optional.
    #[serde(rename = "identityAdMachine__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_contains: Option<String>,
    /// Is AD Connector. Optional.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The cloud provider account ID. Optional.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The agent missing permissions. Optional.
    #[serde(rename = "agentMissingPermissions", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent location (not in). Optional.
    #[serde(rename = "agentLocation__nin", skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// The operating system of the device. Optional.
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active. Optional.
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The subnets. Optional.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The ranger tags key. Optional.
    #[serde(rename = "rangerTagsKey", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in). Optional.
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name_nin: Option<String>,
    /// Search field key. Allowed values: `resourceType__contains`,
    /// `state__contains`, `cloudProviderOrganization__contains`,
    /// `cloudProviderOrganizationUnit__contains`,
    /// `cloudProviderProjectId__contains`,
    /// `cloudProviderSubscriptionId__contains`,
    /// `cloudProviderAccountName__contains`,
    /// `cloudProviderAccountId__contains`. Required.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// The instance role. Optional.
    #[serde(rename = "instanceRole__contains", skip_serializing_if = "Option::is_none")]
    pub instance_role_contains: Option<String>,
    /// The connection status between the agent and the SDL service.
    /// Optional.
    #[serde(rename = "agentDvConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in). Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data
    /// Classification`, `CNS KSPM`. Optional.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity. Optional.
    #[serde(rename = "agentConsoleConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity. Optional.
    #[serde(rename = "agentIdrConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The network name. Optional.
    #[serde(rename = "networkName", skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// The region (not in). Optional.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// Whether the agent can configure network quarantine (not in).
    /// Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs. Optional.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The manufacturer of the device (not in). Optional.
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer_nin: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to. Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    /// Optional.
    #[serde(rename = "surfaces", skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions. Optional.
    #[serde(rename = "agentPendingActions", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// Kubernetes Resource ID. Optional.
    #[serde(rename = "k8sResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id_contains: Option<String>,
    /// The operating system name and version of the device (not in).
    /// Optional.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in). Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data
    /// Classification`, `CNS KSPM`. Optional.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The serial number. Optional.
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in). Allowed values: `Active`,
    /// `Inactive`. Optional.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The last active date. Optional.
    #[serde(rename = "lastActiveDt__between", skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// The state. Optional.
    #[serde(rename = "state__contains", skip_serializing_if = "Option::is_none")]
    pub state_contains: Option<String>,
    /// The site from which the device was detected. Optional.
    #[serde(rename = "detectedFromSite", skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// Any AD string. Optional.
    #[serde(rename = "identityAd__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// The agent installer type. Optional.
    #[serde(rename = "agentInstallerType", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type. Optional.
    #[serde(rename = "agentDiskMetricsVolumeType", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name. Optional.
    #[serde(rename = "legacy_identity_policy_name__contains", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// The agent supported or unknown state. Optional.
    #[serde(rename = "eppUnsupportedUnknown", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// Search term text. Required.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The geographical area where cloud resources are hosted.
    /// Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The agent health status (not in). Optional.
    #[serde(rename = "agentHealthStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version (not in). Optional.
    #[serde(rename = "agentRangerVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version_nin: Option<String>,
    /// The asset review (not in). Allowed values: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    /// Optional.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(rename = "cloudProviderAccountName", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple
    /// values). Optional.
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value_contains: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__lte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The Kubernetes node. Optional.
    #[serde(rename = "k8sNode__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_contains: Option<String>,
    /// The severity of the alert. Optional.
    #[serde(rename = "alertSeverity", skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value. Optional.
    #[serde(rename = "cloudTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The Kubernetes Node (not in). Optional.
    #[serde(rename = "k8sNode__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_node_nin: Option<String>,
    /// The serial number (not in). Optional.
    #[serde(rename = "serialNumber__nin", skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// Whether the agent is decommissioned. Optional.
    #[serde(rename = "agentDecommissioned", skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time. Optional.
    #[serde(rename = "agentSubscribeOnDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The domain. Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The OS names and versions. Optional.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The canonical name for the resource type (not in). Allowed
    /// values: `Access Control and Surveillance System`, `Access
    /// Point`, `AD Certificate`, `AD Certificate Authority`, `AD
    /// Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD
    /// Domain`. Optional.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs. Optional.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The customer identifier. Optional.
    #[serde(rename = "agentCustomerIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier_contains: Option<String>,
    /// The operating system of the device (not in). Optional.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// The Kubernetes Cluster (not in). Optional.
    #[serde(rename = "k8sCluster__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_nin: Option<String>,
    /// Namespace Name. Optional.
    #[serde(rename = "k8sNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace_contains: Option<String>,
    /// The manufacturer. Optional.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// The Kubernetes Resource ID. Optional.
    #[serde(rename = "k8sResourceId", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(rename = "assetContactEmail", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports. Optional.
    #[serde(rename = "udpPorts", skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset. Allowed values:
    /// `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors", skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date. Optional.
    #[serde(rename = "agentFullDiskScanDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// Free-text filter by Kubernetes Labels key (supports multiple
    /// values). Optional.
    #[serde(rename = "k8sLabelsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_contains: Option<String>,
    /// The agent anti tampering status (not in). Optional.
    #[serde(rename = "agentAntiTamperingStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The environment that the asset exists in - AWS \. Optional.
    #[serde(rename = "assetEnvironment", skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device. Optional.
    #[serde(rename = "osFamily", skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups. Optional.
    #[serde(rename = "identityAdUser__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_contains: Option<String>,
    /// The Surface that each asset belongs to (not in). Allowed values:
    /// `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    /// Optional.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// ADS Enabled. Optional.
    #[serde(rename = "adsEnabled", skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The ranger tags key (not in). Optional.
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_nin: Option<String>,
    /// The agent location. Optional.
    #[serde(rename = "agentLocation", skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in). Optional.
    #[serde(rename = "agentPendingActions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions_nin: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Whether the agent is uninstalled. Optional.
    #[serde(rename = "agentUninstalled", skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple
    /// values). Optional.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in).
    /// Optional.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// Free-text filter by Kubernetes Labels key value (supports
    /// multiple values). Optional.
    #[serde(rename = "k8sLabelsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_value_contains: Option<String>,
    /// The UUID. Optional.
    #[serde(rename = "agentUuid__contains", skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// Tags. Optional.
    #[serde(rename = "tagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The Kubernetes cluster. Optional.
    #[serde(rename = "k8sCluster__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_contains: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The criticality that each asset belongs to. Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality", skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The ranger tags key value. Optional.
    #[serde(rename = "rangerTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// The location awareness. Optional.
    #[serde(rename = "agentLocationAwareness__contains", skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness_contains: Option<String>,
    /// Is DC Server. Optional.
    #[serde(rename = "isDcServer", skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// AD user DN. Optional.
    #[serde(rename = "identityAdUserDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// The operating system family of the device (not in). Optional.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The agent operational state. Optional.
    #[serde(rename = "agentOperationalState", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status. Optional.
    #[serde(rename = "agentNetworkStatus", skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name. Optional.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The Kubernetes Cluster. Optional.
    #[serde(rename = "k8sCluster", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster: Option<String>,
    /// The MAC addresses. Optional.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The instance type. Optional.
    #[serde(rename = "instanceType__contains", skip_serializing_if = "Option::is_none")]
    pub instance_type_contains: Option<String>,
    /// The agent VSS service status. Optional.
    #[serde(rename = "agentVssServiceStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The agent network scanner status (not in). Optional.
    #[serde(rename = "agentRangerStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status_nin: Option<String>,
    /// The site from which the device was detected (not in). Optional.
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site_nin: Option<String>,
    /// The name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The name of the application installed on a workstation or a
    /// server. Optional.
    #[serde(rename = "applicationName", skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// Free-text filter by Kubernetes Annotations key (supports
    /// multiple values). Optional.
    #[serde(rename = "k8sAnnotationsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_contains: Option<String>,
    /// The first seen date. Optional.
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The last update date. Optional.
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt_between: Option<String>,
    /// Whether the agent is pending uninstall. Optional.
    #[serde(rename = "agentPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The agent anti tampering status. Optional.
    #[serde(rename = "agentAntiTamperingStatus", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The Kubernetes Node. Optional.
    #[serde(rename = "k8sNode", skip_serializing_if = "Option::is_none")]
    pub k8s_node: Option<String>,
    /// The agent version (not in). Optional.
    #[serde(rename = "agentAgentVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_nin: Option<String>,
    /// The operating system version of the device. Optional.
    #[serde(rename = "osVersion", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The discovery methods (not in). Optional.
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods_nin: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// The Kubernetes Cluster ID. Optional.
    #[serde(rename = "k8sClusterId", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id: Option<String>,
    /// The agent VSS service status (not in). Optional.
    #[serde(rename = "agentVssServiceStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS last snapshot date. Optional.
    #[serde(rename = "agentVssLastSnapshotDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// Whether the agent has local configuration. Optional.
    #[serde(rename = "agentHasLocalConfig", skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The agent missing permissions (not in). Optional.
    #[serde(rename = "agentMissingPermissions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions_nin: Option<String>,
    /// Whether the agent is pending upgrade. Optional.
    #[serde(rename = "agentPendingUpgrade", skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The manufacturer of the device. Optional.
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The virtual network ID. Optional.
    #[serde(rename = "virtualNetworkId__contains", skip_serializing_if = "Option::is_none")]
    pub virtual_network_id_contains: Option<String>,
    /// The instance ID. Optional.
    #[serde(rename = "instanceId__contains", skip_serializing_if = "Option::is_none")]
    pub instance_id_contains: Option<String>,
    /// The agent VSS rollback status. Optional.
    #[serde(rename = "agentVssRollbackStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in). Optional.
    #[serde(rename = "agentInstallerType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type_nin: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Allowed values:
    /// `Access Control and Surveillance System`, `Access Point`, `AD
    /// Certificate`, `AD Certificate Authority`, `AD Certificate
    /// Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    /// Optional.
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS \. Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// The Kubernetes Version (not in). Optional.
    #[serde(rename = "k8sVersion__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_version_nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Match by the agent UUID. Optional.
    #[serde(rename = "agentUuid", skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(rename = "cloudTagsKey", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The number of cores. Optional.
    #[serde(rename = "coreCount", skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in). Optional.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in). Allowed
    /// values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`. Optional.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The Kubernetes Version. Optional.
    #[serde(rename = "k8sVersion", skip_serializing_if = "Option::is_none")]
    pub k8s_version: Option<String>,
    /// The discovery methods. Optional.
    #[serde(rename = "discoveryMethods", skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset. Allowed values: `Infected`,
    /// `Healthy`. Optional.
    #[serde(rename = "infectionStatus", skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports. Optional.
    #[serde(rename = "tcpPorts", skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset. Allowed values: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`. Optional.
    #[serde(rename = "activeCoverage", skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for.
    /// Optional.
    #[serde(rename = "countsFor", skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in). Optional.
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by. Optional.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values).
    /// Optional.
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_contains: Option<String>,
    /// The network security group. Optional.
    #[serde(rename = "networkSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub network_security_groups_contains: Option<String>,
    /// The ranger tags key value (not in). Optional.
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value_nin: Option<String>,
    /// The architecture of the device (not in). Optional.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// The Kubernetes Namespace Name. Optional.
    #[serde(rename = "k8sNamespace", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace: Option<String>,
    /// AD machine DN. Optional.
    #[serde(rename = "identityAdMachineDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// Live update ID. Optional.
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The Kubernetes Type. Optional.
    #[serde(rename = "k8sType", skip_serializing_if = "Option::is_none")]
    pub k8s_type: Option<String>,
    /// The cloud provider account id. Optional.
    #[serde(rename = "cloudProviderAccountId", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The agent network status (not in). Optional.
    #[serde(rename = "agentNetworkStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_network_status_nin: Option<String>,
    /// Whether the instance is a rogue or not. Optional.
    #[serde(rename = "isRogues", skip_serializing_if = "Option::is_none")]
    pub is_rogues: Option<String>,
    /// The sub-category that each resource belongs to. Allowed values:
    /// `All`, `Access Key and Secret`, `Access Management`, `Account`,
    /// `Account Group`, `AD Objects`, `Administrative Unit`, `Admission
    /// Controller`. Optional.
    #[serde(rename = "subCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The operating system name and version of the device. Optional.
    #[serde(rename = "osNameVersion", skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
    /// The image ID. Optional.
    #[serde(rename = "imageId__contains", skip_serializing_if = "Option::is_none")]
    pub image_id_contains: Option<String>,
    /// The agent network scanner status. Optional.
    #[serde(rename = "agentRangerStatus", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions. Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review. Allowed values: `Not Reviewed`, `Under
    /// Analysis`, `Not Trusted`, `Allowed`, `` (empty). Optional.
    #[serde(rename = "deviceReview", skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(rename = "allTagsKey", skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number. Optional.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number_contains: Option<String>,
    /// The hostnames. Optional.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// AD machine groups. Optional.
    #[serde(rename = "identityAdMachineMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// Limit number of returned items. Optional.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The Kubernetes Type (not in). Optional.
    #[serde(rename = "k8sType__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_type_nin: Option<String>,
    /// Free-text filter by Kubernetes Annotations key value (supports
    /// multiple values). Optional.
    #[serde(rename = "k8sAnnotationsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_value_contains: Option<String>,
    /// The agent VSS rollback status (not in). Optional.
    #[serde(rename = "agentVssRollbackStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent console migration status. Optional.
    #[serde(rename = "agentConsoleMigrationStatus", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device. Optional.
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups. Optional.
    #[serde(rename = "identityAdUserMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership_contains: Option<String>,
    /// The connection status between the agent and the SDL service (not
    /// in). Optional.
    #[serde(rename = "agentDvConnectivity__nin", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent VSS protection status. Optional.
    #[serde(rename = "agentVssProtectionStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
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
    /// The status of the asset. Allowed values: `Active`, `Inactive`.
    /// Optional.
    #[serde(rename = "assetStatus", skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in). Optional.
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown_nin: Option<String>,
    /// Whether the agent can configure network quarantine. Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status. Optional.
    #[serde(rename = "agentHealthStatus", skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version. Optional.
    #[serde(rename = "agentRangerVersion", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk metrics volume type (not in). Optional.
    #[serde(rename = "agentDiskMetricsVolumeType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The domain of the device (not in). Optional.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// The last logged in user. Optional.
    #[serde(rename = "agentLastLoggedInUser__contains", skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Kubernetes Cluster ID. Optional.
    #[serde(rename = "k8sClusterId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id_contains: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values).
    /// Optional.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version. Optional.
    #[serde(rename = "agentAgentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__gte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent version. Optional.
    #[serde(rename = "agentAgentVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_contains: Option<String>,
    /// The domain of the device. Optional.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes.
    /// Optional.
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The agent disk encryption. Optional.
    #[serde(rename = "agentDiskEncryption", skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The status alerts of the asset (not in). Allowed values:
    /// `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl AutoCompleteQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(v));
        self
    }
    /// The agent operational state (not in).
    pub fn agent_operational_state_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state_nin = Some(join_csv(v));
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
    /// Legacy Identity Policy Name.
    pub fn legacy_identity_policy_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name = Some(join_csv(v));
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
    /// The agent console migration status (not in).
    pub fn agent_console_migration_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status_nin = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_disk_metrics_free_percentage_between = Some(v.into());
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
    /// The CPU.
    pub fn cpu_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_contains = Some(join_csv(v));
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
    /// The agent VSS protection status (not in).
    pub fn agent_vss_protection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status_nin = Some(join_csv(v));
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
    /// The risk factors associated with the asset (not in).
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(join_csv(v));
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
    /// AD machine or its groups.
    pub fn identity_ad_machine_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_contains = Some(join_csv(v));
        self
    }
    /// Is AD Connector.
    pub fn is_ad_connector<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(join_csv(v));
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
    /// The agent missing permissions.
    pub fn agent_missing_permissions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions = Some(join_csv(v));
        self
    }
    /// The agent location (not in).
    pub fn agent_location_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_nin = Some(join_csv(v));
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
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// The SDL connectivity last active.
    pub fn agent_dv_connectivity_last_updated_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_dv_connectivity_last_updated_dt_between = Some(v.into());
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
    /// The ranger tags key.
    pub fn ranger_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key = Some(join_csv(v));
        self
    }
    /// The network name (not in).
    pub fn network_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name_nin = Some(join_csv(v));
        self
    }
    /// Search field key.
    pub fn key(mut self, v: impl Into<String>) -> Self {
        self.key = Some(v.into());
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
    /// The connection status between the agent and the SDL service.
    pub fn agent_dv_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity = Some(join_csv(v));
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
    /// The agent console connectivity.
    pub fn agent_console_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_connectivity = Some(join_csv(v));
        self
    }
    /// The agent Idr connectivity.
    pub fn agent_idr_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_idr_connectivity = Some(join_csv(v));
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
    /// The network name.
    pub fn network_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name = Some(join_csv(v));
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
    /// Whether the agent can configure network quarantine (not in).
    pub fn agent_configurable_network_quarantine_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine_nin = Some(join_csv(v));
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
    /// The manufacturer of the device (not in).
    pub fn manufacturer_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_nin = Some(join_csv(v));
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
    /// The agent pending actions.
    pub fn agent_pending_actions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions = Some(join_csv(v));
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
    /// The operating system name and version of the device (not in).
    pub fn os_name_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_nin = Some(join_csv(v));
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
    /// The serial number.
    pub fn serial_number<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number = Some(join_csv(v));
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
    /// The last active date.
    pub fn last_active_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_active_dt_between = Some(v.into());
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
    /// The site from which the device was detected.
    pub fn detected_from_site<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site = Some(join_csv(v));
        self
    }
    /// Any AD string.
    pub fn identity_ad_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_contains = Some(join_csv(v));
        self
    }
    /// The agent installer type.
    pub fn agent_installer_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type = Some(join_csv(v));
        self
    }
    /// The agent disk metrics volume type.
    pub fn agent_disk_metrics_volume_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type = Some(join_csv(v));
        self
    }
    /// The legacy identity policy name.
    pub fn legacy_identity_policy_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name_contains = Some(join_csv(v));
        self
    }
    /// The agent supported or unknown state.
    pub fn epp_unsupported_unknown<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown = Some(join_csv(v));
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
    /// The agent health status (not in).
    pub fn agent_health_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status_nin = Some(join_csv(v));
        self
    }
    /// The agent network scanner version (not in).
    pub fn agent_ranger_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version_nin = Some(join_csv(v));
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
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(join_csv(v));
        self
    }
    /// Free-text filter by Ranger tag key value (supports multiple
    /// values).
    pub fn ranger_tag_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_value_contains = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_lte(mut self, n: f64) -> Self {
        self.agent_disk_metrics_free_percentage_lte = Some(n);
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
    /// The serial number (not in).
    pub fn serial_number_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent is decommissioned.
    pub fn agent_decommissioned<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_decommissioned = Some(join_csv(v));
        self
    }
    /// The agent subscribe time.
    pub fn agent_subscribe_on_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_subscribe_on_dt_between = Some(v.into());
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
    /// The OS names and versions.
    pub fn os_name_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_contains = Some(join_csv(v));
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
    /// The customer identifier.
    pub fn agent_customer_identifier_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_customer_identifier_contains = Some(join_csv(v));
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
    /// The manufacturer.
    pub fn manufacturer_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_contains = Some(join_csv(v));
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
    /// The UDP ports.
    pub fn udp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports = Some(join_csv(v));
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
    /// The agent full disk scan date.
    pub fn agent_full_disk_scan_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_full_disk_scan_dt_between = Some(v.into());
        self
    }
    /// Free-text filter by Kubernetes Labels key (supports multiple
    /// values).
    pub fn k8s_labels_unified_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_labels_unified_key_contains = Some(join_csv(v));
        self
    }
    /// The agent anti tampering status (not in).
    pub fn agent_anti_tampering_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status_nin = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS \.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(join_csv(v));
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
    /// AD user or their groups.
    pub fn identity_ad_user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_contains = Some(join_csv(v));
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
    /// ADS Enabled.
    pub fn ads_enabled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ads_enabled = Some(join_csv(v));
        self
    }
    /// The ranger tags key (not in).
    pub fn ranger_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_nin = Some(join_csv(v));
        self
    }
    /// The agent location.
    pub fn agent_location<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location = Some(join_csv(v));
        self
    }
    /// The agent pending actions (not in).
    pub fn agent_pending_actions_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions_nin = Some(join_csv(v));
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
    /// Whether the agent is uninstalled.
    pub fn agent_uninstalled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uninstalled = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple
    /// values).
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
    /// Free-text filter by Kubernetes Labels key value (supports
    /// multiple values).
    pub fn k8s_labels_unified_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_labels_unified_key_value_contains = Some(join_csv(v));
        self
    }
    /// The UUID.
    pub fn agent_uuid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid_contains = Some(join_csv(v));
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
    /// The criticality that each asset belongs to.
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(v));
        self
    }
    /// The ranger tags key value.
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(join_csv(v));
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
    /// The location awareness.
    pub fn agent_location_awareness_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_awareness_contains = Some(join_csv(v));
        self
    }
    /// Is DC Server.
    pub fn is_dc_server<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_dc_server = Some(join_csv(v));
        self
    }
    /// AD user DN.
    pub fn identity_ad_user_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_distinguished_name_contains = Some(join_csv(v));
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
    /// The agent operational state.
    pub fn agent_operational_state<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state = Some(join_csv(v));
        self
    }
    /// The agent network status.
    pub fn agent_network_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status = Some(join_csv(v));
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
    /// The agent VSS service status.
    pub fn agent_vss_service_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status = Some(join_csv(v));
        self
    }
    /// The agent network scanner status (not in).
    pub fn agent_ranger_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status_nin = Some(join_csv(v));
        self
    }
    /// The site from which the device was detected (not in).
    pub fn detected_from_site_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site_nin = Some(join_csv(v));
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
    /// The name of the application installed on a workstation or a
    /// server.
    pub fn application_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name = Some(join_csv(v));
        self
    }
    /// Free-text filter by Kubernetes Annotations key (supports
    /// multiple values).
    pub fn k8s_annotations_unified_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_annotations_unified_key_contains = Some(join_csv(v));
        self
    }
    /// The first seen date.
    pub fn first_seen_dt_between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt_between = Some(v.into());
        self
    }
    /// The last update date.
    pub fn last_update_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_update_dt_between = Some(v.into());
        self
    }
    /// Whether the agent is pending uninstall.
    pub fn agent_pending_uninstall<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_uninstall = Some(join_csv(v));
        self
    }
    /// The agent anti tampering status.
    pub fn agent_anti_tampering_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status = Some(join_csv(v));
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
    /// The agent version (not in).
    pub fn agent_agent_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_nin = Some(join_csv(v));
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
    /// The discovery methods (not in).
    pub fn discovery_methods_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods_nin = Some(join_csv(v));
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
    /// The Kubernetes Cluster ID.
    pub fn k8s_cluster_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_id = Some(join_csv(v));
        self
    }
    /// The agent VSS service status (not in).
    pub fn agent_vss_service_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status_nin = Some(join_csv(v));
        self
    }
    /// The agent VSS last snapshot date.
    pub fn agent_vss_last_snapshot_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_last_snapshot_dt_between = Some(v.into());
        self
    }
    /// Whether the agent has local configuration.
    pub fn agent_has_local_config<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_has_local_config = Some(join_csv(v));
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
    /// The agent missing permissions (not in).
    pub fn agent_missing_permissions_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent is pending upgrade.
    pub fn agent_pending_upgrade<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_upgrade = Some(join_csv(v));
        self
    }
    /// The manufacturer of the device.
    pub fn manufacturer<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer = Some(join_csv(v));
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
    /// The agent VSS rollback status.
    pub fn agent_vss_rollback_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status = Some(join_csv(v));
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
    /// The agent installer type (not in).
    pub fn agent_installer_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type_nin = Some(join_csv(v));
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
    /// The environment that the asset exists in - AWS \.
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
    /// Match by the agent UUID.
    pub fn agent_uuid<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid = Some(join_csv(v));
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
    /// The discovery methods.
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(join_csv(v));
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
    /// The TCP ports.
    pub fn tcp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports = Some(join_csv(v));
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
    /// Free-text filter by Ranger tag key (supports multiple values).
    pub fn ranger_tag_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_contains = Some(join_csv(v));
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
    /// The ranger tags key value (not in).
    pub fn ranger_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value_nin = Some(join_csv(v));
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
    /// AD machine DN.
    pub fn identity_ad_machine_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_distinguished_name_contains = Some(join_csv(v));
        self
    }
    /// Live update ID.
    pub fn agent_s1_agent_live_updates_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_s1_agent_live_updates_version_contains = Some(join_csv(v));
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
    /// The agent network status (not in).
    pub fn agent_network_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status_nin = Some(join_csv(v));
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
    /// The operating system name and version of the device.
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(join_csv(v));
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
    /// The agent network scanner status.
    pub fn agent_ranger_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status = Some(join_csv(v));
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
    /// The serial number.
    pub fn serial_number_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_contains = Some(join_csv(v));
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
    /// AD machine groups.
    pub fn identity_ad_machine_membership_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_membership_contains = Some(join_csv(v));
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
    /// Free-text filter by Kubernetes Annotations key value (supports
    /// multiple values).
    pub fn k8s_annotations_unified_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_annotations_unified_key_value_contains = Some(join_csv(v));
        self
    }
    /// The agent VSS rollback status (not in).
    pub fn agent_vss_rollback_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status_nin = Some(join_csv(v));
        self
    }
    /// The agent console migration status.
    pub fn agent_console_migration_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status = Some(join_csv(v));
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
    /// AD user groups.
    pub fn identity_ad_user_membership_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_membership_contains = Some(join_csv(v));
        self
    }
    /// The connection status between the agent and the SDL service (not
    /// in).
    pub fn agent_dv_connectivity_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity_nin = Some(join_csv(v));
        self
    }
    /// The agent VSS protection status.
    pub fn agent_vss_protection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status = Some(join_csv(v));
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
    /// The agent supported or unknown state (not in).
    pub fn epp_unsupported_unknown_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent can configure network quarantine.
    pub fn agent_configurable_network_quarantine<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine = Some(join_csv(v));
        self
    }
    /// The agent health status.
    pub fn agent_health_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status = Some(join_csv(v));
        self
    }
    /// The agent network scanner version.
    pub fn agent_ranger_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version = Some(join_csv(v));
        self
    }
    /// The agent disk metrics volume type (not in).
    pub fn agent_disk_metrics_volume_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type_nin = Some(join_csv(v));
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
    /// The last logged in user.
    pub fn agent_last_logged_in_user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_last_logged_in_user_contains = Some(join_csv(v));
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
    /// The agent version.
    pub fn agent_agent_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_gte(mut self, n: f64) -> Self {
        self.agent_disk_metrics_free_percentage_gte = Some(n);
        self
    }
    /// The agent version.
    pub fn agent_agent_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_contains = Some(join_csv(v));
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
    /// The agent free VSS volume percentage on any of the volumes.
    pub fn agent_vss_volumes_diff_area_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_volumes_diff_area_free_percentage_between = Some(v.into());
        self
    }
    /// The agent disk encryption.
    pub fn agent_disk_encryption<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_encryption = Some(join_csv(v));
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
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(join_csv(v));
        self
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/server/filters/count`.
///
/// All array params are serialized comma-joined, as the API expects. Every
/// param is optional. This is the same filter surface as [`AutoCompleteQuery`],
/// minus the required `text`/`key`/`limit` fields.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterCountsQuery {
    /// Free-text filter by tag key (supports multiple values).
    /// Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The agent operational state (not in). Optional.
    #[serde(rename = "agentOperationalState__nin", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state_nin: Option<String>,
    /// The criticality that each asset belongs to (not in). Allowed
    /// values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// Legacy Identity Policy Name. Optional.
    #[serde(rename = "legacyIdentityPolicyName", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The state of the instance (not in). Optional.
    #[serde(rename = "state__nin", skip_serializing_if = "Option::is_none")]
    pub state_nin: Option<String>,
    /// The missing coverage for the asset. Allowed values: `CWS`,
    /// `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data
    /// Classification`, `CNS KSPM`. Optional.
    #[serde(rename = "missingCoverage", skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(rename = "allTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The agent console migration status (not in). Optional.
    #[serde(rename = "agentConsoleMigrationStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status_nin: Option<String>,
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
    /// The agent free disk percentage on any of the disks. Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CPU. Optional.
    #[serde(rename = "cpu__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The region. Optional.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The memory of the device in human readable format. Optional.
    #[serde(rename = "memoryReadable", skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses. Optional.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values).
    /// Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Running on Nodes. Optional.
    #[serde(rename = "k8sRunningOnNodes", skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes: Option<String>,
    /// Tag Keys. Optional.
    #[serde(rename = "tagsKey", skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in). Optional.
    #[serde(rename = "agentVssProtectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The state of the instance. Optional.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The risk factors associated with the asset (not in). Allowed
    /// values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// AD machine or its groups. Optional.
    #[serde(rename = "identityAdMachine__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_contains: Option<String>,
    /// Is AD Connector. Optional.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The cloud provider account ID. Optional.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The agent missing permissions. Optional.
    #[serde(rename = "agentMissingPermissions", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent location (not in). Optional.
    #[serde(rename = "agentLocation__nin", skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// The operating system of the device. Optional.
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active. Optional.
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The subnets. Optional.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The ranger tags key. Optional.
    #[serde(rename = "rangerTagsKey", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in). Optional.
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name_nin: Option<String>,
    /// The instance role. Optional.
    #[serde(rename = "instanceRole__contains", skip_serializing_if = "Option::is_none")]
    pub instance_role_contains: Option<String>,
    /// The connection status between the agent and the SDL service.
    /// Optional.
    #[serde(rename = "agentDvConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in). Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data
    /// Classification`, `CNS KSPM`. Optional.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity. Optional.
    #[serde(rename = "agentConsoleConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity. Optional.
    #[serde(rename = "agentIdrConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The network name. Optional.
    #[serde(rename = "networkName", skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// The region (not in). Optional.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// Whether the agent can configure network quarantine (not in).
    /// Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs. Optional.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The manufacturer of the device (not in). Optional.
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer_nin: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to. Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    /// Optional.
    #[serde(rename = "surfaces", skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions. Optional.
    #[serde(rename = "agentPendingActions", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// Kubernetes Resource ID. Optional.
    #[serde(rename = "k8sResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id_contains: Option<String>,
    /// The operating system name and version of the device (not in).
    /// Optional.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in). Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data
    /// Classification`, `CNS KSPM`. Optional.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The serial number. Optional.
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in). Allowed values: `Active`,
    /// `Inactive`. Optional.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The last active date. Optional.
    #[serde(rename = "lastActiveDt__between", skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// The state. Optional.
    #[serde(rename = "state__contains", skip_serializing_if = "Option::is_none")]
    pub state_contains: Option<String>,
    /// The site from which the device was detected. Optional.
    #[serde(rename = "detectedFromSite", skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// Any AD string. Optional.
    #[serde(rename = "identityAd__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// The agent installer type. Optional.
    #[serde(rename = "agentInstallerType", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type. Optional.
    #[serde(rename = "agentDiskMetricsVolumeType", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name. Optional.
    #[serde(rename = "legacy_identity_policy_name__contains", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// The agent supported or unknown state. Optional.
    #[serde(rename = "eppUnsupportedUnknown", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The geographical area where cloud resources are hosted.
    /// Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The agent health status (not in). Optional.
    #[serde(rename = "agentHealthStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version (not in). Optional.
    #[serde(rename = "agentRangerVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version_nin: Option<String>,
    /// The asset review (not in). Allowed values: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    /// Optional.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(rename = "cloudProviderAccountName", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple
    /// values). Optional.
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value_contains: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__lte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The Kubernetes node. Optional.
    #[serde(rename = "k8sNode__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_contains: Option<String>,
    /// The severity of the alert. Optional.
    #[serde(rename = "alertSeverity", skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value. Optional.
    #[serde(rename = "cloudTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The Kubernetes Node (not in). Optional.
    #[serde(rename = "k8sNode__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_node_nin: Option<String>,
    /// The serial number (not in). Optional.
    #[serde(rename = "serialNumber__nin", skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// Whether the agent is decommissioned. Optional.
    #[serde(rename = "agentDecommissioned", skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time. Optional.
    #[serde(rename = "agentSubscribeOnDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The domain. Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The OS names and versions. Optional.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The canonical name for the resource type (not in). Allowed
    /// values: `Access Control and Surveillance System`, `Access
    /// Point`, `AD Certificate`, `AD Certificate Authority`, `AD
    /// Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD
    /// Domain`. Optional.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs. Optional.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The customer identifier. Optional.
    #[serde(rename = "agentCustomerIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier_contains: Option<String>,
    /// The operating system of the device (not in). Optional.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// The Kubernetes Cluster (not in). Optional.
    #[serde(rename = "k8sCluster__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_nin: Option<String>,
    /// Namespace Name. Optional.
    #[serde(rename = "k8sNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace_contains: Option<String>,
    /// The manufacturer. Optional.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// The Kubernetes Resource ID. Optional.
    #[serde(rename = "k8sResourceId", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(rename = "assetContactEmail", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports. Optional.
    #[serde(rename = "udpPorts", skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset. Allowed values:
    /// `Unresolved Alerts`, `High Value`. Optional.
    #[serde(rename = "riskFactors", skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date. Optional.
    #[serde(rename = "agentFullDiskScanDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// Free-text filter by Kubernetes Labels key (supports multiple
    /// values). Optional.
    #[serde(rename = "k8sLabelsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_contains: Option<String>,
    /// The agent anti tampering status (not in). Optional.
    #[serde(rename = "agentAntiTamperingStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The environment that the asset exists in - AWS \. Optional.
    #[serde(rename = "assetEnvironment", skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device. Optional.
    #[serde(rename = "osFamily", skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups. Optional.
    #[serde(rename = "identityAdUser__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_contains: Option<String>,
    /// The Surface that each asset belongs to (not in). Allowed values:
    /// `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    /// Optional.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// ADS Enabled. Optional.
    #[serde(rename = "adsEnabled", skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The ranger tags key (not in). Optional.
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_nin: Option<String>,
    /// The agent location. Optional.
    #[serde(rename = "agentLocation", skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in). Optional.
    #[serde(rename = "agentPendingActions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions_nin: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Whether the agent is uninstalled. Optional.
    #[serde(rename = "agentUninstalled", skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple
    /// values). Optional.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in).
    /// Optional.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// Free-text filter by Kubernetes Labels key value (supports
    /// multiple values). Optional.
    #[serde(rename = "k8sLabelsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_value_contains: Option<String>,
    /// The UUID. Optional.
    #[serde(rename = "agentUuid__contains", skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// Tags. Optional.
    #[serde(rename = "tagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The Kubernetes cluster. Optional.
    #[serde(rename = "k8sCluster__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_contains: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The criticality that each asset belongs to. Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(rename = "assetCriticality", skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The ranger tags key value. Optional.
    #[serde(rename = "rangerTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// The location awareness. Optional.
    #[serde(rename = "agentLocationAwareness__contains", skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness_contains: Option<String>,
    /// Is DC Server. Optional.
    #[serde(rename = "isDcServer", skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// AD user DN. Optional.
    #[serde(rename = "identityAdUserDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// The operating system family of the device (not in). Optional.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The agent operational state. Optional.
    #[serde(rename = "agentOperationalState", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status. Optional.
    #[serde(rename = "agentNetworkStatus", skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name. Optional.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The Kubernetes Cluster. Optional.
    #[serde(rename = "k8sCluster", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster: Option<String>,
    /// The MAC addresses. Optional.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The instance type. Optional.
    #[serde(rename = "instanceType__contains", skip_serializing_if = "Option::is_none")]
    pub instance_type_contains: Option<String>,
    /// The agent VSS service status. Optional.
    #[serde(rename = "agentVssServiceStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The agent network scanner status (not in). Optional.
    #[serde(rename = "agentRangerStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status_nin: Option<String>,
    /// The site from which the device was detected (not in). Optional.
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site_nin: Option<String>,
    /// The name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The name of the application installed on a workstation or a
    /// server. Optional.
    #[serde(rename = "applicationName", skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// Free-text filter by Kubernetes Annotations key (supports
    /// multiple values). Optional.
    #[serde(rename = "k8sAnnotationsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_contains: Option<String>,
    /// The first seen date. Optional.
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The last update date. Optional.
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt_between: Option<String>,
    /// Whether the agent is pending uninstall. Optional.
    #[serde(rename = "agentPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The agent anti tampering status. Optional.
    #[serde(rename = "agentAntiTamperingStatus", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The Kubernetes Node. Optional.
    #[serde(rename = "k8sNode", skip_serializing_if = "Option::is_none")]
    pub k8s_node: Option<String>,
    /// The agent version (not in). Optional.
    #[serde(rename = "agentAgentVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_nin: Option<String>,
    /// The operating system version of the device. Optional.
    #[serde(rename = "osVersion", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The discovery methods (not in). Optional.
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods_nin: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// The Kubernetes Cluster ID. Optional.
    #[serde(rename = "k8sClusterId", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id: Option<String>,
    /// The agent VSS service status (not in). Optional.
    #[serde(rename = "agentVssServiceStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS last snapshot date. Optional.
    #[serde(rename = "agentVssLastSnapshotDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// Whether the agent has local configuration. Optional.
    #[serde(rename = "agentHasLocalConfig", skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The agent missing permissions (not in). Optional.
    #[serde(rename = "agentMissingPermissions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions_nin: Option<String>,
    /// Whether the agent is pending upgrade. Optional.
    #[serde(rename = "agentPendingUpgrade", skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The manufacturer of the device. Optional.
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The virtual network ID. Optional.
    #[serde(rename = "virtualNetworkId__contains", skip_serializing_if = "Option::is_none")]
    pub virtual_network_id_contains: Option<String>,
    /// The instance ID. Optional.
    #[serde(rename = "instanceId__contains", skip_serializing_if = "Option::is_none")]
    pub instance_id_contains: Option<String>,
    /// The agent VSS rollback status. Optional.
    #[serde(rename = "agentVssRollbackStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in). Optional.
    #[serde(rename = "agentInstallerType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type_nin: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Allowed values:
    /// `Access Control and Surveillance System`, `Access Point`, `AD
    /// Certificate`, `AD Certificate Authority`, `AD Certificate
    /// Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    /// Optional.
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS \. Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// The Kubernetes Version (not in). Optional.
    #[serde(rename = "k8sVersion__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_version_nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Match by the agent UUID. Optional.
    #[serde(rename = "agentUuid", skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(rename = "cloudTagsKey", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The number of cores. Optional.
    #[serde(rename = "coreCount", skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in). Optional.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in). Allowed
    /// values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`. Optional.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The Kubernetes Version. Optional.
    #[serde(rename = "k8sVersion", skip_serializing_if = "Option::is_none")]
    pub k8s_version: Option<String>,
    /// The discovery methods. Optional.
    #[serde(rename = "discoveryMethods", skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset. Allowed values: `Infected`,
    /// `Healthy`. Optional.
    #[serde(rename = "infectionStatus", skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports. Optional.
    #[serde(rename = "tcpPorts", skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset. Allowed values: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`. Optional.
    #[serde(rename = "activeCoverage", skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for.
    /// Optional.
    #[serde(rename = "countsFor", skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in). Optional.
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by. Optional.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values).
    /// Optional.
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_contains: Option<String>,
    /// The network security group. Optional.
    #[serde(rename = "networkSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub network_security_groups_contains: Option<String>,
    /// The ranger tags key value (not in). Optional.
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value_nin: Option<String>,
    /// The architecture of the device (not in). Optional.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// The Kubernetes Namespace Name. Optional.
    #[serde(rename = "k8sNamespace", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace: Option<String>,
    /// AD machine DN. Optional.
    #[serde(rename = "identityAdMachineDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// Live update ID. Optional.
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The Kubernetes Type. Optional.
    #[serde(rename = "k8sType", skip_serializing_if = "Option::is_none")]
    pub k8s_type: Option<String>,
    /// The cloud provider account id. Optional.
    #[serde(rename = "cloudProviderAccountId", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The agent network status (not in). Optional.
    #[serde(rename = "agentNetworkStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_network_status_nin: Option<String>,
    /// Whether the instance is a rogue or not. Optional.
    #[serde(rename = "isRogues", skip_serializing_if = "Option::is_none")]
    pub is_rogues: Option<String>,
    /// The sub-category that each resource belongs to. Allowed values:
    /// `All`, `Access Key and Secret`, `Access Management`, `Account`,
    /// `Account Group`, `AD Objects`, `Administrative Unit`, `Admission
    /// Controller`. Optional.
    #[serde(rename = "subCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The operating system name and version of the device. Optional.
    #[serde(rename = "osNameVersion", skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
    /// The image ID. Optional.
    #[serde(rename = "imageId__contains", skip_serializing_if = "Option::is_none")]
    pub image_id_contains: Option<String>,
    /// The agent network scanner status. Optional.
    #[serde(rename = "agentRangerStatus", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions. Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review. Allowed values: `Not Reviewed`, `Under
    /// Analysis`, `Not Trusted`, `Allowed`, `` (empty). Optional.
    #[serde(rename = "deviceReview", skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(rename = "allTagsKey", skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number. Optional.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number_contains: Option<String>,
    /// The hostnames. Optional.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// AD machine groups. Optional.
    #[serde(rename = "identityAdMachineMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// The Kubernetes Type (not in). Optional.
    #[serde(rename = "k8sType__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_type_nin: Option<String>,
    /// Free-text filter by Kubernetes Annotations key value (supports
    /// multiple values). Optional.
    #[serde(rename = "k8sAnnotationsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_value_contains: Option<String>,
    /// The agent VSS rollback status (not in). Optional.
    #[serde(rename = "agentVssRollbackStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent console migration status. Optional.
    #[serde(rename = "agentConsoleMigrationStatus", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device. Optional.
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups. Optional.
    #[serde(rename = "identityAdUserMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership_contains: Option<String>,
    /// The connection status between the agent and the SDL service (not
    /// in). Optional.
    #[serde(rename = "agentDvConnectivity__nin", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent VSS protection status. Optional.
    #[serde(rename = "agentVssProtectionStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
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
    /// The status of the asset. Allowed values: `Active`, `Inactive`.
    /// Optional.
    #[serde(rename = "assetStatus", skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in). Optional.
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown_nin: Option<String>,
    /// Whether the agent can configure network quarantine. Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status. Optional.
    #[serde(rename = "agentHealthStatus", skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version. Optional.
    #[serde(rename = "agentRangerVersion", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk metrics volume type (not in). Optional.
    #[serde(rename = "agentDiskMetricsVolumeType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The domain of the device (not in). Optional.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// The last logged in user. Optional.
    #[serde(rename = "agentLastLoggedInUser__contains", skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Kubernetes Cluster ID. Optional.
    #[serde(rename = "k8sClusterId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id_contains: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values).
    /// Optional.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version. Optional.
    #[serde(rename = "agentAgentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__gte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent version. Optional.
    #[serde(rename = "agentAgentVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_contains: Option<String>,
    /// The domain of the device. Optional.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes.
    /// Optional.
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The agent disk encryption. Optional.
    #[serde(rename = "agentDiskEncryption", skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The status alerts of the asset (not in). Allowed values:
    /// `Infected`, `Healthy`. Optional.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl FilterCountsQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(v));
        self
    }
    /// The agent operational state (not in).
    pub fn agent_operational_state_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state_nin = Some(join_csv(v));
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
    /// Legacy Identity Policy Name.
    pub fn legacy_identity_policy_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name = Some(join_csv(v));
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
    /// The agent console migration status (not in).
    pub fn agent_console_migration_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status_nin = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_disk_metrics_free_percentage_between = Some(v.into());
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
    /// The CPU.
    pub fn cpu_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_contains = Some(join_csv(v));
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
    /// The agent VSS protection status (not in).
    pub fn agent_vss_protection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status_nin = Some(join_csv(v));
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
    /// The risk factors associated with the asset (not in).
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(join_csv(v));
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
    /// AD machine or its groups.
    pub fn identity_ad_machine_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_contains = Some(join_csv(v));
        self
    }
    /// Is AD Connector.
    pub fn is_ad_connector<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(join_csv(v));
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
    /// The agent missing permissions.
    pub fn agent_missing_permissions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions = Some(join_csv(v));
        self
    }
    /// The agent location (not in).
    pub fn agent_location_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_nin = Some(join_csv(v));
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
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// The SDL connectivity last active.
    pub fn agent_dv_connectivity_last_updated_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_dv_connectivity_last_updated_dt_between = Some(v.into());
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
    /// The ranger tags key.
    pub fn ranger_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key = Some(join_csv(v));
        self
    }
    /// The network name (not in).
    pub fn network_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name_nin = Some(join_csv(v));
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
    /// The connection status between the agent and the SDL service.
    pub fn agent_dv_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity = Some(join_csv(v));
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
    /// The agent console connectivity.
    pub fn agent_console_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_connectivity = Some(join_csv(v));
        self
    }
    /// The agent Idr connectivity.
    pub fn agent_idr_connectivity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_idr_connectivity = Some(join_csv(v));
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
    /// The network name.
    pub fn network_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name = Some(join_csv(v));
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
    /// Whether the agent can configure network quarantine (not in).
    pub fn agent_configurable_network_quarantine_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine_nin = Some(join_csv(v));
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
    /// The manufacturer of the device (not in).
    pub fn manufacturer_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_nin = Some(join_csv(v));
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
    /// The agent pending actions.
    pub fn agent_pending_actions<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions = Some(join_csv(v));
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
    /// The operating system name and version of the device (not in).
    pub fn os_name_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_nin = Some(join_csv(v));
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
    /// The serial number.
    pub fn serial_number<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number = Some(join_csv(v));
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
    /// The last active date.
    pub fn last_active_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_active_dt_between = Some(v.into());
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
    /// The site from which the device was detected.
    pub fn detected_from_site<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site = Some(join_csv(v));
        self
    }
    /// Any AD string.
    pub fn identity_ad_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_contains = Some(join_csv(v));
        self
    }
    /// The agent installer type.
    pub fn agent_installer_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type = Some(join_csv(v));
        self
    }
    /// The agent disk metrics volume type.
    pub fn agent_disk_metrics_volume_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type = Some(join_csv(v));
        self
    }
    /// The legacy identity policy name.
    pub fn legacy_identity_policy_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name_contains = Some(join_csv(v));
        self
    }
    /// The agent supported or unknown state.
    pub fn epp_unsupported_unknown<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown = Some(join_csv(v));
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
    /// The agent health status (not in).
    pub fn agent_health_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status_nin = Some(join_csv(v));
        self
    }
    /// The agent network scanner version (not in).
    pub fn agent_ranger_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version_nin = Some(join_csv(v));
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
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(join_csv(v));
        self
    }
    /// Free-text filter by Ranger tag key value (supports multiple
    /// values).
    pub fn ranger_tag_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_value_contains = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_lte(mut self, n: f64) -> Self {
        self.agent_disk_metrics_free_percentage_lte = Some(n);
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
    /// The serial number (not in).
    pub fn serial_number_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent is decommissioned.
    pub fn agent_decommissioned<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_decommissioned = Some(join_csv(v));
        self
    }
    /// The agent subscribe time.
    pub fn agent_subscribe_on_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_subscribe_on_dt_between = Some(v.into());
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
    /// The OS names and versions.
    pub fn os_name_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version_contains = Some(join_csv(v));
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
    /// The customer identifier.
    pub fn agent_customer_identifier_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_customer_identifier_contains = Some(join_csv(v));
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
    /// The manufacturer.
    pub fn manufacturer_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_contains = Some(join_csv(v));
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
    /// The UDP ports.
    pub fn udp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports = Some(join_csv(v));
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
    /// The agent full disk scan date.
    pub fn agent_full_disk_scan_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_full_disk_scan_dt_between = Some(v.into());
        self
    }
    /// Free-text filter by Kubernetes Labels key (supports multiple
    /// values).
    pub fn k8s_labels_unified_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_labels_unified_key_contains = Some(join_csv(v));
        self
    }
    /// The agent anti tampering status (not in).
    pub fn agent_anti_tampering_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status_nin = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS \.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(join_csv(v));
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
    /// AD user or their groups.
    pub fn identity_ad_user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_contains = Some(join_csv(v));
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
    /// ADS Enabled.
    pub fn ads_enabled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ads_enabled = Some(join_csv(v));
        self
    }
    /// The ranger tags key (not in).
    pub fn ranger_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_nin = Some(join_csv(v));
        self
    }
    /// The agent location.
    pub fn agent_location<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location = Some(join_csv(v));
        self
    }
    /// The agent pending actions (not in).
    pub fn agent_pending_actions_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions_nin = Some(join_csv(v));
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
    /// Whether the agent is uninstalled.
    pub fn agent_uninstalled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uninstalled = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple
    /// values).
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
    /// Free-text filter by Kubernetes Labels key value (supports
    /// multiple values).
    pub fn k8s_labels_unified_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_labels_unified_key_value_contains = Some(join_csv(v));
        self
    }
    /// The UUID.
    pub fn agent_uuid_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid_contains = Some(join_csv(v));
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
    /// The criticality that each asset belongs to.
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(v));
        self
    }
    /// The ranger tags key value.
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(join_csv(v));
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
    /// The location awareness.
    pub fn agent_location_awareness_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_awareness_contains = Some(join_csv(v));
        self
    }
    /// Is DC Server.
    pub fn is_dc_server<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_dc_server = Some(join_csv(v));
        self
    }
    /// AD user DN.
    pub fn identity_ad_user_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_distinguished_name_contains = Some(join_csv(v));
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
    /// The agent operational state.
    pub fn agent_operational_state<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state = Some(join_csv(v));
        self
    }
    /// The agent network status.
    pub fn agent_network_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status = Some(join_csv(v));
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
    /// The agent VSS service status.
    pub fn agent_vss_service_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status = Some(join_csv(v));
        self
    }
    /// The agent network scanner status (not in).
    pub fn agent_ranger_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status_nin = Some(join_csv(v));
        self
    }
    /// The site from which the device was detected (not in).
    pub fn detected_from_site_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site_nin = Some(join_csv(v));
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
    /// The name of the application installed on a workstation or a
    /// server.
    pub fn application_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name = Some(join_csv(v));
        self
    }
    /// Free-text filter by Kubernetes Annotations key (supports
    /// multiple values).
    pub fn k8s_annotations_unified_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_annotations_unified_key_contains = Some(join_csv(v));
        self
    }
    /// The first seen date.
    pub fn first_seen_dt_between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt_between = Some(v.into());
        self
    }
    /// The last update date.
    pub fn last_update_dt_between(mut self, v: impl Into<String>) -> Self {
        self.last_update_dt_between = Some(v.into());
        self
    }
    /// Whether the agent is pending uninstall.
    pub fn agent_pending_uninstall<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_uninstall = Some(join_csv(v));
        self
    }
    /// The agent anti tampering status.
    pub fn agent_anti_tampering_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status = Some(join_csv(v));
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
    /// The agent version (not in).
    pub fn agent_agent_version_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_nin = Some(join_csv(v));
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
    /// The discovery methods (not in).
    pub fn discovery_methods_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods_nin = Some(join_csv(v));
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
    /// The Kubernetes Cluster ID.
    pub fn k8s_cluster_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_id = Some(join_csv(v));
        self
    }
    /// The agent VSS service status (not in).
    pub fn agent_vss_service_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status_nin = Some(join_csv(v));
        self
    }
    /// The agent VSS last snapshot date.
    pub fn agent_vss_last_snapshot_dt_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_last_snapshot_dt_between = Some(v.into());
        self
    }
    /// Whether the agent has local configuration.
    pub fn agent_has_local_config<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_has_local_config = Some(join_csv(v));
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
    /// The agent missing permissions (not in).
    pub fn agent_missing_permissions_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent is pending upgrade.
    pub fn agent_pending_upgrade<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_upgrade = Some(join_csv(v));
        self
    }
    /// The manufacturer of the device.
    pub fn manufacturer<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer = Some(join_csv(v));
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
    /// The agent VSS rollback status.
    pub fn agent_vss_rollback_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status = Some(join_csv(v));
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
    /// The agent installer type (not in).
    pub fn agent_installer_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type_nin = Some(join_csv(v));
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
    /// The environment that the asset exists in - AWS \.
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
    /// Match by the agent UUID.
    pub fn agent_uuid<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid = Some(join_csv(v));
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
    /// The discovery methods.
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(join_csv(v));
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
    /// The TCP ports.
    pub fn tcp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports = Some(join_csv(v));
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
    /// Free-text filter by Ranger tag key (supports multiple values).
    pub fn ranger_tag_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_contains = Some(join_csv(v));
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
    /// The ranger tags key value (not in).
    pub fn ranger_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value_nin = Some(join_csv(v));
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
    /// AD machine DN.
    pub fn identity_ad_machine_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_distinguished_name_contains = Some(join_csv(v));
        self
    }
    /// Live update ID.
    pub fn agent_s1_agent_live_updates_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_s1_agent_live_updates_version_contains = Some(join_csv(v));
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
    /// The agent network status (not in).
    pub fn agent_network_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status_nin = Some(join_csv(v));
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
    /// The operating system name and version of the device.
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(join_csv(v));
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
    /// The agent network scanner status.
    pub fn agent_ranger_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status = Some(join_csv(v));
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
    /// The serial number.
    pub fn serial_number_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number_contains = Some(join_csv(v));
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
    /// AD machine groups.
    pub fn identity_ad_machine_membership_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_membership_contains = Some(join_csv(v));
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
    /// Free-text filter by Kubernetes Annotations key value (supports
    /// multiple values).
    pub fn k8s_annotations_unified_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_annotations_unified_key_value_contains = Some(join_csv(v));
        self
    }
    /// The agent VSS rollback status (not in).
    pub fn agent_vss_rollback_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status_nin = Some(join_csv(v));
        self
    }
    /// The agent console migration status.
    pub fn agent_console_migration_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status = Some(join_csv(v));
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
    /// AD user groups.
    pub fn identity_ad_user_membership_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_membership_contains = Some(join_csv(v));
        self
    }
    /// The connection status between the agent and the SDL service (not
    /// in).
    pub fn agent_dv_connectivity_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity_nin = Some(join_csv(v));
        self
    }
    /// The agent VSS protection status.
    pub fn agent_vss_protection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status = Some(join_csv(v));
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
    /// The agent supported or unknown state (not in).
    pub fn epp_unsupported_unknown_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown_nin = Some(join_csv(v));
        self
    }
    /// Whether the agent can configure network quarantine.
    pub fn agent_configurable_network_quarantine<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine = Some(join_csv(v));
        self
    }
    /// The agent health status.
    pub fn agent_health_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status = Some(join_csv(v));
        self
    }
    /// The agent network scanner version.
    pub fn agent_ranger_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version = Some(join_csv(v));
        self
    }
    /// The agent disk metrics volume type (not in).
    pub fn agent_disk_metrics_volume_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type_nin = Some(join_csv(v));
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
    /// The last logged in user.
    pub fn agent_last_logged_in_user_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_last_logged_in_user_contains = Some(join_csv(v));
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
    /// The agent version.
    pub fn agent_agent_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version = Some(join_csv(v));
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
    /// The agent free disk percentage on any of the disks.
    pub fn agent_disk_metrics_free_percentage_gte(mut self, n: f64) -> Self {
        self.agent_disk_metrics_free_percentage_gte = Some(n);
        self
    }
    /// The agent version.
    pub fn agent_agent_version_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version_contains = Some(join_csv(v));
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
    /// The agent free VSS volume percentage on any of the volumes.
    pub fn agent_vss_volumes_diff_area_free_percentage_between(mut self, v: impl Into<String>) -> Self {
        self.agent_vss_volumes_diff_area_free_percentage_between = Some(v.into());
        self
    }
    /// The agent disk encryption.
    pub fn agent_disk_encryption<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_encryption = Some(join_csv(v));
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
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(join_csv(v));
        self
    }
}

/// Comma-join an iterator of string-likes into a single CSV query value.
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

impl InventoryServerFiltersService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/server/filters/autocomplete` — Auto Complete.
    ///
    /// Use this command to get values for other fields. When you send this
    /// command with input text and a field name, it returns auto-complete
    /// suggestions for the field.
    ///
    /// `text` and `key` are required by the API; set them via the
    /// [`AutoCompleteQuery`] builder before calling.
    pub async fn autocomplete(
        &self,
        query: &AutoCompleteQuery,
    ) -> Result<Response<AutoCompleteResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/server/filters/autocomplete", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/server/filters/count` — Filter counts.
    ///
    /// Get filter counts.
    pub async fn count(
        &self,
        query: &FilterCountsQuery,
    ) -> Result<Response<Vec<CountFiltersResponse>>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/server/filters/count", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/server/filters/free-text` — Free text filters.
    ///
    /// Get free text filters.
    pub async fn free_text(&self) -> Result<Response<Vec<FreeTextFilterResponse>>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/server/filters/free-text", None)
            .await?)
    }
}

