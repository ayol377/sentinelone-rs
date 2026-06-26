use serde::{Deserialize, Serialize};

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::Agent;
use crate::pagination::{Paginated, Response};

/// `Agents` tag — Agent views and operations.
pub struct AgentsService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/agents` (Get Agents).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
pub struct AgentsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000 items, use "cursor". Example: "150". Optional.
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects. Optional.
    #[serde(rename = "countOnly", skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up execution time. Optional.
    #[serde(rename = "skipCount", skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: "id". Optional.
    ///
    /// Allowed values: createdAt, updatedAt, id, computerName, groupId, osType, osName, uuid, scanStatus, isDecommissioned, decommissionedAt, isUninstalled, threatMitigationStatus, threatResolved, threatContentHash, threatCreatedAt, registeredAt, lastActiveDate, isActive, isPendingUninstall, activeThreats, isUpToDate, agentVersion, osArch, machineType, networkStatus, domain, mitigationMode, mitigationModeSuspicious, encryptedApplications, totalMemory, cpuCount, coreCount, externalIp, lastLoggedInUserName, groupName, networkInterfacePhysical, siteId, siteName, infected, threatRebootRequired, consoleMigrationStatus, appsVulnerabilityStatus, accountName, externalId, installerType, rangerVersion, operationalState, remoteProfilingState, networkQuarantineEnabled, firewallEnabled, locationEnabled, cloudAccount, cloudImage, cloudInstanceId, cloudInstanceSize, cloudLocation, cloudNetwork, cloudProvider, clusterName, kubernetesVersion, kubernetesType, agentNamespace, agentPodName, serialNumber, cpuId, fullDiskScanLastUpdatedAt, lastSuccessfulScanDate, ecsType, ecsVersion, ecsClusterName, ecsTaskArn, ecsTaskAvailabilityZone, ecsServiceName, ecsServiceArn, ecsTaskDefinitionFamily, ecsTaskDefinitionRevision, ecsTaskDefinitionArn, isAdConnector, activeProtection, pacFileUsage, proxyMethod, isMgmtProxyEnabled, console, mgmtProxyAddress, isEventSearchProxyEnabled, deepVisibility, eventSearchAddress, entraId.
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Optional.
    ///
    /// Allowed values: asc, desc.
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredGroupIds", skip_serializing_if = "Option::is_none")]
    pub filtered_group_ids: Option<String>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredSiteIds", skip_serializing_if = "Option::is_none")]
    pub filtered_site_ids: Option<String>,
    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "registeredAt__between", skip_serializing_if = "Option::is_none")]
    pub registered_at__between: Option<String>,
    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastActiveDate__between", skip_serializing_if = "Option::is_none")]
    pub last_active_date__between: Option<String>,
    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastSuccessfulScanDate__between", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__between: Option<String>,
    /// Include only active Agents. Optional.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<String>,
    /// Include only Agents with pending uninstall requests. Optional.
    #[serde(rename = "isPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub is_pending_uninstall: Option<String>,
    /// Include only Agents with at least one active threat. Optional.
    #[serde(rename = "infected", skip_serializing_if = "Option::is_none")]
    pub infected: Option<bool>,
    /// Include only Agents with updated software. Optional.
    #[serde(rename = "isUpToDate", skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<String>,
    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux". Optional.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersions", skip_serializing_if = "Option::is_none")]
    pub agent_versions: Option<String>,
    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersionsNin", skip_serializing_if = "Option::is_none")]
    pub agent_versions_nin: Option<String>,
    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersions", skip_serializing_if = "Option::is_none")]
    pub ranger_versions: Option<String>,
    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersionsNin", skip_serializing_if = "Option::is_none")]
    pub ranger_versions_nin: Option<String>,
    /// OS architecture. Example: "32 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArch", skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<String>,
    /// OS architectures to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArches", skip_serializing_if = "Option::is_none")]
    pub os_arches: Option<String>,
    /// OS architectures not to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArchesNin", skip_serializing_if = "Option::is_none")]
    pub os_arches_nin: Option<String>,
    /// Included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypes", skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Not included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypesNin", skip_serializing_if = "Option::is_none")]
    pub os_types_nin: Option<String>,
    /// Included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatuses", skip_serializing_if = "Option::is_none")]
    pub scan_statuses: Option<String>,
    /// Not included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatusesNin", skip_serializing_if = "Option::is_none")]
    pub scan_statuses_nin: Option<String>,
    /// Included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypes", skip_serializing_if = "Option::is_none")]
    pub machine_types: Option<String>,
    /// Not included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypesNin", skip_serializing_if = "Option::is_none")]
    pub machine_types_nin: Option<String>,
    /// Included storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypes", skip_serializing_if = "Option::is_none")]
    pub storage_types: Option<String>,
    /// Excluded storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypesNin", skip_serializing_if = "Option::is_none")]
    pub storage_types_nin: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatuses", skip_serializing_if = "Option::is_none")]
    pub network_statuses: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatusesNin", skip_serializing_if = "Option::is_none")]
    pub network_statuses_nin: Option<String>,
    /// Included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// Not included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domainsNin", skip_serializing_if = "Option::is_none")]
    pub domains_nin: Option<String>,
    /// Disk encryption status. Optional.
    #[serde(rename = "encryptedApplications", skip_serializing_if = "Option::is_none")]
    pub encrypted_applications: Option<String>,
    /// Total memory range (GB, inclusive). Example: "4-8". Optional.
    #[serde(rename = "totalMemory__between", skip_serializing_if = "Option::is_none")]
    pub total_memory__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "coreCount__between", skip_serializing_if = "Option::is_none")]
    pub core_count__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "cpuCount__between", skip_serializing_if = "Option::is_none")]
    pub cpu_count__between: Option<String>,
    /// Included pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeeded", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed: Option<String>,
    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissions", skip_serializing_if = "Option::is_none")]
    pub missing_permissions: Option<String>,
    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeededNin", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed_nin: Option<String>,
    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissionsNin", skip_serializing_if = "Option::is_none")]
    pub missing_permissions_nin: Option<String>,
    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com". Optional.
    #[serde(rename = "adQuery", skip_serializing_if = "Option::is_none")]
    pub ad_query: Option<String>,
    /// Agent has a local configuration set. Optional.
    #[serde(rename = "hasLocalConfiguration", skip_serializing_if = "Option::is_none")]
    pub has_local_configuration: Option<bool>,
    /// Migration status in. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatuses", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses: Option<String>,
    /// Migration status nin. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatusesNin", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses_nin: Option<String>,
    /// Apps vulnerability status in. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatuses", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses: Option<String>,
    /// Apps vulnerability status nin. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatusesNin", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses_nin: Option<String>,
    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIds", skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<String>,
    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIdsNin", skip_serializing_if = "Option::is_none")]
    pub location_ids_nin: Option<String>,
    /// Include only Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypes", skip_serializing_if = "Option::is_none")]
    pub installer_types: Option<String>,
    /// Exclude Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypesNin", skip_serializing_if = "Option::is_none")]
    pub installer_types_nin: Option<String>,
    /// Agent operational state. Optional.
    #[serde(rename = "operationalStates", skip_serializing_if = "Option::is_none")]
    pub operational_states: Option<String>,
    /// Do not include these Agent operational states. Optional.
    #[serde(rename = "operationalStatesNin", skip_serializing_if = "Option::is_none")]
    pub operational_states_nin: Option<String>,
    /// Agent remote profiling state. Optional.
    #[serde(rename = "remoteProfilingStates", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states: Option<String>,
    /// Do not include these Agent remote profiling states. Optional.
    #[serde(rename = "remoteProfilingStatesNin", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states_nin: Option<String>,
    /// Status of Network Discovery. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatuses", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses: Option<String>,
    /// Do not include these Network Scanner Statuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatusesNin", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses_nin: Option<String>,
    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatus", skip_serializing_if = "Option::is_none")]
    pub ranger_status: Option<String>,
    /// Has at least one threat with at least one mitigation action pending reboot to succeed. Optional.
    #[serde(rename = "threatRebootRequired", skip_serializing_if = "Option::is_none")]
    pub threat_reboot_required: Option<String>,
    /// The agents supports Network Quarantine Control and its enabled for the agent's group. Optional.
    #[serde(rename = "networkQuarantineEnabled", skip_serializing_if = "Option::is_none")]
    pub network_quarantine_enabled: Option<String>,
    /// The agents supports Firewall Control and it is enabled for the agent's group. Optional.
    #[serde(rename = "firewallEnabled", skip_serializing_if = "Option::is_none")]
    pub firewall_enabled: Option<String>,
    /// The agents supports Location Awareness and it is enabled for the agent's group. Optional.
    #[serde(rename = "locationEnabled", skip_serializing_if = "Option::is_none")]
    pub location_enabled: Option<String>,
    /// Agents from which cloud provider. Optional.
    #[serde(rename = "cloudProvider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// Exclude Agents from these cloud provider. Optional.
    #[serde(rename = "cloudProviderNin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_nin: Option<String>,
    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}". Optional.
    #[serde(rename = "tagsData", skip_serializing_if = "Option::is_none")]
    pub tags_data: Option<String>,
    /// Include only Agents that have any tags assigned if True, or none if False. Optional.
    #[serde(rename = "hasTags", skip_serializing_if = "Option::is_none")]
    pub has_tags: Option<bool>,
    /// The agents that are ADConnectors if True, or not if False. Optional.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The agents has Hyper Automate PNA enabled if True, or not if False. Optional.
    #[serde(rename = "isHyperAutomate", skip_serializing_if = "Option::is_none")]
    pub is_hyper_automate: Option<String>,
    /// Included agent active protections. Example: "edr,idr". Optional.
    ///
    /// Allowed values: edr, idr.
    #[serde(rename = "activeProtection", skip_serializing_if = "Option::is_none")]
    pub active_protection: Option<String>,
    /// Containerized workload counts. Optional.
    #[serde(rename = "containerizedWorkloadCounts", skip_serializing_if = "Option::is_none")]
    pub containerized_workload_counts: Option<String>,
    /// Indicates whether the agent protects containerized workload at the moment. Optional.
    #[serde(rename = "hasContainerizedWorkload", skip_serializing_if = "Option::is_none")]
    pub has_containerized_workload: Option<String>,
    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "pacFileUsage", skip_serializing_if = "Option::is_none")]
    pub pac_file_usage: Option<String>,
    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethod", skip_serializing_if = "Option::is_none")]
    pub proxy_method: Option<String>,
    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethodNin", skip_serializing_if = "Option::is_none")]
    pub proxy_method_nin: Option<String>,
    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isMgmtProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_mgmt_proxy_enabled: Option<String>,
    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isEventSearchProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_event_search_proxy_enabled: Option<String>,
    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0". Optional.
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip__contains: Option<String>,
    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN". Optional.
    #[serde(rename = "computerName__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name__contains: Option<String>,
    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0". Optional.
    #[serde(rename = "networkInterfaceInet__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_inet__contains: Option<String>,
    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfacePhysical__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_physical__contains: Option<String>,
    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfaceGatewayMacAddress__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_gateway_mac_address__contains: Option<String>,
    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1". Optional.
    #[serde(rename = "lastLoggedInUserName__contains", skip_serializing_if = "Option::is_none")]
    pub last_logged_in_user_name__contains: Option<String>,
    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1". Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_query__contains: Option<String>,
    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_name__contains: Option<String>,
    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John". Optional.
    #[serde(rename = "adUserQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_query__contains: Option<String>,
    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_name__contains: Option<String>,
    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows". Optional.
    #[serde(rename = "adComputerQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_query__contains: Option<String>,
    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055". Optional.
    #[serde(rename = "uuid__contains", skip_serializing_if = "Option::is_none")]
    pub uuid__contains: Option<String>,
    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine". Optional.
    #[serde(rename = "externalId__contains", skip_serializing_if = "Option::is_none")]
    pub external_id__contains: Option<String>,
    /// Free-text filter by aws role(supports multiple values). Optional.
    #[serde(rename = "awsRole__contains", skip_serializing_if = "Option::is_none")]
    pub aws_role__contains: Option<String>,
    /// Free-text filter by aws securityGroups(supports multiple values). Optional.
    #[serde(rename = "awsSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub aws_security_groups__contains: Option<String>,
    /// Free-text filter by aws subnet ids (supports multiple values). Optional.
    #[serde(rename = "awsSubnetIds__contains", skip_serializing_if = "Option::is_none")]
    pub aws_subnet_ids__contains: Option<String>,
    /// Free-text filter by agent namespace (supports multiple values). Optional.
    #[serde(rename = "agentNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub agent_namespace__contains: Option<String>,
    /// Free-text filter by agent pod name (supports multiple values). Optional.
    #[serde(rename = "agentPodName__contains", skip_serializing_if = "Option::is_none")]
    pub agent_pod_name__contains: Option<String>,
    /// Free-text filter by azure resource group(supports multiple values). Optional.
    #[serde(rename = "azureResourceGroup__contains", skip_serializing_if = "Option::is_none")]
    pub azure_resource_group__contains: Option<String>,
    /// Free-text filter by cloud account (supports multiple values). Optional.
    #[serde(rename = "cloudAccount__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_account__contains: Option<String>,
    /// Free-text filter by cloud image (supports multiple values). Optional.
    #[serde(rename = "cloudImage__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_image__contains: Option<String>,
    /// Free-text filter by cloud instance id(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_id__contains: Option<String>,
    /// Free-text filter by cloud instance size(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceSize__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_size__contains: Option<String>,
    /// Free-text filter by cloud location (supports multiple values). Optional.
    #[serde(rename = "cloudLocation__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_location__contains: Option<String>,
    /// Free-text filter by cloud network (supports multiple values). Optional.
    #[serde(rename = "cloudNetwork__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_network__contains: Option<String>,
    /// Free-text filter by cloud tags (supports multiple values). Optional.
    #[serde(rename = "cloudTags__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags__contains: Option<String>,
    /// Free-text filter by cluster name (supports multiple values). Optional.
    #[serde(rename = "clusterName__contains", skip_serializing_if = "Option::is_none")]
    pub cluster_name__contains: Option<String>,
    /// Free-text filter by gcp service account (supports multiple values). Optional.
    #[serde(rename = "gcpServiceAccount__contains", skip_serializing_if = "Option::is_none")]
    pub gcp_service_account__contains: Option<String>,
    /// Free-text filter by K8s node labels (supports multiple values). Optional.
    #[serde(rename = "k8sNodeLabels__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_labels__contains: Option<String>,
    /// Free-text filter by K8s node name (supports multiple values). Optional.
    #[serde(rename = "k8sNodeName__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_name__contains: Option<String>,
    /// Free-text filter by K8s type(supports multiple values). Optional.
    #[serde(rename = "k8sType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_type__contains: Option<String>,
    /// Free-text filter by K8s version (supports multiple values). Optional.
    #[serde(rename = "k8sVersion__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_version__contains: Option<String>,
    /// Free-text filter by live update ID (supports multiple values). Optional.
    #[serde(rename = "liveUpdateId__contains", skip_serializing_if = "Option::is_none")]
    pub live_update_id__contains: Option<String>,
    /// Free-text filter by Serial Number (supports multiple values). Optional.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number__contains: Option<String>,
    /// Free-text filter by Entra ID (supports multiple values). Optional.
    #[serde(rename = "entraId__contains", skip_serializing_if = "Option::is_none")]
    pub entra_id__contains: Option<String>,
    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD". Optional.
    #[serde(rename = "cpuId__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_id__contains: Option<String>,
    /// Free-text filter by ECS type. Optional.
    #[serde(rename = "ecsType__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_type__contains: Option<String>,
    /// Free-text filter by ECS version. Optional.
    #[serde(rename = "ecsVersion__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_version__contains: Option<String>,
    /// Free-text filter by ECS cluster name. Optional.
    #[serde(rename = "ecsClusterName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_name__contains: Option<String>,
    /// Free-text filter by ECS task arn. Optional.
    #[serde(rename = "ecsTaskArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_arn__contains: Option<String>,
    /// Free-text filter by ECS task availability zone. Optional.
    #[serde(rename = "ecsTaskAvailabilityZone__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_availability_zone__contains: Option<String>,
    /// Free-text filter by ECS service name. Optional.
    #[serde(rename = "ecsServiceName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_name__contains: Option<String>,
    /// Free-text filter by ECS service arn. Optional.
    #[serde(rename = "ecsServiceArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_arn__contains: Option<String>,
    /// Free-text filter by ECS task definition family. Optional.
    #[serde(rename = "ecsTaskDefinitionFamily__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_family__contains: Option<String>,
    /// Free-text filter by ECS task definition revision. Optional.
    #[serde(rename = "ecsTaskDefinitionRevision__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_revision__contains: Option<String>,
    /// Free-text filter by ECS task definition arn. Optional.
    #[serde(rename = "ecsTaskDefinitionArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_arn__contains: Option<String>,
    /// Include active, decommissioned or both. Example: "True,False". Optional.
    #[serde(rename = "isDecommissioned", skip_serializing_if = "Option::is_none")]
    pub is_decommissioned: Option<String>,
    /// Include installed, uninstalled or both. Example: "True,False". Optional.
    #[serde(rename = "isUninstalled", skip_serializing_if = "Option::is_none")]
    pub is_uninstalled: Option<String>,
    /// Free-text filter by computer name or uuid (supports multiple values). Optional.
    #[serde(rename = "computerNameOrUuid__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name_or_uuid__contains: Option<String>,
    /// Included Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "idsNin", skip_serializing_if = "Option::is_none")]
    pub ids_nin: Option<String>,
    /// Include all Agents matching this saved filter. Example: "225494730938493804". Optional.
    #[serde(rename = "filterId", skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<String>,
    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gte: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lt: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lte: Option<String>,
    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gt: Option<String>,
    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "decommissionedAt__between", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__between: Option<String>,
    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at__lt: Option<String>,
    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at__lte: Option<String>,
    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at__gt: Option<String>,
    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at__gte: Option<String>,
    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at__between: Option<String>,
    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at__lt: Option<String>,
    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at__lte: Option<String>,
    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at__gt: Option<String>,
    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at__gte: Option<String>,
    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at__between: Option<String>,
    /// Match computer name partially (substring). Example: "Lab1". Optional.
    #[serde(rename = "computerName__like", skip_serializing_if = "Option::is_none")]
    pub computer_name__like: Option<String>,
    /// Computer name. Example: "My Office Desktop". Optional.
    #[serde(rename = "computerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// Agents versions less than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lt", skip_serializing_if = "Option::is_none")]
    pub agent_version__lt: Option<String>,
    /// Agents versions less than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lte", skip_serializing_if = "Option::is_none")]
    pub agent_version__lte: Option<String>,
    /// Agents versions greater than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gt", skip_serializing_if = "Option::is_none")]
    pub agent_version__gt: Option<String>,
    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gte", skip_serializing_if = "Option::is_none")]
    pub agent_version__gte: Option<String>,
    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144". Optional.
    #[serde(rename = "agentVersion__between", skip_serializing_if = "Option::is_none")]
    pub agent_version__between: Option<String>,
    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2". Optional.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22". Optional.
    #[serde(rename = "uuids", skip_serializing_if = "Option::is_none")]
    pub uuids: Option<String>,
    /// Scan status. Example: "none". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatus", skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// Include only Agents that have threats with this mitigation status. Example: "mitigated". Optional.
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    #[serde(rename = "threatMitigationStatus", skip_serializing_if = "Option::is_none")]
    pub threat_mitigation_status: Option<String>,
    /// Include only Agents with at least one resolved threat. Optional.
    #[serde(rename = "threatResolved", skip_serializing_if = "Option::is_none")]
    pub threat_resolved: Option<bool>,
    /// Include only Agents with at least one hidden threat. Optional.
    #[serde(rename = "threatHidden", skip_serializing_if = "Option::is_none")]
    pub threat_hidden: Option<bool>,
    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94". Optional.
    #[serde(rename = "threatContentHash", skip_serializing_if = "Option::is_none")]
    pub threat_content_hash: Option<String>,
    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lt: Option<String>,
    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lte: Option<String>,
    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gt: Option<String>,
    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gte: Option<String>,
    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "threatCreatedAt__between", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__between: Option<String>,
    /// Include Agents with this amount of active threats. Example: "3". Optional.
    #[serde(rename = "activeThreats", skip_serializing_if = "Option::is_none")]
    pub active_threats: Option<i64>,
    /// Include Agents with at least this amount of active threats. Example: "5". Optional.
    #[serde(rename = "activeThreats__gt", skip_serializing_if = "Option::is_none")]
    pub active_threats__gt: Option<i64>,
    /// Agent mitigation mode policy. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationMode", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode: Option<String>,
    /// Mitigation mode policy for suspicious activity. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationModeSuspicious", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode_suspicious: Option<String>,
    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lt", skip_serializing_if = "Option::is_none")]
    pub registered_at__lt: Option<String>,
    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lte", skip_serializing_if = "Option::is_none")]
    pub registered_at__lte: Option<String>,
    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gt", skip_serializing_if = "Option::is_none")]
    pub registered_at__gt: Option<String>,
    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gte", skip_serializing_if = "Option::is_none")]
    pub registered_at__gte: Option<String>,
    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lt: Option<String>,
    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lte: Option<String>,
    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gt: Option<String>,
    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gte: Option<String>,
    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lt: Option<String>,
    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lte: Option<String>,
    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gt: Option<String>,
    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gte: Option<String>,
    /// CPU cores (less than). Optional.
    #[serde(rename = "coreCount__lt", skip_serializing_if = "Option::is_none")]
    pub core_count__lt: Option<i64>,
    /// CPU cores (less than or equal). Optional.
    #[serde(rename = "coreCount__lte", skip_serializing_if = "Option::is_none")]
    pub core_count__lte: Option<i64>,
    /// CPU cores (more than). Optional.
    #[serde(rename = "coreCount__gt", skip_serializing_if = "Option::is_none")]
    pub core_count__gt: Option<i64>,
    /// CPU cores (more than or equal). Optional.
    #[serde(rename = "coreCount__gte", skip_serializing_if = "Option::is_none")]
    pub core_count__gte: Option<i64>,
    /// Number of CPUs (less than). Optional.
    #[serde(rename = "cpuCount__lt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lt: Option<i64>,
    /// Number of CPUs (less than or equal). Optional.
    #[serde(rename = "cpuCount__lte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lte: Option<i64>,
    /// Number of CPUs (more than). Optional.
    #[serde(rename = "cpuCount__gt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gt: Option<i64>,
    /// Number of CPUs (more than or equal). Optional.
    #[serde(rename = "cpuCount__gte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gte: Option<i64>,
    /// Memory size (MB, less than). Optional.
    #[serde(rename = "totalMemory__lt", skip_serializing_if = "Option::is_none")]
    pub total_memory__lt: Option<i64>,
    /// Memory size (MB, less than or equal). Optional.
    #[serde(rename = "totalMemory__lte", skip_serializing_if = "Option::is_none")]
    pub total_memory__lte: Option<i64>,
    /// Memory size (MB, more than). Optional.
    #[serde(rename = "totalMemory__gt", skip_serializing_if = "Option::is_none")]
    pub total_memory__gt: Option<i64>,
    /// Memory size (MB, more than or equal). Optional.
    #[serde(rename = "totalMemory__gte", skip_serializing_if = "Option::is_none")]
    pub total_memory__gte: Option<i64>,
    /// Migration status. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "migrationStatus", skip_serializing_if = "Option::is_none")]
    pub migration_status: Option<String>,
    /// Gateway ip. Example: "192.168.0.1". Optional.
    #[serde(rename = "gatewayIp", skip_serializing_if = "Option::is_none")]
    pub gateway_ip: Option<String>,
    /// The ID of the CSV file to filter by. Example: "225494730938493804". Optional.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<String>,
    /// Supported Remote Script Orchestration level. Example: "none". Optional.
    ///
    /// Allowed values: none, pro, ars.
    #[serde(rename = "rsoLevel", skip_serializing_if = "Option::is_none")]
    pub rso_level: Option<String>,
    /// Include only agents that has Remote Ops Forensicsfeature supported. Optional.
    #[serde(rename = "remoteOpsForensicsSupported", skip_serializing_if = "Option::is_none")]
    pub remote_ops_forensics_supported: Option<bool>,
    /// Agents os revision than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__gte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__gte: Option<i64>,
    /// Agents os revision lower than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__lte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__lte: Option<i64>,
    /// A list of included rso_levels. Example: "pro,ars". Optional.
    #[serde(rename = "rsoLevels", skip_serializing_if = "Option::is_none")]
    pub rso_levels: Option<String>,
}

impl AgentsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000 items, use "cursor". Example: "150".
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }

    /// Limit number of returned items (1-1000). Example: "10".
    pub fn limit(mut self, n: u32) -> Self {
        self.limit = Some(n);
        self
    }

    /// Cursor position returned by the last request. Use to iterate over more than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=".
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }

    /// If true, only total number of items will be returned, without any of the actual objects.
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }

    /// If true, total number of items will not be calculated, which speeds up execution time.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }

    /// The column to sort the results by. Example: "id".
    ///
    /// Allowed values: createdAt, updatedAt, id, computerName, groupId, osType, osName, uuid, scanStatus, isDecommissioned, decommissionedAt, isUninstalled, threatMitigationStatus, threatResolved, threatContentHash, threatCreatedAt, registeredAt, lastActiveDate, isActive, isPendingUninstall, activeThreats, isUpToDate, agentVersion, osArch, machineType, networkStatus, domain, mitigationMode, mitigationModeSuspicious, encryptedApplications, totalMemory, cpuCount, coreCount, externalIp, lastLoggedInUserName, groupName, networkInterfacePhysical, siteId, siteName, infected, threatRebootRequired, consoleMigrationStatus, appsVulnerabilityStatus, accountName, externalId, installerType, rangerVersion, operationalState, remoteProfilingState, networkQuarantineEnabled, firewallEnabled, locationEnabled, cloudAccount, cloudImage, cloudInstanceId, cloudInstanceSize, cloudLocation, cloudNetwork, cloudProvider, clusterName, kubernetesVersion, kubernetesType, agentNamespace, agentPodName, serialNumber, cpuId, fullDiskScanLastUpdatedAt, lastSuccessfulScanDate, ecsType, ecsVersion, ecsClusterName, ecsTaskArn, ecsTaskAvailabilityZone, ecsServiceName, ecsServiceArn, ecsTaskDefinitionFamily, ecsTaskDefinitionRevision, ecsTaskDefinitionArn, isAdConnector, activeProtection, pacFileUsage, proxyMethod, isMgmtProxyEnabled, console, mgmtProxyAddress, isEventSearchProxyEnabled, deepVisibility, eventSearchAddress, entraId.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }

    /// Sort direction. Example: "asc".
    ///
    /// Allowed values: asc, desc.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }

    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }

    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_group_ids = Some(join_csv(vals));
        self
    }

    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_site_ids = Some(join_csv(vals));
        self
    }

    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn registered_at__between(mut self, v: impl Into<String>) -> Self {
        self.registered_at__between = Some(v.into());
        self
    }

    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_active_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__between = Some(v.into());
        self
    }

    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_successful_scan_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__between = Some(v.into());
        self
    }

    /// Include only active Agents.
    pub fn is_active<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_active = Some(join_csv(vals));
        self
    }

    /// Include only Agents with pending uninstall requests.
    pub fn is_pending_uninstall<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_pending_uninstall = Some(join_csv(vals));
        self
    }

    /// Include only Agents with at least one active threat.
    pub fn infected(mut self, b: bool) -> Self {
        self.infected = Some(b);
        self
    }

    /// Include only Agents with updated software.
    pub fn is_up_to_date<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_up_to_date = Some(join_csv(vals));
        self
    }

    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux".
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }

    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions = Some(join_csv(vals));
        self
    }

    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions_nin = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions_nin = Some(join_csv(vals));
        self
    }

    /// OS architecture. Example: "32 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arch(mut self, v: impl Into<String>) -> Self {
        self.os_arch = Some(v.into());
        self
    }

    /// OS architectures to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches = Some(join_csv(vals));
        self
    }

    /// OS architectures not to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches_nin = Some(join_csv(vals));
        self
    }

    /// Included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(vals));
        self
    }

    /// Not included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types_nin = Some(join_csv(vals));
        self
    }

    /// Included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses = Some(join_csv(vals));
        self
    }

    /// Not included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types = Some(join_csv(vals));
        self
    }

    /// Not included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types_nin = Some(join_csv(vals));
        self
    }

    /// Included storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types = Some(join_csv(vals));
        self
    }

    /// Excluded storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types_nin = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains = Some(join_csv(vals));
        self
    }

    /// Not included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains_nin = Some(join_csv(vals));
        self
    }

    /// Disk encryption status.
    pub fn encrypted_applications<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encrypted_applications = Some(join_csv(vals));
        self
    }

    /// Total memory range (GB, inclusive). Example: "4-8".
    pub fn total_memory__between(mut self, v: impl Into<String>) -> Self {
        self.total_memory__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn core_count__between(mut self, v: impl Into<String>) -> Self {
        self.core_count__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn cpu_count__between(mut self, v: impl Into<String>) -> Self {
        self.cpu_count__between = Some(v.into());
        self
    }

    /// Included pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed = Some(join_csv(vals));
        self
    }

    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions = Some(join_csv(vals));
        self
    }

    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed_nin = Some(join_csv(vals));
        self
    }

    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions_nin = Some(join_csv(vals));
        self
    }

    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com".
    pub fn ad_query(mut self, v: impl Into<String>) -> Self {
        self.ad_query = Some(v.into());
        self
    }

    /// Agent has a local configuration set.
    pub fn has_local_configuration(mut self, b: bool) -> Self {
        self.has_local_configuration = Some(b);
        self
    }

    /// Migration status in. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses = Some(join_csv(vals));
        self
    }

    /// Migration status nin. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status in. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status nin. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids = Some(join_csv(vals));
        self
    }

    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types = Some(join_csv(vals));
        self
    }

    /// Exclude Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types_nin = Some(join_csv(vals));
        self
    }

    /// Agent operational state.
    pub fn operational_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent operational states.
    pub fn operational_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states_nin = Some(join_csv(vals));
        self
    }

    /// Agent remote profiling state.
    pub fn remote_profiling_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent remote profiling states.
    pub fn remote_profiling_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states_nin = Some(join_csv(vals));
        self
    }

    /// Status of Network Discovery. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses = Some(join_csv(vals));
        self
    }

    /// Do not include these Network Scanner Statuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses_nin = Some(join_csv(vals));
        self
    }

    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_status(mut self, v: impl Into<String>) -> Self {
        self.ranger_status = Some(v.into());
        self
    }

    /// Has at least one threat with at least one mitigation action pending reboot to succeed.
    pub fn threat_reboot_required<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_reboot_required = Some(join_csv(vals));
        self
    }

    /// The agents supports Network Quarantine Control and its enabled for the agent's group.
    pub fn network_quarantine_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_quarantine_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Firewall Control and it is enabled for the agent's group.
    pub fn firewall_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.firewall_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Location Awareness and it is enabled for the agent's group.
    pub fn location_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_enabled = Some(join_csv(vals));
        self
    }

    /// Agents from which cloud provider.
    pub fn cloud_provider<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider = Some(join_csv(vals));
        self
    }

    /// Exclude Agents from these cloud provider.
    pub fn cloud_provider_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_nin = Some(join_csv(vals));
        self
    }

    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}".
    pub fn tags_data(mut self, v: impl Into<String>) -> Self {
        self.tags_data = Some(v.into());
        self
    }

    /// Include only Agents that have any tags assigned if True, or none if False.
    pub fn has_tags(mut self, b: bool) -> Self {
        self.has_tags = Some(b);
        self
    }

    /// The agents that are ADConnectors if True, or not if False.
    pub fn is_ad_connector<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(join_csv(vals));
        self
    }

    /// The agents has Hyper Automate PNA enabled if True, or not if False.
    pub fn is_hyper_automate<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_hyper_automate = Some(join_csv(vals));
        self
    }

    /// Included agent active protections. Example: "edr,idr".
    ///
    /// Allowed values: edr, idr.
    pub fn active_protection<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_protection = Some(join_csv(vals));
        self
    }

    /// Containerized workload counts.
    pub fn containerized_workload_counts(mut self, v: impl Into<String>) -> Self {
        self.containerized_workload_counts = Some(v.into());
        self
    }

    /// Indicates whether the agent protects containerized workload at the moment.
    pub fn has_containerized_workload<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.has_containerized_workload = Some(join_csv(vals));
        self
    }

    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported).
    pub fn pac_file_usage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.pac_file_usage = Some(join_csv(vals));
        self
    }

    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method = Some(join_csv(vals));
        self
    }

    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported).
    pub fn is_mgmt_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_mgmt_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported).
    pub fn is_event_search_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_event_search_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0".
    pub fn external_ip__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN".
    pub fn computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0".
    pub fn network_interface_inet__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_inet__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_physical__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_physical__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_gateway_mac_address__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_gateway_mac_address__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1".
    pub fn last_logged_in_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.last_logged_in_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1".
    pub fn os_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John".
    pub fn ad_user_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows".
    pub fn ad_computer_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055".
    pub fn uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuid__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine".
    pub fn external_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws role(supports multiple values).
    pub fn aws_role__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_role__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws securityGroups(supports multiple values).
    pub fn aws_security_groups__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_security_groups__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws subnet ids (supports multiple values).
    pub fn aws_subnet_ids__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_subnet_ids__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent namespace (supports multiple values).
    pub fn agent_namespace__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_namespace__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent pod name (supports multiple values).
    pub fn agent_pod_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pod_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by azure resource group(supports multiple values).
    pub fn azure_resource_group__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.azure_resource_group__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud account (supports multiple values).
    pub fn cloud_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud image (supports multiple values).
    pub fn cloud_image__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_image__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance id(supports multiple values).
    pub fn cloud_instance_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance size(supports multiple values).
    pub fn cloud_instance_size__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_size__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud location (supports multiple values).
    pub fn cloud_location__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_location__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud network (supports multiple values).
    pub fn cloud_network__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_network__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud tags (supports multiple values).
    pub fn cloud_tags__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cluster name (supports multiple values).
    pub fn cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by gcp service account (supports multiple values).
    pub fn gcp_service_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gcp_service_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node labels (supports multiple values).
    pub fn k8s_node_labels__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_labels__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node name (supports multiple values).
    pub fn k8s_node_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s type(supports multiple values).
    pub fn k8s_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s version (supports multiple values).
    pub fn k8s_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by live update ID (supports multiple values).
    pub fn live_update_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.live_update_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Serial Number (supports multiple values).
    pub fn serial_number__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Entra ID (supports multiple values).
    pub fn entra_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.entra_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD".
    pub fn cpu_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS type.
    pub fn ecs_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS version.
    pub fn ecs_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS cluster name.
    pub fn ecs_cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task arn.
    pub fn ecs_task_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task availability zone.
    pub fn ecs_task_availability_zone__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_availability_zone__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service name.
    pub fn ecs_service_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service arn.
    pub fn ecs_service_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition family.
    pub fn ecs_task_definition_family__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_family__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition revision.
    pub fn ecs_task_definition_revision__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_revision__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition arn.
    pub fn ecs_task_definition_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_arn__contains = Some(join_csv(vals));
        self
    }

    /// Include active, decommissioned or both. Example: "True,False".
    pub fn is_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_decommissioned = Some(join_csv(vals));
        self
    }

    /// Include installed, uninstalled or both. Example: "True,False".
    pub fn is_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_uninstalled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name or uuid (supports multiple values).
    pub fn computer_name_or_uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name_or_uuid__contains = Some(join_csv(vals));
        self
    }

    /// Included Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(vals));
        self
    }

    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids_nin = Some(join_csv(vals));
        self
    }

    /// Include all Agents matching this saved filter. Example: "225494730938493804".
    pub fn filter_id(mut self, v: impl Into<String>) -> Self {
        self.filter_id = Some(v.into());
        self
    }

    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gte = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lt = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lte = Some(v.into());
        self
    }

    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gt = Some(v.into());
        self
    }

    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn decommissioned_at__between(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__between = Some(v.into());
        self
    }

    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.created_at__lt = Some(v.into());
        self
    }

    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.created_at__lte = Some(v.into());
        self
    }

    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.created_at__gt = Some(v.into());
        self
    }

    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.created_at__gte = Some(v.into());
        self
    }

    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn created_at__between(mut self, v: impl Into<String>) -> Self {
        self.created_at__between = Some(v.into());
        self
    }

    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lt = Some(v.into());
        self
    }

    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lte = Some(v.into());
        self
    }

    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gt = Some(v.into());
        self
    }

    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gte = Some(v.into());
        self
    }

    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.updated_at__between = Some(v.into());
        self
    }

    /// Match computer name partially (substring). Example: "Lab1".
    pub fn computer_name__like(mut self, v: impl Into<String>) -> Self {
        self.computer_name__like = Some(v.into());
        self
    }

    /// Computer name. Example: "My Office Desktop".
    pub fn computer_name(mut self, v: impl Into<String>) -> Self {
        self.computer_name = Some(v.into());
        self
    }

    /// Agents versions less than given version. Example: "2.5.1.1320".
    pub fn agent_version__lt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lt = Some(v.into());
        self
    }

    /// Agents versions less than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__lte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lte = Some(v.into());
        self
    }

    /// Agents versions greater than given version. Example: "2.5.1.1320".
    pub fn agent_version__gt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gt = Some(v.into());
        self
    }

    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__gte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gte = Some(v.into());
        self
    }

    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144".
    pub fn agent_version__between(mut self, v: impl Into<String>) -> Self {
        self.agent_version__between = Some(v.into());
        self
    }

    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2".
    pub fn uuid(mut self, v: impl Into<String>) -> Self {
        self.uuid = Some(v.into());
        self
    }

    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22".
    pub fn uuids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuids = Some(join_csv(vals));
        self
    }

    /// Scan status. Example: "none".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_status(mut self, v: impl Into<String>) -> Self {
        self.scan_status = Some(v.into());
        self
    }

    /// Include only Agents that have threats with this mitigation status. Example: "mitigated".
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    pub fn threat_mitigation_status(mut self, v: impl Into<String>) -> Self {
        self.threat_mitigation_status = Some(v.into());
        self
    }

    /// Include only Agents with at least one resolved threat.
    pub fn threat_resolved(mut self, b: bool) -> Self {
        self.threat_resolved = Some(b);
        self
    }

    /// Include only Agents with at least one hidden threat.
    pub fn threat_hidden(mut self, b: bool) -> Self {
        self.threat_hidden = Some(b);
        self
    }

    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94".
    pub fn threat_content_hash(mut self, v: impl Into<String>) -> Self {
        self.threat_content_hash = Some(v.into());
        self
    }

    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lt = Some(v.into());
        self
    }

    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lte = Some(v.into());
        self
    }

    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gt = Some(v.into());
        self
    }

    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gte = Some(v.into());
        self
    }

    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn threat_created_at__between(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__between = Some(v.into());
        self
    }

    /// Include Agents with this amount of active threats. Example: "3".
    pub fn active_threats(mut self, n: i64) -> Self {
        self.active_threats = Some(n);
        self
    }

    /// Include Agents with at least this amount of active threats. Example: "5".
    pub fn active_threats__gt(mut self, n: i64) -> Self {
        self.active_threats__gt = Some(n);
        self
    }

    /// Agent mitigation mode policy. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode = Some(v.into());
        self
    }

    /// Mitigation mode policy for suspicious activity. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode_suspicious(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode_suspicious = Some(v.into());
        self
    }

    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lt = Some(v.into());
        self
    }

    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lte = Some(v.into());
        self
    }

    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gt = Some(v.into());
        self
    }

    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gte = Some(v.into());
        self
    }

    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lt = Some(v.into());
        self
    }

    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lte = Some(v.into());
        self
    }

    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gt = Some(v.into());
        self
    }

    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gte = Some(v.into());
        self
    }

    /// CPU cores (less than).
    pub fn core_count__lt(mut self, n: i64) -> Self {
        self.core_count__lt = Some(n);
        self
    }

    /// CPU cores (less than or equal).
    pub fn core_count__lte(mut self, n: i64) -> Self {
        self.core_count__lte = Some(n);
        self
    }

    /// CPU cores (more than).
    pub fn core_count__gt(mut self, n: i64) -> Self {
        self.core_count__gt = Some(n);
        self
    }

    /// CPU cores (more than or equal).
    pub fn core_count__gte(mut self, n: i64) -> Self {
        self.core_count__gte = Some(n);
        self
    }

    /// Number of CPUs (less than).
    pub fn cpu_count__lt(mut self, n: i64) -> Self {
        self.cpu_count__lt = Some(n);
        self
    }

    /// Number of CPUs (less than or equal).
    pub fn cpu_count__lte(mut self, n: i64) -> Self {
        self.cpu_count__lte = Some(n);
        self
    }

    /// Number of CPUs (more than).
    pub fn cpu_count__gt(mut self, n: i64) -> Self {
        self.cpu_count__gt = Some(n);
        self
    }

    /// Number of CPUs (more than or equal).
    pub fn cpu_count__gte(mut self, n: i64) -> Self {
        self.cpu_count__gte = Some(n);
        self
    }

    /// Memory size (MB, less than).
    pub fn total_memory__lt(mut self, n: i64) -> Self {
        self.total_memory__lt = Some(n);
        self
    }

    /// Memory size (MB, less than or equal).
    pub fn total_memory__lte(mut self, n: i64) -> Self {
        self.total_memory__lte = Some(n);
        self
    }

    /// Memory size (MB, more than).
    pub fn total_memory__gt(mut self, n: i64) -> Self {
        self.total_memory__gt = Some(n);
        self
    }

    /// Memory size (MB, more than or equal).
    pub fn total_memory__gte(mut self, n: i64) -> Self {
        self.total_memory__gte = Some(n);
        self
    }

    /// Migration status. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn migration_status(mut self, v: impl Into<String>) -> Self {
        self.migration_status = Some(v.into());
        self
    }

    /// Gateway ip. Example: "192.168.0.1".
    pub fn gateway_ip(mut self, v: impl Into<String>) -> Self {
        self.gateway_ip = Some(v.into());
        self
    }

    /// The ID of the CSV file to filter by. Example: "225494730938493804".
    pub fn csv_filter_id(mut self, v: impl Into<String>) -> Self {
        self.csv_filter_id = Some(v.into());
        self
    }

    /// Supported Remote Script Orchestration level. Example: "none".
    ///
    /// Allowed values: none, pro, ars.
    pub fn rso_level(mut self, v: impl Into<String>) -> Self {
        self.rso_level = Some(v.into());
        self
    }

    /// Include only agents that has Remote Ops Forensicsfeature supported.
    pub fn remote_ops_forensics_supported(mut self, b: bool) -> Self {
        self.remote_ops_forensics_supported = Some(b);
        self
    }

    /// Agents os revision than or equal to given version.
    pub fn windows_os_revision__gte(mut self, n: i64) -> Self {
        self.windows_os_revision__gte = Some(n);
        self
    }

    /// Agents os revision lower than or equal to given version.
    pub fn windows_os_revision__lte(mut self, n: i64) -> Self {
        self.windows_os_revision__lte = Some(n);
        self
    }

    /// A list of included rso_levels. Example: "pro,ars".
    pub fn rso_levels<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.rso_levels = Some(join_csv(vals));
        self
    }
}

/// Query params for `GET /web/api/v2.1/agents/count` (Count Agents).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
pub struct AgentsCountQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredGroupIds", skip_serializing_if = "Option::is_none")]
    pub filtered_group_ids: Option<String>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredSiteIds", skip_serializing_if = "Option::is_none")]
    pub filtered_site_ids: Option<String>,
    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "registeredAt__between", skip_serializing_if = "Option::is_none")]
    pub registered_at__between: Option<String>,
    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastActiveDate__between", skip_serializing_if = "Option::is_none")]
    pub last_active_date__between: Option<String>,
    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastSuccessfulScanDate__between", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__between: Option<String>,
    /// Include only active Agents. Optional.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<String>,
    /// Include only Agents with pending uninstall requests. Optional.
    #[serde(rename = "isPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub is_pending_uninstall: Option<String>,
    /// Include only Agents with at least one active threat. Optional.
    #[serde(rename = "infected", skip_serializing_if = "Option::is_none")]
    pub infected: Option<bool>,
    /// Include only Agents with updated software. Optional.
    #[serde(rename = "isUpToDate", skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<String>,
    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux". Optional.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersions", skip_serializing_if = "Option::is_none")]
    pub agent_versions: Option<String>,
    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersionsNin", skip_serializing_if = "Option::is_none")]
    pub agent_versions_nin: Option<String>,
    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersions", skip_serializing_if = "Option::is_none")]
    pub ranger_versions: Option<String>,
    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersionsNin", skip_serializing_if = "Option::is_none")]
    pub ranger_versions_nin: Option<String>,
    /// OS architecture. Example: "32 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArch", skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<String>,
    /// OS architectures to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArches", skip_serializing_if = "Option::is_none")]
    pub os_arches: Option<String>,
    /// OS architectures not to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArchesNin", skip_serializing_if = "Option::is_none")]
    pub os_arches_nin: Option<String>,
    /// Included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypes", skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Not included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypesNin", skip_serializing_if = "Option::is_none")]
    pub os_types_nin: Option<String>,
    /// Included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatuses", skip_serializing_if = "Option::is_none")]
    pub scan_statuses: Option<String>,
    /// Not included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatusesNin", skip_serializing_if = "Option::is_none")]
    pub scan_statuses_nin: Option<String>,
    /// Included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypes", skip_serializing_if = "Option::is_none")]
    pub machine_types: Option<String>,
    /// Not included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypesNin", skip_serializing_if = "Option::is_none")]
    pub machine_types_nin: Option<String>,
    /// Included storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypes", skip_serializing_if = "Option::is_none")]
    pub storage_types: Option<String>,
    /// Excluded storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypesNin", skip_serializing_if = "Option::is_none")]
    pub storage_types_nin: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatuses", skip_serializing_if = "Option::is_none")]
    pub network_statuses: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatusesNin", skip_serializing_if = "Option::is_none")]
    pub network_statuses_nin: Option<String>,
    /// Included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// Not included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domainsNin", skip_serializing_if = "Option::is_none")]
    pub domains_nin: Option<String>,
    /// Disk encryption status. Optional.
    #[serde(rename = "encryptedApplications", skip_serializing_if = "Option::is_none")]
    pub encrypted_applications: Option<String>,
    /// Total memory range (GB, inclusive). Example: "4-8". Optional.
    #[serde(rename = "totalMemory__between", skip_serializing_if = "Option::is_none")]
    pub total_memory__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "coreCount__between", skip_serializing_if = "Option::is_none")]
    pub core_count__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "cpuCount__between", skip_serializing_if = "Option::is_none")]
    pub cpu_count__between: Option<String>,
    /// Included pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeeded", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed: Option<String>,
    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissions", skip_serializing_if = "Option::is_none")]
    pub missing_permissions: Option<String>,
    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeededNin", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed_nin: Option<String>,
    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissionsNin", skip_serializing_if = "Option::is_none")]
    pub missing_permissions_nin: Option<String>,
    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com". Optional.
    #[serde(rename = "adQuery", skip_serializing_if = "Option::is_none")]
    pub ad_query: Option<String>,
    /// Agent has a local configuration set. Optional.
    #[serde(rename = "hasLocalConfiguration", skip_serializing_if = "Option::is_none")]
    pub has_local_configuration: Option<bool>,
    /// Migration status in. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatuses", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses: Option<String>,
    /// Migration status nin. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatusesNin", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses_nin: Option<String>,
    /// Apps vulnerability status in. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatuses", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses: Option<String>,
    /// Apps vulnerability status nin. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatusesNin", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses_nin: Option<String>,
    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIds", skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<String>,
    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIdsNin", skip_serializing_if = "Option::is_none")]
    pub location_ids_nin: Option<String>,
    /// Include only Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypes", skip_serializing_if = "Option::is_none")]
    pub installer_types: Option<String>,
    /// Exclude Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypesNin", skip_serializing_if = "Option::is_none")]
    pub installer_types_nin: Option<String>,
    /// Agent operational state. Optional.
    #[serde(rename = "operationalStates", skip_serializing_if = "Option::is_none")]
    pub operational_states: Option<String>,
    /// Do not include these Agent operational states. Optional.
    #[serde(rename = "operationalStatesNin", skip_serializing_if = "Option::is_none")]
    pub operational_states_nin: Option<String>,
    /// Agent remote profiling state. Optional.
    #[serde(rename = "remoteProfilingStates", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states: Option<String>,
    /// Do not include these Agent remote profiling states. Optional.
    #[serde(rename = "remoteProfilingStatesNin", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states_nin: Option<String>,
    /// Status of Network Discovery. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatuses", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses: Option<String>,
    /// Do not include these Network Scanner Statuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatusesNin", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses_nin: Option<String>,
    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatus", skip_serializing_if = "Option::is_none")]
    pub ranger_status: Option<String>,
    /// Has at least one threat with at least one mitigation action pending reboot to succeed. Optional.
    #[serde(rename = "threatRebootRequired", skip_serializing_if = "Option::is_none")]
    pub threat_reboot_required: Option<String>,
    /// The agents supports Network Quarantine Control and its enabled for the agent's group. Optional.
    #[serde(rename = "networkQuarantineEnabled", skip_serializing_if = "Option::is_none")]
    pub network_quarantine_enabled: Option<String>,
    /// The agents supports Firewall Control and it is enabled for the agent's group. Optional.
    #[serde(rename = "firewallEnabled", skip_serializing_if = "Option::is_none")]
    pub firewall_enabled: Option<String>,
    /// The agents supports Location Awareness and it is enabled for the agent's group. Optional.
    #[serde(rename = "locationEnabled", skip_serializing_if = "Option::is_none")]
    pub location_enabled: Option<String>,
    /// Agents from which cloud provider. Optional.
    #[serde(rename = "cloudProvider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// Exclude Agents from these cloud provider. Optional.
    #[serde(rename = "cloudProviderNin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_nin: Option<String>,
    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}". Optional.
    #[serde(rename = "tagsData", skip_serializing_if = "Option::is_none")]
    pub tags_data: Option<String>,
    /// Include only Agents that have any tags assigned if True, or none if False. Optional.
    #[serde(rename = "hasTags", skip_serializing_if = "Option::is_none")]
    pub has_tags: Option<bool>,
    /// The agents that are ADConnectors if True, or not if False. Optional.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The agents has Hyper Automate PNA enabled if True, or not if False. Optional.
    #[serde(rename = "isHyperAutomate", skip_serializing_if = "Option::is_none")]
    pub is_hyper_automate: Option<String>,
    /// Included agent active protections. Example: "edr,idr". Optional.
    ///
    /// Allowed values: edr, idr.
    #[serde(rename = "activeProtection", skip_serializing_if = "Option::is_none")]
    pub active_protection: Option<String>,
    /// Containerized workload counts. Optional.
    #[serde(rename = "containerizedWorkloadCounts", skip_serializing_if = "Option::is_none")]
    pub containerized_workload_counts: Option<String>,
    /// Indicates whether the agent protects containerized workload at the moment. Optional.
    #[serde(rename = "hasContainerizedWorkload", skip_serializing_if = "Option::is_none")]
    pub has_containerized_workload: Option<String>,
    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "pacFileUsage", skip_serializing_if = "Option::is_none")]
    pub pac_file_usage: Option<String>,
    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethod", skip_serializing_if = "Option::is_none")]
    pub proxy_method: Option<String>,
    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethodNin", skip_serializing_if = "Option::is_none")]
    pub proxy_method_nin: Option<String>,
    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isMgmtProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_mgmt_proxy_enabled: Option<String>,
    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isEventSearchProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_event_search_proxy_enabled: Option<String>,
    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0". Optional.
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip__contains: Option<String>,
    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN". Optional.
    #[serde(rename = "computerName__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name__contains: Option<String>,
    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0". Optional.
    #[serde(rename = "networkInterfaceInet__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_inet__contains: Option<String>,
    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfacePhysical__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_physical__contains: Option<String>,
    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfaceGatewayMacAddress__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_gateway_mac_address__contains: Option<String>,
    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1". Optional.
    #[serde(rename = "lastLoggedInUserName__contains", skip_serializing_if = "Option::is_none")]
    pub last_logged_in_user_name__contains: Option<String>,
    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1". Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_query__contains: Option<String>,
    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_name__contains: Option<String>,
    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John". Optional.
    #[serde(rename = "adUserQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_query__contains: Option<String>,
    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_name__contains: Option<String>,
    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows". Optional.
    #[serde(rename = "adComputerQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_query__contains: Option<String>,
    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055". Optional.
    #[serde(rename = "uuid__contains", skip_serializing_if = "Option::is_none")]
    pub uuid__contains: Option<String>,
    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine". Optional.
    #[serde(rename = "externalId__contains", skip_serializing_if = "Option::is_none")]
    pub external_id__contains: Option<String>,
    /// Free-text filter by aws role(supports multiple values). Optional.
    #[serde(rename = "awsRole__contains", skip_serializing_if = "Option::is_none")]
    pub aws_role__contains: Option<String>,
    /// Free-text filter by aws securityGroups(supports multiple values). Optional.
    #[serde(rename = "awsSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub aws_security_groups__contains: Option<String>,
    /// Free-text filter by aws subnet ids (supports multiple values). Optional.
    #[serde(rename = "awsSubnetIds__contains", skip_serializing_if = "Option::is_none")]
    pub aws_subnet_ids__contains: Option<String>,
    /// Free-text filter by agent namespace (supports multiple values). Optional.
    #[serde(rename = "agentNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub agent_namespace__contains: Option<String>,
    /// Free-text filter by agent pod name (supports multiple values). Optional.
    #[serde(rename = "agentPodName__contains", skip_serializing_if = "Option::is_none")]
    pub agent_pod_name__contains: Option<String>,
    /// Free-text filter by azure resource group(supports multiple values). Optional.
    #[serde(rename = "azureResourceGroup__contains", skip_serializing_if = "Option::is_none")]
    pub azure_resource_group__contains: Option<String>,
    /// Free-text filter by cloud account (supports multiple values). Optional.
    #[serde(rename = "cloudAccount__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_account__contains: Option<String>,
    /// Free-text filter by cloud image (supports multiple values). Optional.
    #[serde(rename = "cloudImage__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_image__contains: Option<String>,
    /// Free-text filter by cloud instance id(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_id__contains: Option<String>,
    /// Free-text filter by cloud instance size(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceSize__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_size__contains: Option<String>,
    /// Free-text filter by cloud location (supports multiple values). Optional.
    #[serde(rename = "cloudLocation__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_location__contains: Option<String>,
    /// Free-text filter by cloud network (supports multiple values). Optional.
    #[serde(rename = "cloudNetwork__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_network__contains: Option<String>,
    /// Free-text filter by cloud tags (supports multiple values). Optional.
    #[serde(rename = "cloudTags__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags__contains: Option<String>,
    /// Free-text filter by cluster name (supports multiple values). Optional.
    #[serde(rename = "clusterName__contains", skip_serializing_if = "Option::is_none")]
    pub cluster_name__contains: Option<String>,
    /// Free-text filter by gcp service account (supports multiple values). Optional.
    #[serde(rename = "gcpServiceAccount__contains", skip_serializing_if = "Option::is_none")]
    pub gcp_service_account__contains: Option<String>,
    /// Free-text filter by K8s node labels (supports multiple values). Optional.
    #[serde(rename = "k8sNodeLabels__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_labels__contains: Option<String>,
    /// Free-text filter by K8s node name (supports multiple values). Optional.
    #[serde(rename = "k8sNodeName__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_name__contains: Option<String>,
    /// Free-text filter by K8s type(supports multiple values). Optional.
    #[serde(rename = "k8sType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_type__contains: Option<String>,
    /// Free-text filter by K8s version (supports multiple values). Optional.
    #[serde(rename = "k8sVersion__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_version__contains: Option<String>,
    /// Free-text filter by live update ID (supports multiple values). Optional.
    #[serde(rename = "liveUpdateId__contains", skip_serializing_if = "Option::is_none")]
    pub live_update_id__contains: Option<String>,
    /// Free-text filter by Serial Number (supports multiple values). Optional.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number__contains: Option<String>,
    /// Free-text filter by Entra ID (supports multiple values). Optional.
    #[serde(rename = "entraId__contains", skip_serializing_if = "Option::is_none")]
    pub entra_id__contains: Option<String>,
    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD". Optional.
    #[serde(rename = "cpuId__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_id__contains: Option<String>,
    /// Free-text filter by ECS type. Optional.
    #[serde(rename = "ecsType__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_type__contains: Option<String>,
    /// Free-text filter by ECS version. Optional.
    #[serde(rename = "ecsVersion__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_version__contains: Option<String>,
    /// Free-text filter by ECS cluster name. Optional.
    #[serde(rename = "ecsClusterName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_name__contains: Option<String>,
    /// Free-text filter by ECS task arn. Optional.
    #[serde(rename = "ecsTaskArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_arn__contains: Option<String>,
    /// Free-text filter by ECS task availability zone. Optional.
    #[serde(rename = "ecsTaskAvailabilityZone__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_availability_zone__contains: Option<String>,
    /// Free-text filter by ECS service name. Optional.
    #[serde(rename = "ecsServiceName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_name__contains: Option<String>,
    /// Free-text filter by ECS service arn. Optional.
    #[serde(rename = "ecsServiceArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_arn__contains: Option<String>,
    /// Free-text filter by ECS task definition family. Optional.
    #[serde(rename = "ecsTaskDefinitionFamily__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_family__contains: Option<String>,
    /// Free-text filter by ECS task definition revision. Optional.
    #[serde(rename = "ecsTaskDefinitionRevision__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_revision__contains: Option<String>,
    /// Free-text filter by ECS task definition arn. Optional.
    #[serde(rename = "ecsTaskDefinitionArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_arn__contains: Option<String>,
    /// Include active, decommissioned or both. Example: "True,False". Optional.
    #[serde(rename = "isDecommissioned", skip_serializing_if = "Option::is_none")]
    pub is_decommissioned: Option<String>,
    /// Include installed, uninstalled or both. Example: "True,False". Optional.
    #[serde(rename = "isUninstalled", skip_serializing_if = "Option::is_none")]
    pub is_uninstalled: Option<String>,
    /// Free-text filter by computer name or uuid (supports multiple values). Optional.
    #[serde(rename = "computerNameOrUuid__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name_or_uuid__contains: Option<String>,
    /// Included Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "idsNin", skip_serializing_if = "Option::is_none")]
    pub ids_nin: Option<String>,
    /// Include all Agents matching this saved filter. Example: "225494730938493804". Optional.
    #[serde(rename = "filterId", skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<String>,
    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gte: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lt: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lte: Option<String>,
    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gt: Option<String>,
    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "decommissionedAt__between", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__between: Option<String>,
    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at__lt: Option<String>,
    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at__lte: Option<String>,
    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at__gt: Option<String>,
    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at__gte: Option<String>,
    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at__between: Option<String>,
    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at__lt: Option<String>,
    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at__lte: Option<String>,
    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at__gt: Option<String>,
    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at__gte: Option<String>,
    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at__between: Option<String>,
    /// Match computer name partially (substring). Example: "Lab1". Optional.
    #[serde(rename = "computerName__like", skip_serializing_if = "Option::is_none")]
    pub computer_name__like: Option<String>,
    /// Computer name. Example: "My Office Desktop". Optional.
    #[serde(rename = "computerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// Agents versions less than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lt", skip_serializing_if = "Option::is_none")]
    pub agent_version__lt: Option<String>,
    /// Agents versions less than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lte", skip_serializing_if = "Option::is_none")]
    pub agent_version__lte: Option<String>,
    /// Agents versions greater than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gt", skip_serializing_if = "Option::is_none")]
    pub agent_version__gt: Option<String>,
    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gte", skip_serializing_if = "Option::is_none")]
    pub agent_version__gte: Option<String>,
    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144". Optional.
    #[serde(rename = "agentVersion__between", skip_serializing_if = "Option::is_none")]
    pub agent_version__between: Option<String>,
    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2". Optional.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22". Optional.
    #[serde(rename = "uuids", skip_serializing_if = "Option::is_none")]
    pub uuids: Option<String>,
    /// Scan status. Example: "none". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatus", skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// Include only Agents that have threats with this mitigation status. Example: "mitigated". Optional.
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    #[serde(rename = "threatMitigationStatus", skip_serializing_if = "Option::is_none")]
    pub threat_mitigation_status: Option<String>,
    /// Include only Agents with at least one resolved threat. Optional.
    #[serde(rename = "threatResolved", skip_serializing_if = "Option::is_none")]
    pub threat_resolved: Option<bool>,
    /// Include only Agents with at least one hidden threat. Optional.
    #[serde(rename = "threatHidden", skip_serializing_if = "Option::is_none")]
    pub threat_hidden: Option<bool>,
    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94". Optional.
    #[serde(rename = "threatContentHash", skip_serializing_if = "Option::is_none")]
    pub threat_content_hash: Option<String>,
    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lt: Option<String>,
    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lte: Option<String>,
    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gt: Option<String>,
    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gte: Option<String>,
    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "threatCreatedAt__between", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__between: Option<String>,
    /// Include Agents with this amount of active threats. Example: "3". Optional.
    #[serde(rename = "activeThreats", skip_serializing_if = "Option::is_none")]
    pub active_threats: Option<i64>,
    /// Include Agents with at least this amount of active threats. Example: "5". Optional.
    #[serde(rename = "activeThreats__gt", skip_serializing_if = "Option::is_none")]
    pub active_threats__gt: Option<i64>,
    /// Agent mitigation mode policy. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationMode", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode: Option<String>,
    /// Mitigation mode policy for suspicious activity. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationModeSuspicious", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode_suspicious: Option<String>,
    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lt", skip_serializing_if = "Option::is_none")]
    pub registered_at__lt: Option<String>,
    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lte", skip_serializing_if = "Option::is_none")]
    pub registered_at__lte: Option<String>,
    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gt", skip_serializing_if = "Option::is_none")]
    pub registered_at__gt: Option<String>,
    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gte", skip_serializing_if = "Option::is_none")]
    pub registered_at__gte: Option<String>,
    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lt: Option<String>,
    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lte: Option<String>,
    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gt: Option<String>,
    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gte: Option<String>,
    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lt: Option<String>,
    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lte: Option<String>,
    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gt: Option<String>,
    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gte: Option<String>,
    /// CPU cores (less than). Optional.
    #[serde(rename = "coreCount__lt", skip_serializing_if = "Option::is_none")]
    pub core_count__lt: Option<i64>,
    /// CPU cores (less than or equal). Optional.
    #[serde(rename = "coreCount__lte", skip_serializing_if = "Option::is_none")]
    pub core_count__lte: Option<i64>,
    /// CPU cores (more than). Optional.
    #[serde(rename = "coreCount__gt", skip_serializing_if = "Option::is_none")]
    pub core_count__gt: Option<i64>,
    /// CPU cores (more than or equal). Optional.
    #[serde(rename = "coreCount__gte", skip_serializing_if = "Option::is_none")]
    pub core_count__gte: Option<i64>,
    /// Number of CPUs (less than). Optional.
    #[serde(rename = "cpuCount__lt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lt: Option<i64>,
    /// Number of CPUs (less than or equal). Optional.
    #[serde(rename = "cpuCount__lte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lte: Option<i64>,
    /// Number of CPUs (more than). Optional.
    #[serde(rename = "cpuCount__gt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gt: Option<i64>,
    /// Number of CPUs (more than or equal). Optional.
    #[serde(rename = "cpuCount__gte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gte: Option<i64>,
    /// Memory size (MB, less than). Optional.
    #[serde(rename = "totalMemory__lt", skip_serializing_if = "Option::is_none")]
    pub total_memory__lt: Option<i64>,
    /// Memory size (MB, less than or equal). Optional.
    #[serde(rename = "totalMemory__lte", skip_serializing_if = "Option::is_none")]
    pub total_memory__lte: Option<i64>,
    /// Memory size (MB, more than). Optional.
    #[serde(rename = "totalMemory__gt", skip_serializing_if = "Option::is_none")]
    pub total_memory__gt: Option<i64>,
    /// Memory size (MB, more than or equal). Optional.
    #[serde(rename = "totalMemory__gte", skip_serializing_if = "Option::is_none")]
    pub total_memory__gte: Option<i64>,
    /// Migration status. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "migrationStatus", skip_serializing_if = "Option::is_none")]
    pub migration_status: Option<String>,
    /// Gateway ip. Example: "192.168.0.1". Optional.
    #[serde(rename = "gatewayIp", skip_serializing_if = "Option::is_none")]
    pub gateway_ip: Option<String>,
    /// The ID of the CSV file to filter by. Example: "225494730938493804". Optional.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<String>,
    /// Supported Remote Script Orchestration level. Example: "none". Optional.
    ///
    /// Allowed values: none, pro, ars.
    #[serde(rename = "rsoLevel", skip_serializing_if = "Option::is_none")]
    pub rso_level: Option<String>,
    /// Include only agents that has Remote Ops Forensicsfeature supported. Optional.
    #[serde(rename = "remoteOpsForensicsSupported", skip_serializing_if = "Option::is_none")]
    pub remote_ops_forensics_supported: Option<bool>,
    /// Agents os revision than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__gte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__gte: Option<i64>,
    /// Agents os revision lower than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__lte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__lte: Option<i64>,
    /// A list of included rso_levels. Example: "pro,ars". Optional.
    #[serde(rename = "rsoLevels", skip_serializing_if = "Option::is_none")]
    pub rso_levels: Option<String>,
}

impl AgentsCountQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }

    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_group_ids = Some(join_csv(vals));
        self
    }

    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_site_ids = Some(join_csv(vals));
        self
    }

    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn registered_at__between(mut self, v: impl Into<String>) -> Self {
        self.registered_at__between = Some(v.into());
        self
    }

    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_active_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__between = Some(v.into());
        self
    }

    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_successful_scan_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__between = Some(v.into());
        self
    }

    /// Include only active Agents.
    pub fn is_active<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_active = Some(join_csv(vals));
        self
    }

    /// Include only Agents with pending uninstall requests.
    pub fn is_pending_uninstall<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_pending_uninstall = Some(join_csv(vals));
        self
    }

    /// Include only Agents with at least one active threat.
    pub fn infected(mut self, b: bool) -> Self {
        self.infected = Some(b);
        self
    }

    /// Include only Agents with updated software.
    pub fn is_up_to_date<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_up_to_date = Some(join_csv(vals));
        self
    }

    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux".
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }

    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions = Some(join_csv(vals));
        self
    }

    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions_nin = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions_nin = Some(join_csv(vals));
        self
    }

    /// OS architecture. Example: "32 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arch(mut self, v: impl Into<String>) -> Self {
        self.os_arch = Some(v.into());
        self
    }

    /// OS architectures to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches = Some(join_csv(vals));
        self
    }

    /// OS architectures not to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches_nin = Some(join_csv(vals));
        self
    }

    /// Included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(vals));
        self
    }

    /// Not included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types_nin = Some(join_csv(vals));
        self
    }

    /// Included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses = Some(join_csv(vals));
        self
    }

    /// Not included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types = Some(join_csv(vals));
        self
    }

    /// Not included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types_nin = Some(join_csv(vals));
        self
    }

    /// Included storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types = Some(join_csv(vals));
        self
    }

    /// Excluded storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types_nin = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains = Some(join_csv(vals));
        self
    }

    /// Not included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains_nin = Some(join_csv(vals));
        self
    }

    /// Disk encryption status.
    pub fn encrypted_applications<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encrypted_applications = Some(join_csv(vals));
        self
    }

    /// Total memory range (GB, inclusive). Example: "4-8".
    pub fn total_memory__between(mut self, v: impl Into<String>) -> Self {
        self.total_memory__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn core_count__between(mut self, v: impl Into<String>) -> Self {
        self.core_count__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn cpu_count__between(mut self, v: impl Into<String>) -> Self {
        self.cpu_count__between = Some(v.into());
        self
    }

    /// Included pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed = Some(join_csv(vals));
        self
    }

    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions = Some(join_csv(vals));
        self
    }

    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed_nin = Some(join_csv(vals));
        self
    }

    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions_nin = Some(join_csv(vals));
        self
    }

    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com".
    pub fn ad_query(mut self, v: impl Into<String>) -> Self {
        self.ad_query = Some(v.into());
        self
    }

    /// Agent has a local configuration set.
    pub fn has_local_configuration(mut self, b: bool) -> Self {
        self.has_local_configuration = Some(b);
        self
    }

    /// Migration status in. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses = Some(join_csv(vals));
        self
    }

    /// Migration status nin. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status in. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status nin. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids = Some(join_csv(vals));
        self
    }

    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types = Some(join_csv(vals));
        self
    }

    /// Exclude Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types_nin = Some(join_csv(vals));
        self
    }

    /// Agent operational state.
    pub fn operational_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent operational states.
    pub fn operational_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states_nin = Some(join_csv(vals));
        self
    }

    /// Agent remote profiling state.
    pub fn remote_profiling_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent remote profiling states.
    pub fn remote_profiling_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states_nin = Some(join_csv(vals));
        self
    }

    /// Status of Network Discovery. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses = Some(join_csv(vals));
        self
    }

    /// Do not include these Network Scanner Statuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses_nin = Some(join_csv(vals));
        self
    }

    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_status(mut self, v: impl Into<String>) -> Self {
        self.ranger_status = Some(v.into());
        self
    }

    /// Has at least one threat with at least one mitigation action pending reboot to succeed.
    pub fn threat_reboot_required<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_reboot_required = Some(join_csv(vals));
        self
    }

    /// The agents supports Network Quarantine Control and its enabled for the agent's group.
    pub fn network_quarantine_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_quarantine_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Firewall Control and it is enabled for the agent's group.
    pub fn firewall_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.firewall_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Location Awareness and it is enabled for the agent's group.
    pub fn location_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_enabled = Some(join_csv(vals));
        self
    }

    /// Agents from which cloud provider.
    pub fn cloud_provider<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider = Some(join_csv(vals));
        self
    }

    /// Exclude Agents from these cloud provider.
    pub fn cloud_provider_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_nin = Some(join_csv(vals));
        self
    }

    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}".
    pub fn tags_data(mut self, v: impl Into<String>) -> Self {
        self.tags_data = Some(v.into());
        self
    }

    /// Include only Agents that have any tags assigned if True, or none if False.
    pub fn has_tags(mut self, b: bool) -> Self {
        self.has_tags = Some(b);
        self
    }

    /// The agents that are ADConnectors if True, or not if False.
    pub fn is_ad_connector<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(join_csv(vals));
        self
    }

    /// The agents has Hyper Automate PNA enabled if True, or not if False.
    pub fn is_hyper_automate<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_hyper_automate = Some(join_csv(vals));
        self
    }

    /// Included agent active protections. Example: "edr,idr".
    ///
    /// Allowed values: edr, idr.
    pub fn active_protection<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_protection = Some(join_csv(vals));
        self
    }

    /// Containerized workload counts.
    pub fn containerized_workload_counts(mut self, v: impl Into<String>) -> Self {
        self.containerized_workload_counts = Some(v.into());
        self
    }

    /// Indicates whether the agent protects containerized workload at the moment.
    pub fn has_containerized_workload<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.has_containerized_workload = Some(join_csv(vals));
        self
    }

    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported).
    pub fn pac_file_usage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.pac_file_usage = Some(join_csv(vals));
        self
    }

    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method = Some(join_csv(vals));
        self
    }

    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported).
    pub fn is_mgmt_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_mgmt_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported).
    pub fn is_event_search_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_event_search_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0".
    pub fn external_ip__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN".
    pub fn computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0".
    pub fn network_interface_inet__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_inet__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_physical__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_physical__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_gateway_mac_address__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_gateway_mac_address__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1".
    pub fn last_logged_in_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.last_logged_in_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1".
    pub fn os_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John".
    pub fn ad_user_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows".
    pub fn ad_computer_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055".
    pub fn uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuid__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine".
    pub fn external_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws role(supports multiple values).
    pub fn aws_role__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_role__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws securityGroups(supports multiple values).
    pub fn aws_security_groups__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_security_groups__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws subnet ids (supports multiple values).
    pub fn aws_subnet_ids__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_subnet_ids__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent namespace (supports multiple values).
    pub fn agent_namespace__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_namespace__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent pod name (supports multiple values).
    pub fn agent_pod_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pod_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by azure resource group(supports multiple values).
    pub fn azure_resource_group__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.azure_resource_group__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud account (supports multiple values).
    pub fn cloud_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud image (supports multiple values).
    pub fn cloud_image__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_image__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance id(supports multiple values).
    pub fn cloud_instance_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance size(supports multiple values).
    pub fn cloud_instance_size__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_size__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud location (supports multiple values).
    pub fn cloud_location__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_location__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud network (supports multiple values).
    pub fn cloud_network__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_network__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud tags (supports multiple values).
    pub fn cloud_tags__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cluster name (supports multiple values).
    pub fn cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by gcp service account (supports multiple values).
    pub fn gcp_service_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gcp_service_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node labels (supports multiple values).
    pub fn k8s_node_labels__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_labels__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node name (supports multiple values).
    pub fn k8s_node_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s type(supports multiple values).
    pub fn k8s_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s version (supports multiple values).
    pub fn k8s_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by live update ID (supports multiple values).
    pub fn live_update_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.live_update_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Serial Number (supports multiple values).
    pub fn serial_number__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Entra ID (supports multiple values).
    pub fn entra_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.entra_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD".
    pub fn cpu_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS type.
    pub fn ecs_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS version.
    pub fn ecs_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS cluster name.
    pub fn ecs_cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task arn.
    pub fn ecs_task_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task availability zone.
    pub fn ecs_task_availability_zone__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_availability_zone__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service name.
    pub fn ecs_service_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service arn.
    pub fn ecs_service_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition family.
    pub fn ecs_task_definition_family__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_family__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition revision.
    pub fn ecs_task_definition_revision__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_revision__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition arn.
    pub fn ecs_task_definition_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_arn__contains = Some(join_csv(vals));
        self
    }

    /// Include active, decommissioned or both. Example: "True,False".
    pub fn is_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_decommissioned = Some(join_csv(vals));
        self
    }

    /// Include installed, uninstalled or both. Example: "True,False".
    pub fn is_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_uninstalled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name or uuid (supports multiple values).
    pub fn computer_name_or_uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name_or_uuid__contains = Some(join_csv(vals));
        self
    }

    /// Included Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(vals));
        self
    }

    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids_nin = Some(join_csv(vals));
        self
    }

    /// Include all Agents matching this saved filter. Example: "225494730938493804".
    pub fn filter_id(mut self, v: impl Into<String>) -> Self {
        self.filter_id = Some(v.into());
        self
    }

    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gte = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lt = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lte = Some(v.into());
        self
    }

    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gt = Some(v.into());
        self
    }

    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn decommissioned_at__between(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__between = Some(v.into());
        self
    }

    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.created_at__lt = Some(v.into());
        self
    }

    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.created_at__lte = Some(v.into());
        self
    }

    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.created_at__gt = Some(v.into());
        self
    }

    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.created_at__gte = Some(v.into());
        self
    }

    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn created_at__between(mut self, v: impl Into<String>) -> Self {
        self.created_at__between = Some(v.into());
        self
    }

    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lt = Some(v.into());
        self
    }

    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lte = Some(v.into());
        self
    }

    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gt = Some(v.into());
        self
    }

    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gte = Some(v.into());
        self
    }

    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.updated_at__between = Some(v.into());
        self
    }

    /// Match computer name partially (substring). Example: "Lab1".
    pub fn computer_name__like(mut self, v: impl Into<String>) -> Self {
        self.computer_name__like = Some(v.into());
        self
    }

    /// Computer name. Example: "My Office Desktop".
    pub fn computer_name(mut self, v: impl Into<String>) -> Self {
        self.computer_name = Some(v.into());
        self
    }

    /// Agents versions less than given version. Example: "2.5.1.1320".
    pub fn agent_version__lt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lt = Some(v.into());
        self
    }

    /// Agents versions less than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__lte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lte = Some(v.into());
        self
    }

    /// Agents versions greater than given version. Example: "2.5.1.1320".
    pub fn agent_version__gt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gt = Some(v.into());
        self
    }

    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__gte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gte = Some(v.into());
        self
    }

    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144".
    pub fn agent_version__between(mut self, v: impl Into<String>) -> Self {
        self.agent_version__between = Some(v.into());
        self
    }

    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2".
    pub fn uuid(mut self, v: impl Into<String>) -> Self {
        self.uuid = Some(v.into());
        self
    }

    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22".
    pub fn uuids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuids = Some(join_csv(vals));
        self
    }

    /// Scan status. Example: "none".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_status(mut self, v: impl Into<String>) -> Self {
        self.scan_status = Some(v.into());
        self
    }

    /// Include only Agents that have threats with this mitigation status. Example: "mitigated".
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    pub fn threat_mitigation_status(mut self, v: impl Into<String>) -> Self {
        self.threat_mitigation_status = Some(v.into());
        self
    }

    /// Include only Agents with at least one resolved threat.
    pub fn threat_resolved(mut self, b: bool) -> Self {
        self.threat_resolved = Some(b);
        self
    }

    /// Include only Agents with at least one hidden threat.
    pub fn threat_hidden(mut self, b: bool) -> Self {
        self.threat_hidden = Some(b);
        self
    }

    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94".
    pub fn threat_content_hash(mut self, v: impl Into<String>) -> Self {
        self.threat_content_hash = Some(v.into());
        self
    }

    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lt = Some(v.into());
        self
    }

    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lte = Some(v.into());
        self
    }

    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gt = Some(v.into());
        self
    }

    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gte = Some(v.into());
        self
    }

    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn threat_created_at__between(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__between = Some(v.into());
        self
    }

    /// Include Agents with this amount of active threats. Example: "3".
    pub fn active_threats(mut self, n: i64) -> Self {
        self.active_threats = Some(n);
        self
    }

    /// Include Agents with at least this amount of active threats. Example: "5".
    pub fn active_threats__gt(mut self, n: i64) -> Self {
        self.active_threats__gt = Some(n);
        self
    }

    /// Agent mitigation mode policy. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode = Some(v.into());
        self
    }

    /// Mitigation mode policy for suspicious activity. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode_suspicious(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode_suspicious = Some(v.into());
        self
    }

    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lt = Some(v.into());
        self
    }

    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lte = Some(v.into());
        self
    }

    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gt = Some(v.into());
        self
    }

    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gte = Some(v.into());
        self
    }

    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lt = Some(v.into());
        self
    }

    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lte = Some(v.into());
        self
    }

    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gt = Some(v.into());
        self
    }

    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gte = Some(v.into());
        self
    }

    /// CPU cores (less than).
    pub fn core_count__lt(mut self, n: i64) -> Self {
        self.core_count__lt = Some(n);
        self
    }

    /// CPU cores (less than or equal).
    pub fn core_count__lte(mut self, n: i64) -> Self {
        self.core_count__lte = Some(n);
        self
    }

    /// CPU cores (more than).
    pub fn core_count__gt(mut self, n: i64) -> Self {
        self.core_count__gt = Some(n);
        self
    }

    /// CPU cores (more than or equal).
    pub fn core_count__gte(mut self, n: i64) -> Self {
        self.core_count__gte = Some(n);
        self
    }

    /// Number of CPUs (less than).
    pub fn cpu_count__lt(mut self, n: i64) -> Self {
        self.cpu_count__lt = Some(n);
        self
    }

    /// Number of CPUs (less than or equal).
    pub fn cpu_count__lte(mut self, n: i64) -> Self {
        self.cpu_count__lte = Some(n);
        self
    }

    /// Number of CPUs (more than).
    pub fn cpu_count__gt(mut self, n: i64) -> Self {
        self.cpu_count__gt = Some(n);
        self
    }

    /// Number of CPUs (more than or equal).
    pub fn cpu_count__gte(mut self, n: i64) -> Self {
        self.cpu_count__gte = Some(n);
        self
    }

    /// Memory size (MB, less than).
    pub fn total_memory__lt(mut self, n: i64) -> Self {
        self.total_memory__lt = Some(n);
        self
    }

    /// Memory size (MB, less than or equal).
    pub fn total_memory__lte(mut self, n: i64) -> Self {
        self.total_memory__lte = Some(n);
        self
    }

    /// Memory size (MB, more than).
    pub fn total_memory__gt(mut self, n: i64) -> Self {
        self.total_memory__gt = Some(n);
        self
    }

    /// Memory size (MB, more than or equal).
    pub fn total_memory__gte(mut self, n: i64) -> Self {
        self.total_memory__gte = Some(n);
        self
    }

    /// Migration status. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn migration_status(mut self, v: impl Into<String>) -> Self {
        self.migration_status = Some(v.into());
        self
    }

    /// Gateway ip. Example: "192.168.0.1".
    pub fn gateway_ip(mut self, v: impl Into<String>) -> Self {
        self.gateway_ip = Some(v.into());
        self
    }

    /// The ID of the CSV file to filter by. Example: "225494730938493804".
    pub fn csv_filter_id(mut self, v: impl Into<String>) -> Self {
        self.csv_filter_id = Some(v.into());
        self
    }

    /// Supported Remote Script Orchestration level. Example: "none".
    ///
    /// Allowed values: none, pro, ars.
    pub fn rso_level(mut self, v: impl Into<String>) -> Self {
        self.rso_level = Some(v.into());
        self
    }

    /// Include only agents that has Remote Ops Forensicsfeature supported.
    pub fn remote_ops_forensics_supported(mut self, b: bool) -> Self {
        self.remote_ops_forensics_supported = Some(b);
        self
    }

    /// Agents os revision than or equal to given version.
    pub fn windows_os_revision__gte(mut self, n: i64) -> Self {
        self.windows_os_revision__gte = Some(n);
        self
    }

    /// Agents os revision lower than or equal to given version.
    pub fn windows_os_revision__lte(mut self, n: i64) -> Self {
        self.windows_os_revision__lte = Some(n);
        self
    }

    /// A list of included rso_levels. Example: "pro,ars".
    pub fn rso_levels<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.rso_levels = Some(join_csv(vals));
        self
    }
}

/// Query params for `GET /web/api/v2.1/agents/passphrases` (Get Passphrase).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
pub struct AgentsPassphrasesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000 items, use "cursor". Example: "150". Optional.
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects. Optional.
    #[serde(rename = "countOnly", skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up execution time. Optional.
    #[serde(rename = "skipCount", skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredGroupIds", skip_serializing_if = "Option::is_none")]
    pub filtered_group_ids: Option<String>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredSiteIds", skip_serializing_if = "Option::is_none")]
    pub filtered_site_ids: Option<String>,
    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "registeredAt__between", skip_serializing_if = "Option::is_none")]
    pub registered_at__between: Option<String>,
    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastActiveDate__between", skip_serializing_if = "Option::is_none")]
    pub last_active_date__between: Option<String>,
    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastSuccessfulScanDate__between", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__between: Option<String>,
    /// Include only active Agents. Optional.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<String>,
    /// Include only Agents with pending uninstall requests. Optional.
    #[serde(rename = "isPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub is_pending_uninstall: Option<String>,
    /// Include only Agents with at least one active threat. Optional.
    #[serde(rename = "infected", skip_serializing_if = "Option::is_none")]
    pub infected: Option<bool>,
    /// Include only Agents with updated software. Optional.
    #[serde(rename = "isUpToDate", skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<String>,
    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux". Optional.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersions", skip_serializing_if = "Option::is_none")]
    pub agent_versions: Option<String>,
    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersionsNin", skip_serializing_if = "Option::is_none")]
    pub agent_versions_nin: Option<String>,
    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersions", skip_serializing_if = "Option::is_none")]
    pub ranger_versions: Option<String>,
    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersionsNin", skip_serializing_if = "Option::is_none")]
    pub ranger_versions_nin: Option<String>,
    /// OS architecture. Example: "32 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArch", skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<String>,
    /// OS architectures to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArches", skip_serializing_if = "Option::is_none")]
    pub os_arches: Option<String>,
    /// OS architectures not to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArchesNin", skip_serializing_if = "Option::is_none")]
    pub os_arches_nin: Option<String>,
    /// Included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypes", skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Not included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypesNin", skip_serializing_if = "Option::is_none")]
    pub os_types_nin: Option<String>,
    /// Included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatuses", skip_serializing_if = "Option::is_none")]
    pub scan_statuses: Option<String>,
    /// Not included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatusesNin", skip_serializing_if = "Option::is_none")]
    pub scan_statuses_nin: Option<String>,
    /// Included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypes", skip_serializing_if = "Option::is_none")]
    pub machine_types: Option<String>,
    /// Not included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypesNin", skip_serializing_if = "Option::is_none")]
    pub machine_types_nin: Option<String>,
    /// Included storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypes", skip_serializing_if = "Option::is_none")]
    pub storage_types: Option<String>,
    /// Excluded storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypesNin", skip_serializing_if = "Option::is_none")]
    pub storage_types_nin: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatuses", skip_serializing_if = "Option::is_none")]
    pub network_statuses: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatusesNin", skip_serializing_if = "Option::is_none")]
    pub network_statuses_nin: Option<String>,
    /// Included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// Not included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domainsNin", skip_serializing_if = "Option::is_none")]
    pub domains_nin: Option<String>,
    /// Disk encryption status. Optional.
    #[serde(rename = "encryptedApplications", skip_serializing_if = "Option::is_none")]
    pub encrypted_applications: Option<String>,
    /// Total memory range (GB, inclusive). Example: "4-8". Optional.
    #[serde(rename = "totalMemory__between", skip_serializing_if = "Option::is_none")]
    pub total_memory__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "coreCount__between", skip_serializing_if = "Option::is_none")]
    pub core_count__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "cpuCount__between", skip_serializing_if = "Option::is_none")]
    pub cpu_count__between: Option<String>,
    /// Included pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeeded", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed: Option<String>,
    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissions", skip_serializing_if = "Option::is_none")]
    pub missing_permissions: Option<String>,
    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeededNin", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed_nin: Option<String>,
    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissionsNin", skip_serializing_if = "Option::is_none")]
    pub missing_permissions_nin: Option<String>,
    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com". Optional.
    #[serde(rename = "adQuery", skip_serializing_if = "Option::is_none")]
    pub ad_query: Option<String>,
    /// Agent has a local configuration set. Optional.
    #[serde(rename = "hasLocalConfiguration", skip_serializing_if = "Option::is_none")]
    pub has_local_configuration: Option<bool>,
    /// Migration status in. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatuses", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses: Option<String>,
    /// Migration status nin. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatusesNin", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses_nin: Option<String>,
    /// Apps vulnerability status in. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatuses", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses: Option<String>,
    /// Apps vulnerability status nin. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatusesNin", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses_nin: Option<String>,
    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIds", skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<String>,
    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIdsNin", skip_serializing_if = "Option::is_none")]
    pub location_ids_nin: Option<String>,
    /// Include only Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypes", skip_serializing_if = "Option::is_none")]
    pub installer_types: Option<String>,
    /// Exclude Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypesNin", skip_serializing_if = "Option::is_none")]
    pub installer_types_nin: Option<String>,
    /// Agent operational state. Optional.
    #[serde(rename = "operationalStates", skip_serializing_if = "Option::is_none")]
    pub operational_states: Option<String>,
    /// Do not include these Agent operational states. Optional.
    #[serde(rename = "operationalStatesNin", skip_serializing_if = "Option::is_none")]
    pub operational_states_nin: Option<String>,
    /// Agent remote profiling state. Optional.
    #[serde(rename = "remoteProfilingStates", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states: Option<String>,
    /// Do not include these Agent remote profiling states. Optional.
    #[serde(rename = "remoteProfilingStatesNin", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states_nin: Option<String>,
    /// Status of Network Discovery. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatuses", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses: Option<String>,
    /// Do not include these Network Scanner Statuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatusesNin", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses_nin: Option<String>,
    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatus", skip_serializing_if = "Option::is_none")]
    pub ranger_status: Option<String>,
    /// Has at least one threat with at least one mitigation action pending reboot to succeed. Optional.
    #[serde(rename = "threatRebootRequired", skip_serializing_if = "Option::is_none")]
    pub threat_reboot_required: Option<String>,
    /// The agents supports Network Quarantine Control and its enabled for the agent's group. Optional.
    #[serde(rename = "networkQuarantineEnabled", skip_serializing_if = "Option::is_none")]
    pub network_quarantine_enabled: Option<String>,
    /// The agents supports Firewall Control and it is enabled for the agent's group. Optional.
    #[serde(rename = "firewallEnabled", skip_serializing_if = "Option::is_none")]
    pub firewall_enabled: Option<String>,
    /// The agents supports Location Awareness and it is enabled for the agent's group. Optional.
    #[serde(rename = "locationEnabled", skip_serializing_if = "Option::is_none")]
    pub location_enabled: Option<String>,
    /// Agents from which cloud provider. Optional.
    #[serde(rename = "cloudProvider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// Exclude Agents from these cloud provider. Optional.
    #[serde(rename = "cloudProviderNin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_nin: Option<String>,
    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}". Optional.
    #[serde(rename = "tagsData", skip_serializing_if = "Option::is_none")]
    pub tags_data: Option<String>,
    /// Include only Agents that have any tags assigned if True, or none if False. Optional.
    #[serde(rename = "hasTags", skip_serializing_if = "Option::is_none")]
    pub has_tags: Option<bool>,
    /// The agents that are ADConnectors if True, or not if False. Optional.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The agents has Hyper Automate PNA enabled if True, or not if False. Optional.
    #[serde(rename = "isHyperAutomate", skip_serializing_if = "Option::is_none")]
    pub is_hyper_automate: Option<String>,
    /// Included agent active protections. Example: "edr,idr". Optional.
    ///
    /// Allowed values: edr, idr.
    #[serde(rename = "activeProtection", skip_serializing_if = "Option::is_none")]
    pub active_protection: Option<String>,
    /// Containerized workload counts. Optional.
    #[serde(rename = "containerizedWorkloadCounts", skip_serializing_if = "Option::is_none")]
    pub containerized_workload_counts: Option<String>,
    /// Indicates whether the agent protects containerized workload at the moment. Optional.
    #[serde(rename = "hasContainerizedWorkload", skip_serializing_if = "Option::is_none")]
    pub has_containerized_workload: Option<String>,
    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "pacFileUsage", skip_serializing_if = "Option::is_none")]
    pub pac_file_usage: Option<String>,
    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethod", skip_serializing_if = "Option::is_none")]
    pub proxy_method: Option<String>,
    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethodNin", skip_serializing_if = "Option::is_none")]
    pub proxy_method_nin: Option<String>,
    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isMgmtProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_mgmt_proxy_enabled: Option<String>,
    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isEventSearchProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_event_search_proxy_enabled: Option<String>,
    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0". Optional.
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip__contains: Option<String>,
    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN". Optional.
    #[serde(rename = "computerName__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name__contains: Option<String>,
    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0". Optional.
    #[serde(rename = "networkInterfaceInet__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_inet__contains: Option<String>,
    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfacePhysical__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_physical__contains: Option<String>,
    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfaceGatewayMacAddress__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_gateway_mac_address__contains: Option<String>,
    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1". Optional.
    #[serde(rename = "lastLoggedInUserName__contains", skip_serializing_if = "Option::is_none")]
    pub last_logged_in_user_name__contains: Option<String>,
    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1". Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_query__contains: Option<String>,
    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_name__contains: Option<String>,
    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John". Optional.
    #[serde(rename = "adUserQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_query__contains: Option<String>,
    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_name__contains: Option<String>,
    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows". Optional.
    #[serde(rename = "adComputerQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_query__contains: Option<String>,
    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055". Optional.
    #[serde(rename = "uuid__contains", skip_serializing_if = "Option::is_none")]
    pub uuid__contains: Option<String>,
    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine". Optional.
    #[serde(rename = "externalId__contains", skip_serializing_if = "Option::is_none")]
    pub external_id__contains: Option<String>,
    /// Free-text filter by aws role(supports multiple values). Optional.
    #[serde(rename = "awsRole__contains", skip_serializing_if = "Option::is_none")]
    pub aws_role__contains: Option<String>,
    /// Free-text filter by aws securityGroups(supports multiple values). Optional.
    #[serde(rename = "awsSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub aws_security_groups__contains: Option<String>,
    /// Free-text filter by aws subnet ids (supports multiple values). Optional.
    #[serde(rename = "awsSubnetIds__contains", skip_serializing_if = "Option::is_none")]
    pub aws_subnet_ids__contains: Option<String>,
    /// Free-text filter by agent namespace (supports multiple values). Optional.
    #[serde(rename = "agentNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub agent_namespace__contains: Option<String>,
    /// Free-text filter by agent pod name (supports multiple values). Optional.
    #[serde(rename = "agentPodName__contains", skip_serializing_if = "Option::is_none")]
    pub agent_pod_name__contains: Option<String>,
    /// Free-text filter by azure resource group(supports multiple values). Optional.
    #[serde(rename = "azureResourceGroup__contains", skip_serializing_if = "Option::is_none")]
    pub azure_resource_group__contains: Option<String>,
    /// Free-text filter by cloud account (supports multiple values). Optional.
    #[serde(rename = "cloudAccount__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_account__contains: Option<String>,
    /// Free-text filter by cloud image (supports multiple values). Optional.
    #[serde(rename = "cloudImage__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_image__contains: Option<String>,
    /// Free-text filter by cloud instance id(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_id__contains: Option<String>,
    /// Free-text filter by cloud instance size(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceSize__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_size__contains: Option<String>,
    /// Free-text filter by cloud location (supports multiple values). Optional.
    #[serde(rename = "cloudLocation__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_location__contains: Option<String>,
    /// Free-text filter by cloud network (supports multiple values). Optional.
    #[serde(rename = "cloudNetwork__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_network__contains: Option<String>,
    /// Free-text filter by cloud tags (supports multiple values). Optional.
    #[serde(rename = "cloudTags__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags__contains: Option<String>,
    /// Free-text filter by cluster name (supports multiple values). Optional.
    #[serde(rename = "clusterName__contains", skip_serializing_if = "Option::is_none")]
    pub cluster_name__contains: Option<String>,
    /// Free-text filter by gcp service account (supports multiple values). Optional.
    #[serde(rename = "gcpServiceAccount__contains", skip_serializing_if = "Option::is_none")]
    pub gcp_service_account__contains: Option<String>,
    /// Free-text filter by K8s node labels (supports multiple values). Optional.
    #[serde(rename = "k8sNodeLabels__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_labels__contains: Option<String>,
    /// Free-text filter by K8s node name (supports multiple values). Optional.
    #[serde(rename = "k8sNodeName__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_name__contains: Option<String>,
    /// Free-text filter by K8s type(supports multiple values). Optional.
    #[serde(rename = "k8sType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_type__contains: Option<String>,
    /// Free-text filter by K8s version (supports multiple values). Optional.
    #[serde(rename = "k8sVersion__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_version__contains: Option<String>,
    /// Free-text filter by live update ID (supports multiple values). Optional.
    #[serde(rename = "liveUpdateId__contains", skip_serializing_if = "Option::is_none")]
    pub live_update_id__contains: Option<String>,
    /// Free-text filter by Serial Number (supports multiple values). Optional.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number__contains: Option<String>,
    /// Free-text filter by Entra ID (supports multiple values). Optional.
    #[serde(rename = "entraId__contains", skip_serializing_if = "Option::is_none")]
    pub entra_id__contains: Option<String>,
    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD". Optional.
    #[serde(rename = "cpuId__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_id__contains: Option<String>,
    /// Free-text filter by ECS type. Optional.
    #[serde(rename = "ecsType__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_type__contains: Option<String>,
    /// Free-text filter by ECS version. Optional.
    #[serde(rename = "ecsVersion__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_version__contains: Option<String>,
    /// Free-text filter by ECS cluster name. Optional.
    #[serde(rename = "ecsClusterName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_name__contains: Option<String>,
    /// Free-text filter by ECS task arn. Optional.
    #[serde(rename = "ecsTaskArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_arn__contains: Option<String>,
    /// Free-text filter by ECS task availability zone. Optional.
    #[serde(rename = "ecsTaskAvailabilityZone__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_availability_zone__contains: Option<String>,
    /// Free-text filter by ECS service name. Optional.
    #[serde(rename = "ecsServiceName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_name__contains: Option<String>,
    /// Free-text filter by ECS service arn. Optional.
    #[serde(rename = "ecsServiceArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_arn__contains: Option<String>,
    /// Free-text filter by ECS task definition family. Optional.
    #[serde(rename = "ecsTaskDefinitionFamily__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_family__contains: Option<String>,
    /// Free-text filter by ECS task definition revision. Optional.
    #[serde(rename = "ecsTaskDefinitionRevision__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_revision__contains: Option<String>,
    /// Free-text filter by ECS task definition arn. Optional.
    #[serde(rename = "ecsTaskDefinitionArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_arn__contains: Option<String>,
    /// Include active, decommissioned or both. Example: "True,False". Optional.
    #[serde(rename = "isDecommissioned", skip_serializing_if = "Option::is_none")]
    pub is_decommissioned: Option<String>,
    /// Include installed, uninstalled or both. Example: "True,False". Optional.
    #[serde(rename = "isUninstalled", skip_serializing_if = "Option::is_none")]
    pub is_uninstalled: Option<String>,
    /// Free-text filter by computer name or uuid (supports multiple values). Optional.
    #[serde(rename = "computerNameOrUuid__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name_or_uuid__contains: Option<String>,
    /// Included Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "idsNin", skip_serializing_if = "Option::is_none")]
    pub ids_nin: Option<String>,
    /// Include all Agents matching this saved filter. Example: "225494730938493804". Optional.
    #[serde(rename = "filterId", skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<String>,
    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gte: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lt: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lte: Option<String>,
    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gt: Option<String>,
    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "decommissionedAt__between", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__between: Option<String>,
    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at__lt: Option<String>,
    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at__lte: Option<String>,
    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at__gt: Option<String>,
    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at__gte: Option<String>,
    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at__between: Option<String>,
    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at__lt: Option<String>,
    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at__lte: Option<String>,
    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at__gt: Option<String>,
    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at__gte: Option<String>,
    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at__between: Option<String>,
    /// Match computer name partially (substring). Example: "Lab1". Optional.
    #[serde(rename = "computerName__like", skip_serializing_if = "Option::is_none")]
    pub computer_name__like: Option<String>,
    /// Computer name. Example: "My Office Desktop". Optional.
    #[serde(rename = "computerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// Agents versions less than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lt", skip_serializing_if = "Option::is_none")]
    pub agent_version__lt: Option<String>,
    /// Agents versions less than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lte", skip_serializing_if = "Option::is_none")]
    pub agent_version__lte: Option<String>,
    /// Agents versions greater than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gt", skip_serializing_if = "Option::is_none")]
    pub agent_version__gt: Option<String>,
    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gte", skip_serializing_if = "Option::is_none")]
    pub agent_version__gte: Option<String>,
    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144". Optional.
    #[serde(rename = "agentVersion__between", skip_serializing_if = "Option::is_none")]
    pub agent_version__between: Option<String>,
    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2". Optional.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22". Optional.
    #[serde(rename = "uuids", skip_serializing_if = "Option::is_none")]
    pub uuids: Option<String>,
    /// Scan status. Example: "none". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatus", skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// Include only Agents that have threats with this mitigation status. Example: "mitigated". Optional.
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    #[serde(rename = "threatMitigationStatus", skip_serializing_if = "Option::is_none")]
    pub threat_mitigation_status: Option<String>,
    /// Include only Agents with at least one resolved threat. Optional.
    #[serde(rename = "threatResolved", skip_serializing_if = "Option::is_none")]
    pub threat_resolved: Option<bool>,
    /// Include only Agents with at least one hidden threat. Optional.
    #[serde(rename = "threatHidden", skip_serializing_if = "Option::is_none")]
    pub threat_hidden: Option<bool>,
    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94". Optional.
    #[serde(rename = "threatContentHash", skip_serializing_if = "Option::is_none")]
    pub threat_content_hash: Option<String>,
    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lt: Option<String>,
    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lte: Option<String>,
    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gt: Option<String>,
    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gte: Option<String>,
    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "threatCreatedAt__between", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__between: Option<String>,
    /// Include Agents with this amount of active threats. Example: "3". Optional.
    #[serde(rename = "activeThreats", skip_serializing_if = "Option::is_none")]
    pub active_threats: Option<i64>,
    /// Include Agents with at least this amount of active threats. Example: "5". Optional.
    #[serde(rename = "activeThreats__gt", skip_serializing_if = "Option::is_none")]
    pub active_threats__gt: Option<i64>,
    /// Agent mitigation mode policy. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationMode", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode: Option<String>,
    /// Mitigation mode policy for suspicious activity. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationModeSuspicious", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode_suspicious: Option<String>,
    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lt", skip_serializing_if = "Option::is_none")]
    pub registered_at__lt: Option<String>,
    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lte", skip_serializing_if = "Option::is_none")]
    pub registered_at__lte: Option<String>,
    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gt", skip_serializing_if = "Option::is_none")]
    pub registered_at__gt: Option<String>,
    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gte", skip_serializing_if = "Option::is_none")]
    pub registered_at__gte: Option<String>,
    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lt: Option<String>,
    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lte: Option<String>,
    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gt: Option<String>,
    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gte: Option<String>,
    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lt: Option<String>,
    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lte: Option<String>,
    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gt: Option<String>,
    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gte: Option<String>,
    /// CPU cores (less than). Optional.
    #[serde(rename = "coreCount__lt", skip_serializing_if = "Option::is_none")]
    pub core_count__lt: Option<i64>,
    /// CPU cores (less than or equal). Optional.
    #[serde(rename = "coreCount__lte", skip_serializing_if = "Option::is_none")]
    pub core_count__lte: Option<i64>,
    /// CPU cores (more than). Optional.
    #[serde(rename = "coreCount__gt", skip_serializing_if = "Option::is_none")]
    pub core_count__gt: Option<i64>,
    /// CPU cores (more than or equal). Optional.
    #[serde(rename = "coreCount__gte", skip_serializing_if = "Option::is_none")]
    pub core_count__gte: Option<i64>,
    /// Number of CPUs (less than). Optional.
    #[serde(rename = "cpuCount__lt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lt: Option<i64>,
    /// Number of CPUs (less than or equal). Optional.
    #[serde(rename = "cpuCount__lte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lte: Option<i64>,
    /// Number of CPUs (more than). Optional.
    #[serde(rename = "cpuCount__gt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gt: Option<i64>,
    /// Number of CPUs (more than or equal). Optional.
    #[serde(rename = "cpuCount__gte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gte: Option<i64>,
    /// Memory size (MB, less than). Optional.
    #[serde(rename = "totalMemory__lt", skip_serializing_if = "Option::is_none")]
    pub total_memory__lt: Option<i64>,
    /// Memory size (MB, less than or equal). Optional.
    #[serde(rename = "totalMemory__lte", skip_serializing_if = "Option::is_none")]
    pub total_memory__lte: Option<i64>,
    /// Memory size (MB, more than). Optional.
    #[serde(rename = "totalMemory__gt", skip_serializing_if = "Option::is_none")]
    pub total_memory__gt: Option<i64>,
    /// Memory size (MB, more than or equal). Optional.
    #[serde(rename = "totalMemory__gte", skip_serializing_if = "Option::is_none")]
    pub total_memory__gte: Option<i64>,
    /// Migration status. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "migrationStatus", skip_serializing_if = "Option::is_none")]
    pub migration_status: Option<String>,
    /// Gateway ip. Example: "192.168.0.1". Optional.
    #[serde(rename = "gatewayIp", skip_serializing_if = "Option::is_none")]
    pub gateway_ip: Option<String>,
    /// The ID of the CSV file to filter by. Example: "225494730938493804". Optional.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<String>,
    /// Supported Remote Script Orchestration level. Example: "none". Optional.
    ///
    /// Allowed values: none, pro, ars.
    #[serde(rename = "rsoLevel", skip_serializing_if = "Option::is_none")]
    pub rso_level: Option<String>,
    /// Include only agents that has Remote Ops Forensicsfeature supported. Optional.
    #[serde(rename = "remoteOpsForensicsSupported", skip_serializing_if = "Option::is_none")]
    pub remote_ops_forensics_supported: Option<bool>,
    /// Agents os revision than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__gte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__gte: Option<i64>,
    /// Agents os revision lower than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__lte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__lte: Option<i64>,
    /// A list of included rso_levels. Example: "pro,ars". Optional.
    #[serde(rename = "rsoLevels", skip_serializing_if = "Option::is_none")]
    pub rso_levels: Option<String>,
}

impl AgentsPassphrasesQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000 items, use "cursor". Example: "150".
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }

    /// Limit number of returned items (1-1000). Example: "10".
    pub fn limit(mut self, n: u32) -> Self {
        self.limit = Some(n);
        self
    }

    /// Cursor position returned by the last request. Use to iterate over more than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=".
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }

    /// If true, only total number of items will be returned, without any of the actual objects.
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }

    /// If true, total number of items will not be calculated, which speeds up execution time.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }

    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }

    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_group_ids = Some(join_csv(vals));
        self
    }

    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_site_ids = Some(join_csv(vals));
        self
    }

    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn registered_at__between(mut self, v: impl Into<String>) -> Self {
        self.registered_at__between = Some(v.into());
        self
    }

    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_active_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__between = Some(v.into());
        self
    }

    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_successful_scan_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__between = Some(v.into());
        self
    }

    /// Include only active Agents.
    pub fn is_active<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_active = Some(join_csv(vals));
        self
    }

    /// Include only Agents with pending uninstall requests.
    pub fn is_pending_uninstall<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_pending_uninstall = Some(join_csv(vals));
        self
    }

    /// Include only Agents with at least one active threat.
    pub fn infected(mut self, b: bool) -> Self {
        self.infected = Some(b);
        self
    }

    /// Include only Agents with updated software.
    pub fn is_up_to_date<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_up_to_date = Some(join_csv(vals));
        self
    }

    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux".
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }

    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions = Some(join_csv(vals));
        self
    }

    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions_nin = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions_nin = Some(join_csv(vals));
        self
    }

    /// OS architecture. Example: "32 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arch(mut self, v: impl Into<String>) -> Self {
        self.os_arch = Some(v.into());
        self
    }

    /// OS architectures to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches = Some(join_csv(vals));
        self
    }

    /// OS architectures not to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches_nin = Some(join_csv(vals));
        self
    }

    /// Included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(vals));
        self
    }

    /// Not included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types_nin = Some(join_csv(vals));
        self
    }

    /// Included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses = Some(join_csv(vals));
        self
    }

    /// Not included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types = Some(join_csv(vals));
        self
    }

    /// Not included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types_nin = Some(join_csv(vals));
        self
    }

    /// Included storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types = Some(join_csv(vals));
        self
    }

    /// Excluded storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types_nin = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains = Some(join_csv(vals));
        self
    }

    /// Not included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains_nin = Some(join_csv(vals));
        self
    }

    /// Disk encryption status.
    pub fn encrypted_applications<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encrypted_applications = Some(join_csv(vals));
        self
    }

    /// Total memory range (GB, inclusive). Example: "4-8".
    pub fn total_memory__between(mut self, v: impl Into<String>) -> Self {
        self.total_memory__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn core_count__between(mut self, v: impl Into<String>) -> Self {
        self.core_count__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn cpu_count__between(mut self, v: impl Into<String>) -> Self {
        self.cpu_count__between = Some(v.into());
        self
    }

    /// Included pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed = Some(join_csv(vals));
        self
    }

    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions = Some(join_csv(vals));
        self
    }

    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed_nin = Some(join_csv(vals));
        self
    }

    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions_nin = Some(join_csv(vals));
        self
    }

    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com".
    pub fn ad_query(mut self, v: impl Into<String>) -> Self {
        self.ad_query = Some(v.into());
        self
    }

    /// Agent has a local configuration set.
    pub fn has_local_configuration(mut self, b: bool) -> Self {
        self.has_local_configuration = Some(b);
        self
    }

    /// Migration status in. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses = Some(join_csv(vals));
        self
    }

    /// Migration status nin. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status in. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status nin. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids = Some(join_csv(vals));
        self
    }

    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types = Some(join_csv(vals));
        self
    }

    /// Exclude Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types_nin = Some(join_csv(vals));
        self
    }

    /// Agent operational state.
    pub fn operational_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent operational states.
    pub fn operational_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states_nin = Some(join_csv(vals));
        self
    }

    /// Agent remote profiling state.
    pub fn remote_profiling_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent remote profiling states.
    pub fn remote_profiling_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states_nin = Some(join_csv(vals));
        self
    }

    /// Status of Network Discovery. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses = Some(join_csv(vals));
        self
    }

    /// Do not include these Network Scanner Statuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses_nin = Some(join_csv(vals));
        self
    }

    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_status(mut self, v: impl Into<String>) -> Self {
        self.ranger_status = Some(v.into());
        self
    }

    /// Has at least one threat with at least one mitigation action pending reboot to succeed.
    pub fn threat_reboot_required<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_reboot_required = Some(join_csv(vals));
        self
    }

    /// The agents supports Network Quarantine Control and its enabled for the agent's group.
    pub fn network_quarantine_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_quarantine_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Firewall Control and it is enabled for the agent's group.
    pub fn firewall_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.firewall_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Location Awareness and it is enabled for the agent's group.
    pub fn location_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_enabled = Some(join_csv(vals));
        self
    }

    /// Agents from which cloud provider.
    pub fn cloud_provider<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider = Some(join_csv(vals));
        self
    }

    /// Exclude Agents from these cloud provider.
    pub fn cloud_provider_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_nin = Some(join_csv(vals));
        self
    }

    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}".
    pub fn tags_data(mut self, v: impl Into<String>) -> Self {
        self.tags_data = Some(v.into());
        self
    }

    /// Include only Agents that have any tags assigned if True, or none if False.
    pub fn has_tags(mut self, b: bool) -> Self {
        self.has_tags = Some(b);
        self
    }

    /// The agents that are ADConnectors if True, or not if False.
    pub fn is_ad_connector<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(join_csv(vals));
        self
    }

    /// The agents has Hyper Automate PNA enabled if True, or not if False.
    pub fn is_hyper_automate<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_hyper_automate = Some(join_csv(vals));
        self
    }

    /// Included agent active protections. Example: "edr,idr".
    ///
    /// Allowed values: edr, idr.
    pub fn active_protection<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_protection = Some(join_csv(vals));
        self
    }

    /// Containerized workload counts.
    pub fn containerized_workload_counts(mut self, v: impl Into<String>) -> Self {
        self.containerized_workload_counts = Some(v.into());
        self
    }

    /// Indicates whether the agent protects containerized workload at the moment.
    pub fn has_containerized_workload<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.has_containerized_workload = Some(join_csv(vals));
        self
    }

    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported).
    pub fn pac_file_usage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.pac_file_usage = Some(join_csv(vals));
        self
    }

    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method = Some(join_csv(vals));
        self
    }

    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported).
    pub fn is_mgmt_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_mgmt_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported).
    pub fn is_event_search_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_event_search_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0".
    pub fn external_ip__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN".
    pub fn computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0".
    pub fn network_interface_inet__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_inet__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_physical__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_physical__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_gateway_mac_address__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_gateway_mac_address__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1".
    pub fn last_logged_in_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.last_logged_in_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1".
    pub fn os_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John".
    pub fn ad_user_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows".
    pub fn ad_computer_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055".
    pub fn uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuid__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine".
    pub fn external_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws role(supports multiple values).
    pub fn aws_role__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_role__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws securityGroups(supports multiple values).
    pub fn aws_security_groups__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_security_groups__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws subnet ids (supports multiple values).
    pub fn aws_subnet_ids__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_subnet_ids__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent namespace (supports multiple values).
    pub fn agent_namespace__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_namespace__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent pod name (supports multiple values).
    pub fn agent_pod_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pod_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by azure resource group(supports multiple values).
    pub fn azure_resource_group__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.azure_resource_group__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud account (supports multiple values).
    pub fn cloud_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud image (supports multiple values).
    pub fn cloud_image__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_image__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance id(supports multiple values).
    pub fn cloud_instance_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance size(supports multiple values).
    pub fn cloud_instance_size__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_size__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud location (supports multiple values).
    pub fn cloud_location__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_location__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud network (supports multiple values).
    pub fn cloud_network__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_network__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud tags (supports multiple values).
    pub fn cloud_tags__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cluster name (supports multiple values).
    pub fn cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by gcp service account (supports multiple values).
    pub fn gcp_service_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gcp_service_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node labels (supports multiple values).
    pub fn k8s_node_labels__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_labels__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node name (supports multiple values).
    pub fn k8s_node_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s type(supports multiple values).
    pub fn k8s_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s version (supports multiple values).
    pub fn k8s_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by live update ID (supports multiple values).
    pub fn live_update_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.live_update_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Serial Number (supports multiple values).
    pub fn serial_number__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Entra ID (supports multiple values).
    pub fn entra_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.entra_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD".
    pub fn cpu_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS type.
    pub fn ecs_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS version.
    pub fn ecs_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS cluster name.
    pub fn ecs_cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task arn.
    pub fn ecs_task_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task availability zone.
    pub fn ecs_task_availability_zone__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_availability_zone__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service name.
    pub fn ecs_service_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service arn.
    pub fn ecs_service_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition family.
    pub fn ecs_task_definition_family__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_family__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition revision.
    pub fn ecs_task_definition_revision__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_revision__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition arn.
    pub fn ecs_task_definition_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_arn__contains = Some(join_csv(vals));
        self
    }

    /// Include active, decommissioned or both. Example: "True,False".
    pub fn is_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_decommissioned = Some(join_csv(vals));
        self
    }

    /// Include installed, uninstalled or both. Example: "True,False".
    pub fn is_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_uninstalled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name or uuid (supports multiple values).
    pub fn computer_name_or_uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name_or_uuid__contains = Some(join_csv(vals));
        self
    }

    /// Included Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(vals));
        self
    }

    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids_nin = Some(join_csv(vals));
        self
    }

    /// Include all Agents matching this saved filter. Example: "225494730938493804".
    pub fn filter_id(mut self, v: impl Into<String>) -> Self {
        self.filter_id = Some(v.into());
        self
    }

    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gte = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lt = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lte = Some(v.into());
        self
    }

    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gt = Some(v.into());
        self
    }

    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn decommissioned_at__between(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__between = Some(v.into());
        self
    }

    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.created_at__lt = Some(v.into());
        self
    }

    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.created_at__lte = Some(v.into());
        self
    }

    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.created_at__gt = Some(v.into());
        self
    }

    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.created_at__gte = Some(v.into());
        self
    }

    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn created_at__between(mut self, v: impl Into<String>) -> Self {
        self.created_at__between = Some(v.into());
        self
    }

    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lt = Some(v.into());
        self
    }

    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lte = Some(v.into());
        self
    }

    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gt = Some(v.into());
        self
    }

    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gte = Some(v.into());
        self
    }

    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.updated_at__between = Some(v.into());
        self
    }

    /// Match computer name partially (substring). Example: "Lab1".
    pub fn computer_name__like(mut self, v: impl Into<String>) -> Self {
        self.computer_name__like = Some(v.into());
        self
    }

    /// Computer name. Example: "My Office Desktop".
    pub fn computer_name(mut self, v: impl Into<String>) -> Self {
        self.computer_name = Some(v.into());
        self
    }

    /// Agents versions less than given version. Example: "2.5.1.1320".
    pub fn agent_version__lt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lt = Some(v.into());
        self
    }

    /// Agents versions less than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__lte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lte = Some(v.into());
        self
    }

    /// Agents versions greater than given version. Example: "2.5.1.1320".
    pub fn agent_version__gt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gt = Some(v.into());
        self
    }

    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__gte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gte = Some(v.into());
        self
    }

    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144".
    pub fn agent_version__between(mut self, v: impl Into<String>) -> Self {
        self.agent_version__between = Some(v.into());
        self
    }

    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2".
    pub fn uuid(mut self, v: impl Into<String>) -> Self {
        self.uuid = Some(v.into());
        self
    }

    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22".
    pub fn uuids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuids = Some(join_csv(vals));
        self
    }

    /// Scan status. Example: "none".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_status(mut self, v: impl Into<String>) -> Self {
        self.scan_status = Some(v.into());
        self
    }

    /// Include only Agents that have threats with this mitigation status. Example: "mitigated".
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    pub fn threat_mitigation_status(mut self, v: impl Into<String>) -> Self {
        self.threat_mitigation_status = Some(v.into());
        self
    }

    /// Include only Agents with at least one resolved threat.
    pub fn threat_resolved(mut self, b: bool) -> Self {
        self.threat_resolved = Some(b);
        self
    }

    /// Include only Agents with at least one hidden threat.
    pub fn threat_hidden(mut self, b: bool) -> Self {
        self.threat_hidden = Some(b);
        self
    }

    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94".
    pub fn threat_content_hash(mut self, v: impl Into<String>) -> Self {
        self.threat_content_hash = Some(v.into());
        self
    }

    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lt = Some(v.into());
        self
    }

    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lte = Some(v.into());
        self
    }

    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gt = Some(v.into());
        self
    }

    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gte = Some(v.into());
        self
    }

    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn threat_created_at__between(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__between = Some(v.into());
        self
    }

    /// Include Agents with this amount of active threats. Example: "3".
    pub fn active_threats(mut self, n: i64) -> Self {
        self.active_threats = Some(n);
        self
    }

    /// Include Agents with at least this amount of active threats. Example: "5".
    pub fn active_threats__gt(mut self, n: i64) -> Self {
        self.active_threats__gt = Some(n);
        self
    }

    /// Agent mitigation mode policy. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode = Some(v.into());
        self
    }

    /// Mitigation mode policy for suspicious activity. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode_suspicious(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode_suspicious = Some(v.into());
        self
    }

    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lt = Some(v.into());
        self
    }

    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lte = Some(v.into());
        self
    }

    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gt = Some(v.into());
        self
    }

    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gte = Some(v.into());
        self
    }

    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lt = Some(v.into());
        self
    }

    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lte = Some(v.into());
        self
    }

    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gt = Some(v.into());
        self
    }

    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gte = Some(v.into());
        self
    }

    /// CPU cores (less than).
    pub fn core_count__lt(mut self, n: i64) -> Self {
        self.core_count__lt = Some(n);
        self
    }

    /// CPU cores (less than or equal).
    pub fn core_count__lte(mut self, n: i64) -> Self {
        self.core_count__lte = Some(n);
        self
    }

    /// CPU cores (more than).
    pub fn core_count__gt(mut self, n: i64) -> Self {
        self.core_count__gt = Some(n);
        self
    }

    /// CPU cores (more than or equal).
    pub fn core_count__gte(mut self, n: i64) -> Self {
        self.core_count__gte = Some(n);
        self
    }

    /// Number of CPUs (less than).
    pub fn cpu_count__lt(mut self, n: i64) -> Self {
        self.cpu_count__lt = Some(n);
        self
    }

    /// Number of CPUs (less than or equal).
    pub fn cpu_count__lte(mut self, n: i64) -> Self {
        self.cpu_count__lte = Some(n);
        self
    }

    /// Number of CPUs (more than).
    pub fn cpu_count__gt(mut self, n: i64) -> Self {
        self.cpu_count__gt = Some(n);
        self
    }

    /// Number of CPUs (more than or equal).
    pub fn cpu_count__gte(mut self, n: i64) -> Self {
        self.cpu_count__gte = Some(n);
        self
    }

    /// Memory size (MB, less than).
    pub fn total_memory__lt(mut self, n: i64) -> Self {
        self.total_memory__lt = Some(n);
        self
    }

    /// Memory size (MB, less than or equal).
    pub fn total_memory__lte(mut self, n: i64) -> Self {
        self.total_memory__lte = Some(n);
        self
    }

    /// Memory size (MB, more than).
    pub fn total_memory__gt(mut self, n: i64) -> Self {
        self.total_memory__gt = Some(n);
        self
    }

    /// Memory size (MB, more than or equal).
    pub fn total_memory__gte(mut self, n: i64) -> Self {
        self.total_memory__gte = Some(n);
        self
    }

    /// Migration status. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn migration_status(mut self, v: impl Into<String>) -> Self {
        self.migration_status = Some(v.into());
        self
    }

    /// Gateway ip. Example: "192.168.0.1".
    pub fn gateway_ip(mut self, v: impl Into<String>) -> Self {
        self.gateway_ip = Some(v.into());
        self
    }

    /// The ID of the CSV file to filter by. Example: "225494730938493804".
    pub fn csv_filter_id(mut self, v: impl Into<String>) -> Self {
        self.csv_filter_id = Some(v.into());
        self
    }

    /// Supported Remote Script Orchestration level. Example: "none".
    ///
    /// Allowed values: none, pro, ars.
    pub fn rso_level(mut self, v: impl Into<String>) -> Self {
        self.rso_level = Some(v.into());
        self
    }

    /// Include only agents that has Remote Ops Forensicsfeature supported.
    pub fn remote_ops_forensics_supported(mut self, b: bool) -> Self {
        self.remote_ops_forensics_supported = Some(b);
        self
    }

    /// Agents os revision than or equal to given version.
    pub fn windows_os_revision__gte(mut self, n: i64) -> Self {
        self.windows_os_revision__gte = Some(n);
        self
    }

    /// Agents os revision lower than or equal to given version.
    pub fn windows_os_revision__lte(mut self, n: i64) -> Self {
        self.windows_os_revision__lte = Some(n);
        self
    }

    /// A list of included rso_levels. Example: "pro,ars".
    pub fn rso_levels<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.rso_levels = Some(join_csv(vals));
        self
    }
}

/// Query params for `GET /web/api/v2.1/agents/tags` (Get endpoint Tags).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
pub struct AgentsTagsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000 items, use "cursor". Example: "150". Optional.
    #[serde(rename = "skip", skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Example: "10". Optional.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=". Optional.
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects. Optional.
    #[serde(rename = "countOnly", skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up execution time. Optional.
    #[serde(rename = "skipCount", skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The column to sort the results by. Example: "id". Optional.
    ///
    /// Allowed values: key, value, description, updatedAt, createdBy, updatedBy, scopePath.
    #[serde(rename = "sortBy", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction. Example: "asc". Optional.
    ///
    /// Allowed values: asc, desc.
    #[serde(rename = "sortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Return tags from parent scope levels. Optional.
    #[serde(rename = "includeParents", skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return tags from children scope levels. Optional.
    #[serde(rename = "includeChildren", skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// List of tag IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Free-text filter by tag key. Example: "server". Optional.
    #[serde(rename = "key__contains", skip_serializing_if = "Option::is_none")]
    pub key__contains: Option<String>,
    /// Free text search on fields key, value, description. Optional.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Tag key. Optional.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Tag value. Optional.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Tag description. Optional.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Scope path. Optional.
    #[serde(rename = "scopePath", skip_serializing_if = "Option::is_none")]
    pub scope_path: Option<String>,
    /// Created by. Optional.
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Updated by. Optional.
    #[serde(rename = "updatedBy", skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    /// Free-text filter by tag value. Example: "server". Optional.
    #[serde(rename = "value__contains", skip_serializing_if = "Option::is_none")]
    pub value__contains: Option<String>,
    /// Include endpoint counters. Optional.
    #[serde(rename = "includeEndpointCounters", skip_serializing_if = "Option::is_none")]
    pub include_endpoint_counters: Option<bool>,
    /// Include exclusion counters. Optional.
    #[serde(rename = "includeExclusionCounters", skip_serializing_if = "Option::is_none")]
    pub include_exclusion_counters: Option<bool>,
    /// Filter endpoint counts by decommissioned status: [False] for active only, [True] for decommissioned only, [True, False] for both. Example: "True,False". Optional.
    #[serde(rename = "isDecommissioned", skip_serializing_if = "Option::is_none")]
    pub is_decommissioned: Option<String>,
    /// Filter endpoint counts by uninstalled status: [False] for installed only, [True] for uninstalled only, [True, False] for both. Example: "True,False". Optional.
    #[serde(rename = "isUninstalled", skip_serializing_if = "Option::is_none")]
    pub is_uninstalled: Option<String>,
}

impl AgentsTagsQuery {
    /// Skip first number of items (0-1000). To iterate over more than 1000 items, use "cursor". Example: "150".
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }

    /// Limit number of returned items (1-1000). Example: "10".
    pub fn limit(mut self, n: u32) -> Self {
        self.limit = Some(n);
        self
    }

    /// Cursor position returned by the last request. Use to iterate over more than 1000 items. Example: "YWdlbnRfaWQ6NTgwMjkzODE=".
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }

    /// If true, only total number of items will be returned, without any of the actual objects.
    pub fn count_only(mut self, b: bool) -> Self {
        self.count_only = Some(b);
        self
    }

    /// If true, total number of items will not be calculated, which speeds up execution time.
    pub fn skip_count(mut self, b: bool) -> Self {
        self.skip_count = Some(b);
        self
    }

    /// The column to sort the results by. Example: "id".
    ///
    /// Allowed values: key, value, description, updatedAt, createdBy, updatedBy, scopePath.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }

    /// Sort direction. Example: "asc".
    ///
    /// Allowed values: asc, desc.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }

    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }

    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }

    /// Return tags from parent scope levels.
    pub fn include_parents(mut self, b: bool) -> Self {
        self.include_parents = Some(b);
        self
    }

    /// Return tags from children scope levels.
    pub fn include_children(mut self, b: bool) -> Self {
        self.include_children = Some(b);
        self
    }

    /// List of tag IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(vals));
        self
    }

    /// Free-text filter by tag key. Example: "server".
    pub fn key__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.key__contains = Some(join_csv(vals));
        self
    }

    /// Free text search on fields key, value, description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }

    /// Tag key.
    pub fn key(mut self, v: impl Into<String>) -> Self {
        self.key = Some(v.into());
        self
    }

    /// Tag value.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }

    /// Tag description.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }

    /// Scope path.
    pub fn scope_path(mut self, v: impl Into<String>) -> Self {
        self.scope_path = Some(v.into());
        self
    }

    /// Created by.
    pub fn created_by(mut self, v: impl Into<String>) -> Self {
        self.created_by = Some(v.into());
        self
    }

    /// Updated by.
    pub fn updated_by(mut self, v: impl Into<String>) -> Self {
        self.updated_by = Some(v.into());
        self
    }

    /// Free-text filter by tag value. Example: "server".
    pub fn value__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.value__contains = Some(join_csv(vals));
        self
    }

    /// Include endpoint counters.
    pub fn include_endpoint_counters(mut self, b: bool) -> Self {
        self.include_endpoint_counters = Some(b);
        self
    }

    /// Include exclusion counters.
    pub fn include_exclusion_counters(mut self, b: bool) -> Self {
        self.include_exclusion_counters = Some(b);
        self
    }

    /// Filter endpoint counts by decommissioned status: [False] for active only, [True] for decommissioned only, [True, False] for both. Example: "True,False".
    pub fn is_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_decommissioned = Some(join_csv(vals));
        self
    }

    /// Filter endpoint counts by uninstalled status: [False] for installed only, [True] for uninstalled only, [True, False] for both. Example: "True,False".
    pub fn is_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_uninstalled = Some(join_csv(vals));
        self
    }
}

/// Query params for `GET /web/api/v2.1/agents/tags/filters-count` (Endpoint tags count by Filters).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
pub struct AgentsTagsFiltersCountQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Return tags from parent scope levels. Optional.
    #[serde(rename = "includeParents", skip_serializing_if = "Option::is_none")]
    pub include_parents: Option<bool>,
    /// Return tags from children scope levels. Optional.
    #[serde(rename = "includeChildren", skip_serializing_if = "Option::is_none")]
    pub include_children: Option<bool>,
    /// List of tag IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Free-text filter by tag key. Example: "server". Optional.
    #[serde(rename = "key__contains", skip_serializing_if = "Option::is_none")]
    pub key__contains: Option<String>,
    /// Free text search on fields key, value, description. Optional.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Tag key. Optional.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Tag value. Optional.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Tag description. Optional.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Scope path. Optional.
    #[serde(rename = "scopePath", skip_serializing_if = "Option::is_none")]
    pub scope_path: Option<String>,
    /// Created by. Optional.
    #[serde(rename = "createdBy", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Updated by. Optional.
    #[serde(rename = "updatedBy", skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    /// Free-text filter by tag value. Example: "server". Optional.
    #[serde(rename = "value__contains", skip_serializing_if = "Option::is_none")]
    pub value__contains: Option<String>,
}

impl AgentsTagsFiltersCountQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }

    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }

    /// Return tags from parent scope levels.
    pub fn include_parents(mut self, b: bool) -> Self {
        self.include_parents = Some(b);
        self
    }

    /// Return tags from children scope levels.
    pub fn include_children(mut self, b: bool) -> Self {
        self.include_children = Some(b);
        self
    }

    /// List of tag IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(vals));
        self
    }

    /// Free-text filter by tag key. Example: "server".
    pub fn key__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.key__contains = Some(join_csv(vals));
        self
    }

    /// Free text search on fields key, value, description.
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }

    /// Tag key.
    pub fn key(mut self, v: impl Into<String>) -> Self {
        self.key = Some(v.into());
        self
    }

    /// Tag value.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = Some(v.into());
        self
    }

    /// Tag description.
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = Some(v.into());
        self
    }

    /// Scope path.
    pub fn scope_path(mut self, v: impl Into<String>) -> Self {
        self.scope_path = Some(v.into());
        self
    }

    /// Created by.
    pub fn created_by(mut self, v: impl Into<String>) -> Self {
        self.created_by = Some(v.into());
        self
    }

    /// Updated by.
    pub fn updated_by(mut self, v: impl Into<String>) -> Self {
        self.updated_by = Some(v.into());
        self
    }

    /// Free-text filter by tag value. Example: "server".
    pub fn value__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.value__contains = Some(join_csv(vals));
        self
    }
}

/// Query params for `GET /web/api/v2.1/export/agents` (Export Agents).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
pub struct AgentsExportQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredGroupIds", skip_serializing_if = "Option::is_none")]
    pub filtered_group_ids: Option<String>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredSiteIds", skip_serializing_if = "Option::is_none")]
    pub filtered_site_ids: Option<String>,
    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "registeredAt__between", skip_serializing_if = "Option::is_none")]
    pub registered_at__between: Option<String>,
    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastActiveDate__between", skip_serializing_if = "Option::is_none")]
    pub last_active_date__between: Option<String>,
    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastSuccessfulScanDate__between", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__between: Option<String>,
    /// Include only active Agents. Optional.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<String>,
    /// Include only Agents with pending uninstall requests. Optional.
    #[serde(rename = "isPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub is_pending_uninstall: Option<String>,
    /// Include only Agents with at least one active threat. Optional.
    #[serde(rename = "infected", skip_serializing_if = "Option::is_none")]
    pub infected: Option<bool>,
    /// Include only Agents with updated software. Optional.
    #[serde(rename = "isUpToDate", skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<String>,
    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux". Optional.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersions", skip_serializing_if = "Option::is_none")]
    pub agent_versions: Option<String>,
    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersionsNin", skip_serializing_if = "Option::is_none")]
    pub agent_versions_nin: Option<String>,
    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersions", skip_serializing_if = "Option::is_none")]
    pub ranger_versions: Option<String>,
    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersionsNin", skip_serializing_if = "Option::is_none")]
    pub ranger_versions_nin: Option<String>,
    /// OS architecture. Example: "32 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArch", skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<String>,
    /// OS architectures to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArches", skip_serializing_if = "Option::is_none")]
    pub os_arches: Option<String>,
    /// OS architectures not to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArchesNin", skip_serializing_if = "Option::is_none")]
    pub os_arches_nin: Option<String>,
    /// Included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypes", skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Not included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypesNin", skip_serializing_if = "Option::is_none")]
    pub os_types_nin: Option<String>,
    /// Included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatuses", skip_serializing_if = "Option::is_none")]
    pub scan_statuses: Option<String>,
    /// Not included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatusesNin", skip_serializing_if = "Option::is_none")]
    pub scan_statuses_nin: Option<String>,
    /// Included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypes", skip_serializing_if = "Option::is_none")]
    pub machine_types: Option<String>,
    /// Not included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypesNin", skip_serializing_if = "Option::is_none")]
    pub machine_types_nin: Option<String>,
    /// Included storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypes", skip_serializing_if = "Option::is_none")]
    pub storage_types: Option<String>,
    /// Excluded storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypesNin", skip_serializing_if = "Option::is_none")]
    pub storage_types_nin: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatuses", skip_serializing_if = "Option::is_none")]
    pub network_statuses: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatusesNin", skip_serializing_if = "Option::is_none")]
    pub network_statuses_nin: Option<String>,
    /// Included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// Not included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domainsNin", skip_serializing_if = "Option::is_none")]
    pub domains_nin: Option<String>,
    /// Disk encryption status. Optional.
    #[serde(rename = "encryptedApplications", skip_serializing_if = "Option::is_none")]
    pub encrypted_applications: Option<String>,
    /// Total memory range (GB, inclusive). Example: "4-8". Optional.
    #[serde(rename = "totalMemory__between", skip_serializing_if = "Option::is_none")]
    pub total_memory__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "coreCount__between", skip_serializing_if = "Option::is_none")]
    pub core_count__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "cpuCount__between", skip_serializing_if = "Option::is_none")]
    pub cpu_count__between: Option<String>,
    /// Included pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeeded", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed: Option<String>,
    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissions", skip_serializing_if = "Option::is_none")]
    pub missing_permissions: Option<String>,
    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeededNin", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed_nin: Option<String>,
    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissionsNin", skip_serializing_if = "Option::is_none")]
    pub missing_permissions_nin: Option<String>,
    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com". Optional.
    #[serde(rename = "adQuery", skip_serializing_if = "Option::is_none")]
    pub ad_query: Option<String>,
    /// Agent has a local configuration set. Optional.
    #[serde(rename = "hasLocalConfiguration", skip_serializing_if = "Option::is_none")]
    pub has_local_configuration: Option<bool>,
    /// Migration status in. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatuses", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses: Option<String>,
    /// Migration status nin. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatusesNin", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses_nin: Option<String>,
    /// Apps vulnerability status in. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatuses", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses: Option<String>,
    /// Apps vulnerability status nin. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatusesNin", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses_nin: Option<String>,
    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIds", skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<String>,
    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIdsNin", skip_serializing_if = "Option::is_none")]
    pub location_ids_nin: Option<String>,
    /// Include only Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypes", skip_serializing_if = "Option::is_none")]
    pub installer_types: Option<String>,
    /// Exclude Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypesNin", skip_serializing_if = "Option::is_none")]
    pub installer_types_nin: Option<String>,
    /// Agent operational state. Optional.
    #[serde(rename = "operationalStates", skip_serializing_if = "Option::is_none")]
    pub operational_states: Option<String>,
    /// Do not include these Agent operational states. Optional.
    #[serde(rename = "operationalStatesNin", skip_serializing_if = "Option::is_none")]
    pub operational_states_nin: Option<String>,
    /// Agent remote profiling state. Optional.
    #[serde(rename = "remoteProfilingStates", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states: Option<String>,
    /// Do not include these Agent remote profiling states. Optional.
    #[serde(rename = "remoteProfilingStatesNin", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states_nin: Option<String>,
    /// Status of Network Discovery. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatuses", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses: Option<String>,
    /// Do not include these Network Scanner Statuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatusesNin", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses_nin: Option<String>,
    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatus", skip_serializing_if = "Option::is_none")]
    pub ranger_status: Option<String>,
    /// Has at least one threat with at least one mitigation action pending reboot to succeed. Optional.
    #[serde(rename = "threatRebootRequired", skip_serializing_if = "Option::is_none")]
    pub threat_reboot_required: Option<String>,
    /// The agents supports Network Quarantine Control and its enabled for the agent's group. Optional.
    #[serde(rename = "networkQuarantineEnabled", skip_serializing_if = "Option::is_none")]
    pub network_quarantine_enabled: Option<String>,
    /// The agents supports Firewall Control and it is enabled for the agent's group. Optional.
    #[serde(rename = "firewallEnabled", skip_serializing_if = "Option::is_none")]
    pub firewall_enabled: Option<String>,
    /// The agents supports Location Awareness and it is enabled for the agent's group. Optional.
    #[serde(rename = "locationEnabled", skip_serializing_if = "Option::is_none")]
    pub location_enabled: Option<String>,
    /// Agents from which cloud provider. Optional.
    #[serde(rename = "cloudProvider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// Exclude Agents from these cloud provider. Optional.
    #[serde(rename = "cloudProviderNin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_nin: Option<String>,
    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}". Optional.
    #[serde(rename = "tagsData", skip_serializing_if = "Option::is_none")]
    pub tags_data: Option<String>,
    /// Include only Agents that have any tags assigned if True, or none if False. Optional.
    #[serde(rename = "hasTags", skip_serializing_if = "Option::is_none")]
    pub has_tags: Option<bool>,
    /// The agents that are ADConnectors if True, or not if False. Optional.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The agents has Hyper Automate PNA enabled if True, or not if False. Optional.
    #[serde(rename = "isHyperAutomate", skip_serializing_if = "Option::is_none")]
    pub is_hyper_automate: Option<String>,
    /// Included agent active protections. Example: "edr,idr". Optional.
    ///
    /// Allowed values: edr, idr.
    #[serde(rename = "activeProtection", skip_serializing_if = "Option::is_none")]
    pub active_protection: Option<String>,
    /// Containerized workload counts. Optional.
    #[serde(rename = "containerizedWorkloadCounts", skip_serializing_if = "Option::is_none")]
    pub containerized_workload_counts: Option<String>,
    /// Indicates whether the agent protects containerized workload at the moment. Optional.
    #[serde(rename = "hasContainerizedWorkload", skip_serializing_if = "Option::is_none")]
    pub has_containerized_workload: Option<String>,
    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "pacFileUsage", skip_serializing_if = "Option::is_none")]
    pub pac_file_usage: Option<String>,
    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethod", skip_serializing_if = "Option::is_none")]
    pub proxy_method: Option<String>,
    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethodNin", skip_serializing_if = "Option::is_none")]
    pub proxy_method_nin: Option<String>,
    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isMgmtProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_mgmt_proxy_enabled: Option<String>,
    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isEventSearchProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_event_search_proxy_enabled: Option<String>,
    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0". Optional.
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip__contains: Option<String>,
    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN". Optional.
    #[serde(rename = "computerName__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name__contains: Option<String>,
    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0". Optional.
    #[serde(rename = "networkInterfaceInet__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_inet__contains: Option<String>,
    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfacePhysical__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_physical__contains: Option<String>,
    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfaceGatewayMacAddress__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_gateway_mac_address__contains: Option<String>,
    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1". Optional.
    #[serde(rename = "lastLoggedInUserName__contains", skip_serializing_if = "Option::is_none")]
    pub last_logged_in_user_name__contains: Option<String>,
    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1". Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_query__contains: Option<String>,
    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_name__contains: Option<String>,
    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John". Optional.
    #[serde(rename = "adUserQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_query__contains: Option<String>,
    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_name__contains: Option<String>,
    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows". Optional.
    #[serde(rename = "adComputerQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_query__contains: Option<String>,
    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055". Optional.
    #[serde(rename = "uuid__contains", skip_serializing_if = "Option::is_none")]
    pub uuid__contains: Option<String>,
    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine". Optional.
    #[serde(rename = "externalId__contains", skip_serializing_if = "Option::is_none")]
    pub external_id__contains: Option<String>,
    /// Free-text filter by aws role(supports multiple values). Optional.
    #[serde(rename = "awsRole__contains", skip_serializing_if = "Option::is_none")]
    pub aws_role__contains: Option<String>,
    /// Free-text filter by aws securityGroups(supports multiple values). Optional.
    #[serde(rename = "awsSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub aws_security_groups__contains: Option<String>,
    /// Free-text filter by aws subnet ids (supports multiple values). Optional.
    #[serde(rename = "awsSubnetIds__contains", skip_serializing_if = "Option::is_none")]
    pub aws_subnet_ids__contains: Option<String>,
    /// Free-text filter by agent namespace (supports multiple values). Optional.
    #[serde(rename = "agentNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub agent_namespace__contains: Option<String>,
    /// Free-text filter by agent pod name (supports multiple values). Optional.
    #[serde(rename = "agentPodName__contains", skip_serializing_if = "Option::is_none")]
    pub agent_pod_name__contains: Option<String>,
    /// Free-text filter by azure resource group(supports multiple values). Optional.
    #[serde(rename = "azureResourceGroup__contains", skip_serializing_if = "Option::is_none")]
    pub azure_resource_group__contains: Option<String>,
    /// Free-text filter by cloud account (supports multiple values). Optional.
    #[serde(rename = "cloudAccount__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_account__contains: Option<String>,
    /// Free-text filter by cloud image (supports multiple values). Optional.
    #[serde(rename = "cloudImage__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_image__contains: Option<String>,
    /// Free-text filter by cloud instance id(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_id__contains: Option<String>,
    /// Free-text filter by cloud instance size(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceSize__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_size__contains: Option<String>,
    /// Free-text filter by cloud location (supports multiple values). Optional.
    #[serde(rename = "cloudLocation__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_location__contains: Option<String>,
    /// Free-text filter by cloud network (supports multiple values). Optional.
    #[serde(rename = "cloudNetwork__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_network__contains: Option<String>,
    /// Free-text filter by cloud tags (supports multiple values). Optional.
    #[serde(rename = "cloudTags__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags__contains: Option<String>,
    /// Free-text filter by cluster name (supports multiple values). Optional.
    #[serde(rename = "clusterName__contains", skip_serializing_if = "Option::is_none")]
    pub cluster_name__contains: Option<String>,
    /// Free-text filter by gcp service account (supports multiple values). Optional.
    #[serde(rename = "gcpServiceAccount__contains", skip_serializing_if = "Option::is_none")]
    pub gcp_service_account__contains: Option<String>,
    /// Free-text filter by K8s node labels (supports multiple values). Optional.
    #[serde(rename = "k8sNodeLabels__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_labels__contains: Option<String>,
    /// Free-text filter by K8s node name (supports multiple values). Optional.
    #[serde(rename = "k8sNodeName__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_name__contains: Option<String>,
    /// Free-text filter by K8s type(supports multiple values). Optional.
    #[serde(rename = "k8sType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_type__contains: Option<String>,
    /// Free-text filter by K8s version (supports multiple values). Optional.
    #[serde(rename = "k8sVersion__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_version__contains: Option<String>,
    /// Free-text filter by live update ID (supports multiple values). Optional.
    #[serde(rename = "liveUpdateId__contains", skip_serializing_if = "Option::is_none")]
    pub live_update_id__contains: Option<String>,
    /// Free-text filter by Serial Number (supports multiple values). Optional.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number__contains: Option<String>,
    /// Free-text filter by Entra ID (supports multiple values). Optional.
    #[serde(rename = "entraId__contains", skip_serializing_if = "Option::is_none")]
    pub entra_id__contains: Option<String>,
    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD". Optional.
    #[serde(rename = "cpuId__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_id__contains: Option<String>,
    /// Free-text filter by ECS type. Optional.
    #[serde(rename = "ecsType__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_type__contains: Option<String>,
    /// Free-text filter by ECS version. Optional.
    #[serde(rename = "ecsVersion__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_version__contains: Option<String>,
    /// Free-text filter by ECS cluster name. Optional.
    #[serde(rename = "ecsClusterName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_name__contains: Option<String>,
    /// Free-text filter by ECS task arn. Optional.
    #[serde(rename = "ecsTaskArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_arn__contains: Option<String>,
    /// Free-text filter by ECS task availability zone. Optional.
    #[serde(rename = "ecsTaskAvailabilityZone__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_availability_zone__contains: Option<String>,
    /// Free-text filter by ECS service name. Optional.
    #[serde(rename = "ecsServiceName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_name__contains: Option<String>,
    /// Free-text filter by ECS service arn. Optional.
    #[serde(rename = "ecsServiceArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_arn__contains: Option<String>,
    /// Free-text filter by ECS task definition family. Optional.
    #[serde(rename = "ecsTaskDefinitionFamily__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_family__contains: Option<String>,
    /// Free-text filter by ECS task definition revision. Optional.
    #[serde(rename = "ecsTaskDefinitionRevision__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_revision__contains: Option<String>,
    /// Free-text filter by ECS task definition arn. Optional.
    #[serde(rename = "ecsTaskDefinitionArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_arn__contains: Option<String>,
    /// Include active, decommissioned or both. Example: "True,False". Optional.
    #[serde(rename = "isDecommissioned", skip_serializing_if = "Option::is_none")]
    pub is_decommissioned: Option<String>,
    /// Include installed, uninstalled or both. Example: "True,False". Optional.
    #[serde(rename = "isUninstalled", skip_serializing_if = "Option::is_none")]
    pub is_uninstalled: Option<String>,
    /// Free-text filter by computer name or uuid (supports multiple values). Optional.
    #[serde(rename = "computerNameOrUuid__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name_or_uuid__contains: Option<String>,
    /// Included Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "idsNin", skip_serializing_if = "Option::is_none")]
    pub ids_nin: Option<String>,
    /// Include all Agents matching this saved filter. Example: "225494730938493804". Optional.
    #[serde(rename = "filterId", skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<String>,
    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gte: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lt: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lte: Option<String>,
    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gt: Option<String>,
    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "decommissionedAt__between", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__between: Option<String>,
    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at__lt: Option<String>,
    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at__lte: Option<String>,
    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at__gt: Option<String>,
    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at__gte: Option<String>,
    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at__between: Option<String>,
    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at__lt: Option<String>,
    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at__lte: Option<String>,
    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at__gt: Option<String>,
    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at__gte: Option<String>,
    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at__between: Option<String>,
    /// Match computer name partially (substring). Example: "Lab1". Optional.
    #[serde(rename = "computerName__like", skip_serializing_if = "Option::is_none")]
    pub computer_name__like: Option<String>,
    /// Computer name. Example: "My Office Desktop". Optional.
    #[serde(rename = "computerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// Agents versions less than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lt", skip_serializing_if = "Option::is_none")]
    pub agent_version__lt: Option<String>,
    /// Agents versions less than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lte", skip_serializing_if = "Option::is_none")]
    pub agent_version__lte: Option<String>,
    /// Agents versions greater than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gt", skip_serializing_if = "Option::is_none")]
    pub agent_version__gt: Option<String>,
    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gte", skip_serializing_if = "Option::is_none")]
    pub agent_version__gte: Option<String>,
    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144". Optional.
    #[serde(rename = "agentVersion__between", skip_serializing_if = "Option::is_none")]
    pub agent_version__between: Option<String>,
    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2". Optional.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22". Optional.
    #[serde(rename = "uuids", skip_serializing_if = "Option::is_none")]
    pub uuids: Option<String>,
    /// Scan status. Example: "none". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatus", skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// Include only Agents that have threats with this mitigation status. Example: "mitigated". Optional.
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    #[serde(rename = "threatMitigationStatus", skip_serializing_if = "Option::is_none")]
    pub threat_mitigation_status: Option<String>,
    /// Include only Agents with at least one resolved threat. Optional.
    #[serde(rename = "threatResolved", skip_serializing_if = "Option::is_none")]
    pub threat_resolved: Option<bool>,
    /// Include only Agents with at least one hidden threat. Optional.
    #[serde(rename = "threatHidden", skip_serializing_if = "Option::is_none")]
    pub threat_hidden: Option<bool>,
    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94". Optional.
    #[serde(rename = "threatContentHash", skip_serializing_if = "Option::is_none")]
    pub threat_content_hash: Option<String>,
    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lt: Option<String>,
    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lte: Option<String>,
    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gt: Option<String>,
    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gte: Option<String>,
    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "threatCreatedAt__between", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__between: Option<String>,
    /// Include Agents with this amount of active threats. Example: "3". Optional.
    #[serde(rename = "activeThreats", skip_serializing_if = "Option::is_none")]
    pub active_threats: Option<i64>,
    /// Include Agents with at least this amount of active threats. Example: "5". Optional.
    #[serde(rename = "activeThreats__gt", skip_serializing_if = "Option::is_none")]
    pub active_threats__gt: Option<i64>,
    /// Agent mitigation mode policy. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationMode", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode: Option<String>,
    /// Mitigation mode policy for suspicious activity. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationModeSuspicious", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode_suspicious: Option<String>,
    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lt", skip_serializing_if = "Option::is_none")]
    pub registered_at__lt: Option<String>,
    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lte", skip_serializing_if = "Option::is_none")]
    pub registered_at__lte: Option<String>,
    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gt", skip_serializing_if = "Option::is_none")]
    pub registered_at__gt: Option<String>,
    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gte", skip_serializing_if = "Option::is_none")]
    pub registered_at__gte: Option<String>,
    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lt: Option<String>,
    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lte: Option<String>,
    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gt: Option<String>,
    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gte: Option<String>,
    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lt: Option<String>,
    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lte: Option<String>,
    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gt: Option<String>,
    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gte: Option<String>,
    /// CPU cores (less than). Optional.
    #[serde(rename = "coreCount__lt", skip_serializing_if = "Option::is_none")]
    pub core_count__lt: Option<i64>,
    /// CPU cores (less than or equal). Optional.
    #[serde(rename = "coreCount__lte", skip_serializing_if = "Option::is_none")]
    pub core_count__lte: Option<i64>,
    /// CPU cores (more than). Optional.
    #[serde(rename = "coreCount__gt", skip_serializing_if = "Option::is_none")]
    pub core_count__gt: Option<i64>,
    /// CPU cores (more than or equal). Optional.
    #[serde(rename = "coreCount__gte", skip_serializing_if = "Option::is_none")]
    pub core_count__gte: Option<i64>,
    /// Number of CPUs (less than). Optional.
    #[serde(rename = "cpuCount__lt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lt: Option<i64>,
    /// Number of CPUs (less than or equal). Optional.
    #[serde(rename = "cpuCount__lte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lte: Option<i64>,
    /// Number of CPUs (more than). Optional.
    #[serde(rename = "cpuCount__gt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gt: Option<i64>,
    /// Number of CPUs (more than or equal). Optional.
    #[serde(rename = "cpuCount__gte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gte: Option<i64>,
    /// Memory size (MB, less than). Optional.
    #[serde(rename = "totalMemory__lt", skip_serializing_if = "Option::is_none")]
    pub total_memory__lt: Option<i64>,
    /// Memory size (MB, less than or equal). Optional.
    #[serde(rename = "totalMemory__lte", skip_serializing_if = "Option::is_none")]
    pub total_memory__lte: Option<i64>,
    /// Memory size (MB, more than). Optional.
    #[serde(rename = "totalMemory__gt", skip_serializing_if = "Option::is_none")]
    pub total_memory__gt: Option<i64>,
    /// Memory size (MB, more than or equal). Optional.
    #[serde(rename = "totalMemory__gte", skip_serializing_if = "Option::is_none")]
    pub total_memory__gte: Option<i64>,
    /// Migration status. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "migrationStatus", skip_serializing_if = "Option::is_none")]
    pub migration_status: Option<String>,
    /// Gateway ip. Example: "192.168.0.1". Optional.
    #[serde(rename = "gatewayIp", skip_serializing_if = "Option::is_none")]
    pub gateway_ip: Option<String>,
    /// The ID of the CSV file to filter by. Example: "225494730938493804". Optional.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<String>,
    /// Supported Remote Script Orchestration level. Example: "none". Optional.
    ///
    /// Allowed values: none, pro, ars.
    #[serde(rename = "rsoLevel", skip_serializing_if = "Option::is_none")]
    pub rso_level: Option<String>,
    /// Include only agents that has Remote Ops Forensicsfeature supported. Optional.
    #[serde(rename = "remoteOpsForensicsSupported", skip_serializing_if = "Option::is_none")]
    pub remote_ops_forensics_supported: Option<bool>,
    /// Agents os revision than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__gte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__gte: Option<i64>,
    /// Agents os revision lower than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__lte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__lte: Option<i64>,
    /// A list of included rso_levels. Example: "pro,ars". Optional.
    #[serde(rename = "rsoLevels", skip_serializing_if = "Option::is_none")]
    pub rso_levels: Option<String>,
}

impl AgentsExportQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }

    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_group_ids = Some(join_csv(vals));
        self
    }

    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_site_ids = Some(join_csv(vals));
        self
    }

    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn registered_at__between(mut self, v: impl Into<String>) -> Self {
        self.registered_at__between = Some(v.into());
        self
    }

    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_active_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__between = Some(v.into());
        self
    }

    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_successful_scan_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__between = Some(v.into());
        self
    }

    /// Include only active Agents.
    pub fn is_active<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_active = Some(join_csv(vals));
        self
    }

    /// Include only Agents with pending uninstall requests.
    pub fn is_pending_uninstall<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_pending_uninstall = Some(join_csv(vals));
        self
    }

    /// Include only Agents with at least one active threat.
    pub fn infected(mut self, b: bool) -> Self {
        self.infected = Some(b);
        self
    }

    /// Include only Agents with updated software.
    pub fn is_up_to_date<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_up_to_date = Some(join_csv(vals));
        self
    }

    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux".
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }

    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions = Some(join_csv(vals));
        self
    }

    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions_nin = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions_nin = Some(join_csv(vals));
        self
    }

    /// OS architecture. Example: "32 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arch(mut self, v: impl Into<String>) -> Self {
        self.os_arch = Some(v.into());
        self
    }

    /// OS architectures to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches = Some(join_csv(vals));
        self
    }

    /// OS architectures not to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches_nin = Some(join_csv(vals));
        self
    }

    /// Included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(vals));
        self
    }

    /// Not included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types_nin = Some(join_csv(vals));
        self
    }

    /// Included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses = Some(join_csv(vals));
        self
    }

    /// Not included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types = Some(join_csv(vals));
        self
    }

    /// Not included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types_nin = Some(join_csv(vals));
        self
    }

    /// Included storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types = Some(join_csv(vals));
        self
    }

    /// Excluded storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types_nin = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains = Some(join_csv(vals));
        self
    }

    /// Not included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains_nin = Some(join_csv(vals));
        self
    }

    /// Disk encryption status.
    pub fn encrypted_applications<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encrypted_applications = Some(join_csv(vals));
        self
    }

    /// Total memory range (GB, inclusive). Example: "4-8".
    pub fn total_memory__between(mut self, v: impl Into<String>) -> Self {
        self.total_memory__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn core_count__between(mut self, v: impl Into<String>) -> Self {
        self.core_count__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn cpu_count__between(mut self, v: impl Into<String>) -> Self {
        self.cpu_count__between = Some(v.into());
        self
    }

    /// Included pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed = Some(join_csv(vals));
        self
    }

    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions = Some(join_csv(vals));
        self
    }

    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed_nin = Some(join_csv(vals));
        self
    }

    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions_nin = Some(join_csv(vals));
        self
    }

    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com".
    pub fn ad_query(mut self, v: impl Into<String>) -> Self {
        self.ad_query = Some(v.into());
        self
    }

    /// Agent has a local configuration set.
    pub fn has_local_configuration(mut self, b: bool) -> Self {
        self.has_local_configuration = Some(b);
        self
    }

    /// Migration status in. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses = Some(join_csv(vals));
        self
    }

    /// Migration status nin. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status in. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status nin. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids = Some(join_csv(vals));
        self
    }

    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types = Some(join_csv(vals));
        self
    }

    /// Exclude Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types_nin = Some(join_csv(vals));
        self
    }

    /// Agent operational state.
    pub fn operational_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent operational states.
    pub fn operational_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states_nin = Some(join_csv(vals));
        self
    }

    /// Agent remote profiling state.
    pub fn remote_profiling_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent remote profiling states.
    pub fn remote_profiling_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states_nin = Some(join_csv(vals));
        self
    }

    /// Status of Network Discovery. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses = Some(join_csv(vals));
        self
    }

    /// Do not include these Network Scanner Statuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses_nin = Some(join_csv(vals));
        self
    }

    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_status(mut self, v: impl Into<String>) -> Self {
        self.ranger_status = Some(v.into());
        self
    }

    /// Has at least one threat with at least one mitigation action pending reboot to succeed.
    pub fn threat_reboot_required<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_reboot_required = Some(join_csv(vals));
        self
    }

    /// The agents supports Network Quarantine Control and its enabled for the agent's group.
    pub fn network_quarantine_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_quarantine_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Firewall Control and it is enabled for the agent's group.
    pub fn firewall_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.firewall_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Location Awareness and it is enabled for the agent's group.
    pub fn location_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_enabled = Some(join_csv(vals));
        self
    }

    /// Agents from which cloud provider.
    pub fn cloud_provider<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider = Some(join_csv(vals));
        self
    }

    /// Exclude Agents from these cloud provider.
    pub fn cloud_provider_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_nin = Some(join_csv(vals));
        self
    }

    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}".
    pub fn tags_data(mut self, v: impl Into<String>) -> Self {
        self.tags_data = Some(v.into());
        self
    }

    /// Include only Agents that have any tags assigned if True, or none if False.
    pub fn has_tags(mut self, b: bool) -> Self {
        self.has_tags = Some(b);
        self
    }

    /// The agents that are ADConnectors if True, or not if False.
    pub fn is_ad_connector<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(join_csv(vals));
        self
    }

    /// The agents has Hyper Automate PNA enabled if True, or not if False.
    pub fn is_hyper_automate<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_hyper_automate = Some(join_csv(vals));
        self
    }

    /// Included agent active protections. Example: "edr,idr".
    ///
    /// Allowed values: edr, idr.
    pub fn active_protection<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_protection = Some(join_csv(vals));
        self
    }

    /// Containerized workload counts.
    pub fn containerized_workload_counts(mut self, v: impl Into<String>) -> Self {
        self.containerized_workload_counts = Some(v.into());
        self
    }

    /// Indicates whether the agent protects containerized workload at the moment.
    pub fn has_containerized_workload<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.has_containerized_workload = Some(join_csv(vals));
        self
    }

    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported).
    pub fn pac_file_usage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.pac_file_usage = Some(join_csv(vals));
        self
    }

    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method = Some(join_csv(vals));
        self
    }

    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported).
    pub fn is_mgmt_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_mgmt_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported).
    pub fn is_event_search_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_event_search_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0".
    pub fn external_ip__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN".
    pub fn computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0".
    pub fn network_interface_inet__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_inet__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_physical__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_physical__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_gateway_mac_address__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_gateway_mac_address__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1".
    pub fn last_logged_in_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.last_logged_in_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1".
    pub fn os_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John".
    pub fn ad_user_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows".
    pub fn ad_computer_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055".
    pub fn uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuid__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine".
    pub fn external_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws role(supports multiple values).
    pub fn aws_role__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_role__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws securityGroups(supports multiple values).
    pub fn aws_security_groups__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_security_groups__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws subnet ids (supports multiple values).
    pub fn aws_subnet_ids__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_subnet_ids__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent namespace (supports multiple values).
    pub fn agent_namespace__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_namespace__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent pod name (supports multiple values).
    pub fn agent_pod_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pod_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by azure resource group(supports multiple values).
    pub fn azure_resource_group__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.azure_resource_group__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud account (supports multiple values).
    pub fn cloud_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud image (supports multiple values).
    pub fn cloud_image__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_image__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance id(supports multiple values).
    pub fn cloud_instance_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance size(supports multiple values).
    pub fn cloud_instance_size__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_size__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud location (supports multiple values).
    pub fn cloud_location__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_location__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud network (supports multiple values).
    pub fn cloud_network__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_network__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud tags (supports multiple values).
    pub fn cloud_tags__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cluster name (supports multiple values).
    pub fn cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by gcp service account (supports multiple values).
    pub fn gcp_service_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gcp_service_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node labels (supports multiple values).
    pub fn k8s_node_labels__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_labels__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node name (supports multiple values).
    pub fn k8s_node_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s type(supports multiple values).
    pub fn k8s_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s version (supports multiple values).
    pub fn k8s_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by live update ID (supports multiple values).
    pub fn live_update_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.live_update_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Serial Number (supports multiple values).
    pub fn serial_number__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Entra ID (supports multiple values).
    pub fn entra_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.entra_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD".
    pub fn cpu_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS type.
    pub fn ecs_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS version.
    pub fn ecs_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS cluster name.
    pub fn ecs_cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task arn.
    pub fn ecs_task_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task availability zone.
    pub fn ecs_task_availability_zone__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_availability_zone__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service name.
    pub fn ecs_service_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service arn.
    pub fn ecs_service_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition family.
    pub fn ecs_task_definition_family__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_family__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition revision.
    pub fn ecs_task_definition_revision__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_revision__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition arn.
    pub fn ecs_task_definition_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_arn__contains = Some(join_csv(vals));
        self
    }

    /// Include active, decommissioned or both. Example: "True,False".
    pub fn is_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_decommissioned = Some(join_csv(vals));
        self
    }

    /// Include installed, uninstalled or both. Example: "True,False".
    pub fn is_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_uninstalled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name or uuid (supports multiple values).
    pub fn computer_name_or_uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name_or_uuid__contains = Some(join_csv(vals));
        self
    }

    /// Included Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(vals));
        self
    }

    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids_nin = Some(join_csv(vals));
        self
    }

    /// Include all Agents matching this saved filter. Example: "225494730938493804".
    pub fn filter_id(mut self, v: impl Into<String>) -> Self {
        self.filter_id = Some(v.into());
        self
    }

    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gte = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lt = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lte = Some(v.into());
        self
    }

    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gt = Some(v.into());
        self
    }

    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn decommissioned_at__between(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__between = Some(v.into());
        self
    }

    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.created_at__lt = Some(v.into());
        self
    }

    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.created_at__lte = Some(v.into());
        self
    }

    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.created_at__gt = Some(v.into());
        self
    }

    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.created_at__gte = Some(v.into());
        self
    }

    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn created_at__between(mut self, v: impl Into<String>) -> Self {
        self.created_at__between = Some(v.into());
        self
    }

    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lt = Some(v.into());
        self
    }

    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lte = Some(v.into());
        self
    }

    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gt = Some(v.into());
        self
    }

    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gte = Some(v.into());
        self
    }

    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.updated_at__between = Some(v.into());
        self
    }

    /// Match computer name partially (substring). Example: "Lab1".
    pub fn computer_name__like(mut self, v: impl Into<String>) -> Self {
        self.computer_name__like = Some(v.into());
        self
    }

    /// Computer name. Example: "My Office Desktop".
    pub fn computer_name(mut self, v: impl Into<String>) -> Self {
        self.computer_name = Some(v.into());
        self
    }

    /// Agents versions less than given version. Example: "2.5.1.1320".
    pub fn agent_version__lt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lt = Some(v.into());
        self
    }

    /// Agents versions less than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__lte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lte = Some(v.into());
        self
    }

    /// Agents versions greater than given version. Example: "2.5.1.1320".
    pub fn agent_version__gt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gt = Some(v.into());
        self
    }

    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__gte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gte = Some(v.into());
        self
    }

    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144".
    pub fn agent_version__between(mut self, v: impl Into<String>) -> Self {
        self.agent_version__between = Some(v.into());
        self
    }

    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2".
    pub fn uuid(mut self, v: impl Into<String>) -> Self {
        self.uuid = Some(v.into());
        self
    }

    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22".
    pub fn uuids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuids = Some(join_csv(vals));
        self
    }

    /// Scan status. Example: "none".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_status(mut self, v: impl Into<String>) -> Self {
        self.scan_status = Some(v.into());
        self
    }

    /// Include only Agents that have threats with this mitigation status. Example: "mitigated".
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    pub fn threat_mitigation_status(mut self, v: impl Into<String>) -> Self {
        self.threat_mitigation_status = Some(v.into());
        self
    }

    /// Include only Agents with at least one resolved threat.
    pub fn threat_resolved(mut self, b: bool) -> Self {
        self.threat_resolved = Some(b);
        self
    }

    /// Include only Agents with at least one hidden threat.
    pub fn threat_hidden(mut self, b: bool) -> Self {
        self.threat_hidden = Some(b);
        self
    }

    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94".
    pub fn threat_content_hash(mut self, v: impl Into<String>) -> Self {
        self.threat_content_hash = Some(v.into());
        self
    }

    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lt = Some(v.into());
        self
    }

    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lte = Some(v.into());
        self
    }

    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gt = Some(v.into());
        self
    }

    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gte = Some(v.into());
        self
    }

    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn threat_created_at__between(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__between = Some(v.into());
        self
    }

    /// Include Agents with this amount of active threats. Example: "3".
    pub fn active_threats(mut self, n: i64) -> Self {
        self.active_threats = Some(n);
        self
    }

    /// Include Agents with at least this amount of active threats. Example: "5".
    pub fn active_threats__gt(mut self, n: i64) -> Self {
        self.active_threats__gt = Some(n);
        self
    }

    /// Agent mitigation mode policy. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode = Some(v.into());
        self
    }

    /// Mitigation mode policy for suspicious activity. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode_suspicious(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode_suspicious = Some(v.into());
        self
    }

    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lt = Some(v.into());
        self
    }

    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lte = Some(v.into());
        self
    }

    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gt = Some(v.into());
        self
    }

    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gte = Some(v.into());
        self
    }

    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lt = Some(v.into());
        self
    }

    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lte = Some(v.into());
        self
    }

    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gt = Some(v.into());
        self
    }

    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gte = Some(v.into());
        self
    }

    /// CPU cores (less than).
    pub fn core_count__lt(mut self, n: i64) -> Self {
        self.core_count__lt = Some(n);
        self
    }

    /// CPU cores (less than or equal).
    pub fn core_count__lte(mut self, n: i64) -> Self {
        self.core_count__lte = Some(n);
        self
    }

    /// CPU cores (more than).
    pub fn core_count__gt(mut self, n: i64) -> Self {
        self.core_count__gt = Some(n);
        self
    }

    /// CPU cores (more than or equal).
    pub fn core_count__gte(mut self, n: i64) -> Self {
        self.core_count__gte = Some(n);
        self
    }

    /// Number of CPUs (less than).
    pub fn cpu_count__lt(mut self, n: i64) -> Self {
        self.cpu_count__lt = Some(n);
        self
    }

    /// Number of CPUs (less than or equal).
    pub fn cpu_count__lte(mut self, n: i64) -> Self {
        self.cpu_count__lte = Some(n);
        self
    }

    /// Number of CPUs (more than).
    pub fn cpu_count__gt(mut self, n: i64) -> Self {
        self.cpu_count__gt = Some(n);
        self
    }

    /// Number of CPUs (more than or equal).
    pub fn cpu_count__gte(mut self, n: i64) -> Self {
        self.cpu_count__gte = Some(n);
        self
    }

    /// Memory size (MB, less than).
    pub fn total_memory__lt(mut self, n: i64) -> Self {
        self.total_memory__lt = Some(n);
        self
    }

    /// Memory size (MB, less than or equal).
    pub fn total_memory__lte(mut self, n: i64) -> Self {
        self.total_memory__lte = Some(n);
        self
    }

    /// Memory size (MB, more than).
    pub fn total_memory__gt(mut self, n: i64) -> Self {
        self.total_memory__gt = Some(n);
        self
    }

    /// Memory size (MB, more than or equal).
    pub fn total_memory__gte(mut self, n: i64) -> Self {
        self.total_memory__gte = Some(n);
        self
    }

    /// Migration status. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn migration_status(mut self, v: impl Into<String>) -> Self {
        self.migration_status = Some(v.into());
        self
    }

    /// Gateway ip. Example: "192.168.0.1".
    pub fn gateway_ip(mut self, v: impl Into<String>) -> Self {
        self.gateway_ip = Some(v.into());
        self
    }

    /// The ID of the CSV file to filter by. Example: "225494730938493804".
    pub fn csv_filter_id(mut self, v: impl Into<String>) -> Self {
        self.csv_filter_id = Some(v.into());
        self
    }

    /// Supported Remote Script Orchestration level. Example: "none".
    ///
    /// Allowed values: none, pro, ars.
    pub fn rso_level(mut self, v: impl Into<String>) -> Self {
        self.rso_level = Some(v.into());
        self
    }

    /// Include only agents that has Remote Ops Forensicsfeature supported.
    pub fn remote_ops_forensics_supported(mut self, b: bool) -> Self {
        self.remote_ops_forensics_supported = Some(b);
        self
    }

    /// Agents os revision than or equal to given version.
    pub fn windows_os_revision__gte(mut self, n: i64) -> Self {
        self.windows_os_revision__gte = Some(n);
        self
    }

    /// Agents os revision lower than or equal to given version.
    pub fn windows_os_revision__lte(mut self, n: i64) -> Self {
        self.windows_os_revision__lte = Some(n);
        self
    }

    /// A list of included rso_levels. Example: "pro,ars".
    pub fn rso_levels<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.rso_levels = Some(join_csv(vals));
        self
    }
}

/// Query params for `GET /web/api/v2.1/export/agents-light` (Export Agents - Light).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
pub struct AgentsExportLightQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredGroupIds", skip_serializing_if = "Option::is_none")]
    pub filtered_group_ids: Option<String>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredSiteIds", skip_serializing_if = "Option::is_none")]
    pub filtered_site_ids: Option<String>,
    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "registeredAt__between", skip_serializing_if = "Option::is_none")]
    pub registered_at__between: Option<String>,
    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastActiveDate__between", skip_serializing_if = "Option::is_none")]
    pub last_active_date__between: Option<String>,
    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastSuccessfulScanDate__between", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__between: Option<String>,
    /// Include only active Agents. Optional.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<String>,
    /// Include only Agents with pending uninstall requests. Optional.
    #[serde(rename = "isPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub is_pending_uninstall: Option<String>,
    /// Include only Agents with at least one active threat. Optional.
    #[serde(rename = "infected", skip_serializing_if = "Option::is_none")]
    pub infected: Option<bool>,
    /// Include only Agents with updated software. Optional.
    #[serde(rename = "isUpToDate", skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<String>,
    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux". Optional.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersions", skip_serializing_if = "Option::is_none")]
    pub agent_versions: Option<String>,
    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersionsNin", skip_serializing_if = "Option::is_none")]
    pub agent_versions_nin: Option<String>,
    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersions", skip_serializing_if = "Option::is_none")]
    pub ranger_versions: Option<String>,
    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersionsNin", skip_serializing_if = "Option::is_none")]
    pub ranger_versions_nin: Option<String>,
    /// OS architecture. Example: "32 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArch", skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<String>,
    /// OS architectures to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArches", skip_serializing_if = "Option::is_none")]
    pub os_arches: Option<String>,
    /// OS architectures not to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArchesNin", skip_serializing_if = "Option::is_none")]
    pub os_arches_nin: Option<String>,
    /// Included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypes", skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Not included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypesNin", skip_serializing_if = "Option::is_none")]
    pub os_types_nin: Option<String>,
    /// Included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatuses", skip_serializing_if = "Option::is_none")]
    pub scan_statuses: Option<String>,
    /// Not included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatusesNin", skip_serializing_if = "Option::is_none")]
    pub scan_statuses_nin: Option<String>,
    /// Included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypes", skip_serializing_if = "Option::is_none")]
    pub machine_types: Option<String>,
    /// Not included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypesNin", skip_serializing_if = "Option::is_none")]
    pub machine_types_nin: Option<String>,
    /// Included storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypes", skip_serializing_if = "Option::is_none")]
    pub storage_types: Option<String>,
    /// Excluded storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypesNin", skip_serializing_if = "Option::is_none")]
    pub storage_types_nin: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatuses", skip_serializing_if = "Option::is_none")]
    pub network_statuses: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatusesNin", skip_serializing_if = "Option::is_none")]
    pub network_statuses_nin: Option<String>,
    /// Included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// Not included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domainsNin", skip_serializing_if = "Option::is_none")]
    pub domains_nin: Option<String>,
    /// Disk encryption status. Optional.
    #[serde(rename = "encryptedApplications", skip_serializing_if = "Option::is_none")]
    pub encrypted_applications: Option<String>,
    /// Total memory range (GB, inclusive). Example: "4-8". Optional.
    #[serde(rename = "totalMemory__between", skip_serializing_if = "Option::is_none")]
    pub total_memory__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "coreCount__between", skip_serializing_if = "Option::is_none")]
    pub core_count__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "cpuCount__between", skip_serializing_if = "Option::is_none")]
    pub cpu_count__between: Option<String>,
    /// Included pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeeded", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed: Option<String>,
    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissions", skip_serializing_if = "Option::is_none")]
    pub missing_permissions: Option<String>,
    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeededNin", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed_nin: Option<String>,
    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissionsNin", skip_serializing_if = "Option::is_none")]
    pub missing_permissions_nin: Option<String>,
    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com". Optional.
    #[serde(rename = "adQuery", skip_serializing_if = "Option::is_none")]
    pub ad_query: Option<String>,
    /// Agent has a local configuration set. Optional.
    #[serde(rename = "hasLocalConfiguration", skip_serializing_if = "Option::is_none")]
    pub has_local_configuration: Option<bool>,
    /// Migration status in. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatuses", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses: Option<String>,
    /// Migration status nin. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatusesNin", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses_nin: Option<String>,
    /// Apps vulnerability status in. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatuses", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses: Option<String>,
    /// Apps vulnerability status nin. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatusesNin", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses_nin: Option<String>,
    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIds", skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<String>,
    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIdsNin", skip_serializing_if = "Option::is_none")]
    pub location_ids_nin: Option<String>,
    /// Include only Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypes", skip_serializing_if = "Option::is_none")]
    pub installer_types: Option<String>,
    /// Exclude Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypesNin", skip_serializing_if = "Option::is_none")]
    pub installer_types_nin: Option<String>,
    /// Agent operational state. Optional.
    #[serde(rename = "operationalStates", skip_serializing_if = "Option::is_none")]
    pub operational_states: Option<String>,
    /// Do not include these Agent operational states. Optional.
    #[serde(rename = "operationalStatesNin", skip_serializing_if = "Option::is_none")]
    pub operational_states_nin: Option<String>,
    /// Agent remote profiling state. Optional.
    #[serde(rename = "remoteProfilingStates", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states: Option<String>,
    /// Do not include these Agent remote profiling states. Optional.
    #[serde(rename = "remoteProfilingStatesNin", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states_nin: Option<String>,
    /// Status of Network Discovery. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatuses", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses: Option<String>,
    /// Do not include these Network Scanner Statuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatusesNin", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses_nin: Option<String>,
    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatus", skip_serializing_if = "Option::is_none")]
    pub ranger_status: Option<String>,
    /// Has at least one threat with at least one mitigation action pending reboot to succeed. Optional.
    #[serde(rename = "threatRebootRequired", skip_serializing_if = "Option::is_none")]
    pub threat_reboot_required: Option<String>,
    /// The agents supports Network Quarantine Control and its enabled for the agent's group. Optional.
    #[serde(rename = "networkQuarantineEnabled", skip_serializing_if = "Option::is_none")]
    pub network_quarantine_enabled: Option<String>,
    /// The agents supports Firewall Control and it is enabled for the agent's group. Optional.
    #[serde(rename = "firewallEnabled", skip_serializing_if = "Option::is_none")]
    pub firewall_enabled: Option<String>,
    /// The agents supports Location Awareness and it is enabled for the agent's group. Optional.
    #[serde(rename = "locationEnabled", skip_serializing_if = "Option::is_none")]
    pub location_enabled: Option<String>,
    /// Agents from which cloud provider. Optional.
    #[serde(rename = "cloudProvider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// Exclude Agents from these cloud provider. Optional.
    #[serde(rename = "cloudProviderNin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_nin: Option<String>,
    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}". Optional.
    #[serde(rename = "tagsData", skip_serializing_if = "Option::is_none")]
    pub tags_data: Option<String>,
    /// Include only Agents that have any tags assigned if True, or none if False. Optional.
    #[serde(rename = "hasTags", skip_serializing_if = "Option::is_none")]
    pub has_tags: Option<bool>,
    /// The agents that are ADConnectors if True, or not if False. Optional.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The agents has Hyper Automate PNA enabled if True, or not if False. Optional.
    #[serde(rename = "isHyperAutomate", skip_serializing_if = "Option::is_none")]
    pub is_hyper_automate: Option<String>,
    /// Included agent active protections. Example: "edr,idr". Optional.
    ///
    /// Allowed values: edr, idr.
    #[serde(rename = "activeProtection", skip_serializing_if = "Option::is_none")]
    pub active_protection: Option<String>,
    /// Containerized workload counts. Optional.
    #[serde(rename = "containerizedWorkloadCounts", skip_serializing_if = "Option::is_none")]
    pub containerized_workload_counts: Option<String>,
    /// Indicates whether the agent protects containerized workload at the moment. Optional.
    #[serde(rename = "hasContainerizedWorkload", skip_serializing_if = "Option::is_none")]
    pub has_containerized_workload: Option<String>,
    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "pacFileUsage", skip_serializing_if = "Option::is_none")]
    pub pac_file_usage: Option<String>,
    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethod", skip_serializing_if = "Option::is_none")]
    pub proxy_method: Option<String>,
    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethodNin", skip_serializing_if = "Option::is_none")]
    pub proxy_method_nin: Option<String>,
    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isMgmtProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_mgmt_proxy_enabled: Option<String>,
    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isEventSearchProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_event_search_proxy_enabled: Option<String>,
    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0". Optional.
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip__contains: Option<String>,
    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN". Optional.
    #[serde(rename = "computerName__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name__contains: Option<String>,
    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0". Optional.
    #[serde(rename = "networkInterfaceInet__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_inet__contains: Option<String>,
    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfacePhysical__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_physical__contains: Option<String>,
    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfaceGatewayMacAddress__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_gateway_mac_address__contains: Option<String>,
    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1". Optional.
    #[serde(rename = "lastLoggedInUserName__contains", skip_serializing_if = "Option::is_none")]
    pub last_logged_in_user_name__contains: Option<String>,
    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1". Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_query__contains: Option<String>,
    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_name__contains: Option<String>,
    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John". Optional.
    #[serde(rename = "adUserQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_query__contains: Option<String>,
    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_name__contains: Option<String>,
    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows". Optional.
    #[serde(rename = "adComputerQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_query__contains: Option<String>,
    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055". Optional.
    #[serde(rename = "uuid__contains", skip_serializing_if = "Option::is_none")]
    pub uuid__contains: Option<String>,
    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine". Optional.
    #[serde(rename = "externalId__contains", skip_serializing_if = "Option::is_none")]
    pub external_id__contains: Option<String>,
    /// Free-text filter by aws role(supports multiple values). Optional.
    #[serde(rename = "awsRole__contains", skip_serializing_if = "Option::is_none")]
    pub aws_role__contains: Option<String>,
    /// Free-text filter by aws securityGroups(supports multiple values). Optional.
    #[serde(rename = "awsSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub aws_security_groups__contains: Option<String>,
    /// Free-text filter by aws subnet ids (supports multiple values). Optional.
    #[serde(rename = "awsSubnetIds__contains", skip_serializing_if = "Option::is_none")]
    pub aws_subnet_ids__contains: Option<String>,
    /// Free-text filter by agent namespace (supports multiple values). Optional.
    #[serde(rename = "agentNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub agent_namespace__contains: Option<String>,
    /// Free-text filter by agent pod name (supports multiple values). Optional.
    #[serde(rename = "agentPodName__contains", skip_serializing_if = "Option::is_none")]
    pub agent_pod_name__contains: Option<String>,
    /// Free-text filter by azure resource group(supports multiple values). Optional.
    #[serde(rename = "azureResourceGroup__contains", skip_serializing_if = "Option::is_none")]
    pub azure_resource_group__contains: Option<String>,
    /// Free-text filter by cloud account (supports multiple values). Optional.
    #[serde(rename = "cloudAccount__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_account__contains: Option<String>,
    /// Free-text filter by cloud image (supports multiple values). Optional.
    #[serde(rename = "cloudImage__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_image__contains: Option<String>,
    /// Free-text filter by cloud instance id(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_id__contains: Option<String>,
    /// Free-text filter by cloud instance size(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceSize__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_size__contains: Option<String>,
    /// Free-text filter by cloud location (supports multiple values). Optional.
    #[serde(rename = "cloudLocation__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_location__contains: Option<String>,
    /// Free-text filter by cloud network (supports multiple values). Optional.
    #[serde(rename = "cloudNetwork__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_network__contains: Option<String>,
    /// Free-text filter by cloud tags (supports multiple values). Optional.
    #[serde(rename = "cloudTags__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags__contains: Option<String>,
    /// Free-text filter by cluster name (supports multiple values). Optional.
    #[serde(rename = "clusterName__contains", skip_serializing_if = "Option::is_none")]
    pub cluster_name__contains: Option<String>,
    /// Free-text filter by gcp service account (supports multiple values). Optional.
    #[serde(rename = "gcpServiceAccount__contains", skip_serializing_if = "Option::is_none")]
    pub gcp_service_account__contains: Option<String>,
    /// Free-text filter by K8s node labels (supports multiple values). Optional.
    #[serde(rename = "k8sNodeLabels__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_labels__contains: Option<String>,
    /// Free-text filter by K8s node name (supports multiple values). Optional.
    #[serde(rename = "k8sNodeName__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_name__contains: Option<String>,
    /// Free-text filter by K8s type(supports multiple values). Optional.
    #[serde(rename = "k8sType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_type__contains: Option<String>,
    /// Free-text filter by K8s version (supports multiple values). Optional.
    #[serde(rename = "k8sVersion__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_version__contains: Option<String>,
    /// Free-text filter by live update ID (supports multiple values). Optional.
    #[serde(rename = "liveUpdateId__contains", skip_serializing_if = "Option::is_none")]
    pub live_update_id__contains: Option<String>,
    /// Free-text filter by Serial Number (supports multiple values). Optional.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number__contains: Option<String>,
    /// Free-text filter by Entra ID (supports multiple values). Optional.
    #[serde(rename = "entraId__contains", skip_serializing_if = "Option::is_none")]
    pub entra_id__contains: Option<String>,
    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD". Optional.
    #[serde(rename = "cpuId__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_id__contains: Option<String>,
    /// Free-text filter by ECS type. Optional.
    #[serde(rename = "ecsType__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_type__contains: Option<String>,
    /// Free-text filter by ECS version. Optional.
    #[serde(rename = "ecsVersion__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_version__contains: Option<String>,
    /// Free-text filter by ECS cluster name. Optional.
    #[serde(rename = "ecsClusterName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_name__contains: Option<String>,
    /// Free-text filter by ECS task arn. Optional.
    #[serde(rename = "ecsTaskArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_arn__contains: Option<String>,
    /// Free-text filter by ECS task availability zone. Optional.
    #[serde(rename = "ecsTaskAvailabilityZone__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_availability_zone__contains: Option<String>,
    /// Free-text filter by ECS service name. Optional.
    #[serde(rename = "ecsServiceName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_name__contains: Option<String>,
    /// Free-text filter by ECS service arn. Optional.
    #[serde(rename = "ecsServiceArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_arn__contains: Option<String>,
    /// Free-text filter by ECS task definition family. Optional.
    #[serde(rename = "ecsTaskDefinitionFamily__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_family__contains: Option<String>,
    /// Free-text filter by ECS task definition revision. Optional.
    #[serde(rename = "ecsTaskDefinitionRevision__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_revision__contains: Option<String>,
    /// Free-text filter by ECS task definition arn. Optional.
    #[serde(rename = "ecsTaskDefinitionArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_arn__contains: Option<String>,
    /// Include active, decommissioned or both. Example: "True,False". Optional.
    #[serde(rename = "isDecommissioned", skip_serializing_if = "Option::is_none")]
    pub is_decommissioned: Option<String>,
    /// Include installed, uninstalled or both. Example: "True,False". Optional.
    #[serde(rename = "isUninstalled", skip_serializing_if = "Option::is_none")]
    pub is_uninstalled: Option<String>,
    /// Free-text filter by computer name or uuid (supports multiple values). Optional.
    #[serde(rename = "computerNameOrUuid__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name_or_uuid__contains: Option<String>,
    /// Included Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "idsNin", skip_serializing_if = "Option::is_none")]
    pub ids_nin: Option<String>,
    /// Include all Agents matching this saved filter. Example: "225494730938493804". Optional.
    #[serde(rename = "filterId", skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<String>,
    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gte: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lt: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lte: Option<String>,
    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gt: Option<String>,
    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "decommissionedAt__between", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__between: Option<String>,
    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at__lt: Option<String>,
    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at__lte: Option<String>,
    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at__gt: Option<String>,
    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at__gte: Option<String>,
    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at__between: Option<String>,
    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at__lt: Option<String>,
    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at__lte: Option<String>,
    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at__gt: Option<String>,
    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at__gte: Option<String>,
    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at__between: Option<String>,
    /// Match computer name partially (substring). Example: "Lab1". Optional.
    #[serde(rename = "computerName__like", skip_serializing_if = "Option::is_none")]
    pub computer_name__like: Option<String>,
    /// Computer name. Example: "My Office Desktop". Optional.
    #[serde(rename = "computerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// Agents versions less than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lt", skip_serializing_if = "Option::is_none")]
    pub agent_version__lt: Option<String>,
    /// Agents versions less than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lte", skip_serializing_if = "Option::is_none")]
    pub agent_version__lte: Option<String>,
    /// Agents versions greater than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gt", skip_serializing_if = "Option::is_none")]
    pub agent_version__gt: Option<String>,
    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gte", skip_serializing_if = "Option::is_none")]
    pub agent_version__gte: Option<String>,
    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144". Optional.
    #[serde(rename = "agentVersion__between", skip_serializing_if = "Option::is_none")]
    pub agent_version__between: Option<String>,
    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2". Optional.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22". Optional.
    #[serde(rename = "uuids", skip_serializing_if = "Option::is_none")]
    pub uuids: Option<String>,
    /// Scan status. Example: "none". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatus", skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// Include only Agents that have threats with this mitigation status. Example: "mitigated". Optional.
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    #[serde(rename = "threatMitigationStatus", skip_serializing_if = "Option::is_none")]
    pub threat_mitigation_status: Option<String>,
    /// Include only Agents with at least one resolved threat. Optional.
    #[serde(rename = "threatResolved", skip_serializing_if = "Option::is_none")]
    pub threat_resolved: Option<bool>,
    /// Include only Agents with at least one hidden threat. Optional.
    #[serde(rename = "threatHidden", skip_serializing_if = "Option::is_none")]
    pub threat_hidden: Option<bool>,
    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94". Optional.
    #[serde(rename = "threatContentHash", skip_serializing_if = "Option::is_none")]
    pub threat_content_hash: Option<String>,
    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lt: Option<String>,
    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lte: Option<String>,
    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gt: Option<String>,
    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gte: Option<String>,
    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "threatCreatedAt__between", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__between: Option<String>,
    /// Include Agents with this amount of active threats. Example: "3". Optional.
    #[serde(rename = "activeThreats", skip_serializing_if = "Option::is_none")]
    pub active_threats: Option<i64>,
    /// Include Agents with at least this amount of active threats. Example: "5". Optional.
    #[serde(rename = "activeThreats__gt", skip_serializing_if = "Option::is_none")]
    pub active_threats__gt: Option<i64>,
    /// Agent mitigation mode policy. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationMode", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode: Option<String>,
    /// Mitigation mode policy for suspicious activity. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationModeSuspicious", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode_suspicious: Option<String>,
    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lt", skip_serializing_if = "Option::is_none")]
    pub registered_at__lt: Option<String>,
    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lte", skip_serializing_if = "Option::is_none")]
    pub registered_at__lte: Option<String>,
    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gt", skip_serializing_if = "Option::is_none")]
    pub registered_at__gt: Option<String>,
    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gte", skip_serializing_if = "Option::is_none")]
    pub registered_at__gte: Option<String>,
    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lt: Option<String>,
    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lte: Option<String>,
    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gt: Option<String>,
    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gte: Option<String>,
    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lt: Option<String>,
    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lte: Option<String>,
    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gt: Option<String>,
    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gte: Option<String>,
    /// CPU cores (less than). Optional.
    #[serde(rename = "coreCount__lt", skip_serializing_if = "Option::is_none")]
    pub core_count__lt: Option<i64>,
    /// CPU cores (less than or equal). Optional.
    #[serde(rename = "coreCount__lte", skip_serializing_if = "Option::is_none")]
    pub core_count__lte: Option<i64>,
    /// CPU cores (more than). Optional.
    #[serde(rename = "coreCount__gt", skip_serializing_if = "Option::is_none")]
    pub core_count__gt: Option<i64>,
    /// CPU cores (more than or equal). Optional.
    #[serde(rename = "coreCount__gte", skip_serializing_if = "Option::is_none")]
    pub core_count__gte: Option<i64>,
    /// Number of CPUs (less than). Optional.
    #[serde(rename = "cpuCount__lt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lt: Option<i64>,
    /// Number of CPUs (less than or equal). Optional.
    #[serde(rename = "cpuCount__lte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lte: Option<i64>,
    /// Number of CPUs (more than). Optional.
    #[serde(rename = "cpuCount__gt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gt: Option<i64>,
    /// Number of CPUs (more than or equal). Optional.
    #[serde(rename = "cpuCount__gte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gte: Option<i64>,
    /// Memory size (MB, less than). Optional.
    #[serde(rename = "totalMemory__lt", skip_serializing_if = "Option::is_none")]
    pub total_memory__lt: Option<i64>,
    /// Memory size (MB, less than or equal). Optional.
    #[serde(rename = "totalMemory__lte", skip_serializing_if = "Option::is_none")]
    pub total_memory__lte: Option<i64>,
    /// Memory size (MB, more than). Optional.
    #[serde(rename = "totalMemory__gt", skip_serializing_if = "Option::is_none")]
    pub total_memory__gt: Option<i64>,
    /// Memory size (MB, more than or equal). Optional.
    #[serde(rename = "totalMemory__gte", skip_serializing_if = "Option::is_none")]
    pub total_memory__gte: Option<i64>,
    /// Migration status. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "migrationStatus", skip_serializing_if = "Option::is_none")]
    pub migration_status: Option<String>,
    /// Gateway ip. Example: "192.168.0.1". Optional.
    #[serde(rename = "gatewayIp", skip_serializing_if = "Option::is_none")]
    pub gateway_ip: Option<String>,
    /// The ID of the CSV file to filter by. Example: "225494730938493804". Optional.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<String>,
    /// Supported Remote Script Orchestration level. Example: "none". Optional.
    ///
    /// Allowed values: none, pro, ars.
    #[serde(rename = "rsoLevel", skip_serializing_if = "Option::is_none")]
    pub rso_level: Option<String>,
    /// Include only agents that has Remote Ops Forensicsfeature supported. Optional.
    #[serde(rename = "remoteOpsForensicsSupported", skip_serializing_if = "Option::is_none")]
    pub remote_ops_forensics_supported: Option<bool>,
    /// Agents os revision than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__gte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__gte: Option<i64>,
    /// Agents os revision lower than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__lte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__lte: Option<i64>,
    /// A list of included rso_levels. Example: "pro,ars". Optional.
    #[serde(rename = "rsoLevels", skip_serializing_if = "Option::is_none")]
    pub rso_levels: Option<String>,
}

impl AgentsExportLightQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }

    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_group_ids = Some(join_csv(vals));
        self
    }

    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_site_ids = Some(join_csv(vals));
        self
    }

    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn registered_at__between(mut self, v: impl Into<String>) -> Self {
        self.registered_at__between = Some(v.into());
        self
    }

    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_active_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__between = Some(v.into());
        self
    }

    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_successful_scan_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__between = Some(v.into());
        self
    }

    /// Include only active Agents.
    pub fn is_active<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_active = Some(join_csv(vals));
        self
    }

    /// Include only Agents with pending uninstall requests.
    pub fn is_pending_uninstall<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_pending_uninstall = Some(join_csv(vals));
        self
    }

    /// Include only Agents with at least one active threat.
    pub fn infected(mut self, b: bool) -> Self {
        self.infected = Some(b);
        self
    }

    /// Include only Agents with updated software.
    pub fn is_up_to_date<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_up_to_date = Some(join_csv(vals));
        self
    }

    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux".
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }

    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions = Some(join_csv(vals));
        self
    }

    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions_nin = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions_nin = Some(join_csv(vals));
        self
    }

    /// OS architecture. Example: "32 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arch(mut self, v: impl Into<String>) -> Self {
        self.os_arch = Some(v.into());
        self
    }

    /// OS architectures to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches = Some(join_csv(vals));
        self
    }

    /// OS architectures not to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches_nin = Some(join_csv(vals));
        self
    }

    /// Included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(vals));
        self
    }

    /// Not included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types_nin = Some(join_csv(vals));
        self
    }

    /// Included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses = Some(join_csv(vals));
        self
    }

    /// Not included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types = Some(join_csv(vals));
        self
    }

    /// Not included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types_nin = Some(join_csv(vals));
        self
    }

    /// Included storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types = Some(join_csv(vals));
        self
    }

    /// Excluded storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types_nin = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains = Some(join_csv(vals));
        self
    }

    /// Not included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains_nin = Some(join_csv(vals));
        self
    }

    /// Disk encryption status.
    pub fn encrypted_applications<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encrypted_applications = Some(join_csv(vals));
        self
    }

    /// Total memory range (GB, inclusive). Example: "4-8".
    pub fn total_memory__between(mut self, v: impl Into<String>) -> Self {
        self.total_memory__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn core_count__between(mut self, v: impl Into<String>) -> Self {
        self.core_count__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn cpu_count__between(mut self, v: impl Into<String>) -> Self {
        self.cpu_count__between = Some(v.into());
        self
    }

    /// Included pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed = Some(join_csv(vals));
        self
    }

    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions = Some(join_csv(vals));
        self
    }

    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed_nin = Some(join_csv(vals));
        self
    }

    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions_nin = Some(join_csv(vals));
        self
    }

    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com".
    pub fn ad_query(mut self, v: impl Into<String>) -> Self {
        self.ad_query = Some(v.into());
        self
    }

    /// Agent has a local configuration set.
    pub fn has_local_configuration(mut self, b: bool) -> Self {
        self.has_local_configuration = Some(b);
        self
    }

    /// Migration status in. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses = Some(join_csv(vals));
        self
    }

    /// Migration status nin. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status in. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status nin. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids = Some(join_csv(vals));
        self
    }

    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types = Some(join_csv(vals));
        self
    }

    /// Exclude Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types_nin = Some(join_csv(vals));
        self
    }

    /// Agent operational state.
    pub fn operational_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent operational states.
    pub fn operational_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states_nin = Some(join_csv(vals));
        self
    }

    /// Agent remote profiling state.
    pub fn remote_profiling_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent remote profiling states.
    pub fn remote_profiling_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states_nin = Some(join_csv(vals));
        self
    }

    /// Status of Network Discovery. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses = Some(join_csv(vals));
        self
    }

    /// Do not include these Network Scanner Statuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses_nin = Some(join_csv(vals));
        self
    }

    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_status(mut self, v: impl Into<String>) -> Self {
        self.ranger_status = Some(v.into());
        self
    }

    /// Has at least one threat with at least one mitigation action pending reboot to succeed.
    pub fn threat_reboot_required<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_reboot_required = Some(join_csv(vals));
        self
    }

    /// The agents supports Network Quarantine Control and its enabled for the agent's group.
    pub fn network_quarantine_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_quarantine_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Firewall Control and it is enabled for the agent's group.
    pub fn firewall_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.firewall_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Location Awareness and it is enabled for the agent's group.
    pub fn location_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_enabled = Some(join_csv(vals));
        self
    }

    /// Agents from which cloud provider.
    pub fn cloud_provider<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider = Some(join_csv(vals));
        self
    }

    /// Exclude Agents from these cloud provider.
    pub fn cloud_provider_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_nin = Some(join_csv(vals));
        self
    }

    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}".
    pub fn tags_data(mut self, v: impl Into<String>) -> Self {
        self.tags_data = Some(v.into());
        self
    }

    /// Include only Agents that have any tags assigned if True, or none if False.
    pub fn has_tags(mut self, b: bool) -> Self {
        self.has_tags = Some(b);
        self
    }

    /// The agents that are ADConnectors if True, or not if False.
    pub fn is_ad_connector<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(join_csv(vals));
        self
    }

    /// The agents has Hyper Automate PNA enabled if True, or not if False.
    pub fn is_hyper_automate<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_hyper_automate = Some(join_csv(vals));
        self
    }

    /// Included agent active protections. Example: "edr,idr".
    ///
    /// Allowed values: edr, idr.
    pub fn active_protection<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_protection = Some(join_csv(vals));
        self
    }

    /// Containerized workload counts.
    pub fn containerized_workload_counts(mut self, v: impl Into<String>) -> Self {
        self.containerized_workload_counts = Some(v.into());
        self
    }

    /// Indicates whether the agent protects containerized workload at the moment.
    pub fn has_containerized_workload<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.has_containerized_workload = Some(join_csv(vals));
        self
    }

    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported).
    pub fn pac_file_usage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.pac_file_usage = Some(join_csv(vals));
        self
    }

    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method = Some(join_csv(vals));
        self
    }

    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported).
    pub fn is_mgmt_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_mgmt_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported).
    pub fn is_event_search_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_event_search_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0".
    pub fn external_ip__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN".
    pub fn computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0".
    pub fn network_interface_inet__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_inet__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_physical__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_physical__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_gateway_mac_address__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_gateway_mac_address__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1".
    pub fn last_logged_in_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.last_logged_in_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1".
    pub fn os_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John".
    pub fn ad_user_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows".
    pub fn ad_computer_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055".
    pub fn uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuid__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine".
    pub fn external_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws role(supports multiple values).
    pub fn aws_role__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_role__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws securityGroups(supports multiple values).
    pub fn aws_security_groups__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_security_groups__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws subnet ids (supports multiple values).
    pub fn aws_subnet_ids__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_subnet_ids__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent namespace (supports multiple values).
    pub fn agent_namespace__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_namespace__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent pod name (supports multiple values).
    pub fn agent_pod_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pod_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by azure resource group(supports multiple values).
    pub fn azure_resource_group__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.azure_resource_group__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud account (supports multiple values).
    pub fn cloud_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud image (supports multiple values).
    pub fn cloud_image__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_image__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance id(supports multiple values).
    pub fn cloud_instance_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance size(supports multiple values).
    pub fn cloud_instance_size__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_size__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud location (supports multiple values).
    pub fn cloud_location__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_location__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud network (supports multiple values).
    pub fn cloud_network__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_network__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud tags (supports multiple values).
    pub fn cloud_tags__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cluster name (supports multiple values).
    pub fn cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by gcp service account (supports multiple values).
    pub fn gcp_service_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gcp_service_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node labels (supports multiple values).
    pub fn k8s_node_labels__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_labels__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node name (supports multiple values).
    pub fn k8s_node_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s type(supports multiple values).
    pub fn k8s_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s version (supports multiple values).
    pub fn k8s_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by live update ID (supports multiple values).
    pub fn live_update_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.live_update_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Serial Number (supports multiple values).
    pub fn serial_number__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Entra ID (supports multiple values).
    pub fn entra_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.entra_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD".
    pub fn cpu_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS type.
    pub fn ecs_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS version.
    pub fn ecs_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS cluster name.
    pub fn ecs_cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task arn.
    pub fn ecs_task_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task availability zone.
    pub fn ecs_task_availability_zone__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_availability_zone__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service name.
    pub fn ecs_service_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service arn.
    pub fn ecs_service_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition family.
    pub fn ecs_task_definition_family__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_family__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition revision.
    pub fn ecs_task_definition_revision__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_revision__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition arn.
    pub fn ecs_task_definition_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_arn__contains = Some(join_csv(vals));
        self
    }

    /// Include active, decommissioned or both. Example: "True,False".
    pub fn is_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_decommissioned = Some(join_csv(vals));
        self
    }

    /// Include installed, uninstalled or both. Example: "True,False".
    pub fn is_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_uninstalled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name or uuid (supports multiple values).
    pub fn computer_name_or_uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name_or_uuid__contains = Some(join_csv(vals));
        self
    }

    /// Included Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(vals));
        self
    }

    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids_nin = Some(join_csv(vals));
        self
    }

    /// Include all Agents matching this saved filter. Example: "225494730938493804".
    pub fn filter_id(mut self, v: impl Into<String>) -> Self {
        self.filter_id = Some(v.into());
        self
    }

    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gte = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lt = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lte = Some(v.into());
        self
    }

    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gt = Some(v.into());
        self
    }

    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn decommissioned_at__between(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__between = Some(v.into());
        self
    }

    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.created_at__lt = Some(v.into());
        self
    }

    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.created_at__lte = Some(v.into());
        self
    }

    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.created_at__gt = Some(v.into());
        self
    }

    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.created_at__gte = Some(v.into());
        self
    }

    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn created_at__between(mut self, v: impl Into<String>) -> Self {
        self.created_at__between = Some(v.into());
        self
    }

    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lt = Some(v.into());
        self
    }

    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lte = Some(v.into());
        self
    }

    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gt = Some(v.into());
        self
    }

    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gte = Some(v.into());
        self
    }

    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.updated_at__between = Some(v.into());
        self
    }

    /// Match computer name partially (substring). Example: "Lab1".
    pub fn computer_name__like(mut self, v: impl Into<String>) -> Self {
        self.computer_name__like = Some(v.into());
        self
    }

    /// Computer name. Example: "My Office Desktop".
    pub fn computer_name(mut self, v: impl Into<String>) -> Self {
        self.computer_name = Some(v.into());
        self
    }

    /// Agents versions less than given version. Example: "2.5.1.1320".
    pub fn agent_version__lt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lt = Some(v.into());
        self
    }

    /// Agents versions less than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__lte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lte = Some(v.into());
        self
    }

    /// Agents versions greater than given version. Example: "2.5.1.1320".
    pub fn agent_version__gt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gt = Some(v.into());
        self
    }

    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__gte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gte = Some(v.into());
        self
    }

    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144".
    pub fn agent_version__between(mut self, v: impl Into<String>) -> Self {
        self.agent_version__between = Some(v.into());
        self
    }

    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2".
    pub fn uuid(mut self, v: impl Into<String>) -> Self {
        self.uuid = Some(v.into());
        self
    }

    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22".
    pub fn uuids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuids = Some(join_csv(vals));
        self
    }

    /// Scan status. Example: "none".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_status(mut self, v: impl Into<String>) -> Self {
        self.scan_status = Some(v.into());
        self
    }

    /// Include only Agents that have threats with this mitigation status. Example: "mitigated".
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    pub fn threat_mitigation_status(mut self, v: impl Into<String>) -> Self {
        self.threat_mitigation_status = Some(v.into());
        self
    }

    /// Include only Agents with at least one resolved threat.
    pub fn threat_resolved(mut self, b: bool) -> Self {
        self.threat_resolved = Some(b);
        self
    }

    /// Include only Agents with at least one hidden threat.
    pub fn threat_hidden(mut self, b: bool) -> Self {
        self.threat_hidden = Some(b);
        self
    }

    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94".
    pub fn threat_content_hash(mut self, v: impl Into<String>) -> Self {
        self.threat_content_hash = Some(v.into());
        self
    }

    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lt = Some(v.into());
        self
    }

    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lte = Some(v.into());
        self
    }

    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gt = Some(v.into());
        self
    }

    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gte = Some(v.into());
        self
    }

    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn threat_created_at__between(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__between = Some(v.into());
        self
    }

    /// Include Agents with this amount of active threats. Example: "3".
    pub fn active_threats(mut self, n: i64) -> Self {
        self.active_threats = Some(n);
        self
    }

    /// Include Agents with at least this amount of active threats. Example: "5".
    pub fn active_threats__gt(mut self, n: i64) -> Self {
        self.active_threats__gt = Some(n);
        self
    }

    /// Agent mitigation mode policy. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode = Some(v.into());
        self
    }

    /// Mitigation mode policy for suspicious activity. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode_suspicious(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode_suspicious = Some(v.into());
        self
    }

    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lt = Some(v.into());
        self
    }

    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lte = Some(v.into());
        self
    }

    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gt = Some(v.into());
        self
    }

    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gte = Some(v.into());
        self
    }

    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lt = Some(v.into());
        self
    }

    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lte = Some(v.into());
        self
    }

    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gt = Some(v.into());
        self
    }

    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gte = Some(v.into());
        self
    }

    /// CPU cores (less than).
    pub fn core_count__lt(mut self, n: i64) -> Self {
        self.core_count__lt = Some(n);
        self
    }

    /// CPU cores (less than or equal).
    pub fn core_count__lte(mut self, n: i64) -> Self {
        self.core_count__lte = Some(n);
        self
    }

    /// CPU cores (more than).
    pub fn core_count__gt(mut self, n: i64) -> Self {
        self.core_count__gt = Some(n);
        self
    }

    /// CPU cores (more than or equal).
    pub fn core_count__gte(mut self, n: i64) -> Self {
        self.core_count__gte = Some(n);
        self
    }

    /// Number of CPUs (less than).
    pub fn cpu_count__lt(mut self, n: i64) -> Self {
        self.cpu_count__lt = Some(n);
        self
    }

    /// Number of CPUs (less than or equal).
    pub fn cpu_count__lte(mut self, n: i64) -> Self {
        self.cpu_count__lte = Some(n);
        self
    }

    /// Number of CPUs (more than).
    pub fn cpu_count__gt(mut self, n: i64) -> Self {
        self.cpu_count__gt = Some(n);
        self
    }

    /// Number of CPUs (more than or equal).
    pub fn cpu_count__gte(mut self, n: i64) -> Self {
        self.cpu_count__gte = Some(n);
        self
    }

    /// Memory size (MB, less than).
    pub fn total_memory__lt(mut self, n: i64) -> Self {
        self.total_memory__lt = Some(n);
        self
    }

    /// Memory size (MB, less than or equal).
    pub fn total_memory__lte(mut self, n: i64) -> Self {
        self.total_memory__lte = Some(n);
        self
    }

    /// Memory size (MB, more than).
    pub fn total_memory__gt(mut self, n: i64) -> Self {
        self.total_memory__gt = Some(n);
        self
    }

    /// Memory size (MB, more than or equal).
    pub fn total_memory__gte(mut self, n: i64) -> Self {
        self.total_memory__gte = Some(n);
        self
    }

    /// Migration status. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn migration_status(mut self, v: impl Into<String>) -> Self {
        self.migration_status = Some(v.into());
        self
    }

    /// Gateway ip. Example: "192.168.0.1".
    pub fn gateway_ip(mut self, v: impl Into<String>) -> Self {
        self.gateway_ip = Some(v.into());
        self
    }

    /// The ID of the CSV file to filter by. Example: "225494730938493804".
    pub fn csv_filter_id(mut self, v: impl Into<String>) -> Self {
        self.csv_filter_id = Some(v.into());
        self
    }

    /// Supported Remote Script Orchestration level. Example: "none".
    ///
    /// Allowed values: none, pro, ars.
    pub fn rso_level(mut self, v: impl Into<String>) -> Self {
        self.rso_level = Some(v.into());
        self
    }

    /// Include only agents that has Remote Ops Forensicsfeature supported.
    pub fn remote_ops_forensics_supported(mut self, b: bool) -> Self {
        self.remote_ops_forensics_supported = Some(b);
        self
    }

    /// Agents os revision than or equal to given version.
    pub fn windows_os_revision__gte(mut self, n: i64) -> Self {
        self.windows_os_revision__gte = Some(n);
        self
    }

    /// Agents os revision lower than or equal to given version.
    pub fn windows_os_revision__lte(mut self, n: i64) -> Self {
        self.windows_os_revision__lte = Some(n);
        self
    }

    /// A list of included rso_levels. Example: "pro,ars".
    pub fn rso_levels<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.rso_levels = Some(join_csv(vals));
        self
    }
}

/// Query params for `GET /web/api/v2.1/export/agents-passphrases` (Export Agents - Passphrases).
///
/// Array params are serialized comma-joined, as the API expects.
#[derive(Debug, Default, Serialize)]
pub struct AgentsExportPassphrasesQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "siteIds", skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "accountIds", skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of network groups. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "groupIds", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredGroupIds", skip_serializing_if = "Option::is_none")]
    pub filtered_group_ids: Option<String>,
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "filteredSiteIds", skip_serializing_if = "Option::is_none")]
    pub filtered_site_ids: Option<String>,
    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "registeredAt__between", skip_serializing_if = "Option::is_none")]
    pub registered_at__between: Option<String>,
    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastActiveDate__between", skip_serializing_if = "Option::is_none")]
    pub last_active_date__between: Option<String>,
    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "lastSuccessfulScanDate__between", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__between: Option<String>,
    /// Include only active Agents. Optional.
    #[serde(rename = "isActive", skip_serializing_if = "Option::is_none")]
    pub is_active: Option<String>,
    /// Include only Agents with pending uninstall requests. Optional.
    #[serde(rename = "isPendingUninstall", skip_serializing_if = "Option::is_none")]
    pub is_pending_uninstall: Option<String>,
    /// Include only Agents with at least one active threat. Optional.
    #[serde(rename = "infected", skip_serializing_if = "Option::is_none")]
    pub infected: Option<bool>,
    /// Include only Agents with updated software. Optional.
    #[serde(rename = "isUpToDate", skip_serializing_if = "Option::is_none")]
    pub is_up_to_date: Option<String>,
    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux". Optional.
    #[serde(rename = "query", skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersions", skip_serializing_if = "Option::is_none")]
    pub agent_versions: Option<String>,
    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "agentVersionsNin", skip_serializing_if = "Option::is_none")]
    pub agent_versions_nin: Option<String>,
    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersions", skip_serializing_if = "Option::is_none")]
    pub ranger_versions: Option<String>,
    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144". Optional.
    #[serde(rename = "rangerVersionsNin", skip_serializing_if = "Option::is_none")]
    pub ranger_versions_nin: Option<String>,
    /// OS architecture. Example: "32 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArch", skip_serializing_if = "Option::is_none")]
    pub os_arch: Option<String>,
    /// OS architectures to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArches", skip_serializing_if = "Option::is_none")]
    pub os_arches: Option<String>,
    /// OS architectures not to include. Example: "32 bit,64 bit". Optional.
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    #[serde(rename = "osArchesNin", skip_serializing_if = "Option::is_none")]
    pub os_arches_nin: Option<String>,
    /// Included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypes", skip_serializing_if = "Option::is_none")]
    pub os_types: Option<String>,
    /// Not included OS types. Example: "linux". Optional.
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    #[serde(rename = "osTypesNin", skip_serializing_if = "Option::is_none")]
    pub os_types_nin: Option<String>,
    /// Included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatuses", skip_serializing_if = "Option::is_none")]
    pub scan_statuses: Option<String>,
    /// Not included scan statuses. Example: "started,aborted". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatusesNin", skip_serializing_if = "Option::is_none")]
    pub scan_statuses_nin: Option<String>,
    /// Included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypes", skip_serializing_if = "Option::is_none")]
    pub machine_types: Option<String>,
    /// Not included machine types. Example: "laptop,desktop". Optional.
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    #[serde(rename = "machineTypesNin", skip_serializing_if = "Option::is_none")]
    pub machine_types_nin: Option<String>,
    /// Included storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypes", skip_serializing_if = "Option::is_none")]
    pub storage_types: Option<String>,
    /// Excluded storage types. Example: "NetApp,Dell,S3". Optional.
    #[serde(rename = "storageTypesNin", skip_serializing_if = "Option::is_none")]
    pub storage_types_nin: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatuses", skip_serializing_if = "Option::is_none")]
    pub network_statuses: Option<String>,
    /// Included network statuses. Example: "connected,connecting". Optional.
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    #[serde(rename = "networkStatusesNin", skip_serializing_if = "Option::is_none")]
    pub network_statuses_nin: Option<String>,
    /// Included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domains", skip_serializing_if = "Option::is_none")]
    pub domains: Option<String>,
    /// Not included network domains. Example: "mybusiness.net,workgroup". Optional.
    #[serde(rename = "domainsNin", skip_serializing_if = "Option::is_none")]
    pub domains_nin: Option<String>,
    /// Disk encryption status. Optional.
    #[serde(rename = "encryptedApplications", skip_serializing_if = "Option::is_none")]
    pub encrypted_applications: Option<String>,
    /// Total memory range (GB, inclusive). Example: "4-8". Optional.
    #[serde(rename = "totalMemory__between", skip_serializing_if = "Option::is_none")]
    pub total_memory__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "coreCount__between", skip_serializing_if = "Option::is_none")]
    pub core_count__between: Option<String>,
    /// Possible number of CPU cores (inclusive). Example: "2-8". Optional.
    #[serde(rename = "cpuCount__between", skip_serializing_if = "Option::is_none")]
    pub cpu_count__between: Option<String>,
    /// Included pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeeded", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed: Option<String>,
    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissions", skip_serializing_if = "Option::is_none")]
    pub missing_permissions: Option<String>,
    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed". Optional.
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    #[serde(rename = "userActionsNeededNin", skip_serializing_if = "Option::is_none")]
    pub user_actions_needed_nin: Option<String>,
    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper". Optional.
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    #[serde(rename = "missingPermissionsNin", skip_serializing_if = "Option::is_none")]
    pub missing_permissions_nin: Option<String>,
    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com". Optional.
    #[serde(rename = "adQuery", skip_serializing_if = "Option::is_none")]
    pub ad_query: Option<String>,
    /// Agent has a local configuration set. Optional.
    #[serde(rename = "hasLocalConfiguration", skip_serializing_if = "Option::is_none")]
    pub has_local_configuration: Option<bool>,
    /// Migration status in. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatuses", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses: Option<String>,
    /// Migration status nin. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "consoleMigrationStatusesNin", skip_serializing_if = "Option::is_none")]
    pub console_migration_statuses_nin: Option<String>,
    /// Apps vulnerability status in. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatuses", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses: Option<String>,
    /// Apps vulnerability status nin. Example: "patch_required". Optional.
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    #[serde(rename = "appsVulnerabilityStatusesNin", skip_serializing_if = "Option::is_none")]
    pub apps_vulnerability_statuses_nin: Option<String>,
    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIds", skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<String>,
    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "locationIdsNin", skip_serializing_if = "Option::is_none")]
    pub location_ids_nin: Option<String>,
    /// Include only Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypes", skip_serializing_if = "Option::is_none")]
    pub installer_types: Option<String>,
    /// Exclude Agents installed with these package types. Example: ".msi". Optional.
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    #[serde(rename = "installerTypesNin", skip_serializing_if = "Option::is_none")]
    pub installer_types_nin: Option<String>,
    /// Agent operational state. Optional.
    #[serde(rename = "operationalStates", skip_serializing_if = "Option::is_none")]
    pub operational_states: Option<String>,
    /// Do not include these Agent operational states. Optional.
    #[serde(rename = "operationalStatesNin", skip_serializing_if = "Option::is_none")]
    pub operational_states_nin: Option<String>,
    /// Agent remote profiling state. Optional.
    #[serde(rename = "remoteProfilingStates", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states: Option<String>,
    /// Do not include these Agent remote profiling states. Optional.
    #[serde(rename = "remoteProfilingStatesNin", skip_serializing_if = "Option::is_none")]
    pub remote_profiling_states_nin: Option<String>,
    /// Status of Network Discovery. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatuses", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses: Option<String>,
    /// Do not include these Network Scanner Statuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatusesNin", skip_serializing_if = "Option::is_none")]
    pub ranger_statuses_nin: Option<String>,
    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable". Optional.
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    #[serde(rename = "rangerStatus", skip_serializing_if = "Option::is_none")]
    pub ranger_status: Option<String>,
    /// Has at least one threat with at least one mitigation action pending reboot to succeed. Optional.
    #[serde(rename = "threatRebootRequired", skip_serializing_if = "Option::is_none")]
    pub threat_reboot_required: Option<String>,
    /// The agents supports Network Quarantine Control and its enabled for the agent's group. Optional.
    #[serde(rename = "networkQuarantineEnabled", skip_serializing_if = "Option::is_none")]
    pub network_quarantine_enabled: Option<String>,
    /// The agents supports Firewall Control and it is enabled for the agent's group. Optional.
    #[serde(rename = "firewallEnabled", skip_serializing_if = "Option::is_none")]
    pub firewall_enabled: Option<String>,
    /// The agents supports Location Awareness and it is enabled for the agent's group. Optional.
    #[serde(rename = "locationEnabled", skip_serializing_if = "Option::is_none")]
    pub location_enabled: Option<String>,
    /// Agents from which cloud provider. Optional.
    #[serde(rename = "cloudProvider", skip_serializing_if = "Option::is_none")]
    pub cloud_provider: Option<String>,
    /// Exclude Agents from these cloud provider. Optional.
    #[serde(rename = "cloudProviderNin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_nin: Option<String>,
    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}". Optional.
    #[serde(rename = "tagsData", skip_serializing_if = "Option::is_none")]
    pub tags_data: Option<String>,
    /// Include only Agents that have any tags assigned if True, or none if False. Optional.
    #[serde(rename = "hasTags", skip_serializing_if = "Option::is_none")]
    pub has_tags: Option<bool>,
    /// The agents that are ADConnectors if True, or not if False. Optional.
    #[serde(rename = "isAdConnector", skip_serializing_if = "Option::is_none")]
    pub is_ad_connector: Option<String>,
    /// The agents has Hyper Automate PNA enabled if True, or not if False. Optional.
    #[serde(rename = "isHyperAutomate", skip_serializing_if = "Option::is_none")]
    pub is_hyper_automate: Option<String>,
    /// Included agent active protections. Example: "edr,idr". Optional.
    ///
    /// Allowed values: edr, idr.
    #[serde(rename = "activeProtection", skip_serializing_if = "Option::is_none")]
    pub active_protection: Option<String>,
    /// Containerized workload counts. Optional.
    #[serde(rename = "containerizedWorkloadCounts", skip_serializing_if = "Option::is_none")]
    pub containerized_workload_counts: Option<String>,
    /// Indicates whether the agent protects containerized workload at the moment. Optional.
    #[serde(rename = "hasContainerizedWorkload", skip_serializing_if = "Option::is_none")]
    pub has_containerized_workload: Option<String>,
    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "pacFileUsage", skip_serializing_if = "Option::is_none")]
    pub pac_file_usage: Option<String>,
    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethod", skip_serializing_if = "Option::is_none")]
    pub proxy_method: Option<String>,
    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom". Optional.
    #[serde(rename = "proxyMethodNin", skip_serializing_if = "Option::is_none")]
    pub proxy_method_nin: Option<String>,
    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isMgmtProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_mgmt_proxy_enabled: Option<String>,
    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported). Optional.
    #[serde(rename = "isEventSearchProxyEnabled", skip_serializing_if = "Option::is_none")]
    pub is_event_search_proxy_enabled: Option<String>,
    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0". Optional.
    #[serde(rename = "externalIp__contains", skip_serializing_if = "Option::is_none")]
    pub external_ip__contains: Option<String>,
    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN". Optional.
    #[serde(rename = "computerName__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name__contains: Option<String>,
    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0". Optional.
    #[serde(rename = "networkInterfaceInet__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_inet__contains: Option<String>,
    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfacePhysical__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_physical__contains: Option<String>,
    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:". Optional.
    #[serde(rename = "networkInterfaceGatewayMacAddress__contains", skip_serializing_if = "Option::is_none")]
    pub network_interface_gateway_mac_address__contains: Option<String>,
    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1". Optional.
    #[serde(rename = "lastLoggedInUserName__contains", skip_serializing_if = "Option::is_none")]
    pub last_logged_in_user_name__contains: Option<String>,
    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1". Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_query__contains: Option<String>,
    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_name__contains: Option<String>,
    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adUserMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John". Optional.
    #[serde(rename = "adUserQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_user_query__contains: Option<String>,
    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerName__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_name__contains: Option<String>,
    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone". Optional.
    #[serde(rename = "adComputerMember__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_member__contains: Option<String>,
    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows". Optional.
    #[serde(rename = "adComputerQuery__contains", skip_serializing_if = "Option::is_none")]
    pub ad_computer_query__contains: Option<String>,
    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055". Optional.
    #[serde(rename = "uuid__contains", skip_serializing_if = "Option::is_none")]
    pub uuid__contains: Option<String>,
    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine". Optional.
    #[serde(rename = "externalId__contains", skip_serializing_if = "Option::is_none")]
    pub external_id__contains: Option<String>,
    /// Free-text filter by aws role(supports multiple values). Optional.
    #[serde(rename = "awsRole__contains", skip_serializing_if = "Option::is_none")]
    pub aws_role__contains: Option<String>,
    /// Free-text filter by aws securityGroups(supports multiple values). Optional.
    #[serde(rename = "awsSecurityGroups__contains", skip_serializing_if = "Option::is_none")]
    pub aws_security_groups__contains: Option<String>,
    /// Free-text filter by aws subnet ids (supports multiple values). Optional.
    #[serde(rename = "awsSubnetIds__contains", skip_serializing_if = "Option::is_none")]
    pub aws_subnet_ids__contains: Option<String>,
    /// Free-text filter by agent namespace (supports multiple values). Optional.
    #[serde(rename = "agentNamespace__contains", skip_serializing_if = "Option::is_none")]
    pub agent_namespace__contains: Option<String>,
    /// Free-text filter by agent pod name (supports multiple values). Optional.
    #[serde(rename = "agentPodName__contains", skip_serializing_if = "Option::is_none")]
    pub agent_pod_name__contains: Option<String>,
    /// Free-text filter by azure resource group(supports multiple values). Optional.
    #[serde(rename = "azureResourceGroup__contains", skip_serializing_if = "Option::is_none")]
    pub azure_resource_group__contains: Option<String>,
    /// Free-text filter by cloud account (supports multiple values). Optional.
    #[serde(rename = "cloudAccount__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_account__contains: Option<String>,
    /// Free-text filter by cloud image (supports multiple values). Optional.
    #[serde(rename = "cloudImage__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_image__contains: Option<String>,
    /// Free-text filter by cloud instance id(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_id__contains: Option<String>,
    /// Free-text filter by cloud instance size(supports multiple values). Optional.
    #[serde(rename = "cloudInstanceSize__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_instance_size__contains: Option<String>,
    /// Free-text filter by cloud location (supports multiple values). Optional.
    #[serde(rename = "cloudLocation__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_location__contains: Option<String>,
    /// Free-text filter by cloud network (supports multiple values). Optional.
    #[serde(rename = "cloudNetwork__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_network__contains: Option<String>,
    /// Free-text filter by cloud tags (supports multiple values). Optional.
    #[serde(rename = "cloudTags__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags__contains: Option<String>,
    /// Free-text filter by cluster name (supports multiple values). Optional.
    #[serde(rename = "clusterName__contains", skip_serializing_if = "Option::is_none")]
    pub cluster_name__contains: Option<String>,
    /// Free-text filter by gcp service account (supports multiple values). Optional.
    #[serde(rename = "gcpServiceAccount__contains", skip_serializing_if = "Option::is_none")]
    pub gcp_service_account__contains: Option<String>,
    /// Free-text filter by K8s node labels (supports multiple values). Optional.
    #[serde(rename = "k8sNodeLabels__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_labels__contains: Option<String>,
    /// Free-text filter by K8s node name (supports multiple values). Optional.
    #[serde(rename = "k8sNodeName__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_node_name__contains: Option<String>,
    /// Free-text filter by K8s type(supports multiple values). Optional.
    #[serde(rename = "k8sType__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_type__contains: Option<String>,
    /// Free-text filter by K8s version (supports multiple values). Optional.
    #[serde(rename = "k8sVersion__contains", skip_serializing_if = "Option::is_none")]
    pub k8s_version__contains: Option<String>,
    /// Free-text filter by live update ID (supports multiple values). Optional.
    #[serde(rename = "liveUpdateId__contains", skip_serializing_if = "Option::is_none")]
    pub live_update_id__contains: Option<String>,
    /// Free-text filter by Serial Number (supports multiple values). Optional.
    #[serde(rename = "serialNumber__contains", skip_serializing_if = "Option::is_none")]
    pub serial_number__contains: Option<String>,
    /// Free-text filter by Entra ID (supports multiple values). Optional.
    #[serde(rename = "entraId__contains", skip_serializing_if = "Option::is_none")]
    pub entra_id__contains: Option<String>,
    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD". Optional.
    #[serde(rename = "cpuId__contains", skip_serializing_if = "Option::is_none")]
    pub cpu_id__contains: Option<String>,
    /// Free-text filter by ECS type. Optional.
    #[serde(rename = "ecsType__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_type__contains: Option<String>,
    /// Free-text filter by ECS version. Optional.
    #[serde(rename = "ecsVersion__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_version__contains: Option<String>,
    /// Free-text filter by ECS cluster name. Optional.
    #[serde(rename = "ecsClusterName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_name__contains: Option<String>,
    /// Free-text filter by ECS task arn. Optional.
    #[serde(rename = "ecsTaskArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_arn__contains: Option<String>,
    /// Free-text filter by ECS task availability zone. Optional.
    #[serde(rename = "ecsTaskAvailabilityZone__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_availability_zone__contains: Option<String>,
    /// Free-text filter by ECS service name. Optional.
    #[serde(rename = "ecsServiceName__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_name__contains: Option<String>,
    /// Free-text filter by ECS service arn. Optional.
    #[serde(rename = "ecsServiceArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_service_arn__contains: Option<String>,
    /// Free-text filter by ECS task definition family. Optional.
    #[serde(rename = "ecsTaskDefinitionFamily__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_family__contains: Option<String>,
    /// Free-text filter by ECS task definition revision. Optional.
    #[serde(rename = "ecsTaskDefinitionRevision__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_revision__contains: Option<String>,
    /// Free-text filter by ECS task definition arn. Optional.
    #[serde(rename = "ecsTaskDefinitionArn__contains", skip_serializing_if = "Option::is_none")]
    pub ecs_task_definition_arn__contains: Option<String>,
    /// Include active, decommissioned or both. Example: "True,False". Optional.
    #[serde(rename = "isDecommissioned", skip_serializing_if = "Option::is_none")]
    pub is_decommissioned: Option<String>,
    /// Include installed, uninstalled or both. Example: "True,False". Optional.
    #[serde(rename = "isUninstalled", skip_serializing_if = "Option::is_none")]
    pub is_uninstalled: Option<String>,
    /// Free-text filter by computer name or uuid (supports multiple values). Optional.
    #[serde(rename = "computerNameOrUuid__contains", skip_serializing_if = "Option::is_none")]
    pub computer_name_or_uuid__contains: Option<String>,
    /// Included Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "ids", skip_serializing_if = "Option::is_none")]
    pub ids: Option<String>,
    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915". Optional.
    #[serde(rename = "idsNin", skip_serializing_if = "Option::is_none")]
    pub ids_nin: Option<String>,
    /// Include all Agents matching this saved filter. Example: "225494730938493804". Optional.
    #[serde(rename = "filterId", skip_serializing_if = "Option::is_none")]
    pub filter_id: Option<String>,
    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gte: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lt: Option<String>,
    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__lte", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__lte: Option<String>,
    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "decommissionedAt__gt", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__gt: Option<String>,
    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "decommissionedAt__between", skip_serializing_if = "Option::is_none")]
    pub decommissioned_at__between: Option<String>,
    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lt", skip_serializing_if = "Option::is_none")]
    pub created_at__lt: Option<String>,
    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__lte", skip_serializing_if = "Option::is_none")]
    pub created_at__lte: Option<String>,
    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gt", skip_serializing_if = "Option::is_none")]
    pub created_at__gt: Option<String>,
    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "createdAt__gte", skip_serializing_if = "Option::is_none")]
    pub created_at__gte: Option<String>,
    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "createdAt__between", skip_serializing_if = "Option::is_none")]
    pub created_at__between: Option<String>,
    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub updated_at__lt: Option<String>,
    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub updated_at__lte: Option<String>,
    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub updated_at__gt: Option<String>,
    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "updatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub updated_at__gte: Option<String>,
    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130". Optional.
    #[serde(rename = "updatedAt__between", skip_serializing_if = "Option::is_none")]
    pub updated_at__between: Option<String>,
    /// Match computer name partially (substring). Example: "Lab1". Optional.
    #[serde(rename = "computerName__like", skip_serializing_if = "Option::is_none")]
    pub computer_name__like: Option<String>,
    /// Computer name. Example: "My Office Desktop". Optional.
    #[serde(rename = "computerName", skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    /// Agents versions less than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lt", skip_serializing_if = "Option::is_none")]
    pub agent_version__lt: Option<String>,
    /// Agents versions less than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__lte", skip_serializing_if = "Option::is_none")]
    pub agent_version__lte: Option<String>,
    /// Agents versions greater than given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gt", skip_serializing_if = "Option::is_none")]
    pub agent_version__gt: Option<String>,
    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320". Optional.
    #[serde(rename = "agentVersion__gte", skip_serializing_if = "Option::is_none")]
    pub agent_version__gte: Option<String>,
    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144". Optional.
    #[serde(rename = "agentVersion__between", skip_serializing_if = "Option::is_none")]
    pub agent_version__between: Option<String>,
    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2". Optional.
    #[serde(rename = "uuid", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22". Optional.
    #[serde(rename = "uuids", skip_serializing_if = "Option::is_none")]
    pub uuids: Option<String>,
    /// Scan status. Example: "none". Optional.
    ///
    /// Allowed values: none, started, aborted, finished.
    #[serde(rename = "scanStatus", skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// Include only Agents that have threats with this mitigation status. Example: "mitigated". Optional.
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    #[serde(rename = "threatMitigationStatus", skip_serializing_if = "Option::is_none")]
    pub threat_mitigation_status: Option<String>,
    /// Include only Agents with at least one resolved threat. Optional.
    #[serde(rename = "threatResolved", skip_serializing_if = "Option::is_none")]
    pub threat_resolved: Option<bool>,
    /// Include only Agents with at least one hidden threat. Optional.
    #[serde(rename = "threatHidden", skip_serializing_if = "Option::is_none")]
    pub threat_hidden: Option<bool>,
    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94". Optional.
    #[serde(rename = "threatContentHash", skip_serializing_if = "Option::is_none")]
    pub threat_content_hash: Option<String>,
    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lt: Option<String>,
    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__lte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__lte: Option<String>,
    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gt", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gt: Option<String>,
    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "threatCreatedAt__gte", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__gte: Option<String>,
    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999". Optional.
    #[serde(rename = "threatCreatedAt__between", skip_serializing_if = "Option::is_none")]
    pub threat_created_at__between: Option<String>,
    /// Include Agents with this amount of active threats. Example: "3". Optional.
    #[serde(rename = "activeThreats", skip_serializing_if = "Option::is_none")]
    pub active_threats: Option<i64>,
    /// Include Agents with at least this amount of active threats. Example: "5". Optional.
    #[serde(rename = "activeThreats__gt", skip_serializing_if = "Option::is_none")]
    pub active_threats__gt: Option<i64>,
    /// Agent mitigation mode policy. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationMode", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode: Option<String>,
    /// Mitigation mode policy for suspicious activity. Example: "detect". Optional.
    ///
    /// Allowed values: detect, protect.
    #[serde(rename = "mitigationModeSuspicious", skip_serializing_if = "Option::is_none")]
    pub mitigation_mode_suspicious: Option<String>,
    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lt", skip_serializing_if = "Option::is_none")]
    pub registered_at__lt: Option<String>,
    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__lte", skip_serializing_if = "Option::is_none")]
    pub registered_at__lte: Option<String>,
    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gt", skip_serializing_if = "Option::is_none")]
    pub registered_at__gt: Option<String>,
    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "registeredAt__gte", skip_serializing_if = "Option::is_none")]
    pub registered_at__gte: Option<String>,
    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lt: Option<String>,
    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__lte: Option<String>,
    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gt: Option<String>,
    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastActiveDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_active_date__gte: Option<String>,
    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lt: Option<String>,
    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__lte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__lte: Option<String>,
    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gt", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gt: Option<String>,
    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z". Optional.
    #[serde(rename = "lastSuccessfulScanDate__gte", skip_serializing_if = "Option::is_none")]
    pub last_successful_scan_date__gte: Option<String>,
    /// CPU cores (less than). Optional.
    #[serde(rename = "coreCount__lt", skip_serializing_if = "Option::is_none")]
    pub core_count__lt: Option<i64>,
    /// CPU cores (less than or equal). Optional.
    #[serde(rename = "coreCount__lte", skip_serializing_if = "Option::is_none")]
    pub core_count__lte: Option<i64>,
    /// CPU cores (more than). Optional.
    #[serde(rename = "coreCount__gt", skip_serializing_if = "Option::is_none")]
    pub core_count__gt: Option<i64>,
    /// CPU cores (more than or equal). Optional.
    #[serde(rename = "coreCount__gte", skip_serializing_if = "Option::is_none")]
    pub core_count__gte: Option<i64>,
    /// Number of CPUs (less than). Optional.
    #[serde(rename = "cpuCount__lt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lt: Option<i64>,
    /// Number of CPUs (less than or equal). Optional.
    #[serde(rename = "cpuCount__lte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__lte: Option<i64>,
    /// Number of CPUs (more than). Optional.
    #[serde(rename = "cpuCount__gt", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gt: Option<i64>,
    /// Number of CPUs (more than or equal). Optional.
    #[serde(rename = "cpuCount__gte", skip_serializing_if = "Option::is_none")]
    pub cpu_count__gte: Option<i64>,
    /// Memory size (MB, less than). Optional.
    #[serde(rename = "totalMemory__lt", skip_serializing_if = "Option::is_none")]
    pub total_memory__lt: Option<i64>,
    /// Memory size (MB, less than or equal). Optional.
    #[serde(rename = "totalMemory__lte", skip_serializing_if = "Option::is_none")]
    pub total_memory__lte: Option<i64>,
    /// Memory size (MB, more than). Optional.
    #[serde(rename = "totalMemory__gt", skip_serializing_if = "Option::is_none")]
    pub total_memory__gt: Option<i64>,
    /// Memory size (MB, more than or equal). Optional.
    #[serde(rename = "totalMemory__gte", skip_serializing_if = "Option::is_none")]
    pub total_memory__gte: Option<i64>,
    /// Migration status. Example: "N/A". Optional.
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    #[serde(rename = "migrationStatus", skip_serializing_if = "Option::is_none")]
    pub migration_status: Option<String>,
    /// Gateway ip. Example: "192.168.0.1". Optional.
    #[serde(rename = "gatewayIp", skip_serializing_if = "Option::is_none")]
    pub gateway_ip: Option<String>,
    /// The ID of the CSV file to filter by. Example: "225494730938493804". Optional.
    #[serde(rename = "csvFilterId", skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<String>,
    /// Supported Remote Script Orchestration level. Example: "none". Optional.
    ///
    /// Allowed values: none, pro, ars.
    #[serde(rename = "rsoLevel", skip_serializing_if = "Option::is_none")]
    pub rso_level: Option<String>,
    /// Include only agents that has Remote Ops Forensicsfeature supported. Optional.
    #[serde(rename = "remoteOpsForensicsSupported", skip_serializing_if = "Option::is_none")]
    pub remote_ops_forensics_supported: Option<bool>,
    /// Agents os revision than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__gte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__gte: Option<i64>,
    /// Agents os revision lower than or equal to given version. Optional.
    #[serde(rename = "windowsOsRevision__lte", skip_serializing_if = "Option::is_none")]
    pub windows_os_revision__lte: Option<i64>,
    /// A list of included rso_levels. Example: "pro,ars". Optional.
    #[serde(rename = "rsoLevels", skip_serializing_if = "Option::is_none")]
    pub rso_levels: Option<String>,
}

impl AgentsExportPassphrasesQuery {
    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }

    /// List of Account IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }

    /// List of network groups. Example: "225494730938493804,225494730938493915".
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }

    /// List of Group IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_group_ids = Some(join_csv(vals));
        self
    }

    /// List of Site IDs to filter by. Example: "225494730938493804,225494730938493915".
    pub fn filtered_site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.filtered_site_ids = Some(join_csv(vals));
        self
    }

    /// Date range for first registration time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn registered_at__between(mut self, v: impl Into<String>) -> Self {
        self.registered_at__between = Some(v.into());
        self
    }

    /// Date range for last active date(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_active_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__between = Some(v.into());
        self
    }

    /// Date range for last successful full disk scan(format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn last_successful_scan_date__between(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__between = Some(v.into());
        self
    }

    /// Include only active Agents.
    pub fn is_active<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_active = Some(join_csv(vals));
        self
    }

    /// Include only Agents with pending uninstall requests.
    pub fn is_pending_uninstall<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_pending_uninstall = Some(join_csv(vals));
        self
    }

    /// Include only Agents with at least one active threat.
    pub fn infected(mut self, b: bool) -> Self {
        self.infected = Some(b);
        self
    }

    /// Include only Agents with updated software.
    pub fn is_up_to_date<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_up_to_date = Some(join_csv(vals));
        self
    }

    /// A free-text search term, will match applicable attributes (sub-string match). Note: Device's physical addresses will be matched if they start with the search term only (no match if they contain the term). Example: "Linux".
    pub fn query(mut self, v: impl Into<String>) -> Self {
        self.query = Some(v.into());
        self
    }

    /// Agent versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions = Some(join_csv(vals));
        self
    }

    /// Agent versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn agent_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_versions_nin = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions = Some(join_csv(vals));
        self
    }

    /// Network Scanner versions not to include. Example: "2.0.0.0,2.1.5.144".
    pub fn ranger_versions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_versions_nin = Some(join_csv(vals));
        self
    }

    /// OS architecture. Example: "32 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arch(mut self, v: impl Into<String>) -> Self {
        self.os_arch = Some(v.into());
        self
    }

    /// OS architectures to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches = Some(join_csv(vals));
        self
    }

    /// OS architectures not to include. Example: "32 bit,64 bit".
    ///
    /// Allowed values: 32 bit, 64 bit, ARM64.
    pub fn os_arches_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_arches_nin = Some(join_csv(vals));
        self
    }

    /// Included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types = Some(join_csv(vals));
        self
    }

    /// Not included OS types. Example: "linux".
    ///
    /// Allowed values: linux, macos, windows_legacy, windows.
    pub fn os_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_types_nin = Some(join_csv(vals));
        self
    }

    /// Included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses = Some(join_csv(vals));
        self
    }

    /// Not included scan statuses. Example: "started,aborted".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types = Some(join_csv(vals));
        self
    }

    /// Not included machine types. Example: "laptop,desktop".
    ///
    /// Allowed values: unknown, desktop, laptop, server, kubernetes node, storage, kubernetes pod, ecs task, kubernetes helper.
    pub fn machine_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.machine_types_nin = Some(join_csv(vals));
        self
    }

    /// Included storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types = Some(join_csv(vals));
        self
    }

    /// Excluded storage types. Example: "NetApp,Dell,S3".
    pub fn storage_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.storage_types_nin = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses = Some(join_csv(vals));
        self
    }

    /// Included network statuses. Example: "connected,connecting".
    ///
    /// Allowed values: connected, disconnected, connecting, disconnecting.
    pub fn network_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains = Some(join_csv(vals));
        self
    }

    /// Not included network domains. Example: "mybusiness.net,workgroup".
    pub fn domains_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domains_nin = Some(join_csv(vals));
        self
    }

    /// Disk encryption status.
    pub fn encrypted_applications<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encrypted_applications = Some(join_csv(vals));
        self
    }

    /// Total memory range (GB, inclusive). Example: "4-8".
    pub fn total_memory__between(mut self, v: impl Into<String>) -> Self {
        self.total_memory__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn core_count__between(mut self, v: impl Into<String>) -> Self {
        self.core_count__between = Some(v.into());
        self
    }

    /// Possible number of CPU cores (inclusive). Example: "2-8".
    pub fn cpu_count__between(mut self, v: impl Into<String>) -> Self {
        self.cpu_count__between = Some(v.into());
        self
    }

    /// Included pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed = Some(join_csv(vals));
        self
    }

    /// Included missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions = Some(join_csv(vals));
        self
    }

    /// Excluded pending user actions. Example: "reboot_needed,upgrade_needed".
    ///
    /// Allowed values: none, user_action_needed, reboot_needed, upgrade_needed, incompatible_os, unprotected, rebootless_without_dynamic_detection, extended_exclusions_partially_accepted, reboot_required, pending_deprecation, ne_not_running, ne_cf_not_active, pending_performance_insights, reboot_category, missing_permissions_category, agent_suppressed_category, incompatible_os_category, unprotected_category, partial_functionality, performance_insights.
    pub fn user_actions_needed_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.user_actions_needed_nin = Some(join_csv(vals));
        self
    }

    /// Excluded missing permissions. Example: "user_action_needed_bluetooth_per,user_action_needed_fda_helper".
    ///
    /// Allowed values: user_action_needed_fda, user_action_needed_rs_fda, user_action_needed_fda_helper, user_action_needed_fda_sentineld, user_action_needed_bluetooth_per, user_action_needed_network, user_action_needed_notifications.
    pub fn missing_permissions_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_permissions_nin = Some(join_csv(vals));
        self
    }

    /// An Active Directory query string. Example: "CN=Managers,DC=sentinelone,DC=com".
    pub fn ad_query(mut self, v: impl Into<String>) -> Self {
        self.ad_query = Some(v.into());
        self
    }

    /// Agent has a local configuration set.
    pub fn has_local_configuration(mut self, b: bool) -> Self {
        self.has_local_configuration = Some(b);
        self
    }

    /// Migration status in. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses = Some(join_csv(vals));
        self
    }

    /// Migration status nin. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn console_migration_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.console_migration_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status in. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses = Some(join_csv(vals));
        self
    }

    /// Apps vulnerability status nin. Example: "patch_required".
    ///
    /// Allowed values: patch_required, up_to_date, not_applicable.
    pub fn apps_vulnerability_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.apps_vulnerability_statuses_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids = Some(join_csv(vals));
        self
    }

    /// Do not include only Agents reporting these locations. Example: "225494730938493804,225494730938493915".
    pub fn location_ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_ids_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types = Some(join_csv(vals));
        self
    }

    /// Exclude Agents installed with these package types. Example: ".msi".
    ///
    /// Allowed values: .msi, .exe, .deb, .rpm, .bsx, .pkg, .img, unknown, .tar, .zip, .gz, .xz.
    pub fn installer_types_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.installer_types_nin = Some(join_csv(vals));
        self
    }

    /// Agent operational state.
    pub fn operational_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent operational states.
    pub fn operational_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.operational_states_nin = Some(join_csv(vals));
        self
    }

    /// Agent remote profiling state.
    pub fn remote_profiling_states<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states = Some(join_csv(vals));
        self
    }

    /// Do not include these Agent remote profiling states.
    pub fn remote_profiling_states_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.remote_profiling_states_nin = Some(join_csv(vals));
        self
    }

    /// Status of Network Discovery. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses = Some(join_csv(vals));
        self
    }

    /// Do not include these Network Scanner Statuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_statuses_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_statuses_nin = Some(join_csv(vals));
        self
    }

    /// [DEPRECATED] Use rangerStatuses. Example: "NotApplicable".
    ///
    /// Allowed values: NotApplicable, Enabled, Disabled.
    pub fn ranger_status(mut self, v: impl Into<String>) -> Self {
        self.ranger_status = Some(v.into());
        self
    }

    /// Has at least one threat with at least one mitigation action pending reboot to succeed.
    pub fn threat_reboot_required<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_reboot_required = Some(join_csv(vals));
        self
    }

    /// The agents supports Network Quarantine Control and its enabled for the agent's group.
    pub fn network_quarantine_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_quarantine_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Firewall Control and it is enabled for the agent's group.
    pub fn firewall_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.firewall_enabled = Some(join_csv(vals));
        self
    }

    /// The agents supports Location Awareness and it is enabled for the agent's group.
    pub fn location_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.location_enabled = Some(join_csv(vals));
        self
    }

    /// Agents from which cloud provider.
    pub fn cloud_provider<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider = Some(join_csv(vals));
        self
    }

    /// Exclude Agents from these cloud provider.
    pub fn cloud_provider_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_nin = Some(join_csv(vals));
        self
    }

    /// Filter agents by their assigned tags. Given in form of a JSON where each key represents a tag key, and each value represents a list of string values to filter by. To filter by unassigned tag values, use __nin suffix in the tag key. Example: "{"key1": ["value1_1", "value1_2"], "key2__nin": ["value2"]}".
    pub fn tags_data(mut self, v: impl Into<String>) -> Self {
        self.tags_data = Some(v.into());
        self
    }

    /// Include only Agents that have any tags assigned if True, or none if False.
    pub fn has_tags(mut self, b: bool) -> Self {
        self.has_tags = Some(b);
        self
    }

    /// The agents that are ADConnectors if True, or not if False.
    pub fn is_ad_connector<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_ad_connector = Some(join_csv(vals));
        self
    }

    /// The agents has Hyper Automate PNA enabled if True, or not if False.
    pub fn is_hyper_automate<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_hyper_automate = Some(join_csv(vals));
        self
    }

    /// Included agent active protections. Example: "edr,idr".
    ///
    /// Allowed values: edr, idr.
    pub fn active_protection<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_protection = Some(join_csv(vals));
        self
    }

    /// Containerized workload counts.
    pub fn containerized_workload_counts(mut self, v: impl Into<String>) -> Self {
        self.containerized_workload_counts = Some(v.into());
        self
    }

    /// Indicates whether the agent protects containerized workload at the moment.
    pub fn has_containerized_workload<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.has_containerized_workload = Some(join_csv(vals));
        self
    }

    /// Include only Agents using PAC file for proxy configuration. Accepts true, false, or none (for not reported).
    pub fn pac_file_usage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.pac_file_usage = Some(join_csv(vals));
        self
    }

    /// Include only Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method = Some(join_csv(vals));
        self
    }

    /// Exclude Agents using these proxy methods. Example: "None,Auto,System,User,Custom".
    pub fn proxy_method_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.proxy_method_nin = Some(join_csv(vals));
        self
    }

    /// Include only Agents using mgmt proxy. Accepts true, false, or none (for not reported).
    pub fn is_mgmt_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_mgmt_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Include only Agents using event search proxy. Accepts true, false, or none (for not reported).
    pub fn is_event_search_proxy_enabled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_event_search_proxy_enabled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by visible IP (supports multiple values). Example: "205,127.0".
    pub fn external_ip__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_ip__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name (supports multiple values). Example: "john-office,WIN".
    pub fn computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by local IP (supports multiple values). Example: "192,10.0.0".
    pub fn network_interface_inet__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_inet__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_physical__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_physical__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Gateway MAC address (supports multiple values). Example: "aa:0f,:41:".
    pub fn network_interface_gateway_mac_address__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_interface_gateway_mac_address__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by username (supports multiple values). Example: "admin,johnd1".
    pub fn last_logged_in_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.last_logged_in_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by OS full name and version (supports multiple values). Example: "Service Pack 1".
    pub fn os_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory username string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory user groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_user_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,John".
    pub fn ad_user_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_user_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer groups string (supports multiple values). Example: "DC=sentinelone".
    pub fn ad_computer_member__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_member__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Active Directory computer name or its groups (supports multiple values). Example: "DC=sentinelone,Windows".
    pub fn ad_computer_query__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ad_computer_query__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Agent UUID (supports multiple values). Example: "e92-01928,b055".
    pub fn uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuid__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by external ID (Customer ID). Example: "Tag#1 - monitoring,Performance machine".
    pub fn external_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.external_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws role(supports multiple values).
    pub fn aws_role__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_role__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws securityGroups(supports multiple values).
    pub fn aws_security_groups__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_security_groups__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by aws subnet ids (supports multiple values).
    pub fn aws_subnet_ids__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.aws_subnet_ids__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent namespace (supports multiple values).
    pub fn agent_namespace__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_namespace__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by agent pod name (supports multiple values).
    pub fn agent_pod_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.agent_pod_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by azure resource group(supports multiple values).
    pub fn azure_resource_group__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.azure_resource_group__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud account (supports multiple values).
    pub fn cloud_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud image (supports multiple values).
    pub fn cloud_image__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_image__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance id(supports multiple values).
    pub fn cloud_instance_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud instance size(supports multiple values).
    pub fn cloud_instance_size__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_instance_size__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud location (supports multiple values).
    pub fn cloud_location__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_location__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud network (supports multiple values).
    pub fn cloud_network__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_network__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cloud tags (supports multiple values).
    pub fn cloud_tags__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by cluster name (supports multiple values).
    pub fn cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by gcp service account (supports multiple values).
    pub fn gcp_service_account__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gcp_service_account__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node labels (supports multiple values).
    pub fn k8s_node_labels__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_labels__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s node name (supports multiple values).
    pub fn k8s_node_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_node_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s type(supports multiple values).
    pub fn k8s_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by K8s version (supports multiple values).
    pub fn k8s_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.k8s_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by live update ID (supports multiple values).
    pub fn live_update_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.live_update_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Serial Number (supports multiple values).
    pub fn serial_number__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.serial_number__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by Entra ID (supports multiple values).
    pub fn entra_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.entra_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by CPU name (supports multiple values). Example: "Intel,AMD".
    pub fn cpu_id__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cpu_id__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS type.
    pub fn ecs_type__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_type__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS version.
    pub fn ecs_version__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_version__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS cluster name.
    pub fn ecs_cluster_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_cluster_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task arn.
    pub fn ecs_task_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task availability zone.
    pub fn ecs_task_availability_zone__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_availability_zone__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service name.
    pub fn ecs_service_name__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_name__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS service arn.
    pub fn ecs_service_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_service_arn__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition family.
    pub fn ecs_task_definition_family__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_family__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition revision.
    pub fn ecs_task_definition_revision__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_revision__contains = Some(join_csv(vals));
        self
    }

    /// Free-text filter by ECS task definition arn.
    pub fn ecs_task_definition_arn__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ecs_task_definition_arn__contains = Some(join_csv(vals));
        self
    }

    /// Include active, decommissioned or both. Example: "True,False".
    pub fn is_decommissioned<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_decommissioned = Some(join_csv(vals));
        self
    }

    /// Include installed, uninstalled or both. Example: "True,False".
    pub fn is_uninstalled<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_uninstalled = Some(join_csv(vals));
        self
    }

    /// Free-text filter by computer name or uuid (supports multiple values).
    pub fn computer_name_or_uuid__contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.computer_name_or_uuid__contains = Some(join_csv(vals));
        self
    }

    /// Included Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids = Some(join_csv(vals));
        self
    }

    /// Excluded Agent IDs. Example: "225494730938493804,225494730938493915".
    pub fn ids_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ids_nin = Some(join_csv(vals));
        self
    }

    /// Include all Agents matching this saved filter. Example: "225494730938493804".
    pub fn filter_id(mut self, v: impl Into<String>) -> Self {
        self.filter_id = Some(v.into());
        self
    }

    /// Agents decommissioned after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gte = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lt = Some(v.into());
        self
    }

    /// Agents decommissioned before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__lte(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__lte = Some(v.into());
        self
    }

    /// Agents decommissioned after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn decommissioned_at__gt(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__gt = Some(v.into());
        self
    }

    /// Date range for decommission time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn decommissioned_at__between(mut self, v: impl Into<String>) -> Self {
        self.decommissioned_at__between = Some(v.into());
        self
    }

    /// Agents created before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.created_at__lt = Some(v.into());
        self
    }

    /// Agents created before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.created_at__lte = Some(v.into());
        self
    }

    /// Agents created after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.created_at__gt = Some(v.into());
        self
    }

    /// Agents created after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.created_at__gte = Some(v.into());
        self
    }

    /// Date range for creation time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn created_at__between(mut self, v: impl Into<String>) -> Self {
        self.created_at__between = Some(v.into());
        self
    }

    /// Agents updated before this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lt = Some(v.into());
        self
    }

    /// Agents updated before or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__lte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__lte = Some(v.into());
        self
    }

    /// Agents updated after this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gt(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gt = Some(v.into());
        self
    }

    /// Agents updated after or at this timestamp. Example: "2018-02-27T04:49:26.257525Z".
    pub fn updated_at__gte(mut self, v: impl Into<String>) -> Self {
        self.updated_at__gte = Some(v.into());
        self
    }

    /// Date range for update time (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978890136-1514978650130".
    pub fn updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.updated_at__between = Some(v.into());
        self
    }

    /// Match computer name partially (substring). Example: "Lab1".
    pub fn computer_name__like(mut self, v: impl Into<String>) -> Self {
        self.computer_name__like = Some(v.into());
        self
    }

    /// Computer name. Example: "My Office Desktop".
    pub fn computer_name(mut self, v: impl Into<String>) -> Self {
        self.computer_name = Some(v.into());
        self
    }

    /// Agents versions less than given version. Example: "2.5.1.1320".
    pub fn agent_version__lt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lt = Some(v.into());
        self
    }

    /// Agents versions less than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__lte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__lte = Some(v.into());
        self
    }

    /// Agents versions greater than given version. Example: "2.5.1.1320".
    pub fn agent_version__gt(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gt = Some(v.into());
        self
    }

    /// Agents versions greater than or equal to given version. Example: "2.5.1.1320".
    pub fn agent_version__gte(mut self, v: impl Into<String>) -> Self {
        self.agent_version__gte = Some(v.into());
        self
    }

    /// Version range for agent version (format: <from_version>-<to_version>, inclusive). Example: "2.0.0.0-2.1.5.144".
    pub fn agent_version__between(mut self, v: impl Into<String>) -> Self {
        self.agent_version__between = Some(v.into());
        self
    }

    /// Agent's universally unique identifier. Example: "ff819e70af13be381993075eb0ce5f2f6de05be2".
    pub fn uuid(mut self, v: impl Into<String>) -> Self {
        self.uuid = Some(v.into());
        self
    }

    /// A list of included UUIDs. Example: "ff819e70af13be381993075eb0ce5f2f6de05b11,ff819e70af13be381993075eb0ce5f2f6de05c22".
    pub fn uuids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.uuids = Some(join_csv(vals));
        self
    }

    /// Scan status. Example: "none".
    ///
    /// Allowed values: none, started, aborted, finished.
    pub fn scan_status(mut self, v: impl Into<String>) -> Self {
        self.scan_status = Some(v.into());
        self
    }

    /// Include only Agents that have threats with this mitigation status. Example: "mitigated".
    ///
    /// Allowed values: mitigated, blocked, active, suspicious, pending, suspicious_resolved.
    pub fn threat_mitigation_status(mut self, v: impl Into<String>) -> Self {
        self.threat_mitigation_status = Some(v.into());
        self
    }

    /// Include only Agents with at least one resolved threat.
    pub fn threat_resolved(mut self, b: bool) -> Self {
        self.threat_resolved = Some(b);
        self
    }

    /// Include only Agents with at least one hidden threat.
    pub fn threat_hidden(mut self, b: bool) -> Self {
        self.threat_hidden = Some(b);
        self
    }

    /// Include only Agents that have at least one threat with this content hash. Example: "cf23df2207d99a74fbe169e3eba035e633b65d94".
    pub fn threat_content_hash(mut self, v: impl Into<String>) -> Self {
        self.threat_content_hash = Some(v.into());
        self
    }

    /// Agents with threats reported before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lt = Some(v.into());
        self
    }

    /// Agents with threats reported before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__lte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__lte = Some(v.into());
        self
    }

    /// Agents with threats reported after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gt(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gt = Some(v.into());
        self
    }

    /// Agents with threats reported after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn threat_created_at__gte(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__gte = Some(v.into());
        self
    }

    /// Agents with threats reported in a date range (format: <from_timestamp>-<to_timestamp>, inclusive). Example: "1514978764288-1514978999999".
    pub fn threat_created_at__between(mut self, v: impl Into<String>) -> Self {
        self.threat_created_at__between = Some(v.into());
        self
    }

    /// Include Agents with this amount of active threats. Example: "3".
    pub fn active_threats(mut self, n: i64) -> Self {
        self.active_threats = Some(n);
        self
    }

    /// Include Agents with at least this amount of active threats. Example: "5".
    pub fn active_threats__gt(mut self, n: i64) -> Self {
        self.active_threats__gt = Some(n);
        self
    }

    /// Agent mitigation mode policy. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode = Some(v.into());
        self
    }

    /// Mitigation mode policy for suspicious activity. Example: "detect".
    ///
    /// Allowed values: detect, protect.
    pub fn mitigation_mode_suspicious(mut self, v: impl Into<String>) -> Self {
        self.mitigation_mode_suspicious = Some(v.into());
        self
    }

    /// Agents registered before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lt = Some(v.into());
        self
    }

    /// Agents registered before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__lte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__lte = Some(v.into());
        self
    }

    /// Agents registered after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gt(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gt = Some(v.into());
        self
    }

    /// Agents registered after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn registered_at__gte(mut self, v: impl Into<String>) -> Self {
        self.registered_at__gte = Some(v.into());
        self
    }

    /// Agents last active before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lt = Some(v.into());
        self
    }

    /// Agents last active before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__lte = Some(v.into());
        self
    }

    /// Agents last active after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gt = Some(v.into());
        self
    }

    /// Agents last active after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_active_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_active_date__gte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan before or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__lte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__lte = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gt(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gt = Some(v.into());
        self
    }

    /// Agents last successful full disk scan after or at this time. Example: "2018-02-27T04:49:26.257525Z".
    pub fn last_successful_scan_date__gte(mut self, v: impl Into<String>) -> Self {
        self.last_successful_scan_date__gte = Some(v.into());
        self
    }

    /// CPU cores (less than).
    pub fn core_count__lt(mut self, n: i64) -> Self {
        self.core_count__lt = Some(n);
        self
    }

    /// CPU cores (less than or equal).
    pub fn core_count__lte(mut self, n: i64) -> Self {
        self.core_count__lte = Some(n);
        self
    }

    /// CPU cores (more than).
    pub fn core_count__gt(mut self, n: i64) -> Self {
        self.core_count__gt = Some(n);
        self
    }

    /// CPU cores (more than or equal).
    pub fn core_count__gte(mut self, n: i64) -> Self {
        self.core_count__gte = Some(n);
        self
    }

    /// Number of CPUs (less than).
    pub fn cpu_count__lt(mut self, n: i64) -> Self {
        self.cpu_count__lt = Some(n);
        self
    }

    /// Number of CPUs (less than or equal).
    pub fn cpu_count__lte(mut self, n: i64) -> Self {
        self.cpu_count__lte = Some(n);
        self
    }

    /// Number of CPUs (more than).
    pub fn cpu_count__gt(mut self, n: i64) -> Self {
        self.cpu_count__gt = Some(n);
        self
    }

    /// Number of CPUs (more than or equal).
    pub fn cpu_count__gte(mut self, n: i64) -> Self {
        self.cpu_count__gte = Some(n);
        self
    }

    /// Memory size (MB, less than).
    pub fn total_memory__lt(mut self, n: i64) -> Self {
        self.total_memory__lt = Some(n);
        self
    }

    /// Memory size (MB, less than or equal).
    pub fn total_memory__lte(mut self, n: i64) -> Self {
        self.total_memory__lte = Some(n);
        self
    }

    /// Memory size (MB, more than).
    pub fn total_memory__gt(mut self, n: i64) -> Self {
        self.total_memory__gt = Some(n);
        self
    }

    /// Memory size (MB, more than or equal).
    pub fn total_memory__gte(mut self, n: i64) -> Self {
        self.total_memory__gte = Some(n);
        self
    }

    /// Migration status. Example: "N/A".
    ///
    /// Allowed values: N/A, Pending, Migrated, Failed.
    pub fn migration_status(mut self, v: impl Into<String>) -> Self {
        self.migration_status = Some(v.into());
        self
    }

    /// Gateway ip. Example: "192.168.0.1".
    pub fn gateway_ip(mut self, v: impl Into<String>) -> Self {
        self.gateway_ip = Some(v.into());
        self
    }

    /// The ID of the CSV file to filter by. Example: "225494730938493804".
    pub fn csv_filter_id(mut self, v: impl Into<String>) -> Self {
        self.csv_filter_id = Some(v.into());
        self
    }

    /// Supported Remote Script Orchestration level. Example: "none".
    ///
    /// Allowed values: none, pro, ars.
    pub fn rso_level(mut self, v: impl Into<String>) -> Self {
        self.rso_level = Some(v.into());
        self
    }

    /// Include only agents that has Remote Ops Forensicsfeature supported.
    pub fn remote_ops_forensics_supported(mut self, b: bool) -> Self {
        self.remote_ops_forensics_supported = Some(b);
        self
    }

    /// Agents os revision than or equal to given version.
    pub fn windows_os_revision__gte(mut self, n: i64) -> Self {
        self.windows_os_revision__gte = Some(n);
        self
    }

    /// Agents os revision lower than or equal to given version.
    pub fn windows_os_revision__lte(mut self, n: i64) -> Self {
        self.windows_os_revision__lte = Some(n);
        self
    }

    /// A list of included rso_levels. Example: "pro,ars".
    pub fn rso_levels<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.rso_levels = Some(join_csv(vals));
        self
    }
}

// --- Response / body shapes (inline, since the entity model only covers `Agent`) ---

/// One installed application reported by an Agent, as returned by
/// `GET /web/api/v2.1/agents/applications`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentApplication {
    /// Application name.
    pub name: Option<String>,
    /// Application size (bytes).
    pub size: Option<i64>,
    /// Application version.
    pub version: Option<String>,
    /// Application publisher.
    pub publisher: Option<String>,
    /// Installed date.
    pub installed_date: Option<String>,
}

/// One process entry, as returned by `GET /web/api/v2.1/agents/processes`.
///
/// Note: that endpoint is obsolete and returns an empty array.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentProcess {
    /// Process ID.
    pub pid: Option<i64>,
    /// Process name.
    pub process_name: Option<String>,
    /// CPU usage (%).
    pub cpu_usage: Option<i64>,
    /// Memory usage (MB).
    pub memory_usage: Option<i64>,
    /// Executable path.
    pub executable_path: Option<String>,
    /// Start time.
    pub start_time: Option<String>,
}

/// A previously generated passphrase for an Agent.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousPassphrase {
    /// Previous passphrase for the Agent.
    pub passphrase: Option<String>,
    /// Created at.
    pub created_at: Option<String>,
    /// Created by user.
    pub created_by_user: Option<String>,
}

/// An Agent passphrase entry, as returned by
/// `GET /web/api/v2.1/agents/passphrases`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPassphrase {
    /// Agent ID.
    pub id: Option<String>,
    /// Agent's universally unique identifier.
    pub uuid: Option<String>,
    /// Network domain.
    pub domain: Option<String>,
    /// Computer name.
    pub computer_name: Option<String>,
    /// Last logged in user name.
    pub last_logged_in_user_name: Option<String>,
    /// Generated passphrase for the Agent.
    pub passphrase: Option<String>,
    /// Created at.
    pub created_at: Option<String>,
    /// Acknowledged at.
    pub acknowledged_at: Option<String>,
    /// Created by user.
    pub created_by_user: Option<String>,
    /// Previous passphrases.
    #[serde(default)]
    pub previous_passphrases: Option<Vec<PreviousPassphrase>>,
}

/// An endpoint tag entry, as returned by `GET /web/api/v2.1/agents/tags`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTag {
    /// Tag ID.
    pub id: Option<String>,
    /// Tag key.
    pub key: Option<String>,
    /// Tag value.
    pub value: Option<String>,
    /// Tag description.
    pub description: Option<String>,
    /// Tag type.
    #[serde(rename = "type")]
    pub tag_type: Option<String>,
    /// Indicates whether the user can edit the tag.
    pub allow_edit: Option<bool>,
    /// Tag scope path.
    pub scope_path: Option<String>,
    /// Scope level.
    pub scope_level: Option<String>,
    /// Scope ID.
    pub scope_id: Option<String>,
    /// Timestamp of creation.
    pub created_at: Option<String>,
    /// Timestamp of last update.
    pub updated_at: Option<String>,
    /// Tag creator name.
    pub created_by: Option<String>,
    /// Tag updater name.
    pub updated_by: Option<String>,
    /// The total number of endpoints that have this tag.
    pub total_endpoints: Option<i64>,
    /// The number of endpoints in this scope that have this tag.
    pub endpoints_in_current_scope: Option<i64>,
    /// Number of exclusions with this tag.
    pub total_exclusions: Option<i64>,
}

/// A single filter value with its count, used by
/// [`AgentTagsFilterCount`].
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTagsFilterValue {
    /// Value description.
    pub title: Option<String>,
    /// Value.
    pub value: Option<String>,
    /// Number of entities matching this value.
    pub count: Option<i64>,
}

/// An endpoint-tags filter-count entry, as returned by
/// `GET /web/api/v2.1/agents/tags/filters-count`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTagsFilterCount {
    /// Filter argument key.
    pub key: Option<String>,
    /// Filter description.
    pub title: Option<String>,
    /// A list of filter values with their count.
    #[serde(default)]
    pub values: Option<Vec<AgentTagsFilterValue>>,
    /// Whether the negation query is enabled for this filter key.
    pub enable_negation: Option<bool>,
    /// Whether to disable the UI filter-values sorting by counts.
    pub disable_sorting: Option<bool>,
    /// Whether the filter is hidden in the UI filters but kept for grouping.
    pub hidden_filter: Option<bool>,
    /// Whether the filter is hidden in the UI group-by but kept for filtering.
    pub hidden_group_by: Option<bool>,
    /// Filter type.
    #[serde(rename = "type")]
    pub filter_type: Option<String>,
}

/// Local upgrade/downgrade authorization, as returned by
/// `GET /web/api/v2.1/agents/{agent_id}/local-upgrade-authorization`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentLocalUpgradeAuthorization {
    /// Agent authorization (expiry time of local upgrade/downgrade approval).
    pub agent_authorization: Option<String>,
    /// Site authorization.
    pub site_authorization: Option<String>,
}

/// Request body for the Hyperautomation PNA action endpoints
/// (`agents.schemas_AgentsActionSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentsActionRequest {
    /// Applied filter — only matched Agents will be affected by the requested
    /// action. Leave empty (an empty object) to apply the action on all
    /// applicable Agents. Required.
    pub filter: serde_json::Value,
    /// Free-form data object. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl AgentsActionRequest {
    /// Build a request with the given `filter` object (e.g. a
    /// `serde_json::json!({ "ids": ["..."] })`).
    pub fn new(filter: serde_json::Value) -> Self {
        Self { filter, data: None }
    }

    /// Attach a free-form `data` object.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Envelope `data` for `GET /web/api/v2.1/agents/count`.
#[derive(Debug, Clone, Deserialize)]
struct AgentsCount {
    /// Number of Agents matching the input filter.
    #[serde(default)]
    total: i64,
}

/// Envelope `data` for the PNA action endpoints.
#[derive(Debug, Clone, Deserialize)]
struct Affected {
    /// Number of entities affected by the requested operation.
    #[serde(default)]
    affected: i64,
}

/// Join an iterator of string-like items into a comma-separated string.
fn join_csv<I, S>(items: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    items
        .into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}


impl AgentsService<'_> {
    /// `GET /web/api/v2.1/agents` — Get Agents.
    ///
    /// Get the Agents, and their data, that match the filter. This command
    /// gives the Agent ID, which you can use in other commands. To save the
    /// list and data to a CSV file, use `export_agents`.
    pub async fn list(&self, query: &AgentsQuery) -> Result<Paginated<Agent>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/agents", q).await?)
    }

    /// `GET /web/api/v2.1/agents/applications` — Applications.
    ///
    /// Get the installed applications for the given Agents. The `ids` argument
    /// (Agent ID list) is the required `ids` query parameter.
    pub async fn applications<I, S>(&self, ids: I) -> Result<Paginated<AgentApplication>, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let ids = join_csv(ids);
        let qs = serde_urlencoded::to_string(&[("ids", ids.as_str())]).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/agents/applications", q)
            .await?)
    }

    /// `GET /web/api/v2.1/agents/count` — Count Agents.
    ///
    /// Get the count of Agents that match a filter. Returns the number of
    /// matching Agents (the `data.total` field of the response envelope).
    pub async fn count(&self, query: &AgentsCountQuery) -> Result<i64, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        let resp: Response<AgentsCount> = self
            .client
            .http()
            .get("/web/api/v2.1/agents/count", q)
            .await?;
        Ok(resp.data.total)
    }

    /// `GET /web/api/v2.1/agents/passphrases` — Get Passphrase.
    ///
    /// Show the passphrase for the Agents that match the filter.
    pub async fn passphrases(
        &self,
        query: &AgentsPassphrasesQuery,
    ) -> Result<Paginated<AgentPassphrase>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/agents/passphrases", q)
            .await?)
    }

    /// `GET /web/api/v2.1/agents/processes` — Processes.
    ///
    /// [OBSOLETE] Returns an empty array. To get processes of an Agent, see
    /// [`applications`](Self::applications). The `ids` argument (Agent ID list)
    /// is the required `ids` query parameter.
    pub async fn processes<I, S>(&self, ids: I) -> Result<Paginated<AgentProcess>, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let ids = join_csv(ids);
        let qs = serde_urlencoded::to_string(&[("ids", ids.as_str())]).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/agents/processes", q)
            .await?)
    }

    /// `GET /web/api/v2.1/agents/tags` — Get the endpoint tags that match the
    /// filters.
    pub async fn tags(&self, query: &AgentsTagsQuery) -> Result<Paginated<AgentTag>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/agents/tags", q)
            .await?)
    }

    /// `GET /web/api/v2.1/agents/tags/filters-count` — Endpoint tags count by
    /// Filters.
    pub async fn tags_filters_count(
        &self,
        query: &AgentsTagsFiltersCountQuery,
    ) -> Result<Paginated<AgentTagsFilterCount>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/agents/tags/filters-count", q)
            .await?)
    }

    /// `GET /web/api/v2.1/agents/{agent_id}/local-upgrade-authorization` — Get
    /// local upgrade/downgrade Agent authorization.
    ///
    /// Get the time when authorization of local upgrades/downgrades expires.
    pub async fn local_upgrade_authorization(
        &self,
        agent_id: &str,
    ) -> Result<Response<AgentLocalUpgradeAuthorization>, Error> {
        let path = format!("/web/api/v2.1/agents/{agent_id}/local-upgrade-authorization");
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `GET /web/api/v2.1/agents/{agent_id}/uploads/{activity_id}` — Export
    /// Agent Logs.
    ///
    /// Get Agent logs uploaded by the Agent for the given activity. The response
    /// is an export payload rather than a typed envelope, so it is returned as a
    /// freeform JSON value.
    pub async fn upload(
        &self,
        agent_id: &str,
        activity_id: &str,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!("/web/api/v2.1/agents/{agent_id}/uploads/{activity_id}");
        Ok(self.client.http().get(&path, None).await?)
    }

    /// `GET /web/api/v2.1/export/agents` — Export Agents.
    ///
    /// Export Agent data to a CSV, for Agents that match the filter (up to
    /// 50000 items). The response is an export payload rather than a typed
    /// envelope, so it is returned as a freeform JSON value.
    pub async fn export_agents(
        &self,
        query: &AgentsExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/export/agents", q)
            .await?)
    }

    /// `GET /web/api/v2.1/export/agents-light` — Export Agents - Light.
    ///
    /// Export Agent data to a CSV, for Agents that match the filter (up to
    /// 300000 items). The response is an export payload rather than a typed
    /// envelope, so it is returned as a freeform JSON value.
    pub async fn export_agents_light(
        &self,
        query: &AgentsExportLightQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/export/agents-light", q)
            .await?)
    }

    /// `GET /web/api/v2.1/export/agents-passphrases` — Export Agents -
    /// Passphrases.
    ///
    /// Export Agent passphrases to a CSV, for Agents that match the filter (up
    /// to 2000 items). The response is an export payload rather than a typed
    /// envelope, so it is returned as a freeform JSON value.
    pub async fn export_agents_passphrases(
        &self,
        query: &AgentsExportPassphrasesQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/export/agents-passphrases", q)
            .await?)
    }

    /// `POST /web/api/v2.1/agents/disable-hyper-automation-pna` — Disable PNA
    /// for Hyperautomation.
    ///
    /// Disable Agent PNA for Hyperautomation. Returns the number of affected
    /// Agents.
    pub async fn disable_hyper_automation_pna(
        &self,
        request: &AgentsActionRequest,
    ) -> Result<i64, Error> {
        let resp: Response<Affected> = self
            .client
            .http()
            .post("/web/api/v2.1/agents/disable-hyper-automation-pna", request)
            .await?;
        Ok(resp.data.affected)
    }

    /// `POST /web/api/v2.1/agents/enable-hyper-automation-pna` — Enable Agent
    /// PNA for Hyperautomation.
    ///
    /// Enable Agent PNA for Hyperautomation. Returns the number of affected
    /// Agents.
    pub async fn enable_hyper_automation_pna(
        &self,
        request: &AgentsActionRequest,
    ) -> Result<i64, Error> {
        let resp: Response<Affected> = self
            .client
            .http()
            .post("/web/api/v2.1/agents/enable-hyper-automation-pna", request)
            .await?;
        Ok(resp.data.affected)
    }
}
