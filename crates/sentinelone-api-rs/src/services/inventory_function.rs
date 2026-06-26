//! Service for the `Inventory Function` tag — the XDR asset inventory
//! "function" view, its POST query variant, the bulk-action endpoint, the
//! available-actions lookup, and CSV/JSON export.

use serde::Serialize;
use sentinelone_http::Method;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_function::*;
use crate::pagination::{Paginated, Response};

/// `Inventory Function` tag — Inventory Function Resources.
///
/// Endpoints for listing XDR inventory assets, performing bulk actions,
/// discovering available actions, and exporting results.
pub struct InventoryFunctionService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/xdr/assets/function` — Assets.
///
/// Every field is optional. Array params are comma-joined into a single
/// querystring value via their builder methods.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionListQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__contains")]
    pub tags_key__contains: Option<String>,
    /// The criticality that each asset belongs to (not in).
    /// Enum values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetCriticality__nin")]
    pub asset_criticality__nin: Option<String>,
    /// The missing coverage for the asset.
    /// Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountName__nin")]
    pub cloud_provider_account_name__nin: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__nin")]
    pub tags_key__nin: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudResourceId__contains")]
    pub cloud_resource_id__contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderSubscriptionId__contains")]
    pub cloud_provider_subscription_id__contains: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__exists")]
    pub tags_key__exists: Option<String>,
    /// The region. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKeyValue__contains")]
    pub tags_key_value__contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKey__nin")]
    pub cloud_tags_key__nin: Option<String>,
    /// Tag Keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in).
    /// Enum values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "riskFactors__nin")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__nexists")]
    pub tags_key__nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountId__contains")]
    pub cloud_provider_account_id__contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageName__contains")]
    pub image_name__contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The active coverage for the asset (not in).
    /// Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeCoverage__nin")]
    pub active_coverage__nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKeyValue__nin")]
    pub all_tags_key_value__nin: Option<String>,
    /// The region (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region__nin")]
    pub region__nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__nin")]
    pub all_tags_key__nin: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKeyValue__nin")]
    pub cloud_tags_key_value__nin: Option<String>,
    /// The Surface that each asset belongs to.
    /// Enum values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The missing coverage for the asset (not in).
    /// Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "missingCoverage__nin")]
    pub missing_coverage__nin: Option<String>,
    /// The status of the asset (not in).
    /// Enum values: `Active`, `Inactive`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetStatus__nin")]
    pub asset_status__nin: Option<String>,
    /// The column to sort the results by.
    /// Enum values: `s1GroupName`, `s1OnboardedAccountName`,
    /// `cloudProviderProjectId`, `category`, `s1UpdatedAt`, `s1GroupId`,
    /// `cloudProviderAccountId`, `s1OnboardedScopeLevel`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region__contains")]
    pub region__contains: Option<String>,
    /// The asset review (not in).
    /// Enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deviceReview__nin")]
    pub device_review__nin: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetContactEmail__nin")]
    pub asset_contact_email__nin: Option<String>,
    /// The severity of the alert. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in).
    /// Enum values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType__nin")]
    pub resource_type__nin: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset.
    /// Enum values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in).
    /// Enum values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "surfaces__nin")]
    pub surfaces__nin: Option<String>,
    /// The ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__in")]
    pub id__in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKeyValue__contains")]
    pub cloud_tags_key_value__contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType__contains")]
    pub resource_type__contains: Option<String>,
    /// Tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction. Enum values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderOrganizationUnit__contains")]
    pub cloud_provider_organization_unit__contains: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderOrganization__contains")]
    pub cloud_provider_organization__contains: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name__contains")]
    pub name__contains: Option<String>,
    /// The criticality that each asset belongs to.
    /// Enum values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountId__nin")]
    pub cloud_provider_account_id__nin: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__nexists")]
    pub all_tags_key__nexists: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "s1UpdatedAt__between")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type.
    /// Enum values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetEnvironment__nin")]
    pub asset_environment__nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "names__nin")]
    pub names__nin: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in).
    /// Enum values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subCategory__nin")]
    pub sub_category__nin: Option<String>,
    /// The status alerts of the asset.
    /// Enum values: `Infected`, `Healthy`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset.
    /// Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. Optional.
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
    /// The sub-category that each resource belongs to.
    /// Enum values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review.
    /// Enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__contains")]
    pub id__contains: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountName__contains")]
    pub cloud_provider_account_name__contains: Option<String>,
    /// The status of the asset.
    /// Enum values: `Active`, `Inactive`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKey__contains")]
    pub cloud_tags_key__contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKeyValue__nin")]
    pub tags_key_value__nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__exists")]
    pub all_tags_key__exists: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The status alerts of the asset (not in).
    /// Enum values: `Infected`, `Healthy`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "infectionStatus__nin")]
    pub infection_status__nin: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderProjectId__contains")]
    pub cloud_provider_project_id__contains: Option<String>,
}

impl FunctionListQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.tags_key__contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to (not in): `critical`,
    /// `high`, `medium`, `low`, `--`.
    pub fn asset_criticality_nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.asset_criticality__nin = Some(join_csv(v));
        self
    }
    /// The missing coverage for the asset.
    pub fn missing_coverage<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.missing_coverage = Some(join_csv(v));
        self
    }
    /// User and cloud tags.
    pub fn all_tags_key_value<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.all_tags_key_value = Some(join_csv(v));
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// The ID (in).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__in = Some(join_csv(v));
        self
    }
    /// Name.
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.names = Some(join_csv(v));
        self
    }
    /// The name (contains).
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.name__contains = Some(join_csv(v));
        self
    }
    /// The column to sort the results by: `s1GroupName`,
    /// `s1OnboardedAccountName`, `cloudProviderProjectId`, `category`,
    /// `s1UpdatedAt`, `s1GroupId`, `cloudProviderAccountId`,
    /// `s1OnboardedScopeLevel`.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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
    /// If true, only total number of items will be returned.
    pub fn count_only(mut self, v: bool) -> Self {
        self.count_only = Some(v);
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self {
        self.csv_filter_id = Some(n);
        self
    }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.counts_for = Some(join_csv(v));
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/function` — Assets using POST.
///
/// Every field is optional. Array params are comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionPostQuery {
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

impl FunctionPostQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/function` — Assets using POST.
///
/// Source: `#/definitions/v2_1.inventory.function.schemas_FunctionsViewInputSchema`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionPostBody {
    /// Data. Optional / nullable; freeform object (`#/definitions/EmptyStrict`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Filter — required. Paginated function filter (freeform object, see the
    /// `PaginatedFunctionFilter` definition for the full set of supported
    /// filter keys, which mirror the `GET` querystring filters).
    pub filter: serde_json::Value,
}

impl FunctionPostBody {
    /// Construct a body with the required `filter` object.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { data: None, filter }
    }
    /// Set the optional `data` object.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/function/action` — Perform action,
/// and for `POST .../available-actions/with-status` — Available actions.
///
/// Every field is optional. Array params are comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionActionQuery {
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__contains")]
    pub tags_key__contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType__contains")]
    pub resource_type__contains: Option<String>,
    /// The criticality that each asset belongs to (not in).
    /// Enum values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetCriticality__nin")]
    pub asset_criticality__nin: Option<String>,
    /// The status alerts of the asset (not in).
    /// Enum values: `Infected`, `Healthy`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "infectionStatus__nin")]
    pub infection_status__nin: Option<String>,
    /// Tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The missing coverage for the asset.
    /// Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderOrganizationUnit__contains")]
    pub cloud_provider_organization_unit__contains: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountName__nin")]
    pub cloud_provider_account_name__nin: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region__contains")]
    pub region__contains: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__nin")]
    pub tags_key__nin: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudResourceId__contains")]
    pub cloud_resource_id__contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderSubscriptionId__contains")]
    pub cloud_provider_subscription_id__contains: Option<String>,
    /// The cloud provider account id. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The asset review (not in).
    /// Enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deviceReview__nin")]
    pub device_review__nin: Option<String>,
    /// The sub-category that each resource belongs to.
    /// Enum values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderOrganization__contains")]
    pub cloud_provider_organization__contains: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__exists")]
    pub tags_key__exists: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// The status of the asset (not in).
    /// Enum values: `Active`, `Inactive`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetStatus__nin")]
    pub asset_status__nin: Option<String>,
    /// The asset review.
    /// Enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetContactEmail__nin")]
    pub asset_contact_email__nin: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKeyValue__contains")]
    pub tags_key_value__contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKey__nin")]
    pub cloud_tags_key__nin: Option<String>,
    /// Tag Keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name__contains")]
    pub name__contains: Option<String>,
    /// The criticality that each asset belongs to.
    /// Enum values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The risk factors associated with the asset (not in).
    /// Enum values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "riskFactors__nin")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__nexists")]
    pub tags_key__nexists: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountId__nin")]
    pub cloud_provider_account_id__nin: Option<String>,
    /// The canonical name for the resource type (not in).
    /// Enum values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType__nin")]
    pub resource_type__nin: Option<String>,
    /// The ID of the CSV file to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__contains")]
    pub id__contains: Option<String>,
    /// The cloud provider account ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountId__contains")]
    pub cloud_provider_account_id__contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageName__contains")]
    pub image_name__contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountName__contains")]
    pub cloud_provider_account_name__contains: Option<String>,
    /// The status of the asset.
    /// Enum values: `Active`, `Inactive`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__nexists")]
    pub all_tags_key__nexists: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset.
    /// Enum values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The active coverage for the asset (not in).
    /// Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeCoverage__nin")]
    pub active_coverage__nin: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "s1UpdatedAt__between")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type.
    /// Enum values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetEnvironment__nin")]
    pub asset_environment__nin: Option<String>,
    /// The missing coverage for the asset (not in).
    /// Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "missingCoverage__nin")]
    pub missing_coverage__nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKeyValue__nin")]
    pub all_tags_key_value__nin: Option<String>,
    /// The region (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region__nin")]
    pub region__nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__nin")]
    pub all_tags_key__nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "names__nin")]
    pub names__nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKey__contains")]
    pub cloud_tags_key__contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKeyValue__nin")]
    pub tags_key_value__nin: Option<String>,
    /// The Surface that each asset belongs to (not in).
    /// Enum values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "surfaces__nin")]
    pub surfaces__nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__exists")]
    pub all_tags_key__exists: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKeyValue__nin")]
    pub cloud_tags_key_value__nin: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The Surface that each asset belongs to.
    /// Enum values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The sub-category that each resource belongs to (not in).
    /// Enum values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subCategory__nin")]
    pub sub_category__nin: Option<String>,
    /// The ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__in")]
    pub id__in: Option<String>,
    /// The status alerts of the asset.
    /// Enum values: `Infected`, `Healthy`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset.
    /// Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKeyValue__contains")]
    pub cloud_tags_key_value__contains: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderProjectId__contains")]
    pub cloud_provider_project_id__contains: Option<String>,
}

impl FunctionActionQuery {
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// The ID (in).
    pub fn id_in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__in = Some(join_csv(v));
        self
    }
    /// Name.
    pub fn names<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.names = Some(join_csv(v));
        self
    }
    /// The name (contains).
    pub fn name_contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.name__contains = Some(join_csv(v));
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self {
        self.csv_filter_id = Some(n);
        self
    }
    /// The columns for which filter count would be returned for.
    pub fn counts_for<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.counts_for = Some(join_csv(v));
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/function/action` — Perform action.
///
/// Source: `#/definitions/v2_1.inventory.function.schemas_FunctionActionPayloadSchema`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionActionBody {
    /// Action name — required.
    /// Enum values: `export_resource_details`, `mark_asset_criticality_high`,
    /// `mark_asset_criticality_low`, `clear_asset_criticality`,
    /// `mark_asset_criticality_medium`, `mark_asset_criticality_critical`,
    /// `update_asset_contact`, `clear_asset_contact`, `apply_review`,
    /// `add_note`, `manage_tags`, `add_tags`, `remove_tags`, `replace_tags`,
    /// `clear_tags`.
    pub action_name: String,
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__in")]
    pub id__in: Option<Vec<String>>,
    /// List of inventory ids to exclude from `select_all` (max 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__nin")]
    pub id__nin: Option<Vec<String>>,
}

impl FunctionActionBody {
    /// Construct a body with the required `actionName`.
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

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/function/available-actions/with-status` —
/// Available actions.
///
/// Source: `#/definitions/v2_1.inventory.schemas_AffectedResourcesSchema`.
#[derive(Debug, Default, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResourcesBody {
    /// List of selected inventory ids (max 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__in")]
    pub id__in: Option<Vec<String>>,
    /// List of inventory ids to exclude from `select_all` (max 5000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__nin")]
    pub id__nin: Option<Vec<String>>,
}

impl AffectedResourcesBody {
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

/// Query params for `GET /web/api/v2.1/xdr/assets/function/export` —
/// Export assets to CSV or JSON.
///
/// The `export_format` field is required by the API; all other fields are
/// optional and mirror [`FunctionListQuery`].
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionExportQuery {
    /// Export format — required. Enum values: `csv`, `json`.
    pub export_format: String,
    /// Free-text filter by tag key (supports multiple values). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__contains")]
    pub tags_key__contains: Option<String>,
    /// The criticality that each asset belongs to (not in).
    /// Enum values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetCriticality__nin")]
    pub asset_criticality__nin: Option<String>,
    /// The missing coverage for the asset. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountName__nin")]
    pub cloud_provider_account_name__nin: Option<String>,
    /// Tag Keys (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__nin")]
    pub tags_key__nin: Option<String>,
    /// The cloud resource ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudResourceId__contains")]
    pub cloud_resource_id__contains: Option<String>,
    /// The cloud provider subscription ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderSubscriptionId__contains")]
    pub cloud_provider_subscription_id__contains: Option<String>,
    /// Tag Keys exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__exists")]
    pub tags_key__exists: Option<String>,
    /// The region. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKeyValue__contains")]
    pub tags_key_value__contains: Option<String>,
    /// The cloud tags key (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKey__nin")]
    pub cloud_tags_key__nin: Option<String>,
    /// Tag Keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in).
    /// Enum values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "riskFactors__nin")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKey__nexists")]
    pub tags_key__nexists: Option<String>,
    /// Skip first number of items (0-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountId__contains")]
    pub cloud_provider_account_id__contains: Option<String>,
    /// Free-text filter by the image name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imageName__contains")]
    pub image_name__contains: Option<String>,
    /// List of Group IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The active coverage for the asset (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "activeCoverage__nin")]
    pub active_coverage__nin: Option<String>,
    /// User and cloud tags (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKeyValue__nin")]
    pub all_tags_key_value__nin: Option<String>,
    /// The region (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region__nin")]
    pub region__nin: Option<String>,
    /// User and cloud tag keys (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__nin")]
    pub all_tags_key__nin: Option<String>,
    /// The cloud tags key value (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKeyValue__nin")]
    pub cloud_tags_key_value__nin: Option<String>,
    /// The Surface that each asset belongs to.
    /// Enum values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The missing coverage for the asset (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "missingCoverage__nin")]
    pub missing_coverage__nin: Option<String>,
    /// The status of the asset (not in).
    /// Enum values: `Active`, `Inactive`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetStatus__nin")]
    pub asset_status__nin: Option<String>,
    /// The column to sort the results by.
    /// Enum values: `s1GroupName`, `s1OnboardedAccountName`,
    /// `cloudProviderProjectId`, `category`, `s1UpdatedAt`, `s1GroupId`,
    /// `cloudProviderAccountId`, `s1OnboardedScopeLevel`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The geographical area where cloud resources are hosted. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region__contains")]
    pub region__contains: Option<String>,
    /// The asset review (not in).
    /// Enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deviceReview__nin")]
    pub device_review__nin: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetContactEmail__nin")]
    pub asset_contact_email__nin: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType__nin")]
    pub resource_type__nin: Option<String>,
    /// Asset Contact Email. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset.
    /// Enum values: `Unresolved Alerts`, `High Value`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "surfaces__nin")]
    pub surfaces__nin: Option<String>,
    /// The ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__in")]
    pub id__in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    /// Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKeyValue__contains")]
    pub cloud_tags_key_value__contains: Option<String>,
    /// The Asset Type. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "resourceType__contains")]
    pub resource_type__contains: Option<String>,
    /// Tags. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction. Enum values: `asc`, `desc`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The cloud provider organization unit. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderOrganizationUnit__contains")]
    pub cloud_provider_organization_unit__contains: Option<String>,
    /// The cloud provider organization. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderOrganization__contains")]
    pub cloud_provider_organization__contains: Option<String>,
    /// If true, only total number of items will be returned. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name__contains")]
    pub name__contains: Option<String>,
    /// The criticality that each asset belongs to.
    /// Enum values: `critical`, `high`, `medium`, `low`, `--`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountId__nin")]
    pub cloud_provider_account_id__nin: Option<String>,
    /// If true, total number of items will not be calculated. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__nexists")]
    pub all_tags_key__nexists: Option<String>,
    /// List of Site IDs to filter by. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "s1UpdatedAt__between")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type.
    /// Enum values: `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP | Active
    /// Directory (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assetEnvironment__nin")]
    pub asset_environment__nin: Option<String>,
    /// Name (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "names__nin")]
    pub names__nin: Option<String>,
    /// The cloud tags key. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in).
    /// Enum values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "subCategory__nin")]
    pub sub_category__nin: Option<String>,
    /// The status alerts of the asset.
    /// Enum values: `Infected`, `Healthy`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset.
    /// Enum values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`. Optional.
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
    /// The sub-category that each resource belongs to.
    /// Enum values: `All`, `Access Key and Secret`, `Access Management`,
    /// `Account`, `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review.
    /// Enum values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Limit number of returned items (1-1000). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id__contains")]
    pub id__contains: Option<String>,
    /// The cloud provider account name. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderAccountName__contains")]
    pub cloud_provider_account_name__contains: Option<String>,
    /// The status of the asset.
    /// Enum values: `Active`, `Inactive`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudTagsKey__contains")]
    pub cloud_tags_key__contains: Option<String>,
    /// Tags (not in). Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tagsKeyValue__nin")]
    pub tags_key_value__nin: Option<String>,
    /// User and cloud tag keys exists. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allTagsKey__exists")]
    pub all_tags_key__exists: Option<String>,
    /// Cursor position returned by the last request. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The status alerts of the asset (not in).
    /// Enum values: `Infected`, `Healthy`. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "infectionStatus__nin")]
    pub infection_status__nin: Option<String>,
    /// The cloud provider project ID. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloudProviderProjectId__contains")]
    pub cloud_provider_project_id__contains: Option<String>,
}

impl FunctionExportQuery {
    /// Construct an export query with the required `exportFormat` (`csv` or
    /// `json`).
    pub fn new(export_format: impl Into<String>) -> Self {
        Self {
            export_format: export_format.into(),
            tags_key__contains: None,
            asset_criticality__nin: None,
            missing_coverage: None,
            all_tags_key_value: None,
            cloud_provider_account_name__nin: None,
            tags_key__nin: None,
            cloud_resource_id__contains: None,
            cloud_provider_subscription_id__contains: None,
            tags_key__exists: None,
            region: None,
            tags_key_value__contains: None,
            cloud_tags_key__nin: None,
            tags_key: None,
            risk_factors__nin: None,
            tags_key__nexists: None,
            skip: None,
            cloud_provider_account_id__contains: None,
            image_name__contains: None,
            group_ids: None,
            active_coverage__nin: None,
            all_tags_key_value__nin: None,
            region__nin: None,
            all_tags_key__nin: None,
            cloud_tags_key_value__nin: None,
            surfaces: None,
            missing_coverage__nin: None,
            asset_status__nin: None,
            sort_by: None,
            region__contains: None,
            device_review__nin: None,
            cloud_provider_account_name: None,
            asset_contact_email__nin: None,
            alert_severity: None,
            account_ids: None,
            cloud_tags_key_value: None,
            resource_type__nin: None,
            asset_contact_email: None,
            risk_factors: None,
            asset_environment: None,
            surfaces__nin: None,
            id__in: None,
            cloud_tags_key_value__contains: None,
            resource_type__contains: None,
            tags_key_value: None,
            sort_order: None,
            cloud_provider_organization_unit__contains: None,
            cloud_provider_organization__contains: None,
            count_only: None,
            names: None,
            name__contains: None,
            asset_criticality: None,
            cloud_provider_account_id__nin: None,
            skip_count: None,
            all_tags_key__nexists: None,
            site_ids: None,
            s1_updated_at__between: None,
            resource_type: None,
            asset_environment__nin: None,
            names__nin: None,
            cloud_tags_key: None,
            sub_category__nin: None,
            infection_status: None,
            active_coverage: None,
            counts_for: None,
            csv_filter_id: None,
            cloud_provider_account_id: None,
            sub_category: None,
            device_review: None,
            all_tags_key: None,
            limit: None,
            id__contains: None,
            cloud_provider_account_name__contains: None,
            asset_status: None,
            cloud_tags_key__contains: None,
            tags_key_value__nin: None,
            all_tags_key__exists: None,
            cursor: None,
            infection_status__nin: None,
            cloud_provider_project_id__contains: None,
        }
    }
    /// Export format: `csv` or `json`.
    pub fn export_format(mut self, v: impl Into<String>) -> Self {
        self.export_format = v.into();
        self
    }
    /// List of Account IDs to filter by.
    pub fn account_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.account_ids = Some(join_csv(v));
        self
    }
    /// List of Site IDs to filter by.
    pub fn site_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.site_ids = Some(join_csv(v));
        self
    }
    /// List of Group IDs to filter by.
    pub fn group_ids<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.group_ids = Some(join_csv(v));
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, c: impl Into<String>) -> Self {
        self.cursor = Some(c.into());
        self
    }
}

/// Join an iterator of stringy values into a comma-separated query value.
fn join_csv<I, S>(values: I) -> String
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    values
        .into_iter()
        .map(Into::into)
        .collect::<Vec<_>>()
        .join(",")
}

impl InventoryFunctionService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/function` — Assets.
    ///
    /// Get assets.
    pub async fn list(
        &self,
        query: &FunctionListQuery,
    ) -> Result<Paginated<Function>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/function", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/function` — Assets using POST.
    ///
    /// POST API to get Assets.
    pub async fn list_post(
        &self,
        query: &FunctionPostQuery,
        body: &FunctionPostBody,
    ) -> Result<Paginated<Function>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(Method::POST, "/web/api/v2.1/xdr/assets/function", q, Some(body))
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/function/action` — Perform action.
    ///
    /// Perform action on selected assets.
    pub async fn perform_action(
        &self,
        query: &FunctionActionQuery,
        body: &FunctionActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(
                Method::POST,
                "/web/api/v2.1/xdr/assets/function/action",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/function/available-actions/with-status` —
    /// Available actions.
    ///
    /// Get available actions.
    pub async fn available_actions_with_status(
        &self,
        query: &FunctionActionQuery,
        body: &AffectedResourcesBody,
    ) -> Result<Response<AvailableActionWithStatusResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(
                Method::POST,
                "/web/api/v2.1/xdr/assets/function/available-actions/with-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/function/export` — Export assets to CSV or JSON.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    /// The shape of the body varies with `exportFormat`, so the raw JSON
    /// envelope is returned as [`serde_json::Value`].
    pub async fn export(
        &self,
        query: &FunctionExportQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/function/export", q)
            .await?)
    }
}
