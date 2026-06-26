//! Service for the `Inventory Server` tag — XDR server-asset inventory.
//!
//! Hand-written for 1:1 parity with `swagger_2_1.json` / `docs/inventory-server.md`.
//! Covers all 5 endpoints under `/web/api/v2.1/xdr/assets/server`.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_server::{AvailableActionWithStatus, Server};
use crate::pagination::{Paginated, Response};

/// `Inventory Server` tag — query, export and act on XDR server assets.
pub struct InventoryServerService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query parameters for the server-asset filter endpoints.
///
/// Covers the full filter parameter set of `GET /web/api/v2.1/xdr/assets/server`
/// (including pagination/sort controls `skip`, `limit`, `cursor`, `sortBy`,
/// `sortOrder`, `countOnly`, `skipCount`). The same struct is reused for
/// `POST .../server/action`, `POST .../server/available-actions/with-status` and
/// `GET .../server/export`, which accept the same filter parameters; for the
/// `POST` action endpoints simply leave the pagination/sort fields unset.
///
/// Every field is optional. Array parameters are serialized comma-joined, and
/// each field carries an explicit `#[serde(rename)]` to preserve the exact API
/// wire name (most use `__`-suffixed operators that camelCase cannot reproduce).
#[derive(Debug, Default, Serialize)]
pub struct ServerAssetsQuery {
    /// Free-text filter by tag key (supports multiple values)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key__contains: Option<String>,
    /// The agent operational state (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentOperationalState__nin", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state__nin: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality__nin: Option<String>,
    /// Legacy Identity Policy Name
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "legacyIdentityPolicyName", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The state of the instance (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "state__nin", skip_serializing_if = "Option::is_none")]
    pub state__nin: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "missingCoverage", skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "allTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name__nin: Option<String>,
    /// The agent console migration status (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentConsoleMigrationStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status__nin: Option<String>,
    /// Running on nodes
    ///
    /// Array of string lists; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sRunningOnNodes__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes__contains: Option<String>,
    /// Tag Keys (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key__nin: Option<String>,
    /// The cloud resource ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id__contains: Option<String>,
    /// The cloud provider subscription ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id__contains: Option<String>,
    /// The agent free disk percentage on any of the disks
    ///
    /// Required.
    #[serde(rename = "agentDiskMetricsFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage__between: Option<String>,
    /// Tag Keys exists
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key__exists: Option<String>,
    /// The CPU
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cpu__contains", skip_serializing_if = "Option::is_none")]
    pub cpu__contains: Option<String>,
    /// The region
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The memory of the device in human readable format
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "memoryReadable", skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address__contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value__contains: Option<String>,
    /// The cloud tags key (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key__nin: Option<String>,
    /// Running on Nodes
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sRunningOnNodes", skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes: Option<String>,
    /// Tag Keys
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "tagsKey", skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentVssProtectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status__nin: Option<String>,
    /// The state of the instance
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Unresolved Alerts`, `High Value`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key__nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor".
    ///
    /// Required.
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// AD machine or its groups
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "identityAdMachine__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine__contains: Option<String>,
    /// Is AD Connector
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The cloud provider account ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id__contains: Option<String>,
    /// Free-text filter by the image name
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name__contains: Option<String>,
    /// The agent missing permissions
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentMissingPermissions", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent location (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentLocation__nin", skip_serializing_if = "Option::is_none")]
    pub agent_location__nin: Option<String>,
    /// The operating system of the device
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active
    ///
    /// Required.
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt__between: Option<String>,
    /// The subnets
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets__contains: Option<String>,
    /// The ranger tags key
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "rangerTagsKey", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name__nin: Option<String>,
    /// The instance role
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "instanceRole__contains", skip_serializing_if = "Option::is_none")]
    pub instance_role__contains: Option<String>,
    /// The connection status between the agent and the SDL service
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentDvConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage__nin: Option<String>,
    /// The agent console connectivity
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentConsoleConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentIdrConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value__nin: Option<String>,
    /// The network name
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "networkName", skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// The region (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region__nin: Option<String>,
    /// Whether the agent can configure network quarantine (not in)
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine__nin: Option<String>,
    /// User and cloud tag keys (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nin: Option<String>,
    /// The gateway IPs
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips__contains: Option<String>,
    /// The manufacturer of the device (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer__nin: Option<String>,
    /// The cloud tags key value (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value__nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "surfaces", skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentPendingActions", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// Kubernetes Resource ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id__contains: Option<String>,
    /// The operating system name and version of the device (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version__nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage__nin: Option<String>,
    /// The serial number
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Active`, `Inactive`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status__nin: Option<String>,
    /// The last active date
    ///
    /// Required.
    #[serde(rename = "lastActiveDt__between", skip_serializing_if = "Option::is_none")]
    pub last_active_dt__between: Option<String>,
    /// The state
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "state__contains", skip_serializing_if = "Option::is_none")]
    pub state__contains: Option<String>,
    /// The column to sort the results by.
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `s1GroupName`, `cpu`, `legacyIdentityPolicyName`, `previousOsType`, `previousOsVersion`, `agentFirewallStatus`, `s1UpdatedAt`, `cnsVpcAttributeExists`.
    ///
    /// Required.
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The site from which the device was detected
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "detectedFromSite", skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// Any AD string
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "identityAd__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad__contains: Option<String>,
    /// The agent installer type
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentInstallerType", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentDiskMetricsVolumeType", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "legacy_identity_policy_name__contains", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name__contains: Option<String>,
    /// The agent supported or unknown state
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "eppUnsupportedUnknown", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The geographical area where cloud resources are hosted
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region__contains: Option<String>,
    /// The agent health status (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentHealthStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_health_status__nin: Option<String>,
    /// The agent network scanner version (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentRangerVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version__nin: Option<String>,
    /// The asset review (not in)
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `<empty>`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review__nin: Option<String>,
    /// The cloud provider account name
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudProviderAccountName", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value__contains: Option<String>,
    /// Asset Contact Email (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email__nin: Option<String>,
    /// The agent free disk percentage on any of the disks
    ///
    /// Required.
    #[serde(rename = "agentDiskMetricsFreePercentage__lte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage__lte: Option<f64>,
    /// The Kubernetes node
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sNode__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node__contains: Option<String>,
    /// The severity of the alert
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "alertSeverity", skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The Kubernetes Node (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sNode__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_node__nin: Option<String>,
    /// The serial number (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "serialNumber__nin", skip_serializing_if = "Option::is_none")]
    pub serial_number__nin: Option<String>,
    /// Whether the agent is decommissioned
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentDecommissioned", skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time
    ///
    /// Required.
    #[serde(rename = "agentSubscribeOnDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt__between: Option<String>,
    /// The domain
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain__contains: Option<String>,
    /// The OS names and versions
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version__contains: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type__nin: Option<String>,
    /// The gateway MACs
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs__contains: Option<String>,
    /// The customer identifier
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentCustomerIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier__contains: Option<String>,
    /// The operating system of the device (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os__nin: Option<String>,
    /// The Kubernetes Cluster (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sCluster__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster__nin: Option<String>,
    /// Namespace Name
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace__contains: Option<String>,
    /// The manufacturer
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer__contains: Option<String>,
    /// The Kubernetes Resource ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sResourceId", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id: Option<String>,
    /// Asset Contact Email
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "assetContactEmail", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "udpPorts", skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Unresolved Alerts`, `High Value`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "riskFactors", skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date
    ///
    /// Required.
    #[serde(rename = "agentFullDiskScanDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt__between: Option<String>,
    /// Free-text filter by Kubernetes Labels key (supports multiple values)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sLabelsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key__contains: Option<String>,
    /// The agent anti tampering status (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentAntiTamperingStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status__nin: Option<String>,
    /// The environment that the asset exists in - AWS \
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "assetEnvironment", skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "osFamily", skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "identityAdUser__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user__contains: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces__nin: Option<String>,
    /// ADS Enabled
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "adsEnabled", skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The ranger tags key (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key__nin: Option<String>,
    /// The agent location
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentLocation", skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentPendingActions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions__nin: Option<String>,
    /// The ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id__in: Option<String>,
    /// Whether the agent is uninstalled
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentUninstalled", skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value__contains: Option<String>,
    /// The Asset Type
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type__contains: Option<String>,
    /// The memory of the device in human readable format (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable__nin: Option<String>,
    /// Free-text filter by Kubernetes Labels key value (supports multiple values)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sLabelsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_value__contains: Option<String>,
    /// The UUID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentUuid__contains", skip_serializing_if = "Option::is_none")]
    pub agent_uuid__contains: Option<String>,
    /// Tags
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "tagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `asc`, `desc`.
    ///
    /// Required.
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The Kubernetes cluster
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sCluster__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster__contains: Option<String>,
    /// The cloud provider organization unit
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit__contains: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "assetCriticality", skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The ranger tags key value
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "rangerTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// The cloud provider organization
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization__contains: Option<String>,
    /// The location awareness
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentLocationAwareness__contains", skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness__contains: Option<String>,
    /// Is DC Server
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "isDcServer", skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects.
    ///
    /// Required.
    #[serde(rename = "countOnly", skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// AD user DN
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "identityAdUserDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name__contains: Option<String>,
    /// The operating system family of the device (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family__nin: Option<String>,
    /// The agent operational state
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentOperationalState", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentNetworkStatus", skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The Kubernetes Cluster
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sCluster", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster: Option<String>,
    /// The MAC addresses
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses__contains: Option<String>,
    /// The instance type
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "instanceType__contains", skip_serializing_if = "Option::is_none")]
    pub instance_type__contains: Option<String>,
    /// The agent VSS service status
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentVssServiceStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The agent network scanner status (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentRangerStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status__nin: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    ///
    /// Required.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The site from which the device was detected (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site__nin: Option<String>,
    /// The name
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
    /// The name of the application installed on a workstation or a server
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "applicationName", skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// Free-text filter by Kubernetes Annotations key (supports multiple values)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sAnnotationsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key__contains: Option<String>,
    /// The first seen date
    ///
    /// Required.
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt__between: Option<String>,
    /// The last update date
    ///
    /// Required.
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt__between: Option<String>,
    /// Whether the agent is pending uninstall
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The agent anti tampering status
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentAntiTamperingStatus", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The Kubernetes Node
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sNode", skip_serializing_if = "Option::is_none")]
    pub k8s_node: Option<String>,
    /// The agent version (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentAgentVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version__nin: Option<String>,
    /// The operating system version of the device
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "osVersion", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The discovery methods (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods__nin: Option<String>,
    /// The cloud provider account id (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id__nin: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time.
    ///
    /// Required.
    #[serde(rename = "skipCount", skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The Kubernetes Cluster ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sClusterId", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id: Option<String>,
    /// The agent VSS service status (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentVssServiceStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status__nin: Option<String>,
    /// The agent VSS last snapshot date
    ///
    /// Required.
    #[serde(rename = "agentVssLastSnapshotDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt__between: Option<String>,
    /// Whether the agent has local configuration
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentHasLocalConfig", skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nexists: Option<String>,
    /// The agent missing permissions (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentMissingPermissions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions__nin: Option<String>,
    /// Whether the agent is pending upgrade
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentPendingUpgrade", skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The manufacturer of the device
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The virtual network ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "virtualNetworkId__contains", skip_serializing_if = "Option::is_none")]
    pub virtual_network_id__contains: Option<String>,
    /// The instance ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "instanceId__contains", skip_serializing_if = "Option::is_none")]
    pub instance_id__contains: Option<String>,
    /// The agent VSS rollback status
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentVssRollbackStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentInstallerType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type__nin: Option<String>,
    /// The Last Seen date and time for the asset
    ///
    /// Required.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS \
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment__nin: Option<String>,
    /// The Kubernetes Version (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sVersion__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_version__nin: Option<String>,
    /// Name (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names__nin: Option<String>,
    /// Match by the agent UUID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentUuid", skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The cloud tags key
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudTagsKey", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The number of cores
    ///
    /// Array of integers; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "coreCount", skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version__nin: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category__nin: Option<String>,
    /// The Kubernetes Version
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sVersion", skip_serializing_if = "Option::is_none")]
    pub k8s_version: Option<String>,
    /// The discovery methods
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "discoveryMethods", skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Infected`, `Healthy`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "infectionStatus", skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "tcpPorts", skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "activeCoverage", skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "countsFor", skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in)
    ///
    /// Array of integers; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count__nin: Option<String>,
    /// The ID of the CSV file to filter by
    ///
    /// Required.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key__contains: Option<String>,
    /// The network security group
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "networkSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub network_security_groups__contains: Option<String>,
    /// The ranger tags key value (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value__nin: Option<String>,
    /// The architecture of the device (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture__nin: Option<String>,
    /// The Kubernetes Namespace Name
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sNamespace", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace: Option<String>,
    /// AD machine DN
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "identityAdMachineDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name__contains: Option<String>,
    /// Live update ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version__contains: Option<String>,
    /// The Kubernetes Type
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sType", skip_serializing_if = "Option::is_none")]
    pub k8s_type: Option<String>,
    /// The cloud provider account id
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudProviderAccountId", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The agent network status (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentNetworkStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_network_status__nin: Option<String>,
    /// Whether the instance is a rogue or not
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "isRogues", skip_serializing_if = "Option::is_none")]
    pub is_rogues: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "subCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The operating system name and version of the device
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "osNameVersion", skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
    /// The image ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "imageId__contains", skip_serializing_if = "Option::is_none")]
    pub image_id__contains: Option<String>,
    /// The agent network scanner status
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentRangerStatus", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// The asset review
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `<empty>`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "deviceReview", skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "allTagsKey", skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number__contains: Option<String>,
    /// The hostnames
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames__contains: Option<String>,
    /// AD machine groups
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "identityAdMachineMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership__contains: Option<String>,
    /// Limit number of returned items (1-1000)
    ///
    /// Required.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The Kubernetes Type (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sType__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_type__nin: Option<String>,
    /// Free-text filter by Kubernetes Annotations key value (supports multiple values)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sAnnotationsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_value__contains: Option<String>,
    /// The agent VSS rollback status (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentVssRollbackStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status__nin: Option<String>,
    /// The agent console migration status
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentConsoleMigrationStatus", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "identityAdUserMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership__contains: Option<String>,
    /// The connection status between the agent and the SDL service (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentDvConnectivity__nin", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity__nin: Option<String>,
    /// The agent VSS protection status
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentVssProtectionStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id__contains: Option<String>,
    /// The cloud provider account name
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name__contains: Option<String>,
    /// The internal IPs
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips__contains: Option<String>,
    /// The subnet ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "subnetId__contains", skip_serializing_if = "Option::is_none")]
    pub subnet_id__contains: Option<String>,
    /// The status of the asset
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Active`, `Inactive`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "assetStatus", skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown__nin: Option<String>,
    /// Whether the agent can configure network quarantine
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentConfigurableNetworkQuarantine", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentHealthStatus", skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentRangerVersion", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk metrics volume type (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentDiskMetricsVolumeType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type__nin: Option<String>,
    /// The domain of the device (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain__nin: Option<String>,
    /// The last logged in user
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentLastLoggedInUser__contains", skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user__contains: Option<String>,
    /// Kubernetes Cluster ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "k8sClusterId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id__contains: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key__contains: Option<String>,
    /// Tags (not in)
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value__nin: Option<String>,
    /// The agent version
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentAgentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__exists: Option<String>,
    /// The agent free disk percentage on any of the disks
    ///
    /// Required.
    #[serde(rename = "agentDiskMetricsFreePercentage__gte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage__gte: Option<f64>,
    /// The agent version
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentAgentVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version__contains: Option<String>,
    /// The domain of the device
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes
    ///
    /// Required.
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage__between: Option<String>,
    /// The agent disk encryption
    ///
    /// Array of booleans; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "agentDiskEncryption", skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Allowed values (enum, kept as `String` for forward-compat): `Infected`, `Healthy`.
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status__nin: Option<String>,
    /// The cloud provider project ID
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id__contains: Option<String>,
}

impl ServerAssetsQuery {
    pub fn tags_key__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_operational_state__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn asset_criticality__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn legacy_identity_policy_name<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn state__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn missing_coverage<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn all_tags_key_value<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_provider_account_name__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_console_migration_status__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_running_on_nodes__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_running_on_nodes__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn tags_key__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_resource_id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_provider_subscription_id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage__between(mut self, value: impl Into<String>) -> Self {
        self.agent_disk_metrics_free_percentage__between = Some(value.into());
        self
    }
    pub fn tags_key__exists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__exists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cpu__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn region<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn memory_readable<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn ip_address__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn tags_key_value__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_tags_key__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_running_on_nodes<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_running_on_nodes = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn tags_key<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_vss_protection_status__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn state<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn risk_factors__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn tags_key__nexists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nexists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn skip(mut self, value: i64) -> Self {
        self.skip = Some(value);
        self
    }
    pub fn identity_ad_machine__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn is_ad_connector<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_provider_account_id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn image_name__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_missing_permissions<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_location__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn os<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_dv_connectivity_last_updated_dt__between(mut self, value: impl Into<String>) -> Self {
        self.agent_dv_connectivity_last_updated_dt__between = Some(value.into());
        self
    }
    pub fn subnets__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn ranger_tags_key<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn network_name__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn instance_role__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.instance_role__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_dv_connectivity<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn active_coverage__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_console_connectivity<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_connectivity = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_idr_connectivity<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_idr_connectivity = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn all_tags_key_value__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn network_name<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn region__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_configurable_network_quarantine__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn all_tags_key__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn gateway_ips__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn manufacturer__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_tags_key_value__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn surfaces<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_pending_actions<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_resource_id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_resource_id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn os_name_version__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn missing_coverage__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn serial_number<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn asset_status__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn last_active_dt__between(mut self, value: impl Into<String>) -> Self {
        self.last_active_dt__between = Some(value.into());
        self
    }
    pub fn state__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.state__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }
    pub fn detected_from_site<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn identity_ad__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_installer_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_disk_metrics_volume_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn legacy_identity_policy_name__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.legacy_identity_policy_name__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn epp_unsupported_unknown<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn region__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_health_status__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_ranger_version__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn device_review__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_provider_account_name<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn ranger_tag_key_value__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_value__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn asset_contact_email__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage__lte(mut self, value: f64) -> Self {
        self.agent_disk_metrics_free_percentage__lte = Some(value);
        self
    }
    pub fn k8s_node__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn alert_severity<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_tags_key_value<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_node__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn serial_number__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_decommissioned<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_decommissioned = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_subscribe_on_dt__between(mut self, value: impl Into<String>) -> Self {
        self.agent_subscribe_on_dt__between = Some(value.into());
        self
    }
    pub fn domain__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn os_name_version__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn resource_type__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn gateway_macs__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_customer_identifier__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_customer_identifier__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn os__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_cluster__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_namespace__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn manufacturer__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_resource_id<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_resource_id = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn asset_contact_email<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn udp_ports<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn risk_factors<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_full_disk_scan_dt__between(mut self, value: impl Into<String>) -> Self {
        self.agent_full_disk_scan_dt__between = Some(value.into());
        self
    }
    pub fn k8s_labels_unified_key__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_labels_unified_key__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_anti_tampering_status__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn asset_environment<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn os_family<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn identity_ad_user__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn surfaces__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn ads_enabled<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ads_enabled = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn ranger_tags_key__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_location<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_pending_actions__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_actions__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn id__in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__in = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_uninstalled<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uninstalled = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_tags_key_value__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn resource_type__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn memory_readable__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_labels_unified_key_value__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_labels_unified_key_value__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_uuid__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn tags_key_value<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn sort_order(mut self, value: impl Into<String>) -> Self {
        self.sort_order = Some(value.into());
        self
    }
    pub fn k8s_cluster__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_provider_organization_unit__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn asset_criticality<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn ranger_tags_key_value<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_provider_organization__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_location_awareness__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_location_awareness__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn is_dc_server<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_dc_server = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn count_only(mut self, value: bool) -> Self {
        self.count_only = Some(value);
        self
    }
    pub fn identity_ad_user_distinguished_name__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_distinguished_name__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn os_family__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_operational_state<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_network_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn names<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_cluster<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn mac_addresses__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn instance_type__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.instance_type__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_vss_service_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_ranger_status__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }
    pub fn detected_from_site__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn name__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn application_name<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.application_name = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_annotations_unified_key__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_annotations_unified_key__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn first_seen_dt__between(mut self, value: impl Into<String>) -> Self {
        self.first_seen_dt__between = Some(value.into());
        self
    }
    pub fn last_update_dt__between(mut self, value: impl Into<String>) -> Self {
        self.last_update_dt__between = Some(value.into());
        self
    }
    pub fn agent_pending_uninstall<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_uninstall = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_anti_tampering_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_anti_tampering_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_node<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_agent_version__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn os_version<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn discovery_methods__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_provider_account_id__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn skip_count(mut self, value: bool) -> Self {
        self.skip_count = Some(value);
        self
    }
    pub fn k8s_cluster_id<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_id = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_vss_service_status__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_service_status__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_vss_last_snapshot_dt__between(mut self, value: impl Into<String>) -> Self {
        self.agent_vss_last_snapshot_dt__between = Some(value.into());
        self
    }
    pub fn agent_has_local_config<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_has_local_config = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn all_tags_key__nexists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nexists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_missing_permissions__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_missing_permissions__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_pending_upgrade<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pending_upgrade = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn manufacturer<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn virtual_network_id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.virtual_network_id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn instance_id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.instance_id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_vss_rollback_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_installer_type__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_installer_type__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn s1_updated_at__between(mut self, value: impl Into<String>) -> Self {
        self.s1_updated_at__between = Some(value.into());
        self
    }
    pub fn resource_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn asset_environment__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_version__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn names__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_uuid<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_tags_key<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn core_count<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn os_version__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn sub_category__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_version<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn discovery_methods<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn infection_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn tcp_ports<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn active_coverage<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn counts_for<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn core_count__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn csv_filter_id(mut self, value: i64) -> Self {
        self.csv_filter_id = Some(value);
        self
    }
    pub fn ranger_tag_key__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn network_security_groups__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_security_groups__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn ranger_tags_key_value__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn architecture__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_namespace<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_namespace = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn identity_ad_machine_distinguished_name__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_distinguished_name__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_s1_agent_live_updates_version__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_s1_agent_live_updates_version__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_provider_account_id<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_network_status__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_network_status__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn is_rogues<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_rogues = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn sub_category<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn os_name_version<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn image_id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_ranger_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn os_version__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn device_review<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn all_tags_key<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn serial_number__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn hostnames__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn identity_ad_machine_membership__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_machine_membership__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    pub fn k8s_type__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_annotations_unified_key_value__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_annotations_unified_key_value__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_vss_rollback_status__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_rollback_status__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_console_migration_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_console_migration_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn architecture<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn identity_ad_user_membership__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.identity_ad_user_membership__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_dv_connectivity__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_dv_connectivity__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_vss_protection_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_vss_protection_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_provider_account_name__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn internal_ips__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn subnet_id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnet_id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn asset_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn epp_unsupported_unknown__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_configurable_network_quarantine<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_configurable_network_quarantine = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_health_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_health_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_ranger_version<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_ranger_version = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_disk_metrics_volume_type__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_metrics_volume_type__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn domain__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_last_logged_in_user__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_last_logged_in_user__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn k8s_cluster_id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_cluster_id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_tags_key__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn tags_key_value__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_agent_version<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn all_tags_key__exists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__exists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_disk_metrics_free_percentage__gte(mut self, value: f64) -> Self {
        self.agent_disk_metrics_free_percentage__gte = Some(value);
        self
    }
    pub fn agent_agent_version__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn domain<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn agent_vss_volumes_diff_area_free_percentage__between(mut self, value: impl Into<String>) -> Self {
        self.agent_vss_volumes_diff_area_free_percentage__between = Some(value.into());
        self
    }
    pub fn agent_disk_encryption<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_disk_encryption = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn infection_status__nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status__nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn cloud_provider_project_id__contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id__contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query parameters for `POST /web/api/v2.1/xdr/assets/server`.
///
/// Scope selectors applied alongside the request body filter. Every field is
/// optional and array parameters are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
pub struct ServerViewQuery {
    /// List of Account IDs to filter by
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Array of strings; serialized comma-joined. Optional -> `Option`.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl ServerViewQuery {
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/server`.
///
/// Mirrors `v2_1.inventory.server.schemas_ServerViewInputSchema`. The `filter`
/// object is the large, dynamically-shaped `PaginatedServerFilter`; it is kept
/// as [`serde_json::Value`] for forward-compatibility. `filter` is required.
#[derive(Debug, Clone, Serialize)]
pub struct ServerViewBody {
    /// Filter (the `PaginatedServerFilter` object). Required.
    pub filter: serde_json::Value,
    /// Optional free-form data payload.
    ///
    /// Optional / x-nullable -> `Option`, default null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ServerViewBody {
    /// Create a view request with the required `filter`.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { filter, data: None }
    }
    /// Attach an optional free-form data payload.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/server/action`.
///
/// Mirrors `v2_1.inventory.server.schemas_ServerActionPayloadSchema`.
#[derive(Debug, Clone, Serialize)]
pub struct ServerActionBody {
    /// Action name. Required.
    ///
    /// Allowed values (enum, kept as `String` for forward-compat):
    /// `start_vm_scan`, `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`, `add_note`,
    /// `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    #[serde(rename = "actionName")]
    pub action_name: String,
    /// List of selected inventory ids (max 5000).
    ///
    /// Optional -> `Option`.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id__in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000).
    ///
    /// Optional -> `Option`.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id__nin: Option<Vec<String>>,
}

impl ServerActionBody {
    /// Create an action payload with the required `action_name`.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self { action_name: action_name.into(), id__in: None, id__nin: None }
    }
    /// Set the list of selected inventory ids.
    pub fn id_in<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__in = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set the list of inventory ids to exclude from select_all.
    pub fn id_nin<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__nin = Some(ids.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/server/available-actions/with-status`.
///
/// Mirrors `v2_1.inventory.schemas_AffectedResourcesSchema` (the set of
/// affected resources). All fields are optional.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ServerAffectedResourcesBody {
    /// List of selected inventory ids (max 5000).
    ///
    /// Optional -> `Option`.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id__in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000).
    ///
    /// Optional -> `Option`.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id__nin: Option<Vec<String>>,
}

impl ServerAffectedResourcesBody {
    /// Set the list of selected inventory ids.
    pub fn id_in<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__in = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// Set the list of inventory ids to exclude from select_all.
    pub fn id_nin<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__nin = Some(ids.into_iter().map(Into::into).collect());
        self
    }
}

impl InventoryServerService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/server` — Assets.
    ///
    /// Get assets.
    pub async fn list(&self, query: &ServerAssetsQuery) -> Result<Paginated<Server>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/server", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/server` — Assets using POST.
    ///
    /// POST API to get Assets.
    pub async fn list_post(
        &self,
        query: &ServerViewQuery,
        body: &ServerViewBody,
    ) -> Result<Paginated<Server>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(Method::POST, "/web/api/v2.1/xdr/assets/server", q, Some(body))
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/server/action` — Perform action.
    ///
    /// Perform action on selected assets.
    pub async fn action(
        &self,
        query: &ServerAssetsQuery,
        body: &ServerActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(
                Method::POST,
                "/web/api/v2.1/xdr/assets/server/action",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/server/available-actions/with-status` — Available actions.
    ///
    /// Get available actions.
    pub async fn available_actions_with_status(
        &self,
        query: &ServerAssetsQuery,
        body: &ServerAffectedResourcesBody,
    ) -> Result<Response<AvailableActionWithStatus>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(
                Method::POST,
                "/web/api/v2.1/xdr/assets/server/available-actions/with-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/server/export` — Export assets to CSV or JSON.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    ///
    /// `export_format` is required; allowed values (enum, kept as `String` for
    /// forward-compat): `csv`, `json`. It is appended to the encoded `query`.
    pub async fn export(
        &self,
        export_format: impl Into<String>,
        query: &ServerAssetsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let mut qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let ef = serde_urlencoded::to_string([("exportFormat", export_format.into())])
            .unwrap_or_default();
        if qs.is_empty() {
            qs = ef;
        } else {
            qs.push('&');
            qs.push_str(&ef);
        }
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/server/export", q)
            .await?)
    }
}
