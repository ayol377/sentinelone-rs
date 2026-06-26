//! `Inventory Network` tag — network inventory assets.
//!
//! Endpoints under `/web/api/v2.1/xdr/assets/network`.

use sentinelone_http::Method;
use serde::Serialize;

use crate::client::ManagementClient;
use crate::error::Error;
use crate::models::inventory_network::{AvailableActionWithStatus, NetworkAsset};
use crate::pagination::{Paginated, Response};

/// `Inventory Network` tag — Inventory Network Resources.
///
/// Provides access to network inventory assets: listing/filtering them (via
/// both `GET` and `POST`), performing actions on a selection, querying the
/// actions available for a selection, and exporting results to CSV/JSON.
pub struct InventoryNetworkService<'a> {
    pub(crate) client: &'a ManagementClient,
}

/// Query params for `GET /web/api/v2.1/xdr/assets/network` — Assets.
///
/// Every field is optional. Array params are serialized comma-joined, as the
/// API expects. Enum-typed params are kept as `String` for forward
/// compatibility; their allowed values are documented per field.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetsQuery {
    /// Free-text filter by tag key (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality__nin: Option<String>,
    /// The missing coverage for the asset. Allowed values: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name__nin: Option<String>,
    /// Tag Keys (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__nin: Option<String>,
    /// The cloud resource ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id__contains: Option<String>,
    /// The cloud provider subscription ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id__contains: Option<String>,
    /// Tag Keys exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__exists: Option<String>,
    /// The region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Free-text filter by tag key value (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value__contains: Option<String>,
    /// The cloud tags key (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key__nin: Option<String>,
    /// Tag Keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). Allowed values:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__nexists: Option<String>,
    /// Skip first number of items (0-1000). To iterate over more than 1000
    /// items, use `cursor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id__contains: Option<String>,
    /// Free-text filter by the image name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name__contains: Option<String>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The active coverage for the asset (not in). Allowed values: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage__nin: Option<String>,
    /// User and cloud tags (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value__nin: Option<String>,
    /// The region (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region__nin: Option<String>,
    /// User and cloud tag keys (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nin: Option<String>,
    /// The cloud tags key value (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value__nin: Option<String>,
    /// The Surface that each asset belongs to. Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The missing coverage for the asset (not in). Allowed values: `CWS`,
    /// `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage__nin: Option<String>,
    /// The status of the asset (not in). Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status__nin: Option<String>,
    /// The column to sort the results by. Allowed values: `s1GroupName`,
    /// `s1UpdatedAt`, `cnsIsDefault`, `cloudProviderResourceGroup`,
    /// `cnsHasEgressPermissionsRules`, `region`, `dnsStatus`,
    /// `cloudProviderOrganization`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The geographical area where cloud resources are hosted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region__contains: Option<String>,
    /// The asset review (not in). Allowed values: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review__nin: Option<String>,
    /// The cloud provider account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email__nin: Option<String>,
    /// The severity of the alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in). Allowed values:
    /// `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type__nin: Option<String>,
    /// Asset Contact Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Allowed values:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in — AWS | Azure | GCP | Active
    /// Directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in). Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces__nin: Option<String>,
    /// The ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id__in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value__contains: Option<String>,
    /// The Asset Type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type__contains: Option<String>,
    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The cloud provider organization unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit__contains: Option<String>,
    /// The cloud provider organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization__contains: Option<String>,
    /// If true, only total number of items will be returned, without any of the
    /// actual objects.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
    /// The criticality that each asset belongs to. Allowed values: `critical`,
    /// `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id__nin: Option<String>,
    /// If true, total number of items will not be calculated, which speeds up
    /// execution time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nexists: Option<String>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type. Allowed values:
    /// `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in — AWS | Azure | GCP | Active
    /// Directory (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment__nin: Option<String>,
    /// Name (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names__nin: Option<String>,
    /// The cloud tags key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in). Allowed values:
    /// `All`, `Access Key and Secret`, `Access Management`, `Account`,
    /// `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category__nin: Option<String>,
    /// The status alerts of the asset. Allowed values: `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset. Allowed values: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The cloud provider account id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to. Allowed values: `All`,
    /// `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review. Allowed values: `Not Reviewed`, `Under Analysis`,
    /// `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id__contains: Option<String>,
    /// The cloud provider account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name__contains: Option<String>,
    /// The status of the asset. Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key__contains: Option<String>,
    /// Tags (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value__nin: Option<String>,
    /// User and cloud tag keys exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key__exists: Option<String>,
    /// Cursor position returned by the last request. Use to iterate over more
    /// than 1000 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The status alerts of the asset (not in). Allowed values: `Infected`,
    /// `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status__nin: Option<String>,
    /// The cloud provider project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id__contains: Option<String>,
}

/// Comma-join an iterator of string-like values into a single query value.
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

impl GetAssetsQuery {
    /// Free-text filter by tag key (supports multiple values).
    pub fn tags_key__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__contains = Some(join_csv(v));
        self
    }
    /// The criticality that each asset belongs to (not in). Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`.
    pub fn asset_criticality__nin<I, S>(mut self, v: I) -> Self
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
    pub fn cloud_provider_account_name__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_name__nin = Some(join_csv(v));
        self
    }
    /// Tag Keys (not in).
    pub fn tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key__nin = Some(join_csv(v));
        self
    }
    /// The cloud resource ID.
    pub fn cloud_resource_id__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_resource_id__contains = Some(join_csv(v));
        self
    }
    /// The cloud provider subscription ID.
    pub fn cloud_provider_subscription_id__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_subscription_id__contains = Some(join_csv(v));
        self
    }
    /// Tag Keys exists.
    pub fn tags_key__exists<I, S>(mut self, v: I) -> Self
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
    pub fn tags_key_value__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__contains = Some(join_csv(v));
        self
    }
    /// The cloud tags key (not in).
    pub fn cloud_tags_key__nin<I, S>(mut self, v: I) -> Self
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
    pub fn risk_factors__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.risk_factors__nin = Some(join_csv(v));
        self
    }
    /// Tag Keys not exists.
    pub fn tags_key__nexists<I, S>(mut self, v: I) -> Self
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
    pub fn cloud_provider_account_id__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id__contains = Some(join_csv(v));
        self
    }
    /// Free-text filter by the image name.
    pub fn image_name__contains<I, S>(mut self, v: I) -> Self
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
    pub fn active_coverage__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.active_coverage__nin = Some(join_csv(v));
        self
    }
    /// User and cloud tags (not in).
    pub fn all_tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key_value__nin = Some(join_csv(v));
        self
    }
    /// The region (not in).
    pub fn region__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region__nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys (not in).
    pub fn all_tags_key__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__nin = Some(join_csv(v));
        self
    }
    /// The cloud tags key value (not in).
    pub fn cloud_tags_key_value__nin<I, S>(mut self, v: I) -> Self
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
    pub fn missing_coverage__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.missing_coverage__nin = Some(join_csv(v));
        self
    }
    /// The status of the asset (not in).
    pub fn asset_status__nin<I, S>(mut self, v: I) -> Self
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
    pub fn region__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.region__contains = Some(join_csv(v));
        self
    }
    /// The asset review (not in).
    pub fn device_review__nin<I, S>(mut self, v: I) -> Self
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
    pub fn asset_contact_email__nin<I, S>(mut self, v: I) -> Self
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
    pub fn resource_type__nin<I, S>(mut self, v: I) -> Self
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
    pub fn surfaces__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.surfaces__nin = Some(join_csv(v));
        self
    }
    /// The ID.
    pub fn id__in<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__in = Some(join_csv(v));
        self
    }
    /// Free-text filter by cloud tag key value (supports multiple values).
    pub fn cloud_tags_key_value__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key_value__contains = Some(join_csv(v));
        self
    }
    /// The Asset Type.
    pub fn resource_type__contains<I, S>(mut self, v: I) -> Self
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
    pub fn cloud_provider_organization_unit__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization_unit__contains = Some(join_csv(v));
        self
    }
    /// The cloud provider organization.
    pub fn cloud_provider_organization__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_organization__contains = Some(join_csv(v));
        self
    }
    /// If true, only total number of items will be returned.
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
    pub fn name__contains<I, S>(mut self, v: I) -> Self
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
    pub fn cloud_provider_account_id__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_account_id__nin = Some(join_csv(v));
        self
    }
    /// If true, total number of items will not be calculated.
    pub fn skip_count(mut self, v: bool) -> Self {
        self.skip_count = Some(v);
        self
    }
    /// User and cloud tag keys not exists.
    pub fn all_tags_key__nexists<I, S>(mut self, v: I) -> Self
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
    pub fn s1_updated_at__between(mut self, v: impl Into<String>) -> Self {
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
    pub fn asset_environment__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.asset_environment__nin = Some(join_csv(v));
        self
    }
    /// Name (not in).
    pub fn names__nin<I, S>(mut self, v: I) -> Self
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
    pub fn sub_category__nin<I, S>(mut self, v: I) -> Self
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
    pub fn id__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.id__contains = Some(join_csv(v));
        self
    }
    /// The cloud provider account name.
    pub fn cloud_provider_account_name__contains<I, S>(mut self, v: I) -> Self
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
    pub fn cloud_tags_key__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_tags_key__contains = Some(join_csv(v));
        self
    }
    /// Tags (not in).
    pub fn tags_key_value__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.tags_key_value__nin = Some(join_csv(v));
        self
    }
    /// User and cloud tag keys exists.
    pub fn all_tags_key__exists<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.all_tags_key__exists = Some(join_csv(v));
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
    /// The status alerts of the asset (not in).
    pub fn infection_status__nin<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.infection_status__nin = Some(join_csv(v));
        self
    }
    /// The cloud provider project ID.
    pub fn cloud_provider_project_id__contains<I, S>(mut self, v: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.cloud_provider_project_id__contains = Some(join_csv(v));
        self
    }
}

/// Query params shared by the `POST` filter/action/available-actions endpoints
/// that accept the inventory filter as query parameters.
///
/// Used by `POST /web/api/v2.1/xdr/assets/network/action` and
/// `POST /web/api/v2.1/xdr/assets/network/available-actions/with-status`. All
/// fields are optional; array params are serialized comma-joined. Enum-typed
/// params are kept as `String` for forward compatibility.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkActionFilterQuery {
    /// Free-text filter by tag key (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__contains: Option<String>,
    /// The Asset Type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type__contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality__nin: Option<String>,
    /// The status alerts of the asset (not in). Allowed values: `Infected`,
    /// `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status__nin: Option<String>,
    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// The missing coverage for the asset. Allowed values: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider organization unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit__contains: Option<String>,
    /// The cloud provider account name (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name__nin: Option<String>,
    /// The geographical area where cloud resources are hosted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region__contains: Option<String>,
    /// Tag Keys (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__nin: Option<String>,
    /// The cloud resource ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id__contains: Option<String>,
    /// The cloud provider subscription ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id__contains: Option<String>,
    /// The cloud provider account id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The asset review (not in). Allowed values: `Not Reviewed`,
    /// `Under Analysis`, `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review__nin: Option<String>,
    /// The sub-category that each resource belongs to. Allowed values: `All`,
    /// `Access Key and Secret`, `Access Management`, `Account`, `Account Group`,
    /// `AD Objects`, `Administrative Unit`, `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The cloud provider organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization__contains: Option<String>,
    /// Tag Keys exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__exists: Option<String>,
    /// The cloud provider account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// The status of the asset (not in). Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status__nin: Option<String>,
    /// The asset review. Allowed values: `Not Reviewed`, `Under Analysis`,
    /// `Not Trusted`, `Allowed`, `` (empty).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// Asset Contact Email (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email__nin: Option<String>,
    /// User and cloud tag keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// The region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The severity of the alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// Free-text filter by tag key value (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value__contains: Option<String>,
    /// The cloud tags key (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key__nin: Option<String>,
    /// Tag Keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
    /// The criticality that each asset belongs to. Allowed values: `critical`,
    /// `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The risk factors associated with the asset (not in). Allowed values:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__nexists: Option<String>,
    /// The cloud provider account id (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id__nin: Option<String>,
    /// The canonical name for the resource type (not in). Allowed values:
    /// `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type__nin: Option<String>,
    /// The ID of the CSV file to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id__contains: Option<String>,
    /// The cloud provider account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id__contains: Option<String>,
    /// Free-text filter by the image name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name__contains: Option<String>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The cloud provider account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name__contains: Option<String>,
    /// The status of the asset. Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// User and cloud tag keys not exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nexists: Option<String>,
    /// Asset Contact Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Allowed values:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The active coverage for the asset (not in). Allowed values: `CWS`, `CDS`,
    /// `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage__nin: Option<String>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The environment that the asset exists in — AWS | Azure | GCP | Active
    /// Directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Last Seen date and time for the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type. Allowed values:
    /// `Access Control and Surveillance System`, `Access Point`,
    /// `AD Certificate`, `AD Certificate Authority`, `AD Certificate Template`,
    /// `AD Containers`, `AD DNS Zone`, `AD Domain`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment__nin: Option<String>,
    /// The missing coverage for the asset (not in). Allowed values: `CWS`,
    /// `CDS`, `EPP`, `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`,
    /// `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage__nin: Option<String>,
    /// User and cloud tags (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value__nin: Option<String>,
    /// The region (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region__nin: Option<String>,
    /// User and cloud tag keys (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nin: Option<String>,
    /// Name (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names__nin: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key__contains: Option<String>,
    /// Tags (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value__nin: Option<String>,
    /// The Surface that each asset belongs to (not in). Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces__nin: Option<String>,
    /// User and cloud tag keys exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key__exists: Option<String>,
    /// The cloud tags key value (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value__nin: Option<String>,
    /// The cloud tags key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The Surface that each asset belongs to. Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The sub-category that each resource belongs to (not in). Allowed values:
    /// `All`, `Access Key and Secret`, `Access Management`, `Account`,
    /// `Account Group`, `AD Objects`, `Administrative Unit`,
    /// `Admission Controller`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category__nin: Option<String>,
    /// The ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id__in: Option<String>,
    /// The status alerts of the asset. Allowed values: `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset. Allowed values: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value__contains: Option<String>,
    /// The cloud provider project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id__contains: Option<String>,
}

macro_rules! csv_setter {
    ($field:ident, $doc:expr) => {
        #[doc = $doc]
        pub fn $field<I, S>(mut self, v: I) -> Self
        where
            I: IntoIterator<Item = S>,
            S: AsRef<str>,
        {
            self.$field = Some(join_csv(v));
            self
        }
    };
}

impl NetworkActionFilterQuery {
    csv_setter!(tags_key__contains, "Free-text filter by tag key (supports multiple values).");
    csv_setter!(resource_type__contains, "The Asset Type.");
    csv_setter!(asset_criticality__nin, "The criticality that each asset belongs to (not in).");
    csv_setter!(infection_status__nin, "The status alerts of the asset (not in).");
    csv_setter!(tags_key_value, "Tags.");
    csv_setter!(missing_coverage, "The missing coverage for the asset.");
    csv_setter!(all_tags_key_value, "User and cloud tags.");
    csv_setter!(cloud_provider_organization_unit__contains, "The cloud provider organization unit.");
    csv_setter!(cloud_provider_account_name__nin, "The cloud provider account name (not in).");
    csv_setter!(region__contains, "The geographical area where cloud resources are hosted.");
    csv_setter!(tags_key__nin, "Tag Keys (not in).");
    csv_setter!(cloud_resource_id__contains, "The cloud resource ID.");
    csv_setter!(cloud_provider_subscription_id__contains, "The cloud provider subscription ID.");
    csv_setter!(cloud_provider_account_id, "The cloud provider account id.");
    csv_setter!(device_review__nin, "The asset review (not in).");
    csv_setter!(sub_category, "The sub-category that each resource belongs to.");
    csv_setter!(cloud_provider_organization__contains, "The cloud provider organization.");
    csv_setter!(tags_key__exists, "Tag Keys exists.");
    csv_setter!(cloud_provider_account_name, "The cloud provider account name.");
    csv_setter!(asset_status__nin, "The status of the asset (not in).");
    csv_setter!(device_review, "The asset review.");
    csv_setter!(asset_contact_email__nin, "Asset Contact Email (not in).");
    csv_setter!(all_tags_key, "User and cloud tag keys.");
    csv_setter!(region, "The region.");
    csv_setter!(alert_severity, "The severity of the alert.");
    csv_setter!(account_ids, "List of Account IDs to filter by.");
    csv_setter!(cloud_tags_key_value, "The cloud tags key value.");
    csv_setter!(names, "Name.");
    csv_setter!(tags_key_value__contains, "Free-text filter by tag key value (supports multiple values).");
    csv_setter!(cloud_tags_key__nin, "The cloud tags key (not in).");
    csv_setter!(tags_key, "Tag Keys.");
    csv_setter!(name__contains, "The name.");
    csv_setter!(asset_criticality, "The criticality that each asset belongs to.");
    csv_setter!(risk_factors__nin, "The risk factors associated with the asset (not in).");
    csv_setter!(tags_key__nexists, "Tag Keys not exists.");
    csv_setter!(cloud_provider_account_id__nin, "The cloud provider account id (not in).");
    csv_setter!(resource_type__nin, "The canonical name for the resource type (not in).");
    csv_setter!(id__contains, "The ID.");
    csv_setter!(cloud_provider_account_id__contains, "The cloud provider account ID.");
    csv_setter!(image_name__contains, "Free-text filter by the image name.");
    csv_setter!(group_ids, "List of Group IDs to filter by.");
    csv_setter!(cloud_provider_account_name__contains, "The cloud provider account name.");
    csv_setter!(asset_status, "The status of the asset.");
    csv_setter!(all_tags_key__nexists, "User and cloud tag keys not exists.");
    csv_setter!(asset_contact_email, "Asset Contact Email.");
    csv_setter!(risk_factors, "The risk factors associated with the asset.");
    csv_setter!(active_coverage__nin, "The active coverage for the asset (not in).");
    csv_setter!(site_ids, "List of Site IDs to filter by.");
    csv_setter!(asset_environment, "The environment that the asset exists in.");
    csv_setter!(resource_type, "The canonical name for the resource type.");
    csv_setter!(asset_environment__nin, "The environment that the asset exists in (not in).");
    csv_setter!(missing_coverage__nin, "The missing coverage for the asset (not in).");
    csv_setter!(all_tags_key_value__nin, "User and cloud tags (not in).");
    csv_setter!(region__nin, "The region (not in).");
    csv_setter!(all_tags_key__nin, "User and cloud tag keys (not in).");
    csv_setter!(names__nin, "Name (not in).");
    csv_setter!(cloud_tags_key__contains, "Free-text filter by cloud tag key (supports multiple values).");
    csv_setter!(tags_key_value__nin, "Tags (not in).");
    csv_setter!(surfaces__nin, "The Surface that each asset belongs to (not in).");
    csv_setter!(all_tags_key__exists, "User and cloud tag keys exists.");
    csv_setter!(cloud_tags_key_value__nin, "The cloud tags key value (not in).");
    csv_setter!(cloud_tags_key, "The cloud tags key.");
    csv_setter!(surfaces, "The Surface that each asset belongs to.");
    csv_setter!(sub_category__nin, "The sub-category that each resource belongs to (not in).");
    csv_setter!(id__in, "The ID.");
    csv_setter!(infection_status, "The status alerts of the asset.");
    csv_setter!(active_coverage, "The active coverage for the asset.");
    csv_setter!(counts_for, "The columns for which filter count would be returned for.");
    csv_setter!(cloud_tags_key_value__contains, "Free-text filter by cloud tag key value (supports multiple values).");
    csv_setter!(cloud_provider_project_id__contains, "The cloud provider project ID.");

    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self {
        self.csv_filter_id = Some(n);
        self
    }
    /// The Last Seen date and time for the asset.
    pub fn s1_updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at__between = Some(v.into());
        self
    }
}

/// Query params for `POST /web/api/v2.1/xdr/assets/network` — Assets using POST.
///
/// Only org-scope filters are accepted as query params on this endpoint; the
/// detailed filter is supplied via the request body. All fields are optional;
/// array params are serialized comma-joined.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAssetsQuery {
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
}

impl PostAssetsQuery {
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

/// Query params for `GET /web/api/v2.1/xdr/assets/network/export` — Export
/// assets to CSV or JSON.
///
/// Note: `exportFormat` is required and is therefore a dedicated argument on
/// the [`InventoryNetworkService::export`] method rather than a field here. All
/// fields below are optional; array params are serialized comma-joined. Enum
/// params are kept as `String` for forward compatibility.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportAssetsQuery {
    /// Free-text filter by tag key (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__contains: Option<String>,
    /// The criticality that each asset belongs to (not in). Allowed values:
    /// `critical`, `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality__nin: Option<String>,
    /// The missing coverage for the asset. Allowed values: `CWS`, `CDS`, `EPP`,
    /// `Ranger Insights`, `RAD`, `ISPM`, `Data Classification`, `CNS KSPM`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage: Option<String>,
    /// User and cloud tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value: Option<String>,
    /// The cloud provider account name (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name__nin: Option<String>,
    /// Tag Keys (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__nin: Option<String>,
    /// The cloud resource ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_resource_id__contains: Option<String>,
    /// The cloud provider subscription ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_subscription_id__contains: Option<String>,
    /// Tag Keys exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__exists: Option<String>,
    /// The region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Free-text filter by tag key value (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value__contains: Option<String>,
    /// The cloud tags key (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key__nin: Option<String>,
    /// Tag Keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key: Option<String>,
    /// The risk factors associated with the asset (not in). Allowed values:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors__nin: Option<String>,
    /// Tag Keys not exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key__nexists: Option<String>,
    /// Skip first number of items (0-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<i64>,
    /// The cloud provider account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id__contains: Option<String>,
    /// Free-text filter by the image name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_name__contains: Option<String>,
    /// List of Group IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    /// The active coverage for the asset (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage__nin: Option<String>,
    /// User and cloud tags (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key_value__nin: Option<String>,
    /// The region (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region__nin: Option<String>,
    /// User and cloud tag keys (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nin: Option<String>,
    /// The cloud tags key value (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value__nin: Option<String>,
    /// The Surface that each asset belongs to. Allowed values: `Cloud`,
    /// `Identity`, `Network`, `Endpoint`, `Network Discovery`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces: Option<String>,
    /// The missing coverage for the asset (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_coverage__nin: Option<String>,
    /// The status of the asset (not in). Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status__nin: Option<String>,
    /// The column to sort the results by. Allowed values: `s1GroupName`,
    /// `s1UpdatedAt`, `cnsIsDefault`, `cloudProviderResourceGroup`,
    /// `cnsHasEgressPermissionsRules`, `region`, `dnsStatus`,
    /// `cloudProviderOrganization`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// The geographical area where cloud resources are hosted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region__contains: Option<String>,
    /// The asset review (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review__nin: Option<String>,
    /// The cloud provider account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name: Option<String>,
    /// Asset Contact Email (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email__nin: Option<String>,
    /// The severity of the alert.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_severity: Option<String>,
    /// List of Account IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<String>,
    /// The cloud tags key value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value: Option<String>,
    /// The canonical name for the resource type (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type__nin: Option<String>,
    /// Asset Contact Email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_contact_email: Option<String>,
    /// The risk factors associated with the asset. Allowed values:
    /// `Unresolved Alerts`, `High Value`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_factors: Option<String>,
    /// The environment that the asset exists in — AWS | Azure | GCP | Active
    /// Directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment: Option<String>,
    /// The Surface that each asset belongs to (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surfaces__nin: Option<String>,
    /// The ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id__in: Option<String>,
    /// Free-text filter by cloud tag key value (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key_value__contains: Option<String>,
    /// The Asset Type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type__contains: Option<String>,
    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value: Option<String>,
    /// Sort direction. Allowed values: `asc`, `desc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// The cloud provider organization unit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization_unit__contains: Option<String>,
    /// The cloud provider organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_organization__contains: Option<String>,
    /// If true, only total number of items will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_only: Option<bool>,
    /// Name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<String>,
    /// The name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name__contains: Option<String>,
    /// The criticality that each asset belongs to. Allowed values: `critical`,
    /// `high`, `medium`, `low`, `--`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_criticality: Option<String>,
    /// The cloud provider account id (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id__nin: Option<String>,
    /// If true, total number of items will not be calculated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_count: Option<bool>,
    /// User and cloud tag keys not exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key__nexists: Option<String>,
    /// List of Site IDs to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<String>,
    /// The Last Seen date and time for the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s1_updated_at__between: Option<String>,
    /// The canonical name for the resource type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The environment that the asset exists in (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_environment__nin: Option<String>,
    /// Name (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names__nin: Option<String>,
    /// The cloud tags key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key: Option<String>,
    /// The sub-category that each resource belongs to (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category__nin: Option<String>,
    /// The status alerts of the asset. Allowed values: `Infected`, `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status: Option<String>,
    /// The active coverage for the asset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_coverage: Option<String>,
    /// The columns for which filter count would be returned for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_for: Option<String>,
    /// The ID of the CSV file to filter by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_filter_id: Option<i64>,
    /// The cloud provider account id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_id: Option<String>,
    /// The sub-category that each resource belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_category: Option<String>,
    /// The asset review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_review: Option<String>,
    /// User and cloud tag keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key: Option<String>,
    /// Limit number of returned items (1-1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// The ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id__contains: Option<String>,
    /// The cloud provider account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_account_name__contains: Option<String>,
    /// The status of the asset. Allowed values: `Active`, `Inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_status: Option<String>,
    /// Free-text filter by cloud tag key (supports multiple values).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_tags_key__contains: Option<String>,
    /// Tags (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_key_value__nin: Option<String>,
    /// User and cloud tag keys exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_tags_key__exists: Option<String>,
    /// Cursor position returned by the last request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// The status alerts of the asset (not in).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infection_status__nin: Option<String>,
    /// The cloud provider project ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_provider_project_id__contains: Option<String>,
}

impl ExportAssetsQuery {
    csv_setter!(tags_key__contains, "Free-text filter by tag key (supports multiple values).");
    csv_setter!(asset_criticality__nin, "The criticality that each asset belongs to (not in).");
    csv_setter!(missing_coverage, "The missing coverage for the asset.");
    csv_setter!(all_tags_key_value, "User and cloud tags.");
    csv_setter!(cloud_provider_account_name__nin, "The cloud provider account name (not in).");
    csv_setter!(tags_key__nin, "Tag Keys (not in).");
    csv_setter!(cloud_resource_id__contains, "The cloud resource ID.");
    csv_setter!(cloud_provider_subscription_id__contains, "The cloud provider subscription ID.");
    csv_setter!(tags_key__exists, "Tag Keys exists.");
    csv_setter!(region, "The region.");
    csv_setter!(tags_key_value__contains, "Free-text filter by tag key value (supports multiple values).");
    csv_setter!(cloud_tags_key__nin, "The cloud tags key (not in).");
    csv_setter!(tags_key, "Tag Keys.");
    csv_setter!(risk_factors__nin, "The risk factors associated with the asset (not in).");
    csv_setter!(tags_key__nexists, "Tag Keys not exists.");
    csv_setter!(cloud_provider_account_id__contains, "The cloud provider account ID.");
    csv_setter!(image_name__contains, "Free-text filter by the image name.");
    csv_setter!(group_ids, "List of Group IDs to filter by.");
    csv_setter!(active_coverage__nin, "The active coverage for the asset (not in).");
    csv_setter!(all_tags_key_value__nin, "User and cloud tags (not in).");
    csv_setter!(region__nin, "The region (not in).");
    csv_setter!(all_tags_key__nin, "User and cloud tag keys (not in).");
    csv_setter!(cloud_tags_key_value__nin, "The cloud tags key value (not in).");
    csv_setter!(surfaces, "The Surface that each asset belongs to.");
    csv_setter!(missing_coverage__nin, "The missing coverage for the asset (not in).");
    csv_setter!(asset_status__nin, "The status of the asset (not in).");
    csv_setter!(region__contains, "The geographical area where cloud resources are hosted.");
    csv_setter!(device_review__nin, "The asset review (not in).");
    csv_setter!(cloud_provider_account_name, "The cloud provider account name.");
    csv_setter!(asset_contact_email__nin, "Asset Contact Email (not in).");
    csv_setter!(alert_severity, "The severity of the alert.");
    csv_setter!(account_ids, "List of Account IDs to filter by.");
    csv_setter!(cloud_tags_key_value, "The cloud tags key value.");
    csv_setter!(resource_type__nin, "The canonical name for the resource type (not in).");
    csv_setter!(asset_contact_email, "Asset Contact Email.");
    csv_setter!(risk_factors, "The risk factors associated with the asset.");
    csv_setter!(asset_environment, "The environment that the asset exists in.");
    csv_setter!(surfaces__nin, "The Surface that each asset belongs to (not in).");
    csv_setter!(id__in, "The ID.");
    csv_setter!(cloud_tags_key_value__contains, "Free-text filter by cloud tag key value (supports multiple values).");
    csv_setter!(resource_type__contains, "The Asset Type.");
    csv_setter!(tags_key_value, "Tags.");
    csv_setter!(cloud_provider_organization_unit__contains, "The cloud provider organization unit.");
    csv_setter!(cloud_provider_organization__contains, "The cloud provider organization.");
    csv_setter!(names, "Name.");
    csv_setter!(name__contains, "The name.");
    csv_setter!(asset_criticality, "The criticality that each asset belongs to.");
    csv_setter!(cloud_provider_account_id__nin, "The cloud provider account id (not in).");
    csv_setter!(all_tags_key__nexists, "User and cloud tag keys not exists.");
    csv_setter!(site_ids, "List of Site IDs to filter by.");
    csv_setter!(resource_type, "The canonical name for the resource type.");
    csv_setter!(asset_environment__nin, "The environment that the asset exists in (not in).");
    csv_setter!(names__nin, "Name (not in).");
    csv_setter!(cloud_tags_key, "The cloud tags key.");
    csv_setter!(sub_category__nin, "The sub-category that each resource belongs to (not in).");
    csv_setter!(infection_status, "The status alerts of the asset.");
    csv_setter!(active_coverage, "The active coverage for the asset.");
    csv_setter!(counts_for, "The columns for which filter count would be returned for.");
    csv_setter!(cloud_provider_account_id, "The cloud provider account id.");
    csv_setter!(sub_category, "The sub-category that each resource belongs to.");
    csv_setter!(device_review, "The asset review.");
    csv_setter!(all_tags_key, "User and cloud tag keys.");
    csv_setter!(id__contains, "The ID.");
    csv_setter!(cloud_provider_account_name__contains, "The cloud provider account name.");
    csv_setter!(asset_status, "The status of the asset.");
    csv_setter!(cloud_tags_key__contains, "Free-text filter by cloud tag key (supports multiple values).");
    csv_setter!(tags_key_value__nin, "Tags (not in).");
    csv_setter!(all_tags_key__exists, "User and cloud tag keys exists.");
    csv_setter!(infection_status__nin, "The status alerts of the asset (not in).");
    csv_setter!(cloud_provider_project_id__contains, "The cloud provider project ID.");

    /// Skip first number of items (0-1000).
    pub fn skip(mut self, n: i64) -> Self {
        self.skip = Some(n);
        self
    }
    /// The column to sort the results by.
    pub fn sort_by(mut self, v: impl Into<String>) -> Self {
        self.sort_by = Some(v.into());
        self
    }
    /// Sort direction. Allowed values: `asc`, `desc`.
    pub fn sort_order(mut self, v: impl Into<String>) -> Self {
        self.sort_order = Some(v.into());
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
    /// The Last Seen date and time for the asset.
    pub fn s1_updated_at__between(mut self, v: impl Into<String>) -> Self {
        self.s1_updated_at__between = Some(v.into());
        self
    }
    /// The ID of the CSV file to filter by.
    pub fn csv_filter_id(mut self, n: i64) -> Self {
        self.csv_filter_id = Some(n);
        self
    }
    /// Limit number of returned items (1-1000).
    pub fn limit(mut self, n: i64) -> Self {
        self.limit = Some(n);
        self
    }
    /// Cursor position returned by the last request.
    pub fn cursor(mut self, v: impl Into<String>) -> Self {
        self.cursor = Some(v.into());
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/network` — Assets using POST.
///
/// Corresponds to `NetworkViewInputSchema` in the spec.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAssetsBody {
    /// Filter (required). The `PaginatedNetworkFilter` shape is large and
    /// freeform, so it is represented as an arbitrary JSON object.
    pub filter: serde_json::Value,
    /// Data (optional / nullable). The `EmptyStrict` shape declares no
    /// properties; represented as an arbitrary JSON value when supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl PostAssetsBody {
    /// Construct a body from the required `filter` value.
    pub fn new(filter: serde_json::Value) -> Self {
        Self { filter, data: None }
    }
    /// Set the optional `data` field.
    pub fn data(mut self, data: serde_json::Value) -> Self {
        self.data = Some(data);
        self
    }
}

/// Request body for `POST /web/api/v2.1/xdr/assets/network/action` — Perform
/// action.
///
/// Corresponds to `NetworkActionPayloadSchema` in the spec.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkActionBody {
    /// Action name (required).
    ///
    /// Allowed values: `export_resource_details`,
    /// `mark_asset_criticality_high`, `mark_asset_criticality_low`,
    /// `clear_asset_criticality`, `mark_asset_criticality_medium`,
    /// `mark_asset_criticality_critical`, `update_asset_contact`,
    /// `clear_asset_contact`, `apply_review`, `add_note`, `manage_tags`,
    /// `add_tags`, `remove_tags`, `replace_tags`, `clear_tags`.
    pub action_name: String,
    /// List of selected inventory ids (optional, max 5000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id__in: Option<Vec<String>>,
    /// List of inventory ids to exclude from `select_all` (optional, max 5000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id__nin: Option<Vec<String>>,
}

impl NetworkActionBody {
    /// Construct a body from the required `actionName`.
    pub fn new(action_name: impl Into<String>) -> Self {
        Self {
            action_name: action_name.into(),
            id__in: None,
            id__nin: None,
        }
    }
    /// List of selected inventory ids (max 5000).
    pub fn id__in<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__in = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from `select_all` (max 5000).
    pub fn id__nin<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__nin = Some(ids.into_iter().map(Into::into).collect());
        self
    }
}

/// Request body for
/// `POST /web/api/v2.1/xdr/assets/network/available-actions/with-status` —
/// Available actions.
///
/// Corresponds to `AffectedResourcesSchema` in the spec.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AffectedResourcesBody {
    /// List of selected inventory ids (optional, max 5000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id__in: Option<Vec<String>>,
    /// List of inventory ids to exclude from `select_all` (optional, max 5000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id__nin: Option<Vec<String>>,
}

impl AffectedResourcesBody {
    /// List of selected inventory ids (max 5000).
    pub fn id__in<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__in = Some(ids.into_iter().map(Into::into).collect());
        self
    }
    /// List of inventory ids to exclude from `select_all` (max 5000).
    pub fn id__nin<I, S>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.id__nin = Some(ids.into_iter().map(Into::into).collect());
        self
    }
}

impl InventoryNetworkService<'_> {
    /// `GET /web/api/v2.1/xdr/assets/network` — Assets.
    ///
    /// Get assets.
    ///
    /// Network inventory assets are returned as freeform JSON values
    /// ([`NetworkAsset`]); the spec defines the entity as an open object with no
    /// declared properties.
    pub async fn list(
        &self,
        query: &GetAssetsQuery,
    ) -> Result<Paginated<NetworkAsset>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/network", q)
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/network` — Assets using POST.
    ///
    /// POST API to get Assets. The detailed inventory filter is provided in the
    /// request body; only org-scope filters (`accountIds`, `siteIds`,
    /// `groupIds`) are accepted as query params.
    ///
    /// Network inventory assets are returned as freeform JSON values
    /// ([`NetworkAsset`]).
    pub async fn list_post(
        &self,
        query: &PostAssetsQuery,
        body: &PostAssetsBody,
    ) -> Result<Paginated<NetworkAsset>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(Method::POST, "/web/api/v2.1/xdr/assets/network", q, Some(body))
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/network/action` — Perform action.
    ///
    /// Perform action on selected assets. The selection is driven by the query
    /// filter ([`NetworkActionFilterQuery`]) combined with the body's explicit
    /// id lists.
    pub async fn perform_action(
        &self,
        query: &NetworkActionFilterQuery,
        body: &NetworkActionBody,
    ) -> Result<Response<serde_json::Value>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(
                Method::POST,
                "/web/api/v2.1/xdr/assets/network/action",
                q,
                Some(body),
            )
            .await?)
    }

    /// `POST /web/api/v2.1/xdr/assets/network/available-actions/with-status` —
    /// Available actions.
    ///
    /// Get cloud inventory network available-actions, each annotated with its
    /// enabled/disabled status for the current selection.
    pub async fn available_actions_with_status(
        &self,
        query: &NetworkActionFilterQuery,
        body: &AffectedResourcesBody,
    ) -> Result<Response<AvailableActionWithStatus>, Error> {
        let qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .request_json(
                Method::POST,
                "/web/api/v2.1/xdr/assets/network/available-actions/with-status",
                q,
                Some(body),
            )
            .await?)
    }

    /// `GET /web/api/v2.1/xdr/assets/network/export` — Export assets to CSV or
    /// JSON.
    ///
    /// Returns the results for the given inventory filter in a CSV or JSON
    /// format. `export_format` is required; allowed values: `csv`, `json`.
    ///
    /// The response body is the raw export payload, returned as freeform JSON.
    pub async fn export(
        &self,
        export_format: impl Into<String>,
        query: &ExportAssetsQuery,
    ) -> Result<Response<serde_json::Value>, Error> {
        let mut qs = serde_urlencoded::to_string(query).unwrap_or_default();
        let fmt = serde_urlencoded::to_string([("exportFormat", export_format.into())])
            .unwrap_or_default();
        if !fmt.is_empty() {
            if qs.is_empty() {
                qs = fmt;
            } else {
                qs.push('&');
                qs.push_str(&fmt);
            }
        }
        let q = (!qs.is_empty()).then_some(qs.as_str());
        Ok(self
            .client
            .http()
            .get("/web/api/v2.1/xdr/assets/network/export", q)
            .await?)
    }
}
