//! `Inventory` tag — Inventory Resources.

use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory::{
    AssetCountsResponse, AvailableActionWithStatusResponse, CategoriesResponse, InventoryAsset,
    SubCategoriesResponse,
};
use crate::pagination::{Paginated, Response};

/// `Inventory` tag — operations on inventory assets.
pub struct InventoryService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Joins an iterator of string-likes by comma (the form array query params use).
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

/// Shared inventory-asset filter query parameters.
///
/// These filter fields are common to the list (`GET /xdr/assets`), POST list
/// (`POST /xdr/assets`), action (`POST /xdr/assets/action`), available-actions
/// (`POST /xdr/assets/available-actions/with-status`) and export
/// (`GET /xdr/assets/export`) endpoints. Per-method query structs embed this
/// set and add any method-specific extras (paging, sorting, export format,
/// scope ids, ...).
///
/// Every field is optional. Array filters are serialized as a single
/// comma-joined string (the form the API expects); use the builder methods,
/// which accept an iterator and join by comma.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryFilterQuery {
    /// Free-text filter by tag key (supports multiple values). Array param.
    #[serde(rename = "tagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Array param.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(rename = "assetCriticality__nin", skip_serializing_if = "Option::is_none")]
    pub asset_criticality_nin: Option<String>,
    /// The missing coverage for the asset. Array param.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
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
    /// The region. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Free-text filter by tag key value (supports multiple values). Array param.
    #[serde(rename = "tagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_contains: Option<String>,
    /// The cloud tags key (not in). Array param.
    #[serde(rename = "cloudTagsKey__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_nin: Option<String>,
    /// Tag Keys. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    /// List of Group IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The active coverage for the asset (not in). Array param.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "activeCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub active_coverage_nin: Option<String>,
    /// The agent console connectivity. Array param (`array<boolean>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_console_connectivity: Option<String>,
    /// The agent idr connectivity. Array param (`array<boolean>`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_idr_connectivity: Option<String>,
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
    /// The Surface that each asset belongs to. Array param.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The agent pending actions. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_pending_actions: Option<String>,
    /// The missing coverage for the asset (not in). Array param.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(rename = "missingCoverage__nin", skip_serializing_if = "Option::is_none")]
    pub missing_coverage_nin: Option<String>,
    /// The status of the asset (not in). Array param.
    /// Allowed values: `Active`, `Inactive`.
    #[serde(rename = "assetStatus__nin", skip_serializing_if = "Option::is_none")]
    pub asset_status_nin: Option<String>,
    /// The agent supported or unknown state. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epp_unsupported_unknown: Option<String>,
    /// The geographical area where cloud resources are hosted. Array param.
    #[serde(rename = "region__contains", skip_serializing_if = "Option::is_none")]
    pub region_contains: Option<String>,
    /// The category that each resource belongs to. Array param.
    /// Allowed values: `All`, `Account`, `AI ML`, `Application Integration`,
    /// `Cloud Application`, `Code`, `Container`, `Data Analysis`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The asset review (not in). Array param.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty).
    #[serde(rename = "deviceReview__nin", skip_serializing_if = "Option::is_none")]
    pub device_review_nin: Option<String>,
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
    /// The canonical name for the resource type (not in). Array param.
    /// Allowed values include: `Access Control and Surveillance System`,
    /// `Access Point`, `AD Certificate`, `AD Certificate Authority`,
    /// `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(rename = "resourceType__nin", skip_serializing_if = "Option::is_none")]
    pub resource_type_nin: Option<String>,
    /// Asset Contact Email. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Array param.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP |
    /// Active Directory. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The operating system family of the device. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os_family: Option<String>,
    /// The Surface that each asset belongs to (not in). Array param.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    #[serde(rename = "surfaces__nin", skip_serializing_if = "Option::is_none")]
    pub surfaces_nin: Option<String>,
    /// The ID. Array param.
    #[serde(rename = "id__in", skip_serializing_if = "Option::is_none")]
    pub id_in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values). Array param.
    #[serde(rename = "cloudTagsKeyValue__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value_contains: Option<String>,
    /// The Asset Type. Array param.
    #[serde(rename = "resourceType__contains", skip_serializing_if = "Option::is_none")]
    pub resource_type_contains: Option<String>,
    /// Tags. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The cloud provider organization unit. Array param.
    #[serde(rename = "cloudProviderOrganizationUnit__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit_contains: Option<String>,
    /// The cloud provider organization. Array param.
    #[serde(rename = "cloudProviderOrganization__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_contains: Option<String>,
    /// The agent operational state. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_operational_state: Option<String>,
    /// Name. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name. Array param.
    #[serde(rename = "name__contains", skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    /// The criticality that each asset belongs to. Array param.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The agent anti tampering status. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_anti_tampering_status: Option<String>,
    /// The cloud provider account id (not in). Array param.
    #[serde(rename = "cloudProviderAccountId__nin", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id_nin: Option<String>,
    /// User and cloud tag keys not exists. Array param.
    #[serde(rename = "allTagsKey__nexists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_nexists: Option<String>,
    /// List of Site IDs to filter by. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The agent VSS rollback status. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_vss_rollback_status: Option<String>,
    /// The Last Seen date and time for the asset.
    #[serde(rename = "s1UpdatedAt__between", skip_serializing_if = "Option::is_none")]
    pub s1_updated_at_between: Option<String>,
    /// The canonical name for the resource type. Array param.
    /// Allowed values include: `Access Control and Surveillance System`,
    /// `Access Point`, `AD Certificate`, `AD Certificate Authority`,
    /// `AD Certificate Template`, `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in - AWS | Azure | GCP |
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
    /// Allowed values include: `All`, `Access Key and Secret`,
    /// `Access Management`, `Account`, `Account Group`, `AD Objects`,
    /// `Administrative Unit`, `Admission Controller`.
    #[serde(rename = "subCategory__nin", skip_serializing_if = "Option::is_none")]
    pub sub_category_nin: Option<String>,
    /// The status alerts of the asset. Array param.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset. Array param.
    /// Allowed values: `CWS`, `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`,
    /// `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The cloud provider account id. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to. Array param.
    /// Allowed values include: `All`, `Access Key and Secret`,
    /// `Access Management`, `Account`, `Account Group`, `AD Objects`,
    /// `Administrative Unit`, `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review. Array param.
    /// Allowed values: `Not Reviewed`, `Under Analysis`, `Not Trusted`,
    /// `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The ID. Array param.
    #[serde(rename = "id__contains", skip_serializing_if = "Option::is_none")]
    pub id_contains: Option<String>,
    /// The cloud provider account name. Array param.
    #[serde(rename = "cloudProviderAccountName__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name_contains: Option<String>,
    /// The status of the asset. Array param.
    /// Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values). Array param.
    #[serde(rename = "cloudTagsKey__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_contains: Option<String>,
    /// Tags (not in). Array param.
    #[serde(rename = "tagsKeyValue__nin", skip_serializing_if = "Option::is_none")]
    pub tags_key_value_nin: Option<String>,
    /// The agent version. Array param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_agent_version: Option<String>,
    /// User and cloud tag keys exists. Array param.
    #[serde(rename = "allTagsKey__exists", skip_serializing_if = "Option::is_none")]
    pub all_tags_key_exists: Option<String>,
    /// The status alerts of the asset (not in). Array param.
    /// Allowed values: `Infected`, `Healthy`.
    #[serde(rename = "infectionStatus__nin", skip_serializing_if = "Option::is_none")]
    pub infection_status_nin: Option<String>,
    /// The cloud provider project ID. Array param.
    #[serde(rename = "cloudProviderProjectId__contains", skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id_contains: Option<String>,
}

impl InventoryFilterQuery {
    /// The ID (`id__in`). Array param.
    pub fn id_in<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_in = Some(join_csv(values));
        self
    }
    /// The ID (`id__contains`). Array param.
    pub fn id_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id_contains = Some(join_csv(values));
        self
    }
    /// Name (`names`). Array param.
    pub fn names<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.names = Some(join_csv(values));
        self
    }
    /// The name (`name__contains`). Array param.
    pub fn name_contains<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.name_contains = Some(join_csv(values));
        self
    }
    /// List of Account IDs to filter by (`accountIds`). Array param.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// List of Site IDs to filter by (`siteIds`). Array param.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// List of Group IDs to filter by (`groupIds`). Array param.
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(values));
        self
    }
    /// The criticality that each asset belongs to (`assetCriticality`). Array param.
    /// Allowed values: `critical`, `high`, `medium`, `low`, `--`.
    pub fn asset_criticality<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_criticality = Some(join_csv(values));
        self
    }
    /// The category that each resource belongs to (`category`). Array param.
    /// Allowed values: `All`, `Account`, `AI ML`, `Application Integration`,
    /// `Cloud Application`, `Code`, `Container`, `Data Analysis`.
    pub fn category<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.category = Some(join_csv(values));
        self
    }
    /// The sub-category that each resource belongs to (`subCategory`). Array param.
    pub fn sub_category<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.sub_category = Some(join_csv(values));
        self
    }
    /// The Surface that each asset belongs to (`surfaces`). Array param.
    /// Allowed values: `Cloud`, `Identity`, `Network`, `Endpoint`,
    /// `Network Discovery`.
    pub fn surfaces<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces = Some(join_csv(values));
        self
    }
    /// The canonical name for the resource type (`resourceType`). Array param.
    pub fn resource_type<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.resource_type = Some(join_csv(values));
        self
    }
    /// The region (`region`). Array param.
    pub fn region<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region = Some(join_csv(values));
        self
    }
    /// The status of the asset (`assetStatus`). Array param.
    /// Allowed values: `Active`, `Inactive`.
    pub fn asset_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_status = Some(join_csv(values));
        self
    }
    /// The status alerts of the asset (`infectionStatus`). Array param.
    /// Allowed values: `Infected`, `Healthy`.
    pub fn infection_status<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status = Some(join_csv(values));
        self
    }
    /// The active coverage for the asset (`activeCoverage`). Array param.
    pub fn active_coverage<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage = Some(join_csv(values));
        self
    }
    /// The missing coverage for the asset (`missingCoverage`). Array param.
    pub fn missing_coverage<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage = Some(join_csv(values));
        self
    }
    /// The risk factors associated with the asset (`riskFactors`). Array param.
    /// Allowed values: `Unresolved Alerts`, `High Value`.
    pub fn risk_factors<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors = Some(join_csv(values));
        self
    }
    /// The columns for which filter count would be returned for (`countsFor`). Array param.
    pub fn counts_for<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.counts_for = Some(join_csv(values));
        self
    }
    /// The ID of the CSV file to filter by (`csvFilterId`).
    pub fn csv_filter_id(mut self, id: i64) -> Self {
        self.csv_filter_id = Some(id);
        self
    }
    /// The Last Seen date and time for the asset (`s1UpdatedAt__between`).
    pub fn s1_updated_at_between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at_between = Some(v.into());
        self
    }
}

/// List/export-only paging, sorting and count params (serialized separately
/// from the shared filter set).
///
/// `serde_urlencoded` does not support `#[serde(flatten)]`, so these extras and
/// the [`InventoryFilterQuery`] are serialized independently and concatenated.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct PagingExtras {
    #[serde(skip_serializing_if = "Option::is_none")]
    skip: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    count_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skip_count: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    export_format: Option<String>,
}

/// Concatenates the (non-empty) urlencoded forms of `extras` and `filter`.
fn build_query_string<E: Serialize>(extras: &E, filter: &InventoryFilterQuery) -> String {
    let mut parts: Vec<String> = Vec::new();
    let extra_qs = serde_urlencoded::to_string(extras).unwrap_or_default();
    if !extra_qs.is_empty() {
        parts.push(extra_qs);
    }
    let filter_qs = serde_urlencoded::to_string(filter).unwrap_or_default();
    if !filter_qs.is_empty() {
        parts.push(filter_qs);
    }
    parts.join("&")
}

/// Query params for `GET /web/api/v2.1/xdr/assets`.
///
/// Wraps the shared [`InventoryFilterQuery`] and adds the list-specific paging,
/// sorting, and count controls. Array filter params are serialized comma-joined.
#[derive(Debug, Default)]
pub struct ListAssetsQuery {
    /// Shared inventory-asset filter fields.
    pub filter: InventoryFilterQuery,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    pub cursor: Option<String>,
    /// The column to sort the results by. Optional.
    /// Allowed values: `s1GroupName`, `cpu`, `legacyIdentityPolicyName`,
    /// `previousOsType`, `previousOsVersion`, `agentFirewallStatus`,
    /// `s1UpdatedAt`, `cloudProviderResourceGroup`.
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`.
    pub sort_order: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    pub skip_count: Option<bool>,
}

impl ListAssetsQuery {
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: InventoryFilterQuery) -> Self {
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
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction (`asc` or `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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

    /// Build the full querystring (paging extras + the filter set).
    fn to_query_string(&self) -> String {
        let extras = PagingExtras {
            skip: self.skip,
            limit: self.limit,
            cursor: self.cursor.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            count_only: self.count_only,
            skip_count: self.skip_count,
            export_format: None,
        };
        build_query_string(&extras, &self.filter)
    }
}

/// Scope-only query params (`accountIds`, `siteIds`, `groupIds`).
///
/// Used by the POST list (`POST /xdr/assets`) and the count endpoints
/// (`/xdr/assets/asset-counts`, `/xdr/assets/categories`,
/// `/xdr/assets/sub-categories`). Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopeQuery {
    /// List of Account IDs to filter by. Array param. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Array param. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Array param. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl ScopeQuery {
    /// List of Account IDs to filter by (`accountIds`). Array param.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// List of Site IDs to filter by (`siteIds`). Array param.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// List of Group IDs to filter by (`groupIds`). Array param.
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(values));
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/action`.
///
/// Only the shared [`InventoryFilterQuery`] filter fields are accepted as query
/// params on this endpoint (the action itself is supplied in the body).
#[derive(Debug, Default)]
pub struct PerformActionQuery {
    /// Shared inventory-asset filter fields.
    pub filter: InventoryFilterQuery,
}

impl PerformActionQuery {
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: InventoryFilterQuery) -> Self {
        self.filter = filter;
        self
    }
    /// Build the full querystring (the filter set).
    fn to_query_string(&self) -> String {
        serde_urlencoded::to_string(&self.filter).unwrap_or_default()
    }
}

/// Query params for
/// `POST /web/api/v2.1/xdr/assets/available-actions/with-status`.
///
/// Only the shared [`InventoryFilterQuery`] filter fields are accepted as query
/// params on this endpoint.
#[derive(Debug, Default)]
pub struct AvailableActionsQuery {
    /// Shared inventory-asset filter fields.
    pub filter: InventoryFilterQuery,
}

impl AvailableActionsQuery {
    /// Set the shared filter fields.
    pub fn filter(mut self, filter: InventoryFilterQuery) -> Self {
        self.filter = filter;
        self
    }
    /// Build the full querystring (the filter set).
    fn to_query_string(&self) -> String {
        serde_urlencoded::to_string(&self.filter).unwrap_or_default()
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/export`.
///
/// Wraps the shared [`InventoryFilterQuery`] and adds export-specific paging/
/// sorting controls plus the required `exportFormat`. The only required param is
/// `export_format`; set it via [`ExportAssetsQuery::new`].
#[derive(Debug, Default)]
pub struct ExportAssetsQuery {
    /// Shared inventory-asset filter fields.
    pub filter: InventoryFilterQuery,
    /// Export format. Required. Allowed values: `csv`, `json`.
    pub export_format: String,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`. Optional.
    pub skip: Option<i64>,
    /// Limit number of returned items (1-1000). Optional.
    pub limit: Option<i64>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items. Optional.
    pub cursor: Option<String>,
    /// The column to sort the results by. Optional.
    /// Allowed values: `s1GroupName`, `cpu`, `legacyIdentityPolicyName`,
    /// `previousOsType`, `previousOsVersion`, `agentFirewallStatus`,
    /// `s1UpdatedAt`, `cloudProviderResourceGroup`.
    pub sort_by: Option<String>,
    /// Sort direction. Optional. Allowed values: `asc`, `desc`.
    pub sort_order: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects. Optional.
    pub count_only: Option<bool>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time. Optional.
    pub skip_count: Option<bool>,
}

impl ExportAssetsQuery {
    /// Create a new export query with the required export format
    /// (`csv` or `json`).
    pub fn new(export_format: impl Into<String>) -> Self {
        Self {
            filter: InventoryFilterQuery::default(),
            export_format: export_format.into(),
            skip: None,
            limit: None,
            cursor: None,
            sort_by: None,
            sort_order: None,
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
    pub fn filter(mut self, filter: InventoryFilterQuery) -> Self {
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
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction (`asc` or `desc`).
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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

    /// Build the full querystring (`exportFormat` + paging extras + filter set).
    fn to_query_string(&self) -> String {
        let extras = PagingExtras {
            skip: self.skip,
            limit: self.limit,
            cursor: self.cursor.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            count_only: self.count_only,
            skip_count: self.skip_count,
            export_format: Some(self.export_format.clone()),
        };
        build_query_string(&extras, &self.filter)
    }
}

/// Query params for `GET /web/api/v2.1/xdr/assets/cloud-tags/export`.
///
/// The only required param is `export_format`; set it via
/// [`ExportCloudTagsQuery::new`]. Array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportCloudTagsQuery {
    /// Export format. Required. Allowed values: `csv`, `json`.
    pub export_format: String,
    /// List of resource ids to filter by. Array param. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// List of Account IDs to filter by. Array param. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by. Array param. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by. Array param. Optional.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl ExportCloudTagsQuery {
    /// Create a new cloud-tags export query with the required export format
    /// (`csv` or `json`).
    pub fn new(export_format: impl Into<String>) -> Self {
        Self {
            export_format: export_format.into(),
            id: None,
            account_ids: None,
            site_ids: None,
            group_ids: None,
        }
    }
    /// Set the required export format. Allowed values: `csv`, `json`.
    pub fn export_format(mut self, v: impl Into<String>) -> Self {
        self.export_format = v.into();
        self
    }
    /// List of resource ids to filter by (`id`). Array param.
    pub fn id<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id = Some(join_csv(values));
        self
    }
    /// List of Account IDs to filter by (`accountIds`). Array param.
    pub fn account_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.account_ids = Some(join_csv(values));
        self
    }
    /// List of Site IDs to filter by (`siteIds`). Array param.
    pub fn site_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.site_ids = Some(join_csv(values));
        self
    }
    /// List of Group IDs to filter by (`groupIds`). Array param.
    pub fn group_ids<I, S>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.group_ids = Some(join_csv(values));
        self
    }
}

/// Body for `POST /web/api/v2.1/xdr/assets`
/// (`InventoryViewInputSchema`).
///
/// `filter` is required (freeform `PaginatedInventoryFilter`); `data` is an
/// optional freeform value.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAssetsBody {
    /// Data. Optional, nullable freeform value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// Filter. Required freeform `PaginatedInventoryFilter` value.
    pub filter: serde_json::Value,
}

impl ListAssetsBody {
    /// Create a new body with the required `filter`.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { data: None, filter }
    }
    /// Set the optional `data` value.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Body for `POST /web/api/v2.1/xdr/assets/action`
/// (`InventoryActionPayloadSchema`).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PerformActionBody {
    /// Action name. Required. Allowed values: `start_full_scan`,
    /// `stop_full_scan`, `enable_cws_monitoring`, `enable_cns`, `disable_cns`,
    /// `start_vm_scan`, `export_resource_details`,
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
/// `POST /web/api/v2.1/xdr/assets/available-actions/with-status`
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

impl InventoryService<'_> {
    /// **Assets** — Get assets.
    ///
    /// Get assets.
    ///
    /// `GET /web/api/v2.1/xdr/assets`
    pub async fn list(&self, query: &ListAssetsQuery) -> Result<Paginated<InventoryAsset>, Error> {
        let qs = query.to_query_string();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self.client.http().get("/web/api/v2.1/xdr/assets", q).await?)
    }

    /// **Assets using POST** — POST API to get assets.
    ///
    /// POST API to get assets.
    ///
    /// `POST /web/api/v2.1/xdr/assets`
    pub async fn list_post(
        &self,
        query: &ScopeQuery,
        body: &ListAssetsBody,
    ) -> Result<Paginated<InventoryAsset>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let path = if qs.is_empty() {
            "/web/api/v2.1/xdr/assets".to_owned()
        } else {
            format!("/web/api/v2.1/xdr/assets?{qs}")
        };
        Ok(self.client.http().post(&path, body).await?)
    }

    /// **Perform action** — Perform action on selected assets.
    ///
    /// Perform action on selected assets.
    ///
    /// `POST /web/api/v2.1/xdr/assets/action`
    pub async fn perform_action(
        &self,
        query: &PerformActionQuery,
        body: &PerformActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = query.to_query_string();
        let path = if qs.is_empty() {
            "/web/api/v2.1/xdr/assets/action".to_owned()
        } else {
            format!("/web/api/v2.1/xdr/assets/action?{qs}")
        };
        Ok(self.client.http().post(&path, body).await?)
    }

    /// **Get inventory counts for menu items** — Get inventory counts
    /// categories, subcategories and surfaces.
    ///
    /// Get inventory counts categories, subcategories and surfaces.
    ///
    /// `GET /web/api/v2.1/xdr/assets/asset-counts`
    pub async fn asset_counts(
        &self,
        query: &ScopeQuery,
    ) -> Result<Response<AssetCountsResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/asset-counts", q)
            .await?)
    }

    /// **Available actions** — Get available actions.
    ///
    /// Get available actions.
    ///
    /// `POST /web/api/v2.1/xdr/assets/available-actions/with-status`
    pub async fn available_actions(
        &self,
        query: &AvailableActionsQuery,
        body: &AvailableActionsBody,
    ) -> Result<Response<AvailableActionWithStatusResponse>, Error> {
        let qs = query.to_query_string();
        let path = if qs.is_empty() {
            "/web/api/v2.1/xdr/assets/available-actions/with-status".to_owned()
        } else {
            format!("/web/api/v2.1/xdr/assets/available-actions/with-status?{qs}")
        };
        Ok(self.client.http().post(&path, body).await?)
    }

    /// **Categories and counts** — Get inventory categories and their asset
    /// counts.
    ///
    /// Get inventory categories and their asset counts.
    ///
    /// `GET /web/api/v2.1/xdr/assets/categories`
    pub async fn categories(
        &self,
        query: &ScopeQuery,
    ) -> Result<Response<CategoriesResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/categories", q)
            .await?)
    }

    /// **Export Cloud Tags to CSV or JSON** — Returns the tags for given id in a
    /// CSV or JSON format.
    ///
    /// Returns the tags for given id in a CSV or JSON format.
    ///
    /// `GET /web/api/v2.1/xdr/assets/cloud-tags/export`
    pub async fn export_cloud_tags(
        &self,
        query: &ExportCloudTagsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/cloud-tags/export", q)
            .await?)
    }

    /// **Export assets to CSV or JSON** — Returns the results for given
    /// inventory filter in a CSV or JSON format.
    ///
    /// Returns the results for given inventory filter in a CSV or JSON format.
    ///
    /// `GET /web/api/v2.1/xdr/assets/export`
    pub async fn export(
        &self,
        query: &ExportAssetsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = query.to_query_string();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/export", q)
            .await?)
    }

    /// **Counts per subcategory for categories** — Get asset counts per
    /// subcategory for each category.
    ///
    /// Get asset counts per subcategory for each category.
    ///
    /// `GET /web/api/v2.1/xdr/assets/sub-categories`
    pub async fn sub_categories(
        &self,
        query: &ScopeQuery,
    ) -> Result<Response<SubCategoriesResponse>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/sub-categories", q)
            .await?)
    }
}
