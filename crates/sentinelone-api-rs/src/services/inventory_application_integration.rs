//! Service for the `Inventory Application Integration` tag
//! (Inventory Application Integration Resources).

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_application_integration::{
    ApplicationIntegration, AvailableActionWithStatusResponse,
};
use crate::pagination::{Paginated, Response};

/// `Inventory Application Integration` tag — Inventory Application Integration
/// Resources.
///
/// Exposes the application-integration asset inventory: list/query assets,
/// perform bulk actions, discover available actions, and export results.
pub struct InventoryApplicationIntegrationService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Joins an iterator of string-like values into a comma-separated string,
/// as the SentinelOne API expects for array query parameters.
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

/// Query params for `GET /web/api/v2.1/xdr/assets/application-integration`
/// ("Assets") and `GET /web/api/v2.1/xdr/assets/application-integration/export`
/// ("Export assets to CSV or JSON").
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects. Enum params are kept as `String` for forward-compatibility; the
/// allowed values are documented per field.
///
/// Note: the export endpoint additionally requires `export_format`, which is a
/// dedicated function argument rather than a field on this struct.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The missing coverage for the asset. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The region. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID. Optional.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The active coverage for the asset (not in). Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in). Optional.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to. Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The missing coverage for the asset (not in). Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). Optional.
    /// Allowed values: `Active`, `Inactive`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The column to sort the results by. Optional.
    /// Allowed values: `s1GroupName`, `cnsConfirmedSubscriptionCount`,
    /// `s1OnboardedAccountName`, `cloudProviderProjectId`, `category`,
    /// `s1UpdatedAt`, `s1GroupId`, `cloudProviderAccountId`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The asset review (not in). Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty).
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// The severity of the alert. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in). Optional.
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in). Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    /// Optional.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// Tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to. Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Optional.
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory (not in). Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in). Optional.
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset. Optional.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The cloud provider account id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to. Optional.
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review. Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID. Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset. Optional. Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Optional.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The status alerts of the asset (not in). Optional.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl AssetsQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_contains = Some(join_csv(v)); self }
    /// The criticality that each asset belongs to (not in).
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_criticality_nin = Some(join_csv(v)); self }
    /// The missing coverage for the asset.
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.missing_coverage = Some(join_csv(v)); self }
    /// User and cloud tags.
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_value = Some(join_csv(v)); self }
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name_nin = Some(join_csv(v)); self }
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_nin = Some(join_csv(v)); self }
    /// The cloud resource ID.
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_resource_id_contains = Some(join_csv(v)); self }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_subscription_id_contains = Some(join_csv(v)); self }
    /// Tag Keys exists.
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_exists = Some(join_csv(v)); self }
    /// The region.
    pub fn region<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region = Some(join_csv(v)); self }
    /// Free-text filter by tag key value (supports multiple values).
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value_contains = Some(join_csv(v)); self }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_nin = Some(join_csv(v)); self }
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key = Some(join_csv(v)); self }
    /// The risk factors associated with the asset (not in).
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.risk_factors_nin = Some(join_csv(v)); self }
    /// Tag Keys not exists.
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_nexists = Some(join_csv(v)); self }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self { self.skip = Some(n); self }
    /// The cloud provider account ID.
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id_contains = Some(join_csv(v)); self }
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.image_name_contains = Some(join_csv(v)); self }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.group_ids = Some(join_csv(v)); self }
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.active_coverage_nin = Some(join_csv(v)); self }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_value_nin = Some(join_csv(v)); self }
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region_nin = Some(join_csv(v)); self }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_nin = Some(join_csv(v)); self }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value_nin = Some(join_csv(v)); self }
    /// The Surface that each asset belongs to.
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.surfaces = Some(join_csv(v)); self }
    /// The missing coverage for the asset (not in).
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.missing_coverage_nin = Some(join_csv(v)); self }
    /// The status of the asset (not in).
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_status_nin = Some(join_csv(v)); self }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self { self.sort_by = Some(v.into()); self }
    /// The geographical area where cloud resources are hosted.
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region_contains = Some(join_csv(v)); self }
    /// The asset review (not in).
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.device_review_nin = Some(join_csv(v)); self }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name = Some(join_csv(v)); self }
    /// Asset Contact Email (not in).
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_contact_email_nin = Some(join_csv(v)); self }
    /// The severity of the alert.
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.alert_severity = Some(join_csv(v)); self }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.account_ids = Some(join_csv(v)); self }
    /// The cloud tags key value.
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value = Some(join_csv(v)); self }
    /// The canonical name for the resource type (not in).
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type_nin = Some(join_csv(v)); self }
    /// Asset Contact Email.
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_contact_email = Some(join_csv(v)); self }
    /// The risk factors associated with the asset.
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.risk_factors = Some(join_csv(v)); self }
    /// The environment that the asset exists in.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_environment = Some(join_csv(v)); self }
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.surfaces_nin = Some(join_csv(v)); self }
    /// The ID.
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.id_in = Some(join_csv(v)); self }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value_contains = Some(join_csv(v)); self }
    /// The Asset Type.
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type_contains = Some(join_csv(v)); self }
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value = Some(join_csv(v)); self }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self { self.sort_order = Some(v.into()); self }
    /// The cloud provider organization unit.
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_organization_unit_contains = Some(join_csv(v)); self }
    /// The cloud provider organization.
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_organization_contains = Some(join_csv(v)); self }
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self { self.count_only = Some(v); self }
    /// Name.
    pub fn names<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.names = Some(join_csv(v)); self }
    /// The name.
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.name_contains = Some(join_csv(v)); self }
    /// The criticality that each asset belongs to.
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_criticality = Some(join_csv(v)); self }
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id_nin = Some(join_csv(v)); self }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self { self.skip_count = Some(v); self }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_nexists = Some(join_csv(v)); self }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.site_ids = Some(join_csv(v)); self }
    /// The Last Seen date and time for the asset.
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self { self.s1_updated_at_between = Some(v.into()); self }
    /// The canonical name for the resource type.
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type = Some(join_csv(v)); self }
    /// The environment that the asset exists in (not in).
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_environment_nin = Some(join_csv(v)); self }
    /// Name (not in).
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.names_nin = Some(join_csv(v)); self }
    /// The cloud tags key.
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key = Some(join_csv(v)); self }
    /// The sub-category that each resource belongs to (not in).
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sub_category_nin = Some(join_csv(v)); self }
    /// The status alerts of the asset.
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.infection_status = Some(join_csv(v)); self }
    /// The active coverage for the asset.
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.active_coverage = Some(join_csv(v)); self }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.counts_for = Some(join_csv(v)); self }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self { self.csv_filter_id = Some(n); self }
    /// The cloud provider account id.
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id = Some(join_csv(v)); self }
    /// The sub-category that each resource belongs to.
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sub_category = Some(join_csv(v)); self }
    /// The asset review.
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.device_review = Some(join_csv(v)); self }
    /// User and cloud tag keys.
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key = Some(join_csv(v)); self }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self { self.limit = Some(n); self }
    /// The ID.
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.id_contains = Some(join_csv(v)); self }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name_contains = Some(join_csv(v)); self }
    /// The status of the asset.
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_status = Some(join_csv(v)); self }
    /// Free-text filter by cloud tag key (supports multiple values).
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_contains = Some(join_csv(v)); self }
    /// Tags (not in).
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value_nin = Some(join_csv(v)); self }
    /// User and cloud tag keys exists.
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_exists = Some(join_csv(v)); self }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self { self.cursor = Some(c.into()); self }
    /// The status alerts of the asset (not in).
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.infection_status_nin = Some(join_csv(v)); self }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_project_id_contains = Some(join_csv(v)); self }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/application-integration`
/// ("Assets using POST"). Every field is optional. Array params are serialized
/// comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsPostQuery {
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

impl AssetsPostQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.account_ids = Some(join_csv(v)); self }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.site_ids = Some(join_csv(v)); self }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.group_ids = Some(join_csv(v)); self }
}

/// Query params shared by the "Perform action"
/// (`POST .../application-integration/action`) and "Available actions"
/// (`POST .../application-integration/available-actions/with-status`)
/// endpoints. Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetActionQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in). Optional.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// Tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The missing coverage for the asset. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(rename = "cloudProviderAccountName__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(rename = "cloudResourceId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(rename = "cloudProviderSubscriptionId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id_contains: Option<String>,
    /// The cloud provider account id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The asset review (not in). Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty).
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
    /// The sub-category that each resource belongs to. Optional.
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(rename = "tagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub tags_key_exists: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// The status of the asset (not in). Optional.
    /// Allowed values: `Active`, `Inactive`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The asset review. Optional.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(rename = "assetContactEmail__nin", skip_serializing_if = "Option::is_none")]
    pub asset_contact_email_nin: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The region. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The severity of the alert. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The name. Optional.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to. Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The risk factors associated with the asset (not in). Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(rename = "riskFactors__nin", skip_serializing_if = "Option::is_none")]
    pub risk_factors_nin: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(rename = "tagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub tags_key_nexists: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// The canonical name for the resource type (not in). Optional.
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// The ID of the CSV file to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The ID. Optional.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account ID. Optional.
    #[serde(rename = "cloudProviderAccountId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset. Optional. Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Optional.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The active coverage for the asset (not in). Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Optional.
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in (not in). Optional.
    #[serde(rename = "assetEnvironment__nin", skip_serializing_if = "Option::is_none")]
    pub asset_environment_nin: Option<String>,
    /// The missing coverage for the asset (not in). Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(rename = "allTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value_nin: Option<String>,
    /// The region (not in). Optional.
    #[serde(rename = "region__nin", skip_serializing_if = "Option::is_none")]
    pub region_nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(rename = "allTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(rename = "names__nin", skip_serializing_if = "Option::is_none")]
    pub names_nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Optional.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The Surface that each asset belongs to (not in). Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(rename = "cloudTagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_nin: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The Surface that each asset belongs to. Optional.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The sub-category that each resource belongs to (not in). Optional.
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The ID. Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// The status alerts of the asset. Optional.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset. Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    /// Optional.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl AssetActionQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_contains = Some(join_csv(v)); self }
    /// The Asset Type.
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type_contains = Some(join_csv(v)); self }
    /// The criticality that each asset belongs to (not in).
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_criticality_nin = Some(join_csv(v)); self }
    /// The status alerts of the asset (not in).
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.infection_status_nin = Some(join_csv(v)); self }
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value = Some(join_csv(v)); self }
    /// The missing coverage for the asset.
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.missing_coverage = Some(join_csv(v)); self }
    /// User and cloud tags.
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_value = Some(join_csv(v)); self }
    /// The cloud provider organization unit.
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_organization_unit_contains = Some(join_csv(v)); self }
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name_nin = Some(join_csv(v)); self }
    /// The geographical area where cloud resources are hosted.
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region_contains = Some(join_csv(v)); self }
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_nin = Some(join_csv(v)); self }
    /// The cloud resource ID.
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_resource_id_contains = Some(join_csv(v)); self }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_subscription_id_contains = Some(join_csv(v)); self }
    /// The cloud provider account id.
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id = Some(join_csv(v)); self }
    /// The asset review (not in).
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.device_review_nin = Some(join_csv(v)); self }
    /// The sub-category that each resource belongs to.
    pub fn sub_category<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sub_category = Some(join_csv(v)); self }
    /// The cloud provider organization.
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_organization_contains = Some(join_csv(v)); self }
    /// Tag Keys exists.
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_exists = Some(join_csv(v)); self }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name = Some(join_csv(v)); self }
    /// The status of the asset (not in).
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_status_nin = Some(join_csv(v)); self }
    /// The asset review.
    pub fn device_review<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.device_review = Some(join_csv(v)); self }
    /// Asset Contact Email (not in).
    pub fn asset_contact_email_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_contact_email_nin = Some(join_csv(v)); self }
    /// User and cloud tag keys.
    pub fn all_tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key = Some(join_csv(v)); self }
    /// The region.
    pub fn region<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region = Some(join_csv(v)); self }
    /// The severity of the alert.
    pub fn alert_severity<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.alert_severity = Some(join_csv(v)); self }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.account_ids = Some(join_csv(v)); self }
    /// The cloud tags key value.
    pub fn cloud_tags_key_value<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value = Some(join_csv(v)); self }
    /// Name.
    pub fn names<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.names = Some(join_csv(v)); self }
    /// Free-text filter by tag key value (supports multiple values).
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value_contains = Some(join_csv(v)); self }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_nin = Some(join_csv(v)); self }
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key = Some(join_csv(v)); self }
    /// The name.
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.name_contains = Some(join_csv(v)); self }
    /// The criticality that each asset belongs to.
    pub fn asset_criticality<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_criticality = Some(join_csv(v)); self }
    /// The risk factors associated with the asset (not in).
    pub fn risk_factors_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.risk_factors_nin = Some(join_csv(v)); self }
    /// Tag Keys not exists.
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_nexists = Some(join_csv(v)); self }
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id_nin = Some(join_csv(v)); self }
    /// The canonical name for the resource type (not in).
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type_nin = Some(join_csv(v)); self }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self { self.csv_filter_id = Some(n); self }
    /// The ID.
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.id_contains = Some(join_csv(v)); self }
    /// The cloud provider account ID.
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_id_contains = Some(join_csv(v)); self }
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.image_name_contains = Some(join_csv(v)); self }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.group_ids = Some(join_csv(v)); self }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_account_name_contains = Some(join_csv(v)); self }
    /// The status of the asset.
    pub fn asset_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_status = Some(join_csv(v)); self }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_nexists = Some(join_csv(v)); self }
    /// Asset Contact Email.
    pub fn asset_contact_email<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_contact_email = Some(join_csv(v)); self }
    /// The risk factors associated with the asset.
    pub fn risk_factors<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.risk_factors = Some(join_csv(v)); self }
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.active_coverage_nin = Some(join_csv(v)); self }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.site_ids = Some(join_csv(v)); self }
    /// The environment that the asset exists in.
    pub fn asset_environment<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_environment = Some(join_csv(v)); self }
    /// The Last Seen date and time for the asset.
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self { self.s1_updated_at_between = Some(v.into()); self }
    /// The canonical name for the resource type.
    pub fn resource_type<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.resource_type = Some(join_csv(v)); self }
    /// The environment that the asset exists in (not in).
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.asset_environment_nin = Some(join_csv(v)); self }
    /// The missing coverage for the asset (not in).
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.missing_coverage_nin = Some(join_csv(v)); self }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_value_nin = Some(join_csv(v)); self }
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.region_nin = Some(join_csv(v)); self }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_nin = Some(join_csv(v)); self }
    /// Name (not in).
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.names_nin = Some(join_csv(v)); self }
    /// Free-text filter by cloud tag key (supports multiple values).
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_contains = Some(join_csv(v)); self }
    /// Tags (not in).
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.tags_key_value_nin = Some(join_csv(v)); self }
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.surfaces_nin = Some(join_csv(v)); self }
    /// User and cloud tag keys exists.
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.all_tags_key_exists = Some(join_csv(v)); self }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value_nin = Some(join_csv(v)); self }
    /// The cloud tags key.
    pub fn cloud_tags_key<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key = Some(join_csv(v)); self }
    /// The Surface that each asset belongs to.
    pub fn surfaces<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.surfaces = Some(join_csv(v)); self }
    /// The sub-category that each resource belongs to (not in).
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.sub_category_nin = Some(join_csv(v)); self }
    /// The ID.
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.id_in = Some(join_csv(v)); self }
    /// The status alerts of the asset.
    pub fn infection_status<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.infection_status = Some(join_csv(v)); self }
    /// The active coverage for the asset.
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.active_coverage = Some(join_csv(v)); self }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.counts_for = Some(join_csv(v)); self }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_tags_key_value_contains = Some(join_csv(v)); self }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: AsRef<str> { self.cloud_provider_project_id_contains = Some(join_csv(v)); self }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/application-integration`
/// ("Assets using POST").
///
/// Spec definition:
/// `ApplicationIntegrationsViewInputSchema`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetsPostBody {
    /// Filter. Required. Free-form filter object
    /// (`PaginatedApplicationIntegrationFilter`); passed through as raw JSON.
    pub filter: serde_json::Value,
    /// Data. Optional/nullable. Free-form object (`EmptyStrict`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/application-integration/action`
/// ("Perform action").
///
/// Spec definition:
/// `ApplicationIntegrationActionPayloadSchema`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionBody {
    /// Action name. Required.
    ///
    /// Allowed values: `export_resource_details`,
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
    /// Create a new action payload with the required `action_name`.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self { action_name: action_name.into(), id_in: None, id_nin: None }
    }
    /// List of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.id_in = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from select_all (max 5000).
    pub fn id_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.id_nin = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/application-integration/available-actions/with-status`
/// ("Available actions").
///
/// Spec definition: `AffectedResourcesSchema`.
#[derive(Debug, Clone, Default, Serialize)]
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
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.id_in = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from select_all (max 5000).
    pub fn id_nin<I, S>(mut self, v: I) -> Self
    where I: IntoIterator<Item = S>, S: Into<String> {
        self.id_nin = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

impl InventoryApplicationIntegrationService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/application-integration` — Assets.
    ///
    /// Get assets.
    pub async fn get_assets(
        &self,
        query: &AssetsQuery,
    ) -> Result<Paginated<ApplicationIntegration>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/application-integration", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/application-integration` — Assets using
    /// POST.
    ///
    /// POST API to get Assets.
    pub async fn get_assets_post(
        &self,
        query: &AssetsPostQuery,
        body: &AssetsPostBody,
    ) -> Result<Paginated<ApplicationIntegration>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<AssetsPostBody, Paginated<ApplicationIntegration>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/application-integration",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/application-integration/action` — Perform
    /// action.
    ///
    /// Perform action on selected assets.
    pub async fn perform_action(
        &self,
        query: &AssetActionQuery,
        body: &PerformActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<PerformActionBody, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/application-integration/action",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/application-integration/available-actions/with-status`
    /// — Available actions.
    ///
    /// Get available actions.
    pub async fn available_actions(
        &self,
        query: &AssetActionQuery,
        body: &AvailableActionsBody,
    ) -> Result<Response<AvailableActionWithStatusResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<AvailableActionsBody, Response<AvailableActionWithStatusResponse>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/application-integration/available-actions/with-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/application-integration/export` — Export
    /// assets to CSV or JSON.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    ///
    /// `export_format`: Export format. Required (query). Allowed values: `csv`,
    /// `json`.
    pub async fn export(
        &self,
        export_format: impl Into<String>,
        query: &AssetsQuery,
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
            .get("/web/api/v2.1/xdr/assets/application-integration/export", q)
            .await?)
    }
}
