//! `Inventory Data Analysis Filters` tag — Inventory Data Analysis Resource Filters.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_data_analysis_filters::{
    AutoCompleteResponse, CountFiltersResponse, FreeTextFilterResponse,
};
use crate::pagination::Response;

/// Service for the `Inventory Data Analysis Filters` tag.
///
/// Inventory Data Analysis Resource Filters. Provides auto-complete
/// suggestions, filter counts, and free-text filter descriptors for the XDR
/// asset data-analysis surface.
pub struct InventoryDataAnalysisFiltersService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for
/// `GET /web/api/v2.1/xdr/assets/data-analysis/filters/autocomplete`.
///
/// Array params are serialized comma-joined, as the API expects. Every field is
/// optional except `text` and `key`, which are required by the endpoint and are
/// therefore passed as method arguments rather than living on this struct.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutocompleteQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(
        rename = "resourceType__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(
        rename = "assetCriticality__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in). Optional.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(
        rename = "infectionStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudProviderOrganizationUnit__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(
        rename = "cloudProviderAccountName__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(
        rename = "cloudResourceId__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(
        rename = "cloudProviderSubscriptionId__contains",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudProviderOrganization__contains",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "assetContactEmail__nin",
        skip_serializing_if = "Option::is_none"
    )]
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
    /// Limit number of returned items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    #[serde(
        rename = "tagsKeyValue__contains",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudProviderAccountId__nin",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudProviderAccountId__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(
        rename = "cloudProviderAccountName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset. Optional.
    /// Allowed values: `Active`, `Inactive`.
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
    #[serde(
        rename = "activeCoverage__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_coverage_nin: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(
        rename = "s1UpdatedAt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Optional.
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory (not in). Optional.
    #[serde(
        rename = "assetEnvironment__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_environment_nin: Option<String>,
    /// The missing coverage for the asset (not in). Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(
        rename = "missingCoverage__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub missing_coverage_nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(
        rename = "allTagsKeyValue__nin",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudTagsKey__contains",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudTagsKeyValue__nin",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudTagsKeyValue__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(
        rename = "cloudProviderProjectId__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_project_id_contains: Option<String>,
    /// Search term text. Required (set via the `autocomplete` method argument).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Search field key. Required (set via the `autocomplete` method argument).
    /// Allowed values: `resourceType__contains`,
    /// `cloudProviderOrganization__contains`,
    /// `cloudProviderOrganizationUnit__contains`,
    /// `cloudProviderProjectId__contains`,
    /// `cloudProviderSubscriptionId__contains`,
    /// `cloudProviderAccountName__contains`,
    /// `cloudProviderAccountId__contains`, `cloudTagsKey__contains`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

impl AutocompleteQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(vals));
        self
    }
    /// The Asset Type.
    pub fn resource_type_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(join_csv(vals));
        self
    }
    /// The criticality that each asset belongs to (not in).
    pub fn asset_criticality_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(join_csv(vals));
        self
    }
    /// The status alerts of the asset (not in).
    pub fn infection_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(join_csv(vals));
        self
    }
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(join_csv(vals));
        self
    }
    /// The missing coverage for the asset.
    pub fn missing_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(join_csv(vals));
        self
    }
    /// User and cloud tags.
    pub fn all_tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(join_csv(vals));
        self
    }
    /// The cloud provider organization unit.
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit_contains = Some(join_csv(vals));
        self
    }
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(join_csv(vals));
        self
    }
    /// The geographical area where cloud resources are hosted.
    pub fn region_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(join_csv(vals));
        self
    }
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(join_csv(vals));
        self
    }
    /// The cloud resource ID.
    pub fn cloud_resource_id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id_contains = Some(join_csv(vals));
        self
    }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id_contains = Some(join_csv(vals));
        self
    }
    /// The cloud provider account id.
    pub fn cloud_provider_account_id<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id = Some(join_csv(vals));
        self
    }
    /// The asset review (not in).
    pub fn device_review_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(join_csv(vals));
        self
    }
    /// The sub-category that each resource belongs to.
    pub fn sub_category<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(join_csv(vals));
        self
    }
    /// The cloud provider organization.
    pub fn cloud_provider_organization_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_contains = Some(join_csv(vals));
        self
    }
    /// Tag Keys exists.
    pub fn tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(join_csv(vals));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(join_csv(vals));
        self
    }
    /// The status of the asset (not in).
    pub fn asset_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(join_csv(vals));
        self
    }
    /// The asset review.
    pub fn device_review<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(join_csv(vals));
        self
    }
    /// Asset Contact Email (not in).
    pub fn asset_contact_email_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(join_csv(vals));
        self
    }
    /// User and cloud tag keys.
    pub fn all_tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(join_csv(vals));
        self
    }
    /// The region.
    pub fn region<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(vals));
        self
    }
    /// The severity of the alert.
    pub fn alert_severity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(join_csv(vals));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }
    /// The cloud tags key value.
    pub fn cloud_tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value = Some(join_csv(vals));
        self
    }
    /// Name.
    pub fn names<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join_csv(vals));
        self
    }
    /// Limit number of returned items.
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Free-text filter by tag key value (supports multiple values).
    pub fn tags_key_value_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(join_csv(vals));
        self
    }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_nin = Some(join_csv(vals));
        self
    }
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(join_csv(vals));
        self
    }
    /// The name.
    pub fn name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(vals));
        self
    }
    /// The criticality that each asset belongs to.
    pub fn asset_criticality<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(vals));
        self
    }
    /// The risk factors associated with the asset (not in).
    pub fn risk_factors_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(join_csv(vals));
        self
    }
    /// Tag Keys not exists.
    pub fn tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(join_csv(vals));
        self
    }
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(join_csv(vals));
        self
    }
    /// The canonical name for the resource type (not in).
    pub fn resource_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(join_csv(vals));
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self {
        self.csv_filter_id = Some(n);
        self
    }
    /// The ID.
    pub fn id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(vals));
        self
    }
    /// The cloud provider account ID.
    pub fn cloud_provider_account_id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(join_csv(vals));
        self
    }
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(join_csv(vals));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(join_csv(vals));
        self
    }
    /// The status of the asset.
    pub fn asset_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(join_csv(vals));
        self
    }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(join_csv(vals));
        self
    }
    /// Asset Contact Email.
    pub fn asset_contact_email<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(join_csv(vals));
        self
    }
    /// The risk factors associated with the asset.
    pub fn risk_factors<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(vals));
        self
    }
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(join_csv(vals));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory.
    pub fn asset_environment<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(join_csv(vals));
        self
    }
    /// The Last Seen date and time for the asset.
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    /// The canonical name for the resource type.
    pub fn resource_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(vals));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory (not in).
    pub fn asset_environment_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(join_csv(vals));
        self
    }
    /// The missing coverage for the asset (not in).
    pub fn missing_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(join_csv(vals));
        self
    }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(join_csv(vals));
        self
    }
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(join_csv(vals));
        self
    }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(join_csv(vals));
        self
    }
    /// Name (not in).
    pub fn names_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(join_csv(vals));
        self
    }
    /// Free-text filter by cloud tag key (supports multiple values).
    pub fn cloud_tags_key_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_contains = Some(join_csv(vals));
        self
    }
    /// Tags (not in).
    pub fn tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(join_csv(vals));
        self
    }
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(join_csv(vals));
        self
    }
    /// User and cloud tag keys exists.
    pub fn all_tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(join_csv(vals));
        self
    }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(join_csv(vals));
        self
    }
    /// The cloud tags key.
    pub fn cloud_tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key = Some(join_csv(vals));
        self
    }
    /// The Surface that each asset belongs to.
    pub fn surfaces<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(join_csv(vals));
        self
    }
    /// The sub-category that each resource belongs to (not in).
    pub fn sub_category_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(join_csv(vals));
        self
    }
    /// The ID.
    pub fn id_in<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(join_csv(vals));
        self
    }
    /// The status alerts of the asset.
    pub fn infection_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(join_csv(vals));
        self
    }
    /// The active coverage for the asset.
    pub fn active_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(join_csv(vals));
        self
    }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(join_csv(vals));
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_contains = Some(join_csv(vals));
        self
    }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(join_csv(vals));
        self
    }
}

/// Query params for
/// `GET /web/api/v2.1/xdr/assets/data-analysis/filters/count`.
///
/// Array params are serialized comma-joined, as the API expects. All fields are
/// optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CountQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(
        rename = "resourceType__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub resource_type_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Optional.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(
        rename = "assetCriticality__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_criticality_nin: Option<String>,
    /// The status alerts of the asset (not in). Optional.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(
        rename = "infectionStatus__nin",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudProviderOrganizationUnit__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(
        rename = "cloudProviderAccountName__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_account_name_nin: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(rename = "tagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_nin: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(
        rename = "cloudResourceId__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_resource_id_contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(
        rename = "cloudProviderSubscriptionId__contains",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudProviderOrganization__contains",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "assetContactEmail__nin",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "tagsKeyValue__contains",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudProviderAccountId__nin",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudProviderAccountId__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_account_id_contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(rename = "imageName__contains", skip_serializing_if = "Option::is_none")]
    pub image_name_contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(
        rename = "cloudProviderAccountName__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset. Optional.
    /// Allowed values: `Active`, `Inactive`.
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
    #[serde(
        rename = "activeCoverage__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub active_coverage_nin: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(
        rename = "s1UpdatedAt__between",
        skip_serializing_if = "Option::is_none"
    )]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Optional.
    /// Allowed values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory (not in). Optional.
    #[serde(
        rename = "assetEnvironment__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_environment_nin: Option<String>,
    /// The missing coverage for the asset (not in). Optional.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(
        rename = "missingCoverage__nin",
        skip_serializing_if = "Option::is_none"
    )]
    pub missing_coverage_nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(
        rename = "allTagsKeyValue__nin",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudTagsKey__contains",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudTagsKeyValue__nin",
        skip_serializing_if = "Option::is_none"
    )]
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
    #[serde(
        rename = "cloudTagsKeyValue__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(
        rename = "cloudProviderProjectId__contains",
        skip_serializing_if = "Option::is_none"
    )]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl CountQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_contains = Some(join_csv(vals));
        self
    }
    /// The Asset Type.
    pub fn resource_type_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_contains = Some(join_csv(vals));
        self
    }
    /// The criticality that each asset belongs to (not in).
    pub fn asset_criticality_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality_nin = Some(join_csv(vals));
        self
    }
    /// The status alerts of the asset (not in).
    pub fn infection_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status_nin = Some(join_csv(vals));
        self
    }
    /// Tags.
    pub fn tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value = Some(join_csv(vals));
        self
    }
    /// The missing coverage for the asset.
    pub fn missing_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(join_csv(vals));
        self
    }
    /// User and cloud tags.
    pub fn all_tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value = Some(join_csv(vals));
        self
    }
    /// The cloud provider organization unit.
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit_contains = Some(join_csv(vals));
        self
    }
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_nin = Some(join_csv(vals));
        self
    }
    /// The geographical area where cloud resources are hosted.
    pub fn region_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_contains = Some(join_csv(vals));
        self
    }
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nin = Some(join_csv(vals));
        self
    }
    /// The cloud resource ID.
    pub fn cloud_resource_id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id_contains = Some(join_csv(vals));
        self
    }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id_contains = Some(join_csv(vals));
        self
    }
    /// The cloud provider account id.
    pub fn cloud_provider_account_id<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id = Some(join_csv(vals));
        self
    }
    /// The asset review (not in).
    pub fn device_review_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review_nin = Some(join_csv(vals));
        self
    }
    /// The sub-category that each resource belongs to.
    pub fn sub_category<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(join_csv(vals));
        self
    }
    /// The cloud provider organization.
    pub fn cloud_provider_organization_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_contains = Some(join_csv(vals));
        self
    }
    /// Tag Keys exists.
    pub fn tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_exists = Some(join_csv(vals));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name = Some(join_csv(vals));
        self
    }
    /// The status of the asset (not in).
    pub fn asset_status_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status_nin = Some(join_csv(vals));
        self
    }
    /// The asset review.
    pub fn device_review<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review = Some(join_csv(vals));
        self
    }
    /// Asset Contact Email (not in).
    pub fn asset_contact_email_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email_nin = Some(join_csv(vals));
        self
    }
    /// User and cloud tag keys.
    pub fn all_tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key = Some(join_csv(vals));
        self
    }
    /// The region.
    pub fn region<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(vals));
        self
    }
    /// The severity of the alert.
    pub fn alert_severity<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.alert_severity = Some(join_csv(vals));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(vals));
        self
    }
    /// The cloud tags key value.
    pub fn cloud_tags_key_value<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value = Some(join_csv(vals));
        self
    }
    /// Name.
    pub fn names<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join_csv(vals));
        self
    }
    /// Free-text filter by tag key value (supports multiple values).
    pub fn tags_key_value_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_contains = Some(join_csv(vals));
        self
    }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_nin = Some(join_csv(vals));
        self
    }
    /// Tag Keys.
    pub fn tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key = Some(join_csv(vals));
        self
    }
    /// The name.
    pub fn name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(vals));
        self
    }
    /// The criticality that each asset belongs to.
    pub fn asset_criticality<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(vals));
        self
    }
    /// The risk factors associated with the asset (not in).
    pub fn risk_factors_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors_nin = Some(join_csv(vals));
        self
    }
    /// Tag Keys not exists.
    pub fn tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_nexists = Some(join_csv(vals));
        self
    }
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_nin = Some(join_csv(vals));
        self
    }
    /// The canonical name for the resource type (not in).
    pub fn resource_type_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type_nin = Some(join_csv(vals));
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self {
        self.csv_filter_id = Some(n);
        self
    }
    /// The ID.
    pub fn id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(vals));
        self
    }
    /// The cloud provider account ID.
    pub fn cloud_provider_account_id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id_contains = Some(join_csv(vals));
        self
    }
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name_contains = Some(join_csv(vals));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(vals));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name_contains = Some(join_csv(vals));
        self
    }
    /// The status of the asset.
    pub fn asset_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(join_csv(vals));
        self
    }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nexists = Some(join_csv(vals));
        self
    }
    /// Asset Contact Email.
    pub fn asset_contact_email<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_contact_email = Some(join_csv(vals));
        self
    }
    /// The risk factors associated with the asset.
    pub fn risk_factors<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(vals));
        self
    }
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage_nin = Some(join_csv(vals));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(vals));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory.
    pub fn asset_environment<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment = Some(join_csv(vals));
        self
    }
    /// The Last Seen date and time for the asset.
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
    /// The canonical name for the resource type.
    pub fn resource_type<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(vals));
        self
    }
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory (not in).
    pub fn asset_environment_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment_nin = Some(join_csv(vals));
        self
    }
    /// The missing coverage for the asset (not in).
    pub fn missing_coverage_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage_nin = Some(join_csv(vals));
        self
    }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value_nin = Some(join_csv(vals));
        self
    }
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region_nin = Some(join_csv(vals));
        self
    }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_nin = Some(join_csv(vals));
        self
    }
    /// Name (not in).
    pub fn names_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names_nin = Some(join_csv(vals));
        self
    }
    /// Free-text filter by cloud tag key (supports multiple values).
    pub fn cloud_tags_key_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_contains = Some(join_csv(vals));
        self
    }
    /// Tags (not in).
    pub fn tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value_nin = Some(join_csv(vals));
        self
    }
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces_nin = Some(join_csv(vals));
        self
    }
    /// User and cloud tag keys exists.
    pub fn all_tags_key_exists<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_exists = Some(join_csv(vals));
        self
    }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_nin = Some(join_csv(vals));
        self
    }
    /// The cloud tags key.
    pub fn cloud_tags_key<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key = Some(join_csv(vals));
        self
    }
    /// The Surface that each asset belongs to.
    pub fn surfaces<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(join_csv(vals));
        self
    }
    /// The sub-category that each resource belongs to (not in).
    pub fn sub_category_nin<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category_nin = Some(join_csv(vals));
        self
    }
    /// The ID.
    pub fn id_in<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(join_csv(vals));
        self
    }
    /// The status alerts of the asset.
    pub fn infection_status<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(join_csv(vals));
        self
    }
    /// The active coverage for the asset.
    pub fn active_coverage<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(join_csv(vals));
        self
    }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(join_csv(vals));
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value_contains = Some(join_csv(vals));
        self
    }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, vals: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id_contains = Some(join_csv(vals));
        self
    }
}

/// Joins an iterator of string-like values into a comma-separated string, as
/// the SentinelOne API expects array query params to be serialized.
fn join_csv<I, S>(vals: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    vals.into_iter()
        .map(|s| s.as_ref().to_owned())
        .collect::<Vec<_>>()
        .join(",")
}

impl InventoryDataAnalysisFiltersService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/data-analysis/filters/autocomplete` — Auto
    /// Complete.
    ///
    /// Use this command to get values for other fields. When you send this
    /// command with input text and a field name, it returns auto-complete
    /// suggestions for the field.
    ///
    /// `text` (search term) and `key` (search field key) are required by the
    /// API and passed as arguments; any further filtering lives on `query`.
    ///
    /// Allowed values for `key`: `resourceType__contains`,
    /// `cloudProviderOrganization__contains`,
    /// `cloudProviderOrganizationUnit__contains`,
    /// `cloudProviderProjectId__contains`,
    /// `cloudProviderSubscriptionId__contains`,
    /// `cloudProviderAccountName__contains`,
    /// `cloudProviderAccountId__contains`, `cloudTagsKey__contains`.
    pub async fn autocomplete(
        &self,
        text: impl Into<String>,
        key: impl Into<String>,
        query: &AutocompleteQuery,
    ) -> Result<Response<AutoCompleteResponse>, Error> {
        // Start from the caller's filters, then set the required `text`/`key`.
        let mut q = query.clone();
        q.text = Some(text.into());
        q.key = Some(key.into());
        let qs = serde_urlencoded::to_string(&q).unwrap_or_default();
        let qref = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/assets/data-analysis/filters/autocomplete",
                qref,
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/data-analysis/filters/count` — Filter
    /// counts.
    ///
    /// Get filter counts.
    pub async fn count(
        &self,
        query: &CountQuery,
    ) -> Result<Response<Vec<CountFiltersResponse>>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let qref = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/data-analysis/filters/count", qref)
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/data-analysis/filters/free-text` — Free
    /// text filters.
    ///
    /// Get free text filters.
    pub async fn free_text(&self) -> Result<Response<Vec<FreeTextFilterResponse>>, Error> {
        Ok(self
            .client
            .http()
            .get(
                "/web/api/v2.1/xdr/assets/data-analysis/filters/free-text",
                None,
            )
            .await?)
    }
}
