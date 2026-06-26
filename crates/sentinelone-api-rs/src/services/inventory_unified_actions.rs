//! `Inventory Unified Actions` tag — unified actions across the XDR asset
//! inventory (endpoints, agents, identities, cloud resources, etc.).
//!
//! Endpoints:
//! - `POST /web/api/v2.1/xdr/assets/actions/fetch-agent-ids`
//! - `POST /web/api/v2.1/xdr/assets/actions/fetch-unified-actions`
//! - `POST /web/api/v2.1/xdr/assets/actions/perform-unified-action`
//!
//! All three endpoints accept the same large shared filter set as query params
//! (see [`InventoryUnifiedActionsQuery`]) and an entity-selection body.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_unified_actions::*;
use crate::pagination::Response;

/// `Inventory Unified Actions` tag — fetch agent ids, discover available unified
/// actions, and perform a unified action on selected assets/entities.
pub struct InventoryUnifiedActionsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Comma-join an iterator of string-like values into a single query value, as
/// the SentinelOne API expects for repeated/array query parameters.
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

/// Append a pre-serialized querystring to a path so it can be sent on a POST
/// (the HTTP client's `post` helper does not take a separate query argument;
/// `Url::join` parses the `?...` suffix as the query component).
fn path_with_query(path: &str, query: &impl Serialize) -> String {
    let qs = serde_urlencoded::to_string(query).unwrap_or_default();
    if qs.is_empty() {
        path.to_owned()
    } else {
        format!("{path}?{qs}")
    }
}

/// Query params shared by every `Inventory Unified Actions` endpoint.
///
/// All filters are optional. Array filters are serialized as a single
/// comma-separated value; their builder methods accept any iterator of
/// string-like items. Enum-typed filters are kept as `String` for
/// forward-compatibility; allowed values are documented per builder where the
/// spec provides them.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryUnifiedActionsQuery {
    /// List of Account IDs to filter by. Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The active coverage for the asset. Optional.
    #[serde(rename = "activeCoverage", skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The active coverage for the asset (not in) Optional.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// ADS Enabled. Optional.
    #[serde(rename = "adsEnabled", skip_serializing_if = "Option::is_none")]
    pub ads_enabled: Option<String>,
    /// Age Group. Optional.
    #[serde(rename = "ageGroup", skip_serializing_if = "Option::is_none")]
    pub age_group: Option<String>,
    /// The agent version. Optional.
    #[serde(rename = "agentAgentVersion", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// The agent version. Optional.
    #[serde(rename = "agentAgentVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_contains: Option<String>,
    /// The agent version (not in) Optional.
    #[serde(rename = "agentAgentVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_agent_version_nin: Option<String>,
    /// The agent anti tampering status. Optional.
    #[serde(rename = "agentAntiTamperingStatus", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The agent anti tampering status (not in) Optional.
    #[serde(rename = "agentAntiTamperingStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status_nin: Option<String>,
    /// Whether the agent can configure network quarantine. Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine: Option<String>,
    /// Whether the agent can configure network quarantine (not in) Optional.
    #[serde(rename = "agentConfigurableNetworkQuarantine__nin", skip_serializing_if = "Option::is_none")]
    pub agent_configurable_network_quarantine_nin: Option<String>,
    /// The agent console connectivity. Optional.
    #[serde(rename = "agentConsoleConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent console migration status. Optional.
    #[serde(rename = "agentConsoleMigrationStatus", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status: Option<String>,
    /// The agent console migration status (not in) Optional.
    #[serde(rename = "agentConsoleMigrationStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_console_migration_status_nin: Option<String>,
    /// The customer identifier. Optional.
    #[serde(rename = "agentCustomerIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub agent_customer_identifier_contains: Option<String>,
    /// Whether the agent is decommissioned. Optional.
    #[serde(rename = "agentDecommissioned", skip_serializing_if = "Option::is_none")]
    pub agent_decommissioned: Option<String>,
    /// The agent disk encryption. Optional.
    #[serde(rename = "agentDiskEncryption", skip_serializing_if = "Option::is_none")]
    pub agent_disk_encryption: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_between: Option<String>,
    /// The agent free disk percentage on any of the disks. Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__gte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_gte: Option<f64>,
    /// The agent free disk percentage on any of the disks. Optional.
    #[serde(rename = "agentDiskMetricsFreePercentage__lte", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_free_percentage_lte: Option<f64>,
    /// The agent disk metrics volume type. Optional.
    #[serde(rename = "agentDiskMetricsVolumeType", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type: Option<String>,
    /// The agent disk metrics volume type (not in) Optional.
    #[serde(rename = "agentDiskMetricsVolumeType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_disk_metrics_volume_type_nin: Option<String>,
    /// The connection status between the agent and the SDL service. Optional.
    #[serde(rename = "agentDvConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity: Option<String>,
    /// The SDL connectivity last active. Optional.
    #[serde(rename = "agentDvConnectivityLastUpdatedDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_last_updated_dt_between: Option<String>,
    /// The connection status between the agent and the SDL service (not in) Optional.
    #[serde(rename = "agentDvConnectivity__nin", skip_serializing_if = "Option::is_none")]
    pub agent_dv_connectivity_nin: Option<String>,
    /// The agent full disk scan date. Optional.
    #[serde(rename = "agentFullDiskScanDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_full_disk_scan_dt_between: Option<String>,
    /// Whether the agent has local configuration. Optional.
    #[serde(rename = "agentHasLocalConfig", skip_serializing_if = "Option::is_none")]
    pub agent_has_local_config: Option<String>,
    /// The agent health status. Optional.
    #[serde(rename = "agentHealthStatus", skip_serializing_if = "Option::is_none")]
    pub agent_health_status: Option<String>,
    /// The agent health status (not in) Optional.
    #[serde(rename = "agentHealthStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_health_status_nin: Option<String>,
    /// The agent Idr connectivity. Optional.
    #[serde(rename = "agentIdrConnectivity", skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
    /// The agent installer type. Optional.
    #[serde(rename = "agentInstallerType", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type: Option<String>,
    /// The agent installer type (not in) Optional.
    #[serde(rename = "agentInstallerType__nin", skip_serializing_if = "Option::is_none")]
    pub agent_installer_type_nin: Option<String>,
    /// The last logged in user. Optional.
    #[serde(rename = "agentLastLoggedInUser__contains", skip_serializing_if = "Option::is_none")]
    pub agent_last_logged_in_user_contains: Option<String>,
    /// The agent location. Optional.
    #[serde(rename = "agentLocation", skip_serializing_if = "Option::is_none")]
    pub agent_location: Option<String>,
    /// The location awareness. Optional.
    #[serde(rename = "agentLocationAwareness__contains", skip_serializing_if = "Option::is_none")]
    pub agent_location_awareness_contains: Option<String>,
    /// The agent location (not in) Optional.
    #[serde(rename = "agentLocation__nin", skip_serializing_if = "Option::is_none")]
    pub agent_location_nin: Option<String>,
    /// The agent missing permissions. Optional.
    #[serde(rename = "agentMissingPermissions", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions: Option<String>,
    /// The agent missing permissions (not in) Optional.
    #[serde(rename = "agentMissingPermissions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_missing_permissions_nin: Option<String>,
    /// The agent network status. Optional.
    #[serde(rename = "agentNetworkStatus", skip_serializing_if = "Option::is_none")]
    pub agent_network_status: Option<String>,
    /// The agent network status (not in) Optional.
    #[serde(rename = "agentNetworkStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_network_status_nin: Option<String>,
    /// The agent operational state. Optional.
    #[serde(rename = "agentOperationalState", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// The agent operational state (not in) Optional.
    #[serde(rename = "agentOperationalState__nin", skip_serializing_if = "Option::is_none")]
    pub agent_operational_state_nin: Option<String>,
    /// The agent pending actions. Optional.
    #[serde(rename = "agentPendingActions", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The agent pending actions (not in) Optional.
    #[serde(rename = "agentPendingActions__nin", skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions_nin: Option<String>,
    /// Whether the agent is pending uninstall. Optional.
    #[serde(rename = "agentPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub agent_pending_uninstall: Option<String>,
    /// Whether the agent is pending upgrade. Optional.
    #[serde(rename = "agentPendingUpgrade", skip_serializing_if = "Option::is_none")]
    pub agent_pending_upgrade: Option<String>,
    /// The agent network scanner status. Optional.
    #[serde(rename = "agentRangerStatus", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status: Option<String>,
    /// The agent network scanner status (not in) Optional.
    #[serde(rename = "agentRangerStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_status_nin: Option<String>,
    /// The agent network scanner version. Optional.
    #[serde(rename = "agentRangerVersion", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version: Option<String>,
    /// The agent network scanner version (not in) Optional.
    #[serde(rename = "agentRangerVersion__nin", skip_serializing_if = "Option::is_none")]
    pub agent_ranger_version_nin: Option<String>,
    /// Live update ID. Optional.
    #[serde(rename = "agentS1AgentLiveUpdatesVersion__contains", skip_serializing_if = "Option::is_none")]
    pub agent_s1_agent_live_updates_version_contains: Option<String>,
    /// The agent subscribe time. Optional.
    #[serde(rename = "agentSubscribeOnDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_subscribe_on_dt_between: Option<String>,
    /// Whether the agent is uninstalled. Optional.
    #[serde(rename = "agentUninstalled", skip_serializing_if = "Option::is_none")]
    pub agent_uninstalled: Option<String>,
    /// Match by the agent UUID. Optional.
    #[serde(rename = "agentUuid", skip_serializing_if = "Option::is_none")]
    pub agent_uuid: Option<String>,
    /// The UUID. Optional.
    #[serde(rename = "agentUuid__contains", skip_serializing_if = "Option::is_none")]
    pub agent_uuid_contains: Option<String>,
    /// The agent VSS last snapshot date. Optional.
    #[serde(rename = "agentVssLastSnapshotDt__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_last_snapshot_dt_between: Option<String>,
    /// The agent VSS protection status. Optional.
    #[serde(rename = "agentVssProtectionStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status: Option<String>,
    /// The agent VSS protection status (not in) Optional.
    #[serde(rename = "agentVssProtectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_protection_status_nin: Option<String>,
    /// The agent VSS rollback status. Optional.
    #[serde(rename = "agentVssRollbackStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// The agent VSS rollback status (not in) Optional.
    #[serde(rename = "agentVssRollbackStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status_nin: Option<String>,
    /// The agent VSS service status. Optional.
    #[serde(rename = "agentVssServiceStatus", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status: Option<String>,
    /// The agent VSS service status (not in) Optional.
    #[serde(rename = "agentVssServiceStatus__nin", skip_serializing_if = "Option::is_none")]
    pub agent_vss_service_status_nin: Option<String>,
    /// The agent free VSS volume percentage on any of the volumes. Optional.
    #[serde(rename = "agentVssVolumesDiffAreaFreePercentage__between", skip_serializing_if = "Option::is_none")]
    pub agent_vss_volumes_diff_area_free_percentage_between: Option<String>,
    /// The severity of the alert. Optional.
    #[serde(rename = "alertSeverity", skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(rename = "allTagsKey", skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(rename = "allTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// User and cloud tags (not in) Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// User and cloud tag keys (not in) Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The name of the application installed on a workstation or a server. Optional.
    #[serde(rename = "applicationName", skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    /// The architecture of the device. Optional.
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The architecture of the device (not in) Optional.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(rename = "assetContactEmail", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// Asset Contact Email (not in) Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The criticality that each asset belongs to. Optional.
    #[serde(rename = "assetCriticality", skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The criticality that each asset belongs to (not in) Optional.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The environment that the asset exists in - AWS \| Azure \| GCP \| Active Directory. Optional.
    #[serde(rename = "assetEnvironment", skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The environment that the asset exists in - AWS \| Azure \| GCP \| Active Directory (not in) Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// The status of the asset. Optional.
    #[serde(rename = "assetStatus", skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The status of the asset (not in) Optional.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// Bad Password Time. Optional.
    #[serde(rename = "badPasswordTime__between", skip_serializing_if = "Option::is_none")]
    pub bad_password_time_between: Option<String>,
    /// Classification. Optional.
    #[serde(rename = "classification", skip_serializing_if = "Option::is_none")]
    pub classification: Option<String>,
    /// The cloud provider account id. Optional.
    #[serde(rename = "cloudProviderAccountId", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The cloud provider account ID. Optional.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// The cloud provider account id (not in) Optional.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(rename = "cloudProviderAccountName", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The cloud provider account name (not in) Optional.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(rename = "cloudTagsKey", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The cloud tags key value. Optional.
    #[serde(rename = "cloudTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values) Optional.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The cloud tags key value (not in) Optional.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values) Optional.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// The cloud tags key (not in) Optional.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// The LDAP Common Name. Optional.
    #[serde(rename = "cn", skip_serializing_if = "Option::is_none")]
    pub cn: Option<String>,
    /// The LDAP Common Name. Optional.
    #[serde(rename = "cn__contains", skip_serializing_if = "Option::is_none")]
    pub cn_contains: Option<String>,
    /// The LDAP Common Name (not in) Optional.
    #[serde(rename = "cn__nin", skip_serializing_if = "Option::is_none")]
    pub cn_nin: Option<String>,
    /// The number of cores. Optional.
    #[serde(rename = "coreCount", skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The number of cores (not in) Optional.
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The columns for which filter count would be returned for. Optional.
    #[serde(rename = "countsFor", skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The CPU. Optional.
    #[serde(rename = "cpu__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_contains: Option<String>,
    /// Whether the AD Entity is deleted or not. Optional.
    #[serde(rename = "deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
    /// Deleted Time. Optional.
    #[serde(rename = "deletedTime__between", skip_serializing_if = "Option::is_none")]
    pub deleted_time_between: Option<String>,
    /// Department. Optional.
    #[serde(rename = "department", skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// The site from which the device was detected. Optional.
    #[serde(rename = "detectedFromSite", skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// The site from which the device was detected (not in) Optional.
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site_nin: Option<String>,
    /// The asset review. Optional.
    #[serde(rename = "deviceReview", skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// The asset review (not in) Optional.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The discovery methods. Optional.
    #[serde(rename = "discoveryMethods", skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The discovery methods (not in) Optional.
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods_nin: Option<String>,
    /// The Display Name. Optional.
    #[serde(rename = "displayName__contains", skip_serializing_if = "Option::is_none")]
    pub display_name_contains: Option<String>,
    /// The Distinguished Name. Optional.
    #[serde(rename = "distinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub distinguished_name_contains: Option<String>,
    /// The domain of the device. Optional.
    #[serde(rename = "domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The domain. Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The domain of the device (not in) Optional.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// Employee Type. Optional.
    #[serde(rename = "employeeType", skip_serializing_if = "Option::is_none")]
    pub employee_type: Option<String>,
    /// Whether the Identity Group is enabled or not. Optional.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    /// The encryption type. Optional.
    #[serde(rename = "encryptionType", skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// The encryption type (not in) Optional.
    #[serde(rename = "encryptionType__nin", skip_serializing_if = "Option::is_none")]
    pub encryption_type_nin: Option<String>,
    /// Entra ID Group Type. Optional.
    #[serde(rename = "entraidGroupType", skip_serializing_if = "Option::is_none")]
    pub entraid_group_type: Option<String>,
    /// Entra ID Group Type (not in) Optional.
    #[serde(rename = "entraidGroupType__nin", skip_serializing_if = "Option::is_none")]
    pub entraid_group_type_nin: Option<String>,
    /// The agent supported or unknown state. Optional.
    #[serde(rename = "eppUnsupportedUnknown", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The agent supported or unknown state (not in) Optional.
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown_nin: Option<String>,
    /// Expiration Time. Optional.
    #[serde(rename = "expirationTime__between", skip_serializing_if = "Option::is_none")]
    pub expiration_time_between: Option<String>,
    /// The first seen date. Optional.
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The Forest Name. Optional.
    #[serde(rename = "forest", skip_serializing_if = "Option::is_none")]
    pub forest: Option<String>,
    /// The Forest Name. Optional.
    #[serde(rename = "forest__contains", skip_serializing_if = "Option::is_none")]
    pub forest_contains: Option<String>,
    /// The Forest Name (not in) Optional.
    #[serde(rename = "forest__nin", skip_serializing_if = "Option::is_none")]
    pub forest_nin: Option<String>,
    /// The gateway IPs. Optional.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The gateway MACs. Optional.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// Given Name. Optional.
    #[serde(rename = "givenName", skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// Given Name. Optional.
    #[serde(rename = "givenName__contains", skip_serializing_if = "Option::is_none")]
    pub given_name_contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The Group Type. Optional.
    #[serde(rename = "groupType", skip_serializing_if = "Option::is_none")]
    pub group_type: Option<String>,
    /// The Group Type (not in) Optional.
    #[serde(rename = "groupType__nin", skip_serializing_if = "Option::is_none")]
    pub group_type_nin: Option<String>,
    /// The hostnames. Optional.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// AD machine DN. Optional.
    #[serde(rename = "identityAdMachineDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_distinguished_name_contains: Option<String>,
    /// AD machine groups. Optional.
    #[serde(rename = "identityAdMachineMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_membership_contains: Option<String>,
    /// AD machine or its groups. Optional.
    #[serde(rename = "identityAdMachine__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_machine_contains: Option<String>,
    /// AD user DN. Optional.
    #[serde(rename = "identityAdUserDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_distinguished_name_contains: Option<String>,
    /// AD user groups. Optional.
    #[serde(rename = "identityAdUserMembership__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_membership_contains: Option<String>,
    /// AD user or their groups. Optional.
    #[serde(rename = "identityAdUser__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_user_contains: Option<String>,
    /// Any AD string. Optional.
    #[serde(rename = "identityAd__contains", skip_serializing_if = "Option::is_none")]
    pub identity_ad_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The status alerts of the asset. Optional.
    #[serde(rename = "infectionStatus", skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The status alerts of the asset (not in) Optional.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The internal IPs. Optional.
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The IP addresses. Optional.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Is AD Connector. Optional.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// Is DC Server. Optional.
    #[serde(rename = "isDcServer", skip_serializing_if = "Option::is_none")]
    pub is_dc_server: Option<String>,
    /// Whether the instance is a rogue or not. Optional.
    #[serde(rename = "isRogues", skip_serializing_if = "Option::is_none")]
    pub is_rogues: Option<String>,
    /// Job Title. Optional.
    #[serde(rename = "jobTitle", skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    /// Free-text filter by Kubernetes Annotations key value (supports multiple values) Optional.
    #[serde(rename = "k8sAnnotationsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_value_contains: Option<String>,
    /// Free-text filter by Kubernetes Annotations key (supports multiple values) Optional.
    #[serde(rename = "k8sAnnotationsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_annotations_unified_key_contains: Option<String>,
    /// The Kubernetes Cluster. Optional.
    #[serde(rename = "k8sCluster", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster: Option<String>,
    /// The Kubernetes Cluster ID. Optional.
    #[serde(rename = "k8sClusterId", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id: Option<String>,
    /// Kubernetes Cluster ID. Optional.
    #[serde(rename = "k8sClusterId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_id_contains: Option<String>,
    /// The Kubernetes cluster. Optional.
    #[serde(rename = "k8sCluster__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_contains: Option<String>,
    /// The Kubernetes Cluster (not in) Optional.
    #[serde(rename = "k8sCluster__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_cluster_nin: Option<String>,
    /// Free-text filter by Kubernetes Labels key value (supports multiple values) Optional.
    #[serde(rename = "k8sLabelsUnifiedKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_value_contains: Option<String>,
    /// Free-text filter by Kubernetes Labels key (supports multiple values) Optional.
    #[serde(rename = "k8sLabelsUnifiedKey__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_labels_unified_key_contains: Option<String>,
    /// The Kubernetes Namespace Name. Optional.
    #[serde(rename = "k8sNamespace", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace: Option<String>,
    /// Namespace Name. Optional.
    #[serde(rename = "k8sNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_namespace_contains: Option<String>,
    /// The Kubernetes Node. Optional.
    #[serde(rename = "k8sNode", skip_serializing_if = "Option::is_none")]
    pub k8s_node: Option<String>,
    /// The Kubernetes node. Optional.
    #[serde(rename = "k8sNode__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_contains: Option<String>,
    /// The Kubernetes Node (not in) Optional.
    #[serde(rename = "k8sNode__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_node_nin: Option<String>,
    /// The Kubernetes Resource ID. Optional.
    #[serde(rename = "k8sResourceId", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id: Option<String>,
    /// Kubernetes Resource ID. Optional.
    #[serde(rename = "k8sResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_resource_id_contains: Option<String>,
    /// Running on Nodes. Optional.
    #[serde(rename = "k8sRunningOnNodes", skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes: Option<String>,
    /// Running on nodes. Optional.
    #[serde(rename = "k8sRunningOnNodes__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_running_on_nodes_contains: Option<String>,
    /// The Kubernetes Type. Optional.
    #[serde(rename = "k8sType", skip_serializing_if = "Option::is_none")]
    pub k8s_type: Option<String>,
    /// The Kubernetes Type (not in) Optional.
    #[serde(rename = "k8sType__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_type_nin: Option<String>,
    /// The Kubernetes Version. Optional.
    #[serde(rename = "k8sVersion", skip_serializing_if = "Option::is_none")]
    pub k8s_version: Option<String>,
    /// The Kubernetes Version (not in) Optional.
    #[serde(rename = "k8sVersion__nin", skip_serializing_if = "Option::is_none")]
    pub k8s_version_nin: Option<String>,
    /// The last active date. Optional.
    #[serde(rename = "lastActiveDt__between", skip_serializing_if = "Option::is_none")]
    pub last_active_dt_between: Option<String>,
    /// The Last time of User Login. Optional.
    #[serde(rename = "lastLogonTime__between", skip_serializing_if = "Option::is_none")]
    pub last_logon_time_between: Option<String>,
    /// The last update date. Optional.
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt_between: Option<String>,
    /// Lock-out Time. Optional.
    #[serde(rename = "last_modified_time__between", skip_serializing_if = "Option::is_none")]
    pub last_modified_time_between: Option<String>,
    /// Legacy Identity Policy Name. Optional.
    #[serde(rename = "legacyIdentityPolicyName", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name: Option<String>,
    /// The legacy identity policy name. Optional.
    #[serde(rename = "legacy_identity_policy_name__contains", skip_serializing_if = "Option::is_none")]
    pub legacy_identity_policy_name_contains: Option<String>,
    /// Lock-out Time. Optional.
    #[serde(rename = "lockOutTime__between", skip_serializing_if = "Option::is_none")]
    pub lock_out_time_between: Option<String>,
    /// The MAC addresses. Optional.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The Email Address. Optional.
    #[serde(rename = "mail", skip_serializing_if = "Option::is_none")]
    pub mail: Option<String>,
    /// The Email Address. Optional.
    #[serde(rename = "mail__contains", skip_serializing_if = "Option::is_none")]
    pub mail_contains: Option<String>,
    /// The manufacturer of the device. Optional.
    #[serde(rename = "manufacturer", skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// The manufacturer. Optional.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// The manufacturer of the device (not in) Optional.
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer_nin: Option<String>,
    /// The memory of the device in human readable format. Optional.
    #[serde(rename = "memoryReadable", skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The memory of the device in human readable format (not in) Optional.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// The missing coverage for the asset. Optional.
    #[serde(rename = "missingCoverage", skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// The missing coverage for the asset (not in) Optional.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// Name. Optional.
    #[serde(rename = "names", skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Name (not in) Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The network name. Optional.
    #[serde(rename = "networkName", skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// The network name (not in) Optional.
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name_nin: Option<String>,
    /// The Object Category. Optional.
    #[serde(rename = "objectCategory", skip_serializing_if = "Option::is_none")]
    pub object_category: Option<String>,
    /// The Object Category (not in) Optional.
    #[serde(rename = "objectCategory__nin", skip_serializing_if = "Option::is_none")]
    pub object_category_nin: Option<String>,
    /// The Object Class. Optional.
    #[serde(rename = "objectClass", skip_serializing_if = "Option::is_none")]
    pub object_class: Option<String>,
    /// The Object Class (not in) Optional.
    #[serde(rename = "objectClass__nin", skip_serializing_if = "Option::is_none")]
    pub object_class_nin: Option<String>,
    /// The number of objects in the bucket. Optional.
    #[serde(rename = "objectCount", skip_serializing_if = "Option::is_none")]
    pub object_count: Option<String>,
    /// The number of objects in the bucket (not in) Optional.
    #[serde(rename = "objectCount__nin", skip_serializing_if = "Option::is_none")]
    pub object_count_nin: Option<String>,
    /// The Object GUID. Optional.
    #[serde(rename = "objectGuid__contains", skip_serializing_if = "Option::is_none")]
    pub object_guid_contains: Option<String>,
    /// The Object SID. Optional.
    #[serde(rename = "objectSid__contains", skip_serializing_if = "Option::is_none")]
    pub object_sid_contains: Option<String>,
    /// On Premises Distinguished Name. Optional.
    #[serde(rename = "onPremisesDistinguishedName", skip_serializing_if = "Option::is_none")]
    pub on_premises_distinguished_name: Option<String>,
    /// onPremisesDistinguishedName__contains. Optional.
    #[serde(rename = "onPremisesDistinguishedName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_distinguished_name_contains: Option<String>,
    /// On Premises Domain Name. Optional.
    #[serde(rename = "onPremisesDomainName", skip_serializing_if = "Option::is_none")]
    pub on_premises_domain_name: Option<String>,
    /// On Premises Distinguished Name. Optional.
    #[serde(rename = "onPremisesDomainName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_domain_name_contains: Option<String>,
    /// On Premises Immutable Id. Optional.
    #[serde(rename = "onPremisesImmutableId", skip_serializing_if = "Option::is_none")]
    pub on_premises_immutable_id: Option<String>,
    /// On Premises Last Sync Time. Optional.
    #[serde(rename = "onPremisesLastSyncTime__between", skip_serializing_if = "Option::is_none")]
    pub on_premises_last_sync_time_between: Option<String>,
    /// On Premises SAM Account Name. Optional.
    #[serde(rename = "onPremisesSamAccountName", skip_serializing_if = "Option::is_none")]
    pub on_premises_sam_account_name: Option<String>,
    /// On Premises SAM Account Name. Optional.
    #[serde(rename = "onPremisesSamAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_sam_account_name_contains: Option<String>,
    /// On Premises Security Identifier. Optional.
    #[serde(rename = "onPremisesSecurityIdentifier", skip_serializing_if = "Option::is_none")]
    pub on_premises_security_identifier: Option<String>,
    /// onPremisesSecurityIdentifier__contains. Optional.
    #[serde(rename = "onPremisesSecurityIdentifier__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_security_identifier_contains: Option<String>,
    /// On Premises Sync Enabled. Optional.
    #[serde(rename = "onPremisesSyncEnabled", skip_serializing_if = "Option::is_none")]
    pub on_premises_sync_enabled: Option<String>,
    /// On Premises User Principal Name. Optional.
    #[serde(rename = "onPremisesUserPrincipalName", skip_serializing_if = "Option::is_none")]
    pub on_premises_user_principal_name: Option<String>,
    /// On Premises User Principal Name. Optional.
    #[serde(rename = "onPremisesUserPrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub on_premises_user_principal_name_contains: Option<String>,
    /// The operating system of the device. Optional.
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// The operating system family of the device. Optional.
    #[serde(rename = "osFamily", skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The operating system family of the device (not in) Optional.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The operating system name and version of the device. Optional.
    #[serde(rename = "osNameVersion", skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
    /// The OS names and versions. Optional.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The operating system name and version of the device (not in) Optional.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The operating system version of the device. Optional.
    #[serde(rename = "osVersion", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The OS versions. Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The operating system version of the device (not in) Optional.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The operating system of the device (not in) Optional.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// Other Mails. Optional.
    #[serde(rename = "otherMails", skip_serializing_if = "Option::is_none")]
    pub other_mails: Option<String>,
    /// Other Mails. Optional.
    #[serde(rename = "otherMails__contains", skip_serializing_if = "Option::is_none")]
    pub other_mails_contains: Option<String>,
    /// Whether the password never expires. Optional.
    #[serde(rename = "passwordNeverExpire", skip_serializing_if = "Option::is_none")]
    pub password_never_expire: Option<String>,
    /// Whether the AD Entity is privileged or not. Optional.
    #[serde(rename = "privileged", skip_serializing_if = "Option::is_none")]
    pub privileged: Option<String>,
    /// Proxy Addresses. Optional.
    #[serde(rename = "proxyAddresses", skip_serializing_if = "Option::is_none")]
    pub proxy_addresses: Option<String>,
    /// Proxy Addresses. Optional.
    #[serde(rename = "proxyAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub proxy_addresses_contains: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values) Optional.
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value_contains: Option<String>,
    /// Free-text filter by Ranger tag key (supports multiple values) Optional.
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_contains: Option<String>,
    /// The ranger tags key. Optional.
    #[serde(rename = "rangerTagsKey", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The ranger tags key value. Optional.
    #[serde(rename = "rangerTagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// The ranger tags key value (not in) Optional.
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value_nin: Option<String>,
    /// The ranger tags key (not in) Optional.
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_nin: Option<String>,
    /// The region. Optional.
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The region (not in) Optional.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// The canonical name for the resource type. Optional.
    #[serde(rename = "resourceType", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The Asset Type. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The canonical name for the resource type (not in) Optional.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The risk factors associated with the asset. Optional.
    #[serde(rename = "riskFactors", skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The risk factors associated with the asset (not in) Optional.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The SAM Account Name. Optional.
    #[serde(rename = "samAccountName", skip_serializing_if = "Option::is_none")]
    pub sam_account_name: Option<String>,
    /// The SAM Account Name (not in) Optional.
    #[serde(rename = "samAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub sam_account_name_nin: Option<String>,
    /// The CDS malware scan status. Optional.
    #[serde(rename = "scanStatus", skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// The CDS malware scan status. Optional.
    #[serde(rename = "scanStatus__contains", skip_serializing_if = "Option::is_none")]
    pub scan_status_contains: Option<String>,
    /// The CDS malware scan status (not in) Optional.
    #[serde(rename = "scanStatus__nin", skip_serializing_if = "Option::is_none")]
    pub scan_status_nin: Option<String>,
    /// Security Enabled. Optional.
    #[serde(rename = "securityEnabled", skip_serializing_if = "Option::is_none")]
    pub security_enabled: Option<String>,
    /// The serial number. Optional.
    #[serde(rename = "serialNumber", skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// The serial number. Optional.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number_contains: Option<String>,
    /// The serial number (not in) Optional.
    #[serde(rename = "serialNumber__nin", skip_serializing_if = "Option::is_none")]
    pub serial_number_nin: Option<String>,
    /// The Service Account. Optional.
    #[serde(rename = "serviceAccount", skip_serializing_if = "Option::is_none")]
    pub service_account: Option<String>,
    /// The Service Principal Name. Optional.
    #[serde(rename = "servicePrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub service_principal_name_contains: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The state of the instance. Optional.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The state. Optional.
    #[serde(rename = "state__contains", skip_serializing_if = "Option::is_none")]
    pub state_contains: Option<String>,
    /// The state of the instance (not in) Optional.
    #[serde(rename = "state__nin", skip_serializing_if = "Option::is_none")]
    pub state_nin: Option<String>,
    /// The sub-category that each resource belongs to. Optional.
    #[serde(rename = "subCategory", skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The sub-category that each resource belongs to (not in) Optional.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The subnets. Optional.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The Surface that each asset belongs to. Optional.
    #[serde(rename = "surfaces", skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The Surface that each asset belongs to (not in) Optional.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// Surname. Optional.
    #[serde(rename = "surname", skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    /// Tag Keys. Optional.
    #[serde(rename = "tagsKey", skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// Tags. Optional.
    #[serde(rename = "tagsKeyValue", skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Free-text filter by tag key value (supports multiple values) Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tags (not in) Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// Free-text filter by tag key (supports multiple values) Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// Tag Keys (not in) Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The TCP ports. Optional.
    #[serde(rename = "tcpPorts", skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// Threat Detection Policy. Optional.
    #[serde(rename = "threatDetectionPolicyStatus", skip_serializing_if = "Option::is_none")]
    pub threat_detection_policy_status: Option<String>,
    /// Threat Detection Status. Optional.
    #[serde(rename = "threatDetectionStatus", skip_serializing_if = "Option::is_none")]
    pub threat_detection_status: Option<String>,
    /// The threat detection status. Optional.
    #[serde(rename = "threatDetectionStatus__contains", skip_serializing_if = "Option::is_none")]
    pub threat_detection_status_contains: Option<String>,
    /// Threat Detection Status (not in) Optional.
    #[serde(rename = "threatDetectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub threat_detection_status_nin: Option<String>,
    /// The UDP ports. Optional.
    #[serde(rename = "udpPorts", skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// Unique Name. Optional.
    #[serde(rename = "uniqueName", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The User Account Control. Optional.
    #[serde(rename = "userAccountControl", skip_serializing_if = "Option::is_none")]
    pub user_account_control: Option<String>,
    /// The User Account Control (not in) Optional.
    #[serde(rename = "userAccountControl__nin", skip_serializing_if = "Option::is_none")]
    pub user_account_control_nin: Option<String>,
    /// The User Password Expiry Time. Optional.
    #[serde(rename = "userPasswordExpiryTimeComputed__between", skip_serializing_if = "Option::is_none")]
    pub user_password_expiry_time_computed_between: Option<String>,
    /// The User Principal Name. Optional.
    #[serde(rename = "userPrincipalName", skip_serializing_if = "Option::is_none")]
    pub user_principal_name: Option<String>,
    /// The User Principal Name. Optional.
    #[serde(rename = "userPrincipalName__contains", skip_serializing_if = "Option::is_none")]
    pub user_principal_name_contains: Option<String>,
    /// The User Principal Name (not in) Optional.
    #[serde(rename = "userPrincipalName__nin", skip_serializing_if = "Option::is_none")]
    pub user_principal_name_nin: Option<String>,
    /// Visibility. Optional.
    #[serde(rename = "visibility", skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl InventoryUnifiedActionsQuery {
    /// List of Account IDs to filter by
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.account_ids = Some(join_csv(v)); self }
    /// The active coverage for the asset
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.active_coverage = Some(join_csv(v)); self }
    /// The active coverage for the asset (not in)
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.active_coverage_nin = Some(join_csv(v)); self }
    /// ADS Enabled
    pub fn ads_enabled<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.ads_enabled = Some(join_csv(v)); self }
    /// Age Group
    pub fn age_group<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.age_group = Some(join_csv(v)); self }
    /// The agent version
    pub fn agent_agent_version<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_agent_version = Some(join_csv(v)); self }
    /// The agent version
    pub fn agent_agent_version_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_agent_version_contains = Some(join_csv(v)); self }
    /// The agent version (not in)
    pub fn agent_agent_version_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_agent_version_nin = Some(join_csv(v)); self }
    /// The agent anti tampering status
    pub fn agent_anti_tampering_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_anti_tampering_status = Some(join_csv(v)); self }
    /// The agent anti tampering status (not in)
    pub fn agent_anti_tampering_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_anti_tampering_status_nin = Some(join_csv(v)); self }
    /// Whether the agent can configure network quarantine
    pub fn agent_configurable_network_quarantine<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_configurable_network_quarantine = Some(join_csv(v)); self }
    /// Whether the agent can configure network quarantine (not in)
    pub fn agent_configurable_network_quarantine_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_configurable_network_quarantine_nin = Some(join_csv(v)); self }
    /// The agent console connectivity
    pub fn agent_console_connectivity<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_console_connectivity = Some(join_csv(v)); self }
    /// The agent console migration status
    pub fn agent_console_migration_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_console_migration_status = Some(join_csv(v)); self }
    /// The agent console migration status (not in)
    pub fn agent_console_migration_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_console_migration_status_nin = Some(join_csv(v)); self }
    /// The customer identifier
    pub fn agent_customer_identifier_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_customer_identifier_contains = Some(join_csv(v)); self }
    /// Whether the agent is decommissioned
    pub fn agent_decommissioned<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_decommissioned = Some(join_csv(v)); self }
    /// The agent disk encryption
    pub fn agent_disk_encryption<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_disk_encryption = Some(join_csv(v)); self }
    /// The agent free disk percentage on any of the disks
    pub fn agent_disk_metrics_free_percentage_between(mut self, v: impl Into<String>) -> Self { self.agent_disk_metrics_free_percentage_between = Some(v.into()); self }
    /// The agent free disk percentage on any of the disks
    pub fn agent_disk_metrics_free_percentage_gte(mut self, v: f64) -> Self { self.agent_disk_metrics_free_percentage_gte = Some(v); self }
    /// The agent free disk percentage on any of the disks
    pub fn agent_disk_metrics_free_percentage_lte(mut self, v: f64) -> Self { self.agent_disk_metrics_free_percentage_lte = Some(v); self }
    /// The agent disk metrics volume type
    pub fn agent_disk_metrics_volume_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_disk_metrics_volume_type = Some(join_csv(v)); self }
    /// The agent disk metrics volume type (not in)
    pub fn agent_disk_metrics_volume_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_disk_metrics_volume_type_nin = Some(join_csv(v)); self }
    /// The connection status between the agent and the SDL service
    pub fn agent_dv_connectivity<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_dv_connectivity = Some(join_csv(v)); self }
    /// The SDL connectivity last active
    pub fn agent_dv_connectivity_last_updated_dt_between(mut self, v: impl Into<String>) -> Self { self.agent_dv_connectivity_last_updated_dt_between = Some(v.into()); self }
    /// The connection status between the agent and the SDL service (not in)
    pub fn agent_dv_connectivity_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_dv_connectivity_nin = Some(join_csv(v)); self }
    /// The agent full disk scan date
    pub fn agent_full_disk_scan_dt_between(mut self, v: impl Into<String>) -> Self { self.agent_full_disk_scan_dt_between = Some(v.into()); self }
    /// Whether the agent has local configuration
    pub fn agent_has_local_config<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_has_local_config = Some(join_csv(v)); self }
    /// The agent health status
    pub fn agent_health_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_health_status = Some(join_csv(v)); self }
    /// The agent health status (not in)
    pub fn agent_health_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_health_status_nin = Some(join_csv(v)); self }
    /// The agent Idr connectivity
    pub fn agent_idr_connectivity<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_idr_connectivity = Some(join_csv(v)); self }
    /// The agent installer type
    pub fn agent_installer_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_installer_type = Some(join_csv(v)); self }
    /// The agent installer type (not in)
    pub fn agent_installer_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_installer_type_nin = Some(join_csv(v)); self }
    /// The last logged in user
    pub fn agent_last_logged_in_user_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_last_logged_in_user_contains = Some(join_csv(v)); self }
    /// The agent location
    pub fn agent_location<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_location = Some(join_csv(v)); self }
    /// The location awareness
    pub fn agent_location_awareness_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_location_awareness_contains = Some(join_csv(v)); self }
    /// The agent location (not in)
    pub fn agent_location_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_location_nin = Some(join_csv(v)); self }
    /// The agent missing permissions
    pub fn agent_missing_permissions<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_missing_permissions = Some(join_csv(v)); self }
    /// The agent missing permissions (not in)
    pub fn agent_missing_permissions_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_missing_permissions_nin = Some(join_csv(v)); self }
    /// The agent network status
    pub fn agent_network_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_network_status = Some(join_csv(v)); self }
    /// The agent network status (not in)
    pub fn agent_network_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_network_status_nin = Some(join_csv(v)); self }
    /// The agent operational state
    pub fn agent_operational_state<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_operational_state = Some(join_csv(v)); self }
    /// The agent operational state (not in)
    pub fn agent_operational_state_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_operational_state_nin = Some(join_csv(v)); self }
    /// The agent pending actions
    pub fn agent_pending_actions<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_pending_actions = Some(join_csv(v)); self }
    /// The agent pending actions (not in)
    pub fn agent_pending_actions_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_pending_actions_nin = Some(join_csv(v)); self }
    /// Whether the agent is pending uninstall
    pub fn agent_pending_uninstall<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_pending_uninstall = Some(join_csv(v)); self }
    /// Whether the agent is pending upgrade
    pub fn agent_pending_upgrade<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_pending_upgrade = Some(join_csv(v)); self }
    /// The agent network scanner status
    pub fn agent_ranger_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_ranger_status = Some(join_csv(v)); self }
    /// The agent network scanner status (not in)
    pub fn agent_ranger_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_ranger_status_nin = Some(join_csv(v)); self }
    /// The agent network scanner version
    pub fn agent_ranger_version<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_ranger_version = Some(join_csv(v)); self }
    /// The agent network scanner version (not in)
    pub fn agent_ranger_version_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_ranger_version_nin = Some(join_csv(v)); self }
    /// Live update ID
    pub fn agent_s1_agent_live_updates_version_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_s1_agent_live_updates_version_contains = Some(join_csv(v)); self }
    /// The agent subscribe time
    pub fn agent_subscribe_on_dt_between(mut self, v: impl Into<String>) -> Self { self.agent_subscribe_on_dt_between = Some(v.into()); self }
    /// Whether the agent is uninstalled
    pub fn agent_uninstalled<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_uninstalled = Some(join_csv(v)); self }
    /// Match by the agent UUID
    pub fn agent_uuid<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_uuid = Some(join_csv(v)); self }
    /// The UUID
    pub fn agent_uuid_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_uuid_contains = Some(join_csv(v)); self }
    /// The agent VSS last snapshot date
    pub fn agent_vss_last_snapshot_dt_between(mut self, v: impl Into<String>) -> Self { self.agent_vss_last_snapshot_dt_between = Some(v.into()); self }
    /// The agent VSS protection status
    pub fn agent_vss_protection_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_vss_protection_status = Some(join_csv(v)); self }
    /// The agent VSS protection status (not in)
    pub fn agent_vss_protection_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_vss_protection_status_nin = Some(join_csv(v)); self }
    /// The agent VSS rollback status
    pub fn agent_vss_rollback_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_vss_rollback_status = Some(join_csv(v)); self }
    /// The agent VSS rollback status (not in)
    pub fn agent_vss_rollback_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_vss_rollback_status_nin = Some(join_csv(v)); self }
    /// The agent VSS service status
    pub fn agent_vss_service_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_vss_service_status = Some(join_csv(v)); self }
    /// The agent VSS service status (not in)
    pub fn agent_vss_service_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.agent_vss_service_status_nin = Some(join_csv(v)); self }
    /// The agent free VSS volume percentage on any of the volumes
    pub fn agent_vss_volumes_diff_area_free_percentage_between(mut self, v: impl Into<String>) -> Self { self.agent_vss_volumes_diff_area_free_percentage_between = Some(v.into()); self }
    /// The severity of the alert
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.alert_severity = Some(join_csv(v)); self }
    /// User and cloud tag keys
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key = Some(join_csv(v)); self }
    /// User and cloud tags
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_value = Some(join_csv(v)); self }
    /// User and cloud tags (not in)
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_value_nin = Some(join_csv(v)); self }
    /// User and cloud tag keys exists
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_exists = Some(join_csv(v)); self }
    /// User and cloud tag keys not exists
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_nexists = Some(join_csv(v)); self }
    /// User and cloud tag keys (not in)
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_nin = Some(join_csv(v)); self }
    /// The name of the application installed on a workstation or a server
    pub fn application_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.application_name = Some(join_csv(v)); self }
    /// The architecture of the device
    pub fn architecture<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.architecture = Some(join_csv(v)); self }
    /// The architecture of the device (not in)
    pub fn architecture_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.architecture_nin = Some(join_csv(v)); self }
    /// Asset Contact Email
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_contact_email = Some(join_csv(v)); self }
    /// Asset Contact Email (not in)
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_contact_email_nin = Some(join_csv(v)); self }
    /// The criticality that each asset belongs to
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_criticality = Some(join_csv(v)); self }
    /// The criticality that each asset belongs to (not in)
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_criticality_nin = Some(join_csv(v)); self }
    /// The environment that the asset exists in - AWS \| Azure \| GCP \| Active Directory
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_environment = Some(join_csv(v)); self }
    /// The environment that the asset exists in - AWS \| Azure \| GCP \| Active Directory (not in)
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_environment_nin = Some(join_csv(v)); self }
    /// The status of the asset
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_status = Some(join_csv(v)); self }
    /// The status of the asset (not in)
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_status_nin = Some(join_csv(v)); self }
    /// Bad Password Time
    pub fn bad_password_time_between(mut self, v: impl Into<String>) -> Self { self.bad_password_time_between = Some(v.into()); self }
    /// Classification
    pub fn classification<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.classification = Some(join_csv(v)); self }
    /// The cloud provider account id
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id = Some(join_csv(v)); self }
    /// The cloud provider account ID
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id_contains = Some(join_csv(v)); self }
    /// The cloud provider account id (not in)
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id_nin = Some(join_csv(v)); self }
    /// The cloud provider account name
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name = Some(join_csv(v)); self }
    /// The cloud provider account name
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name_contains = Some(join_csv(v)); self }
    /// The cloud provider account name (not in)
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name_nin = Some(join_csv(v)); self }
    /// The cloud provider organization unit
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_organization_unit_contains = Some(join_csv(v)); self }
    /// The cloud provider organization
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_organization_contains = Some(join_csv(v)); self }
    /// The cloud provider project ID
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_project_id_contains = Some(join_csv(v)); self }
    /// The cloud provider subscription ID
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_subscription_id_contains = Some(join_csv(v)); self }
    /// The cloud resource ID
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_resource_id_contains = Some(join_csv(v)); self }
    /// The cloud tags key
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key = Some(join_csv(v)); self }
    /// The cloud tags key value
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value = Some(join_csv(v)); self }
    /// Free-text filter by cloud tag key value (supports multiple values)
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value_contains = Some(join_csv(v)); self }
    /// The cloud tags key value (not in)
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value_nin = Some(join_csv(v)); self }
    /// Free-text filter by cloud tag key (supports multiple values)
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_contains = Some(join_csv(v)); self }
    /// The cloud tags key (not in)
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_nin = Some(join_csv(v)); self }
    /// The LDAP Common Name
    pub fn cn<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cn = Some(join_csv(v)); self }
    /// The LDAP Common Name
    pub fn cn_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cn_contains = Some(join_csv(v)); self }
    /// The LDAP Common Name (not in)
    pub fn cn_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cn_nin = Some(join_csv(v)); self }
    /// The number of cores
    pub fn core_count<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.core_count = Some(join_csv(v)); self }
    /// The number of cores (not in)
    pub fn core_count_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.core_count_nin = Some(join_csv(v)); self }
    /// The columns for which filter count would be returned for
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.counts_for = Some(join_csv(v)); self }
    /// The CPU
    pub fn cpu_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cpu_contains = Some(join_csv(v)); self }
    /// Whether the AD Entity is deleted or not
    pub fn deleted<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.deleted = Some(join_csv(v)); self }
    /// Deleted Time
    pub fn deleted_time_between<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.deleted_time_between = Some(join_csv(v)); self }
    /// Department
    pub fn department<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.department = Some(join_csv(v)); self }
    /// The site from which the device was detected
    pub fn detected_from_site<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.detected_from_site = Some(join_csv(v)); self }
    /// The site from which the device was detected (not in)
    pub fn detected_from_site_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.detected_from_site_nin = Some(join_csv(v)); self }
    /// The asset review
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.device_review = Some(join_csv(v)); self }
    /// The asset review (not in)
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.device_review_nin = Some(join_csv(v)); self }
    /// The discovery methods
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.discovery_methods = Some(join_csv(v)); self }
    /// The discovery methods (not in)
    pub fn discovery_methods_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.discovery_methods_nin = Some(join_csv(v)); self }
    /// The Display Name
    pub fn display_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.display_name_contains = Some(join_csv(v)); self }
    /// The Distinguished Name
    pub fn distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.distinguished_name_contains = Some(join_csv(v)); self }
    /// The domain of the device
    pub fn domain<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.domain = Some(join_csv(v)); self }
    /// The domain
    pub fn domain_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.domain_contains = Some(join_csv(v)); self }
    /// The domain of the device (not in)
    pub fn domain_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.domain_nin = Some(join_csv(v)); self }
    /// Employee Type
    pub fn employee_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.employee_type = Some(join_csv(v)); self }
    /// Whether the Identity Group is enabled or not
    pub fn enabled<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.enabled = Some(join_csv(v)); self }
    /// The encryption type
    pub fn encryption_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.encryption_type = Some(join_csv(v)); self }
    /// The encryption type (not in)
    pub fn encryption_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.encryption_type_nin = Some(join_csv(v)); self }
    /// Entra ID Group Type
    pub fn entraid_group_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.entraid_group_type = Some(join_csv(v)); self }
    /// Entra ID Group Type (not in)
    pub fn entraid_group_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.entraid_group_type_nin = Some(join_csv(v)); self }
    /// The agent supported or unknown state
    pub fn epp_unsupported_unknown<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.epp_unsupported_unknown = Some(join_csv(v)); self }
    /// The agent supported or unknown state (not in)
    pub fn epp_unsupported_unknown_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.epp_unsupported_unknown_nin = Some(join_csv(v)); self }
    /// Expiration Time
    pub fn expiration_time_between<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.expiration_time_between = Some(join_csv(v)); self }
    /// The first seen date
    pub fn first_seen_dt_between(mut self, v: impl Into<String>) -> Self { self.first_seen_dt_between = Some(v.into()); self }
    /// The Forest Name
    pub fn forest<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.forest = Some(join_csv(v)); self }
    /// The Forest Name
    pub fn forest_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.forest_contains = Some(join_csv(v)); self }
    /// The Forest Name (not in)
    pub fn forest_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.forest_nin = Some(join_csv(v)); self }
    /// The gateway IPs
    pub fn gateway_ips_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.gateway_ips_contains = Some(join_csv(v)); self }
    /// The gateway MACs
    pub fn gateway_macs_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.gateway_macs_contains = Some(join_csv(v)); self }
    /// Given Name
    pub fn given_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.given_name = Some(join_csv(v)); self }
    /// Given Name
    pub fn given_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.given_name_contains = Some(join_csv(v)); self }
    /// List of Group IDs to filter by
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.group_ids = Some(join_csv(v)); self }
    /// The Group Type
    pub fn group_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.group_type = Some(join_csv(v)); self }
    /// The Group Type (not in)
    pub fn group_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.group_type_nin = Some(join_csv(v)); self }
    /// The hostnames
    pub fn hostnames_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.hostnames_contains = Some(join_csv(v)); self }
    /// The ID
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.id_contains = Some(join_csv(v)); self }
    /// The ID
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.id_in = Some(join_csv(v)); self }
    /// AD machine DN
    pub fn identity_ad_machine_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.identity_ad_machine_distinguished_name_contains = Some(join_csv(v)); self }
    /// AD machine groups
    pub fn identity_ad_machine_membership_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.identity_ad_machine_membership_contains = Some(join_csv(v)); self }
    /// AD machine or its groups
    pub fn identity_ad_machine_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.identity_ad_machine_contains = Some(join_csv(v)); self }
    /// AD user DN
    pub fn identity_ad_user_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.identity_ad_user_distinguished_name_contains = Some(join_csv(v)); self }
    /// AD user groups
    pub fn identity_ad_user_membership_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.identity_ad_user_membership_contains = Some(join_csv(v)); self }
    /// AD user or their groups
    pub fn identity_ad_user_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.identity_ad_user_contains = Some(join_csv(v)); self }
    /// Any AD string
    pub fn identity_ad_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.identity_ad_contains = Some(join_csv(v)); self }
    /// Free-text filter by the image name
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.image_name_contains = Some(join_csv(v)); self }
    /// The status alerts of the asset
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.infection_status = Some(join_csv(v)); self }
    /// The status alerts of the asset (not in)
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.infection_status_nin = Some(join_csv(v)); self }
    /// The internal IPs
    pub fn internal_ips_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.internal_ips_contains = Some(join_csv(v)); self }
    /// The IP addresses
    pub fn ip_address_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.ip_address_contains = Some(join_csv(v)); self }
    /// Is AD Connector
    pub fn is_ad_connector<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.is_ad_connector = Some(join_csv(v)); self }
    /// Is DC Server
    pub fn is_dc_server<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.is_dc_server = Some(join_csv(v)); self }
    /// Whether the instance is a rogue or not
    pub fn is_rogues<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.is_rogues = Some(join_csv(v)); self }
    /// Job Title
    pub fn job_title<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.job_title = Some(join_csv(v)); self }
    /// Free-text filter by Kubernetes Annotations key value (supports multiple values)
    pub fn k8s_annotations_unified_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_annotations_unified_key_value_contains = Some(join_csv(v)); self }
    /// Free-text filter by Kubernetes Annotations key (supports multiple values)
    pub fn k8s_annotations_unified_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_annotations_unified_key_contains = Some(join_csv(v)); self }
    /// The Kubernetes Cluster
    pub fn k8s_cluster<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_cluster = Some(join_csv(v)); self }
    /// The Kubernetes Cluster ID
    pub fn k8s_cluster_id<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_cluster_id = Some(join_csv(v)); self }
    /// Kubernetes Cluster ID
    pub fn k8s_cluster_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_cluster_id_contains = Some(join_csv(v)); self }
    /// The Kubernetes cluster
    pub fn k8s_cluster_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_cluster_contains = Some(join_csv(v)); self }
    /// The Kubernetes Cluster (not in)
    pub fn k8s_cluster_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_cluster_nin = Some(join_csv(v)); self }
    /// Free-text filter by Kubernetes Labels key value (supports multiple values)
    pub fn k8s_labels_unified_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_labels_unified_key_value_contains = Some(join_csv(v)); self }
    /// Free-text filter by Kubernetes Labels key (supports multiple values)
    pub fn k8s_labels_unified_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_labels_unified_key_contains = Some(join_csv(v)); self }
    /// The Kubernetes Namespace Name
    pub fn k8s_namespace<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_namespace = Some(join_csv(v)); self }
    /// Namespace Name
    pub fn k8s_namespace_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_namespace_contains = Some(join_csv(v)); self }
    /// The Kubernetes Node
    pub fn k8s_node<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_node = Some(join_csv(v)); self }
    /// The Kubernetes node
    pub fn k8s_node_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_node_contains = Some(join_csv(v)); self }
    /// The Kubernetes Node (not in)
    pub fn k8s_node_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_node_nin = Some(join_csv(v)); self }
    /// The Kubernetes Resource ID
    pub fn k8s_resource_id<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_resource_id = Some(join_csv(v)); self }
    /// Kubernetes Resource ID
    pub fn k8s_resource_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_resource_id_contains = Some(join_csv(v)); self }
    /// Running on Nodes
    pub fn k8s_running_on_nodes<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_running_on_nodes = Some(join_csv(v)); self }
    /// Running on nodes
    pub fn k8s_running_on_nodes_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_running_on_nodes_contains = Some(join_csv(v)); self }
    /// The Kubernetes Type
    pub fn k8s_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_type = Some(join_csv(v)); self }
    /// The Kubernetes Type (not in)
    pub fn k8s_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_type_nin = Some(join_csv(v)); self }
    /// The Kubernetes Version
    pub fn k8s_version<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_version = Some(join_csv(v)); self }
    /// The Kubernetes Version (not in)
    pub fn k8s_version_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.k8s_version_nin = Some(join_csv(v)); self }
    /// The last active date
    pub fn last_active_dt_between(mut self, v: impl Into<String>) -> Self { self.last_active_dt_between = Some(v.into()); self }
    /// The Last time of User Login
    pub fn last_logon_time_between(mut self, v: impl Into<String>) -> Self { self.last_logon_time_between = Some(v.into()); self }
    /// The last update date
    pub fn last_update_dt_between(mut self, v: impl Into<String>) -> Self { self.last_update_dt_between = Some(v.into()); self }
    /// Lock-out Time
    pub fn last_modified_time_between(mut self, v: impl Into<String>) -> Self { self.last_modified_time_between = Some(v.into()); self }
    /// Legacy Identity Policy Name
    pub fn legacy_identity_policy_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.legacy_identity_policy_name = Some(join_csv(v)); self }
    /// The legacy identity policy name
    pub fn legacy_identity_policy_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.legacy_identity_policy_name_contains = Some(join_csv(v)); self }
    /// Lock-out Time
    pub fn lock_out_time_between(mut self, v: impl Into<String>) -> Self { self.lock_out_time_between = Some(v.into()); self }
    /// The MAC addresses
    pub fn mac_addresses_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.mac_addresses_contains = Some(join_csv(v)); self }
    /// The Email Address
    pub fn mail<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.mail = Some(join_csv(v)); self }
    /// The Email Address
    pub fn mail_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.mail_contains = Some(join_csv(v)); self }
    /// The manufacturer of the device
    pub fn manufacturer<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.manufacturer = Some(join_csv(v)); self }
    /// The manufacturer
    pub fn manufacturer_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.manufacturer_contains = Some(join_csv(v)); self }
    /// The manufacturer of the device (not in)
    pub fn manufacturer_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.manufacturer_nin = Some(join_csv(v)); self }
    /// The memory of the device in human readable format
    pub fn memory_readable<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.memory_readable = Some(join_csv(v)); self }
    /// The memory of the device in human readable format (not in)
    pub fn memory_readable_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.memory_readable_nin = Some(join_csv(v)); self }
    /// The missing coverage for the asset
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.missing_coverage = Some(join_csv(v)); self }
    /// The missing coverage for the asset (not in)
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.missing_coverage_nin = Some(join_csv(v)); self }
    /// The name
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.name_contains = Some(join_csv(v)); self }
    /// Name
    pub fn names<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.names = Some(join_csv(v)); self }
    /// Name (not in)
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.names_nin = Some(join_csv(v)); self }
    /// The network name
    pub fn network_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.network_name = Some(join_csv(v)); self }
    /// The network name (not in)
    pub fn network_name_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.network_name_nin = Some(join_csv(v)); self }
    /// The Object Category
    pub fn object_category<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_category = Some(join_csv(v)); self }
    /// The Object Category (not in)
    pub fn object_category_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_category_nin = Some(join_csv(v)); self }
    /// The Object Class
    pub fn object_class<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_class = Some(join_csv(v)); self }
    /// The Object Class (not in)
    pub fn object_class_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_class_nin = Some(join_csv(v)); self }
    /// The number of objects in the bucket
    pub fn object_count<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_count = Some(join_csv(v)); self }
    /// The number of objects in the bucket (not in)
    pub fn object_count_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_count_nin = Some(join_csv(v)); self }
    /// The Object GUID
    pub fn object_guid_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_guid_contains = Some(join_csv(v)); self }
    /// The Object SID
    pub fn object_sid_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.object_sid_contains = Some(join_csv(v)); self }
    /// On Premises Distinguished Name
    pub fn on_premises_distinguished_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_distinguished_name = Some(join_csv(v)); self }
    /// onPremisesDistinguishedName__contains
    pub fn on_premises_distinguished_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_distinguished_name_contains = Some(join_csv(v)); self }
    /// On Premises Domain Name
    pub fn on_premises_domain_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_domain_name = Some(join_csv(v)); self }
    /// On Premises Distinguished Name
    pub fn on_premises_domain_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_domain_name_contains = Some(join_csv(v)); self }
    /// On Premises Immutable Id
    pub fn on_premises_immutable_id<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_immutable_id = Some(join_csv(v)); self }
    /// On Premises Last Sync Time
    pub fn on_premises_last_sync_time_between<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_last_sync_time_between = Some(join_csv(v)); self }
    /// On Premises SAM Account Name
    pub fn on_premises_sam_account_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_sam_account_name = Some(join_csv(v)); self }
    /// On Premises SAM Account Name
    pub fn on_premises_sam_account_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_sam_account_name_contains = Some(join_csv(v)); self }
    /// On Premises Security Identifier
    pub fn on_premises_security_identifier<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_security_identifier = Some(join_csv(v)); self }
    /// onPremisesSecurityIdentifier__contains
    pub fn on_premises_security_identifier_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_security_identifier_contains = Some(join_csv(v)); self }
    /// On Premises Sync Enabled
    pub fn on_premises_sync_enabled<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_sync_enabled = Some(join_csv(v)); self }
    /// On Premises User Principal Name
    pub fn on_premises_user_principal_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_user_principal_name = Some(join_csv(v)); self }
    /// On Premises User Principal Name
    pub fn on_premises_user_principal_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.on_premises_user_principal_name_contains = Some(join_csv(v)); self }
    /// The operating system of the device
    pub fn os<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.os = Some(join_csv(v)); self }
    /// The operating system family of the device
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.os_family = Some(join_csv(v)); self }
    /// The operating system family of the device (not in)
    pub fn os_family_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.os_family_nin = Some(join_csv(v)); self }
    /// The operating system name and version of the device
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.os_name_version = Some(join_csv(v)); self }
    /// The OS names and versions
    pub fn os_name_version_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.os_name_version_contains = Some(join_csv(v)); self }
    /// The operating system name and version of the device (not in)
    pub fn os_name_version_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.os_name_version_nin = Some(join_csv(v)); self }
    /// The operating system version of the device
    pub fn os_version<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.os_version = Some(join_csv(v)); self }
    /// The OS versions
    pub fn os_version_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.os_version_contains = Some(join_csv(v)); self }
    /// The operating system version of the device (not in)
    pub fn os_version_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.os_version_nin = Some(join_csv(v)); self }
    /// The operating system of the device (not in)
    pub fn os_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.os_nin = Some(join_csv(v)); self }
    /// Other Mails
    pub fn other_mails<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.other_mails = Some(join_csv(v)); self }
    /// Other Mails
    pub fn other_mails_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.other_mails_contains = Some(join_csv(v)); self }
    /// Whether the password never expires
    pub fn password_never_expire<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.password_never_expire = Some(join_csv(v)); self }
    /// Whether the AD Entity is privileged or not
    pub fn privileged<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.privileged = Some(join_csv(v)); self }
    /// Proxy Addresses
    pub fn proxy_addresses<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.proxy_addresses = Some(join_csv(v)); self }
    /// Proxy Addresses
    pub fn proxy_addresses_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.proxy_addresses_contains = Some(join_csv(v)); self }
    /// Free-text filter by Ranger tag key value (supports multiple values)
    pub fn ranger_tag_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.ranger_tag_key_value_contains = Some(join_csv(v)); self }
    /// Free-text filter by Ranger tag key (supports multiple values)
    pub fn ranger_tag_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.ranger_tag_key_contains = Some(join_csv(v)); self }
    /// The ranger tags key
    pub fn ranger_tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.ranger_tags_key = Some(join_csv(v)); self }
    /// The ranger tags key value
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.ranger_tags_key_value = Some(join_csv(v)); self }
    /// The ranger tags key value (not in)
    pub fn ranger_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.ranger_tags_key_value_nin = Some(join_csv(v)); self }
    /// The ranger tags key (not in)
    pub fn ranger_tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.ranger_tags_key_nin = Some(join_csv(v)); self }
    /// The region
    pub fn region<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region = Some(join_csv(v)); self }
    /// The geographical area where cloud resources are hosted
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region_contains = Some(join_csv(v)); self }
    /// The region (not in)
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region_nin = Some(join_csv(v)); self }
    /// The canonical name for the resource type
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type = Some(join_csv(v)); self }
    /// The Asset Type
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type_contains = Some(join_csv(v)); self }
    /// The canonical name for the resource type (not in)
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type_nin = Some(join_csv(v)); self }
    /// The risk factors associated with the asset
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.risk_factors = Some(join_csv(v)); self }
    /// The risk factors associated with the asset (not in)
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.risk_factors_nin = Some(join_csv(v)); self }
    /// The Last Seen date and time for the asset
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self { self.s1_updated_at_between = Some(v.into()); self }
    /// The SAM Account Name
    pub fn sam_account_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sam_account_name = Some(join_csv(v)); self }
    /// The SAM Account Name (not in)
    pub fn sam_account_name_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sam_account_name_nin = Some(join_csv(v)); self }
    /// The CDS malware scan status
    pub fn scan_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.scan_status = Some(join_csv(v)); self }
    /// The CDS malware scan status
    pub fn scan_status_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.scan_status_contains = Some(join_csv(v)); self }
    /// The CDS malware scan status (not in)
    pub fn scan_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.scan_status_nin = Some(join_csv(v)); self }
    /// Security Enabled
    pub fn security_enabled<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.security_enabled = Some(join_csv(v)); self }
    /// The serial number
    pub fn serial_number<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.serial_number = Some(join_csv(v)); self }
    /// The serial number
    pub fn serial_number_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.serial_number_contains = Some(join_csv(v)); self }
    /// The serial number (not in)
    pub fn serial_number_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.serial_number_nin = Some(join_csv(v)); self }
    /// The Service Account
    pub fn service_account<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.service_account = Some(join_csv(v)); self }
    /// The Service Principal Name
    pub fn service_principal_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.service_principal_name_contains = Some(join_csv(v)); self }
    /// List of Site IDs to filter by
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.site_ids = Some(join_csv(v)); self }
    /// The state of the instance
    pub fn state<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.state = Some(join_csv(v)); self }
    /// The state
    pub fn state_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.state_contains = Some(join_csv(v)); self }
    /// The state of the instance (not in)
    pub fn state_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.state_nin = Some(join_csv(v)); self }
    /// The sub-category that each resource belongs to
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sub_category = Some(join_csv(v)); self }
    /// The sub-category that each resource belongs to (not in)
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sub_category_nin = Some(join_csv(v)); self }
    /// The subnets
    pub fn subnets_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.subnets_contains = Some(join_csv(v)); self }
    /// The Surface that each asset belongs to
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.surfaces = Some(join_csv(v)); self }
    /// The Surface that each asset belongs to (not in)
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.surfaces_nin = Some(join_csv(v)); self }
    /// Surname
    pub fn surname<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.surname = Some(join_csv(v)); self }
    /// Tag Keys
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key = Some(join_csv(v)); self }
    /// Tags
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value = Some(join_csv(v)); self }
    /// Free-text filter by tag key value (supports multiple values)
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value_contains = Some(join_csv(v)); self }
    /// Tags (not in)
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value_nin = Some(join_csv(v)); self }
    /// Free-text filter by tag key (supports multiple values)
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_contains = Some(join_csv(v)); self }
    /// Tag Keys exists
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_exists = Some(join_csv(v)); self }
    /// Tag Keys not exists
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_nexists = Some(join_csv(v)); self }
    /// Tag Keys (not in)
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_nin = Some(join_csv(v)); self }
    /// The TCP ports
    pub fn tcp_ports<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tcp_ports = Some(join_csv(v)); self }
    /// Threat Detection Policy
    pub fn threat_detection_policy_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.threat_detection_policy_status = Some(join_csv(v)); self }
    /// Threat Detection Status
    pub fn threat_detection_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.threat_detection_status = Some(join_csv(v)); self }
    /// The threat detection status
    pub fn threat_detection_status_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.threat_detection_status_contains = Some(join_csv(v)); self }
    /// Threat Detection Status (not in)
    pub fn threat_detection_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.threat_detection_status_nin = Some(join_csv(v)); self }
    /// The UDP ports
    pub fn udp_ports<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.udp_ports = Some(join_csv(v)); self }
    /// Unique Name
    pub fn unique_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.unique_name = Some(join_csv(v)); self }
    /// The User Account Control
    pub fn user_account_control<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.user_account_control = Some(join_csv(v)); self }
    /// The User Account Control (not in)
    pub fn user_account_control_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.user_account_control_nin = Some(join_csv(v)); self }
    /// The User Password Expiry Time
    pub fn user_password_expiry_time_computed_between(mut self, v: impl Into<String>) -> Self { self.user_password_expiry_time_computed_between = Some(v.into()); self }
    /// The User Principal Name
    pub fn user_principal_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.user_principal_name = Some(join_csv(v)); self }
    /// The User Principal Name
    pub fn user_principal_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.user_principal_name_contains = Some(join_csv(v)); self }
    /// The User Principal Name (not in)
    pub fn user_principal_name_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.user_principal_name_nin = Some(join_csv(v)); self }
    /// Visibility
    pub fn visibility<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.visibility = Some(join_csv(v)); self }
}

/// Request body for the entity-selection endpoints
/// (`fetch-agent-ids`, `fetch-unified-actions`).
///
/// Spec definition: `AffectedEntitiesSchema`. Both `id__in` and `id__nin` are
/// freeform objects keyed by entity/asset type, each mapping to a list of ids;
/// they are therefore modeled as `serde_json::Value` for fidelity. All fields
/// are optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedEntitiesBody {
    /// List of selected entity ids. Freeform object: `{ "<entityType>":
    /// ["id", ...] }`. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<serde_json::Value>,
    /// List of entity ids to exclude from `select_all`. Freeform object:
    /// `{ "<entityType>": ["id", ...] }`. Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<serde_json::Value>,
}

impl AffectedEntitiesBody {
    /// List of selected entity ids, keyed by entity/asset type.
    pub fn id_in(mut self, v: serde_json::Value) -> Self {
        self.id_in = Some(v);
        self
    }
    /// List of entity ids to exclude from `select_all`, keyed by entity/asset
    /// type.
    pub fn id_nin(mut self, v: serde_json::Value) -> Self {
        self.id_nin = Some(v);
        self
    }
}

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/actions/perform-unified-action`.
///
/// Spec definition: `PerformActionRequestSchema`. `actionPath` is required; the
/// `payload` is freeform and `id__in`/`id__nin` are freeform objects keyed by
/// entity/asset type.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionBody {
    /// Action path. Required.
    pub action_path: String,
    /// Freeform action payload. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
    /// List of selected entity ids. Freeform object: `{ "<entityType>":
    /// ["id", ...] }`. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<serde_json::Value>,
    /// List of entity ids to exclude from `select_all`. Freeform object:
    /// `{ "<entityType>": ["id", ...] }`. Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<serde_json::Value>,
}

impl PerformActionBody {
    /// Construct with the required `action_path`.
    pub fn new(action_path: impl Into<String>) -> Self {
        Self {
            action_path: action_path.into(),
            payload: None,
            id_in: None,
            id_nin: None,
        }
    }
    /// Freeform action payload.
    pub fn payload(mut self, v: serde_json::Value) -> Self {
        self.payload = Some(v);
        self
    }
    /// List of selected entity ids, keyed by entity/asset type.
    pub fn id_in(mut self, v: serde_json::Value) -> Self {
        self.id_in = Some(v);
        self
    }
    /// List of entity ids to exclude from `select_all`, keyed by entity/asset
    /// type.
    pub fn id_nin(mut self, v: serde_json::Value) -> Self {
        self.id_nin = Some(v);
        self
    }
}

impl InventoryUnifiedActionsService<'_> {
    /// `POST /web/api/v2.1/xdr/assets/actions/fetch-agent-ids` — Loads all
    /// agent ids for the unified actions.
    ///
    /// Loads all agent ids for the unified actions. Filters narrow the target
    /// set (query params); the explicit entity selection is supplied in the
    /// body.
    pub async fn fetch_agent_ids(
        &self,
        query: &InventoryUnifiedActionsQuery,
        body: &AffectedEntitiesBody,
    ) -> Result<Response<FetchAgentIdsResponse>, Error> {
        let path =
            path_with_query("/web/api/v2.1/xdr/assets/actions/fetch-agent-ids", query);
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/actions/fetch-unified-actions` — Get
    /// Available Actions by Asset/Entity Type.
    ///
    /// Get Available Actions by Asset/Entity Type. Filters narrow the target
    /// set (query params); the explicit entity selection is supplied in the
    /// body.
    pub async fn fetch_unified_actions(
        &self,
        query: &InventoryUnifiedActionsQuery,
        body: &AffectedEntitiesBody,
    ) -> Result<Response<AvailableActionsResponse>, Error> {
        let path = path_with_query(
            "/web/api/v2.1/xdr/assets/actions/fetch-unified-actions",
            query,
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/actions/perform-unified-action` — Perform
    /// an Action on selected assets/entities.
    ///
    /// Perform an Action on selected assets/entities. Filters narrow the target
    /// set (query params); the action path, payload and explicit entity
    /// selection are supplied in the body. The spec declares no `200` response
    /// body, so the result is returned untyped as `serde_json::Value`.
    pub async fn perform_unified_action(
        &self,
        query: &InventoryUnifiedActionsQuery,
        body: &PerformActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = path_with_query(
            "/web/api/v2.1/xdr/assets/actions/perform-unified-action",
            query,
        );
        Ok(self.client.http().post(&path, body).await?)
    }
}
