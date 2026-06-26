//! Service for the `Inventory Device` tag — XDR asset inventory (device assets).
//!
//! Inventory Device Resources. Endpoints let you list device assets, run the
//! same query via `POST` (for large filter payloads that would exceed
//! querystring limits), perform actions on a selection of assets, discover
//! which actions are available for a selection (with their enabled/disabled
//! status), and export the filtered inventory to CSV/JSON.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_device::{AvailableActionWithStatus, Device};
use crate::pagination::{Paginated, Response};

/// `Inventory Device` tag — XDR asset inventory for device assets.
pub struct InventoryDeviceService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// ---------------------------------------------------------------------------
// Helper: comma-join an iterator of string-likes for array query params.
// ---------------------------------------------------------------------------
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

// ===========================================================================
// Shared filter query params.
//
// The GET list, POST action, POST available-actions and GET export endpoints
// all share the same (large) filter set. The full superset is exposed here;
// fields a given endpoint does not use simply serialize to nothing. Every
// field is optional. Array params are serialized comma-joined (the API accepts
// repeated values as a single comma-separated string). Enum params are kept as
// `String` for forward-compatibility; allowed values are documented per field.
// ===========================================================================

/// Query (filter) params shared by the `Inventory Device` endpoints:
/// `GET /web/api/v2.1/xdr/assets/device`,
/// `POST /web/api/v2.1/xdr/assets/device/action`,
/// `POST /web/api/v2.1/xdr/assets/device/available-actions/with-status` and
/// `GET /web/api/v2.1/xdr/assets/device/export`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDeviceQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `critical`, `high`, `medium`, `low`, `--`.
    /// Array param serialized comma-joined.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The missing coverage for the asset. Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// Tag Keys (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// Tag Keys exists. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The memory of the device in human readable format. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Unresolved Alerts`, `High Value`.
    /// Array param serialized comma-joined.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor".. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The operating system of the device. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Free-text filter by the image name. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The subnets. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The ranger tags key. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name_nin: Option<String>,
    /// The active coverage for the asset (not in). Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    /// Array param serialized comma-joined.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The network name. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The manufacturer of the device (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer_nin: Option<String>,
    /// The Surface that each asset belongs to. Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The operating system name and version of the device (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in). Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    /// Array param serialized comma-joined.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Active`, `Inactive`.
    /// Array param serialized comma-joined.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The column to sort the results by.. Optional.
    /// Allowed values (enum, kept as `String` for forward-compat): `s1GroupName`, `memory`, `detectedFromSite`, `previousOsType`, `previousOsVersion`, `eppUnsupportedUnknown`, `s1OnboardedAccountName`, `category`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The site from which the device was detected. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// The agent supported or unknown state. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The asset review (not in). Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    /// Array param serialized comma-joined.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value_contains: Option<String>,
    /// Asset Contact Email (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The OS names and versions. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The domain. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    /// Array param serialized comma-joined.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The operating system of the device (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// The manufacturer. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// Asset Contact Email. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset. Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Unresolved Alerts`, `High Value`.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS \| Azure \| GCP \| Active Directory. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The Surface that each asset belongs to (not in). Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    /// Array param serialized comma-joined.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The ranger tags key (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_nin: Option<String>,
    /// The ID. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// The Asset Type. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// Tags. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction. Optional.
    /// Allowed values (enum, kept as `String` for forward-compat): `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The ranger tags key value. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects.. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Name. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The MAC addresses. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The site from which the device was detected (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site_nin: Option<String>,
    /// The name. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The operating system version of the device. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The criticality that each asset belongs to. Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `critical`, `high`, `medium`, `low`, `--`.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The first seen date. Optional.
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The last update date. Optional.
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt_between: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time.. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// The discovery methods (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods_nin: Option<String>,
    /// User and cloud tag keys not exists. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The manufacturer of the device. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// List of Site IDs to filter by. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS \| Azure \| GCP \| Active Directory (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The number of cores. Optional.
    /// Array param serialized comma-joined (items are int32).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in). Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    /// Array param serialized comma-joined.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The discovery methods. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset. Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Infected`, `Healthy`.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset. Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in). Optional.
    /// Array param serialized comma-joined (items are int32).
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_contains: Option<String>,
    /// The ranger tags key value (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value_nin: Option<String>,
    /// The architecture of the device (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// The sub-category that each resource belongs to. Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The OS versions. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review. Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The hostnames. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The architecture of the device. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The ID. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset. Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Active`, `Inactive`.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown_nin: Option<String>,
    /// The domain of the device (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// Tags (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The domain of the device. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The operating system family of the device (not in). Optional.
    /// Array param serialized comma-joined.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The status alerts of the asset (not in). Optional.
    /// Allowed item values (enum, kept as `String` for forward-compat): `Infected`, `Healthy`.
    /// Array param serialized comma-joined.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device. Optional.
    /// Array param serialized comma-joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl InventoryDeviceQuery {
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
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(join_csv(v));
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
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(join_csv(v));
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
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor"..
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
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
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(join_csv(v));
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
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(join_csv(v));
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
    /// The Surface that each asset belongs to.
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(join_csv(v));
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
    /// The status of the asset (not in).
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(join_csv(v));
        self
    }
    /// The column to sort the results by..
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
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
    /// The agent supported or unknown state.
    pub fn epp_unsupported_unknown<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown = Some(join_csv(v));
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
    /// Free-text filter by Ranger tag key value (supports multiple values).
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
    /// The manufacturer.
    pub fn manufacturer_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer_contains = Some(join_csv(v));
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
    /// The environment that the asset exists in - AWS \| Azure \| GCP \| Active Directory.
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
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(join_csv(v));
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
    /// The ID.
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(join_csv(v));
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
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(join_csv(v));
        self
    }
    /// Sort direction.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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
    /// If true, only total number of items will be returned, without any of the actual objects..
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items..
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
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
    /// The MAC addresses.
    pub fn mac_addresses_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses_contains = Some(join_csv(v));
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
    /// The operating system version of the device.
    pub fn os_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(join_csv(v));
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
    /// If true, total number of items will not be calculated, which speeds up execution time..
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
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
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(join_csv(v));
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
    /// The environment that the asset exists in - AWS \| Azure \| GCP \| Active Directory (not in).
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(join_csv(v));
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
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
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
    /// The sub-category that each resource belongs to.
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(join_csv(v));
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
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
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
    /// The internal IPs.
    pub fn internal_ips_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips_contains = Some(join_csv(v));
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
    /// The domain of the device (not in).
    pub fn domain_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain_nin = Some(join_csv(v));
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
    /// The status alerts of the asset (not in).
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(join_csv(v));
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

/// Query params for `POST /web/api/v2.1/xdr/assets/device` (the POST variant of
/// the list endpoint). Only the account/site/group scoping params are accepted
/// on the querystring; the bulk of the filter travels in the request body.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDevicePostQuery {
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}
impl InventoryDevicePostQuery {
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
}

// ===========================================================================
// Request bodies.
// ===========================================================================

/// Body for `POST /web/api/v2.1/xdr/assets/device`
/// (`v2_1.inventory.device.schemas_DeviceViewInputSchema`).
///
/// `filter` is required and is a freeform object mirroring the GET query filter
/// set (`PaginatedDeviceFilter`); it is modelled as [`serde_json::Value`] to
/// carry the wide, dynamic filter shape losslessly.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDevicePostBody {
    /// Filter. Required. Freeform `PaginatedDeviceFilter` object.
    pub filter: serde_json::Value,
    /// Data. Optional / nullable -> `Option`, default null. Empty/strict object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Body for `POST /web/api/v2.1/xdr/assets/device/action`
/// (`v2_1.inventory.device.schemas_DeviceActionPayloadSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDeviceActionBody {
    /// Action name. Required.
    ///
    /// Allowed values (enum, kept as `String` for forward-compat):
    /// `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`, `add_note`,
    /// `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    pub action_name: String,
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000). Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

/// Body for
/// `POST /web/api/v2.1/xdr/assets/device/available-actions/with-status`
/// (`v2_1.inventory.schemas_AffectedResourcesSchema`).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryDeviceAffectedResourcesBody {
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000). Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

// ===========================================================================
// Service methods.
// ===========================================================================

impl InventoryDeviceService<'_> {
    /// **Assets** — Get assets.
    ///
    /// Get assets. Returns the paginated list of device assets in the XDR
    /// inventory, filtered by the supplied query.
    ///
    /// `GET /web/api/v2.1/xdr/assets/device`
    pub async fn list(&self, query: &InventoryDeviceQuery) -> Result<Paginated<Device>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/xdr/assets/device", q).await?)
    }

    /// **Assets using POST** — POST API to get Assets.
    ///
    /// POST API to get Assets. Equivalent to [`list`](Self::list) but accepts a
    /// large filter payload in the request body (to avoid querystring length
    /// limits). `accountIds`, `siteIds` and `groupIds` may also be supplied as
    /// query params.
    ///
    /// `POST /web/api/v2.1/xdr/assets/device`
    pub async fn list_post(
        &self,
        query: &InventoryDevicePostQuery,
        body: &InventoryDevicePostBody,
    ) -> Result<Paginated<Device>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(Method::POST, "/web/api/v2.1/xdr/assets/device", q, Some(body))
            .await?)
    }

    /// **Perform action** — Perform action on selected assets.
    ///
    /// Perform action on selected assets. The selection is described by the
    /// query filters (the same set as [`list`](Self::list)) and/or the body's
    /// `id__in` / `id__nin` lists.
    ///
    /// `POST /web/api/v2.1/xdr/assets/device/action`
    pub async fn perform_action(
        &self,
        query: &InventoryDeviceQuery,
        body: &InventoryDeviceActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(Method::POST, "/web/api/v2.1/xdr/assets/device/action", q, Some(body))
            .await?)
    }

    /// **Available actions** — Get cloud inventory device available-actions.
    ///
    /// Get the actions that can be performed on the selected assets, each
    /// annotated with its enabled/disabled status. The selection is described by
    /// the query filters (the same set as [`list`](Self::list)) and/or the
    /// body's `id__in` / `id__nin` lists.
    ///
    /// `POST /web/api/v2.1/xdr/assets/device/available-actions/with-status`
    pub async fn available_actions_with_status(
        &self,
        query: &InventoryDeviceQuery,
        body: &InventoryDeviceAffectedResourcesBody,
    ) -> Result<Response<AvailableActionWithStatus>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(
                Method::POST,
                "/web/api/v2.1/xdr/assets/device/available-actions/with-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// **Export assets to CSV or JSON** — Returns the results for a given
    /// inventory filter in a CSV or JSON format.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    /// `export_format` is required; the remaining filters match
    /// [`list`](Self::list).
    ///
    /// `GET /web/api/v2.1/xdr/assets/device/export`
    ///
    /// `export_format` allowed values (enum): `csv`, `json`.
    pub async fn export(
        &self,
        export_format: impl Into<String>,
        query: &InventoryDeviceQuery,
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
        Ok(self.client.http().get("/web/api/v2.1/xdr/assets/device/export", q).await?)
    }
}
