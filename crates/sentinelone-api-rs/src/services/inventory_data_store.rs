//! Service for the `Inventory Data Store` tag.
//!
//! Inventory Data Store Resources.
//!
//! Endpoints:
//! - `GET  /web/api/v2.1/xdr/assets/data-store`
//! - `POST /web/api/v2.1/xdr/assets/data-store`
//! - `POST /web/api/v2.1/xdr/assets/data-store/action`
//! - `POST /web/api/v2.1/xdr/assets/data-store/available-actions/with-status`
//! - `GET  /web/api/v2.1/xdr/assets/data-store/export`
//!
//! Implemented 1:1 with the SentinelOne Management API spec.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_data_store::{AvailableActionWithStatusResponse, DataStoreAsset};
use crate::pagination::{Paginated, Response};

/// `Inventory Data Store` tag — query, filter, export and act on inventory
/// data store assets.
pub struct InventoryDataStoreService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query parameters for `GET /web/api/v2.1/xdr/assets/data-store`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
    /// Free-text filter by tag key (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__contains")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetCriticality__nin")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "infectionStatus__nin")]
    pub infection_status_nin: Option<String>,
    /// Data types contains
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_types__contains")]
    pub data_types_contains: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountName__nin")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// Tag Keys (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__nin")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudResourceId__contains")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderSubscriptionId__contains")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__exists")]
    pub tags_key_exists: Option<String>,
    /// The CDS malware scan status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scanStatus__nin")]
    pub scan_status_nin: Option<String>,
    /// The region
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The encryption type (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "encryptionType__nin")]
    pub encryption_type_nin: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKeyValue__contains")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKey__nin")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "riskFactors__nin")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__nexists")]
    pub tags_key_nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor".
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountId__contains")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "imageName__contains")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Threat Detection Status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "threatDetectionStatus__nin")]
    pub threat_detection_status_nin: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeCoverage__nin")]
    pub active_coverage_nin: Option<String>,
    /// Data classification status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_classification_status__contains")]
    pub data_classification_status_contains: Option<String>,
    /// User and cloud tags (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKeyValue__nin")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "region__nin")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__nin")]
    pub all_tags_key_nin: Option<String>,
    /// The cloud tags key value (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKeyValue__nin")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The number of objects in the bucket (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectCount__nin")]
    pub object_count_nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "missingCoverage__nin")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetStatus__nin")]
    pub asset_status_nin: Option<String>,
    /// The column to sort the results by.
    ///
    /// Allowed values: `s1GroupName`, `monitoringEnabled`, `cnsMonitorTargetEnabled`, `loggingEnabled`, `encryptionEnabled`, `s1UpdatedAt`, `cnsVolumeGroupIdExists`, `cnsRetentionDays`.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The geographical area where cloud resources are hosted
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "region__contains")]
    pub region_contains: Option<String>,
    /// The asset review (not in)
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `(empty string)`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deviceReview__nin")]
    pub device_review_nin: Option<String>,
    /// Threat Detection Policy
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_policy_status: Option<String>,
    /// The cloud provider account name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetContactEmail__nin")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType__nin")]
    pub resource_type_nin: Option<String>,
    /// Asset Contact Email
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// Data classification scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_classification_scan_status__contains")]
    pub data_classification_scan_status_contains: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS \
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The number of objects in the bucket
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "surfaces__nin")]
    pub surfaces_nin: Option<String>,
    /// Data Types
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_types: Option<String>,
    /// The encryption type
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// The ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "id__in")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKeyValue__contains")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType__contains")]
    pub resource_type_contains: Option<String>,
    /// Tags
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction
    ///
    /// Allowed values: `asc`, `desc`.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Whether there is public access or not
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<String>,
    /// The cloud provider organization unit
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderOrganizationUnit__contains")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderOrganization__contains")]
    pub cloud_provider_organization_contains: Option<String>,
    /// Data classification scan status to exclude
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_scan_status: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Whether there is public access or not (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "isPublic__nin")]
    pub is_public_nin: Option<String>,
    /// Data Types (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataTypes__nin")]
    pub data_types_nin: Option<String>,
    /// Name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "name__contains")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The threat detection status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "threatDetectionStatus__contains")]
    pub threat_detection_status_contains: Option<String>,
    /// The cloud provider account id (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountId__nin")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__nexists")]
    pub all_tags_key_nexists: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "s1UpdatedAt__between")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS \
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetEnvironment__nin")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "names__nin")]
    pub names_nin: Option<String>,
    /// The cloud tags key
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "subCategory__nin")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// Connectivity
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_status: Option<String>,
    /// The active coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Threat Detection Status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_status: Option<String>,
    /// Data Classification Policy
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_policy_status: Option<String>,
    /// The cloud provider account id
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// Data Classification Status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_status: Option<String>,
    /// The asset review
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `(empty string)`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Limit number of returned items (1-1000)
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The CDS malware scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scanStatus__contains")]
    pub scan_status_contains: Option<String>,
    /// The ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "id__contains")]
    pub id_contains: Option<String>,
    /// The cloud provider account name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountName__contains")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// Scanner
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_status: Option<String>,
    /// The status of the asset
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether there is versioning enabled or not
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_enabled: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKey__contains")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKeyValue__nin")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__exists")]
    pub all_tags_key_exists: Option<String>,
    /// Data Classification Status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataClassificationStatus__nin")]
    pub data_classification_status_nin: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The CDS malware scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// The cloud provider project ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderProjectId__contains")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl ListQuery {
    /// Set `tagsKey__contains` (array param, comma-joined).
    pub fn tags_key_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality__nin` (array param, comma-joined).
    pub fn asset_criticality_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus__nin` (array param, comma-joined).
    pub fn infection_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `data_types__contains` (array param, comma-joined).
    pub fn data_types_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage` (array param, comma-joined).
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
    /// Set `allTagsKeyValue` (array param, comma-joined).
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
    /// Set `cloudProviderAccountName__nin` (array param, comma-joined).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nin` (array param, comma-joined).
    pub fn tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudResourceId__contains` (array param, comma-joined).
    pub fn cloud_resource_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderSubscriptionId__contains` (array param, comma-joined).
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__exists` (array param, comma-joined).
    pub fn tags_key_exists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `scanStatus__nin` (array param, comma-joined).
    pub fn scan_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `region` (array param, comma-joined).
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
    /// Set `encryptionType__nin` (array param, comma-joined).
    pub fn encryption_type_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__contains` (array param, comma-joined).
    pub fn tags_key_value_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey__nin` (array param, comma-joined).
    pub fn cloud_tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey` (array param, comma-joined).
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
    /// Set `riskFactors__nin` (array param, comma-joined).
    pub fn risk_factors_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nexists` (array param, comma-joined).
    pub fn tags_key_nexists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `skip`.
    pub fn skip(mut self, value: i64) -> Self {
        self.skip = Some(value);
        self
    }
    /// Set `cloudProviderAccountId__contains` (array param, comma-joined).
    pub fn cloud_provider_account_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `imageName__contains` (array param, comma-joined).
    pub fn image_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `groupIds` (array param, comma-joined).
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
    /// Set `threatDetectionStatus__nin` (array param, comma-joined).
    pub fn threat_detection_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage__nin` (array param, comma-joined).
    pub fn active_coverage_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `data_classification_status__contains` (array param, comma-joined).
    pub fn data_classification_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue__nin` (array param, comma-joined).
    pub fn all_tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `region__nin` (array param, comma-joined).
    pub fn region_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nin` (array param, comma-joined).
    pub fn all_tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKeyValue__nin` (array param, comma-joined).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces` (array param, comma-joined).
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
    /// Set `objectCount__nin` (array param, comma-joined).
    pub fn object_count_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage__nin` (array param, comma-joined).
    pub fn missing_coverage_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus__nin` (array param, comma-joined).
    pub fn asset_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `sortBy`.
    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }
    /// Set `region__contains` (array param, comma-joined).
    pub fn region_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview__nin` (array param, comma-joined).
    pub fn device_review_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `threatDetectionPolicyStatus` (array param, comma-joined).
    pub fn threat_detection_policy_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_policy_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountName` (array param, comma-joined).
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
    /// Set `assetContactEmail__nin` (array param, comma-joined).
    pub fn asset_contact_email_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `alertSeverity` (array param, comma-joined).
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
    /// Set `accountIds` (array param, comma-joined).
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
    /// Set `cloudTagsKeyValue` (array param, comma-joined).
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
    /// Set `resourceType__nin` (array param, comma-joined).
    pub fn resource_type_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail` (array param, comma-joined).
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
    /// Set `data_classification_scan_status__contains` (array param, comma-joined).
    pub fn data_classification_scan_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_scan_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors` (array param, comma-joined).
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
    /// Set `assetEnvironment` (array param, comma-joined).
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
    /// Set `objectCount` (array param, comma-joined).
    pub fn object_count<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces__nin` (array param, comma-joined).
    pub fn surfaces_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataTypes` (array param, comma-joined).
    pub fn data_types<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `encryptionType` (array param, comma-joined).
    pub fn encryption_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__in` (array param, comma-joined).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKeyValue__contains` (array param, comma-joined).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__contains` (array param, comma-joined).
    pub fn resource_type_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue` (array param, comma-joined).
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
    /// Set `sortOrder`.
    pub fn sort_order(mut self, value: impl Into<String>) -> Self {
        self.sort_order = Some(value.into());
        self
    }
    /// Set `isPublic` (array param, comma-joined).
    pub fn is_public<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_public = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderOrganizationUnit__contains` (array param, comma-joined).
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderOrganization__contains` (array param, comma-joined).
    pub fn cloud_provider_organization_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationScanStatus` (array param, comma-joined).
    pub fn data_classification_scan_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_scan_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `countOnly`.
    pub fn count_only(mut self, value: bool) -> Self {
        self.count_only = Some(value);
        self
    }
    /// Set `isPublic__nin` (array param, comma-joined).
    pub fn is_public_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_public_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataTypes__nin` (array param, comma-joined).
    pub fn data_types_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names` (array param, comma-joined).
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
    /// Set `name__contains` (array param, comma-joined).
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality` (array param, comma-joined).
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
    /// Set `threatDetectionStatus__contains` (array param, comma-joined).
    pub fn threat_detection_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountId__nin` (array param, comma-joined).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `skipCount`.
    pub fn skip_count(mut self, value: bool) -> Self {
        self.skip_count = Some(value);
        self
    }
    /// Set `allTagsKey__nexists` (array param, comma-joined).
    pub fn all_tags_key_nexists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `siteIds` (array param, comma-joined).
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
    /// Set `s1UpdatedAt__between`.
    pub fn s1_updated_at_between(mut self, value: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(value.into());
        self
    }
    /// Set `resourceType` (array param, comma-joined).
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
    /// Set `assetEnvironment__nin` (array param, comma-joined).
    pub fn asset_environment_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names__nin` (array param, comma-joined).
    pub fn names_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey` (array param, comma-joined).
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
    /// Set `subCategory__nin` (array param, comma-joined).
    pub fn sub_category_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus` (array param, comma-joined).
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
    /// Set `connectivityStatus` (array param, comma-joined).
    pub fn connectivity_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.connectivity_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage` (array param, comma-joined).
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
    /// Set `countsFor` (array param, comma-joined).
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
    /// Set `csvFilterId`.
    pub fn csv_filter_id(mut self, value: i64) -> Self {
        self.csv_filter_id = Some(value);
        self
    }
    /// Set `threatDetectionStatus` (array param, comma-joined).
    pub fn threat_detection_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationPolicyStatus` (array param, comma-joined).
    pub fn data_classification_policy_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_policy_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountId` (array param, comma-joined).
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
    /// Set `subCategory` (array param, comma-joined).
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
    /// Set `dataClassificationStatus` (array param, comma-joined).
    pub fn data_classification_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview` (array param, comma-joined).
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
    /// Set `allTagsKey` (array param, comma-joined).
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
    /// Set `limit`.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set `scanStatus__contains` (array param, comma-joined).
    pub fn scan_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__contains` (array param, comma-joined).
    pub fn id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountName__contains` (array param, comma-joined).
    pub fn cloud_provider_account_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `scannerStatus` (array param, comma-joined).
    pub fn scanner_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scanner_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus` (array param, comma-joined).
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
    /// Set `versioningEnabled` (array param, comma-joined).
    pub fn versioning_enabled<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.versioning_enabled = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey__contains` (array param, comma-joined).
    pub fn cloud_tags_key_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__nin` (array param, comma-joined).
    pub fn tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__exists` (array param, comma-joined).
    pub fn all_tags_key_exists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationStatus__nin` (array param, comma-joined).
    pub fn data_classification_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cursor`.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }
    /// Set `scanStatus` (array param, comma-joined).
    pub fn scan_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderProjectId__contains` (array param, comma-joined).
    pub fn cloud_provider_project_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query parameters for `POST /web/api/v2.1/xdr/assets/data-store`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPostQuery {
    /// List of Account IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl ListPostQuery {
    /// Set `accountIds` (array param, comma-joined).
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
    /// Set `siteIds` (array param, comma-joined).
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
    /// Set `groupIds` (array param, comma-joined).
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

/// Query parameters for `POST /web/api/v2.1/xdr/assets/data-store/action`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionQuery {
    /// Free-text filter by tag key (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__contains")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetCriticality__nin")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "infectionStatus__nin")]
    pub infection_status_nin: Option<String>,
    /// Data types contains
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_types__contains")]
    pub data_types_contains: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountName__nin")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// Tag Keys (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__nin")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudResourceId__contains")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderSubscriptionId__contains")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__exists")]
    pub tags_key_exists: Option<String>,
    /// The CDS malware scan status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scanStatus__nin")]
    pub scan_status_nin: Option<String>,
    /// The region
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The encryption type (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "encryptionType__nin")]
    pub encryption_type_nin: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKeyValue__contains")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKey__nin")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "riskFactors__nin")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__nexists")]
    pub tags_key_nexists: Option<String>,
    /// The cloud provider account ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountId__contains")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "imageName__contains")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Threat Detection Status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "threatDetectionStatus__nin")]
    pub threat_detection_status_nin: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeCoverage__nin")]
    pub active_coverage_nin: Option<String>,
    /// Data classification status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_classification_status__contains")]
    pub data_classification_status_contains: Option<String>,
    /// User and cloud tags (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKeyValue__nin")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "region__nin")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__nin")]
    pub all_tags_key_nin: Option<String>,
    /// The cloud tags key value (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKeyValue__nin")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The number of objects in the bucket (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectCount__nin")]
    pub object_count_nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "missingCoverage__nin")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetStatus__nin")]
    pub asset_status_nin: Option<String>,
    /// The geographical area where cloud resources are hosted
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "region__contains")]
    pub region_contains: Option<String>,
    /// The asset review (not in)
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `(empty string)`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deviceReview__nin")]
    pub device_review_nin: Option<String>,
    /// Threat Detection Policy
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_policy_status: Option<String>,
    /// The cloud provider account name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetContactEmail__nin")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType__nin")]
    pub resource_type_nin: Option<String>,
    /// Asset Contact Email
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// Data classification scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_classification_scan_status__contains")]
    pub data_classification_scan_status_contains: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS \
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The number of objects in the bucket
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "surfaces__nin")]
    pub surfaces_nin: Option<String>,
    /// Data Types
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_types: Option<String>,
    /// The encryption type
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// The ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "id__in")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKeyValue__contains")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType__contains")]
    pub resource_type_contains: Option<String>,
    /// Tags
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Whether there is public access or not
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<String>,
    /// The cloud provider organization unit
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderOrganizationUnit__contains")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderOrganization__contains")]
    pub cloud_provider_organization_contains: Option<String>,
    /// Data classification scan status to exclude
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_scan_status: Option<String>,
    /// Whether there is public access or not (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "isPublic__nin")]
    pub is_public_nin: Option<String>,
    /// Data Types (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataTypes__nin")]
    pub data_types_nin: Option<String>,
    /// Name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "name__contains")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The threat detection status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "threatDetectionStatus__contains")]
    pub threat_detection_status_contains: Option<String>,
    /// The cloud provider account id (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountId__nin")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// User and cloud tag keys not exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__nexists")]
    pub all_tags_key_nexists: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "s1UpdatedAt__between")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS \
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetEnvironment__nin")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "names__nin")]
    pub names_nin: Option<String>,
    /// The cloud tags key
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "subCategory__nin")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// Connectivity
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_status: Option<String>,
    /// The active coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Threat Detection Status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_status: Option<String>,
    /// Data Classification Policy
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_policy_status: Option<String>,
    /// The cloud provider account id
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// Data Classification Status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_status: Option<String>,
    /// The asset review
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `(empty string)`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The CDS malware scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scanStatus__contains")]
    pub scan_status_contains: Option<String>,
    /// The ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "id__contains")]
    pub id_contains: Option<String>,
    /// The cloud provider account name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountName__contains")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// Scanner
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_status: Option<String>,
    /// The status of the asset
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether there is versioning enabled or not
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_enabled: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKey__contains")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKeyValue__nin")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__exists")]
    pub all_tags_key_exists: Option<String>,
    /// Data Classification Status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataClassificationStatus__nin")]
    pub data_classification_status_nin: Option<String>,
    /// The CDS malware scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// The cloud provider project ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderProjectId__contains")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl PerformActionQuery {
    /// Set `tagsKey__contains` (array param, comma-joined).
    pub fn tags_key_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality__nin` (array param, comma-joined).
    pub fn asset_criticality_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus__nin` (array param, comma-joined).
    pub fn infection_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `data_types__contains` (array param, comma-joined).
    pub fn data_types_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage` (array param, comma-joined).
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
    /// Set `allTagsKeyValue` (array param, comma-joined).
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
    /// Set `cloudProviderAccountName__nin` (array param, comma-joined).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nin` (array param, comma-joined).
    pub fn tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudResourceId__contains` (array param, comma-joined).
    pub fn cloud_resource_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderSubscriptionId__contains` (array param, comma-joined).
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__exists` (array param, comma-joined).
    pub fn tags_key_exists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `scanStatus__nin` (array param, comma-joined).
    pub fn scan_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `region` (array param, comma-joined).
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
    /// Set `encryptionType__nin` (array param, comma-joined).
    pub fn encryption_type_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__contains` (array param, comma-joined).
    pub fn tags_key_value_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey__nin` (array param, comma-joined).
    pub fn cloud_tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey` (array param, comma-joined).
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
    /// Set `riskFactors__nin` (array param, comma-joined).
    pub fn risk_factors_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nexists` (array param, comma-joined).
    pub fn tags_key_nexists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountId__contains` (array param, comma-joined).
    pub fn cloud_provider_account_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `imageName__contains` (array param, comma-joined).
    pub fn image_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `groupIds` (array param, comma-joined).
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
    /// Set `threatDetectionStatus__nin` (array param, comma-joined).
    pub fn threat_detection_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage__nin` (array param, comma-joined).
    pub fn active_coverage_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `data_classification_status__contains` (array param, comma-joined).
    pub fn data_classification_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue__nin` (array param, comma-joined).
    pub fn all_tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `region__nin` (array param, comma-joined).
    pub fn region_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nin` (array param, comma-joined).
    pub fn all_tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKeyValue__nin` (array param, comma-joined).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces` (array param, comma-joined).
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
    /// Set `objectCount__nin` (array param, comma-joined).
    pub fn object_count_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage__nin` (array param, comma-joined).
    pub fn missing_coverage_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus__nin` (array param, comma-joined).
    pub fn asset_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `region__contains` (array param, comma-joined).
    pub fn region_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview__nin` (array param, comma-joined).
    pub fn device_review_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `threatDetectionPolicyStatus` (array param, comma-joined).
    pub fn threat_detection_policy_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_policy_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountName` (array param, comma-joined).
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
    /// Set `assetContactEmail__nin` (array param, comma-joined).
    pub fn asset_contact_email_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `alertSeverity` (array param, comma-joined).
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
    /// Set `accountIds` (array param, comma-joined).
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
    /// Set `cloudTagsKeyValue` (array param, comma-joined).
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
    /// Set `resourceType__nin` (array param, comma-joined).
    pub fn resource_type_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail` (array param, comma-joined).
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
    /// Set `data_classification_scan_status__contains` (array param, comma-joined).
    pub fn data_classification_scan_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_scan_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors` (array param, comma-joined).
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
    /// Set `assetEnvironment` (array param, comma-joined).
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
    /// Set `objectCount` (array param, comma-joined).
    pub fn object_count<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces__nin` (array param, comma-joined).
    pub fn surfaces_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataTypes` (array param, comma-joined).
    pub fn data_types<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `encryptionType` (array param, comma-joined).
    pub fn encryption_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__in` (array param, comma-joined).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKeyValue__contains` (array param, comma-joined).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__contains` (array param, comma-joined).
    pub fn resource_type_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue` (array param, comma-joined).
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
    /// Set `isPublic` (array param, comma-joined).
    pub fn is_public<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_public = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderOrganizationUnit__contains` (array param, comma-joined).
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderOrganization__contains` (array param, comma-joined).
    pub fn cloud_provider_organization_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationScanStatus` (array param, comma-joined).
    pub fn data_classification_scan_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_scan_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `isPublic__nin` (array param, comma-joined).
    pub fn is_public_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_public_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataTypes__nin` (array param, comma-joined).
    pub fn data_types_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names` (array param, comma-joined).
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
    /// Set `name__contains` (array param, comma-joined).
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality` (array param, comma-joined).
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
    /// Set `threatDetectionStatus__contains` (array param, comma-joined).
    pub fn threat_detection_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountId__nin` (array param, comma-joined).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nexists` (array param, comma-joined).
    pub fn all_tags_key_nexists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `siteIds` (array param, comma-joined).
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
    /// Set `s1UpdatedAt__between`.
    pub fn s1_updated_at_between(mut self, value: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(value.into());
        self
    }
    /// Set `resourceType` (array param, comma-joined).
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
    /// Set `assetEnvironment__nin` (array param, comma-joined).
    pub fn asset_environment_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names__nin` (array param, comma-joined).
    pub fn names_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey` (array param, comma-joined).
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
    /// Set `subCategory__nin` (array param, comma-joined).
    pub fn sub_category_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus` (array param, comma-joined).
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
    /// Set `connectivityStatus` (array param, comma-joined).
    pub fn connectivity_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.connectivity_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage` (array param, comma-joined).
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
    /// Set `countsFor` (array param, comma-joined).
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
    /// Set `csvFilterId`.
    pub fn csv_filter_id(mut self, value: i64) -> Self {
        self.csv_filter_id = Some(value);
        self
    }
    /// Set `threatDetectionStatus` (array param, comma-joined).
    pub fn threat_detection_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationPolicyStatus` (array param, comma-joined).
    pub fn data_classification_policy_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_policy_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountId` (array param, comma-joined).
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
    /// Set `subCategory` (array param, comma-joined).
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
    /// Set `dataClassificationStatus` (array param, comma-joined).
    pub fn data_classification_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview` (array param, comma-joined).
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
    /// Set `allTagsKey` (array param, comma-joined).
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
    /// Set `scanStatus__contains` (array param, comma-joined).
    pub fn scan_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__contains` (array param, comma-joined).
    pub fn id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountName__contains` (array param, comma-joined).
    pub fn cloud_provider_account_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `scannerStatus` (array param, comma-joined).
    pub fn scanner_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scanner_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus` (array param, comma-joined).
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
    /// Set `versioningEnabled` (array param, comma-joined).
    pub fn versioning_enabled<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.versioning_enabled = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey__contains` (array param, comma-joined).
    pub fn cloud_tags_key_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__nin` (array param, comma-joined).
    pub fn tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__exists` (array param, comma-joined).
    pub fn all_tags_key_exists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationStatus__nin` (array param, comma-joined).
    pub fn data_classification_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `scanStatus` (array param, comma-joined).
    pub fn scan_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderProjectId__contains` (array param, comma-joined).
    pub fn cloud_provider_project_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query parameters for `POST /web/api/v2.1/xdr/assets/data-store/available-actions/with-status`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionsQuery {
    /// Free-text filter by tag key (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__contains")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetCriticality__nin")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "infectionStatus__nin")]
    pub infection_status_nin: Option<String>,
    /// Data types contains
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_types__contains")]
    pub data_types_contains: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountName__nin")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// Tag Keys (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__nin")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudResourceId__contains")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderSubscriptionId__contains")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__exists")]
    pub tags_key_exists: Option<String>,
    /// The CDS malware scan status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scanStatus__nin")]
    pub scan_status_nin: Option<String>,
    /// The region
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The encryption type (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "encryptionType__nin")]
    pub encryption_type_nin: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKeyValue__contains")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKey__nin")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "riskFactors__nin")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__nexists")]
    pub tags_key_nexists: Option<String>,
    /// The cloud provider account ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountId__contains")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "imageName__contains")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Threat Detection Status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "threatDetectionStatus__nin")]
    pub threat_detection_status_nin: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeCoverage__nin")]
    pub active_coverage_nin: Option<String>,
    /// Data classification status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_classification_status__contains")]
    pub data_classification_status_contains: Option<String>,
    /// User and cloud tags (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKeyValue__nin")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "region__nin")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__nin")]
    pub all_tags_key_nin: Option<String>,
    /// The cloud tags key value (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKeyValue__nin")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The number of objects in the bucket (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectCount__nin")]
    pub object_count_nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "missingCoverage__nin")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetStatus__nin")]
    pub asset_status_nin: Option<String>,
    /// The geographical area where cloud resources are hosted
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "region__contains")]
    pub region_contains: Option<String>,
    /// The asset review (not in)
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `(empty string)`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deviceReview__nin")]
    pub device_review_nin: Option<String>,
    /// Threat Detection Policy
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_policy_status: Option<String>,
    /// The cloud provider account name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetContactEmail__nin")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType__nin")]
    pub resource_type_nin: Option<String>,
    /// Asset Contact Email
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// Data classification scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_classification_scan_status__contains")]
    pub data_classification_scan_status_contains: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS \
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The number of objects in the bucket
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "surfaces__nin")]
    pub surfaces_nin: Option<String>,
    /// Data Types
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_types: Option<String>,
    /// The encryption type
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// The ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "id__in")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKeyValue__contains")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType__contains")]
    pub resource_type_contains: Option<String>,
    /// Tags
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Whether there is public access or not
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<String>,
    /// The cloud provider organization unit
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderOrganizationUnit__contains")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderOrganization__contains")]
    pub cloud_provider_organization_contains: Option<String>,
    /// Data classification scan status to exclude
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_scan_status: Option<String>,
    /// Whether there is public access or not (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "isPublic__nin")]
    pub is_public_nin: Option<String>,
    /// Data Types (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataTypes__nin")]
    pub data_types_nin: Option<String>,
    /// Name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "name__contains")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The threat detection status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "threatDetectionStatus__contains")]
    pub threat_detection_status_contains: Option<String>,
    /// The cloud provider account id (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountId__nin")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// User and cloud tag keys not exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__nexists")]
    pub all_tags_key_nexists: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "s1UpdatedAt__between")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS \
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetEnvironment__nin")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "names__nin")]
    pub names_nin: Option<String>,
    /// The cloud tags key
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "subCategory__nin")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// Connectivity
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_status: Option<String>,
    /// The active coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Threat Detection Status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_status: Option<String>,
    /// Data Classification Policy
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_policy_status: Option<String>,
    /// The cloud provider account id
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// Data Classification Status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_status: Option<String>,
    /// The asset review
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `(empty string)`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The CDS malware scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scanStatus__contains")]
    pub scan_status_contains: Option<String>,
    /// The ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "id__contains")]
    pub id_contains: Option<String>,
    /// The cloud provider account name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountName__contains")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// Scanner
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_status: Option<String>,
    /// The status of the asset
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether there is versioning enabled or not
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_enabled: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKey__contains")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKeyValue__nin")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__exists")]
    pub all_tags_key_exists: Option<String>,
    /// Data Classification Status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataClassificationStatus__nin")]
    pub data_classification_status_nin: Option<String>,
    /// The CDS malware scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// The cloud provider project ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderProjectId__contains")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl AvailableActionsQuery {
    /// Set `tagsKey__contains` (array param, comma-joined).
    pub fn tags_key_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality__nin` (array param, comma-joined).
    pub fn asset_criticality_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus__nin` (array param, comma-joined).
    pub fn infection_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `data_types__contains` (array param, comma-joined).
    pub fn data_types_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage` (array param, comma-joined).
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
    /// Set `allTagsKeyValue` (array param, comma-joined).
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
    /// Set `cloudProviderAccountName__nin` (array param, comma-joined).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nin` (array param, comma-joined).
    pub fn tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudResourceId__contains` (array param, comma-joined).
    pub fn cloud_resource_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderSubscriptionId__contains` (array param, comma-joined).
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__exists` (array param, comma-joined).
    pub fn tags_key_exists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `scanStatus__nin` (array param, comma-joined).
    pub fn scan_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `region` (array param, comma-joined).
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
    /// Set `encryptionType__nin` (array param, comma-joined).
    pub fn encryption_type_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__contains` (array param, comma-joined).
    pub fn tags_key_value_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey__nin` (array param, comma-joined).
    pub fn cloud_tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey` (array param, comma-joined).
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
    /// Set `riskFactors__nin` (array param, comma-joined).
    pub fn risk_factors_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nexists` (array param, comma-joined).
    pub fn tags_key_nexists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountId__contains` (array param, comma-joined).
    pub fn cloud_provider_account_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `imageName__contains` (array param, comma-joined).
    pub fn image_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `groupIds` (array param, comma-joined).
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
    /// Set `threatDetectionStatus__nin` (array param, comma-joined).
    pub fn threat_detection_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage__nin` (array param, comma-joined).
    pub fn active_coverage_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `data_classification_status__contains` (array param, comma-joined).
    pub fn data_classification_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue__nin` (array param, comma-joined).
    pub fn all_tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `region__nin` (array param, comma-joined).
    pub fn region_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nin` (array param, comma-joined).
    pub fn all_tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKeyValue__nin` (array param, comma-joined).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces` (array param, comma-joined).
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
    /// Set `objectCount__nin` (array param, comma-joined).
    pub fn object_count_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage__nin` (array param, comma-joined).
    pub fn missing_coverage_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus__nin` (array param, comma-joined).
    pub fn asset_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `region__contains` (array param, comma-joined).
    pub fn region_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview__nin` (array param, comma-joined).
    pub fn device_review_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `threatDetectionPolicyStatus` (array param, comma-joined).
    pub fn threat_detection_policy_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_policy_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountName` (array param, comma-joined).
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
    /// Set `assetContactEmail__nin` (array param, comma-joined).
    pub fn asset_contact_email_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `alertSeverity` (array param, comma-joined).
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
    /// Set `accountIds` (array param, comma-joined).
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
    /// Set `cloudTagsKeyValue` (array param, comma-joined).
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
    /// Set `resourceType__nin` (array param, comma-joined).
    pub fn resource_type_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail` (array param, comma-joined).
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
    /// Set `data_classification_scan_status__contains` (array param, comma-joined).
    pub fn data_classification_scan_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_scan_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors` (array param, comma-joined).
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
    /// Set `assetEnvironment` (array param, comma-joined).
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
    /// Set `objectCount` (array param, comma-joined).
    pub fn object_count<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces__nin` (array param, comma-joined).
    pub fn surfaces_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataTypes` (array param, comma-joined).
    pub fn data_types<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `encryptionType` (array param, comma-joined).
    pub fn encryption_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__in` (array param, comma-joined).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKeyValue__contains` (array param, comma-joined).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__contains` (array param, comma-joined).
    pub fn resource_type_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue` (array param, comma-joined).
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
    /// Set `isPublic` (array param, comma-joined).
    pub fn is_public<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_public = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderOrganizationUnit__contains` (array param, comma-joined).
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderOrganization__contains` (array param, comma-joined).
    pub fn cloud_provider_organization_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationScanStatus` (array param, comma-joined).
    pub fn data_classification_scan_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_scan_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `isPublic__nin` (array param, comma-joined).
    pub fn is_public_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_public_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataTypes__nin` (array param, comma-joined).
    pub fn data_types_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names` (array param, comma-joined).
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
    /// Set `name__contains` (array param, comma-joined).
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality` (array param, comma-joined).
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
    /// Set `threatDetectionStatus__contains` (array param, comma-joined).
    pub fn threat_detection_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountId__nin` (array param, comma-joined).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nexists` (array param, comma-joined).
    pub fn all_tags_key_nexists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `siteIds` (array param, comma-joined).
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
    /// Set `s1UpdatedAt__between`.
    pub fn s1_updated_at_between(mut self, value: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(value.into());
        self
    }
    /// Set `resourceType` (array param, comma-joined).
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
    /// Set `assetEnvironment__nin` (array param, comma-joined).
    pub fn asset_environment_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names__nin` (array param, comma-joined).
    pub fn names_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey` (array param, comma-joined).
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
    /// Set `subCategory__nin` (array param, comma-joined).
    pub fn sub_category_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus` (array param, comma-joined).
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
    /// Set `connectivityStatus` (array param, comma-joined).
    pub fn connectivity_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.connectivity_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage` (array param, comma-joined).
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
    /// Set `countsFor` (array param, comma-joined).
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
    /// Set `csvFilterId`.
    pub fn csv_filter_id(mut self, value: i64) -> Self {
        self.csv_filter_id = Some(value);
        self
    }
    /// Set `threatDetectionStatus` (array param, comma-joined).
    pub fn threat_detection_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationPolicyStatus` (array param, comma-joined).
    pub fn data_classification_policy_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_policy_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountId` (array param, comma-joined).
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
    /// Set `subCategory` (array param, comma-joined).
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
    /// Set `dataClassificationStatus` (array param, comma-joined).
    pub fn data_classification_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview` (array param, comma-joined).
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
    /// Set `allTagsKey` (array param, comma-joined).
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
    /// Set `scanStatus__contains` (array param, comma-joined).
    pub fn scan_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__contains` (array param, comma-joined).
    pub fn id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountName__contains` (array param, comma-joined).
    pub fn cloud_provider_account_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `scannerStatus` (array param, comma-joined).
    pub fn scanner_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scanner_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus` (array param, comma-joined).
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
    /// Set `versioningEnabled` (array param, comma-joined).
    pub fn versioning_enabled<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.versioning_enabled = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey__contains` (array param, comma-joined).
    pub fn cloud_tags_key_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__nin` (array param, comma-joined).
    pub fn tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__exists` (array param, comma-joined).
    pub fn all_tags_key_exists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationStatus__nin` (array param, comma-joined).
    pub fn data_classification_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `scanStatus` (array param, comma-joined).
    pub fn scan_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderProjectId__contains` (array param, comma-joined).
    pub fn cloud_provider_project_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Query parameters for `GET /web/api/v2.1/xdr/assets/data-store/export`.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportQuery {
    /// Free-text filter by tag key (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__contains")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in)
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetCriticality__nin")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in)
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "infectionStatus__nin")]
    pub infection_status_nin: Option<String>,
    /// Data types contains
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_types__contains")]
    pub data_types_contains: Option<String>,
    /// The missing coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountName__nin")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// Tag Keys (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__nin")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudResourceId__contains")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderSubscriptionId__contains")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__exists")]
    pub tags_key_exists: Option<String>,
    /// The CDS malware scan status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scanStatus__nin")]
    pub scan_status_nin: Option<String>,
    /// The region
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The encryption type (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "encryptionType__nin")]
    pub encryption_type_nin: Option<String>,
    /// Free-text filter by tag key value (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKeyValue__contains")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKey__nin")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in)
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "riskFactors__nin")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKey__nexists")]
    pub tags_key_nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000 items,  use "cursor".
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountId__contains")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "imageName__contains")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// Threat Detection Status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "threatDetectionStatus__nin")]
    pub threat_detection_status_nin: Option<String>,
    /// The active coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "activeCoverage__nin")]
    pub active_coverage_nin: Option<String>,
    /// Data classification status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_classification_status__contains")]
    pub data_classification_status_contains: Option<String>,
    /// User and cloud tags (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKeyValue__nin")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "region__nin")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__nin")]
    pub all_tags_key_nin: Option<String>,
    /// The cloud tags key value (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKeyValue__nin")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The number of objects in the bucket (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectCount__nin")]
    pub object_count_nin: Option<String>,
    /// The missing coverage for the asset (not in)
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "missingCoverage__nin")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in)
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetStatus__nin")]
    pub asset_status_nin: Option<String>,
    /// The column to sort the results by.
    ///
    /// Allowed values: `s1GroupName`, `monitoringEnabled`, `cnsMonitorTargetEnabled`, `loggingEnabled`, `encryptionEnabled`, `s1UpdatedAt`, `cnsVolumeGroupIdExists`, `cnsRetentionDays`.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The geographical area where cloud resources are hosted
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "region__contains")]
    pub region_contains: Option<String>,
    /// The asset review (not in)
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `(empty string)`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "deviceReview__nin")]
    pub device_review_nin: Option<String>,
    /// Threat Detection Policy
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_policy_status: Option<String>,
    /// The cloud provider account name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetContactEmail__nin")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in)
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType__nin")]
    pub resource_type_nin: Option<String>,
    /// Asset Contact Email
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// Data classification scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "data_classification_scan_status__contains")]
    pub data_classification_scan_status_contains: Option<String>,
    /// The risk factors associated with the asset
    ///
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS \
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The number of objects in the bucket
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<String>,
    /// The Surface that each asset belongs to (not in)
    ///
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "surfaces__nin")]
    pub surfaces_nin: Option<String>,
    /// Data Types
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_types: Option<String>,
    /// The encryption type
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// The ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "id__in")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKeyValue__contains")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "resourceType__contains")]
    pub resource_type_contains: Option<String>,
    /// Tags
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction
    ///
    /// Allowed values: `asc`, `desc`.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// Whether there is public access or not
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<String>,
    /// The cloud provider organization unit
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderOrganizationUnit__contains")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderOrganization__contains")]
    pub cloud_provider_organization_contains: Option<String>,
    /// Data classification scan status to exclude
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_scan_status: Option<String>,
    /// If true, only total number of items will be returned, without any of the actual objects.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Whether there is public access or not (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "isPublic__nin")]
    pub is_public_nin: Option<String>,
    /// Data Types (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataTypes__nin")]
    pub data_types_nin: Option<String>,
    /// Name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "name__contains")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to
    ///
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The threat detection status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "threatDetectionStatus__contains")]
    pub threat_detection_status_contains: Option<String>,
    /// The cloud provider account id (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountId__nin")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up execution time.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__nexists")]
    pub all_tags_key_nexists: Option<String>,
    /// List of Site IDs to filter by
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "s1UpdatedAt__between")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type
    ///
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`, `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS \
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "assetEnvironment__nin")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "names__nin")]
    pub names_nin: Option<String>,
    /// The cloud tags key
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in)
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "subCategory__nin")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset
    ///
    /// Allowed values: `Infected`, `Healthy`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// Connectivity
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_status: Option<String>,
    /// The active coverage for the asset
    ///
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// Threat Detection Status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_detection_status: Option<String>,
    /// Export format
    ///
    /// Allowed values: `csv`, `json`.
    ///
    /// Required: yes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<String>,
    /// Data Classification Policy
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_policy_status: Option<String>,
    /// The cloud provider account id
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to
    ///
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`, `Account`, `Account Group`, `AD Objects`, `Administrative Unit`, `Admission Controller`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// Data Classification Status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_classification_status: Option<String>,
    /// The asset review
    ///
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`, `Allowed`, `(empty string)`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Limit number of returned items (1-1000)
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The CDS malware scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scanStatus__contains")]
    pub scan_status_contains: Option<String>,
    /// The ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "id__contains")]
    pub id_contains: Option<String>,
    /// The cloud provider account name
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderAccountName__contains")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// Scanner
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanner_status: Option<String>,
    /// The status of the asset
    ///
    /// Allowed values: `Active`, `Inactive`.
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Whether there is versioning enabled or not
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_enabled: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudTagsKey__contains")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "tagsKeyValue__nin")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "allTagsKey__exists")]
    pub all_tags_key_exists: Option<String>,
    /// Data Classification Status (not in)
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataClassificationStatus__nin")]
    pub data_classification_status_nin: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more than 1000 items.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The CDS malware scan status
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_status: Option<String>,
    /// The cloud provider project ID
    ///
    /// Array query param: serialized comma-joined.
    ///
    /// Required: no (optional).
    #[serde(skip_serializing_if = "Option::is_none", rename = "cloudProviderProjectId__contains")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl ExportQuery {
    /// Set `tagsKey__contains` (array param, comma-joined).
    pub fn tags_key_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality__nin` (array param, comma-joined).
    pub fn asset_criticality_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus__nin` (array param, comma-joined).
    pub fn infection_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `data_types__contains` (array param, comma-joined).
    pub fn data_types_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage` (array param, comma-joined).
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
    /// Set `allTagsKeyValue` (array param, comma-joined).
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
    /// Set `cloudProviderAccountName__nin` (array param, comma-joined).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nin` (array param, comma-joined).
    pub fn tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudResourceId__contains` (array param, comma-joined).
    pub fn cloud_resource_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderSubscriptionId__contains` (array param, comma-joined).
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__exists` (array param, comma-joined).
    pub fn tags_key_exists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `scanStatus__nin` (array param, comma-joined).
    pub fn scan_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `region` (array param, comma-joined).
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
    /// Set `encryptionType__nin` (array param, comma-joined).
    pub fn encryption_type_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__contains` (array param, comma-joined).
    pub fn tags_key_value_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey__nin` (array param, comma-joined).
    pub fn cloud_tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey` (array param, comma-joined).
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
    /// Set `riskFactors__nin` (array param, comma-joined).
    pub fn risk_factors_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKey__nexists` (array param, comma-joined).
    pub fn tags_key_nexists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `skip`.
    pub fn skip(mut self, value: i64) -> Self {
        self.skip = Some(value);
        self
    }
    /// Set `cloudProviderAccountId__contains` (array param, comma-joined).
    pub fn cloud_provider_account_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `imageName__contains` (array param, comma-joined).
    pub fn image_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `groupIds` (array param, comma-joined).
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
    /// Set `threatDetectionStatus__nin` (array param, comma-joined).
    pub fn threat_detection_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage__nin` (array param, comma-joined).
    pub fn active_coverage_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `data_classification_status__contains` (array param, comma-joined).
    pub fn data_classification_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKeyValue__nin` (array param, comma-joined).
    pub fn all_tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `region__nin` (array param, comma-joined).
    pub fn region_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__nin` (array param, comma-joined).
    pub fn all_tags_key_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKeyValue__nin` (array param, comma-joined).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces` (array param, comma-joined).
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
    /// Set `objectCount__nin` (array param, comma-joined).
    pub fn object_count_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `missingCoverage__nin` (array param, comma-joined).
    pub fn missing_coverage_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus__nin` (array param, comma-joined).
    pub fn asset_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `sortBy`.
    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }
    /// Set `region__contains` (array param, comma-joined).
    pub fn region_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview__nin` (array param, comma-joined).
    pub fn device_review_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `threatDetectionPolicyStatus` (array param, comma-joined).
    pub fn threat_detection_policy_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_policy_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountName` (array param, comma-joined).
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
    /// Set `assetContactEmail__nin` (array param, comma-joined).
    pub fn asset_contact_email_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `alertSeverity` (array param, comma-joined).
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
    /// Set `accountIds` (array param, comma-joined).
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
    /// Set `cloudTagsKeyValue` (array param, comma-joined).
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
    /// Set `resourceType__nin` (array param, comma-joined).
    pub fn resource_type_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetContactEmail` (array param, comma-joined).
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
    /// Set `data_classification_scan_status__contains` (array param, comma-joined).
    pub fn data_classification_scan_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_scan_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `riskFactors` (array param, comma-joined).
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
    /// Set `assetEnvironment` (array param, comma-joined).
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
    /// Set `objectCount` (array param, comma-joined).
    pub fn object_count<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.object_count = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `surfaces__nin` (array param, comma-joined).
    pub fn surfaces_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataTypes` (array param, comma-joined).
    pub fn data_types<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `encryptionType` (array param, comma-joined).
    pub fn encryption_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.encryption_type = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__in` (array param, comma-joined).
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKeyValue__contains` (array param, comma-joined).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `resourceType__contains` (array param, comma-joined).
    pub fn resource_type_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue` (array param, comma-joined).
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
    /// Set `sortOrder`.
    pub fn sort_order(mut self, value: impl Into<String>) -> Self {
        self.sort_order = Some(value.into());
        self
    }
    /// Set `isPublic` (array param, comma-joined).
    pub fn is_public<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_public = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderOrganizationUnit__contains` (array param, comma-joined).
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderOrganization__contains` (array param, comma-joined).
    pub fn cloud_provider_organization_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationScanStatus` (array param, comma-joined).
    pub fn data_classification_scan_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_scan_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `countOnly`.
    pub fn count_only(mut self, value: bool) -> Self {
        self.count_only = Some(value);
        self
    }
    /// Set `isPublic__nin` (array param, comma-joined).
    pub fn is_public_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.is_public_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataTypes__nin` (array param, comma-joined).
    pub fn data_types_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_types_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names` (array param, comma-joined).
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
    /// Set `name__contains` (array param, comma-joined).
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetCriticality` (array param, comma-joined).
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
    /// Set `threatDetectionStatus__contains` (array param, comma-joined).
    pub fn threat_detection_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountId__nin` (array param, comma-joined).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `skipCount`.
    pub fn skip_count(mut self, value: bool) -> Self {
        self.skip_count = Some(value);
        self
    }
    /// Set `allTagsKey__nexists` (array param, comma-joined).
    pub fn all_tags_key_nexists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `siteIds` (array param, comma-joined).
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
    /// Set `s1UpdatedAt__between`.
    pub fn s1_updated_at_between(mut self, value: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(value.into());
        self
    }
    /// Set `resourceType` (array param, comma-joined).
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
    /// Set `assetEnvironment__nin` (array param, comma-joined).
    pub fn asset_environment_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `names__nin` (array param, comma-joined).
    pub fn names_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey` (array param, comma-joined).
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
    /// Set `subCategory__nin` (array param, comma-joined).
    pub fn sub_category_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `infectionStatus` (array param, comma-joined).
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
    /// Set `connectivityStatus` (array param, comma-joined).
    pub fn connectivity_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.connectivity_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `activeCoverage` (array param, comma-joined).
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
    /// Set `countsFor` (array param, comma-joined).
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
    /// Set `csvFilterId`.
    pub fn csv_filter_id(mut self, value: i64) -> Self {
        self.csv_filter_id = Some(value);
        self
    }
    /// Set `threatDetectionStatus` (array param, comma-joined).
    pub fn threat_detection_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.threat_detection_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `exportFormat`.
    pub fn export_format(mut self, value: impl Into<String>) -> Self {
        self.export_format = Some(value.into());
        self
    }
    /// Set `dataClassificationPolicyStatus` (array param, comma-joined).
    pub fn data_classification_policy_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_policy_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountId` (array param, comma-joined).
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
    /// Set `subCategory` (array param, comma-joined).
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
    /// Set `dataClassificationStatus` (array param, comma-joined).
    pub fn data_classification_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `deviceReview` (array param, comma-joined).
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
    /// Set `allTagsKey` (array param, comma-joined).
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
    /// Set `limit`.
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }
    /// Set `scanStatus__contains` (array param, comma-joined).
    pub fn scan_status_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `id__contains` (array param, comma-joined).
    pub fn id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderAccountName__contains` (array param, comma-joined).
    pub fn cloud_provider_account_name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `scannerStatus` (array param, comma-joined).
    pub fn scanner_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scanner_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `assetStatus` (array param, comma-joined).
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
    /// Set `versioningEnabled` (array param, comma-joined).
    pub fn versioning_enabled<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.versioning_enabled = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudTagsKey__contains` (array param, comma-joined).
    pub fn cloud_tags_key_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `tagsKeyValue__nin` (array param, comma-joined).
    pub fn tags_key_value_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `allTagsKey__exists` (array param, comma-joined).
    pub fn all_tags_key_exists<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `dataClassificationStatus__nin` (array param, comma-joined).
    pub fn data_classification_status_nin<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.data_classification_status_nin = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cursor`.
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }
    /// Set `scanStatus` (array param, comma-joined).
    pub fn scan_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.scan_status = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
    /// Set `cloudProviderProjectId__contains` (array param, comma-joined).
    pub fn cloud_provider_project_id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(
            values
                .into_iter()
                .map(|s| s.as_ref().to_owned())
                .collect::<Vec<_>>()
                .join(","),
        );
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/data-store`
/// (`v2_1.inventory.data_store.schemas_DataStoreViewInputSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPostBody {
    /// Data.
    ///
    /// Freeform object in the spec (`EmptyStrict`); optional/nullable -> `Option`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,

    /// Filter.
    ///
    /// Deeply nested filter object (`PaginatedDataStoreFilter`); represented as a
    /// freeform JSON value. Required: yes.
    pub filter: serde_json::Value,
}

/// Request body for `POST /web/api/v2.1/xdr/assets/data-store/action`
/// (`v2_1.inventory.data_store.schemas_DataStoreActionPayloadSchema`).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionBody {
    /// Action name (enum in the spec; kept as `String` for forward-compatibility).
    ///
    /// Allowed values: `start_full_scan`, `stop_full_scan`,
    /// `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`, `add_note`,
    /// `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    ///
    /// Required: yes.
    pub action_name: String,

    /// List of selected inventory ids (max 5000).
    ///
    /// Optional -> `Option`.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,

    /// List of inventory ids to exclude from select_all (max 5000).
    ///
    /// Optional -> `Option`.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/data-store/available-actions/with-status`
/// (`v2_1.inventory.schemas_AffectedResourcesSchema`).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionsBody {
    /// List of selected inventory ids (max 5000).
    ///
    /// Optional -> `Option`.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<Vec<String>>,

    /// List of inventory ids to exclude from select_all (max 5000).
    ///
    /// Optional -> `Option`.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id_nin: Option<Vec<String>>,
}

/// Builds `path?querystring`, omitting the `?` when the query is empty.
fn path_with_query(path: &str, query: &impl Serialize) -> String {
    let qs = serde_urlencoded::to_string(query).unwrap_or_default();
    if qs.is_empty() {
        path.to_owned()
    } else {
        format!("{path}?{qs}")
    }
}

impl InventoryDataStoreService<'_> {
    /// Assets.
    ///
    /// Get assets
    ///
    /// `GET /web/api/v2.1/xdr/assets/data-store`
    pub async fn list(&self, query: &ListQuery) -> Result<Paginated<DataStoreAsset>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/xdr/assets/data-store", q).await?)
    }

    /// Assets using POST.
    ///
    /// POST API to get Assets
    ///
    /// `POST /web/api/v2.1/xdr/assets/data-store`
    pub async fn list_post(
        &self,
        query: &ListPostQuery,
        body: &ListPostBody,
    ) -> Result<Paginated<DataStoreAsset>, Error> {
        let path = path_with_query("/web/api/v2.1/xdr/assets/data-store", query);
        Ok(self.client.http().post(&path, body).await?)
    }

    /// Perform action.
    ///
    /// Perform action on selected assets
    ///
    /// `POST /web/api/v2.1/xdr/assets/data-store/action`
    pub async fn perform_action(
        &self,
        query: &PerformActionQuery,
        body: &PerformActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let path = path_with_query("/web/api/v2.1/xdr/assets/data-store/action", query);
        Ok(self.client.http().post(&path, body).await?)
    }

    /// Available actions.
    ///
    /// Get available actions
    ///
    /// `POST /web/api/v2.1/xdr/assets/data-store/available-actions/with-status`
    pub async fn available_actions(
        &self,
        query: &AvailableActionsQuery,
        body: &AvailableActionsBody,
    ) -> Result<Response<AvailableActionWithStatusResponse>, Error> {
        let path = path_with_query("/web/api/v2.1/xdr/assets/data-store/available-actions/with-status", query);
        Ok(self.client.http().post(&path, body).await?)
    }

    /// Export assets to CSV or JSON.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format
    ///
    /// `GET /web/api/v2.1/xdr/assets/data-store/export`
    pub async fn export(&self, query: &ExportQuery) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/xdr/assets/data-store/export", q).await?)
    }

}
