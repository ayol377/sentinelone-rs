//! `Inventory AI ML` tag — AI/ML asset inventory resources.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_ai_ml::{Asset, AvailableActionWithStatus};
use crate::pagination::{Paginated, Response};

/// `Inventory AI ML` tag.
///
/// Inventory AI ML Resources — list, filter, export, and act on the AI/ML
/// asset inventory exposed under `/web/api/v2.1/xdr/assets/ai-ml`.
pub struct InventoryAiMlService<'a> {
    pub(crate) client: &'a ManagementClient,
}

// Internal helper: join an iterator of string-ish values into a comma list.
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

/// Query params for `GET /web/api/v2.1/xdr/assets/ai-ml`.
///
/// Every field is optional. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListQuery {
    /// Free-text filter by tag key (supports multiple values). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__contains")]
    pub tags_key__contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Array param.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetCriticality__nin")]
    pub asset_criticality__nin: Option<String>,
    /// The missing coverage for the asset. Array param. Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountName__nin")]
    pub cloud_provider_account_name__nin: Option<String>,
    /// Tag Keys (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__nin")]
    pub tags_key__nin: Option<String>,
    /// The cloud resource ID. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudResourceId__contains")]
    pub cloud_resource_id__contains: Option<String>,
    /// The cloud provider subscription ID. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderSubscriptionId__contains")]
    pub cloud_provider_subscription_id__contains: Option<String>,
    /// Tag Keys exists. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__exists")]
    pub tags_key__exists: Option<String>,
    /// The region. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKeyValue__contains")]
    pub tags_key_value__contains: Option<String>,
    /// The cloud tags key (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKey__nin")]
    pub cloud_tags_key__nin: Option<String>,
    /// Tag Keys. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). Array param.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "riskFactors__nin")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__nexists")]
    pub tags_key__nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountId__contains")]
    pub cloud_provider_account_id__contains: Option<String>,
    /// Free-text filter by the image name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageName__contains")]
    pub image_name__contains: Option<String>,
    /// List of Group IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The active coverage for the asset (not in). Array param. Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeCoverage__nin")]
    pub active_coverage__nin: Option<String>,
    /// User and cloud tags (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKeyValue__nin")]
    pub all_tags_key_value__nin: Option<String>,
    /// The region (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region__nin")]
    pub region__nin: Option<String>,
    /// User and cloud tag keys (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__nin")]
    pub all_tags_key__nin: Option<String>,
    /// The cloud tags key value (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKeyValue__nin")]
    pub cloud_tags_key_value__nin: Option<String>,
    /// The Surface that each asset belongs to. Array param. Allowed values:
    /// `Cloud`, `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The missing coverage for the asset (not in). Array param. Allowed values:
    /// `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "missingCoverage__nin")]
    pub missing_coverage__nin: Option<String>,
    /// The status of the asset (not in). Array param. Allowed values:
    /// `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetStatus__nin")]
    pub asset_status__nin: Option<String>,
    /// The column to sort the results by. Allowed values: `s1GroupName`,
    /// `s1OnboardedAccountName`, `cloudProviderProjectId`, `category`,
    /// `s1UpdatedAt`, `s1GroupId`, `cloudProviderAccountId`,
    /// `s1OnboardedScopeLevel`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The geographical area where cloud resources are hosted. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region__contains")]
    pub region__contains: Option<String>,
    /// The asset review (not in). Array param. Allowed values: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deviceReview__nin")]
    pub device_review__nin: Option<String>,
    /// The cloud provider account name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetContactEmail__nin")]
    pub asset_contact_email__nin: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType__nin")]
    pub resource_type__nin: Option<String>,
    /// Asset Contact Email. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Array param. Allowed values:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in — AWS | Azure | GCP |
    /// Active Directory. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in). Array param. Allowed
    /// values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "surfaces__nin")]
    pub surfaces__nin: Option<String>,
    /// The ID. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__in")]
    pub id__in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    /// Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKeyValue__contains")]
    pub cloud_tags_key_value__contains: Option<String>,
    /// The Asset Type. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType__contains")]
    pub resource_type__contains: Option<String>,
    /// Tags. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The cloud provider organization unit. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderOrganizationUnit__contains")]
    pub cloud_provider_organization_unit__contains: Option<String>,
    /// The cloud provider organization. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderOrganization__contains")]
    pub cloud_provider_organization__contains: Option<String>,
    /// If true, only the total number of items is returned, without any of the
    /// actual objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name__contains")]
    pub name__contains: Option<String>,
    /// The criticality that each asset belongs to. Array param. Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountId__nin")]
    pub cloud_provider_account_id__nin: Option<String>,
    /// If true, the total number of items is not calculated, which speeds up
    /// execution time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__nexists")]
    pub all_tags_key__nexists: Option<String>,
    /// List of Site IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "s1UpdatedAt__between")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type. Array param. Allowed values:
    /// `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in — AWS | Azure | GCP |
    /// Active Directory (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetEnvironment__nin")]
    pub asset_environment__nin: Option<String>,
    /// Name (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "names__nin")]
    pub names__nin: Option<String>,
    /// The cloud tags key. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in). Array param.
    /// Allowed values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subCategory__nin")]
    pub sub_category__nin: Option<String>,
    /// The status alerts of the asset. Array param. Allowed values: `Infected`,
    /// `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset. Array param. Allowed values: `CWS`,
    /// `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which a filter count would be returned. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The cloud provider account id. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to. Array param. Allowed
    /// values: `All`, `Access Key and Secret`, `Access Management`, `Account`,
    /// `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review. Array param. Allowed values: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__contains")]
    pub id__contains: Option<String>,
    /// The cloud provider account name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountName__contains")]
    pub cloud_provider_account_name__contains: Option<String>,
    /// The status of the asset. Array param. Allowed values: `Active`,
    /// `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKey__contains")]
    pub cloud_tags_key__contains: Option<String>,
    /// Tags (not in). Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKeyValue__nin")]
    pub tags_key_value__nin: Option<String>,
    /// User and cloud tag keys exists. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__exists")]
    pub all_tags_key__exists: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The status alerts of the asset (not in). Array param. Allowed values:
    /// `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "infectionStatus__nin")]
    pub infection_status__nin: Option<String>,
    /// The cloud provider project ID. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderProjectId__contains")]
    pub cloud_provider_project_id__contains: Option<String>,
}

impl ListQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to (not in).
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality__nin = Some(join_csv(v));
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
    /// The cloud provider account name (not in).
    pub fn cloud_provider_account_name_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name__nin = Some(join_csv(v));
        self
    }
    /// Tag Keys (not in).
    pub fn tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nin = Some(join_csv(v));
        self
    }
    /// The cloud resource ID.
    pub fn cloud_resource_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id__contains = Some(join_csv(v));
        self
    }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id__contains = Some(join_csv(v));
        self
    }
    /// Tag Keys exists.
    pub fn tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__exists = Some(join_csv(v));
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
    /// Free-text filter by tag key value (supports multiple values).
    pub fn tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key__nin = Some(join_csv(v));
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
        self.risk_factors__nin = Some(join_csv(v));
        self
    }
    /// Tag Keys not exists.
    pub fn tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nexists = Some(join_csv(v));
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// The cloud provider account ID.
    pub fn cloud_provider_account_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id__contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the image name.
    pub fn image_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.image_name__contains = Some(join_csv(v));
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
    /// The active coverage for the asset (not in).
    pub fn active_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage__nin = Some(join_csv(v));
        self
    }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value__nin = Some(join_csv(v));
        self
    }
    /// The region (not in).
    pub fn region_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region__nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nin = Some(join_csv(v));
        self
    }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value__nin = Some(join_csv(v));
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
    /// The missing coverage for the asset (not in).
    pub fn missing_coverage_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage__nin = Some(join_csv(v));
        self
    }
    /// The status of the asset (not in).
    pub fn asset_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status__nin = Some(join_csv(v));
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// The geographical area where cloud resources are hosted.
    pub fn region_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region__contains = Some(join_csv(v));
        self
    }
    /// The asset review (not in).
    pub fn device_review_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.device_review__nin = Some(join_csv(v));
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
        self.asset_contact_email__nin = Some(join_csv(v));
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
    /// The canonical name for the resource type (not in).
    pub fn resource_type_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__nin = Some(join_csv(v));
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
    /// The risk factors associated with the asset.
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
    /// The Surface that each asset belongs to (not in).
    pub fn surfaces_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces__nin = Some(join_csv(v));
        self
    }
    /// The ID.
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__in = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value__contains = Some(join_csv(v));
        self
    }
    /// The Asset Type.
    pub fn resource_type_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type__contains = Some(join_csv(v));
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
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
        self
    }
    /// The cloud provider organization unit.
    pub fn cloud_provider_organization_unit_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit__contains = Some(join_csv(v));
        self
    }
    /// The cloud provider organization.
    pub fn cloud_provider_organization_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization__contains = Some(join_csv(v));
        self
    }
    /// If true, only the total number of items is returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
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
        self.name__contains = Some(join_csv(v));
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
    /// The cloud provider account id (not in).
    pub fn cloud_provider_account_id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id__nin = Some(join_csv(v));
        self
    }
    /// If true, the total number of items is not calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key_nexists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nexists = Some(join_csv(v));
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
        self.s1_updated_at__between = Some(v.into());
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
    /// The environment that the asset exists in (not in).
    pub fn asset_environment_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment__nin = Some(join_csv(v));
        self
    }
    /// Name (not in).
    pub fn names_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names__nin = Some(join_csv(v));
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
    /// The sub-category that each resource belongs to (not in).
    pub fn sub_category_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category__nin = Some(join_csv(v));
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
    /// The active coverage for the asset.
    pub fn active_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(join_csv(v));
        self
    }
    /// The columns for which a filter count would be returned.
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
    /// The cloud provider account id.
    pub fn cloud_provider_account_id<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id = Some(join_csv(v));
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
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// The ID.
    pub fn id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__contains = Some(join_csv(v));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name__contains = Some(join_csv(v));
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
    /// Free-text filter by cloud tag key (supports multiple values).
    pub fn cloud_tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key__contains = Some(join_csv(v));
        self
    }
    /// Tags (not in).
    pub fn tags_key_value_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys exists.
    pub fn all_tags_key_exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__exists = Some(join_csv(v));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
    /// The status alerts of the asset (not in).
    pub fn infection_status_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status__nin = Some(join_csv(v));
        self
    }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id__contains = Some(join_csv(v));
        self
    }
}

/// Serialized form of the export-only param (`exportFormat`).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportExtra {
    export_format: String,
}

/// Query params for `GET /web/api/v2.1/xdr/assets/ai-ml/export`.
///
/// Identical filter set to [`ListQuery`], plus the required `exportFormat`.
/// Array params are serialized comma-joined. The only required param is
/// `export_format`; it is set via [`ExportQuery::new`].
///
/// (`serde_urlencoded` does not support `#[serde(flatten)]`, so the
/// `exportFormat` flag and the filter set are serialized separately and
/// concatenated by [`ExportQuery::to_query_string`].)
#[derive(Debug, Default)]
pub struct ExportQuery {
    /// Export format (required). Allowed values: `csv`, `json`.
    pub export_format: String,
    /// All optional filter params, identical to the list endpoint. See
    /// [`ListQuery`] for the full documented field set.
    pub filter: ListQuery,
}

impl ExportQuery {
    /// Build an export query. `export_format` is required; allowed values:
    /// `csv`, `json`.
    pub fn new(export_format: impl Into<String>) -> Self {
        Self {
            export_format: export_format.into(),
            filter: ListQuery::default(),
        }
    }
    /// Set the required export format. Allowed values: `csv`, `json`.
    pub fn export_format(mut self, v: impl Into<String>) -> Self {
        self.export_format = v.into();
        self
    }
    /// Set the optional filter params (mirrors [`ListQuery`] builders).
    pub fn filter(mut self, filter: ListQuery) -> Self {
        self.filter = filter;
        self
    }

    /// Build the full querystring (`exportFormat` + the filter set).
    fn to_query_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        let extra = ExportExtra {
            export_format: self.export_format.clone(),
        };
        let extra_qs = serde_urlencoded::to_string(&extra).unwrap_or_default();
        if !extra_qs.is_empty() {
            parts.push(extra_qs);
        }
        let filter_qs = serde_urlencoded::to_string(&self.filter).unwrap_or_default();
        if !filter_qs.is_empty() {
            parts.push(filter_qs);
        }
        parts.join("&")
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/ai-ml`.
///
/// Array params are serialized comma-joined; all fields are optional.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPostQuery {
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

impl ListPostQuery {
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

/// Request body for `POST /web/api/v2.1/xdr/assets/ai-ml`
/// (`AIMLViewInputSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListPostBody {
    /// Filter (required). The `PaginatedAIMLFilter` object is large and
    /// free-form; modelled as arbitrary JSON for forward-compat.
    pub filter: serde_json::Value,
    /// Data. Optional / nullable (`EmptyStrict`, an open object); modelled as
    /// arbitrary JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl ListPostBody {
    /// Build the body with the required `filter` object.
    pub fn new(filter: serde_json::Value) -> Self {
        Self {
            filter,
            data: None,
        }
    }
    /// Set the optional `data` object.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/ai-ml/action`.
///
/// Mirrors [`ListQuery`]'s filter set (without paging/sort-only params is not
/// applicable — the full filter set is accepted). All fields optional; array
/// params serialized comma-joined.
pub type ActionQuery = ListQuery;

/// Request body for `POST /web/api/v2.1/xdr/assets/ai-ml/action`
/// (`AIMLActionPayloadSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionBody {
    /// Action name (required). Enum (kept as `String` for forward-compat).
    /// Allowed values: `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    pub action_name: String,
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id__in: Option<Vec<String>>,
    /// List of inventory ids to exclude from `select_all` (max 5000). Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id__nin: Option<Vec<String>>,
}

impl ActionBody {
    /// Build the action body with the required `action_name`.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self {
            action_name: action_name.into(),
            id__in: None,
            id__nin: None,
        }
    }
    /// List of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__in = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from `select_all` (max 5000).
    pub fn id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__nin = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

/// Query params for
/// `POST /web/api/v2.1/xdr/assets/ai-ml/available-actions/with-status`.
///
/// Mirrors [`ListQuery`]'s filter set. All fields optional; array params
/// serialized comma-joined.
pub type AvailableActionsQuery = ListQuery;

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/ai-ml/available-actions/with-status`
/// (`AffectedResourcesSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableActionsBody {
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id__in: Option<Vec<String>>,
    /// List of inventory ids to exclude from `select_all` (max 5000). Optional.
    #[serde(rename = "id__nin", skip_serializing_if = "Option::is_none")]
    pub id__nin: Option<Vec<String>>,
}

impl AvailableActionsBody {
    /// List of selected inventory ids (max 5000).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__in = Some(v.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from `select_all` (max 5000).
    pub fn id_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__nin = Some(v.into_iter().map(Into::into).collect());
        self
    }
}

impl InventoryAiMlService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/ai-ml` — Assets.
    ///
    /// Get assets. Returns the AI/ML asset inventory, filtered by the supplied
    /// query params. Each asset is a free-form object (`AIMLResponse`).
    pub async fn list(&self, query: &ListQuery) -> Result<Paginated<Asset>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/ai-ml", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/ai-ml` — Assets using POST.
    ///
    /// POST API to get Assets. Accepts a `filter` body (`AIMLViewInputSchema`)
    /// alongside the `accountIds` / `siteIds` / `groupIds` query scoping
    /// params, and returns the matching assets.
    pub async fn list_post(
        &self,
        query: &ListPostQuery,
        body: &ListPostBody,
    ) -> Result<Paginated<Asset>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<ListPostBody, Paginated<Asset>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/ai-ml",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/ai-ml/action` — Perform action.
    ///
    /// Perform action on selected assets. The `actionName` body field selects
    /// the operation; `id__in` / `id__nin` (with the query filter set) scope
    /// the affected assets.
    pub async fn action(
        &self,
        query: &ActionQuery,
        body: &ActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<ActionBody, Response<serde_json::Value>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/ai-ml/action",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/ai-ml/available-actions/with-status`
    /// — Available actions.
    ///
    /// Get available actions. Returns the actions available for the selected
    /// assets (scoped by the query filter set and the `id__in` / `id__nin`
    /// body fields), each with its enabled/disabled status.
    pub async fn available_actions_with_status(
        &self,
        query: &AvailableActionsQuery,
        body: &AvailableActionsBody,
    ) -> Result<Response<AvailableActionWithStatus>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json::<AvailableActionsBody, Response<AvailableActionWithStatus>>(
                Method::POST,
                "/web/api/v2.1/xdr/assets/ai-ml/available-actions/with-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/ai-ml/export` — Export assets to CSV or JSON.
    ///
    /// Returns the results for the given inventory filter in a CSV or JSON
    /// format. The `exportFormat` query param is required (`csv` or `json`).
    pub async fn export(
        &self,
        query: &ExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = query.to_query_string();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/ai-ml/export", q)
            .await?)
    }
}
