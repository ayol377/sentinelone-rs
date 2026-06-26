//! Service for the `Inventory Network Discovery Surface` tag.
//!
//! Inventory Network Discovery Surface Resources. Implements all 4 endpoints
//! under `/web/api/v2.1/xdr/assets/surface/networkDiscovery`.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_network_discovery_surface::{
    NetworkDiscovery, NetworkDiscoveryAvailableActions,
};
use crate::pagination::{Paginated, Response};

/// `Inventory Network Discovery Surface` tag.
///
/// Inventory Network Discovery Surface Resources.
pub struct InventoryNetworkDiscoverySurfaceService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/xdr/assets/surface/networkDiscovery`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
    /// Free-text filter by tag key (supports multiple values)
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key__contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Spec enum values: critical, high, medium, low, --.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality__nin: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// Tag Keys (not in)
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key__nin: Option<String>,
    /// Tag Keys exists
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key__exists: Option<String>,
    /// The memory of the device in human readable format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address__contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value__contains: Option<String>,
    /// Tag Keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Spec enum values: Unresolved Alerts, High Value.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key__nexists: Option<String>,
    /// The operating system of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Free-text filter by the image name
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name__contains: Option<String>,
    /// The subnets
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets__contains: Option<String>,
    /// The ranger tags key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in)
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name__nin: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage__nin: Option<String>,
    /// User and cloud tags (not in)
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value__nin: Option<String>,
    /// The network name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// User and cloud tag keys (not in)
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nin: Option<String>,
    /// The gateway IPs
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips__contains: Option<String>,
    /// The manufacturer of the device (not in)
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer__nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Spec enum values: Cloud, Identity, Network, Endpoint, Network Discovery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The operating system name and version of the device (not in)
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version__nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage__nin: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Spec enum values: Active, Inactive.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status__nin: Option<String>,
    /// The site from which the device was detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// The agent supported or unknown state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The category that each resource belongs to
    ///
    /// Spec enum values: All, Account, AI ML, Application Integration, Cloud Application, Code, Container, Data Analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The asset review (not in)
    ///
    /// Spec enum values: Not Reviewed, Under Analysis, Not Trusted, Allowed.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review__nin: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values)
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value__contains: Option<String>,
    /// Asset Contact Email (not in)
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email__nin: Option<String>,
    /// The severity of the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The OS names and versions
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version__contains: Option<String>,
    /// The domain
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain__contains: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Spec enum values: Access Control and Surveillance System, Access Point, AD Certificate, AD Certificate Authority, AD Certificate Template, AD Containers, AD DNS Zone, AD Domain.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type__nin: Option<String>,
    /// The gateway MACs
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs__contains: Option<String>,
    /// The operating system of the device (not in)
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os__nin: Option<String>,
    /// The manufacturer
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer__contains: Option<String>,
    /// Asset Contact Email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Spec enum values: Unresolved Alerts, High Value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Spec enum values: Cloud, Identity, Network, Endpoint, Network Discovery.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces__nin: Option<String>,
    /// The ranger tags key (not in)
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key__nin: Option<String>,
    /// The ID
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id__in: Option<String>,
    /// The Asset Type
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type__contains: Option<String>,
    /// The memory of the device in human readable format (not in)
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable__nin: Option<String>,
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The ranger tags key value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The MAC addresses
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses__contains: Option<String>,
    /// The site from which the device was detected (not in)
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site__nin: Option<String>,
    /// The name
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
    /// The operating system version of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Spec enum values: critical, high, medium, low, --.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The first seen date
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt__between: Option<String>,
    /// The last update date
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt__between: Option<String>,
    /// The discovery methods (not in)
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods__nin: Option<String>,
    /// User and cloud tag keys not exists
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nexists: Option<String>,
    /// The manufacturer of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// List of Site IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Spec enum values: Access Control and Surveillance System, Access Point, AD Certificate, AD Certificate Authority, AD Certificate Template, AD Containers, AD DNS Zone, AD Domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment__nin: Option<String>,
    /// Name (not in)
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names__nin: Option<String>,
    /// The number of cores
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in)
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version__nin: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Spec enum values: All, Access Key and Secret, Access Management, Account, Account Group, AD Objects, Administrative Unit, Admission Controller.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category__nin: Option<String>,
    /// The discovery methods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset
    ///
    /// Spec enum values: Infected, Healthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in)
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count__nin: Option<String>,
    /// The ID of the CSV file to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values)
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key__contains: Option<String>,
    /// The ranger tags key value (not in)
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value__nin: Option<String>,
    /// The architecture of the device (not in)
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture__nin: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Spec enum values: All, Access Key and Secret, Access Management, Account, Account Group, AD Objects, Administrative Unit, Admission Controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The OS versions
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// The asset review
    ///
    /// Spec enum values: Not Reviewed, Under Analysis, Not Trusted, Allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The hostnames
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames__contains: Option<String>,
    /// The architecture of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The ID
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id__contains: Option<String>,
    /// The internal IPs
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips__contains: Option<String>,
    /// The status of the asset
    ///
    /// Spec enum values: Active, Inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in)
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown__nin: Option<String>,
    /// The domain of the device (not in)
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain__nin: Option<String>,
    /// Tags (not in)
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value__nin: Option<String>,
    /// User and cloud tag keys exists
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__exists: Option<String>,
    /// The domain of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The operating system family of the device (not in)
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family__nin: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Spec enum values: Infected, Healthy.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status__nin: Option<String>,
    /// The operating system name and version of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items, use "cursor".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The column to sort the results by.
    ///
    /// Spec enum values: s1GroupName, cpu, legacyIdentityPolicyName, previousOsType, previousOsVersion, agentFirewallStatus, s1UpdatedAt, memoryReadable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction.
    ///
    /// Spec enum values: asc, desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListQuery {
    /// Set `tagsKey__contains` (array param, comma-joined).
    pub fn tags_key__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality__nin` (array param, comma-joined).
    pub fn asset_criticality__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage` (array param, comma-joined).
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue` (array param, comma-joined).
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nin` (array param, comma-joined).
    pub fn tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__exists` (array param, comma-joined).
    pub fn tags_key__exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__exists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `memoryReadable` (array param, comma-joined).
    pub fn memory_readable<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `ipAddress__contains` (array param, comma-joined).
    pub fn ip_address__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__contains` (array param, comma-joined).
    pub fn tags_key_value__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey` (array param, comma-joined).
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors__nin` (array param, comma-joined).
    pub fn risk_factors__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nexists` (array param, comma-joined).
    pub fn tags_key__nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nexists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `os` (array param, comma-joined).
    pub fn os<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `groupIds` (array param, comma-joined).
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `imageName__contains` (array param, comma-joined).
    pub fn image_name__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subnets__contains` (array param, comma-joined).
    pub fn subnets__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKey` (array param, comma-joined).
    pub fn ranger_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `networkName__nin` (array param, comma-joined).
    pub fn network_name__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage__nin` (array param, comma-joined).
    pub fn active_coverage__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue__nin` (array param, comma-joined).
    pub fn all_tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `networkName` (array param, comma-joined).
    pub fn network_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nin` (array param, comma-joined).
    pub fn all_tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `gatewayIps__contains` (array param, comma-joined).
    pub fn gateway_ips__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer__nin` (array param, comma-joined).
    pub fn manufacturer__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces` (array param, comma-joined).
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion__nin` (array param, comma-joined).
    pub fn os_name_version__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage__nin` (array param, comma-joined).
    pub fn missing_coverage__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus__nin` (array param, comma-joined).
    pub fn asset_status__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `detectedFromSite` (array param, comma-joined).
    pub fn detected_from_site<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `eppUnsupportedUnknown` (array param, comma-joined).
    pub fn epp_unsupported_unknown<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `category` (array param, comma-joined).
    pub fn category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.category = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview__nin` (array param, comma-joined).
    pub fn device_review__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagKeyValue__contains` (array param, comma-joined).
    pub fn ranger_tag_key_value__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_value__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail__nin` (array param, comma-joined).
    pub fn asset_contact_email__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `alertSeverity` (array param, comma-joined).
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `accountIds` (array param, comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion__contains` (array param, comma-joined).
    pub fn os_name_version__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain__contains` (array param, comma-joined).
    pub fn domain__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__nin` (array param, comma-joined).
    pub fn resource_type__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `gatewayMacs__contains` (array param, comma-joined).
    pub fn gateway_macs__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `os__nin` (array param, comma-joined).
    pub fn os__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer__contains` (array param, comma-joined).
    pub fn manufacturer__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail` (array param, comma-joined).
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `udpPorts` (array param, comma-joined).
    pub fn udp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors` (array param, comma-joined).
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetEnvironment` (array param, comma-joined).
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osFamily` (array param, comma-joined).
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces__nin` (array param, comma-joined).
    pub fn surfaces__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKey__nin` (array param, comma-joined).
    pub fn ranger_tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__in` (array param, comma-joined).
    pub fn id__in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__in = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__contains` (array param, comma-joined).
    pub fn resource_type__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `memoryReadable__nin` (array param, comma-joined).
    pub fn memory_readable__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue` (array param, comma-joined).
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKeyValue` (array param, comma-joined).
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names` (array param, comma-joined).
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `macAddresses__contains` (array param, comma-joined).
    pub fn mac_addresses__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `detectedFromSite__nin` (array param, comma-joined).
    pub fn detected_from_site__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `name__contains` (array param, comma-joined).
    pub fn name__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion` (array param, comma-joined).
    pub fn os_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality` (array param, comma-joined).
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `firstSeenDt__between`.
    pub fn first_seen_dt__between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt__between = Some(v.into());
        self
    }
    /// Set `lastUpdateDt__between`.
    pub fn last_update_dt__between(mut self, v: impl Into<String>) -> Self {
        self.last_update_dt__between = Some(v.into());
        self
    }
    /// Set `discoveryMethods__nin` (array param, comma-joined).
    pub fn discovery_methods__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nexists` (array param, comma-joined).
    pub fn all_tags_key__nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nexists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer` (array param, comma-joined).
    pub fn manufacturer<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `siteIds` (array param, comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `s1UpdatedAt__between`.
    pub fn s1_updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at__between = Some(v.into());
        self
    }
    /// Set `resourceType` (array param, comma-joined).
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetEnvironment__nin` (array param, comma-joined).
    pub fn asset_environment__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names__nin` (array param, comma-joined).
    pub fn names__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `coreCount` (array param, comma-joined).
    pub fn core_count<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion__nin` (array param, comma-joined).
    pub fn os_version__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subCategory__nin` (array param, comma-joined).
    pub fn sub_category__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `discoveryMethods` (array param, comma-joined).
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus` (array param, comma-joined).
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tcpPorts` (array param, comma-joined).
    pub fn tcp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage` (array param, comma-joined).
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `countsFor` (array param, comma-joined).
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `coreCount__nin` (array param, comma-joined).
    pub fn core_count__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `csvFilterId`.
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// Set `rangerTagKey__contains` (array param, comma-joined).
    pub fn ranger_tag_key__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKeyValue__nin` (array param, comma-joined).
    pub fn ranger_tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `architecture__nin` (array param, comma-joined).
    pub fn architecture__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subCategory` (array param, comma-joined).
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion__contains` (array param, comma-joined).
    pub fn os_version__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview` (array param, comma-joined).
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey` (array param, comma-joined).
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `hostnames__contains` (array param, comma-joined).
    pub fn hostnames__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `architecture` (array param, comma-joined).
    pub fn architecture<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__contains` (array param, comma-joined).
    pub fn id__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `internalIps__contains` (array param, comma-joined).
    pub fn internal_ips__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus` (array param, comma-joined).
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `eppUnsupportedUnknown__nin` (array param, comma-joined).
    pub fn epp_unsupported_unknown__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain__nin` (array param, comma-joined).
    pub fn domain__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__nin` (array param, comma-joined).
    pub fn tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__exists` (array param, comma-joined).
    pub fn all_tags_key__exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__exists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain` (array param, comma-joined).
    pub fn domain<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osFamily__nin` (array param, comma-joined).
    pub fn os_family__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus__nin` (array param, comma-joined).
    pub fn infection_status__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion` (array param, comma-joined).
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `skip`.
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
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
    /// Set `countOnly`.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Set `cursor`.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Set `skipCount`.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Set `limit`.
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/surface/networkDiscovery/action`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionQuery {
    /// Free-text filter by tag key (supports multiple values)
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key__contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Spec enum values: critical, high, medium, low, --.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality__nin: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// Tag Keys (not in)
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key__nin: Option<String>,
    /// Tag Keys exists
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key__exists: Option<String>,
    /// The memory of the device in human readable format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address__contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value__contains: Option<String>,
    /// Tag Keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Spec enum values: Unresolved Alerts, High Value.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key__nexists: Option<String>,
    /// The operating system of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Free-text filter by the image name
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name__contains: Option<String>,
    /// The subnets
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets__contains: Option<String>,
    /// The ranger tags key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in)
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name__nin: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage__nin: Option<String>,
    /// User and cloud tags (not in)
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value__nin: Option<String>,
    /// The network name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// User and cloud tag keys (not in)
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nin: Option<String>,
    /// The gateway IPs
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips__contains: Option<String>,
    /// The manufacturer of the device (not in)
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer__nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Spec enum values: Cloud, Identity, Network, Endpoint, Network Discovery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The operating system name and version of the device (not in)
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version__nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage__nin: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Spec enum values: Active, Inactive.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status__nin: Option<String>,
    /// The site from which the device was detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// The agent supported or unknown state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The category that each resource belongs to
    ///
    /// Spec enum values: All, Account, AI ML, Application Integration, Cloud Application, Code, Container, Data Analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The asset review (not in)
    ///
    /// Spec enum values: Not Reviewed, Under Analysis, Not Trusted, Allowed.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review__nin: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values)
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value__contains: Option<String>,
    /// Asset Contact Email (not in)
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email__nin: Option<String>,
    /// The severity of the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The OS names and versions
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version__contains: Option<String>,
    /// The domain
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain__contains: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Spec enum values: Access Control and Surveillance System, Access Point, AD Certificate, AD Certificate Authority, AD Certificate Template, AD Containers, AD DNS Zone, AD Domain.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type__nin: Option<String>,
    /// The gateway MACs
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs__contains: Option<String>,
    /// The operating system of the device (not in)
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os__nin: Option<String>,
    /// The manufacturer
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer__contains: Option<String>,
    /// Asset Contact Email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Spec enum values: Unresolved Alerts, High Value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Spec enum values: Cloud, Identity, Network, Endpoint, Network Discovery.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces__nin: Option<String>,
    /// The ranger tags key (not in)
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key__nin: Option<String>,
    /// The ID
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id__in: Option<String>,
    /// The Asset Type
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type__contains: Option<String>,
    /// The memory of the device in human readable format (not in)
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable__nin: Option<String>,
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The ranger tags key value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The MAC addresses
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses__contains: Option<String>,
    /// The site from which the device was detected (not in)
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site__nin: Option<String>,
    /// The name
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
    /// The operating system version of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Spec enum values: critical, high, medium, low, --.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The first seen date
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt__between: Option<String>,
    /// The last update date
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt__between: Option<String>,
    /// The discovery methods (not in)
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods__nin: Option<String>,
    /// User and cloud tag keys not exists
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nexists: Option<String>,
    /// The manufacturer of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// List of Site IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Spec enum values: Access Control and Surveillance System, Access Point, AD Certificate, AD Certificate Authority, AD Certificate Template, AD Containers, AD DNS Zone, AD Domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment__nin: Option<String>,
    /// Name (not in)
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names__nin: Option<String>,
    /// The number of cores
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in)
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version__nin: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Spec enum values: All, Access Key and Secret, Access Management, Account, Account Group, AD Objects, Administrative Unit, Admission Controller.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category__nin: Option<String>,
    /// The discovery methods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset
    ///
    /// Spec enum values: Infected, Healthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in)
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count__nin: Option<String>,
    /// The ID of the CSV file to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values)
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key__contains: Option<String>,
    /// The ranger tags key value (not in)
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value__nin: Option<String>,
    /// The architecture of the device (not in)
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture__nin: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Spec enum values: All, Access Key and Secret, Access Management, Account, Account Group, AD Objects, Administrative Unit, Admission Controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The OS versions
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// The asset review
    ///
    /// Spec enum values: Not Reviewed, Under Analysis, Not Trusted, Allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The hostnames
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames__contains: Option<String>,
    /// The architecture of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The ID
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id__contains: Option<String>,
    /// The internal IPs
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips__contains: Option<String>,
    /// The status of the asset
    ///
    /// Spec enum values: Active, Inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in)
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown__nin: Option<String>,
    /// The domain of the device (not in)
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain__nin: Option<String>,
    /// Tags (not in)
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value__nin: Option<String>,
    /// User and cloud tag keys exists
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__exists: Option<String>,
    /// The domain of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The operating system family of the device (not in)
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family__nin: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Spec enum values: Infected, Healthy.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status__nin: Option<String>,
    /// The operating system name and version of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl PerformActionQuery {
    /// Set `tagsKey__contains` (array param, comma-joined).
    pub fn tags_key__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality__nin` (array param, comma-joined).
    pub fn asset_criticality__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage` (array param, comma-joined).
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue` (array param, comma-joined).
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nin` (array param, comma-joined).
    pub fn tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__exists` (array param, comma-joined).
    pub fn tags_key__exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__exists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `memoryReadable` (array param, comma-joined).
    pub fn memory_readable<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `ipAddress__contains` (array param, comma-joined).
    pub fn ip_address__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__contains` (array param, comma-joined).
    pub fn tags_key_value__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey` (array param, comma-joined).
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors__nin` (array param, comma-joined).
    pub fn risk_factors__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nexists` (array param, comma-joined).
    pub fn tags_key__nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nexists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `os` (array param, comma-joined).
    pub fn os<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `groupIds` (array param, comma-joined).
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `imageName__contains` (array param, comma-joined).
    pub fn image_name__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subnets__contains` (array param, comma-joined).
    pub fn subnets__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKey` (array param, comma-joined).
    pub fn ranger_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `networkName__nin` (array param, comma-joined).
    pub fn network_name__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage__nin` (array param, comma-joined).
    pub fn active_coverage__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue__nin` (array param, comma-joined).
    pub fn all_tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `networkName` (array param, comma-joined).
    pub fn network_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nin` (array param, comma-joined).
    pub fn all_tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `gatewayIps__contains` (array param, comma-joined).
    pub fn gateway_ips__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer__nin` (array param, comma-joined).
    pub fn manufacturer__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces` (array param, comma-joined).
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion__nin` (array param, comma-joined).
    pub fn os_name_version__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage__nin` (array param, comma-joined).
    pub fn missing_coverage__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus__nin` (array param, comma-joined).
    pub fn asset_status__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `detectedFromSite` (array param, comma-joined).
    pub fn detected_from_site<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `eppUnsupportedUnknown` (array param, comma-joined).
    pub fn epp_unsupported_unknown<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `category` (array param, comma-joined).
    pub fn category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.category = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview__nin` (array param, comma-joined).
    pub fn device_review__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagKeyValue__contains` (array param, comma-joined).
    pub fn ranger_tag_key_value__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_value__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail__nin` (array param, comma-joined).
    pub fn asset_contact_email__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `alertSeverity` (array param, comma-joined).
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `accountIds` (array param, comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion__contains` (array param, comma-joined).
    pub fn os_name_version__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain__contains` (array param, comma-joined).
    pub fn domain__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__nin` (array param, comma-joined).
    pub fn resource_type__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `gatewayMacs__contains` (array param, comma-joined).
    pub fn gateway_macs__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `os__nin` (array param, comma-joined).
    pub fn os__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer__contains` (array param, comma-joined).
    pub fn manufacturer__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail` (array param, comma-joined).
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `udpPorts` (array param, comma-joined).
    pub fn udp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors` (array param, comma-joined).
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetEnvironment` (array param, comma-joined).
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osFamily` (array param, comma-joined).
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces__nin` (array param, comma-joined).
    pub fn surfaces__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKey__nin` (array param, comma-joined).
    pub fn ranger_tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__in` (array param, comma-joined).
    pub fn id__in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__in = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__contains` (array param, comma-joined).
    pub fn resource_type__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `memoryReadable__nin` (array param, comma-joined).
    pub fn memory_readable__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue` (array param, comma-joined).
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKeyValue` (array param, comma-joined).
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names` (array param, comma-joined).
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `macAddresses__contains` (array param, comma-joined).
    pub fn mac_addresses__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `detectedFromSite__nin` (array param, comma-joined).
    pub fn detected_from_site__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `name__contains` (array param, comma-joined).
    pub fn name__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion` (array param, comma-joined).
    pub fn os_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality` (array param, comma-joined).
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `firstSeenDt__between`.
    pub fn first_seen_dt__between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt__between = Some(v.into());
        self
    }
    /// Set `lastUpdateDt__between`.
    pub fn last_update_dt__between(mut self, v: impl Into<String>) -> Self {
        self.last_update_dt__between = Some(v.into());
        self
    }
    /// Set `discoveryMethods__nin` (array param, comma-joined).
    pub fn discovery_methods__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nexists` (array param, comma-joined).
    pub fn all_tags_key__nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nexists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer` (array param, comma-joined).
    pub fn manufacturer<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `siteIds` (array param, comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `s1UpdatedAt__between`.
    pub fn s1_updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at__between = Some(v.into());
        self
    }
    /// Set `resourceType` (array param, comma-joined).
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetEnvironment__nin` (array param, comma-joined).
    pub fn asset_environment__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names__nin` (array param, comma-joined).
    pub fn names__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `coreCount` (array param, comma-joined).
    pub fn core_count<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion__nin` (array param, comma-joined).
    pub fn os_version__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subCategory__nin` (array param, comma-joined).
    pub fn sub_category__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `discoveryMethods` (array param, comma-joined).
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus` (array param, comma-joined).
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tcpPorts` (array param, comma-joined).
    pub fn tcp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage` (array param, comma-joined).
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `countsFor` (array param, comma-joined).
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `coreCount__nin` (array param, comma-joined).
    pub fn core_count__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `csvFilterId`.
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// Set `rangerTagKey__contains` (array param, comma-joined).
    pub fn ranger_tag_key__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKeyValue__nin` (array param, comma-joined).
    pub fn ranger_tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `architecture__nin` (array param, comma-joined).
    pub fn architecture__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subCategory` (array param, comma-joined).
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion__contains` (array param, comma-joined).
    pub fn os_version__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview` (array param, comma-joined).
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey` (array param, comma-joined).
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `hostnames__contains` (array param, comma-joined).
    pub fn hostnames__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `architecture` (array param, comma-joined).
    pub fn architecture<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__contains` (array param, comma-joined).
    pub fn id__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `internalIps__contains` (array param, comma-joined).
    pub fn internal_ips__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus` (array param, comma-joined).
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `eppUnsupportedUnknown__nin` (array param, comma-joined).
    pub fn epp_unsupported_unknown__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain__nin` (array param, comma-joined).
    pub fn domain__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__nin` (array param, comma-joined).
    pub fn tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__exists` (array param, comma-joined).
    pub fn all_tags_key__exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__exists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain` (array param, comma-joined).
    pub fn domain<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osFamily__nin` (array param, comma-joined).
    pub fn os_family__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus__nin` (array param, comma-joined).
    pub fn infection_status__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion` (array param, comma-joined).
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/surface/networkDiscovery/available-actions/with-status`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionsQuery {
    /// Free-text filter by tag key (supports multiple values)
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key__contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Spec enum values: critical, high, medium, low, --.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality__nin: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// Tag Keys (not in)
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key__nin: Option<String>,
    /// Tag Keys exists
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key__exists: Option<String>,
    /// The memory of the device in human readable format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address__contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value__contains: Option<String>,
    /// Tag Keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Spec enum values: Unresolved Alerts, High Value.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key__nexists: Option<String>,
    /// The operating system of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Free-text filter by the image name
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name__contains: Option<String>,
    /// The subnets
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets__contains: Option<String>,
    /// The ranger tags key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in)
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name__nin: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage__nin: Option<String>,
    /// User and cloud tags (not in)
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value__nin: Option<String>,
    /// The network name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// User and cloud tag keys (not in)
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nin: Option<String>,
    /// The gateway IPs
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips__contains: Option<String>,
    /// The manufacturer of the device (not in)
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer__nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Spec enum values: Cloud, Identity, Network, Endpoint, Network Discovery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The operating system name and version of the device (not in)
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version__nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage__nin: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Spec enum values: Active, Inactive.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status__nin: Option<String>,
    /// The site from which the device was detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// The agent supported or unknown state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The category that each resource belongs to
    ///
    /// Spec enum values: All, Account, AI ML, Application Integration, Cloud Application, Code, Container, Data Analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The asset review (not in)
    ///
    /// Spec enum values: Not Reviewed, Under Analysis, Not Trusted, Allowed.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review__nin: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values)
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value__contains: Option<String>,
    /// Asset Contact Email (not in)
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email__nin: Option<String>,
    /// The severity of the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The OS names and versions
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version__contains: Option<String>,
    /// The domain
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain__contains: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Spec enum values: Access Control and Surveillance System, Access Point, AD Certificate, AD Certificate Authority, AD Certificate Template, AD Containers, AD DNS Zone, AD Domain.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type__nin: Option<String>,
    /// The gateway MACs
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs__contains: Option<String>,
    /// The operating system of the device (not in)
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os__nin: Option<String>,
    /// The manufacturer
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer__contains: Option<String>,
    /// Asset Contact Email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Spec enum values: Unresolved Alerts, High Value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Spec enum values: Cloud, Identity, Network, Endpoint, Network Discovery.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces__nin: Option<String>,
    /// The ranger tags key (not in)
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key__nin: Option<String>,
    /// The ID
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id__in: Option<String>,
    /// The Asset Type
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type__contains: Option<String>,
    /// The memory of the device in human readable format (not in)
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable__nin: Option<String>,
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The ranger tags key value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The MAC addresses
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses__contains: Option<String>,
    /// The site from which the device was detected (not in)
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site__nin: Option<String>,
    /// The name
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
    /// The operating system version of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Spec enum values: critical, high, medium, low, --.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The first seen date
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt__between: Option<String>,
    /// The last update date
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt__between: Option<String>,
    /// The discovery methods (not in)
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods__nin: Option<String>,
    /// User and cloud tag keys not exists
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nexists: Option<String>,
    /// The manufacturer of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// List of Site IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Spec enum values: Access Control and Surveillance System, Access Point, AD Certificate, AD Certificate Authority, AD Certificate Template, AD Containers, AD DNS Zone, AD Domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment__nin: Option<String>,
    /// Name (not in)
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names__nin: Option<String>,
    /// The number of cores
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in)
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version__nin: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Spec enum values: All, Access Key and Secret, Access Management, Account, Account Group, AD Objects, Administrative Unit, Admission Controller.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category__nin: Option<String>,
    /// The discovery methods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset
    ///
    /// Spec enum values: Infected, Healthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in)
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count__nin: Option<String>,
    /// The ID of the CSV file to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values)
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key__contains: Option<String>,
    /// The ranger tags key value (not in)
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value__nin: Option<String>,
    /// The architecture of the device (not in)
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture__nin: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Spec enum values: All, Access Key and Secret, Access Management, Account, Account Group, AD Objects, Administrative Unit, Admission Controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The OS versions
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// The asset review
    ///
    /// Spec enum values: Not Reviewed, Under Analysis, Not Trusted, Allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The hostnames
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames__contains: Option<String>,
    /// The architecture of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The ID
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id__contains: Option<String>,
    /// The internal IPs
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips__contains: Option<String>,
    /// The status of the asset
    ///
    /// Spec enum values: Active, Inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in)
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown__nin: Option<String>,
    /// The domain of the device (not in)
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain__nin: Option<String>,
    /// Tags (not in)
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value__nin: Option<String>,
    /// User and cloud tag keys exists
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__exists: Option<String>,
    /// The domain of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The operating system family of the device (not in)
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family__nin: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Spec enum values: Infected, Healthy.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status__nin: Option<String>,
    /// The operating system name and version of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
}

impl AvailableActionsQuery {
    /// Set `tagsKey__contains` (array param, comma-joined).
    pub fn tags_key__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality__nin` (array param, comma-joined).
    pub fn asset_criticality__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage` (array param, comma-joined).
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue` (array param, comma-joined).
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nin` (array param, comma-joined).
    pub fn tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__exists` (array param, comma-joined).
    pub fn tags_key__exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__exists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `memoryReadable` (array param, comma-joined).
    pub fn memory_readable<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `ipAddress__contains` (array param, comma-joined).
    pub fn ip_address__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__contains` (array param, comma-joined).
    pub fn tags_key_value__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey` (array param, comma-joined).
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors__nin` (array param, comma-joined).
    pub fn risk_factors__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nexists` (array param, comma-joined).
    pub fn tags_key__nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nexists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `os` (array param, comma-joined).
    pub fn os<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `groupIds` (array param, comma-joined).
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `imageName__contains` (array param, comma-joined).
    pub fn image_name__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subnets__contains` (array param, comma-joined).
    pub fn subnets__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKey` (array param, comma-joined).
    pub fn ranger_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `networkName__nin` (array param, comma-joined).
    pub fn network_name__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage__nin` (array param, comma-joined).
    pub fn active_coverage__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue__nin` (array param, comma-joined).
    pub fn all_tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `networkName` (array param, comma-joined).
    pub fn network_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nin` (array param, comma-joined).
    pub fn all_tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `gatewayIps__contains` (array param, comma-joined).
    pub fn gateway_ips__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer__nin` (array param, comma-joined).
    pub fn manufacturer__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces` (array param, comma-joined).
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion__nin` (array param, comma-joined).
    pub fn os_name_version__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage__nin` (array param, comma-joined).
    pub fn missing_coverage__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus__nin` (array param, comma-joined).
    pub fn asset_status__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `detectedFromSite` (array param, comma-joined).
    pub fn detected_from_site<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `eppUnsupportedUnknown` (array param, comma-joined).
    pub fn epp_unsupported_unknown<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `category` (array param, comma-joined).
    pub fn category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.category = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview__nin` (array param, comma-joined).
    pub fn device_review__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagKeyValue__contains` (array param, comma-joined).
    pub fn ranger_tag_key_value__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_value__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail__nin` (array param, comma-joined).
    pub fn asset_contact_email__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `alertSeverity` (array param, comma-joined).
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `accountIds` (array param, comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion__contains` (array param, comma-joined).
    pub fn os_name_version__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain__contains` (array param, comma-joined).
    pub fn domain__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__nin` (array param, comma-joined).
    pub fn resource_type__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `gatewayMacs__contains` (array param, comma-joined).
    pub fn gateway_macs__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `os__nin` (array param, comma-joined).
    pub fn os__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer__contains` (array param, comma-joined).
    pub fn manufacturer__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail` (array param, comma-joined).
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `udpPorts` (array param, comma-joined).
    pub fn udp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors` (array param, comma-joined).
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetEnvironment` (array param, comma-joined).
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osFamily` (array param, comma-joined).
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces__nin` (array param, comma-joined).
    pub fn surfaces__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKey__nin` (array param, comma-joined).
    pub fn ranger_tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__in` (array param, comma-joined).
    pub fn id__in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__in = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__contains` (array param, comma-joined).
    pub fn resource_type__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `memoryReadable__nin` (array param, comma-joined).
    pub fn memory_readable__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue` (array param, comma-joined).
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKeyValue` (array param, comma-joined).
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names` (array param, comma-joined).
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `macAddresses__contains` (array param, comma-joined).
    pub fn mac_addresses__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `detectedFromSite__nin` (array param, comma-joined).
    pub fn detected_from_site__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `name__contains` (array param, comma-joined).
    pub fn name__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion` (array param, comma-joined).
    pub fn os_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality` (array param, comma-joined).
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `firstSeenDt__between`.
    pub fn first_seen_dt__between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt__between = Some(v.into());
        self
    }
    /// Set `lastUpdateDt__between`.
    pub fn last_update_dt__between(mut self, v: impl Into<String>) -> Self {
        self.last_update_dt__between = Some(v.into());
        self
    }
    /// Set `discoveryMethods__nin` (array param, comma-joined).
    pub fn discovery_methods__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nexists` (array param, comma-joined).
    pub fn all_tags_key__nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nexists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer` (array param, comma-joined).
    pub fn manufacturer<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `siteIds` (array param, comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `s1UpdatedAt__between`.
    pub fn s1_updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at__between = Some(v.into());
        self
    }
    /// Set `resourceType` (array param, comma-joined).
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetEnvironment__nin` (array param, comma-joined).
    pub fn asset_environment__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names__nin` (array param, comma-joined).
    pub fn names__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `coreCount` (array param, comma-joined).
    pub fn core_count<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion__nin` (array param, comma-joined).
    pub fn os_version__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subCategory__nin` (array param, comma-joined).
    pub fn sub_category__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `discoveryMethods` (array param, comma-joined).
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus` (array param, comma-joined).
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tcpPorts` (array param, comma-joined).
    pub fn tcp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage` (array param, comma-joined).
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `countsFor` (array param, comma-joined).
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `coreCount__nin` (array param, comma-joined).
    pub fn core_count__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `csvFilterId`.
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// Set `rangerTagKey__contains` (array param, comma-joined).
    pub fn ranger_tag_key__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKeyValue__nin` (array param, comma-joined).
    pub fn ranger_tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `architecture__nin` (array param, comma-joined).
    pub fn architecture__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subCategory` (array param, comma-joined).
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion__contains` (array param, comma-joined).
    pub fn os_version__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview` (array param, comma-joined).
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey` (array param, comma-joined).
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `hostnames__contains` (array param, comma-joined).
    pub fn hostnames__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `architecture` (array param, comma-joined).
    pub fn architecture<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__contains` (array param, comma-joined).
    pub fn id__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `internalIps__contains` (array param, comma-joined).
    pub fn internal_ips__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus` (array param, comma-joined).
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `eppUnsupportedUnknown__nin` (array param, comma-joined).
    pub fn epp_unsupported_unknown__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain__nin` (array param, comma-joined).
    pub fn domain__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__nin` (array param, comma-joined).
    pub fn tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__exists` (array param, comma-joined).
    pub fn all_tags_key__exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__exists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain` (array param, comma-joined).
    pub fn domain<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osFamily__nin` (array param, comma-joined).
    pub fn os_family__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus__nin` (array param, comma-joined).
    pub fn infection_status__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion` (array param, comma-joined).
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/surface/networkDiscovery/export`.
///
/// `exportFormat` is required; all other fields are optional. Array params are
/// serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportQuery {
    /// Export format.
    ///
    /// Spec enum values: csv, json. Required.
    pub export_format: String,
    /// Free-text filter by tag key (supports multiple values)
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key__contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Spec enum values: critical, high, medium, low, --.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality__nin: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// Tag Keys (not in)
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key__nin: Option<String>,
    /// Tag Keys exists
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key__exists: Option<String>,
    /// The memory of the device in human readable format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_readable: Option<String>,
    /// The IP addresses
    #[serde(rename = "ipAddress__contains", skip_serializing_if = "Option::is_none")]
    pub ip_address__contains: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value__contains: Option<String>,
    /// Tag Keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Spec enum values: Unresolved Alerts, High Value.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key__nexists: Option<String>,
    /// The operating system of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// List of Group IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Free-text filter by the image name
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name__contains: Option<String>,
    /// The subnets
    #[serde(rename = "subnets__contains", skip_serializing_if = "Option::is_none")]
    pub subnets__contains: Option<String>,
    /// The ranger tags key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key: Option<String>,
    /// The network name (not in)
    #[serde(rename = "networkName__nin", skip_serializing_if = "Option::is_none")]
    pub network_name__nin: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage__nin: Option<String>,
    /// User and cloud tags (not in)
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value__nin: Option<String>,
    /// The network name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// User and cloud tag keys (not in)
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nin: Option<String>,
    /// The gateway IPs
    #[serde(rename = "gatewayIps__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_ips__contains: Option<String>,
    /// The manufacturer of the device (not in)
    #[serde(rename = "manufacturer__nin", skip_serializing_if = "Option::is_none")]
    pub manufacturer__nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Spec enum values: Cloud, Identity, Network, Endpoint, Network Discovery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The operating system name and version of the device (not in)
    #[serde(rename = "osNameVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_name_version__nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage__nin: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Spec enum values: Active, Inactive.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status__nin: Option<String>,
    /// The site from which the device was detected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_from_site: Option<String>,
    /// The agent supported or unknown state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The category that each resource belongs to
    ///
    /// Spec enum values: All, Account, AI ML, Application Integration, Cloud Application, Code, Container, Data Analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The asset review (not in)
    ///
    /// Spec enum values: Not Reviewed, Under Analysis, Not Trusted, Allowed.
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review__nin: Option<String>,
    /// Free-text filter by Ranger tag key value (supports multiple values)
    #[serde(rename = "rangerTagKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key_value__contains: Option<String>,
    /// Asset Contact Email (not in)
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email__nin: Option<String>,
    /// The severity of the alert
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The OS names and versions
    #[serde(rename = "osNameVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_name_version__contains: Option<String>,
    /// The domain
    #[serde(rename = "domain__contains", skip_serializing_if = "Option::is_none")]
    pub domain__contains: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Spec enum values: Access Control and Surveillance System, Access Point, AD Certificate, AD Certificate Authority, AD Certificate Template, AD Containers, AD DNS Zone, AD Domain.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type__nin: Option<String>,
    /// The gateway MACs
    #[serde(rename = "gatewayMacs__contains", skip_serializing_if = "Option::is_none")]
    pub gateway_macs__contains: Option<String>,
    /// The operating system of the device (not in)
    #[serde(rename = "os__nin", skip_serializing_if = "Option::is_none")]
    pub os__nin: Option<String>,
    /// The manufacturer
    #[serde(rename = "manufacturer__contains", skip_serializing_if = "Option::is_none")]
    pub manufacturer__contains: Option<String>,
    /// Asset Contact Email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The UDP ports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_ports: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Spec enum values: Unresolved Alerts, High Value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Spec enum values: Cloud, Identity, Network, Endpoint, Network Discovery.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces__nin: Option<String>,
    /// The ranger tags key (not in)
    #[serde(rename = "rangerTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key__nin: Option<String>,
    /// The ID
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id__in: Option<String>,
    /// The Asset Type
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type__contains: Option<String>,
    /// The memory of the device in human readable format (not in)
    #[serde(rename = "memoryReadable__nin", skip_serializing_if = "Option::is_none")]
    pub memory_readable__nin: Option<String>,
    /// Tags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The ranger tags key value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value: Option<String>,
    /// Name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The MAC addresses
    #[serde(rename = "macAddresses__contains", skip_serializing_if = "Option::is_none")]
    pub mac_addresses__contains: Option<String>,
    /// The site from which the device was detected (not in)
    #[serde(rename = "detectedFromSite__nin", skip_serializing_if = "Option::is_none")]
    pub detected_from_site__nin: Option<String>,
    /// The name
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
    /// The operating system version of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Spec enum values: critical, high, medium, low, --.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The first seen date
    #[serde(rename = "firstSeenDt__between", skip_serializing_if = "Option::is_none")]
    pub first_seen_dt__between: Option<String>,
    /// The last update date
    #[serde(rename = "lastUpdateDt__between", skip_serializing_if = "Option::is_none")]
    pub last_update_dt__between: Option<String>,
    /// The discovery methods (not in)
    #[serde(rename = "discoveryMethods__nin", skip_serializing_if = "Option::is_none")]
    pub discovery_methods__nin: Option<String>,
    /// User and cloud tag keys not exists
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nexists: Option<String>,
    /// The manufacturer of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    /// List of Site IDs to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Spec enum values: Access Control and Surveillance System, Access Point, AD Certificate, AD Certificate Authority, AD Certificate Template, AD Containers, AD DNS Zone, AD Domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active Directory (not in)
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment__nin: Option<String>,
    /// Name (not in)
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names__nin: Option<String>,
    /// The number of cores
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<String>,
    /// The operating system version of the device (not in)
    #[serde(rename = "osVersion__nin", skip_serializing_if = "Option::is_none")]
    pub os_version__nin: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Spec enum values: All, Access Key and Secret, Access Management, Account, Account Group, AD Objects, Administrative Unit, Admission Controller.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category__nin: Option<String>,
    /// The discovery methods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovery_methods: Option<String>,
    /// The status alerts of the asset
    ///
    /// Spec enum values: Infected, Healthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The TCP ports
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcp_ports: Option<String>,
    /// The active coverage for the asset
    ///
    /// Spec enum values: CWS, CDS, EPP, Ranger Insights, RAD, ISPM, Data Classification, CNS KSPM.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The number of cores (not in)
    #[serde(rename = "coreCount__nin", skip_serializing_if = "Option::is_none")]
    pub core_count__nin: Option<String>,
    /// The ID of the CSV file to filter by
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Free-text filter by Ranger tag key (supports multiple values)
    #[serde(rename = "rangerTagKey__contains", skip_serializing_if = "Option::is_none")]
    pub ranger_tag_key__contains: Option<String>,
    /// The ranger tags key value (not in)
    #[serde(rename = "rangerTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub ranger_tags_key_value__nin: Option<String>,
    /// The architecture of the device (not in)
    #[serde(rename = "architecture__nin", skip_serializing_if = "Option::is_none")]
    pub architecture__nin: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Spec enum values: All, Access Key and Secret, Access Management, Account, Account Group, AD Objects, Administrative Unit, Admission Controller.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The OS versions
    #[serde(rename = "osVersion__contains", skip_serializing_if = "Option::is_none")]
    pub os_version__contains: Option<String>,
    /// The asset review
    ///
    /// Spec enum values: Not Reviewed, Under Analysis, Not Trusted, Allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The hostnames
    #[serde(rename = "hostnames__contains", skip_serializing_if = "Option::is_none")]
    pub hostnames__contains: Option<String>,
    /// The architecture of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The ID
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id__contains: Option<String>,
    /// The internal IPs
    #[serde(rename = "internalIps__contains", skip_serializing_if = "Option::is_none")]
    pub internal_ips__contains: Option<String>,
    /// The status of the asset
    ///
    /// Spec enum values: Active, Inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// The agent supported or unknown state (not in)
    #[serde(rename = "eppUnsupportedUnknown__nin", skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown__nin: Option<String>,
    /// The domain of the device (not in)
    #[serde(rename = "domain__nin", skip_serializing_if = "Option::is_none")]
    pub domain__nin: Option<String>,
    /// Tags (not in)
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value__nin: Option<String>,
    /// User and cloud tag keys exists
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key__exists: Option<String>,
    /// The domain of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The operating system family of the device (not in)
    #[serde(rename = "osFamily__nin", skip_serializing_if = "Option::is_none")]
    pub os_family__nin: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Spec enum values: Infected, Healthy.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status__nin: Option<String>,
    /// The operating system name and version of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_name_version: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items, use "cursor".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The column to sort the results by.
    ///
    /// Spec enum values: s1GroupName, memory, detectedFromSite, previousOsType, previousOsVersion, eppUnsupportedUnknown, s1OnboardedAccountName, category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction.
    ///
    /// Spec enum values: asc, desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time.
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ExportQuery {
    /// Construct with all required params.
    pub fn new(export_format: impl Into<String>) -> Self {
        Self {
            export_format: export_format.into(),
            ..Default::default()
        }
    }

    /// Set `tagsKey__contains` (array param, comma-joined).
    pub fn tags_key__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality__nin` (array param, comma-joined).
    pub fn asset_criticality__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage` (array param, comma-joined).
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue` (array param, comma-joined).
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nin` (array param, comma-joined).
    pub fn tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__exists` (array param, comma-joined).
    pub fn tags_key__exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__exists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `memoryReadable` (array param, comma-joined).
    pub fn memory_readable<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `ipAddress__contains` (array param, comma-joined).
    pub fn ip_address__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ip_address__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__contains` (array param, comma-joined).
    pub fn tags_key_value__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey` (array param, comma-joined).
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors__nin` (array param, comma-joined).
    pub fn risk_factors__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nexists` (array param, comma-joined).
    pub fn tags_key__nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nexists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `os` (array param, comma-joined).
    pub fn os<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `groupIds` (array param, comma-joined).
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `imageName__contains` (array param, comma-joined).
    pub fn image_name__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subnets__contains` (array param, comma-joined).
    pub fn subnets__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.subnets__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKey` (array param, comma-joined).
    pub fn ranger_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `networkName__nin` (array param, comma-joined).
    pub fn network_name__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage__nin` (array param, comma-joined).
    pub fn active_coverage__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue__nin` (array param, comma-joined).
    pub fn all_tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `networkName` (array param, comma-joined).
    pub fn network_name<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.network_name = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nin` (array param, comma-joined).
    pub fn all_tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `gatewayIps__contains` (array param, comma-joined).
    pub fn gateway_ips__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_ips__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer__nin` (array param, comma-joined).
    pub fn manufacturer__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces` (array param, comma-joined).
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion__nin` (array param, comma-joined).
    pub fn os_name_version__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage__nin` (array param, comma-joined).
    pub fn missing_coverage__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus__nin` (array param, comma-joined).
    pub fn asset_status__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `detectedFromSite` (array param, comma-joined).
    pub fn detected_from_site<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `eppUnsupportedUnknown` (array param, comma-joined).
    pub fn epp_unsupported_unknown<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `category` (array param, comma-joined).
    pub fn category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.category = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview__nin` (array param, comma-joined).
    pub fn device_review__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagKeyValue__contains` (array param, comma-joined).
    pub fn ranger_tag_key_value__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key_value__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail__nin` (array param, comma-joined).
    pub fn asset_contact_email__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `alertSeverity` (array param, comma-joined).
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `accountIds` (array param, comma-joined).
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion__contains` (array param, comma-joined).
    pub fn os_name_version__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain__contains` (array param, comma-joined).
    pub fn domain__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__nin` (array param, comma-joined).
    pub fn resource_type__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `gatewayMacs__contains` (array param, comma-joined).
    pub fn gateway_macs__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.gateway_macs__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `os__nin` (array param, comma-joined).
    pub fn os__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer__contains` (array param, comma-joined).
    pub fn manufacturer__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail` (array param, comma-joined).
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `udpPorts` (array param, comma-joined).
    pub fn udp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.udp_ports = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors` (array param, comma-joined).
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetEnvironment` (array param, comma-joined).
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osFamily` (array param, comma-joined).
    pub fn os_family<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces__nin` (array param, comma-joined).
    pub fn surfaces__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKey__nin` (array param, comma-joined).
    pub fn ranger_tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__in` (array param, comma-joined).
    pub fn id__in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__in = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__contains` (array param, comma-joined).
    pub fn resource_type__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `memoryReadable__nin` (array param, comma-joined).
    pub fn memory_readable__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.memory_readable__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue` (array param, comma-joined).
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKeyValue` (array param, comma-joined).
    pub fn ranger_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names` (array param, comma-joined).
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `macAddresses__contains` (array param, comma-joined).
    pub fn mac_addresses__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.mac_addresses__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `detectedFromSite__nin` (array param, comma-joined).
    pub fn detected_from_site__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.detected_from_site__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `name__contains` (array param, comma-joined).
    pub fn name__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion` (array param, comma-joined).
    pub fn os_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality` (array param, comma-joined).
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `firstSeenDt__between`.
    pub fn first_seen_dt__between(mut self, v: impl Into<String>) -> Self {
        self.first_seen_dt__between = Some(v.into());
        self
    }
    /// Set `lastUpdateDt__between`.
    pub fn last_update_dt__between(mut self, v: impl Into<String>) -> Self {
        self.last_update_dt__between = Some(v.into());
        self
    }
    /// Set `discoveryMethods__nin` (array param, comma-joined).
    pub fn discovery_methods__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nexists` (array param, comma-joined).
    pub fn all_tags_key__nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nexists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `manufacturer` (array param, comma-joined).
    pub fn manufacturer<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.manufacturer = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `siteIds` (array param, comma-joined).
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `s1UpdatedAt__between`.
    pub fn s1_updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at__between = Some(v.into());
        self
    }
    /// Set `resourceType` (array param, comma-joined).
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetEnvironment__nin` (array param, comma-joined).
    pub fn asset_environment__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names__nin` (array param, comma-joined).
    pub fn names__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `coreCount` (array param, comma-joined).
    pub fn core_count<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion__nin` (array param, comma-joined).
    pub fn os_version__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subCategory__nin` (array param, comma-joined).
    pub fn sub_category__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `discoveryMethods` (array param, comma-joined).
    pub fn discovery_methods<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.discovery_methods = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus` (array param, comma-joined).
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tcpPorts` (array param, comma-joined).
    pub fn tcp_ports<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tcp_ports = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage` (array param, comma-joined).
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `countsFor` (array param, comma-joined).
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `coreCount__nin` (array param, comma-joined).
    pub fn core_count__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.core_count__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `csvFilterId`.
    pub fn csv_filter_id(mut self, v: i64) -> Self {
        self.csv_filter_id = Some(v);
        self
    }
    /// Set `rangerTagKey__contains` (array param, comma-joined).
    pub fn ranger_tag_key__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tag_key__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `rangerTagsKeyValue__nin` (array param, comma-joined).
    pub fn ranger_tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.ranger_tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `architecture__nin` (array param, comma-joined).
    pub fn architecture__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `subCategory` (array param, comma-joined).
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osVersion__contains` (array param, comma-joined).
    pub fn os_version__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_version__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview` (array param, comma-joined).
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey` (array param, comma-joined).
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `hostnames__contains` (array param, comma-joined).
    pub fn hostnames__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.hostnames__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `architecture` (array param, comma-joined).
    pub fn architecture<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.architecture = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__contains` (array param, comma-joined).
    pub fn id__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `internalIps__contains` (array param, comma-joined).
    pub fn internal_ips__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.internal_ips__contains = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus` (array param, comma-joined).
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `eppUnsupportedUnknown__nin` (array param, comma-joined).
    pub fn epp_unsupported_unknown__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.epp_unsupported_unknown__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain__nin` (array param, comma-joined).
    pub fn domain__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__nin` (array param, comma-joined).
    pub fn tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__exists` (array param, comma-joined).
    pub fn all_tags_key__exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__exists = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `domain` (array param, comma-joined).
    pub fn domain<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.domain = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osFamily__nin` (array param, comma-joined).
    pub fn os_family__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_family__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus__nin` (array param, comma-joined).
    pub fn infection_status__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status__nin = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `osNameVersion` (array param, comma-joined).
    pub fn os_name_version<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.os_name_version = Some(
            v.into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `skip`.
    pub fn skip(mut self, v: i64) -> Self {
        self.skip = Some(v);
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
    /// Set `countOnly`.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Set `cursor`.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// Set `skipCount`.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// Set `limit`.
    pub fn limit(mut self, v: i64) -> Self {
        self.limit = Some(v);
        self
    }
}


/// Body for `POST /web/api/v2.1/xdr/assets/surface/networkDiscovery/action`.
///
/// Spec schema: `NetworkDiscoveryActionPayloadSchema`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionBody {
    /// Action name. Required.
    ///
    /// Spec enum values: `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    pub action_name: String,
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000). Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl PerformActionBody {
    /// Construct with the required `actionName`.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self {
            action_name: action_name.into(),
            ..Default::default()
        }
    }
    /// Set `id__in` (list of selected inventory ids, max 5000).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Set `id__nin` (list of inventory ids to exclude, max 5000).
    pub fn id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

/// Body for
/// `POST /web/api/v2.1/xdr/assets/surface/networkDiscovery/available-actions/with-status`.
///
/// Spec schema: `AffectedResourcesSchema`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionsBody {
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,
    /// List of inventory ids to exclude from select_all (max 5000). Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

impl AvailableActionsBody {
    /// Set `id__in` (list of selected inventory ids, max 5000).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// Set `id__nin` (list of inventory ids to exclude, max 5000).
    pub fn id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(v.into_iter().map(Into::into).collect());
        self
    }
}


impl InventoryNetworkDiscoverySurfaceService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/surface/networkDiscovery` — Assets.
    ///
    /// Get inventory of Network Discovery surface assets.
    pub async fn list(
        &self,
        query: &ListQuery,
    ) -> Result<Paginated<NetworkDiscovery>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/surface/networkDiscovery", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/surface/networkDiscovery/action` —
    /// Perform action.
    ///
    /// Perform action on selected assets.
    pub async fn perform_action(
        &self,
        query: &PerformActionQuery,
        body: &PerformActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<PerformActionBody, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/surface/networkDiscovery/action",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/surface/networkDiscovery/available-actions/with-status`
    /// — Available actions.
    ///
    /// Get inventory network discovery available actions.
    pub async fn available_actions(
        &self,
        query: &AvailableActionsQuery,
        body: &AvailableActionsBody,
    ) -> Result<Response<NetworkDiscoveryAvailableActions>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<AvailableActionsBody, Response<NetworkDiscoveryAvailableActions>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/surface/networkDiscovery/available-actions/with-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/surface/networkDiscovery/export` — Export
    /// assets to CSV or JSON.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    /// The response shape depends on `exportFormat`, so it is returned as a
    /// raw [`serde_json::Value`].
    pub async fn export(
        &self,
        query: &ExportQuery,
    ) -> Result<serde_json::Value, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/surface/networkDiscovery/export", q)
            .await?)
    }
}
