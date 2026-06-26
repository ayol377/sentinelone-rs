//! `Inventory Container` tag — Inventory Container Resources.
//!
//! Endpoints for listing, filtering, exporting and acting on inventory
//! "container" assets surfaced under `/web/api/v2.1/xdr/assets/container`.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_container::{AvailableActionWithStatusResponse, Container};
use crate::pagination::{Paginated, Response};

/// `Inventory Container` tag.
///
/// Inventory Container Resources. Provides access to the XDR inventory
/// container assets: listing/filtering (`GET`/`POST`), exporting to CSV/JSON,
/// performing bulk actions, and discovering the available actions for a
/// selection.
pub struct InventoryContainerService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// =====================================================================
// Helpers
// =====================================================================

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

// =====================================================================
// Shared filter query
// =====================================================================

/// The full inventory-container filter parameter set.
///
/// This struct is shared by every `Inventory Container` endpoint that accepts
/// the inventory filter in the query string: `GET .../container`,
/// `POST .../container/action`,
/// `POST .../container/available-actions/with-status` and
/// `GET .../container/export`.
///
/// Endpoint-specific notes:
/// - `skip`, `sortBy`, `sortOrder`, `countOnly`, `skipCount`, `cursor` and
///   `limit` apply to the list (`GET`) and `export` (`GET`) endpoints only.
/// - `exportFormat` is **required** for the export endpoint (set via
///   [`ContainerFilterQuery::export_format`]).
///
/// Every field is optional. Array params are serialized as comma-joined
/// strings, which the API accepts. All field names are camelCase; odd names
/// (`name__contains`, `legacy_identity_policy_name__contains`, etc.) carry an
/// explicit `#[serde(rename = ...)]`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerFilterQuery {
    // ---- scoping ----
    /// List of Account IDs to filter by. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,

    // ---- pagination / sorting (list + export only) ----
    /// Limit number of returned items (1-1000). Optional. (integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional. (integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional. (string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The column to sort the results by. Optional.
    ///
    /// Allowed values (enum): `s1GroupName`, `cpu`,
    /// `legacyIdentityPolicyName`, `agentFirewallStatus`, `s1UpdatedAt`,
    /// `cloudProviderResourceGroup`, `region`, `cloudProviderOrganization`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Allowed values (enum): `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// If true, only the total number of items is returned, without any of the
    /// actual objects. Optional. (boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, the total number of items is not calculated, which speeds up
    /// execution time. Optional. (boolean)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Export format. **Required for the export endpoint.** Allowed values
    /// (enum): `csv`, `json`. Optional for all other endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<String>,
    /// The columns for which filter count would be returned for. Optional.
    /// (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by. Optional. (integer)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,

    // ---- identity / tags ----
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// Tag Keys (not in). Optional. (array<string>)
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// Tag Keys exists. Optional. (array<string>)
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// Tag Keys not exists. Optional. (array<string>)
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// Tag Keys. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// Tags. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tags (not in). Optional. (array<string>)
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tags. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// User and cloud tags (not in). Optional. (array<string>)
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// User and cloud tag keys. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// User and cloud tag keys (not in). Optional. (array<string>)
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// User and cloud tag keys exists. Optional. (array<string>)
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// User and cloud tag keys not exists. Optional. (array<string>)
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The cloud tags key. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The cloud tags key (not in). Optional. (array<string>)
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Optional.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// The cloud tags key value. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The cloud tags key value (not in). Optional. (array<string>)
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    /// Optional.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,

    // ---- agent ----
    /// The agent operational state. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent operational state (not in). Optional. (array<string>)
    #[serde(rename = "agentOperationalState__nin", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state_nin: Option<String>,
    /// The agent console migration status. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The agent console migration status (not in). Optional. (array<string>)
    #[serde(rename = "agentConsoleMigrationStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status_nin: Option<String>,
    /// The agent free disk percentage on any of the disks (between). Optional.
    /// (string)
    #[serde(rename = "agentDiskMetricsFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// The agent free disk percentage on any of the disks (<=). Optional.
    /// (number)
    #[serde(rename = "agentDiskMetricsFreePercentage__lte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The agent free disk percentage on any of the disks (>=). Optional.
    /// (number)
    #[serde(rename = "agentDiskMetricsFreePercentage__gte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent disk metrics volume type. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The agent disk metrics volume type (not in). Optional. (array<string>)
    #[serde(rename = "agentDiskMetricsVolumeType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The agent VSS protection status. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The agent VSS protection status (not in). Optional. (array<string>)
    #[serde(rename = "agentVssProtectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The agent VSS service status. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The agent VSS service status (not in). Optional. (array<string>)
    #[serde(rename = "agentVssServiceStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent VSS rollback status. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// The agent VSS rollback status (not in). Optional. (array<string>)
    #[serde(rename = "agentVssRollbackStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent VSS last snapshot date (between). Optional. (string)
    #[serde(rename = "agentVssLastSnapshotDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes (between).
    /// Optional. (string)
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The connection status between the agent and the SDL service. Optional.
    /// (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The connection status between the agent and the SDL service (not in).
    /// Optional. (array<string>)
    #[serde(rename = "agentDvConnectivity__nin", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The SDL connectivity last active (between). Optional. (string)
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The agent console connectivity. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent Idr connectivity. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// Whether the agent can configure network quarantine. Optional.
    /// (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// Whether the agent can configure network quarantine (not in). Optional.
    /// (array<boolean>)
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// The agent missing permissions. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent missing permissions (not in). Optional. (array<string>)
    #[serde(rename = "agentMissingPermissions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions_nin: Option<String>,
    /// The agent location. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The agent location (not in). Optional. (array<string>)
    #[serde(rename = "agentLocation__nin", skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// The location awareness. Optional. (array<string>)
    #[serde(rename = "agentLocationAwareness__contains", skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness_contains: Option<String>,
    /// The agent pending actions. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The agent pending actions (not in). Optional. (array<string>)
    #[serde(rename = "agentPendingActions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions_nin: Option<String>,
    /// Whether the agent is decommissioned. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// Whether the agent is uninstalled. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// Whether the agent is pending uninstall. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// Whether the agent is pending upgrade. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// Whether the agent has local configuration. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// The agent disk encryption. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The agent subscribe time (between). Optional. (string)
    #[serde(rename = "agentSubscribeOnDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// The agent full disk scan date (between). Optional. (string)
    #[serde(rename = "agentFullDiskScanDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// The customer identifier. Optional. (array<string>)
    #[serde(rename = "agentCustomerIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier_contains: Option<String>,
    /// The agent installer type. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent installer type (not in). Optional. (array<string>)
    #[serde(rename = "agentInstallerType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type_nin: Option<String>,
    /// The agent health status. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent health status (not in). Optional. (array<string>)
    #[serde(rename = "agentHealthStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_health_status_nin: Option<String>,
    /// The agent network scanner version. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent network scanner version (not in). Optional. (array<string>)
    #[serde(rename = "agentRangerVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version_nin: Option<String>,
    /// The agent network scanner status. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The agent network scanner status (not in). Optional. (array<string>)
    #[serde(rename = "agentRangerStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status_nin: Option<String>,
    /// The agent anti tampering status. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The agent anti tampering status (not in). Optional. (array<string>)
    #[serde(rename = "agentAntiTamperingStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// The agent network status. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// The agent network status (not in). Optional. (array<string>)
    #[serde(rename = "agentNetworkStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_network_status_nin: Option<String>,
    /// The agent version. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// The agent version (not in). Optional. (array<string>)
    #[serde(rename = "agentAgentVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_nin: Option<String>,
    /// The agent version (contains). Optional. (array<string>)
    #[serde(rename = "agentAgentVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_contains: Option<String>,
    /// The last logged in user. Optional. (array<string>)
    #[serde(rename = "agentLastLoggedInUser__contains", skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// Live update ID. Optional. (array<string>)
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// Match by the agent UUID. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The UUID (contains). Optional. (array<string>)
    #[serde(rename = "agentUuid__contains", skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,

    // ---- cloud ----
    /// The cloud resource ID. Optional. (array<string>)
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Optional. (array<string>)
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// The cloud provider account name (not in). Optional. (array<string>)
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The cloud provider account name. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// The cloud provider account name (contains). Optional. (array<string>)
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The cloud provider account ID (contains). Optional. (array<string>)
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// The cloud provider account id. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The cloud provider account id (not in). Optional. (array<string>)
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// The cloud provider organization unit. Optional. (array<string>)
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Optional. (array<string>)
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// The cloud provider project ID. Optional. (array<string>)
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
    /// The region. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The region (not in). Optional. (array<string>)
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    /// (array<string>)
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The instance role. Optional. (array<string>)
    #[serde(rename = "instanceRole__contains", skip_serializing_if = "Option::is_none")]
    pub instance_role_contains: Option<String>,
    /// The instance type. Optional. (array<string>)
    #[serde(rename = "instanceType__contains", skip_serializing_if = "Option::is_none")]
    pub instance_type_contains: Option<String>,
    /// The instance ID. Optional. (array<string>)
    #[serde(rename = "instanceId__contains", skip_serializing_if = "Option::is_none")]
    pub instance_id_contains: Option<String>,
    /// The image name (free-text). Optional. (array<string>)
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The image ID. Optional. (array<string>)
    #[serde(rename = "imageId__contains", skip_serializing_if = "Option::is_none")]
    pub image_id_contains: Option<String>,
    /// The virtual network ID. Optional. (array<string>)
    #[serde(rename = "virtualNetworkId__contains", skip_serializing_if = "Option::is_none")]
    pub virtual_network_id_contains: Option<String>,
    /// The network security group. Optional. (array<string>)
    #[serde(rename = "networkSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub network_security_groups_contains: Option<String>,
    /// The subnet ID. Optional. (array<string>)
    #[serde(rename = "subnetId__contains", skip_serializing_if = "Option::is_none")]
    pub subnet_id_contains: Option<String>,
    /// The subnets. Optional. (array<string>)
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,

    // ---- network / host ----
    /// The IP addresses. Optional. (array<string>)
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// The internal IPs. Optional. (array<string>)
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The gateway IPs. Optional. (array<string>)
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The gateway MACs. Optional. (array<string>)
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The MAC addresses. Optional. (array<string>)
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The hostnames. Optional. (array<string>)
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// The domain. Optional. (array<string>)
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The domain of the device. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The domain of the device (not in). Optional. (array<string>)
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,

    // ---- OS / hardware ----
    /// The operating system of the device. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// The operating system of the device (not in). Optional. (array<string>)
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// The operating system family of the device. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The operating system family of the device (not in). Optional.
    /// (array<string>)
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The operating system version of the device. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The operating system version of the device (not in). Optional.
    /// (array<string>)
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The OS versions (contains). Optional. (array<string>)
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The operating system name and version of the device. Optional.
    /// (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
    /// The operating system name and version of the device (not in). Optional.
    /// (array<string>)
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The OS names and versions (contains). Optional. (array<string>)
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The architecture of the device. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The architecture of the device (not in). Optional. (array<string>)
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// The CPU. Optional. (array<string>)
    #[serde(rename = "cpu__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// The number of cores. Optional. (array<integer>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The number of cores (not in). Optional. (array<integer>)
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The memory of the device in human readable format. Optional.
    /// (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The memory of the device in human readable format (not in). Optional.
    /// (array<string>)
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// The serial number. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The serial number (not in). Optional. (array<string>)
    #[serde(rename = "serialNumber__nin", skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// The serial number (contains). Optional. (array<string>)
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number_contains: Option<String>,

    // ---- identity / AD ----
    /// Legacy Identity Policy Name. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The legacy identity policy name (contains). Optional. (array<string>)
    #[serde(rename = "legacy_identity_policy_name__contains", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// AD machine or its groups. Optional. (array<string>)
    #[serde(rename = "identityAdMachine__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_contains: Option<String>,
    /// AD machine DN. Optional. (array<string>)
    #[serde(rename = "identityAdMachineDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// AD machine groups. Optional. (array<string>)
    #[serde(rename = "identityAdMachineMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// AD user or their groups. Optional. (array<string>)
    #[serde(rename = "identityAdUser__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_contains: Option<String>,
    /// AD user DN. Optional. (array<string>)
    #[serde(rename = "identityAdUserDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// AD user groups. Optional. (array<string>)
    #[serde(rename = "identityAdUserMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership_contains: Option<String>,
    /// Any AD string. Optional. (array<string>)
    #[serde(rename = "identityAd__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// Is AD Connector. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// Is DC Server. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// ADS Enabled. Optional. (array<boolean>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,

    // ---- kubernetes ----
    /// Running on nodes. Optional. (array<array<string>>)
    #[serde(rename = "k8sRunningOnNodes__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes_contains: Option<String>,
    /// Running on Nodes. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes: Option<String>,
    /// Policy Type (contains). Optional. (array<string>)
    #[serde(rename = "k8sPolicyType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_policy_type_contains: Option<String>,
    /// Policy Type. Optional. (array<array<string>>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_policy_type: Option<String>,
    /// Service Type (contains). Optional. (array<string>)
    #[serde(rename = "k8sServiceType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_service_type_contains: Option<String>,
    /// Service Type. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_service_type: Option<String>,
    /// The Kubernetes node. Optional. (array<string>)
    #[serde(rename = "k8sNode__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_contains: Option<String>,
    /// The Kubernetes Node. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_node: Option<String>,
    /// The Kubernetes Node (not in). Optional. (array<string>)
    #[serde(rename = "k8sNode__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_node_nin: Option<String>,
    /// Kubernetes Resource ID (contains). Optional. (array<string>)
    #[serde(rename = "k8sResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id_contains: Option<String>,
    /// The Kubernetes Resource ID. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id: Option<String>,
    /// Deployment Strategy. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_deployment_strategy: Option<String>,
    /// Deployment Strategy (contains). Optional. (array<string>)
    #[serde(rename = "k8sDeploymentStrategy__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_deployment_strategy_contains: Option<String>,
    /// Update Strategy. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_update_strategy: Option<String>,
    /// Update Strategy (contains). Optional. (array<string>)
    #[serde(rename = "k8sUpdateStrategy__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_update_strategy_contains: Option<String>,
    /// Namespace Name (contains). Optional. (array<string>)
    #[serde(rename = "k8sNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace_contains: Option<String>,
    /// The Kubernetes Namespace Name. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_namespace: Option<String>,
    /// Free-text filter by Kubernetes Labels key (supports multiple values).
    /// Optional. (array<string>)
    #[serde(rename = "k8sLabelsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_contains: Option<String>,
    /// Free-text filter by Kubernetes Labels key value (supports multiple
    /// values). Optional. (array<string>)
    #[serde(rename = "k8sLabelsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_value_contains: Option<String>,
    /// Free-text filter by Kubernetes Annotations key (supports multiple
    /// values). Optional. (array<string>)
    #[serde(rename = "k8sAnnotationsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_contains: Option<String>,
    /// Free-text filter by Kubernetes Annotations key value (supports multiple
    /// values). Optional. (array<string>)
    #[serde(rename = "k8sAnnotationsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_value_contains: Option<String>,
    /// Service Name. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_service_name: Option<String>,
    /// Service Name (contains). Optional. (array<string>)
    #[serde(rename = "k8sServiceName__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_service_name_contains: Option<String>,
    /// The Kubernetes Cluster. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_cluster: Option<String>,
    /// The Kubernetes Cluster (not in). Optional. (array<string>)
    #[serde(rename = "k8sCluster__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_nin: Option<String>,
    /// The Kubernetes cluster (contains). Optional. (array<string>)
    #[serde(rename = "k8sCluster__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_contains: Option<String>,
    /// The Kubernetes Cluster ID. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id: Option<String>,
    /// Kubernetes Cluster ID (contains). Optional. (array<string>)
    #[serde(rename = "k8sClusterId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id_contains: Option<String>,
    /// The Kubernetes Type. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_type: Option<String>,
    /// The Kubernetes Type (not in). Optional. (array<string>)
    #[serde(rename = "k8sType__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_type_nin: Option<String>,
    /// The Kubernetes Version. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k8s_version: Option<String>,
    /// The Kubernetes Version (not in). Optional. (array<string>)
    #[serde(rename = "k8sVersion__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_version_nin: Option<String>,

    // ---- asset / classification ----
    /// The criticality that each asset belongs to. Optional. Allowed values
    /// (enum): `critical`, `high`, `medium`, `low`, `--`. (array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The criticality that each asset belongs to (not in). Optional. Allowed
    /// values (enum): `critical`, `high`, `medium`, `low`, `--`. (array)
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The missing coverage for the asset. Optional. Allowed values (enum):
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. (array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// The missing coverage for the asset (not in). Optional. Allowed values
    /// (enum): `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. (array)
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The active coverage for the asset. Optional. Allowed values (enum):
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. (array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The active coverage for the asset (not in). Optional. Allowed values
    /// (enum): `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. (array)
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The risk factors associated with the asset. Optional. Allowed values
    /// (enum): `Unresolved Alerts`, `High Value`. (array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The risk factors associated with the asset (not in). Optional. Allowed
    /// values (enum): `Unresolved Alerts`, `High Value`. (array)
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// The Surface that each asset belongs to. Optional. Allowed values (enum):
    /// `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    /// (array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The Surface that each asset belongs to (not in). Optional. Allowed
    /// values (enum): `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`. (array)
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The status of the asset. Optional. Allowed values (enum): `Active`,
    /// `Inactive`. (array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The status of the asset (not in). Optional. Allowed values (enum):
    /// `Active`, `Inactive`. (array)
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The status alerts of the asset. Optional. Allowed values (enum):
    /// `Infected`, `Healthy`. (array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The status alerts of the asset (not in). Optional. Allowed values
    /// (enum): `Infected`, `Healthy`. (array)
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The asset review. Optional. Allowed values (enum): `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty). (array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// The asset review (not in). Optional. Allowed values (enum):
    /// `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, ``
    /// (empty). (array)
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The canonical name for the resource type. Optional. Allowed values
    /// (enum): `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`. (array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The canonical name for the resource type (not in). Optional. Same enum
    /// values as `resourceType`. (array)
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The Asset Type (contains). Optional. (array<string>)
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The sub-category that each resource belongs to. Optional. Allowed values
    /// (enum): `All`, `Access Key and Secret`, `Access Management`, `Account`,
    /// `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`. (array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The sub-category that each resource belongs to (not in). Optional. Same
    /// enum values as `subCategory`. (array)
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The environment that the asset exists in (not in). Optional.
    /// (array<string>)
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Asset Contact Email. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// Asset Contact Email (not in). Optional. (array<string>)
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// The name of the application installed on a workstation or a server.
    /// Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,

    // ---- identifiers / names / dates ----
    /// The ID (in). Optional. (array<string>)
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// The ID (contains). Optional. (array<string>)
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// Name. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Name (not in). Optional. (array<string>)
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The name. Optional. (array<string>)
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The last active date (between). Optional. (string)
    #[serde(rename = "lastActiveDt__between", skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// The first seen date (between). Optional. (string)
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The Last Seen date and time for the asset (between). Optional. (string)
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
}

impl ContainerFilterQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(values));
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// The column to sort the results by. See struct field docs for allowed
    /// values.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction (`asc` or `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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
    /// Export format (`csv` or `json`). Required for the export endpoint.
    pub fn export_format(mut self, v: impl Into<String>) -> Self {
        self.export_format = Some(v.into());
        self
    }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(join_csv(values));
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self {
        self.csv_filter_id = Some(n);
        self
    }

    /// Filter by IDs (`id__in`).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(join_csv(values));
        self
    }
    /// Filter by name (`name__contains`).
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(values));
        self
    }
    /// Filter by names.
    pub fn names<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join_csv(values));
        self
    }
    /// The asset criticality. Allowed: `critical`, `high`, `medium`, `low`,
    /// `--`.
    pub fn asset_criticality<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(values));
        self
    }
    /// The Surface that each asset belongs to. Allowed: `Cloud`, `Identity`,
    /// `Network`, `Endpoint`, `Network Discovery`.
    pub fn surfaces<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(join_csv(values));
        self
    }
    /// The status of the asset. Allowed: `Active`, `Inactive`.
    pub fn asset_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(join_csv(values));
        self
    }
    /// The canonical name for the resource type. See field docs for allowed
    /// values.
    pub fn resource_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(values));
        self
    }
    /// The agent operational state.
    pub fn agent_operational_state<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_operational_state = Some(join_csv(values));
        self
    }
    /// The agent version (`agentAgentVersion`).
    pub fn agent_agent_version<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_agent_version = Some(join_csv(values));
        self
    }
    /// Match by the agent UUID.
    pub fn agent_uuid<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_uuid = Some(join_csv(values));
        self
    }
    /// The region.
    pub fn region<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(values));
        self
    }
    /// The operating system of the device.
    pub fn os<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(join_csv(values));
        self
    }
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(join_csv(values));
        self
    }
    /// Tags (`tagsKeyValue`).
    pub fn tags_key_value<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(join_csv(values));
        self
    }
}

// =====================================================================
// Query params for POST .../container (scoping only)
// =====================================================================

/// Query params for `POST /web/api/v2.1/xdr/assets/container`.
///
/// The POST list endpoint accepts the inventory filter in the request **body**
/// ([`ContainerViewInputBody`]); the only query parameters are the scoping id
/// lists. All optional, serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerListScopeQuery {
    /// List of Account IDs to filter by. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Optional. (array<string>)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl ContainerListScopeQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(values));
        self
    }
}

// =====================================================================
// Request bodies
// =====================================================================

/// Request body for `POST /web/api/v2.1/xdr/assets/container`.
///
/// Maps `#/definitions/v2_1.inventory.container.schemas_ContainerViewInputSchema`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerViewInputBody {
    /// Filter. **Required.** The paginated container filter. The spec models
    /// this as the large `PaginatedContainerFilter` object; it is represented
    /// here as free-form JSON for forward-compatibility — pass the same filter
    /// keys documented on [`ContainerFilterQuery`].
    pub filter: serde_json::Value,
    /// Data. Optional/nullable. Free-form object (`EmptyStrict` in the spec).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ContainerViewInputBody {
    /// Construct a body from the (required) filter object.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { filter, data: None }
    }
    /// Set the optional `data` object.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/container/action`.
///
/// Maps
/// `#/definitions/v2_1.inventory.container.schemas_ContainerActionPayloadSchema`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerActionBody {
    /// Action name. **Required.** Allowed values (enum): `enable_cws_monitoring`,
    /// `enable_cns`, `disable_cns`, `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    pub action_name: String,
    /// List of selected inventory ids (max 5000). Optional. (array<string>)
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000). Optional.
    /// (array<string>)
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl ContainerActionBody {
    /// Construct an action body from the (required) action name.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self {
            action_name: action_name.into(),
            id_in: None,
            id_nin: None,
        }
    }
    /// List of selected inventory ids (`id__in`).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from select_all (`id__nin`).
    pub fn id_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(values.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/container/available-actions/with-status`.
///
/// Maps `#/definitions/v2_1.inventory.schemas_AffectedResourcesSchema`. All
/// fields optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResourcesBody {
    /// List of selected inventory ids (max 5000). Optional. (array<string>)
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000). Optional.
    /// (array<string>)
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl AffectedResourcesBody {
    /// List of selected inventory ids (`id__in`).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from select_all (`id__nin`).
    pub fn id_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(values.into_iter().map(Into::into).collect());
        self
    }
}

// =====================================================================
// Service methods
// =====================================================================

impl InventoryContainerService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/container` — Assets.
    ///
    /// Get assets. Returns a paginated list of inventory container assets,
    /// filtered by the supplied [`ContainerFilterQuery`].
    pub async fn list(
        &self,
        query: &ContainerFilterQuery,
    ) -> Result<Paginated<Container>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/container", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/container` — Assets using POST.
    ///
    /// POST API to get Assets. The inventory filter is supplied in the request
    /// body ([`ContainerViewInputBody`]); only the scoping id lists are passed
    /// in the query string ([`ContainerListScopeQuery`]).
    pub async fn list_post(
        &self,
        query: &ContainerListScopeQuery,
        body: &ContainerViewInputBody,
    ) -> Result<Paginated<Container>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<ContainerViewInputBody, Paginated<Container>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/container",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/container/action` — Perform action.
    ///
    /// Perform action on selected assets. The asset selection is filtered by
    /// the supplied [`ContainerFilterQuery`] and refined by the body
    /// ([`ContainerActionBody`], whose `id__in` / `id__nin` select or exclude
    /// specific assets). The action endpoint's `200` response carries no typed
    /// schema, so the data payload is returned as raw JSON.
    pub async fn perform_action(
        &self,
        query: &ContainerFilterQuery,
        body: &ContainerActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<ContainerActionBody, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/container/action",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/container/available-actions/with-status`
    /// — Available actions.
    ///
    /// Get available actions. Returns, for the selection described by the
    /// supplied [`ContainerFilterQuery`] and refined by the body
    /// ([`AffectedResourcesBody`]), the set of actions and whether each is
    /// currently enabled.
    pub async fn available_actions_with_status(
        &self,
        query: &ContainerFilterQuery,
        body: &AffectedResourcesBody,
    ) -> Result<Response<AvailableActionWithStatusResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<AffectedResourcesBody, Response<AvailableActionWithStatusResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/container/available-actions/with-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/container/export` — Export assets to CSV
    /// or JSON.
    ///
    /// Returns the results for the given inventory filter in a CSV or JSON
    /// format. `exportFormat` is **required** — set it via
    /// [`ContainerFilterQuery::export_format`] (`csv` or `json`). The response
    /// shape is not typed in the spec, so it is returned as raw JSON.
    pub async fn export(
        &self,
        query: &ContainerFilterQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/container/export", q)
            .await?)
    }
}
