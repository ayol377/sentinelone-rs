//! `Inventory Storage` tag — Inventory Storage Resources.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_storage::{AvailableActionWithStatusResponse, StorageAsset};
use crate::pagination::{Paginated, Response};

/// `Inventory Storage` tag — operations on storage assets.
pub struct InventoryStorageService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Join an iterator of stringy values into a comma-separated query value.
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

/// Renders a query struct to a querystring suffix (`?a=b&c=d`) for embedding in
/// a path on `POST` calls. Returns an empty string when nothing serializes.
fn query_suffix<Q: Serialize>(query: &Q) -> String {
    let qs = serde_urlencoded::to_string(query).unwrap_or_default();
    if qs.is_empty() {
        String::new()
    } else {
        format!("?{qs}")
    }
}

/// Shared storage filter query parameters.
///
/// These filter fields are common to the storage list (`GET`), action (`POST`),
/// available-actions (`POST`), and export (`GET`) endpoints. Per-method query
/// structs embed this set via `#[serde(flatten)]` and add any method-specific
/// extras (paging, count, export format, ...).
///
/// Every field is optional. Array filters are serialized as a single
/// comma-joined string (the form the API expects); use the builder methods,
/// which accept an iterator and join by comma.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageFilterQuery {
    /// Free-text filter by tag key (supports multiple values). Array param.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Array param.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in). Array param.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// Data types contains. Array param.
    #[serde(rename = "data_types__contains", skip_serializing_if = "Option::is_none")]
    pub data_types_contains: Option<String>,
    /// The missing coverage for the asset. Array param. Allowed values: `CWS`,
    /// `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Array param.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// Tag Keys (not in). Array param.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID. Array param.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Array param.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys exists. Array param.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The CDS malware scan status (not in). Array param.
    #[serde(rename = "scanStatus__nin", skip_serializing_if = "Option::is_none")]
    pub scan_status_nin: Option<String>,
    /// The region. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The encryption type (not in). Array param.
    #[serde(rename = "encryptionType__nin", skip_serializing_if = "Option::is_none")]
    pub encryption_type_nin: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Array param.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Array param.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys. Array param.
    #[serde(rename = "tagsKey", skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). Array param.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Array param.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The cloud provider account ID. Array param.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Array param.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Threat Detection Status (not in). Array param.
    #[serde(rename = "threatDetectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub threat_detection_status_nin: Option<String>,
    /// The active coverage for the asset (not in). Array param. Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// Data classification status. Array param.
    #[serde(rename = "data_classification_status__contains", skip_serializing_if = "Option::is_none")]
    pub data_classification_status_contains: Option<String>,
    /// User and cloud tags (not in). Array param.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in). Array param.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in). Array param.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The cloud tags key value (not in). Array param.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to. Array param. Allowed values:
    /// `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The number of objects in the bucket (not in). Array param.
    #[serde(rename = "objectCount__nin", skip_serializing_if = "Option::is_none")]
    pub object_count_nin: Option<String>,
    /// The missing coverage for the asset (not in). Array param. Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). Array param. Allowed values: `Active`,
    /// `Inactive`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The geographical area where cloud resources are hosted. Array param.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The asset review (not in). Array param. Allowed values: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// Threat Detection Policy. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_policy_status: Option<String>,
    /// The cloud provider account name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in). Array param.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in). Array param. Allowed
    /// values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// Asset Contact Email. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// Data classification scan status. Array param.
    #[serde(rename = "data_classification_scan_status__contains", skip_serializing_if = "Option::is_none")]
    pub data_classification_scan_status_contains: Option<String>,
    /// The risk factors associated with the asset. Array param. Allowed values:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in — AWS | Azure | GCP |
    /// Active Directory. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The number of objects in the bucket. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<String>,
    /// The Surface that each asset belongs to (not in). Array param. Allowed
    /// values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// Data Types. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_types: Option<String>,
    /// The encryption type. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// The ID. Array param.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    /// Array param.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Array param.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// Tags. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Whether there is public access or not. Array param (of booleans).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<String>,
    /// The cloud provider organization unit. Array param.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Array param.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// Data classification scan status to exclude. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_scan_status: Option<String>,
    /// Whether there is public access or not (not in). Array param (of booleans).
    #[serde(rename = "isPublic__nin", skip_serializing_if = "Option::is_none")]
    pub is_public_nin: Option<String>,
    /// Data Types (not in). Array param.
    #[serde(rename = "dataTypes__nin", skip_serializing_if = "Option::is_none")]
    pub data_types_nin: Option<String>,
    /// Name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name. Array param.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to. Array param. Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The threat detection status. Array param.
    #[serde(rename = "threatDetectionStatus__contains", skip_serializing_if = "Option::is_none")]
    pub threat_detection_status_contains: Option<String>,
    /// The cloud provider account id (not in). Array param.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// User and cloud tag keys not exists. Array param.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// List of Site IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. Date/time string.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Array param. Allowed values:
    /// `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in — AWS | Azure | GCP |
    /// Active Directory (not in). Array param.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). Array param.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The cloud tags key. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in). Array param.
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset. Array param. Allowed values: `Infected`,
    /// `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// Connectivity. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_status: Option<String>,
    /// The active coverage for the asset. Array param. Allowed values: `CWS`,
    /// `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Threat Detection Status. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_status: Option<String>,
    /// Data Classification Policy. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_policy_status: Option<String>,
    /// The cloud provider account id. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to. Array param. Allowed
    /// values: `All`, `Access Key and Secret`, `Access Management`, `Account`,
    /// `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// Data Classification Status. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_status: Option<String>,
    /// The asset review. Array param. Allowed values: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The CDS malware scan status. Array param.
    #[serde(rename = "scanStatus__contains", skip_serializing_if = "Option::is_none")]
    pub scan_status_contains: Option<String>,
    /// The ID. Array param.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name. Array param.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// Scanner. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_status: Option<String>,
    /// The status of the asset. Array param. Allowed values: `Active`,
    /// `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether there is versioning enabled or not. Array param (of booleans).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_enabled: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Array param.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Array param.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. Array param.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// Data Classification Status (not in). Array param.
    #[serde(rename = "dataClassificationStatus__nin", skip_serializing_if = "Option::is_none")]
    pub data_classification_status_nin: Option<String>,
    /// The CDS malware scan status. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// The cloud provider project ID. Array param.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl StorageFilterQuery {
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
    /// Allowed: `critical`, `high`, `medium`, `low`, `--`.
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(join_csv(v));
        self
    }
    /// The status alerts of the asset (not in). Allowed: `Infected`, `Healthy`.
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(join_csv(v));
        self
    }
    /// Data types contains.
    pub fn data_types_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types_contains = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset. Allowed: `CWS`, `CDS`, `EPP`,
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
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(join_csv(v));
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
    /// The encryption type (not in).
    pub fn encryption_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type_nin = Some(join_csv(v));
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
    /// Allowed: `Unresolved Alerts`, `High Value`.
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
    /// Threat Detection Status (not in).
    pub fn threat_detection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_nin = Some(join_csv(v));
        self
    }
    /// The active coverage for the asset (not in). Allowed: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(join_csv(v));
        self
    }
    /// Data classification status.
    pub fn data_classification_status_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status_contains = Some(join_csv(v));
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
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(join_csv(v));
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
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(join_csv(v));
        self
    }
    /// The Surface that each asset belongs to. Allowed: `Cloud`, `Identity`,
    /// `Network`, `Endpoint`, `Network Discovery`.
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
    /// The missing coverage for the asset (not in). Allowed: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(join_csv(v));
        self
    }
    /// The status of the asset (not in). Allowed: `Active`, `Inactive`.
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(join_csv(v));
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
    /// The asset review (not in). Allowed: `Not Reviewed`, `Under Analysis`,
    /// `Not Trusted`, `Allowed`, `` (empty).
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
    /// The canonical name for the resource type (not in). Allowed: `Access
    /// Control and Surveillance System`, `Access Point`, `AD Certificate`,
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
    /// Asset Contact Email.
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(join_csv(v));
        self
    }
    /// Data classification scan status.
    pub fn data_classification_scan_status_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_scan_status_contains = Some(join_csv(v));
        self
    }
    /// The risk factors associated with the asset.
    /// Allowed: `Unresolved Alerts`, `High Value`.
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in.
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
    /// The Surface that each asset belongs to (not in). Allowed: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(join_csv(v));
        self
    }
    /// Data Types.
    pub fn data_types<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types = Some(join_csv(v));
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
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(join_csv(v));
        self
    }
    /// Whether there is public access or not.
    pub fn is_public<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_public = Some(join_csv(v));
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
    /// Data classification scan status to exclude.
    pub fn data_classification_scan_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_scan_status = Some(join_csv(v));
        self
    }
    /// Whether there is public access or not (not in).
    pub fn is_public_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_public_nin = Some(join_csv(v));
        self
    }
    /// Data Types (not in).
    pub fn data_types_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types_nin = Some(join_csv(v));
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
    /// The name.
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to.
    /// Allowed: `critical`, `high`, `medium`, `low`, `--`.
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(v));
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
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(join_csv(v));
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
    /// The canonical name for the resource type. Allowed: `Access Control and
    /// Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate
    /// Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`,
    /// `AD Domain`.
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(v));
        self
    }
    /// The environment that the asset exists in (not in).
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
    /// The cloud tags key.
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key = Some(join_csv(v));
        self
    }
    /// The sub-category that each resource belongs to (not in). Allowed: `All`,
    /// `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`.
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(join_csv(v));
        self
    }
    /// The status alerts of the asset. Allowed: `Infected`, `Healthy`.
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(join_csv(v));
        self
    }
    /// Connectivity.
    pub fn connectivity_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.connectivity_status = Some(join_csv(v));
        self
    }
    /// The active coverage for the asset. Allowed: `CWS`, `CDS`, `EPP`,
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
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self {
        self.csv_filter_id = Some(n);
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
    /// Data Classification Policy.
    pub fn data_classification_policy_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_policy_status = Some(join_csv(v));
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
    /// The sub-category that each resource belongs to. Allowed: `All`,
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
    /// Data Classification Status.
    pub fn data_classification_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status = Some(join_csv(v));
        self
    }
    /// The asset review. Allowed: `Not Reviewed`, `Under Analysis`,
    /// `Not Trusted`, `Allowed`, `` (empty).
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
    /// The CDS malware scan status.
    pub fn scan_status_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_contains = Some(join_csv(v));
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
    /// Scanner.
    pub fn scanner_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scanner_status = Some(join_csv(v));
        self
    }
    /// The status of the asset. Allowed: `Active`, `Inactive`.
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(join_csv(v));
        self
    }
    /// Whether there is versioning enabled or not.
    pub fn versioning_enabled<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.versioning_enabled = Some(join_csv(v));
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
    /// Data Classification Status (not in).
    pub fn data_classification_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status_nin = Some(join_csv(v));
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

/// Query params for `GET /web/api/v2.1/xdr/assets/storage`.
///
/// Wraps the shared [`StorageFilterQuery`] (flattened) and adds the list-specific
/// paging and count controls. Array filter params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssetsQuery {
    /// Shared storage filter fields.
    #[serde(flatten)]
    pub filter: StorageFilterQuery,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
}

impl ListAssetsQuery {
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: StorageFilterQuery) -> Self {
        self.filter = filter;
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// Return only the total number of items.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Skip calculating the total number of items.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/storage`.
///
/// This POST variant accepts only scope filters as query params (the rest of the
/// filter is supplied in the request body).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssetsPostQuery {
    /// List of Account IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl ListAssetsPostQuery {
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

/// Query params for `POST /web/api/v2.1/xdr/assets/storage/action`.
///
/// Only the shared [`StorageFilterQuery`] filter fields are accepted as query
/// params on this endpoint (the action itself is supplied in the body).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionQuery {
    /// Shared storage filter fields.
    #[serde(flatten)]
    pub filter: StorageFilterQuery,
}

impl PerformActionQuery {
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: StorageFilterQuery) -> Self {
        self.filter = filter;
        self
    }
}

/// Query params for
/// `POST /web/api/v2.1/xdr/assets/storage/available-actions/with-status`.
///
/// Only the shared [`StorageFilterQuery`] filter fields are accepted as query
/// params on this endpoint.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionsQuery {
    /// Shared storage filter fields.
    #[serde(flatten)]
    pub filter: StorageFilterQuery,
}

impl AvailableActionsQuery {
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: StorageFilterQuery) -> Self {
        self.filter = filter;
        self
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/storage/export`.
///
/// Wraps the shared [`StorageFilterQuery`] (flattened) and adds export-specific
/// paging controls plus the required `exportFormat`. The only required param is
/// `export_format`; set it via [`ExportAssetsQuery::new`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportAssetsQuery {
    /// Shared storage filter fields.
    #[serde(flatten)]
    pub filter: StorageFilterQuery,
    /// Export format. Required. Allowed values: `csv`, `json`.
    pub export_format: String,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
}

impl ExportAssetsQuery {
    /// Create a new export query with the required export format
    /// (`csv` or `json`).
    pub fn new(export_format: impl Into<String>) -> Self {
        Self {
            filter: StorageFilterQuery::default(),
            export_format: export_format.into(),
            skip: None,
            limit: None,
            cursor: None,
            count_only: None,
            skip_count: None,
        }
    }
    /// Set the required export format. Allowed values: `csv`, `json`.
    pub fn export_format(mut self, v: impl Into<String>) -> Self {
        self.export_format = v.into();
        self
    }
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: StorageFilterQuery) -> Self {
        self.filter = filter;
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// Return only the total number of items.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// Skip calculating the total number of items.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
}

/// Body for `POST /web/api/v2.1/xdr/assets/storage`
/// (`StorageViewInputSchema`).
///
/// The full filter shape is freeform in the spec, so `filter` (required) and
/// `data` (optional, nullable) are modelled as [`serde_json::Value`].
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssetsPostBody {
    /// Filter (`PaginatedStorageFilter`). Required. Freeform object — accepts the
    /// same filter keys as the query params (e.g. `tagsKey`, `assetStatus`,
    /// `limit`, `cursor`, ...).
    pub filter: serde_json::Value,
    /// Data (`EmptyStrict`). Optional, nullable. Freeform object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ListAssetsPostBody {
    /// Create a new body with the required `filter` object.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { filter, data: None }
    }
    /// Set the optional `data` object.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Body for `POST /web/api/v2.1/xdr/assets/storage/action`
/// (`StorageActionPayloadSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionBody {
    /// Action name. Required. Allowed values: `start_full_scan`,
    /// `stop_full_scan`, `export_resource_details`,
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
    /// Create a new action body with the required `actionName`.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self {
            action_name: action_name.into(),
            id_in: None,
            id_nin: None,
        }
    }
    /// List of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from select_all (max 5000).
    pub fn id_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(values.into_iter().map(Into::into).collect());
        self
    }
}

/// Body for
/// `POST /web/api/v2.1/xdr/assets/storage/available-actions/with-status`
/// (`AffectedResourcesSchema`).
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
    /// List of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_in = Some(values.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from select_all (max 5000).
    pub fn id_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id_nin = Some(values.into_iter().map(Into::into).collect());
        self
    }
}

impl InventoryStorageService<'_> {
    /// **Assets** — Get assets.
    ///
    /// Get assets.
    ///
    /// `GET /web/api/v2.1/xdr/assets/storage`
    pub async fn list(&self, query: &ListAssetsQuery) -> Result<Paginated<StorageAsset>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/storage", q)
            .await?)
    }

    /// **Assets using POST** — POST API to get Assets.
    ///
    /// POST API to get Assets.
    ///
    /// `POST /web/api/v2.1/xdr/assets/storage`
    pub async fn list_post(
        &self,
        query: &ListAssetsPostQuery,
        body: &ListAssetsPostBody,
    ) -> Result<Paginated<StorageAsset>, Error> {
        let path = format!("/web/api/v2.1/xdr/assets/storage{}", query_suffix(query));
        Ok(self.client.http().post(&path, body).await?)
    }

    /// **Perform action** — Perform action on selected assets.
    ///
    /// Perform action on selected assets.
    ///
    /// `POST /web/api/v2.1/xdr/assets/storage/action`
    pub async fn perform_action(
        &self,
        query: &PerformActionQuery,
        body: &PerformActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = format!(
            "/web/api/v2.1/xdr/assets/storage/action{}",
            query_suffix(query)
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// **Available actions** — Get available actions.
    ///
    /// Get available actions.
    ///
    /// `POST /web/api/v2.1/xdr/assets/storage/available-actions/with-status`
    pub async fn available_actions(
        &self,
        query: &AvailableActionsQuery,
        body: &AvailableActionsBody,
    ) -> Result<Response<AvailableActionWithStatusResponse>, Error> {
        let path = format!(
            "/web/api/v2.1/xdr/assets/storage/available-actions/with-status{}",
            query_suffix(query)
        );
        Ok(self.client.http().post(&path, body).await?)
    }

    /// **Export assets to CSV or JSON** — Returns the results for given
    /// inventory filter in a CSV or JSON format.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    ///
    /// `GET /web/api/v2.1/xdr/assets/storage/export`
    pub async fn export(
        &self,
        query: &ExportAssetsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/storage/export", q)
            .await?)
    }
}
