use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_endpoint_surface_filters::{
    AutoCompleteResponse, CountFiltersResponse, FreeTextFilterResponse,
};
use crate::pagination::{Paginated, Response};

/// `Inventory Endpoint Surface Filters` tag.
///
/// Inventory Endpoint Surface Resource Filters.
pub struct InventoryEndpointSurfaceFiltersService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/xdr/assets/surface/endpoint/filters/autocomplete`.
///
/// Array params are serialized comma-joined, as the API expects. Every field is
/// optional except `key` and `text`, which are required by the spec and are
/// passed as positional arguments to [`InventoryEndpointSurfaceFiltersService::autocomplete`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutocompleteQuery {
    /// Free-text filter by tag key (supports multiple values). Optional. (array of strings)
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The agent operational state (not in). Optional. (array of strings)
    #[serde(
        rename = "agentOperationalState__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_operational_state_nin: Option<String>,
    /// The criticality that each asset belongs to (not in). Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`. (array of enum)
    #[serde(
        rename = "assetCriticality__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_criticality_nin: Option<String>,
    /// Legacy Identity Policy Name. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The missing coverage for the asset. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The agent console migration status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentConsoleMigrationStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_console_migration_status_nin: Option<String>,
    /// Tag Keys (not in). Optional. (array of strings)
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional. (string)
    #[serde(
        rename = "agentDiskMetricsFreePercentage__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// Tag Keys exists. Optional. (array of strings)
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CPU. Optional. (array of strings)
    #[serde(rename = "cpu__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The memory of the device in human readable format. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses. Optional. (array of strings)
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional. (array of strings)
    #[serde(
        rename = "tagsKeyValue__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentVssProtectionStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The risk factors associated with the asset (not in). Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`. (array of enum)
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Optional. (array of strings)
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// AD machine or its groups. Optional. (array of strings)
    #[serde(
        rename = "identityAdMachine__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_machine_contains: Option<String>,
    /// The operating system of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Is AD Connector. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// Free-text filter by the image name. Optional. (array of strings)
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The agent missing permissions. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent location (not in). Optional. (array of strings)
    #[serde(rename = "agentLocation__nin", skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// List of Group IDs to filter by. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active. Optional. (string)
    #[serde(
        rename = "agentDvConnectivityLastUpdatedDt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The subnets. Optional. (array of strings)
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// Search field key. **Required** — passed as a positional argument.
    /// Allowed values: `resourceType__contains`, `id__contains`, `name__contains`,
    /// `tagsKey__contains`, `tagsKeyValue__contains`, `agentCustomerIdentifier__contains`,
    /// `agentLocationAwareness__contains`, `agentS1AgentLiveUpdatesVersion__contains`. (enum)
    pub key: String,
    /// The connection status between the agent and the SDL service. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in). Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. (array of enum)
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in). Optional. (array of strings)
    #[serde(
        rename = "allTagsKeyValue__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_tags_key_value_nin: Option<String>,
    /// Whether the agent can configure network quarantine (not in). Optional. (array of booleans)
    #[serde(
        rename = "agentConfigurableNetworkQuarantine__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// User and cloud tag keys (not in). Optional. (array of strings)
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs. Optional. (array of strings)
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The Surface that each asset belongs to. Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The operating system name and version of the device (not in). Optional. (array of strings)
    #[serde(
        rename = "osNameVersion__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in). Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. (array of enum)
    #[serde(
        rename = "missingCoverage__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub missing_coverage_nin: Option<String>,
    /// The serial number. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in). Optional.
    /// Allowed values: `Active`, `Inactive`. (array of enum)
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The last active date. Optional. (string)
    #[serde(rename = "lastActiveDt__between", skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// Any AD string. Optional. (array of strings)
    #[serde(rename = "identityAd__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// The agent installer type. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name. Optional. (array of strings)
    #[serde(
        rename = "legacy_identity_policy_name__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// Search term text. **Required** — passed as a positional argument. (string)
    pub text: String,
    /// The agent health status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentHealthStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version (not in). Optional. (array of strings)
    #[serde(
        rename = "agentRangerVersion__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_ranger_version_nin: Option<String>,
    /// The asset review (not in). Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. (array of enum)
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Asset Contact Email (not in). Optional. (array of strings)
    #[serde(
        rename = "assetContactEmail__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_contact_email_nin: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional. (number/float)
    #[serde(
        rename = "agentDiskMetricsFreePercentage__lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The severity of the alert. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The serial number (not in). Optional. (array of strings)
    #[serde(rename = "serialNumber__nin", skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// Whether the agent is decommissioned. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time. Optional. (string)
    #[serde(
        rename = "agentSubscribeOnDt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The OS names and versions. Optional. (array of strings)
    #[serde(
        rename = "osNameVersion__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_name_version_contains: Option<String>,
    /// The domain. Optional. (array of strings)
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). Optional.
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`,
    /// `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`. (array of enum)
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs. Optional. (array of strings)
    #[serde(
        rename = "gatewayMacs__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub gateway_macs_contains: Option<String>,
    /// The customer identifier. Optional. (array of strings)
    #[serde(
        rename = "agentCustomerIdentifier__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_customer_identifier_contains: Option<String>,
    /// The operating system of the device (not in). Optional. (array of strings)
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// Asset Contact Email. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date. Optional. (string)
    #[serde(
        rename = "agentFullDiskScanDt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// The agent anti tampering status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentAntiTamperingStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups. Optional. (array of strings)
    #[serde(
        rename = "identityAdUser__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_user_contains: Option<String>,
    /// The Surface that each asset belongs to (not in). Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. (array of enum)
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// ADS Enabled. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The agent location. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in). Optional. (array of strings)
    #[serde(
        rename = "agentPendingActions__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_pending_actions_nin: Option<String>,
    /// The ID. Optional. (array of strings)
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Whether the agent is uninstalled. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// The Asset Type. Optional. (array of strings)
    #[serde(
        rename = "resourceType__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in). Optional. (array of strings)
    #[serde(
        rename = "memoryReadable__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_readable_nin: Option<String>,
    /// The UUID. Optional. (array of strings)
    #[serde(rename = "agentUuid__contains", skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// Tags. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Is DC Server. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// The location awareness. Optional. (array of strings)
    #[serde(
        rename = "agentLocationAwareness__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_location_awareness_contains: Option<String>,
    /// AD user DN. Optional. (array of strings)
    #[serde(
        rename = "identityAdUserDistinguishedName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// The agent operational state. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The agent VSS service status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The MAC addresses. Optional. (array of strings)
    #[serde(
        rename = "macAddresses__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_addresses_contains: Option<String>,
    /// The agent network scanner status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentRangerStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_ranger_status_nin: Option<String>,
    /// The agent version (not in). Optional. (array of strings)
    #[serde(
        rename = "agentAgentVersion__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_agent_version_nin: Option<String>,
    /// The name. Optional. (array of strings)
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The operating system version of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Whether the agent is pending uninstall. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The agent anti tampering status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The first seen date. Optional. (string)
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The name of the application installed on a workstation or a server. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// The criticality that each asset belongs to. Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The agent VSS service status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentVssServiceStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS last snapshot date. Optional. (string)
    #[serde(
        rename = "agentVssLastSnapshotDt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// Whether the agent has local configuration. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists. Optional. (array of strings)
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The agent missing permissions (not in). Optional. (array of strings)
    #[serde(
        rename = "agentMissingPermissions__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_missing_permissions_nin: Option<String>,
    /// Whether the agent is pending upgrade. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The agent VSS rollback status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in). Optional. (array of strings)
    #[serde(
        rename = "agentInstallerType__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_installer_type_nin: Option<String>,
    /// The Last Seen date and time for the asset. Optional. (string)
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Optional.
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`,
    /// `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in). Optional. (array of strings)
    #[serde(
        rename = "assetEnvironment__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). Optional. (array of strings)
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Match by the agent UUID. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The number of cores. Optional. (array of int32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in). Optional. (array of strings)
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in). Optional.
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`. (array of enum)
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset. Optional.
    /// Allowed values: `Infected`, `Healthy`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in). Optional. (array of int32)
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by. Optional. (int32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The architecture of the device (not in). Optional. (array of strings)
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// AD machine DN. Optional. (array of strings)
    #[serde(
        rename = "identityAdMachineDistinguishedName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// Live update ID. Optional. (array of strings)
    #[serde(
        rename = "agentS1AgentLiveUpdatesVersion__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The agent network status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentNetworkStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_network_status_nin: Option<String>,
    /// The sub-category that each resource belongs to. Optional.
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The agent network scanner status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions. Optional. (array of strings)
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review. Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number. Optional. (array of strings)
    #[serde(
        rename = "serialNumber__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub serial_number_contains: Option<String>,
    /// The hostnames. Optional. (array of strings)
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// AD machine groups. Optional. (array of strings)
    #[serde(
        rename = "identityAdMachineMembership__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// Limit number of returned items. Optional. (int32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The agent VSS rollback status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentVssRollbackStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent console migration status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups. Optional. (array of strings)
    #[serde(
        rename = "identityAdUserMembership__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_user_membership_contains: Option<String>,
    /// The connection status between the agent and the SDL service (not in). Optional. (array of strings)
    #[serde(
        rename = "agentDvConnectivity__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent VSS protection status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The ID. Optional. (array of strings)
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs. Optional. (array of strings)
    #[serde(
        rename = "internalIps__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset. Optional.
    /// Allowed values: `Active`, `Inactive`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether the agent can configure network quarantine. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk encryption. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The domain of the device (not in). Optional. (array of strings)
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// The agent disk metrics volume type (not in). Optional. (array of strings)
    #[serde(
        rename = "agentDiskMetricsVolumeType__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The last logged in user. Optional. (array of strings)
    #[serde(
        rename = "agentLastLoggedInUser__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Tags (not in). Optional. (array of strings)
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists. Optional. (array of strings)
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional. (number/float)
    #[serde(
        rename = "agentDiskMetricsFreePercentage__gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent version. Optional. (array of strings)
    #[serde(
        rename = "agentAgentVersion__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_agent_version_contains: Option<String>,
    /// The domain of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes. Optional. (string)
    #[serde(
        rename = "agentVssVolumesDiffAreaFreePercentage__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The operating system family of the device (not in). Optional. (array of strings)
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The status alerts of the asset (not in). Optional.
    /// Allowed values: `Infected`, `Healthy`. (array of enum)
    #[serde(
        rename = "infectionStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

macro_rules! str_setter {
    ($field:ident) => {
        #[doc = concat!("Set `", stringify!($field), "` (single value).")]
        pub fn $field(mut self, v: impl Into<String>) -> Self {
            self.$field = Some(v.into());
            self
        }
    };
}

macro_rules! arr_setter {
    ($field:ident) => {
        #[doc = concat!("Set `", stringify!($field), "` from an iterator (comma-joined).")]
        pub fn $field<I, S>(mut self, vals: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>,
        {
            let joined = vals
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(",");
            self.$field = Some(joined);
            self
        }
    };
}

impl AutocompleteQuery {
    /// Construct a new query with the two required params, `key` and `text`.
    pub fn new(key: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            text: text.into(),
            ..Default::default()
        }
    }

    /// Set the **required** search field key.
    pub fn key(mut self, v: impl Into<String>) -> Self {
        self.key = v.into();
        self
    }
    /// Set the **required** search term text.
    pub fn text(mut self, v: impl Into<String>) -> Self {
        self.text = v.into();
        self
    }

    arr_setter!(tags_key_contains);
    arr_setter!(agent_operational_state_nin);
    arr_setter!(asset_criticality_nin);
    arr_setter!(legacy_identity_policy_name);
    arr_setter!(missing_coverage);
    arr_setter!(all_tags_key_value);
    arr_setter!(agent_console_migration_status_nin);
    arr_setter!(tags_key_nin);
    str_setter!(agent_disk_metrics_free_percentage_between);
    arr_setter!(tags_key_exists);
    arr_setter!(cpu_contains);
    arr_setter!(memory_readable);
    arr_setter!(ip_address_contains);
    arr_setter!(tags_key_value_contains);
    arr_setter!(tags_key);
    arr_setter!(agent_vss_protection_status_nin);
    arr_setter!(risk_factors_nin);
    arr_setter!(tags_key_nexists);
    arr_setter!(identity_ad_machine_contains);
    arr_setter!(os);
    arr_setter!(is_ad_connector);
    arr_setter!(image_name_contains);
    arr_setter!(agent_missing_permissions);
    arr_setter!(agent_location_nin);
    arr_setter!(group_ids);
    str_setter!(agent_dv_connectivity_last_updated_dt_between);
    arr_setter!(subnets_contains);
    arr_setter!(agent_dv_connectivity);
    arr_setter!(active_coverage_nin);
    arr_setter!(agent_console_connectivity);
    arr_setter!(agent_idr_connectivity);
    arr_setter!(all_tags_key_value_nin);
    arr_setter!(agent_configurable_network_quarantine_nin);
    arr_setter!(all_tags_key_nin);
    arr_setter!(gateway_ips_contains);
    arr_setter!(surfaces);
    arr_setter!(agent_pending_actions);
    arr_setter!(os_name_version_nin);
    arr_setter!(missing_coverage_nin);
    arr_setter!(serial_number);
    arr_setter!(asset_status_nin);
    str_setter!(last_active_dt_between);
    arr_setter!(identity_ad_contains);
    arr_setter!(agent_installer_type);
    arr_setter!(agent_disk_metrics_volume_type);
    arr_setter!(legacy_identity_policy_name_contains);
    arr_setter!(agent_health_status_nin);
    arr_setter!(agent_ranger_version_nin);
    arr_setter!(device_review_nin);
    arr_setter!(asset_contact_email_nin);
    /// Set `agent_disk_metrics_free_percentage_lte`. Optional (number/float).
    pub fn agent_disk_metrics_free_percentage_lte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_lte = Some(v);
        self
    }
    arr_setter!(alert_severity);
    arr_setter!(account_ids);
    arr_setter!(serial_number_nin);
    arr_setter!(agent_decommissioned);
    str_setter!(agent_subscribe_on_dt_between);
    arr_setter!(os_name_version_contains);
    arr_setter!(domain_contains);
    arr_setter!(resource_type_nin);
    arr_setter!(gateway_macs_contains);
    arr_setter!(agent_customer_identifier_contains);
    arr_setter!(os_nin);
    arr_setter!(asset_contact_email);
    arr_setter!(risk_factors);
    str_setter!(agent_full_disk_scan_dt_between);
    arr_setter!(agent_anti_tampering_status_nin);
    arr_setter!(asset_environment);
    arr_setter!(os_family);
    arr_setter!(identity_ad_user_contains);
    arr_setter!(surfaces_nin);
    arr_setter!(ads_enabled);
    arr_setter!(agent_location);
    arr_setter!(agent_pending_actions_nin);
    arr_setter!(id_in);
    arr_setter!(agent_uninstalled);
    arr_setter!(resource_type_contains);
    arr_setter!(memory_readable_nin);
    arr_setter!(agent_uuid_contains);
    arr_setter!(tags_key_value);
    arr_setter!(is_dc_server);
    arr_setter!(agent_location_awareness_contains);
    arr_setter!(identity_ad_user_distinguished_name_contains);
    arr_setter!(agent_operational_state);
    arr_setter!(agent_network_status);
    arr_setter!(names);
    arr_setter!(agent_vss_service_status);
    arr_setter!(mac_addresses_contains);
    arr_setter!(agent_ranger_status_nin);
    arr_setter!(agent_agent_version_nin);
    arr_setter!(name_contains);
    arr_setter!(os_version);
    arr_setter!(agent_pending_uninstall);
    arr_setter!(agent_anti_tampering_status);
    str_setter!(first_seen_dt_between);
    arr_setter!(application_name);
    arr_setter!(asset_criticality);
    arr_setter!(agent_vss_service_status_nin);
    str_setter!(agent_vss_last_snapshot_dt_between);
    arr_setter!(agent_has_local_config);
    arr_setter!(all_tags_key_nexists);
    arr_setter!(agent_missing_permissions_nin);
    arr_setter!(agent_pending_upgrade);
    arr_setter!(agent_vss_rollback_status);
    arr_setter!(site_ids);
    arr_setter!(agent_installer_type_nin);
    str_setter!(s1_updated_at_between);
    arr_setter!(resource_type);
    arr_setter!(asset_environment_nin);
    arr_setter!(names_nin);
    arr_setter!(agent_uuid);
    arr_setter!(core_count);
    arr_setter!(os_version_nin);
    arr_setter!(sub_category_nin);
    arr_setter!(infection_status);
    arr_setter!(active_coverage);
    arr_setter!(counts_for);
    arr_setter!(core_count_nin);
    /// Set `csv_filter_id`. Optional (int32).
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    arr_setter!(architecture_nin);
    arr_setter!(identity_ad_machine_distinguished_name_contains);
    arr_setter!(agent_s1_agent_live_updates_version_contains);
    arr_setter!(agent_network_status_nin);
    arr_setter!(sub_category);
    arr_setter!(agent_ranger_status);
    arr_setter!(os_version_contains);
    arr_setter!(device_review);
    arr_setter!(all_tags_key);
    arr_setter!(serial_number_contains);
    arr_setter!(hostnames_contains);
    arr_setter!(identity_ad_machine_membership_contains);
    /// Set `limit` (number of returned items). Optional (int32).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
    arr_setter!(agent_vss_rollback_status_nin);
    arr_setter!(agent_console_migration_status);
    arr_setter!(architecture);
    arr_setter!(identity_ad_user_membership_contains);
    arr_setter!(agent_dv_connectivity_nin);
    arr_setter!(agent_vss_protection_status);
    arr_setter!(id_contains);
    arr_setter!(internal_ips_contains);
    arr_setter!(asset_status);
    arr_setter!(agent_configurable_network_quarantine);
    arr_setter!(agent_health_status);
    arr_setter!(agent_ranger_version);
    arr_setter!(agent_disk_encryption);
    arr_setter!(domain_nin);
    arr_setter!(agent_disk_metrics_volume_type_nin);
    arr_setter!(agent_last_logged_in_user_contains);
    arr_setter!(tags_key_value_nin);
    arr_setter!(agent_agent_version);
    arr_setter!(all_tags_key_exists);
    /// Set `agent_disk_metrics_free_percentage_gte`. Optional (number/float).
    pub fn agent_disk_metrics_free_percentage_gte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_gte = Some(v);
        self
    }
    arr_setter!(agent_agent_version_contains);
    arr_setter!(domain);
    str_setter!(agent_vss_volumes_diff_area_free_percentage_between);
    arr_setter!(os_family_nin);
    arr_setter!(infection_status_nin);
    arr_setter!(os_name_version);
}

/// Query params for `GET /web/api/v2.1/xdr/assets/surface/endpoint/filters/count`.
///
/// Array params are serialized comma-joined, as the API expects. Every field is
/// optional. (The `count` endpoint has the same filter set as `autocomplete`
/// minus the required `key` and `text` search params.)
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountQuery {
    /// Free-text filter by tag key (supports multiple values). Optional. (array of strings)
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The agent operational state (not in). Optional. (array of strings)
    #[serde(
        rename = "agentOperationalState__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_operational_state_nin: Option<String>,
    /// The criticality that each asset belongs to (not in). Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`. (array of enum)
    #[serde(
        rename = "assetCriticality__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_criticality_nin: Option<String>,
    /// Legacy Identity Policy Name. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The missing coverage for the asset. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The agent console migration status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentConsoleMigrationStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_console_migration_status_nin: Option<String>,
    /// Tag Keys (not in). Optional. (array of strings)
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional. (string)
    #[serde(
        rename = "agentDiskMetricsFreePercentage__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// Tag Keys exists. Optional. (array of strings)
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CPU. Optional. (array of strings)
    #[serde(rename = "cpu__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The memory of the device in human readable format. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses. Optional. (array of strings)
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional. (array of strings)
    #[serde(
        rename = "tagsKeyValue__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The agent VSS protection status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentVssProtectionStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The risk factors associated with the asset (not in). Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`. (array of enum)
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Optional. (array of strings)
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// AD machine or its groups. Optional. (array of strings)
    #[serde(
        rename = "identityAdMachine__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_machine_contains: Option<String>,
    /// The operating system of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Is AD Connector. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// Free-text filter by the image name. Optional. (array of strings)
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The agent missing permissions. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent location (not in). Optional. (array of strings)
    #[serde(rename = "agentLocation__nin", skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// List of Group IDs to filter by. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The SDL connectivity last active. Optional. (string)
    #[serde(
        rename = "agentDvConnectivityLastUpdatedDt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The subnets. Optional. (array of strings)
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The connection status between the agent and the SDL service. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The active coverage for the asset (not in). Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. (array of enum)
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// User and cloud tags (not in). Optional. (array of strings)
    #[serde(
        rename = "allTagsKeyValue__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_tags_key_value_nin: Option<String>,
    /// Whether the agent can configure network quarantine (not in). Optional. (array of booleans)
    #[serde(
        rename = "agentConfigurableNetworkQuarantine__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// User and cloud tag keys (not in). Optional. (array of strings)
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs. Optional. (array of strings)
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The Surface that each asset belongs to. Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The operating system name and version of the device (not in). Optional. (array of strings)
    #[serde(
        rename = "osNameVersion__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in). Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. (array of enum)
    #[serde(
        rename = "missingCoverage__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub missing_coverage_nin: Option<String>,
    /// The serial number. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The status of the asset (not in). Optional.
    /// Allowed values: `Active`, `Inactive`. (array of enum)
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The last active date. Optional. (string)
    #[serde(rename = "lastActiveDt__between", skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// Any AD string. Optional. (array of strings)
    #[serde(rename = "identityAd__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// The agent installer type. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent disk metrics volume type. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The legacy identity policy name. Optional. (array of strings)
    #[serde(
        rename = "legacy_identity_policy_name__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// The agent health status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentHealthStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version (not in). Optional. (array of strings)
    #[serde(
        rename = "agentRangerVersion__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_ranger_version_nin: Option<String>,
    /// The asset review (not in). Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. (array of enum)
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Asset Contact Email (not in). Optional. (array of strings)
    #[serde(
        rename = "assetContactEmail__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_contact_email_nin: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional. (number/float)
    #[serde(
        rename = "agentDiskMetricsFreePercentage__lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The severity of the alert. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The serial number (not in). Optional. (array of strings)
    #[serde(rename = "serialNumber__nin", skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// Whether the agent is decommissioned. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent subscribe time. Optional. (string)
    #[serde(
        rename = "agentSubscribeOnDt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The OS names and versions. Optional. (array of strings)
    #[serde(
        rename = "osNameVersion__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_name_version_contains: Option<String>,
    /// The domain. Optional. (array of strings)
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). Optional.
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`,
    /// `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`. (array of enum)
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs. Optional. (array of strings)
    #[serde(
        rename = "gatewayMacs__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub gateway_macs_contains: Option<String>,
    /// The customer identifier. Optional. (array of strings)
    #[serde(
        rename = "agentCustomerIdentifier__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_customer_identifier_contains: Option<String>,
    /// The operating system of the device (not in). Optional. (array of strings)
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// Asset Contact Email. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The agent full disk scan date. Optional. (string)
    #[serde(
        rename = "agentFullDiskScanDt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// The agent anti tampering status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentAntiTamperingStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// AD user or their groups. Optional. (array of strings)
    #[serde(
        rename = "identityAdUser__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_user_contains: Option<String>,
    /// The Surface that each asset belongs to (not in). Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`. (array of enum)
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// ADS Enabled. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// The agent location. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent pending actions (not in). Optional. (array of strings)
    #[serde(
        rename = "agentPendingActions__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_pending_actions_nin: Option<String>,
    /// The ID. Optional. (array of strings)
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Whether the agent is uninstalled. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// The Asset Type. Optional. (array of strings)
    #[serde(
        rename = "resourceType__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in). Optional. (array of strings)
    #[serde(
        rename = "memoryReadable__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub memory_readable_nin: Option<String>,
    /// The UUID. Optional. (array of strings)
    #[serde(rename = "agentUuid__contains", skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// Tags. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Is DC Server. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// The location awareness. Optional. (array of strings)
    #[serde(
        rename = "agentLocationAwareness__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_location_awareness_contains: Option<String>,
    /// AD user DN. Optional. (array of strings)
    #[serde(
        rename = "identityAdUserDistinguishedName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// The agent operational state. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent network status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// Name. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The agent VSS service status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The MAC addresses. Optional. (array of strings)
    #[serde(
        rename = "macAddresses__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_addresses_contains: Option<String>,
    /// The agent network scanner status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentRangerStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_ranger_status_nin: Option<String>,
    /// The agent version (not in). Optional. (array of strings)
    #[serde(
        rename = "agentAgentVersion__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_agent_version_nin: Option<String>,
    /// The name. Optional. (array of strings)
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The operating system version of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Whether the agent is pending uninstall. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// The agent anti tampering status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The first seen date. Optional. (string)
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The name of the application installed on a workstation or a server. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// The criticality that each asset belongs to. Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The agent VSS service status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentVssServiceStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS last snapshot date. Optional. (string)
    #[serde(
        rename = "agentVssLastSnapshotDt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// Whether the agent has local configuration. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// User and cloud tag keys not exists. Optional. (array of strings)
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The agent missing permissions (not in). Optional. (array of strings)
    #[serde(
        rename = "agentMissingPermissions__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_missing_permissions_nin: Option<String>,
    /// Whether the agent is pending upgrade. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The agent VSS rollback status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// List of Site IDs to filter by. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent installer type (not in). Optional. (array of strings)
    #[serde(
        rename = "agentInstallerType__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_installer_type_nin: Option<String>,
    /// The Last Seen date and time for the asset. Optional. (string)
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Optional.
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`,
    /// `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in). Optional. (array of strings)
    #[serde(
        rename = "assetEnvironment__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). Optional. (array of strings)
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Match by the agent UUID. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The number of cores. Optional. (array of int32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in). Optional. (array of strings)
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in). Optional.
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`. (array of enum)
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset. Optional.
    /// Allowed values: `Infected`, `Healthy`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in). Optional. (array of int32)
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by. Optional. (int32)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The architecture of the device (not in). Optional. (array of strings)
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// AD machine DN. Optional. (array of strings)
    #[serde(
        rename = "identityAdMachineDistinguishedName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// Live update ID. Optional. (array of strings)
    #[serde(
        rename = "agentS1AgentLiveUpdatesVersion__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The agent network status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentNetworkStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_network_status_nin: Option<String>,
    /// The sub-category that each resource belongs to. Optional.
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The agent network scanner status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The OS versions. Optional. (array of strings)
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review. Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The serial number. Optional. (array of strings)
    #[serde(
        rename = "serialNumber__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub serial_number_contains: Option<String>,
    /// The hostnames. Optional. (array of strings)
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// AD machine groups. Optional. (array of strings)
    #[serde(
        rename = "identityAdMachineMembership__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// The agent VSS rollback status (not in). Optional. (array of strings)
    #[serde(
        rename = "agentVssRollbackStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent console migration status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The architecture of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// AD user groups. Optional. (array of strings)
    #[serde(
        rename = "identityAdUserMembership__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_ad_user_membership_contains: Option<String>,
    /// The connection status between the agent and the SDL service (not in). Optional. (array of strings)
    #[serde(
        rename = "agentDvConnectivity__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent VSS protection status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The ID. Optional. (array of strings)
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs. Optional. (array of strings)
    #[serde(
        rename = "internalIps__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset. Optional.
    /// Allowed values: `Active`, `Inactive`. (array of enum)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether the agent can configure network quarantine. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// The agent health status. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent network scanner version. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent disk encryption. Optional. (array of booleans)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The domain of the device (not in). Optional. (array of strings)
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// The agent disk metrics volume type (not in). Optional. (array of strings)
    #[serde(
        rename = "agentDiskMetricsVolumeType__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The last logged in user. Optional. (array of strings)
    #[serde(
        rename = "agentLastLoggedInUser__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Tags (not in). Optional. (array of strings)
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists. Optional. (array of strings)
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional. (number/float)
    #[serde(
        rename = "agentDiskMetricsFreePercentage__gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent version. Optional. (array of strings)
    #[serde(
        rename = "agentAgentVersion__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_agent_version_contains: Option<String>,
    /// The domain of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes. Optional. (string)
    #[serde(
        rename = "agentVssVolumesDiffAreaFreePercentage__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The operating system family of the device (not in). Optional. (array of strings)
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The status alerts of the asset (not in). Optional.
    /// Allowed values: `Infected`, `Healthy`. (array of enum)
    #[serde(
        rename = "infectionStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device. Optional. (array of strings)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl CountQuery {
    arr_setter!(tags_key_contains);
    arr_setter!(agent_operational_state_nin);
    arr_setter!(asset_criticality_nin);
    arr_setter!(legacy_identity_policy_name);
    arr_setter!(missing_coverage);
    arr_setter!(all_tags_key_value);
    arr_setter!(agent_console_migration_status_nin);
    arr_setter!(tags_key_nin);
    str_setter!(agent_disk_metrics_free_percentage_between);
    arr_setter!(tags_key_exists);
    arr_setter!(cpu_contains);
    arr_setter!(memory_readable);
    arr_setter!(ip_address_contains);
    arr_setter!(tags_key_value_contains);
    arr_setter!(tags_key);
    arr_setter!(agent_vss_protection_status_nin);
    arr_setter!(risk_factors_nin);
    arr_setter!(tags_key_nexists);
    arr_setter!(identity_ad_machine_contains);
    arr_setter!(os);
    arr_setter!(is_ad_connector);
    arr_setter!(image_name_contains);
    arr_setter!(agent_missing_permissions);
    arr_setter!(agent_location_nin);
    arr_setter!(group_ids);
    str_setter!(agent_dv_connectivity_last_updated_dt_between);
    arr_setter!(subnets_contains);
    arr_setter!(agent_dv_connectivity);
    arr_setter!(active_coverage_nin);
    arr_setter!(agent_console_connectivity);
    arr_setter!(agent_idr_connectivity);
    arr_setter!(all_tags_key_value_nin);
    arr_setter!(agent_configurable_network_quarantine_nin);
    arr_setter!(all_tags_key_nin);
    arr_setter!(gateway_ips_contains);
    arr_setter!(surfaces);
    arr_setter!(agent_pending_actions);
    arr_setter!(os_name_version_nin);
    arr_setter!(missing_coverage_nin);
    arr_setter!(serial_number);
    arr_setter!(asset_status_nin);
    str_setter!(last_active_dt_between);
    arr_setter!(identity_ad_contains);
    arr_setter!(agent_installer_type);
    arr_setter!(agent_disk_metrics_volume_type);
    arr_setter!(legacy_identity_policy_name_contains);
    arr_setter!(agent_health_status_nin);
    arr_setter!(agent_ranger_version_nin);
    arr_setter!(device_review_nin);
    arr_setter!(asset_contact_email_nin);
    /// Set `agent_disk_metrics_free_percentage_lte`. Optional (number/float).
    pub fn agent_disk_metrics_free_percentage_lte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_lte = Some(v);
        self
    }
    arr_setter!(alert_severity);
    arr_setter!(account_ids);
    arr_setter!(serial_number_nin);
    arr_setter!(agent_decommissioned);
    str_setter!(agent_subscribe_on_dt_between);
    arr_setter!(os_name_version_contains);
    arr_setter!(domain_contains);
    arr_setter!(resource_type_nin);
    arr_setter!(gateway_macs_contains);
    arr_setter!(agent_customer_identifier_contains);
    arr_setter!(os_nin);
    arr_setter!(asset_contact_email);
    arr_setter!(risk_factors);
    str_setter!(agent_full_disk_scan_dt_between);
    arr_setter!(agent_anti_tampering_status_nin);
    arr_setter!(asset_environment);
    arr_setter!(os_family);
    arr_setter!(identity_ad_user_contains);
    arr_setter!(surfaces_nin);
    arr_setter!(ads_enabled);
    arr_setter!(agent_location);
    arr_setter!(agent_pending_actions_nin);
    arr_setter!(id_in);
    arr_setter!(agent_uninstalled);
    arr_setter!(resource_type_contains);
    arr_setter!(memory_readable_nin);
    arr_setter!(agent_uuid_contains);
    arr_setter!(tags_key_value);
    arr_setter!(is_dc_server);
    arr_setter!(agent_location_awareness_contains);
    arr_setter!(identity_ad_user_distinguished_name_contains);
    arr_setter!(agent_operational_state);
    arr_setter!(agent_network_status);
    arr_setter!(names);
    arr_setter!(agent_vss_service_status);
    arr_setter!(mac_addresses_contains);
    arr_setter!(agent_ranger_status_nin);
    arr_setter!(agent_agent_version_nin);
    arr_setter!(name_contains);
    arr_setter!(os_version);
    arr_setter!(agent_pending_uninstall);
    arr_setter!(agent_anti_tampering_status);
    str_setter!(first_seen_dt_between);
    arr_setter!(application_name);
    arr_setter!(asset_criticality);
    arr_setter!(agent_vss_service_status_nin);
    str_setter!(agent_vss_last_snapshot_dt_between);
    arr_setter!(agent_has_local_config);
    arr_setter!(all_tags_key_nexists);
    arr_setter!(agent_missing_permissions_nin);
    arr_setter!(agent_pending_upgrade);
    arr_setter!(agent_vss_rollback_status);
    arr_setter!(site_ids);
    arr_setter!(agent_installer_type_nin);
    str_setter!(s1_updated_at_between);
    arr_setter!(resource_type);
    arr_setter!(asset_environment_nin);
    arr_setter!(names_nin);
    arr_setter!(agent_uuid);
    arr_setter!(core_count);
    arr_setter!(os_version_nin);
    arr_setter!(sub_category_nin);
    arr_setter!(infection_status);
    arr_setter!(active_coverage);
    arr_setter!(counts_for);
    arr_setter!(core_count_nin);
    /// Set `csv_filter_id`. Optional (int32).
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    arr_setter!(architecture_nin);
    arr_setter!(identity_ad_machine_distinguished_name_contains);
    arr_setter!(agent_s1_agent_live_updates_version_contains);
    arr_setter!(agent_network_status_nin);
    arr_setter!(sub_category);
    arr_setter!(agent_ranger_status);
    arr_setter!(os_version_contains);
    arr_setter!(device_review);
    arr_setter!(all_tags_key);
    arr_setter!(serial_number_contains);
    arr_setter!(hostnames_contains);
    arr_setter!(identity_ad_machine_membership_contains);
    arr_setter!(agent_vss_rollback_status_nin);
    arr_setter!(agent_console_migration_status);
    arr_setter!(architecture);
    arr_setter!(identity_ad_user_membership_contains);
    arr_setter!(agent_dv_connectivity_nin);
    arr_setter!(agent_vss_protection_status);
    arr_setter!(id_contains);
    arr_setter!(internal_ips_contains);
    arr_setter!(asset_status);
    arr_setter!(agent_configurable_network_quarantine);
    arr_setter!(agent_health_status);
    arr_setter!(agent_ranger_version);
    arr_setter!(agent_disk_encryption);
    arr_setter!(domain_nin);
    arr_setter!(agent_disk_metrics_volume_type_nin);
    arr_setter!(agent_last_logged_in_user_contains);
    arr_setter!(tags_key_value_nin);
    arr_setter!(agent_agent_version);
    arr_setter!(all_tags_key_exists);
    /// Set `agent_disk_metrics_free_percentage_gte`. Optional (number/float).
    pub fn agent_disk_metrics_free_percentage_gte(mut self, v: f64) -> Self {
        self.agent_disk_metrics_free_percentage_gte = Some(v);
        self
    }
    arr_setter!(agent_agent_version_contains);
    arr_setter!(domain);
    str_setter!(agent_vss_volumes_diff_area_free_percentage_between);
    arr_setter!(os_family_nin);
    arr_setter!(infection_status_nin);
    arr_setter!(os_name_version);
}

impl InventoryEndpointSurfaceFiltersService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/surface/endpoint/filters/autocomplete` — Auto Complete.
    ///
    /// Use this command to get values for other fields. When you send this
    /// command with input text and a field name, it returns auto-complete
    /// suggestions for the field.
    ///
    /// The two required params (`key` and `text`) live on [`AutocompleteQuery`];
    /// construct it with [`AutocompleteQuery::new`].
    pub async fn autocomplete(
        &self,
        query: &AutocompleteQuery,
    ) -> Result<Response<AutoCompleteResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/assets/surface/endpoint/filters/autocomplete",
                q,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/surface/endpoint/filters/count` — Filter counts.
    ///
    /// Get Endpoint filter counts.
    pub async fn count(
        &self,
        query: &CountQuery,
    ) -> Result<Paginated<CountFiltersResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/surface/endpoint/filters/count", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/surface/endpoint/filters/free-text` — Free text filters.
    ///
    /// Get Endpoint free text filters. This endpoint takes no parameters.
    pub async fn free_text(&self) -> Result<Paginated<FreeTextFilterResponse>, Error> {
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/assets/surface/endpoint/filters/free-text",
                None,
            )
            .await?)
    }
}
