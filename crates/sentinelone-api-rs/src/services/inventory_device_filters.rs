//! `Inventory Device Filters` tag — Inventory Device Resource Filters.
//!
//! Three endpoints under `/web/api/v2.1/xdr/assets/device/filters`:
//! `autocomplete`, `count`, and `free-text`.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_device_filters::{
    AutoCompleteResponse, CountFiltersResponse, FreeTextFilterResponse,
};
use crate::pagination::{Paginated, Response};

/// Joins an iterator of string-likes into a single comma-separated value, as the
/// SentinelOne API expects for `array<...>` query parameters.
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

/// Service for the `Inventory Device Filters` tag (Inventory Device Resource Filters).
pub struct InventoryDeviceFiltersService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/xdr/assets/device/filters/autocomplete`.
///
/// Array params are serialized comma-joined, as the API expects. `key` and
/// `text` are required.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteQuery {
    /// Search field key. Required. Allowed values: `resourceType__contains`,
    /// `id__contains`, `name__contains`, `tagsKey__contains`,
    /// `tagsKeyValue__contains`, `domain__contains`, `gatewayMacs__contains`,
    /// `gatewayIps__contains`.
    pub key: String,
    /// Search term text. Required.
    pub text: String,
    /// Free-text filter by tag key (supports multiple values). `array<string>`. Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). `array<enum>`. Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The missing coverage for the asset. `array<enum>`. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// Tag Keys (not in). `array<string>`. Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// Tag Keys exists. `array<string>`. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The memory of the device in human readable format. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses. `array<string>`. Optional.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). `array<string>`. Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). `array<enum>`. Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. `array<string>`. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The operating system of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Free-text filter by the image name. `array<string>`. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The subnets. `array<string>`. Optional.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The ranger tags key. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in). `array<string>`. Optional.
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name_nin: Option<String>,
    /// The active coverage for the asset (not in). `array<enum>`. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// User and cloud tags (not in). `array<string>`. Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The network name. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// User and cloud tag keys (not in). `array<string>`. Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs. `array<string>`. Optional.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The manufacturer of the device (not in). `array<string>`. Optional.
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer_nin: Option<String>,
    /// The Surface that each asset belongs to. `array<enum>`. Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The operating system name and version of the device (not in). `array<string>`. Optional.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in). `array<enum>`. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). `array<enum>`. Optional.
    /// Allowed values: `Active`, `Inactive`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The site from which the device was detected. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// The agent supported or unknown state. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The asset review (not in). `array<enum>`. Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values). `array<string>`. Optional.
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value_contains: Option<String>,
    /// Asset Contact Email (not in). `array<string>`. Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The OS names and versions. `array<string>`. Optional.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The domain. `array<string>`. Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). `array<enum>`. Optional.
    /// Allowed values include: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs. `array<string>`. Optional.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The operating system of the device (not in). `array<string>`. Optional.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// The manufacturer. `array<string>`. Optional.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// Asset Contact Email. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset. `array<enum>`. Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory.
    /// `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The Surface that each asset belongs to (not in). `array<enum>`. Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The ranger tags key (not in). `array<string>`. Optional.
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_nin: Option<String>,
    /// The ID. `array<string>`. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// The Asset Type. `array<string>`. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in). `array<string>`. Optional.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// Tags. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The ranger tags key value. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// Name. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The MAC addresses. `array<string>`. Optional.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The site from which the device was detected (not in). `array<string>`. Optional.
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site_nin: Option<String>,
    /// The name. `array<string>`. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The operating system version of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The criticality that each asset belongs to. `array<enum>`. Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The first seen date. `string`. Optional.
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The last update date. `string`. Optional.
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt_between: Option<String>,
    /// The discovery methods (not in). `array<string>`. Optional.
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods_nin: Option<String>,
    /// User and cloud tag keys not exists. `array<string>`. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The manufacturer of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// List of Site IDs to filter by. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. `string`. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. `array<enum>`. Optional.
    /// Allowed values include: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    /// (not in). `array<string>`. Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). `array<string>`. Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The number of cores. `array<integer(int32)>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in). `array<string>`. Optional.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in). `array<enum>`. Optional.
    /// Allowed values include: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The discovery methods. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset. `array<enum>`. Optional.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset. `array<enum>`. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in). `array<integer(int32)>`. Optional.
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by. `integer(int32)`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values). `array<string>`. Optional.
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_contains: Option<String>,
    /// The ranger tags key value (not in). `array<string>`. Optional.
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value_nin: Option<String>,
    /// The architecture of the device (not in). `array<string>`. Optional.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// The sub-category that each resource belongs to. `array<enum>`. Optional.
    /// Allowed values include: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The OS versions. `array<string>`. Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review. `array<enum>`. Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The hostnames. `array<string>`. Optional.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// Limit number of returned items. `integer(int32)`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The architecture of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The ID. `array<string>`. Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs. `array<string>`. Optional.
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset. `array<enum>`. Optional.
    /// Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in). `array<string>`. Optional.
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown_nin: Option<String>,
    /// The domain of the device (not in). `array<string>`. Optional.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// Tags (not in). `array<string>`. Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. `array<string>`. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The domain of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The operating system family of the device (not in). `array<string>`. Optional.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The status alerts of the asset (not in). `array<enum>`. Optional.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl AutoCompleteQuery {
    /// Creates a new query with the two required params: `key` (search field key)
    /// and `text` (search term text).
    pub fn new(key: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            text: text.into(),
            ..Default::default()
        }
    }
    /// Sets `key` — search field key. Allowed values: `resourceType__contains`,
    /// `id__contains`, `name__contains`, `tagsKey__contains`,
    /// `tagsKeyValue__contains`, `domain__contains`, `gatewayMacs__contains`,
    /// `gatewayIps__contains`.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = key.into();
        self
    }
    /// Sets `text` — search term text.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to (not in). Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`.
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset. Allowed values: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
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
    /// The risk factors associated with the asset (not in). Allowed values:
    /// `Unresolved Alerts`, `High Value`.
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
    /// The active coverage for the asset (not in). Allowed values: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
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
    /// The Surface that each asset belongs to. Allowed values: `Cloud`, `Identity`,
    /// `Network`, `Endpoint`, `Network Discovery`.
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
    /// The missing coverage for the asset (not in). Allowed values: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(join_csv(v));
        self
    }
    /// The status of the asset (not in). Allowed values: `Active`, `Inactive`.
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(join_csv(v));
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
    /// The asset review (not in). Allowed values: `Not Reviewed`, `Under Analysis`,
    /// `Not Trusted`, `Allowed`, `` (empty).
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
    /// The canonical name for the resource type (not in). Allowed values include:
    /// `Access Control and Surveillance System`, `Access Point`, `AD Certificate`,
    /// `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`,
    /// `AD DNS Zone`, `AD Domain`.
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
    /// The risk factors associated with the asset. Allowed values: `Unresolved Alerts`,
    /// `High Value`.
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(v));
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
    /// The operating system family of the device.
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to (not in). Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
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
    /// The ranger tags key value.
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(join_csv(v));
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
    /// The criticality that each asset belongs to. Allowed values: `critical`, `high`,
    /// `medium`, `low`, `--`.
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
    /// The canonical name for the resource type. Allowed values include:
    /// `Access Control and Surveillance System`, `Access Point`, `AD Certificate`,
    /// `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`,
    /// `AD DNS Zone`, `AD Domain`.
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    /// (not in).
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
    /// The number of cores. `array<integer(int32)>`.
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
    /// The sub-category that each resource belongs to (not in). Allowed values include:
    /// `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`.
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
    /// The status alerts of the asset. Allowed values: `Infected`, `Healthy`.
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
    /// The active coverage for the asset. Allowed values: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
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
    /// The number of cores (not in). `array<integer(int32)>`.
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
    /// The sub-category that each resource belongs to. Allowed values include: `All`,
    /// `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`.
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
    /// The asset review. Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty).
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
    /// The status of the asset. Allowed values: `Active`, `Inactive`.
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
    /// The status alerts of the asset (not in). Allowed values: `Infected`, `Healthy`.
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

/// Query params for `GET /web/api/v2.1/xdr/assets/device/filters/count`.
///
/// Array params are serialized comma-joined, as the API expects. All params are
/// optional for this endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountQuery {
    /// Free-text filter by tag key (supports multiple values). `array<string>`. Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). `array<enum>`. Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The missing coverage for the asset. `array<enum>`. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// Tag Keys (not in). `array<string>`. Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// Tag Keys exists. `array<string>`. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The memory of the device in human readable format. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses. `array<string>`. Optional.
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address_contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). `array<string>`. Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// Tag Keys. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). `array<enum>`. Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. `array<string>`. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The operating system of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Free-text filter by the image name. `array<string>`. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// The subnets. `array<string>`. Optional.
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets_contains: Option<String>,
    /// The ranger tags key. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in). `array<string>`. Optional.
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name_nin: Option<String>,
    /// The active coverage for the asset (not in). `array<enum>`. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// User and cloud tags (not in). `array<string>`. Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The network name. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// User and cloud tag keys (not in). `array<string>`. Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The gateway IPs. `array<string>`. Optional.
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips_contains: Option<String>,
    /// The manufacturer of the device (not in). `array<string>`. Optional.
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer_nin: Option<String>,
    /// The Surface that each asset belongs to. `array<enum>`. Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The operating system name and version of the device (not in). `array<string>`. Optional.
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version_nin: Option<String>,
    /// The missing coverage for the asset (not in). `array<enum>`. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). `array<enum>`. Optional.
    /// Allowed values: `Active`, `Inactive`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The site from which the device was detected. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// The agent supported or unknown state. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The asset review (not in). `array<enum>`. Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values). `array<string>`. Optional.
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value_contains: Option<String>,
    /// Asset Contact Email (not in). `array<string>`. Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The OS names and versions. `array<string>`. Optional.
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version_contains: Option<String>,
    /// The domain. `array<string>`. Optional.
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain_contains: Option<String>,
    /// The canonical name for the resource type (not in). `array<enum>`. Optional.
    /// Allowed values include: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The gateway MACs. `array<string>`. Optional.
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs_contains: Option<String>,
    /// The operating system of the device (not in). `array<string>`. Optional.
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os_nin: Option<String>,
    /// The manufacturer. `array<string>`. Optional.
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer_contains: Option<String>,
    /// Asset Contact Email. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset. `array<enum>`. Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory.
    /// `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The Surface that each asset belongs to (not in). `array<enum>`. Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The ranger tags key (not in). `array<string>`. Optional.
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_nin: Option<String>,
    /// The ID. `array<string>`. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// The Asset Type. `array<string>`. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The memory of the device in human readable format (not in). `array<string>`. Optional.
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable_nin: Option<String>,
    /// Tags. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The ranger tags key value. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// Name. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The MAC addresses. `array<string>`. Optional.
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses_contains: Option<String>,
    /// The site from which the device was detected (not in). `array<string>`. Optional.
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site_nin: Option<String>,
    /// The name. `array<string>`. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The operating system version of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The criticality that each asset belongs to. `array<enum>`. Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The first seen date. `string`. Optional.
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt_between: Option<String>,
    /// The last update date. `string`. Optional.
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt_between: Option<String>,
    /// The discovery methods (not in). `array<string>`. Optional.
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods_nin: Option<String>,
    /// User and cloud tag keys not exists. `array<string>`. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// The manufacturer of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// List of Site IDs to filter by. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. `string`. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. `array<enum>`. Optional.
    /// Allowed values include: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    /// (not in). `array<string>`. Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). `array<string>`. Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The number of cores. `array<integer(int32)>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in). `array<string>`. Optional.
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version_nin: Option<String>,
    /// The sub-category that each resource belongs to (not in). `array<enum>`. Optional.
    /// Allowed values include: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The discovery methods. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset. `array<enum>`. Optional.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset. `array<enum>`. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in). `array<integer(int32)>`. Optional.
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count_nin: Option<String>,
    /// The ID of the CSV file to filter by. `integer(int32)`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values). `array<string>`. Optional.
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_contains: Option<String>,
    /// The ranger tags key value (not in). `array<string>`. Optional.
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value_nin: Option<String>,
    /// The architecture of the device (not in). `array<string>`. Optional.
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture_nin: Option<String>,
    /// The sub-category that each resource belongs to. `array<enum>`. Optional.
    /// Allowed values include: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The OS versions. `array<string>`. Optional.
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version_contains: Option<String>,
    /// The asset review. `array<enum>`. Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The hostnames. `array<string>`. Optional.
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames_contains: Option<String>,
    /// The architecture of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The ID. `array<string>`. Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The internal IPs. `array<string>`. Optional.
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips_contains: Option<String>,
    /// The status of the asset. `array<enum>`. Optional.
    /// Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in). `array<string>`. Optional.
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown_nin: Option<String>,
    /// The domain of the device (not in). `array<string>`. Optional.
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain_nin: Option<String>,
    /// Tags (not in). `array<string>`. Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. `array<string>`. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The domain of the device. `array<string>`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The operating system family of the device (not in). `array<string>`. Optional.
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family_nin: Option<String>,
    /// The status alerts of the asset (not in). `array<enum>`. Optional.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The operating system name and version of the device. `array<string>`. Optional.
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
    /// The criticality that each asset belongs to (not in). Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`.
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset. Allowed values: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
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
    /// The risk factors associated with the asset (not in). Allowed values:
    /// `Unresolved Alerts`, `High Value`.
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
    /// The active coverage for the asset (not in). Allowed values: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
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
    /// The Surface that each asset belongs to. Allowed values: `Cloud`, `Identity`,
    /// `Network`, `Endpoint`, `Network Discovery`.
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
    /// The missing coverage for the asset (not in). Allowed values: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(join_csv(v));
        self
    }
    /// The status of the asset (not in). Allowed values: `Active`, `Inactive`.
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(join_csv(v));
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
    /// The asset review (not in). Allowed values: `Not Reviewed`, `Under Analysis`,
    /// `Not Trusted`, `Allowed`, `` (empty).
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
    /// The canonical name for the resource type (not in). Allowed values include:
    /// `Access Control and Surveillance System`, `Access Point`, `AD Certificate`,
    /// `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`,
    /// `AD DNS Zone`, `AD Domain`.
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
    /// The risk factors associated with the asset. Allowed values: `Unresolved Alerts`,
    /// `High Value`.
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(v));
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
    /// The operating system family of the device.
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to (not in). Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
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
    /// The ranger tags key value.
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(join_csv(v));
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
    /// The criticality that each asset belongs to. Allowed values: `critical`, `high`,
    /// `medium`, `low`, `--`.
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
    /// The canonical name for the resource type. Allowed values include:
    /// `Access Control and Surveillance System`, `Access Point`, `AD Certificate`,
    /// `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`,
    /// `AD DNS Zone`, `AD Domain`.
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    /// (not in).
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
    /// The number of cores. `array<integer(int32)>`.
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
    /// The sub-category that each resource belongs to (not in). Allowed values include:
    /// `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`.
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
    /// The status alerts of the asset. Allowed values: `Infected`, `Healthy`.
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
    /// The active coverage for the asset. Allowed values: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
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
    /// The number of cores (not in). `array<integer(int32)>`.
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
    /// The sub-category that each resource belongs to. Allowed values include: `All`,
    /// `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`.
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
    /// The asset review. Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty).
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
    /// The status of the asset. Allowed values: `Active`, `Inactive`.
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
    /// The status alerts of the asset (not in). Allowed values: `Infected`, `Healthy`.
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

impl InventoryDeviceFiltersService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/device/filters/autocomplete` — Auto Complete.
    ///
    /// Use this command to get values for other fields. When you send this command
    /// with input text and a field name, it returns auto-complete suggestions for
    /// the field.
    ///
    /// `key` (search field key) and `text` (search term text) are required and are
    /// set via [`AutoCompleteQuery::new`].
    pub async fn autocomplete(
        &self,
        query: &AutoCompleteQuery,
    ) -> Result<Response<AutoCompleteResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/device/filters/autocomplete", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/device/filters/count` — Filter counts.
    ///
    /// Get filter counts.
    pub async fn count(
        &self,
        query: &CountQuery,
    ) -> Result<Paginated<CountFiltersResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/device/filters/count", q)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/device/filters/free-text` — Free text filters.
    ///
    /// Get free text filters. This endpoint takes no parameters.
    pub async fn free_text(&self) -> Result<Paginated<FreeTextFilterResponse>, Error> {
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/device/filters/free-text", None)
            .await?)
    }
}
